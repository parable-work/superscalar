//! The assembled registry reproduces both vector files: the built-in corpus
//! and the Acme corpus. Accepted inputs parse to their declared canonical form;
//! rejected inputs fail with the declared error category. The two files are
//! merged the way every binding runner merges them: disjoint scalar keys,
//! metadata rows checked against the defs.

use acme_scalars::{registry, AcmeExtension, ORDER_NUMBER, SCALAR_REF};
use serde::Deserialize;
use std::collections::BTreeMap;
use superscalar::{ErrorKind, Extension, Registry, ScalarId};

const CORE_VECTORS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../conformance/core-scalars.v2.json"
));
const ACME_VECTORS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../conformance/acme-scalars.v2.json"
));

#[derive(Deserialize)]
struct Vectors {
    scalars: BTreeMap<String, Cases>,
    #[serde(default)]
    metadata: BTreeMap<String, MetadataVector>,
    #[serde(default)]
    metadata_excluded: Vec<String>,
}

#[derive(Deserialize)]
struct MetadataVector {
    comparability_class: Option<String>,
    is_sortable: bool,
}

#[derive(Deserialize, Default)]
struct Cases {
    #[serde(default)]
    accepted: Vec<Accepted>,
    #[serde(default)]
    rejected: Vec<Rejected>,
}

#[derive(Deserialize)]
struct Accepted {
    input: String,
    normalized: Option<String>,
    #[serde(default)]
    unresolved: bool,
}

#[derive(Deserialize)]
struct Rejected {
    input: String,
    validator: Option<String>,
    #[serde(default)]
    unresolved: bool,
}

fn merged() -> Vectors {
    let mut core: Vectors = serde_json::from_str(CORE_VECTORS).expect("core vectors parse");
    let acme: Vectors = serde_json::from_str(ACME_VECTORS).expect("acme vectors parse");
    for (canonical, cases) in acme.scalars {
        assert!(
            core.scalars.insert(canonical.clone(), cases).is_none(),
            "{canonical} appears in both vector files"
        );
    }
    for (canonical, row) in acme.metadata {
        assert!(
            core.metadata.insert(canonical.clone(), row).is_none(),
            "{canonical} has a metadata row in both vector files"
        );
    }
    core.metadata_excluded.extend(acme.metadata_excluded);
    core
}

#[test]
fn assembled_registry_reproduces_every_vector() {
    let registry = registry();
    let vectors = merged();
    let mut failures: Vec<String> = Vec::new();
    let mut checked = 0usize;

    for (canonical, cases) in &vectors.scalars {
        let Some(def) = registry.by_canonical(canonical) else {
            failures.push(format!("unknown canonical scalar: {canonical}"));
            continue;
        };
        let scalar = registry.scalar(def.id).expect("assembled id has a scalar");
        for case in cases.accepted.iter().filter(|c| !c.unresolved) {
            checked += 1;
            match scalar.parse(registry, &case.input) {
                Ok(got) => {
                    if let Some(expected) = case.normalized.as_ref().filter(|&e| &got != e) {
                        failures.push(format!(
                            "{canonical}: parse({:?}) = {got:?}, expected {expected:?}",
                            case.input
                        ));
                    }
                }
                Err(err) => failures.push(format!(
                    "{canonical}: parse({:?}) rejected ({err}), expected accept",
                    case.input
                )),
            }
        }
        for case in cases.rejected.iter().filter(|c| !c.unresolved) {
            checked += 1;
            match scalar.parse(registry, &case.input) {
                Ok(got) => failures.push(format!(
                    "{canonical}: parse({:?}) = {got:?}, expected reject",
                    case.input
                )),
                Err(err) => {
                    if let Some(expected) =
                        case.validator.as_ref().filter(|&e| err.kind.as_str() != e)
                    {
                        failures.push(format!(
                            "{canonical}: parse({:?}) failed as {:?}, expected {expected:?}",
                            case.input,
                            err.kind.as_str()
                        ));
                    }
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} conformance failures (of {checked} checked):\n{}",
        failures.len(),
        failures.join("\n")
    );
    let expected: usize = vectors
        .scalars
        .values()
        .map(|cases| {
            cases.accepted.iter().filter(|c| !c.unresolved).count()
                + cases.rejected.iter().filter(|c| !c.unresolved).count()
        })
        .sum();
    assert_eq!(checked, expected, "every merged vector is checked");
    assert!(checked > 0, "no vectors checked");
}

#[test]
fn every_assembled_scalar_has_vectors_and_every_vector_a_scalar() {
    let registry = registry();
    let vectors = merged();
    let missing: Vec<&str> = registry
        .defs()
        .map(|def| def.canonical)
        .filter(|canonical| !vectors.scalars.contains_key(*canonical))
        .collect();
    assert!(missing.is_empty(), "scalars without vectors: {missing:?}");
    let unknown: Vec<&String> = vectors
        .scalars
        .keys()
        .filter(|canonical| registry.by_canonical(canonical).is_none())
        .collect();
    assert!(unknown.is_empty(), "vectors without a scalar: {unknown:?}");
}

#[test]
fn metadata_rows_match_the_defs() {
    let registry = registry();
    let vectors = merged();
    for def in registry.defs() {
        let row = vectors.metadata.get(def.canonical);
        if def.metadata_omit {
            assert!(
                row.is_none(),
                "{} is metadata_omit but has a row",
                def.canonical
            );
            assert!(
                vectors
                    .metadata_excluded
                    .contains(&def.canonical.to_string()),
                "{} is metadata_omit but not listed in metadata_excluded",
                def.canonical
            );
            continue;
        }
        let row = row.unwrap_or_else(|| panic!("{} has no metadata row", def.canonical));
        assert_eq!(
            row.comparability_class.as_deref(),
            def.comparability_class,
            "{} comparability_class",
            def.canonical
        );
        assert_eq!(
            row.is_sortable,
            def.is_sortable(),
            "{} is_sortable",
            def.canonical
        );
    }
}

#[test]
fn acme_scalars_sit_in_their_block_after_the_builtins() {
    let registry = registry();
    assert_eq!(
        registry.len(),
        Registry::builtin().len() + AcmeExtension.defs().len()
    );
    assert_eq!(
        registry.by_canonical("Acme.OrderNumber").map(|d| d.id),
        Some(ORDER_NUMBER)
    );
    assert_eq!(
        registry.by_canonical("Acme.ScalarRef").map(|d| d.id),
        Some(SCALAR_REF)
    );
    assert_eq!(ORDER_NUMBER, ScalarId(4096));
    assert_eq!(SCALAR_REF, ScalarId(4097));
    assert_eq!(registry.owner(ORDER_NUMBER), Some("acme"));
    assert_eq!(registry.owner(ScalarId::CONTACT_EMAIL), Some("builtin"));
    let names: Vec<&str> = registry.extensions().iter().map(|e| e.name).collect();
    assert_eq!(names, ["builtin", "acme"]);
}

/// The registry-consulting scalar proves which registry a caller dispatched
/// into: the same impl accepts `Acme.OrderNumber` through the assembly and
/// rejects it through the built-in registry.
#[test]
fn scalar_ref_accept_set_is_the_assembly() {
    let registry = registry();
    let scalar_ref = registry.scalar(SCALAR_REF).expect("assembled");
    assert!(scalar_ref.validate(registry, "Acme.OrderNumber").is_ok());
    assert!(scalar_ref.validate(registry, "Contact.Email").is_ok());
    let rejected = scalar_ref
        .validate(registry, "Nope.Nope")
        .expect_err("unknown name rejects");
    assert_eq!(rejected.kind, ErrorKind::Enum);
    assert!(scalar_ref
        .validate(Registry::builtin(), "Acme.OrderNumber")
        .is_err());
}

#[test]
fn directive_scalar_runs_from_the_def_alone() {
    let registry = registry();
    let order = registry.scalar(ORDER_NUMBER).expect("assembled");
    assert!(order.is_directive());
    assert_eq!(order.parse(registry, "ORD-000123").unwrap(), "ORD-000123");
    assert_eq!(
        order.parse(registry, "ORD-12").unwrap_err().kind,
        ErrorKind::Length
    );
}

#[test]
fn dump_lists_the_acme_scalars_under_their_owner() {
    let dump = registry().dump();
    let scalars = dump["scalars"].as_array().expect("array");
    let acme: Vec<&serde_json::Value> = scalars
        .iter()
        .filter(|s| s["extension"] == "acme")
        .collect();
    assert_eq!(acme.len(), AcmeExtension.defs().len());
    assert_eq!(acme[0]["canonical"], "Acme.OrderNumber");
    assert_eq!(acme[0]["id"], 4096);
    assert_eq!(acme[0]["is_directive"], true);
    assert_eq!(acme[1]["canonical"], "Acme.ScalarRef");
    assert_eq!(acme[1]["is_directive"], false);
}
