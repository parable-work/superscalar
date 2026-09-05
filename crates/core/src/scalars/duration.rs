use crate::catalog::ScalarId;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};

const NANOS_PER_MICROSECOND: i128 = 1_000;
const NANOS_PER_MILLISECOND: i128 = 1_000_000;
const NANOS_PER_SECOND: i128 = 1_000_000_000;
const NANOS_PER_MINUTE: i128 = 60 * NANOS_PER_SECOND;
const NANOS_PER_HOUR: i128 = 60 * NANOS_PER_MINUTE;
/// Guard: above 2^63 the f64->i128 `as` cast loses per-nanosecond precision.
const NANOS_OVERFLOW_GUARD: f64 = 9_223_372_036_854_775_808.0;

fn err(msg: impl Into<String>) -> ScalarError {
    ScalarError::new(ErrorKind::Parse, msg)
}

fn unit_nanos(unit: &str) -> Option<i128> {
    match unit {
        "ns" => Some(1),
        "us" | "\u{00b5}s" | "\u{03bc}s" => Some(NANOS_PER_MICROSECOND),
        "ms" => Some(NANOS_PER_MILLISECOND),
        "s" => Some(NANOS_PER_SECOND),
        "m" => Some(NANOS_PER_MINUTE),
        "h" => Some(NANOS_PER_HOUR),
        _ => None,
    }
}

fn parse_duration_nanos(input: &str) -> Result<i128, ScalarError> {
    let value = input.trim();
    if value.is_empty() {
        return Err(err("failed to parse Duration: empty value"));
    }
    let (sign, rest) = if let Some(stripped) = value.strip_prefix('-') {
        (-1_i128, stripped)
    } else if let Some(stripped) = value.strip_prefix('+') {
        (1_i128, stripped)
    } else {
        (1_i128, value)
    };
    if rest == "0" {
        return Ok(0);
    }

    // Walk chars, not bytes: micro-sign units are multi-byte, so byte-index slicing could split a code point.
    let chars: Vec<(usize, char)> = rest.char_indices().collect();
    let mut idx = 0usize;
    let mut total_nanos = 0_i128;
    while idx < chars.len() {
        let number_start = chars[idx].0;
        let mut saw_digit = false;
        let mut saw_dot = false;
        while idx < chars.len() {
            let ch = chars[idx].1;
            if ch.is_ascii_digit() {
                saw_digit = true;
                idx += 1;
                continue;
            }
            if ch == '.' && !saw_dot {
                saw_dot = true;
                idx += 1;
                continue;
            }
            break;
        }
        if !saw_digit {
            return Err(err(format!(
                "failed to parse Duration: invalid segment in {input}"
            )));
        }
        let number_end = chars.get(idx).map_or(rest.len(), |c| c.0);
        let number = &rest[number_start..number_end];
        let unit_start = number_end;
        while idx < chars.len() {
            let ch = chars[idx].1;
            if ch.is_ascii_alphabetic() || ch == '\u{00b5}' || ch == '\u{03bc}' {
                idx += 1;
                continue;
            }
            break;
        }
        let unit_end = chars.get(idx).map_or(rest.len(), |c| c.0);
        if unit_start == unit_end {
            return Err(err(format!(
                "failed to parse Duration: missing unit in {input}"
            )));
        }
        let unit = &rest[unit_start..unit_end];
        let multiplier = unit_nanos(unit)
            .ok_or_else(|| err(format!("failed to parse Duration: unknown unit {unit}")))?;
        let numeric = number
            .parse::<f64>()
            .map_err(|_| err(format!("failed to parse Duration: invalid number {number}")))?;
        // multiplier (< 2^53) widens to f64 exactly; product is bounds-checked before narrowing since bare `as i128` would saturate silently.
        let scaled = (numeric * multiplier as f64).round();
        if !scaled.is_finite() || scaled.abs() >= NANOS_OVERFLOW_GUARD {
            return Err(err("failed to parse Duration: duration overflow"));
        }
        let nanos = scaled as i128;
        total_nanos = total_nanos
            .checked_add(nanos)
            .ok_or_else(|| err("failed to parse Duration: duration overflow"))?;
    }
    Ok(total_nanos * sign)
}

fn format_seconds_component(whole_seconds: i128, remaining_nanos: i128) -> String {
    if remaining_nanos == 0 {
        return format!("{whole_seconds}s");
    }
    let mut fraction = format!("{remaining_nanos:09}");
    while fraction.ends_with('0') {
        fraction.pop();
    }
    format!("{whole_seconds}.{fraction}s")
}

fn format_duration_nanos(nanos: i128) -> String {
    if nanos == 0 {
        return "0s".to_string();
    }
    let sign = if nanos < 0 { "-" } else { "" };
    let mut remaining = nanos.abs();
    if remaining < NANOS_PER_SECOND {
        if remaining % NANOS_PER_MILLISECOND == 0 {
            return format!("{sign}{}ms", remaining / NANOS_PER_MILLISECOND);
        }
        if remaining % NANOS_PER_MICROSECOND == 0 {
            return format!("{sign}{}us", remaining / NANOS_PER_MICROSECOND);
        }
        return format!("{sign}{remaining}ns");
    }
    let hours = remaining / NANOS_PER_HOUR;
    remaining %= NANOS_PER_HOUR;
    let minutes = remaining / NANOS_PER_MINUTE;
    remaining %= NANOS_PER_MINUTE;
    let seconds = remaining / NANOS_PER_SECOND;
    let fractional = remaining % NANOS_PER_SECOND;
    let mut out = String::new();
    out.push_str(sign);
    if hours > 0 {
        out.push_str(&format!("{hours}h"));
    }
    if hours > 0 || minutes > 0 {
        out.push_str(&format!("{minutes}m"));
        out.push_str(&format_seconds_component(seconds, fractional));
        return out;
    }
    out.push_str(&format_seconds_component(seconds, fractional));
    out
}

fn normalize_duration(input: &str) -> Result<String, ScalarError> {
    parse_duration_nanos(input).map(format_duration_nanos)
}

pub struct Duration;

impl Scalar for Duration {
    fn id(&self) -> ScalarId {
        ScalarId::TEMPORAL_DURATION
    }

    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_duration(input)
    }

    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_duration(input)
    }

    fn validate(&self, _registry: &Registry, input: &str) -> Result<(), ScalarError> {
        parse_duration_nanos(input).map(|_| ())
    }
}
