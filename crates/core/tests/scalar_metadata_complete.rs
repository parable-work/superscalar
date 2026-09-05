//! The engine-facing metadata table covers every catalogued scalar except the
//! ones whose def sets `metadata_omit`, and lookups are exact and case-sensitive.

use superscalar::{scalar_metadata_by_canonical_name, Registry, SCALAR_METADATA};

/// No built-in sets `metadata_omit` today, so the table covers the whole
/// catalog. The loop still honours the flag so a future omitted def is checked
/// for absence rather than presence.
#[test]
fn metadata_covers_every_scalar_but_the_omitted_ones() {
    let registry = Registry::builtin();
    let omitted: Vec<&str> = registry
        .defs()
        .filter(|def| def.metadata_omit)
        .map(|def| def.canonical)
        .collect();
    assert!(
        omitted.is_empty(),
        "unexpected metadata_omit defs: {omitted:?}"
    );

    let expected = registry.defs().filter(|def| !def.metadata_omit).count();
    assert_eq!(SCALAR_METADATA.len(), expected);

    for def in registry.defs() {
        let found = scalar_metadata_by_canonical_name(def.canonical);
        if def.metadata_omit {
            assert!(found.is_none(), "{} must stay excluded", def.canonical);
            continue;
        }
        let md = found.unwrap_or_else(|| panic!("no metadata for {}", def.canonical));
        assert_eq!(md.canonical_name, def.canonical);
    }
}

#[test]
fn lookup_is_exact_and_case_sensitive() {
    assert!(scalar_metadata_by_canonical_name("Not.AReal.Scalar").is_none());
    assert!(scalar_metadata_by_canonical_name("contact.email").is_none());
    assert!(scalar_metadata_by_canonical_name("Contact.Email").is_some());
}
