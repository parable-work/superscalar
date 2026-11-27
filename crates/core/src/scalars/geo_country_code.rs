//! Geo.CountryCode: an ISO 3166-1 alpha-2 country code that the standard
//! lists as officially assigned. Shape (exactly two ASCII uppercase letters)
//! is the catalog's length and pattern rules, borrowed via `DirectiveScalar`
//! so they live in one place; membership is `COUNTRY_CODES`.
//!
//! The value is canonical as given: no trimming and no case folding, so `us`
//! is refused rather than rewritten, as `Finance.Currency` refuses `usd`.
//! Codes the standard does not officially assign are refused like any other
//! unknown code: user-assigned ones (`XK`, `AA`, `ZZ`), exceptionally
//! reserved ones (`UK`, `EU`) and withdrawn ones (`AN`, `CS`, `YU`).

use crate::directive::DirectiveScalar;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use once_cell::sync::Lazy;

static ENGINE: Lazy<DirectiveScalar> = Lazy::new(|| {
    DirectiveScalar::from_def(crate::registry::scalar_def(
        crate::catalog::names::GEO_COUNTRY_CODE,
    ))
});

/// Date of the ISO/TC 46 document (N1127) that `COUNTRY_CODES` is current as
/// of. The table is transcribed from the tz database's `iso3166.tab`, release
/// 2026f, which follows that document. A later edition that assigns a code
/// loosens the accept set (a minor bump); one that withdraws a code tightens
/// it (a major bump).
pub const ISO_3166_1_CURRENT_AS_OF: &str = "2024-02-29";

/// The officially assigned ISO 3166-1 alpha-2 codes as of
/// `ISO_3166_1_CURRENT_AS_OF`, sorted.
#[rustfmt::skip]
pub const COUNTRY_CODES: &[&str] = &[
    "AD", "AE", "AF", "AG", "AI", "AL", "AM", "AO", "AQ", "AR", "AS", "AT", "AU",
    "AW", "AX", "AZ", "BA", "BB", "BD", "BE", "BF", "BG", "BH", "BI", "BJ", "BL",
    "BM", "BN", "BO", "BQ", "BR", "BS", "BT", "BV", "BW", "BY", "BZ", "CA", "CC",
    "CD", "CF", "CG", "CH", "CI", "CK", "CL", "CM", "CN", "CO", "CR", "CU", "CV",
    "CW", "CX", "CY", "CZ", "DE", "DJ", "DK", "DM", "DO", "DZ", "EC", "EE", "EG",
    "EH", "ER", "ES", "ET", "FI", "FJ", "FK", "FM", "FO", "FR", "GA", "GB", "GD",
    "GE", "GF", "GG", "GH", "GI", "GL", "GM", "GN", "GP", "GQ", "GR", "GS", "GT",
    "GU", "GW", "GY", "HK", "HM", "HN", "HR", "HT", "HU", "ID", "IE", "IL", "IM",
    "IN", "IO", "IQ", "IR", "IS", "IT", "JE", "JM", "JO", "JP", "KE", "KG", "KH",
    "KI", "KM", "KN", "KP", "KR", "KW", "KY", "KZ", "LA", "LB", "LC", "LI", "LK",
    "LR", "LS", "LT", "LU", "LV", "LY", "MA", "MC", "MD", "ME", "MF", "MG", "MH",
    "MK", "ML", "MM", "MN", "MO", "MP", "MQ", "MR", "MS", "MT", "MU", "MV", "MW",
    "MX", "MY", "MZ", "NA", "NC", "NE", "NF", "NG", "NI", "NL", "NO", "NP", "NR",
    "NU", "NZ", "OM", "PA", "PE", "PF", "PG", "PH", "PK", "PL", "PM", "PN", "PR",
    "PS", "PT", "PW", "PY", "QA", "RE", "RO", "RS", "RU", "RW", "SA", "SB", "SC",
    "SD", "SE", "SG", "SH", "SI", "SJ", "SK", "SL", "SM", "SN", "SO", "SR", "SS",
    "ST", "SV", "SX", "SY", "SZ", "TC", "TD", "TF", "TG", "TH", "TJ", "TK", "TL",
    "TM", "TN", "TO", "TR", "TT", "TV", "TW", "TZ", "UA", "UG", "UM", "US", "UY",
    "UZ", "VA", "VC", "VE", "VG", "VI", "VN", "VU", "WF", "WS", "YE", "YT", "ZA",
    "ZM", "ZW",
];

/// Whether `code` is an officially assigned ISO 3166-1 alpha-2 code. Matching
/// is exact: `us` is not assigned.
pub fn is_assigned(code: &str) -> bool {
    COUNTRY_CODES.binary_search(&code).is_ok()
}

pub struct GeoCountryCode;

impl Scalar for GeoCountryCode {
    /// Length, then pattern, then membership: a value of the right shape that
    /// the standard does not assign fails as `Enum`.
    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        ENGINE.validate(registry, input)?;
        if !is_assigned(input) {
            return Err(ScalarError::new(
                ErrorKind::Enum,
                format!("not an officially assigned ISO 3166-1 alpha-2 country code: {input}"),
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
    use super::{is_assigned, GeoCountryCode, COUNTRY_CODES};
    use crate::error::ErrorKind;
    use crate::registry::{Registry, Scalar};

    fn kind(input: &str) -> Option<ErrorKind> {
        GeoCountryCode
            .validate(Registry::builtin(), input)
            .err()
            .map(|error| error.kind)
    }

    #[test]
    fn table_is_sorted_unique_and_well_formed() {
        assert_eq!(COUNTRY_CODES.len(), 249);
        for pair in COUNTRY_CODES.windows(2) {
            assert!(pair[0] < pair[1], "{} then {}", pair[0], pair[1]);
        }
        for code in COUNTRY_CODES {
            assert_eq!(kind(code), None, "{code} refused");
        }
    }

    /// The catalog docstring names the document date by hand; it must say
    /// what the module says.
    #[test]
    fn docstring_names_the_date() {
        let docstring =
            crate::registry::scalar_def(crate::catalog::names::GEO_COUNTRY_CODE).docstring;
        assert!(docstring.contains(super::ISO_3166_1_CURRENT_AS_OF));
    }

    #[test]
    fn rejects_by_kind() {
        // Right shape, not officially assigned: user-assigned, exceptionally
        // reserved, withdrawn.
        for code in ["XK", "AA", "ZZ", "UK", "EU", "AN", "CS", "YU"] {
            assert_eq!(kind(code), Some(ErrorKind::Enum), "{code}");
            assert!(!is_assigned(code));
        }
        assert_eq!(kind("us"), Some(ErrorKind::Pattern));
        assert_eq!(kind("Us"), Some(ErrorKind::Pattern));
        assert_eq!(kind("U5"), Some(ErrorKind::Pattern));
        assert_eq!(kind("U"), Some(ErrorKind::Length));
        assert_eq!(kind("USA"), Some(ErrorKind::Length));
        assert_eq!(kind(" US"), Some(ErrorKind::Length));
        assert_eq!(kind(""), Some(ErrorKind::Length));
    }

    #[test]
    fn parse_and_normalize_keep_the_code() {
        let registry = Registry::builtin();
        assert_eq!(GeoCountryCode.parse(registry, "DE").unwrap(), "DE");
        assert_eq!(GeoCountryCode.normalize(registry, "de").unwrap(), "de");
        assert!(GeoCountryCode.parse(registry, "de").is_err());
    }
}
