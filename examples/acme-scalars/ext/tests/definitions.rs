//! The definitions-only assembly answers every definition question the
//! registry answers, the same way, over the built-ins plus the Acme scalars.

use acme_scalars::{definitions, registry, DEFS, NAME};
use superscalar::{Definitions, Extension};

#[test]
fn definitions_agree_with_the_registry_on_every_lookup() {
    let registry = registry();
    let definitions = definitions();
    assert_eq!(definitions.len(), Definitions::builtin().len() + DEFS.len());
    assert_eq!(definitions.len(), registry.len());
    assert_eq!(
        definitions.names().collect::<Vec<_>>(),
        registry.names().collect::<Vec<_>>()
    );
    let mut probes: Vec<&str> = registry.names().collect();
    probes.extend(["acme.ordernumber", "Acme", "No.Such"]);
    for &name in &probes {
        assert_eq!(
            definitions.def(name).map(std::ptr::from_ref),
            registry.def(name).map(std::ptr::from_ref),
            "def({name:?})"
        );
        assert_eq!(definitions.resolved(name), registry.resolved(name));
        for &other in &probes {
            assert_eq!(
                definitions.comparable_with(name, other),
                registry.comparable_with(name, other),
                "comparable_with({name:?}, {other:?})"
            );
        }
    }
    assert!(definitions.def("acme.ordernumber").is_none());
    assert_eq!(NAME, acme_scalars::AcmeExtension.name());
}
