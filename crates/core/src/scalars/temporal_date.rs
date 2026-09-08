use crate::catalog::ScalarId;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use crate::scalars::datetime::{NAIVE_DATETIME_FORMATS, OFFSET_DATETIME_FORMATS};
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime};

// Temporal.Date is calendar-date-only. Bare digit strings (epoch milliseconds,
// epoch seconds, compact YYYYMMDD) are rejected: a digit run carries no unit and
// no calendar, so the scalar refuses to guess. A consumer that must accept epoch
// inputs at a storage boundary (for example a lenient cast into a date column)
// converts them itself after the scalar rejects. That rejection is what makes
// such a fallback safe, and the conformance vectors pin it.

// 2-digit-year formats are deliberately excluded: dropping 2-digit-year support is the
// intended narrowing. The remaining slash/dash formats are still satisfied by chrono for
// 1-3 digit leading years (e.g. "01/02/24" matches "%Y/%m/%d" as year 1), so the final guard
// requires a 4-digit year to avoid silently corrupting such input into year 0001.
const NAIVE_DATE_FORMATS: &[&str] = &[
    "%Y-%m-%d",
    "%Y/%m/%d",
    "%m/%d/%Y",
    "%B %d, %Y",
    "%b %d, %Y",
    "%d %B %Y",
    "%d %b %Y",
];

// A Date accepts anything DateTime accepts (and drops the time) plus the date-only forms.
// Datetime formats are reused from datetime.rs so the two scalars cannot drift apart.
fn parse_to_naive_date(trimmed: &str) -> Option<NaiveDate> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(trimmed) {
        return Some(dt.date_naive());
    }
    if let Ok(dt) = DateTime::parse_from_rfc2822(trimmed) {
        return Some(dt.date_naive());
    }
    for fmt in OFFSET_DATETIME_FORMATS {
        if let Ok(dt) = DateTime::parse_from_str(trimmed, fmt) {
            return Some(dt.date_naive());
        }
    }
    for fmt in NAIVE_DATETIME_FORMATS {
        if let Ok(ndt) = NaiveDateTime::parse_from_str(trimmed, fmt) {
            return Some(ndt.date());
        }
    }
    for fmt in NAIVE_DATE_FORMATS {
        if let Ok(date) = NaiveDate::parse_from_str(trimmed, fmt) {
            return Some(date);
        }
    }
    None
}

fn normalize_temporal_date(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }
    let date = parse_to_naive_date(trimmed)?;
    // Require a 4-digit year: a 1-2 digit leading year (e.g. "01/02/24" or "24-06-15T..")
    // must reject cleanly rather than corrupt into year 0001/0024.
    if date.year() < 1000 {
        return None;
    }
    Some(date.format("%Y-%m-%d").to_string())
}

pub struct TemporalDate;

impl Scalar for TemporalDate {
    fn id(&self) -> ScalarId {
        ScalarId::TEMPORAL_DATE
    }

    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_temporal_date(input).ok_or_else(|| {
            ScalarError::new(ErrorKind::Parse, format!("failed to parse Date: {input}"))
        })
    }

    fn normalize(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.parse(registry, input)
    }

    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        self.parse(registry, input).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_temporal_date as parse;

    #[test]
    fn rejects_two_digit_year_instead_of_corrupting() {
        // Was silently corrupted: "01/02/24" -> "0001-02-24", "12/31/24" -> "0024-12-31".
        // 2-digit-year support is deliberately dropped; reject cleanly like any other bad input.
        assert_eq!(parse("01/02/24"), None);
        assert_eq!(parse("1/2/24"), None);
        assert_eq!(parse("12/31/24"), None);
    }

    #[test]
    fn parses_four_digit_year_inputs_unchanged() {
        assert_eq!(parse("2025-01-01").as_deref(), Some("2025-01-01"));
        assert_eq!(parse("2025/01/15").as_deref(), Some("2025-01-15"));
        assert_eq!(parse("01/15/2025").as_deref(), Some("2025-01-15"));
        assert_eq!(parse("January 15, 2025").as_deref(), Some("2025-01-15"));
        assert_eq!(parse("Jan 15, 2025").as_deref(), Some("2025-01-15"));
        assert_eq!(parse("2025-01-15T12:00:00Z").as_deref(), Some("2025-01-15"));
        assert_eq!(parse("  2025-01-01  ").as_deref(), Some("2025-01-01"));
    }

    #[test]
    fn parses_datetime_inputs_as_date() {
        // A Temporal.Date accepts anything Temporal.DateTime accepts and drops the time
        // (parity with old dateutil + closes the asymmetry where the DateTime scalar parsed
        // these but the Date scalar did not). Real case: MS Graph emits naive event times.
        assert_eq!(parse("2024-01-02T03:04:05").as_deref(), Some("2024-01-02")); // naive ISO datetime
        assert_eq!(parse("2024-06-15T09:00:00").as_deref(), Some("2024-06-15")); // MS Graph start.dateTime
        assert_eq!(
            parse("2017-08-14T21:00:00.0000000").as_deref(),
            Some("2017-08-14")
        ); // 7-digit fraction
        assert_eq!(parse("2024-01-02 03:04:05").as_deref(), Some("2024-01-02")); // spaced datetime
        assert_eq!(parse("2024-01-02T03:04:05Z").as_deref(), Some("2024-01-02")); // already worked (rfc3339)
        assert_eq!(
            parse("2024-01-02 03:04:05+05:00").as_deref(),
            Some("2024-01-02")
        ); // spaced + offset
    }

    #[test]
    fn keeps_narrowed_surface_rejected() {
        // Deliberately-dropped formats; the fix must not re-widen them.
        assert_eq!(parse("Mon, 02 Jan 2024 03:04:05 +0000"), None); // RFC2822 weekday form
        assert_eq!(parse("2024"), None); // bare year
        assert_eq!(parse("02/2024"), None); // MM/YYYY
        assert_eq!(parse("24-06-15T09:00:00"), None); // 2-digit year datetime must still reject
    }

    #[test]
    fn rejects_bare_digit_strings() {
        // Epoch and compact forms carry no unit; reject rather than guess. Consumers rely on
        // this rejection to route such inputs to their own epoch handling (see the note at
        // the top of the file); the conformance vectors pin the three epoch forms.
        assert_eq!(parse("1736899200000"), None); // 13-digit epoch milliseconds
        assert_eq!(parse("173689920000"), None); // 12-digit epoch milliseconds
        assert_eq!(parse("1736899200"), None); // 10-digit epoch seconds
        assert_eq!(parse("20250115"), None); // compact YYYYMMDD
        assert_eq!(parse("-1736899200000"), None); // signed epoch
        assert_eq!(parse("1736899200000.0"), None); // fractional epoch
    }

    #[test]
    fn rejects_obvious_garbage() {
        assert_eq!(parse(""), None);
        assert_eq!(parse("not-a-date"), None);
        assert_eq!(parse("2025-13-01"), None);
    }
}
