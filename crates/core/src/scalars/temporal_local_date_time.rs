//! Temporal.LocalDateTime: a calendar date and a time of day with no zone and
//! no offset, the reading of a wall clock: `2026-10-31T09:00:00`. It names no
//! instant until a zone is given beside it, which is what makes it the right
//! type for a recurrence's start, a store's opening time or an appointment
//! that stays at nine when the clocks change.
//!
//! Canonical form is `YYYY-MM-DDTHH:MM:SS`, with a fraction of up to nine
//! digits only when it is not zero and its trailing zeros trimmed, so equal
//! values are equal text and text order is time order. Input may write the
//! separator as `T`, `t` or one space, and may leave the seconds out.
//!
//! A value that carries a zone or an offset (`Z`, `+05:00`) is refused, not
//! stripped: it is an instant, a `Temporal.DateTime`, and dropping the zone
//! would silently move it. A date alone is a `Temporal.Date`.

use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use chrono::NaiveDate;

/// The longest input read, in bytes, after trimming. The canonical form is at
/// most 29.
const MAX_LENGTH: usize = 40;

pub struct TemporalLocalDateTime;

fn parse_error(message: impl Into<String>) -> ScalarError {
    ScalarError::new(ErrorKind::Parse, message)
}

/// `count` ASCII digits at the front of `text`, as a number, and the rest.
fn digits(text: &str, count: usize) -> Option<(u32, &str)> {
    let head = text.get(..count)?;
    if !head.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    Some((head.parse().ok()?, &text[count..]))
}

/// `text` without the one `literal` byte at its front.
fn skip(text: &str, literal: u8) -> Option<&str> {
    (text.as_bytes().first() == Some(&literal)).then(|| &text[1..])
}

fn canonical(input: &str) -> Result<String, ScalarError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ScalarError::new(
            ErrorKind::Empty,
            "local date-time is empty",
        ));
    }
    if trimmed.len() > MAX_LENGTH {
        return Err(ScalarError::new(
            ErrorKind::Length,
            format!("local date-time is longer than {MAX_LENGTH} characters"),
        ));
    }
    let shape = || {
        parse_error(format!(
            "not a local date-time (YYYY-MM-DDTHH:MM:SS): {trimmed}"
        ))
    };

    let (year, rest) = digits(trimmed, 4).ok_or_else(shape)?;
    let rest = skip(rest, b'-').ok_or_else(shape)?;
    let (month, rest) = digits(rest, 2).ok_or_else(shape)?;
    let rest = skip(rest, b'-').ok_or_else(shape)?;
    let (day, rest) = digits(rest, 2).ok_or_else(shape)?;
    let rest = match rest.as_bytes().first() {
        Some(b'T' | b't' | b' ') => &rest[1..],
        _ => return Err(shape()),
    };
    let (hour, rest) = digits(rest, 2).ok_or_else(shape)?;
    let rest = skip(rest, b':').ok_or_else(shape)?;
    let (minute, rest) = digits(rest, 2).ok_or_else(shape)?;
    let (second, rest) = match skip(rest, b':') {
        Some(rest) => digits(rest, 2).ok_or_else(shape)?,
        None => (0, rest),
    };
    let (fraction, rest) = match skip(rest, b'.') {
        Some(rest) => {
            let end = rest
                .bytes()
                .position(|byte| !byte.is_ascii_digit())
                .unwrap_or(rest.len());
            if end == 0 || end > 9 {
                return Err(parse_error(format!(
                    "a fraction of a second is one to nine digits: {trimmed}"
                )));
            }
            (rest[..end].trim_end_matches('0'), &rest[end..])
        }
        None => ("", rest),
    };
    if !rest.is_empty() {
        let zoned = matches!(rest.as_bytes()[0], b'Z' | b'z' | b'+' | b'-' | b'[');
        return Err(if zoned {
            parse_error(format!(
                "a local date-time carries no zone or offset: {trimmed}"
            ))
        } else {
            shape()
        });
    }

    if year == 0 || NaiveDate::from_ymd_opt(year as i32, month, day).is_none() {
        return Err(parse_error(format!("no such calendar date: {trimmed}")));
    }
    if hour > 23 || minute > 59 || second > 59 {
        return Err(parse_error(format!("no such time of day: {trimmed}")));
    }

    let mut out = format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}");
    if !fraction.is_empty() {
        out.push('.');
        out.push_str(fraction);
    }
    Ok(out)
}

impl Scalar for TemporalLocalDateTime {
    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        canonical(input)
    }

    /// The canonical form of a valid value; any other value as written,
    /// because normalize does not enforce shape.
    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        Ok(canonical(input).unwrap_or_else(|_| input.to_string()))
    }

    fn validate(&self, _registry: &Registry, input: &str) -> Result<(), ScalarError> {
        canonical(input).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::TemporalLocalDateTime;
    use crate::error::ErrorKind;
    use crate::registry::{Registry, Scalar};

    fn kind(input: &str) -> Option<ErrorKind> {
        TemporalLocalDateTime
            .validate(Registry::builtin(), input)
            .err()
            .map(|error| error.kind)
    }

    #[test]
    fn every_accepted_form_parses_to_the_canonical_form() {
        let registry = Registry::builtin();
        let cases = [
            ("2026-10-31T09:00:00", "2026-10-31T09:00:00"),
            ("2026-10-31T09:00", "2026-10-31T09:00:00"),
            ("2026-10-31 09:00", "2026-10-31T09:00:00"),
            ("2026-10-31t09:00:05", "2026-10-31T09:00:05"),
            ("  2026-10-31T09:00  ", "2026-10-31T09:00:00"),
            ("2026-10-31T09:00:00.5", "2026-10-31T09:00:00.5"),
            ("2026-10-31T09:00:00.500", "2026-10-31T09:00:00.5"),
            ("2026-10-31T09:00:00.000", "2026-10-31T09:00:00"),
            (
                "2026-10-31T09:00:00.123456789",
                "2026-10-31T09:00:00.123456789",
            ),
            ("2024-02-29T23:59:59", "2024-02-29T23:59:59"),
            ("0001-01-01T00:00", "0001-01-01T00:00:00"),
            ("9999-12-31T23:59:59", "9999-12-31T23:59:59"),
        ];
        for (input, canonical) in cases {
            assert_eq!(
                TemporalLocalDateTime.parse(registry, input).as_deref(),
                Ok(canonical),
                "parse({input:?})"
            );
            assert_eq!(
                TemporalLocalDateTime.normalize(registry, input).as_deref(),
                Ok(canonical),
                "normalize({input:?})"
            );
            assert_eq!(
                TemporalLocalDateTime.parse(registry, canonical).as_deref(),
                Ok(canonical),
                "parse({canonical:?})"
            );
        }
    }

    #[test]
    fn a_zone_or_an_offset_is_refused_not_stripped() {
        for input in [
            "2026-10-31T09:00:00Z",
            "2026-10-31T09:00z",
            "2026-10-31T09:00:00+05:00",
            "2026-10-31T09:00-08:00",
            "2026-10-31T09:00:00.5Z",
            "2026-10-31T09:00[Europe/Paris]",
        ] {
            let error = TemporalLocalDateTime
                .parse(Registry::builtin(), input)
                .expect_err(input);
            assert_eq!(error.kind, ErrorKind::Parse, "{input}");
            assert!(error.message.contains("no zone or offset"), "{input}");
        }
    }

    #[test]
    fn rejects_by_kind() {
        assert_eq!(kind(""), Some(ErrorKind::Empty));
        assert_eq!(kind("   "), Some(ErrorKind::Empty));
        assert_eq!(kind(&"2026-10-31T09:00".repeat(3)), Some(ErrorKind::Length));
        for input in [
            "2026-10-31",
            "09:00",
            "2026-10-31T9:00",
            "2026-10-31T09",
            "2026-10-31T09:00:",
            "2026-10-31T09:00:00.",
            "2026-10-31T09:00:00.1234567890",
            "2026-10-31T09:00:00,5",
            "2026-10-31  09:00",
            "2026/10/31T09:00",
            "26-10-31T09:00",
            "2026-13-01T09:00",
            "2026-02-30T09:00",
            "2025-02-29T09:00",
            "0000-01-01T00:00",
            "2026-10-31T24:00",
            "2026-10-31T09:60",
            "2026-10-31T09:00:60",
            "2026-10-31T09:00 PM",
            "２０２６-10-31T09:00",
            "yesterday",
        ] {
            assert_eq!(kind(input), Some(ErrorKind::Parse), "{input}");
        }
    }

    #[test]
    fn normalize_returns_a_refused_value_as_written() {
        let registry = Registry::builtin();
        assert_eq!(
            TemporalLocalDateTime
                .normalize(registry, " 2026-10-31T09:00Z ")
                .as_deref(),
            Ok(" 2026-10-31T09:00Z ")
        );
    }
}
