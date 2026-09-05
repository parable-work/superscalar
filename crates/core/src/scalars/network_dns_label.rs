//! Network.DnsLabel canonical = trim, strip a leading http(s) scheme and any
//! path, then take the first label of a pasted multi-label hostname, so
//! "https://acme.okta.com/admin" becomes "acme". Validation (pattern + max
//! length) stays the catalog's, borrowed via `DirectiveScalar` so the label
//! pattern lives in exactly one place.

use crate::catalog::ScalarId;
use crate::directive::DirectiveScalar;
use crate::error::ScalarError;
use crate::registry::{Registry, Scalar};
use once_cell::sync::Lazy;

// Borrow the catalog's pattern/length rules rather than restating them here.
static ENGINE: Lazy<DirectiveScalar> = Lazy::new(|| {
    DirectiveScalar::from_def(crate::registry::scalar_def(ScalarId::NETWORK_DNS_LABEL))
});

pub struct NetworkDnsLabelScalar;

impl NetworkDnsLabelScalar {
    /// Trim; keep an already-valid label unchanged; otherwise strip a leading
    /// http(s) scheme (case-insensitive detection), cut at the first `/`, `?`,
    /// or `#`, drop trailing `/` and `.` runs, and fall back to the first
    /// `.`-separated label of a plain multi-label hostname. A port or any
    /// character outside `[A-Za-z0-9.-]` disqualifies extraction; casing is
    /// preserved throughout.
    fn canonical(input: &str) -> String {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return String::new();
        }
        if ENGINE.validate(Registry::builtin(), trimmed).is_ok() {
            return trimmed.to_string();
        }
        let lower = trimmed.to_ascii_lowercase();
        let mut host = trimmed;
        if lower.starts_with("http://") {
            host = &trimmed["http://".len()..];
        } else if lower.starts_with("https://") {
            host = &trimmed["https://".len()..];
        }
        if let Some(cut) = host.find(['/', '?', '#']) {
            host = &host[..cut];
        }
        let host = host.trim_end_matches(['/', '.']);
        if ENGINE.validate(Registry::builtin(), host).is_ok() {
            return host.to_string();
        }
        if host.contains('.')
            && host
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
        {
            if let Some(label) = host.split('.').next() {
                if ENGINE.validate(Registry::builtin(), label).is_ok() {
                    return label.to_string();
                }
            }
        }
        trimmed.to_string()
    }
}

impl Scalar for NetworkDnsLabelScalar {
    fn id(&self) -> ScalarId {
        ScalarId::NETWORK_DNS_LABEL
    }

    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        Ok(Self::canonical(input))
    }

    /// Strict validation of the input as given (no extraction) -- a pasted
    /// hostname is not a valid Network.DnsLabel. The rescue lives only in
    /// `normalize`/`parse`.
    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        ENGINE.validate(registry, input)
    }

    fn parse(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        let canonical = Self::canonical(input);
        ENGINE.validate(registry, &canonical)?;
        Ok(canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::NetworkDnsLabelScalar;
    use crate::registry::{Registry, Scalar};

    fn s() -> NetworkDnsLabelScalar {
        NetworkDnsLabelScalar
    }

    #[test]
    fn normalize_extracts_first_label_from_pasted_host_or_url() {
        assert_eq!(
            s().normalize(Registry::builtin(), "acme.okta.com").unwrap(),
            "acme"
        );
        assert_eq!(
            s().normalize(Registry::builtin(), "  acme.okta.com  ")
                .unwrap(),
            "acme"
        );
        assert_eq!(
            s().normalize(Registry::builtin(), "https://Sunrun.my.salesforce.com/path")
                .unwrap(),
            "Sunrun"
        );
        assert_eq!(
            s().normalize(Registry::builtin(), "http://x.io/?q=1")
                .unwrap(),
            "x"
        );
        assert_eq!(
            s().normalize(Registry::builtin(), "acme.okta.com.")
                .unwrap(),
            "acme"
        );
    }

    #[test]
    fn normalize_preserves_passing_value() {
        assert_eq!(
            s().normalize(Registry::builtin(), "mycompany").unwrap(),
            "mycompany"
        );
        assert_eq!(
            s().normalize(Registry::builtin(), "acme-corp").unwrap(),
            "acme-corp"
        );
        assert_eq!(s().normalize(Registry::builtin(), "A1").unwrap(), "A1");
    }

    #[test]
    fn normalize_empty_stays_empty() {
        assert_eq!(s().normalize(Registry::builtin(), "").unwrap(), "");
        assert_eq!(s().normalize(Registry::builtin(), "   ").unwrap(), "");
    }

    #[test]
    fn port_disqualifies_extraction() {
        assert_eq!(
            s().normalize(Registry::builtin(), "acme.okta.com:8080")
                .unwrap(),
            "acme.okta.com:8080"
        );
        assert!(s()
            .parse(Registry::builtin(), "acme.okta.com:8080")
            .is_err());
    }

    #[test]
    fn parse_rejects_junk_unchanged() {
        assert!(s().parse(Registry::builtin(), "not_a_domain!").is_err());
        assert!(s().parse(Registry::builtin(), "").is_err());
        assert!(s().parse(Registry::builtin(), "-acme").is_err());
        assert!(s().parse(Registry::builtin(), "acme-").is_err());
    }

    #[test]
    fn validate_stays_strict_on_raw_input() {
        assert!(s().validate(Registry::builtin(), "acme.okta.com").is_err());
        assert!(s().validate(Registry::builtin(), "acme").is_ok());
        assert!(s().validate(Registry::builtin(), &"a".repeat(63)).is_ok());
        assert!(s().validate(Registry::builtin(), &"a".repeat(64)).is_err());
    }
}
