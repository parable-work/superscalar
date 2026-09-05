//! Boundary contract of the generic directive engine: inclusive range bounds,
//! integer canonicalization, and non-finite float rejection. Exercised through
//! real catalog scalars so the def-driven wiring is covered too.

mod common;

use superscalar::directive::DirectiveScalar;
use superscalar::{scalar_def, ErrorKind, Registry, Scalar, ScalarId};

/// Finance.Money: Int, minimum 0, maximum 2^53-1.
fn money() -> DirectiveScalar {
    DirectiveScalar::from_def(scalar_def(ScalarId::FINANCE_MONEY))
}

/// Generic.Probability: Float, minimum 0.0, maximum 1.0.
fn probability() -> DirectiveScalar {
    DirectiveScalar::from_def(scalar_def(ScalarId::GENERIC_PROBABILITY))
}

#[test]
fn range_bounds_are_inclusive() {
    let m = money();
    assert!(
        m.validate(Registry::builtin(), "0").is_ok(),
        "min is inclusive"
    );
    assert!(
        m.validate(Registry::builtin(), "9007199254740991").is_ok(),
        "max is inclusive"
    );

    let below = m.validate(Registry::builtin(), "-1").unwrap_err();
    assert_eq!(below.kind, ErrorKind::Range);
    let above = m
        .validate(Registry::builtin(), "9007199254740992")
        .unwrap_err();
    assert_eq!(above.kind, ErrorKind::Range);

    let p = probability();
    assert!(p.validate(Registry::builtin(), "0").is_ok());
    assert!(p.validate(Registry::builtin(), "1").is_ok());
    assert_eq!(
        p.validate(Registry::builtin(), "-0.1").unwrap_err().kind,
        ErrorKind::Range
    );
    assert_eq!(
        p.validate(Registry::builtin(), "1.1").unwrap_err().kind,
        ErrorKind::Range
    );
}

#[test]
fn int_normalizes_to_canonical_decimal() {
    let m = money();
    assert_eq!(m.normalize(Registry::builtin(), "007").unwrap(), "7");
    assert_eq!(m.normalize(Registry::builtin(), "-0").unwrap(), "0");
    assert_eq!(m.normalize(Registry::builtin(), "0").unwrap(), "0");
}

#[test]
fn non_finite_float_is_rejected() {
    let p = probability();
    assert_eq!(
        p.validate(Registry::builtin(), "inf").unwrap_err().kind,
        ErrorKind::Parse
    );
    assert_eq!(
        p.validate(Registry::builtin(), "nan").unwrap_err().kind,
        ErrorKind::Parse
    );
}

/// A PatternOnly slug with reserved words. No built-in declares reserved words,
/// so the def is inline; the engine reads it the same way it reads a catalog
/// def.
fn connector_slug() -> DirectiveScalar {
    let mut def = common::def(
        ScalarId(ScalarId::EXTENSION_BLOCK),
        "Acme",
        "Acme.ConnectorSlug",
        superscalar::ScalarTag::PatternOnly,
        Some("^[a-z][a-z0-9-]*$"),
    );
    def.reserved_words = &["identity", "artifacts"];
    DirectiveScalar::from_def(&def)
}

#[test]
fn reserved_words_are_rejected() {
    let s = connector_slug();
    for reserved in ["identity", "artifacts"] {
        let err = s.validate(Registry::builtin(), reserved).unwrap_err();
        assert_eq!(err.kind, ErrorKind::Enum);
        assert!(
            err.message.contains("reserved"),
            "message must contain 'reserved' for cross-language validator mapping, got: {}",
            err.message
        );
        assert_eq!(
            s.parse(Registry::builtin(), reserved).unwrap_err().kind,
            ErrorKind::Enum
        );
    }
}

#[test]
fn non_reserved_slugs_still_validate() {
    let s = connector_slug();
    assert!(s.validate(Registry::builtin(), "anaplan").is_ok());
    // Reserved matching is exact-token, not substring.
    assert!(s.validate(Registry::builtin(), "identity-provider").is_ok());
    assert_eq!(s.parse(Registry::builtin(), "workday").unwrap(), "workday");
}
