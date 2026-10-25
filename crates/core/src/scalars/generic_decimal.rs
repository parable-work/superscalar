//! Generic.Decimal: an exact decimal number in plain notation, carried as
//! text. The shape (an optional sign, ASCII digits, an optional fraction) is
//! the catalog's pattern, borrowed via `DirectiveScalar` so it lives in one
//! place; this module trims surrounding whitespace first, then rewrites a value
//! of that shape to its canonical form.
//!
//! The canonical form is unique per number: no `+`, no leading zeros in the
//! integer part (zero is `0`), no trailing zeros in the fraction (an all-zero
//! fraction is dropped) and `-` only before a nonzero value. Two inputs that
//! denote the same number canonicalize to the same text, so `1.50` and `1.5`
//! are one value. Scale is a property of the column (`NUMERIC(p,s)`), not of
//! the value.
//!
//! Refused: exponent forms (`1e3`), which are what a float formatter writes and
//! which let a short input stand for an unbounded string of digits; `NaN` and
//! the infinities, which the decimal types of the bindings' languages do not
//! all hold; a bare point (`.5`, `5.`), digit grouping (`1,000`, `1_000`) and
//! non-ASCII digits, which are typing or locale variants rather than the plain
//! form every decimal library writes.

use crate::directive::DirectiveScalar;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use once_cell::sync::Lazy;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

static ENGINE: Lazy<DirectiveScalar> = Lazy::new(|| {
    DirectiveScalar::from_def(crate::registry::scalar_def(
        crate::catalog::names::GENERIC_DECIMAL,
    ))
});

/// The canonical form of `input`: empty input fails as `Empty`, any other
/// input the declared pattern refuses fails as `Pattern`.
fn canonical(registry: &Registry, input: &str) -> Result<String, ScalarError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ScalarError::new(
            ErrorKind::Empty,
            "decimal must not be empty",
        ));
    }
    ENGINE.validate(registry, trimmed)?;

    let (negative, unsigned) = match trimmed.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, trimmed.strip_prefix('+').unwrap_or(trimmed)),
    };
    let (integer, fraction) = unsigned.split_once('.').unwrap_or((unsigned, ""));
    let integer = integer.trim_start_matches('0');
    let fraction = fraction.trim_end_matches('0');

    let mut out = String::with_capacity(trimmed.len() + 1);
    if negative && !(integer.is_empty() && fraction.is_empty()) {
        out.push('-');
    }
    out.push_str(if integer.is_empty() { "0" } else { integer });
    if !fraction.is_empty() {
        out.push('.');
        out.push_str(fraction);
    }
    Ok(out)
}

pub struct GenericDecimal;

impl Scalar for GenericDecimal {
    fn parse(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        canonical(registry, input)
    }

    /// The same as `parse`: a value that is not a decimal has no canonical
    /// form to move toward.
    fn normalize(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        canonical(registry, input)
    }

    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        canonical(registry, input).map(|_| ())
    }
}

/// Typed Generic.Decimal value, the Rust type generated code carries for the
/// scalar (as `Uuid` and `DateTime` are for theirs). It holds the canonical
/// text, so it is exact for every value the scalar accepts, and it serializes
/// as a JSON string of that text, byte for byte what every binding returns.
///
/// It does no arithmetic. Convert through `as_str` into the decimal crate you
/// compute with (`rust_decimal::Decimal::from_str`,
/// `bigdecimal::BigDecimal::from_str`). Equality is numeric equality, because
/// each number has one canonical form. There is no `Ord`: text order is not
/// numeric order.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Decimal(String);

impl Decimal {
    /// Parse with the scalar's accept set into its canonical form.
    pub fn parse(input: &str) -> Result<Self, ScalarError> {
        canonical(Registry::builtin(), input).map(Self)
    }

    /// The canonical text, such as `-12.5`.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl FromStr for Decimal {
    type Err = ScalarError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for Decimal {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Decimal> for String {
    fn from(value: Decimal) -> Self {
        value.0
    }
}

impl Serialize for Decimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

/// Reads a JSON string only. A JSON number is refused: the wire form is a
/// string so that no reader on the way decodes it through a float.
impl<'de> Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DecimalVisitor;

        impl Visitor<'_> for DecimalVisitor {
            type Value = Decimal;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a decimal number as a string, such as \"12.50\"")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Decimal::parse(value).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(DecimalVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::{Decimal, GenericDecimal};
    use crate::error::ErrorKind;
    use crate::registry::{scalar_def, Registry, Scalar};
    use std::str::FromStr;

    fn parse(input: &str) -> Result<String, ErrorKind> {
        GenericDecimal
            .parse(Registry::builtin(), input)
            .map_err(|error| error.kind)
    }

    #[test]
    fn canonical_forms() {
        for (input, want) in [
            ("0", "0"),
            ("00", "0"),
            ("-0", "0"),
            ("+0.000", "0"),
            ("-0.000", "0"),
            ("1.50", "1.5"),
            ("2.000", "2"),
            ("007.50", "7.5"),
            ("+3.25", "3.25"),
            ("-0.0425", "-0.0425"),
            ("0.001", "0.001"),
            ("100", "100"),
            ("-100.0", "-100"),
            (" \t42.10\n", "42.1"),
            (
                "-12345678901234567890.000000000000000000010",
                "-12345678901234567890.00000000000000000001",
            ),
        ] {
            assert_eq!(parse(input).as_deref(), Ok(want), "{input:?}");
        }
    }

    #[test]
    fn refusals_name_the_kind() {
        for input in ["", "   ", "\n"] {
            assert_eq!(parse(input), Err(ErrorKind::Empty), "{input:?}");
        }
        let not_numbers = ["abc", "NaN", "nan", "Infinity", "-Infinity", "inf"];
        let other_notations = ["1e3", "1E-7", "1.5e+2", "0x1F", "$1"];
        let bare_points_and_signs = [".5", "5.", "-", "+", "+-1", "--1", "- 1", "1.2.3"];
        let grouping_and_locale = ["1,000", "1_000", "1 000", "1,5", "\u{0661}", "\u{FF11}"];
        for input in not_numbers
            .iter()
            .chain(&other_notations)
            .chain(&bare_points_and_signs)
            .chain(&grouping_and_locale)
        {
            assert_eq!(parse(input), Err(ErrorKind::Pattern), "{input:?}");
        }
    }

    /// Parse is idempotent, every canonical form passes the declared pattern,
    /// and normalize and validate agree with parse.
    #[test]
    fn canonical_output_is_stable_and_matches_the_declared_pattern() {
        let registry = Registry::builtin();
        let pattern = regex::Regex::new(
            scalar_def(crate::catalog::names::GENERIC_DECIMAL)
                .pattern
                .expect("Generic.Decimal declares a pattern"),
        )
        .expect("pattern compiles");
        for input in ["-0.00", "007.50", "+3", "19.99", "1000", "-0.0425", " 1 "] {
            let once = GenericDecimal.parse(registry, input).expect("parses");
            assert!(pattern.is_match(&once), "{once:?}");
            assert_eq!(GenericDecimal.parse(registry, &once).as_ref(), Ok(&once));
            assert_eq!(
                GenericDecimal.normalize(registry, input).as_ref(),
                Ok(&once)
            );
            assert!(GenericDecimal.validate(registry, input).is_ok());
        }
    }

    /// An integer's decimal form is already canonical.
    #[test]
    fn integers_are_their_own_canonical_form() {
        for value in [0_i64, 1, -1, 10, -10, i64::MAX, i64::MIN] {
            let text = value.to_string();
            assert_eq!(parse(&text).as_deref(), Ok(text.as_str()));
        }
    }

    #[test]
    fn decimal_value_round_trips_as_a_json_string() {
        let value = Decimal::from_str("0012.50").expect("parses");
        assert_eq!(value.as_str(), "12.5");
        assert_eq!(value.to_string(), "12.5");
        assert_eq!(value, Decimal::from_str("12.5").expect("parses"));
        let json = serde_json::to_string(&value).expect("serializes");
        assert_eq!(json, "\"12.5\"");
        let back: Decimal = serde_json::from_str("\"12.500\"").expect("deserializes");
        assert_eq!(back, value);
        assert_eq!(String::from(back), "12.5");
    }

    #[test]
    fn decimal_value_refuses_numbers_and_bad_text() {
        assert!(serde_json::from_str::<Decimal>("12.5").is_err());
        assert!(serde_json::from_str::<Decimal>("\"1e3\"").is_err());
        assert_eq!(
            Decimal::parse("NaN").map_err(|error| error.kind),
            Err(ErrorKind::Pattern)
        );
    }
}
