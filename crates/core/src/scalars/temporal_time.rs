//! Temporal.Time canonical = 24-hour `HH:MM:SS`, so `1:05pm`, `1:05 PM` and
//! `13:05` all parse to `13:05:00`. One stored form lets a consumer read the
//! value with a strict time decoder; some refuse 12-hour text, such as
//! `1:05pm` with no space before the suffix. Validation (the pattern) stays
//! the catalog's, borrowed via `DirectiveScalar`, so this impl accepts exactly
//! the values the pattern accepts and decides only their canonical form.

use crate::directive::DirectiveScalar;
use crate::error::ScalarError;
use crate::registry::{Registry, Scalar};
use once_cell::sync::Lazy;

// Borrow the catalog's pattern rather than restating it here.
static ENGINE: Lazy<DirectiveScalar> = Lazy::new(|| {
    DirectiveScalar::from_def(crate::registry::scalar_def(
        crate::catalog::names::TEMPORAL_TIME,
    ))
});

pub struct TemporalTime;

impl TemporalTime {
    /// The 24-hour `HH:MM:SS` of a value the pattern accepts. A 12-hour value
    /// ends in AM or PM in any case, with or without whitespace before it:
    /// `12:MM AM` is `00:MM`, and `12:MM PM` stays `12:MM`.
    fn canonical(valid: &str) -> String {
        let suffix = valid.len() - 2;
        let (clock, pm) = match valid.get(suffix..) {
            Some(meridiem) if meridiem.eq_ignore_ascii_case("am") => {
                (valid[..suffix].trim_end(), Some(false))
            }
            Some(meridiem) if meridiem.eq_ignore_ascii_case("pm") => {
                (valid[..suffix].trim_end(), Some(true))
            }
            _ => (valid, None),
        };
        // The pattern admits only ASCII digits in each part, so each parse
        // succeeds.
        let mut parts = clock
            .split(':')
            .map(|part| part.parse::<u32>().unwrap_or(0));
        let hour = parts.next().unwrap_or(0);
        let minute = parts.next().unwrap_or(0);
        let second = parts.next().unwrap_or(0);
        let hour = match pm {
            Some(pm) => hour % 12 + if pm { 12 } else { 0 },
            None => hour,
        };
        format!("{hour:02}:{minute:02}:{second:02}")
    }
}

impl Scalar for TemporalTime {
    /// The canonical form of a valid value. A value the pattern refuses is
    /// returned as written, as the directive engine's normalize does, because
    /// normalize does not enforce shape.
    fn normalize(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        Ok(match ENGINE.validate(registry, input) {
            Ok(()) => Self::canonical(input),
            Err(_) => input.to_string(),
        })
    }

    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        ENGINE.validate(registry, input)
    }

    fn parse(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        ENGINE.validate(registry, input)?;
        Ok(Self::canonical(input))
    }
}

#[cfg(test)]
mod tests {
    use super::TemporalTime;
    use crate::error::ErrorKind;
    use crate::registry::{Registry, Scalar};

    /// Every form the pattern accepts parses to its 24-hour `HH:MM:SS`:
    /// 24-hour with and without seconds, and 12-hour with and without
    /// seconds, a leading zero, and whitespace before AM or PM, in any case.
    #[test]
    fn every_accepted_form_parses_to_24_hour_hh_mm_ss() {
        let registry = Registry::builtin();
        let cases = [
            ("12:25", "12:25:00"),
            ("12:25:30", "12:25:30"),
            ("00:00", "00:00:00"),
            ("09:30", "09:30:00"),
            ("23:59:59", "23:59:59"),
            ("1:05 PM", "13:05:00"),
            ("1:05PM", "13:05:00"),
            ("1:05 pm", "13:05:00"),
            ("1:05pm", "13:05:00"),
            ("1:05 Pm", "13:05:00"),
            ("1:05pM", "13:05:00"),
            ("1:05\tPM", "13:05:00"),
            // The pattern's `\s` is any Unicode whitespace.
            ("1:05\u{a0}PM", "13:05:00"),
            ("01:05 AM", "01:05:00"),
            ("01:05am", "01:05:00"),
            ("1:05 am", "01:05:00"),
            ("1:05:30 PM", "13:05:30"),
            ("1:05:30pm", "13:05:30"),
            ("01:05:30 AM", "01:05:30"),
            ("01:05:30am", "01:05:30"),
            ("11:59:59 PM", "23:59:59"),
            ("12:00 AM", "00:00:00"),
            ("12:30:15am", "00:30:15"),
            ("12:00 PM", "12:00:00"),
            ("12:00pm", "12:00:00"),
        ];
        for (input, canonical) in cases {
            assert_eq!(
                TemporalTime.parse(registry, input).as_deref(),
                Ok(canonical),
                "parse({input:?})"
            );
            assert_eq!(
                TemporalTime.normalize(registry, input).as_deref(),
                Ok(canonical),
                "normalize({input:?})"
            );
            // The canonical form is a value of the scalar and parses to
            // itself.
            assert_eq!(
                TemporalTime.parse(registry, canonical).as_deref(),
                Ok(canonical),
                "parse({canonical:?})"
            );
        }
    }

    /// A value the pattern refuses fails parse with the pattern error it had
    /// before, and normalize returns it as written.
    #[test]
    fn a_refused_value_keeps_the_pattern_error() {
        let registry = Registry::builtin();
        for input in [
            "25:00",
            "9:30",
            "13:05 PM",
            "0:30 AM",
            "1:05",
            "12:25:30.5",
            " 12:25",
            "1:05 P",
        ] {
            let error = TemporalTime.parse(registry, input).expect_err(input);
            assert_eq!(error.kind, ErrorKind::Pattern, "parse({input:?})");
            assert!(
                TemporalTime.validate(registry, input).is_err(),
                "validate({input:?})"
            );
            assert_eq!(
                TemporalTime.normalize(registry, input).as_deref(),
                Ok(input)
            );
        }
    }
}
