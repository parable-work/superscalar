//! Network.Url canonical = trim + prepend `https://` when no http(s) scheme is
//! present, so the common "acme.com" / "www.acme.com" input becomes a valid
//! absolute URL. Validation (pattern + max length) stays the catalog's, borrowed
//! via `DirectiveScalar` so the URL pattern lives in exactly one place.

use crate::catalog::ScalarId;
use crate::directive::DirectiveScalar;
use crate::error::ScalarError;
use crate::registry::{Registry, Scalar};
use once_cell::sync::Lazy;

// Borrow the catalog's pattern/length rules rather than restating them here.
static ENGINE: Lazy<DirectiveScalar> =
    Lazy::new(|| DirectiveScalar::from_def(crate::registry::scalar_def(ScalarId::NETWORK_URL)));

pub struct NetworkUrlScalar;

impl NetworkUrlScalar {
    /// Trim, and prepend `https://` when the input carries no http(s) scheme.
    /// Empty / whitespace-only input stays empty (a caller's "no URL"); scheme
    /// detection is case-insensitive but the input's own casing is preserved.
    fn canonical(input: &str) -> String {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return String::new();
        }
        let lower = trimmed.to_ascii_lowercase();
        if lower.starts_with("http://") || lower.starts_with("https://") {
            return trimmed.to_string();
        }
        format!("https://{trimmed}")
    }
}

impl Scalar for NetworkUrlScalar {
    fn id(&self) -> ScalarId {
        ScalarId::NETWORK_URL
    }

    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        Ok(Self::canonical(input))
    }

    /// Strict validation of the input as given (no prepend) -- preserves the
    /// platform-wide `validateNetworkUrl` contract that a bare host is not a
    /// valid Network.Url. The prepend lives only in `normalize`/`parse`.
    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        ENGINE.validate(registry, input)
    }

    /// Normalize toward canonical (prepend https), then validate the result, so
    /// "acme.com" parses to "https://acme.com" while still rejecting junk.
    fn parse(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        let canonical = Self::canonical(input);
        ENGINE.validate(registry, &canonical)?;
        Ok(canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::NetworkUrlScalar;
    use crate::registry::{Registry, Scalar};

    fn s() -> NetworkUrlScalar {
        NetworkUrlScalar
    }

    #[test]
    fn normalize_prepends_https_when_scheme_absent() {
        assert_eq!(
            s().normalize(Registry::builtin(), "acme.com").unwrap(),
            "https://acme.com"
        );
        assert_eq!(
            s().normalize(Registry::builtin(), "www.acme.com").unwrap(),
            "https://www.acme.com"
        );
    }

    #[test]
    fn normalize_preserves_existing_scheme_and_trims() {
        assert_eq!(
            s().normalize(Registry::builtin(), "  http://x.com  ")
                .unwrap(),
            "http://x.com"
        );
        assert_eq!(
            s().normalize(Registry::builtin(), "https://x.com").unwrap(),
            "https://x.com"
        );
        // Scheme detection is case-insensitive: do not double-prepend.
        assert_eq!(
            s().normalize(Registry::builtin(), "HTTPS://x.com").unwrap(),
            "HTTPS://x.com"
        );
    }

    #[test]
    fn normalize_empty_stays_empty() {
        assert_eq!(s().normalize(Registry::builtin(), "").unwrap(), "");
        assert_eq!(s().normalize(Registry::builtin(), "   ").unwrap(), "");
    }

    #[test]
    fn parse_accepts_scheme_less_dotted_host() {
        assert_eq!(
            s().parse(Registry::builtin(), "acme.com").unwrap(),
            "https://acme.com"
        );
        assert_eq!(
            s().parse(Registry::builtin(), "www.example.com/path")
                .unwrap(),
            "https://www.example.com/path"
        );
    }

    #[test]
    fn parse_rejects_single_label_even_after_prepend() {
        // "noturl" -> "https://noturl": pattern needs a dotted host, so it stays rejected.
        assert!(s().parse(Registry::builtin(), "noturl").is_err());
        assert!(s().parse(Registry::builtin(), "").is_err());
    }

    #[test]
    fn validate_stays_strict_on_raw_input() {
        // validate does NOT prepend: a bare host is still not a valid Network.Url.
        assert!(s().validate(Registry::builtin(), "acme.com").is_err());
        assert!(s()
            .validate(Registry::builtin(), "https://acme.com")
            .is_ok());
    }
}
