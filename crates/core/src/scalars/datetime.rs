//! Canonical = Go RFC3339Nano: sub-seconds are preserved (fraction kept, trailing zeros trimmed, omitted when zero).

use crate::catalog::ScalarId;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime, SecondsFormat, TimeZone, Utc};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

type ChronoDateTime<Tz> = chrono::DateTime<Tz>;

// Shared with temporal_date.rs so the Date scalar accepts the same datetime forms (then drops
// the time); a single source keeps the two scalars from drifting back into an accept-set asymmetry.
pub(crate) const NAIVE_DATETIME_FORMATS: &[&str] = &[
    "%Y-%m-%dT%H:%M:%S",
    "%Y-%m-%dT%H:%M:%S%.f",
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%d %H:%M:%S%.f",
    "%Y/%m/%d %H:%M:%S",
];
pub(crate) const OFFSET_DATETIME_FORMATS: &[&str] = &[
    "%Y-%m-%dT%H:%M:%S%#z",
    "%Y-%m-%dT%H:%M:%S%.f%#z",
    "%Y-%m-%d %H:%M:%S%#z",
    "%Y-%m-%d %H:%M:%S%.f%#z",
];
const DATE_ONLY_FORMATS: &[&str] = &["%Y-%m-%d", "%Y/%m/%d"];

/// Render Go RFC3339Nano form; trims manually because `chrono`'s `AutoSi` pads to 3/6/9 digits.
pub(crate) fn to_canonical_rfc3339(dt: &ChronoDateTime<FixedOffset>) -> String {
    if dt.timestamp_subsec_nanos() == 0 {
        return dt.to_rfc3339_opts(SecondsFormat::Secs, true);
    }
    let full = dt.to_rfc3339_opts(SecondsFormat::Nanos, true);
    trim_fraction(&full)
}

/// Trim trailing zeros from the fractional-second group, dropping it (and the dot) if all zero; handles `Z` and numeric offsets.
fn trim_fraction(s: &str) -> String {
    let dot = match s.find('.') {
        Some(d) => d,
        None => return s.to_string(),
    };
    let head = &s[..dot];
    let frac_and_offset = &s[dot + 1..];
    let digits_end = frac_and_offset
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(frac_and_offset.len());
    let offset = &frac_and_offset[digits_end..];
    let trimmed = frac_and_offset[..digits_end].trim_end_matches('0');
    if trimmed.is_empty() {
        return format!("{head}{offset}");
    }
    format!("{head}.{trimmed}{offset}")
}

fn normalize_datetime(input: &str) -> Option<String> {
    parse_datetime_strict(input).map(|dt| to_canonical_rfc3339(&dt))
}

/// Full accept-set parse with the year guard applied; shared by the string
/// canonicalizer and the typed [`DateTime`] wrapper so the two cannot drift.
fn parse_datetime_strict(input: &str) -> Option<ChronoDateTime<FixedOffset>> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }
    let dt = parse_datetime_candidate(trimmed)?;
    // Mirror the temporal_date.rs 4-digit-year guard: chrono's %Y greedily matches a 1-3
    // digit leading year ("01/02/24" -> year 1, "24-06-15T.." -> year 24), and RFC3339
    // accepts the "0001-01-01" sentinel. Reject year < 1000 cleanly instead of silently
    // corrupting it; the Date scalar already does, and the two scalars must not diverge
    // (parse_temporal_date and parse_temporal_date_time feed the same coverage day-buckets).
    if dt.year() < 1000 {
        return None;
    }
    Some(dt)
}

// Epoch counts are deliberately NOT in the accept-set. An earlier version taught
// this parser 12..=13-digit strings as unix milliseconds for Gmail
// internalDate, which silently excluded every other unit -- the unit of a
// bare count is not recoverable from the value. Epoch wire formats are now
// declared per property (x-temporal-format) and decoded by
// temporal_format::normalize_epoch; an undeclared bare count fails loudly
// here instead of guessing.
fn parse_datetime_candidate(trimmed: &str) -> Option<ChronoDateTime<FixedOffset>> {
    if let Ok(dt) = ChronoDateTime::parse_from_rfc3339(trimmed) {
        return Some(dt);
    }
    if let Ok(dt) = ChronoDateTime::parse_from_rfc2822(trimmed) {
        return Some(dt);
    }
    for fmt in OFFSET_DATETIME_FORMATS {
        if let Ok(dt) = ChronoDateTime::parse_from_str(trimmed, fmt) {
            return Some(dt);
        }
    }
    for fmt in NAIVE_DATETIME_FORMATS {
        if let Ok(ndt) = NaiveDateTime::parse_from_str(trimmed, fmt) {
            return Some(Utc.from_utc_datetime(&ndt).fixed_offset());
        }
    }
    for fmt in DATE_ONLY_FORMATS {
        if let Ok(date) = NaiveDate::parse_from_str(trimmed, fmt) {
            let ndt = date.and_hms_opt(0, 0, 0)?;
            return Some(Utc.from_utc_datetime(&ndt).fixed_offset());
        }
    }
    None
}

pub struct DateTimeScalar;

impl Scalar for DateTimeScalar {
    fn id(&self) -> ScalarId {
        ScalarId::TEMPORAL_DATE_TIME
    }

    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_datetime(input).ok_or_else(|| {
            ScalarError::new(
                ErrorKind::Parse,
                format!("failed to parse DateTime: {input}"),
            )
        })
    }

    fn normalize(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.parse(registry, input)
    }

    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        self.parse(registry, input).map(|_| ())
    }
}

/// Typed Temporal.DateTime newtype carried by generated Rust types (the
/// chrono analog of Go scalar-lib's `type TemporalDateTime time.Time`).
///
/// Stores the instant as `chrono::DateTime<Utc>`; inputs with a non-UTC
/// offset are converted to the same instant in UTC. Serializes in the
/// scalar's canonical Go RFC3339Nano form (sub-second trailing zeros
/// trimmed, fraction omitted when zero, `Z` suffix) -- chrono's own serde
/// pads sub-seconds to 3/6/9 digits, which would diverge from the Go
/// `time.Time` wire form. Deserializes with the scalar's full accept set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DateTime(ChronoDateTime<Utc>);

impl DateTime {
    pub fn from_chrono(value: ChronoDateTime<Utc>) -> Self {
        Self(value)
    }

    pub fn to_chrono(self) -> ChronoDateTime<Utc> {
        self.0
    }

    pub fn as_chrono(&self) -> &ChronoDateTime<Utc> {
        &self.0
    }

    /// Epoch-millisecond constructor; `None` when out of chrono's range.
    pub fn from_timestamp_millis(ms: i64) -> Option<Self> {
        ChronoDateTime::from_timestamp_millis(ms).map(Self)
    }

    pub fn timestamp_millis(&self) -> i64 {
        self.0.timestamp_millis()
    }

    /// Parse with the scalar's full accept set (RFC3339/2822, spaced SQL
    /// form, date-only, naive-assumed-UTC) and the 4-digit-year guard.
    pub fn parse(input: &str) -> Result<Self, ScalarError> {
        parse_datetime_strict(input)
            .map(|dt| Self(dt.with_timezone(&Utc)))
            .ok_or_else(|| {
                ScalarError::new(
                    ErrorKind::Parse,
                    format!("failed to parse DateTime: {input}"),
                )
            })
    }
}

impl From<ChronoDateTime<Utc>> for DateTime {
    fn from(value: ChronoDateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<ChronoDateTime<FixedOffset>> for DateTime {
    fn from(value: ChronoDateTime<FixedOffset>) -> Self {
        Self(value.with_timezone(&Utc))
    }
}

impl From<DateTime> for ChronoDateTime<Utc> {
    fn from(value: DateTime) -> Self {
        value.0
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&to_canonical_rfc3339(&self.0.fixed_offset()))
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DateTimeVisitor;

        impl Visitor<'_> for DateTimeVisitor {
            type Value = DateTime;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an RFC3339 datetime string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                DateTime::parse(value).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(DateTimeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::{normalize_datetime, DateTime};

    fn parse(input: &str) -> Option<String> {
        normalize_datetime(input)
    }

    #[test]
    fn typed_datetime_serializes_canonical_rfc3339() {
        let dt = DateTime::from_timestamp_millis(1_751_565_600_000).unwrap();
        let json = serde_json::to_string(&dt).unwrap();
        assert_eq!(json, "\"2025-07-03T18:00:00Z\"");

        // Sub-second trailing zeros trim to the Go RFC3339Nano form, not
        // chrono's 3/6/9-digit padding.
        let with_ms = DateTime::from_timestamp_millis(1_751_565_600_500).unwrap();
        assert_eq!(
            serde_json::to_string(&with_ms).unwrap(),
            "\"2025-07-03T18:00:00.5Z\""
        );
    }

    #[test]
    fn typed_datetime_round_trips_and_normalizes_offsets_to_utc() {
        let dt: DateTime = serde_json::from_str("\"2025-07-03T18:00:00.5Z\"").unwrap();
        assert_eq!(dt.timestamp_millis(), 1_751_565_600_500);
        let back = serde_json::to_string(&dt).unwrap();
        assert_eq!(back, "\"2025-07-03T18:00:00.5Z\"");

        // Non-UTC offsets convert to the same instant in UTC.
        let offset: DateTime = serde_json::from_str("\"2025-07-03T18:00:00+05:00\"").unwrap();
        assert_eq!(offset.to_string(), "2025-07-03T13:00:00Z");
    }

    #[test]
    fn typed_datetime_rejects_garbage_and_sentinel_years() {
        assert!(DateTime::parse("not a date").is_err());
        assert!(DateTime::parse("0001-01-01T00:00:00Z").is_err());
        assert!(serde_json::from_str::<DateTime>("\"01/02/24\"").is_err());
    }

    #[test]
    fn parses_postgres_timestamptz_hour_offset() {
        assert_eq!(
            parse("2026-05-29 14:52:58.615435+00").as_deref(),
            Some("2026-05-29T14:52:58.615435Z")
        );
        assert_eq!(
            parse("2026-05-29 14:52:58+00").as_deref(),
            Some("2026-05-29T14:52:58Z")
        );
    }

    #[test]
    fn rejects_two_digit_year_instead_of_corrupting() {
        // Symmetric with temporal_date.rs: a 1-3 digit leading year must reject cleanly,
        // never silently corrupt into year 0001/0024 (which would poison coverage min/max).
        assert_eq!(parse("01/02/24"), None);
        assert_eq!(parse("1/2/24"), None);
        assert_eq!(parse("24-06-15T09:00:00"), None);
    }

    #[test]
    fn rejects_year_0001_sentinel() {
        // The Go zero-time / .NET DateTime.MinValue sentinel must not bucket as a real date.
        assert_eq!(parse("0001-01-01T00:00:00Z"), None);
        assert_eq!(parse("0001-01-01"), None);
    }

    #[test]
    fn rejects_bare_epoch_counts_at_every_digit_length() {
        // The unit of a bare count is not recoverable from the
        // value, so DateTime never guesses. The 12..=13-digit unix-ms
        // acceptance (Gmail internalDate) is deleted; declared
        // wire formats (x-temporal-format -> temporal_format::normalize_epoch)
        // own epoch decoding now.
        assert!(DateTime::parse("").is_err());
        assert!(DateTime::parse("123456789").is_err()); // 9 digits
        assert!(DateTime::parse("1724000000").is_err()); // 10 digits — seconds-shaped
        assert!(DateTime::parse("12345678901").is_err()); // 11 digits
        assert!(DateTime::parse("100000000000").is_err()); // 12 digits — was accepted as ms
        assert!(DateTime::parse("1786550463000").is_err()); // 13 digits — was accepted as ms
        assert!(DateTime::parse("9999999999999").is_err()); // 13-digit upper bound
        assert!(DateTime::parse("12345678901234").is_err()); // 14 digits
        assert!(DateTime::parse("1786550463abc").is_err()); // non-digit
        assert!(DateTime::parse("1786550463.0").is_err()); // decimal
        assert!(DateTime::parse("1786550463000; DROP").is_err()); // injection-style
        assert!(DateTime::parse("-1786550463000").is_err()); // signed
        assert!(DateTime::parse("+1786550463000").is_err()); // '+' prefix
        assert_eq!(parse("123456789"), None);
        assert_eq!(parse("1786550463000"), None);
    }

    #[test]
    fn parses_four_digit_year_inputs_unchanged() {
        assert_eq!(
            parse("2024-06-15T09:00:00Z").as_deref(),
            Some("2024-06-15T09:00:00Z")
        );
        // Naive datetime assumed UTC -> Z.
        assert_eq!(
            parse("2024-06-15T09:00:00").as_deref(),
            Some("2024-06-15T09:00:00Z")
        );
        // Spaced SQL form, interpreted as UTC.
        assert_eq!(
            parse("2024-06-15 09:00:00").as_deref(),
            Some("2024-06-15T09:00:00Z")
        );
        // Date-only -> midnight UTC.
        assert_eq!(parse("2024-06-15").as_deref(), Some("2024-06-15T00:00:00Z"));
        // Sub-seconds preserved (trailing zeros trimmed).
        assert_eq!(
            parse("2024-06-15T09:00:00.123Z").as_deref(),
            Some("2024-06-15T09:00:00.123Z")
        );
        // Numeric offset preserved (not normalized to UTC).
        assert_eq!(
            parse("2024-06-15T09:00:00+05:00").as_deref(),
            Some("2024-06-15T09:00:00+05:00")
        );
    }
}
