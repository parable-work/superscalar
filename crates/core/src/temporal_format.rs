//! Unit-parameterized epoch-instant parsing for `x-temporal-format`.
//!
//! `Temporal.DateTime` canonicalizes ISO8601 text. Source APIs frequently
//! send the same instant as a bare epoch count instead, and the unit is not
//! recoverable from the value: `1503435956` is seconds, `1786534524000` is
//! milliseconds, and both are plausible readings of the other. Guessing the
//! unit from digit count is how Gmail `internalDate` and Slack `ts`
//! each shipped a different wrong answer.
//!
//! The unit is therefore a per-property schema declaration
//! (`x-temporal-format` / `@temporalFormat`) on a `Temporal.DateTime` field,
//! not a scalar identity: every epoch encoding canonicalizes to the same
//! RFC3339 UTC string `Temporal.DateTime` produces, so instant fields stay
//! one comparable kind of data in every binding and in the lake. These
//! functions are the decode half; promote passes the declared format from
//! the column plan.
//!
//! Distinct from `Temporal.Milliseconds` / `Temporal.Seconds`, which are
//! *durations* (`Int`/`BIGINT`, an amount of elapsed time) rather than
//! instants.
//!
//! Two properties of the accept-set are deliberate:
//!
//! * A value that is not a bare epoch count falls through to the
//!   `Temporal.DateTime` accept-set. That makes the canonical form a fixed
//!   point (`normalize(normalize(x)) == normalize(x)`), and it lets a source
//!   that mixes ISO text and epoch counts in one column land in one type.
//! * `0` and negative counts are real instants (1970 and before), not
//!   sentinels, so they are accepted. A source that means "unset" by `0`
//!   will land rows in 1970; that is a schema/quality concern, not something
//!   the parser can tell apart from a genuine 1970 event.

use chrono::{DateTime as ChronoDateTime, Datelike, Utc};

use crate::registry::{Registry, Scalar};
use crate::scalars::datetime::{to_canonical_rfc3339, DateTimeScalar};

const NANOS_PER_SECOND: i128 = 1_000_000_000;
const NANOS_PER_MILLI: i128 = 1_000_000;
const NANOS_PER_MICRO: i128 = 1_000;

/// A declared epoch wire encoding: the epoch members of
/// IncrementalTimeFormatEnum. ISO text is the un-annotated default and has
/// no member here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemporalFormat {
    UnixSeconds,
    UnixMillis,
    UnixMicros,
    UnixNanos,
}

impl TemporalFormat {
    /// Parse an `x-temporal-format` annotation value. `None` for anything
    /// outside the declared vocabulary -- callers decide whether that is an
    /// error (promote) or a schema-gate failure (validate).
    pub fn from_annotation(value: &str) -> Option<Self> {
        match value {
            "unix" => Some(Self::UnixSeconds),
            "unix_millis" => Some(Self::UnixMillis),
            "unix_micros" => Some(Self::UnixMicros),
            "unix_nanos" => Some(Self::UnixNanos),
            _ => None,
        }
    }

    pub fn as_annotation(&self) -> &'static str {
        match self {
            Self::UnixSeconds => "unix",
            Self::UnixMillis => "unix_millis",
            Self::UnixMicros => "unix_micros",
            Self::UnixNanos => "unix_nanos",
        }
    }

    /// Scale a stored integer epoch count to microseconds for SQL compare.
    pub fn sql_micros_scale(&self) -> TemporalMicrosScale {
        match self {
            Self::UnixSeconds => TemporalMicrosScale::Multiply(1_000_000),
            Self::UnixMillis => TemporalMicrosScale::Multiply(1_000),
            Self::UnixMicros => TemporalMicrosScale::Identity,
            Self::UnixNanos => TemporalMicrosScale::Divide(1_000),
        }
    }

    fn nanos_per_unit(&self) -> i128 {
        match self {
            Self::UnixSeconds => NANOS_PER_SECOND,
            Self::UnixMillis => NANOS_PER_MILLI,
            Self::UnixMicros => NANOS_PER_MICRO,
            Self::UnixNanos => 1,
        }
    }
}

/// How to turn a declared epoch integer into microseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemporalMicrosScale {
    Multiply(i64),
    Divide(i64),
    Identity,
}

/// Parse a signed decimal epoch value into nanoseconds since the epoch.
///
/// Accepts an optional fractional part so seconds-with-microseconds wire
/// formats (Slack `ts`, `"1503435956.000247"`) keep their sub-second
/// precision instead of being truncated to the second.
fn epoch_nanos(input: &str, nanos_per_unit: i128) -> Option<i128> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }
    let (negative, digits) = match trimmed.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, trimmed.strip_prefix('+').unwrap_or(trimmed)),
    };
    let (whole, fraction) = match digits.split_once('.') {
        Some((whole, fraction)) => (whole, fraction),
        None => (digits, ""),
    };
    if whole.is_empty() && fraction.is_empty() {
        return None;
    }
    if !whole.bytes().all(|b| b.is_ascii_digit()) || !fraction.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let mut nanos = if whole.is_empty() {
        0i128
    } else {
        whole.parse::<i128>().ok()?.checked_mul(nanos_per_unit)?
    };
    // Scale the fractional digits to nanoseconds-of-unit without floating
    // point: pad or truncate to the unit's nanosecond precision. At
    // nanosecond granularity the precision is zero, so a fraction carries no
    // representable value and truncates away entirely -- it must not fail the
    // parse, or an OTel value serialized through a float ("...123.0") would
    // quarantine while the same digits without the fraction succeed.
    let precision = nanos_per_unit.to_string().len() - 1;
    if !fraction.is_empty() && precision > 0 {
        let mut scaled = String::with_capacity(precision);
        for index in 0..precision {
            scaled.push(fraction.as_bytes().get(index).copied().unwrap_or(b'0') as char);
        }
        nanos = nanos.checked_add(scaled.parse::<i128>().ok()?)?;
    }
    if negative {
        nanos = -nanos;
    }
    Some(nanos)
}

/// Decode a bare epoch count at the declared unit into a UTC instant.
/// `None` when the input is not a bare epoch count, or is out of range.
fn epoch_datetime_utc(input: &str, format: TemporalFormat) -> Option<ChronoDateTime<Utc>> {
    let nanos = epoch_nanos(input, format.nanos_per_unit())?;
    let seconds = i64::try_from(nanos.div_euclid(NANOS_PER_SECOND)).ok()?;
    let subsec = u32::try_from(nanos.rem_euclid(NANOS_PER_SECOND)).ok()?;
    let dt: ChronoDateTime<Utc> = ChronoDateTime::from_timestamp(seconds, subsec)?;
    // Parity with the DateTime and Date scalars, which reject year < 1000 so
    // chrono's greedy %Y cannot corrupt input into year 0001. It does not --
    // and cannot -- detect a mis-declared unit: a 1970 instant is valid. The
    // schema validate gate owns that (eventTimeDeclaredUnit-style pins).
    if dt.year() < 1000 {
        return None;
    }
    Some(dt)
}

/// Canonicalize a value from a column with a declared epoch format to the
/// same RFC3339 UTC string `Temporal.DateTime` produces.
///
/// Input that is not a bare epoch count falls through to the
/// `Temporal.DateTime` accept-set, so the canonical output -- an RFC3339
/// string -- parses back to itself, and a column mixing ISO text and epoch
/// counts lands in one type.
pub fn normalize_epoch(input: &str, format: TemporalFormat) -> Option<String> {
    match epoch_datetime_utc(input, format) {
        Some(dt) => Some(to_canonical_rfc3339(&dt.fixed_offset())),
        None if epoch_nanos(input, format.nanos_per_unit()).is_some() => None,
        None => DateTimeScalar.parse(Registry::builtin(), input).ok(),
    }
}

/// Decode a value from a column with a declared epoch format directly to
/// epoch microseconds -- the promote physical unit -- without a canonical
/// string round trip, so sub-second precision (the Slack `ts` dedup key)
/// survives exactly.
pub fn epoch_timestamp_micros(input: &str, format: TemporalFormat) -> Option<i64> {
    match epoch_datetime_utc(input, format) {
        Some(dt) => Some(dt.timestamp_micros()),
        None if epoch_nanos(input, format.nanos_per_unit()).is_some() => None,
        None => {
            let parsed = crate::scalars::datetime::DateTime::parse(input).ok()?;
            Some(parsed.to_chrono().timestamp_micros())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seconds(input: &str) -> Option<String> {
        normalize_epoch(input, TemporalFormat::UnixSeconds)
    }

    #[test]
    fn annotation_names_round_trip() {
        for format in [
            TemporalFormat::UnixSeconds,
            TemporalFormat::UnixMillis,
            TemporalFormat::UnixMicros,
            TemporalFormat::UnixNanos,
        ] {
            assert_eq!(
                TemporalFormat::from_annotation(format.as_annotation()),
                Some(format)
            );
        }
        assert_eq!(TemporalFormat::from_annotation("iso8601"), None);
        assert_eq!(TemporalFormat::from_annotation("unix_ms"), None);
    }

    #[test]
    fn parses_slack_ts_seconds_with_microsecond_fraction() {
        // The exact wire shape that failed promote during incident-13.
        assert_eq!(
            seconds("1503435956.000247").as_deref(),
            Some("2017-08-22T21:05:56.000247Z")
        );
    }

    #[test]
    fn parses_whole_seconds_without_a_fraction() {
        assert_eq!(
            seconds("1503435956").as_deref(),
            Some("2017-08-22T21:05:56Z")
        );
    }

    #[test]
    fn keeps_microsecond_precision_rather_than_truncating_to_the_second() {
        // Slack ts doubles as a dedup/ordering key, so two messages one
        // microsecond apart must not canonicalize to the same instant.
        assert_ne!(seconds("1503435956.000247"), seconds("1503435956.000248"));
        assert_ne!(
            epoch_timestamp_micros("1503435956.000247", TemporalFormat::UnixSeconds),
            epoch_timestamp_micros("1503435956.000248", TemporalFormat::UnixSeconds)
        );
    }

    #[test]
    fn parses_milliseconds_at_their_declared_unit() {
        // Gmail internalDate: same digits, different unit,
        // and the schema -- not a digit-count heuristic -- decides which.
        assert_eq!(
            normalize_epoch("1786534524000", TemporalFormat::UnixMillis).as_deref(),
            Some("2026-08-12T11:35:24Z")
        );
    }

    #[test]
    fn parses_microseconds_at_their_declared_unit() {
        // monday.com activity_logs.created_at is epoch micros
        // (cursorValueFormat unix_micros); the unit the epoch-scalar design
        // could not express without a fourth catalog entry.
        assert_eq!(
            normalize_epoch("1786534524000000", TemporalFormat::UnixMicros).as_deref(),
            Some("2026-08-12T11:35:24Z")
        );
        assert_eq!(
            normalize_epoch("1786534524123456", TemporalFormat::UnixMicros).as_deref(),
            Some("2026-08-12T11:35:24.123456Z")
        );
    }

    #[test]
    fn parses_nanoseconds_at_their_declared_unit() {
        // OTel timeUnixNano (claude-otel/logs) exceeds 2^53, so it arrives
        // as a string and must not round-trip through f64.
        assert_eq!(
            normalize_epoch("1786534524000000123", TemporalFormat::UnixNanos).as_deref(),
            Some("2026-08-12T11:35:24.000000123Z")
        );
    }

    #[test]
    fn the_same_digits_resolve_differently_per_declared_format() {
        // The whole point: the unit is declared, never inferred.
        assert_ne!(
            normalize_epoch("1786534524000", TemporalFormat::UnixMillis),
            normalize_epoch("1786534524000", TemporalFormat::UnixSeconds)
        );
    }

    #[test]
    fn a_mis_declared_unit_is_not_detectable_and_must_be_caught_by_the_schema_gate() {
        // 1503435956 read as nanoseconds is a real instant (1970-01-01), so
        // no parser guard can reject it. This is precisely why the unit is
        // declared in the schema and pinned by the validate gate rather than
        // inferred here.
        assert_eq!(
            normalize_epoch("1503435956", TemporalFormat::UnixNanos).as_deref(),
            Some("1970-01-01T00:00:01.503435956Z")
        );
    }

    #[test]
    fn rejects_non_numeric_input() {
        assert_eq!(seconds("not-an-epoch"), None);
        assert_eq!(seconds(""), None);
        assert_eq!(seconds("  "), None);
        assert_eq!(seconds("1.7865345e9"), None);
    }

    #[test]
    fn rejects_out_of_range_counts_instead_of_wrapping() {
        assert_eq!(seconds("99999999999999999999999"), None);
        assert_eq!(
            epoch_timestamp_micros("99999999999999999999999", TemporalFormat::UnixSeconds),
            None
        );
    }

    #[test]
    fn the_canonical_form_parses_back_to_itself() {
        // The canonical output is an RFC3339 string, so every format has to
        // accept it. Without this, normalize(normalize(x)) errors and every
        // re-promote of already canonicalized data quarantines.
        let canonical = seconds("1503435956").expect("canonical");
        for format in [
            TemporalFormat::UnixSeconds,
            TemporalFormat::UnixMillis,
            TemporalFormat::UnixMicros,
            TemporalFormat::UnixNanos,
        ] {
            assert_eq!(
                normalize_epoch(&canonical, format).as_deref(),
                Some(canonical.as_str()),
                "{} must accept the canonical RFC3339 form",
                format.as_annotation()
            );
        }
    }

    #[test]
    fn accepts_iso_text_so_a_mixed_column_lands_in_one_type() {
        // A source that sends ISO for some rows and an epoch count for others
        // (a migration mid-flight, a vendor that changed format) must not
        // quarantine half the column.
        assert_eq!(
            seconds("2017-08-22T21:05:56Z").as_deref(),
            Some("2017-08-22T21:05:56Z")
        );
        // ISO input keeps its offset, exactly as Temporal.DateTime
        // canonicalizes it; an epoch count has no offset and lands in UTC.
        assert_eq!(
            seconds("2017-08-22T23:05:56+02:00").as_deref(),
            Some("2017-08-22T23:05:56+02:00")
        );
        // The micros path agrees on the instant either way.
        assert_eq!(
            epoch_timestamp_micros("2017-08-22T21:05:56Z", TemporalFormat::UnixSeconds),
            epoch_timestamp_micros("1503435956", TemporalFormat::UnixSeconds)
        );
    }

    #[test]
    fn a_fractional_nanosecond_value_truncates_rather_than_failing() {
        // Nanoseconds are already the finest unit, so a fraction carries no
        // representable value. Failing the parse would quarantine an OTel
        // timestamp that went through a float ("...123.0") while the same
        // digits without the fraction succeed.
        assert_eq!(
            normalize_epoch("1786534524000000123.0", TemporalFormat::UnixNanos).as_deref(),
            Some("2026-08-12T11:35:24.000000123Z")
        );
        assert_eq!(
            normalize_epoch("1786534524000000123.9", TemporalFormat::UnixNanos),
            normalize_epoch("1786534524000000123", TemporalFormat::UnixNanos)
        );
    }

    #[test]
    fn zero_and_negative_counts_are_instants_not_sentinels() {
        // A parser cannot tell "unset" from a genuine 1970 event, so both are
        // accepted and the 1970 pile-up is a quality-rule concern. Pinned so
        // the behaviour is a decision rather than an accident.
        assert_eq!(seconds("0").as_deref(), Some("1970-01-01T00:00:00Z"));
        assert_eq!(
            seconds("-1503435956").as_deref(),
            Some("1922-05-12T02:54:04Z")
        );
    }

    #[test]
    fn tolerates_surrounding_whitespace_and_a_leading_sign() {
        assert_eq!(
            seconds("  1503435956  ").as_deref(),
            Some("2017-08-22T21:05:56Z")
        );
        assert_eq!(
            seconds("+1503435956").as_deref(),
            Some("2017-08-22T21:05:56Z")
        );
    }

    #[test]
    fn truncates_sub_nanosecond_fraction_rather_than_failing() {
        assert_eq!(
            seconds("1503435956.0002479999").as_deref(),
            Some("2017-08-22T21:05:56.000247999Z")
        );
    }

    #[test]
    fn micros_path_matches_the_canonical_string_path() {
        // epoch_timestamp_micros skips the string round trip; the two paths
        // must land on the same instant (at microsecond resolution).
        assert_eq!(
            epoch_timestamp_micros("1503435956.000247", TemporalFormat::UnixSeconds),
            Some(1_503_435_956_000_247)
        );
        assert_eq!(
            epoch_timestamp_micros("1786534524000", TemporalFormat::UnixMillis),
            Some(1_786_534_524_000_000)
        );
        assert_eq!(
            epoch_timestamp_micros("1786534524000000123", TemporalFormat::UnixNanos),
            Some(1_786_534_524_000_000)
        );
    }
}
