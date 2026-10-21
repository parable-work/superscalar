//! Finance.Currency: an ISO 4217 alphabetic currency code that the ISO 4217
//! maintenance agency lists as active (List One). Shape (exactly three ASCII
//! uppercase letters) is the catalog's length and pattern rules, borrowed via
//! `DirectiveScalar` so they live in one place; membership is `CURRENCIES`.
//!
//! The value is canonical as given: no trimming and no case folding, so `usd`
//! is refused rather than rewritten, as `Localization.Locale` refuses `en-us`.
//! Codes the agency has withdrawn (List Three, such as `HRK` or `DEM`) are
//! refused like any other unknown code.

use crate::directive::DirectiveScalar;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use once_cell::sync::Lazy;

static ENGINE: Lazy<DirectiveScalar> = Lazy::new(|| {
    DirectiveScalar::from_def(crate::registry::scalar_def(
        crate::catalog::names::FINANCE_CURRENCY,
    ))
});

/// Publication date of the ISO 4217 List One that `CURRENCIES` transcribes.
/// A later list that adds a code loosens the accept set (a minor bump); one
/// that withdraws a code tightens it (a major bump).
pub const ISO_4217_LIST_ONE_PUBLISHED: &str = "2026-09-17";

/// One active ISO 4217 currency.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Currency {
    /// The alphabetic code, such as `USD`.
    pub code: &'static str,
    /// The minor-unit exponent: an amount in minor units divided by
    /// `10^minor_units` is the amount in major units (USD 2, JPY 0, BHD 3).
    /// `None` where the list says N.A.: precious metals, bond-market units,
    /// SDR, `XSU`, `XUA`, `XTS` and `XXX`.
    pub minor_units: Option<u8>,
}

/// The active codes of ISO 4217 List One as published on
/// `ISO_4217_LIST_ONE_PUBLISHED`, one row per code, sorted by code.
pub const CURRENCIES: &[Currency] = &[
    Currency {
        code: "AED",
        minor_units: Some(2),
    },
    Currency {
        code: "AFN",
        minor_units: Some(2),
    },
    Currency {
        code: "ALL",
        minor_units: Some(2),
    },
    Currency {
        code: "AMD",
        minor_units: Some(2),
    },
    Currency {
        code: "AOA",
        minor_units: Some(2),
    },
    Currency {
        code: "ARS",
        minor_units: Some(2),
    },
    Currency {
        code: "AUD",
        minor_units: Some(2),
    },
    Currency {
        code: "AWG",
        minor_units: Some(2),
    },
    Currency {
        code: "AZN",
        minor_units: Some(2),
    },
    Currency {
        code: "BAM",
        minor_units: Some(2),
    },
    Currency {
        code: "BBD",
        minor_units: Some(2),
    },
    Currency {
        code: "BDT",
        minor_units: Some(2),
    },
    Currency {
        code: "BHD",
        minor_units: Some(3),
    },
    Currency {
        code: "BIF",
        minor_units: Some(0),
    },
    Currency {
        code: "BMD",
        minor_units: Some(2),
    },
    Currency {
        code: "BND",
        minor_units: Some(2),
    },
    Currency {
        code: "BOB",
        minor_units: Some(2),
    },
    Currency {
        code: "BOV",
        minor_units: Some(2),
    },
    Currency {
        code: "BRL",
        minor_units: Some(2),
    },
    Currency {
        code: "BSD",
        minor_units: Some(2),
    },
    Currency {
        code: "BTN",
        minor_units: Some(2),
    },
    Currency {
        code: "BWP",
        minor_units: Some(2),
    },
    Currency {
        code: "BYN",
        minor_units: Some(2),
    },
    Currency {
        code: "BZD",
        minor_units: Some(2),
    },
    Currency {
        code: "CAD",
        minor_units: Some(2),
    },
    Currency {
        code: "CDF",
        minor_units: Some(2),
    },
    Currency {
        code: "CHE",
        minor_units: Some(2),
    },
    Currency {
        code: "CHF",
        minor_units: Some(2),
    },
    Currency {
        code: "CHW",
        minor_units: Some(2),
    },
    Currency {
        code: "CLF",
        minor_units: Some(4),
    },
    Currency {
        code: "CLP",
        minor_units: Some(0),
    },
    Currency {
        code: "CNY",
        minor_units: Some(2),
    },
    Currency {
        code: "COP",
        minor_units: Some(2),
    },
    Currency {
        code: "COU",
        minor_units: Some(2),
    },
    Currency {
        code: "CRC",
        minor_units: Some(2),
    },
    Currency {
        code: "CUP",
        minor_units: Some(2),
    },
    Currency {
        code: "CVE",
        minor_units: Some(2),
    },
    Currency {
        code: "CZK",
        minor_units: Some(2),
    },
    Currency {
        code: "DJF",
        minor_units: Some(0),
    },
    Currency {
        code: "DKK",
        minor_units: Some(2),
    },
    Currency {
        code: "DOP",
        minor_units: Some(2),
    },
    Currency {
        code: "DZD",
        minor_units: Some(2),
    },
    Currency {
        code: "EGP",
        minor_units: Some(2),
    },
    Currency {
        code: "ERN",
        minor_units: Some(2),
    },
    Currency {
        code: "ETB",
        minor_units: Some(2),
    },
    Currency {
        code: "EUR",
        minor_units: Some(2),
    },
    Currency {
        code: "FJD",
        minor_units: Some(2),
    },
    Currency {
        code: "FKP",
        minor_units: Some(2),
    },
    Currency {
        code: "GBP",
        minor_units: Some(2),
    },
    Currency {
        code: "GEL",
        minor_units: Some(2),
    },
    Currency {
        code: "GHS",
        minor_units: Some(2),
    },
    Currency {
        code: "GIP",
        minor_units: Some(2),
    },
    Currency {
        code: "GMD",
        minor_units: Some(2),
    },
    Currency {
        code: "GNF",
        minor_units: Some(0),
    },
    Currency {
        code: "GTQ",
        minor_units: Some(2),
    },
    Currency {
        code: "GYD",
        minor_units: Some(2),
    },
    Currency {
        code: "HKD",
        minor_units: Some(2),
    },
    Currency {
        code: "HNL",
        minor_units: Some(2),
    },
    Currency {
        code: "HTG",
        minor_units: Some(2),
    },
    Currency {
        code: "HUF",
        minor_units: Some(2),
    },
    Currency {
        code: "IDR",
        minor_units: Some(2),
    },
    Currency {
        code: "ILS",
        minor_units: Some(2),
    },
    Currency {
        code: "INR",
        minor_units: Some(2),
    },
    Currency {
        code: "IQD",
        minor_units: Some(3),
    },
    Currency {
        code: "IRR",
        minor_units: Some(2),
    },
    Currency {
        code: "ISK",
        minor_units: Some(0),
    },
    Currency {
        code: "JMD",
        minor_units: Some(2),
    },
    Currency {
        code: "JOD",
        minor_units: Some(3),
    },
    Currency {
        code: "JPY",
        minor_units: Some(0),
    },
    Currency {
        code: "KES",
        minor_units: Some(2),
    },
    Currency {
        code: "KGS",
        minor_units: Some(2),
    },
    Currency {
        code: "KHR",
        minor_units: Some(2),
    },
    Currency {
        code: "KMF",
        minor_units: Some(0),
    },
    Currency {
        code: "KPW",
        minor_units: Some(2),
    },
    Currency {
        code: "KRW",
        minor_units: Some(0),
    },
    Currency {
        code: "KWD",
        minor_units: Some(3),
    },
    Currency {
        code: "KYD",
        minor_units: Some(2),
    },
    Currency {
        code: "KZT",
        minor_units: Some(2),
    },
    Currency {
        code: "LAK",
        minor_units: Some(2),
    },
    Currency {
        code: "LBP",
        minor_units: Some(2),
    },
    Currency {
        code: "LKR",
        minor_units: Some(2),
    },
    Currency {
        code: "LRD",
        minor_units: Some(2),
    },
    Currency {
        code: "LSL",
        minor_units: Some(2),
    },
    Currency {
        code: "LYD",
        minor_units: Some(3),
    },
    Currency {
        code: "MAD",
        minor_units: Some(2),
    },
    Currency {
        code: "MDL",
        minor_units: Some(2),
    },
    Currency {
        code: "MGA",
        minor_units: Some(2),
    },
    Currency {
        code: "MKD",
        minor_units: Some(2),
    },
    Currency {
        code: "MMK",
        minor_units: Some(2),
    },
    Currency {
        code: "MNT",
        minor_units: Some(2),
    },
    Currency {
        code: "MOP",
        minor_units: Some(2),
    },
    Currency {
        code: "MRU",
        minor_units: Some(2),
    },
    Currency {
        code: "MUR",
        minor_units: Some(2),
    },
    Currency {
        code: "MVR",
        minor_units: Some(2),
    },
    Currency {
        code: "MWK",
        minor_units: Some(2),
    },
    Currency {
        code: "MXN",
        minor_units: Some(2),
    },
    Currency {
        code: "MXV",
        minor_units: Some(2),
    },
    Currency {
        code: "MYR",
        minor_units: Some(2),
    },
    Currency {
        code: "MZN",
        minor_units: Some(2),
    },
    Currency {
        code: "NAD",
        minor_units: Some(2),
    },
    Currency {
        code: "NGN",
        minor_units: Some(2),
    },
    Currency {
        code: "NIO",
        minor_units: Some(2),
    },
    Currency {
        code: "NOK",
        minor_units: Some(2),
    },
    Currency {
        code: "NPR",
        minor_units: Some(2),
    },
    Currency {
        code: "NZD",
        minor_units: Some(2),
    },
    Currency {
        code: "OMR",
        minor_units: Some(3),
    },
    Currency {
        code: "PAB",
        minor_units: Some(2),
    },
    Currency {
        code: "PEN",
        minor_units: Some(2),
    },
    Currency {
        code: "PGK",
        minor_units: Some(2),
    },
    Currency {
        code: "PHP",
        minor_units: Some(2),
    },
    Currency {
        code: "PKR",
        minor_units: Some(2),
    },
    Currency {
        code: "PLN",
        minor_units: Some(2),
    },
    Currency {
        code: "PYG",
        minor_units: Some(0),
    },
    Currency {
        code: "QAR",
        minor_units: Some(2),
    },
    Currency {
        code: "RON",
        minor_units: Some(2),
    },
    Currency {
        code: "RSD",
        minor_units: Some(2),
    },
    Currency {
        code: "RUB",
        minor_units: Some(2),
    },
    Currency {
        code: "RWF",
        minor_units: Some(0),
    },
    Currency {
        code: "SAR",
        minor_units: Some(2),
    },
    Currency {
        code: "SBD",
        minor_units: Some(2),
    },
    Currency {
        code: "SCR",
        minor_units: Some(2),
    },
    Currency {
        code: "SDG",
        minor_units: Some(2),
    },
    Currency {
        code: "SEK",
        minor_units: Some(2),
    },
    Currency {
        code: "SGD",
        minor_units: Some(2),
    },
    Currency {
        code: "SHP",
        minor_units: Some(2),
    },
    Currency {
        code: "SLE",
        minor_units: Some(2),
    },
    Currency {
        code: "SOS",
        minor_units: Some(2),
    },
    Currency {
        code: "SRD",
        minor_units: Some(2),
    },
    Currency {
        code: "SSP",
        minor_units: Some(2),
    },
    Currency {
        code: "STN",
        minor_units: Some(2),
    },
    Currency {
        code: "SVC",
        minor_units: Some(2),
    },
    Currency {
        code: "SYP",
        minor_units: Some(2),
    },
    Currency {
        code: "SZL",
        minor_units: Some(2),
    },
    Currency {
        code: "THB",
        minor_units: Some(2),
    },
    Currency {
        code: "TJS",
        minor_units: Some(2),
    },
    Currency {
        code: "TMT",
        minor_units: Some(2),
    },
    Currency {
        code: "TND",
        minor_units: Some(3),
    },
    Currency {
        code: "TOP",
        minor_units: Some(2),
    },
    Currency {
        code: "TRY",
        minor_units: Some(2),
    },
    Currency {
        code: "TTD",
        minor_units: Some(2),
    },
    Currency {
        code: "TWD",
        minor_units: Some(2),
    },
    Currency {
        code: "TZS",
        minor_units: Some(2),
    },
    Currency {
        code: "UAH",
        minor_units: Some(2),
    },
    Currency {
        code: "UGX",
        minor_units: Some(0),
    },
    Currency {
        code: "USD",
        minor_units: Some(2),
    },
    Currency {
        code: "USN",
        minor_units: Some(2),
    },
    Currency {
        code: "UYI",
        minor_units: Some(0),
    },
    Currency {
        code: "UYU",
        minor_units: Some(2),
    },
    Currency {
        code: "UYW",
        minor_units: Some(4),
    },
    Currency {
        code: "UZS",
        minor_units: Some(2),
    },
    Currency {
        code: "VED",
        minor_units: Some(2),
    },
    Currency {
        code: "VES",
        minor_units: Some(2),
    },
    Currency {
        code: "VND",
        minor_units: Some(0),
    },
    Currency {
        code: "VUV",
        minor_units: Some(0),
    },
    Currency {
        code: "WST",
        minor_units: Some(2),
    },
    Currency {
        code: "XAD",
        minor_units: Some(2),
    },
    Currency {
        code: "XAF",
        minor_units: Some(0),
    },
    Currency {
        code: "XAG",
        minor_units: None,
    },
    Currency {
        code: "XAU",
        minor_units: None,
    },
    Currency {
        code: "XBA",
        minor_units: None,
    },
    Currency {
        code: "XBB",
        minor_units: None,
    },
    Currency {
        code: "XBC",
        minor_units: None,
    },
    Currency {
        code: "XBD",
        minor_units: None,
    },
    Currency {
        code: "XCD",
        minor_units: Some(2),
    },
    Currency {
        code: "XCG",
        minor_units: Some(2),
    },
    Currency {
        code: "XDR",
        minor_units: None,
    },
    Currency {
        code: "XOF",
        minor_units: Some(0),
    },
    Currency {
        code: "XPD",
        minor_units: None,
    },
    Currency {
        code: "XPF",
        minor_units: Some(0),
    },
    Currency {
        code: "XPT",
        minor_units: None,
    },
    Currency {
        code: "XSU",
        minor_units: None,
    },
    Currency {
        code: "XTS",
        minor_units: None,
    },
    Currency {
        code: "XUA",
        minor_units: None,
    },
    Currency {
        code: "XXX",
        minor_units: None,
    },
    Currency {
        code: "YER",
        minor_units: Some(2),
    },
    Currency {
        code: "ZAR",
        minor_units: Some(2),
    },
    Currency {
        code: "ZMW",
        minor_units: Some(2),
    },
    Currency {
        code: "ZWG",
        minor_units: Some(2),
    },
];

/// The active currency for `code`, or `None` when `code` is not on List One.
/// Matching is exact: `usd` finds nothing.
pub fn lookup(code: &str) -> Option<&'static Currency> {
    CURRENCIES
        .binary_search_by(|currency| currency.code.cmp(code))
        .ok()
        .map(|index| &CURRENCIES[index])
}

/// The minor-unit exponent for `code`. `None` when the code is not active or
/// the list gives it no minor unit; `lookup` tells the two apart.
pub fn minor_units(code: &str) -> Option<u8> {
    lookup(code).and_then(|currency| currency.minor_units)
}

pub struct FinanceCurrency;

impl Scalar for FinanceCurrency {
    /// Length, then pattern, then membership: a value of the right shape that
    /// List One does not carry fails as `Enum`.
    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        ENGINE.validate(registry, input)?;
        if lookup(input).is_none() {
            return Err(ScalarError::new(
                ErrorKind::Enum,
                format!("not an active ISO 4217 currency code: {input}"),
            ));
        }
        Ok(())
    }

    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        Ok(input.to_string())
    }

    fn parse(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.validate(registry, input)?;
        Ok(input.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{lookup, minor_units, FinanceCurrency, CURRENCIES};
    use crate::error::ErrorKind;
    use crate::registry::{Registry, Scalar};

    fn kind(input: &str) -> Option<ErrorKind> {
        FinanceCurrency
            .validate(Registry::builtin(), input)
            .err()
            .map(|error| error.kind)
    }

    #[test]
    fn table_is_sorted_unique_and_well_formed() {
        assert_eq!(CURRENCIES.len(), 178);
        for pair in CURRENCIES.windows(2) {
            assert!(
                pair[0].code < pair[1].code,
                "{} then {}",
                pair[0].code,
                pair[1].code
            );
        }
        for currency in CURRENCIES {
            assert_eq!(kind(currency.code), None, "{} refused", currency.code);
            assert!(currency.minor_units.is_none_or(|units| units <= 4));
        }
    }

    #[test]
    fn minor_units_cover_each_exponent() {
        assert_eq!(minor_units("USD"), Some(2));
        assert_eq!(minor_units("EUR"), Some(2));
        assert_eq!(minor_units("JPY"), Some(0));
        assert_eq!(minor_units("BHD"), Some(3));
        assert_eq!(minor_units("CLF"), Some(4));
        assert_eq!(minor_units("XAU"), None);
        assert!(lookup("XAU").is_some());
        assert_eq!(minor_units("HRK"), None);
        assert!(lookup("HRK").is_none());
    }

    /// The catalog docstring lists the exponents by hand; it must say what
    /// the table says, and name the list's date.
    #[test]
    fn docstring_matches_the_table() {
        let docstring = crate::registry::scalar_def(crate::catalog::names::FINANCE_CURRENCY)
            .docstring
            .replace('\n', " ");
        assert!(docstring.contains(super::ISO_4217_LIST_ONE_PUBLISHED));
        let codes = |units: Option<u8>| {
            CURRENCIES
                .iter()
                .filter(|currency| currency.minor_units == units)
                .map(|currency| currency.code)
                .collect::<Vec<_>>()
                .join(" ")
        };
        for (label, units) in [
            ("0 for", Some(0)),
            ("3 for", Some(3)),
            ("4 for", Some(4)),
            ("none (N.A.) for", None),
        ] {
            let expected = format!("{label} {}", codes(units));
            assert!(
                docstring.contains(&expected),
                "docstring lacks {expected:?}"
            );
        }
        let listed = CURRENCIES
            .iter()
            .filter(|currency| currency.minor_units != Some(2))
            .count();
        assert_eq!(listed, 39, "every code that is not 2 is listed above");
    }

    #[test]
    fn rejects_by_kind() {
        assert_eq!(kind("ABC"), Some(ErrorKind::Enum));
        assert_eq!(kind("HRK"), Some(ErrorKind::Enum));
        assert_eq!(kind("BGN"), Some(ErrorKind::Enum));
        assert_eq!(kind("usd"), Some(ErrorKind::Pattern));
        assert_eq!(kind("Usd"), Some(ErrorKind::Pattern));
        assert_eq!(kind("U5D"), Some(ErrorKind::Pattern));
        assert_eq!(kind("US"), Some(ErrorKind::Length));
        assert_eq!(kind("USDX"), Some(ErrorKind::Length));
        assert_eq!(kind(" USD"), Some(ErrorKind::Length));
        assert_eq!(kind(""), Some(ErrorKind::Length));
    }

    #[test]
    fn parse_and_normalize_keep_the_code() {
        let registry = Registry::builtin();
        assert_eq!(FinanceCurrency.parse(registry, "JPY").unwrap(), "JPY");
        assert_eq!(FinanceCurrency.normalize(registry, "usd").unwrap(), "usd");
        assert!(FinanceCurrency.parse(registry, "usd").is_err());
    }
}
