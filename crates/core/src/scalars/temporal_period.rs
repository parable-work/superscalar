//! Temporal.Period: an ISO 8601 duration, an amount of calendar time written
//! `PnYnMnWnDTnHnMnS`: `P2D`, `P1M`, `P1Y6M`, `PT30M`, `P1DT12H`. Unlike
//! `Temporal.Duration`, which is an exact elapsed time in hours and smaller,
//! a period counts calendar units: `P1M` after 31 January and after 1
//! February are different numbers of days, and `P1D` across a clock change is
//! not 24 hours. What a period comes to is decided where it is added to a
//! date.
//!
//! Shape (the designators in order, each number one to nine ASCII digits, a
//! fraction on the seconds alone) is the catalog's pattern and length rules,
//! borrowed via `DirectiveScalar`. This module refuses what the pattern
//! cannot: a `T` with no time part after it. It then writes the canonical
//! form: no leading zeros, no component that is zero, no trailing zeros in
//! the seconds' fraction, and `PT0S` for a period of nothing. Components are
//! never carried into one another (`PT90M` stays `PT90M`, `P14D` stays
//! `P14D`), because a month is not a number of days.
//!
//! Refused: lowercase designators, a sign (`-P1D`), a fraction on any unit
//! but seconds, a comma as the decimal sign, and the alternative
//! `P0001-02-03T04:05:06` format.

use crate::directive::DirectiveScalar;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use once_cell::sync::Lazy;

// Borrow the catalog's pattern and length bounds rather than restating them.
static ENGINE: Lazy<DirectiveScalar> = Lazy::new(|| {
    DirectiveScalar::from_def(crate::registry::scalar_def(
        crate::catalog::names::TEMPORAL_PERIOD,
    ))
});

pub struct TemporalPeriod;

/// The canonical form of a value the pattern accepts, or the error for a `T`
/// that no time part follows.
fn canonical(valid: &str) -> Result<String, ScalarError> {
    if valid.ends_with('T') {
        return Err(ScalarError::new(
            ErrorKind::Parse,
            format!("a time part follows T in a period: {valid}"),
        ));
    }
    let mut date = String::new();
    let mut time = String::new();
    let mut in_time = false;
    let mut number = String::new();
    // The pattern admits only `P`, `T`, ASCII digits, one `.` and the unit
    // designators, each unit at most once and in order.
    for symbol in valid.chars().skip(1) {
        match symbol {
            'T' => in_time = true,
            '0'..='9' | '.' => number.push(symbol),
            unit => {
                let (whole, fraction) = match number.split_once('.') {
                    Some((whole, fraction)) => (whole, fraction.trim_end_matches('0')),
                    None => (number.as_str(), ""),
                };
                let whole = whole.trim_start_matches('0');
                if !whole.is_empty() || !fraction.is_empty() {
                    let part = if in_time { &mut time } else { &mut date };
                    part.push_str(if whole.is_empty() { "0" } else { whole });
                    if !fraction.is_empty() {
                        part.push('.');
                        part.push_str(fraction);
                    }
                    part.push(unit);
                }
                number.clear();
            }
        }
    }
    Ok(match (date.is_empty(), time.is_empty()) {
        (true, true) => "PT0S".to_string(),
        (_, true) => format!("P{date}"),
        _ => format!("P{date}T{time}"),
    })
}

impl Scalar for TemporalPeriod {
    /// Length, then pattern, then the time part after `T`.
    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        ENGINE.validate(registry, input)?;
        canonical(input).map(|_| ())
    }

    /// The canonical form of a valid value; any other value as written,
    /// because normalize does not enforce shape.
    fn normalize(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        Ok(self
            .parse(registry, input)
            .unwrap_or_else(|_| input.to_string()))
    }

    fn parse(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        ENGINE.validate(registry, input)?;
        canonical(input)
    }
}

#[cfg(test)]
mod tests {
    use super::TemporalPeriod;
    use crate::error::ErrorKind;
    use crate::registry::{Registry, Scalar};

    fn kind(input: &str) -> Option<ErrorKind> {
        TemporalPeriod
            .validate(Registry::builtin(), input)
            .err()
            .map(|error| error.kind)
    }

    #[test]
    fn every_accepted_form_parses_to_the_canonical_form() {
        let registry = Registry::builtin();
        let cases = [
            ("P2D", "P2D"),
            ("P14D", "P14D"),
            ("P1M", "P1M"),
            ("P1Y", "P1Y"),
            ("P2W", "P2W"),
            ("P1Y6M", "P1Y6M"),
            ("P1Y2M3W4DT5H6M7S", "P1Y2M3W4DT5H6M7S"),
            ("PT30M", "PT30M"),
            ("PT90M", "PT90M"),
            ("P1DT12H", "P1DT12H"),
            ("PT1.5S", "PT1.5S"),
            ("PT1.500S", "PT1.5S"),
            ("PT1.0S", "PT1S"),
            ("PT0.5S", "PT0.5S"),
            ("PT00.25S", "PT0.25S"),
            ("P02D", "P2D"),
            ("P1Y0M", "P1Y"),
            ("P0Y1M", "P1M"),
            ("P1DT0H", "P1D"),
            ("P0DT1H", "PT1H"),
            ("P0D", "PT0S"),
            ("PT0S", "PT0S"),
            ("PT0.0S", "PT0S"),
            ("P0Y0M0W0DT0H0M0S", "PT0S"),
            ("P999999999D", "P999999999D"),
        ];
        for (input, canonical) in cases {
            assert_eq!(
                TemporalPeriod.parse(registry, input).as_deref(),
                Ok(canonical),
                "parse({input:?})"
            );
            assert_eq!(
                TemporalPeriod.normalize(registry, input).as_deref(),
                Ok(canonical),
                "normalize({input:?})"
            );
            assert_eq!(
                TemporalPeriod.parse(registry, canonical).as_deref(),
                Ok(canonical),
                "parse({canonical:?})"
            );
        }
    }

    #[test]
    fn rejects_by_kind() {
        assert_eq!(kind(""), Some(ErrorKind::Length));
        assert_eq!(kind("P"), Some(ErrorKind::Length));
        assert_eq!(kind("PT"), Some(ErrorKind::Length));
        assert_eq!(kind("P1DT"), Some(ErrorKind::Parse));
        assert_eq!(kind("P1YT"), Some(ErrorKind::Parse));
        for input in [
            "p2d",
            "P2d",
            "-P1D",
            "+P1D",
            "P-1D",
            "12D",
            "P12",
            "PD2",
            "P1.5D",
            "PT1.5H",
            "PT1,5S",
            "PT1.S",
            "PT.5S",
            "P1D2M",
            "P1M1Y",
            "PT1S1M",
            "P1H",
            "PT1D",
            "P1D12H",
            "P 1D",
            " P1D",
            "P1D ",
            "P1234567890D",
            "PT1.1234567890S",
            "P0001-02-03T04:05:06",
            "P１D",
            "1h30m",
            "30s",
        ] {
            assert_eq!(kind(input), Some(ErrorKind::Pattern), "{input}");
        }
    }

    #[test]
    fn normalize_returns_a_refused_value_as_written() {
        let registry = Registry::builtin();
        assert_eq!(
            TemporalPeriod.normalize(registry, "p2d").as_deref(),
            Ok("p2d")
        );
        assert_eq!(
            TemporalPeriod.normalize(registry, "P1DT").as_deref(),
            Ok("P1DT")
        );
    }
}
