//! The definitions-only assembly answers every definition question the
//! registry answers, the same way, over the built-ins plus the Acme block.

use acme_scalars::{definitions, registry, DEFS, NAME};
use superscalar::{Definitions, Extension, ScalarId};

#[test]
fn definitions_agree_with_the_registry_on_every_lookup() {
    let registry = registry();
    let definitions = definitions();
    assert_eq!(definitions.len(), Definitions::builtin().len() + DEFS.len());
    assert_eq!(definitions.len(), registry.len());
    assert_eq!(
        definitions.ids().collect::<Vec<_>>(),
        registry.ids().collect::<Vec<_>>()
    );
    let mut probes: Vec<ScalarId> = registry.ids().collect();
    probes.extend([ScalarId(0), ScalarId(4095), ScalarId(8191)]);
    for &id in &probes {
        assert_eq!(
            definitions.def(id).map(std::ptr::from_ref),
            registry.def(id).map(std::ptr::from_ref),
            "def({})",
            id.0
        );
        assert_eq!(definitions.resolved(id), registry.resolved(id));
        for &other in &probes {
            assert_eq!(
                definitions.comparable_with(id, other),
                registry.comparable_with(id, other),
                "comparable_with({}, {})",
                id.0,
                other.0
            );
        }
    }
    for def in registry.defs() {
        assert_eq!(
            definitions.by_canonical(def.canonical).map(|d| d.id),
            Some(def.id)
        );
    }
    assert!(definitions.by_canonical("acme.ordernumber").is_none());
    assert_eq!(NAME, acme_scalars::AcmeExtension.name());
}
