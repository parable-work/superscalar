//! A complete downstream extension, assembled and exercised end to end: one
//! directive scalar and one custom scalar whose accept set is the assembled
//! registry. This is the in-repo stand-in for the `acme-scalars` example the
//! design doc describes; the OSS repo turns it into a workspace with bindings.

mod common;

use common::{def, ScalarRef, TestExtension};
use superscalar::{ErrorKind, Registry, Scalar, ScalarId, ScalarTag};

const ORDER_NUMBER: ScalarId = ScalarId(ScalarId::EXTENSION_BLOCK);
const SCALAR_REF: ScalarId = ScalarId(ScalarId::EXTENSION_BLOCK + 1);

static ACME_DEFS: [superscalar::ScalarDef; 2] = [
    def(
        ORDER_NUMBER,
        "Acme",
        "Acme.OrderNumber",
        ScalarTag::PatternOnly,
        Some("^ORD-[0-9]{6}$"),
    ),
    def(
        SCALAR_REF,
        "Acme",
        "Acme.ScalarRef",
        ScalarTag::CustomLogic,
        None,
    ),
];

fn acme_impls() -> Vec<(ScalarId, Box<dyn Scalar>)> {
    vec![(SCALAR_REF, Box::new(ScalarRef(SCALAR_REF)))]
}

fn acme() -> TestExtension {
    TestExtension::new("acme", ScalarId::EXTENSION_BLOCK, &ACME_DEFS).with_impls(acme_impls)
}

fn assembled() -> Registry {
    Registry::assemble(&[&acme()])
}

#[test]
fn assembly_adds_the_extension_after_the_builtins() {
    let registry = assembled();
    assert_eq!(registry.len(), Registry::builtin().len() + 2);
    assert_eq!(
        registry
            .extensions()
            .iter()
            .map(|e| (e.name, e.id_base, e.legacy_ids))
            .collect::<Vec<_>>(),
        [("builtin", 0, false), ("acme", 4096, false)]
    );
    assert_eq!(registry.owner(ORDER_NUMBER), Some("acme"));
    assert_eq!(registry.owner(ScalarId::CONTACT_EMAIL), Some("builtin"));
    let all: Vec<u32> = registry.ids().map(|id| id.0).collect();
    assert_eq!(all[all.len() - 2..], [4096, 4097]);
}

#[test]
fn directive_scalar_runs_from_the_def_alone() {
    let registry = assembled();
    let order = registry.scalar(ORDER_NUMBER).expect("assembled");
    assert!(order.is_directive());
    assert_eq!(order.id(), ORDER_NUMBER);
    assert_eq!(order.parse(&registry, "ORD-000123").unwrap(), "ORD-000123");
    assert_eq!(order.normalize(&registry, "ORD-12").unwrap(), "ORD-12");
    assert!(order.validate(&registry, "ORD-000123").is_ok());
    let rejected = order.parse(&registry, "ORD-12").unwrap_err();
    assert_eq!(rejected.kind, ErrorKind::Pattern);
    assert_eq!(
        registry.by_canonical("Acme.OrderNumber").map(|d| d.id),
        Some(ORDER_NUMBER)
    );
}

/// The custom scalar sees the registry it was assembled into, not a
/// compile-time one: it accepts the extension's own names.
#[test]
fn custom_scalar_consults_the_assembled_registry() {
    let registry = assembled();
    let scalar_ref = registry.scalar(SCALAR_REF).expect("assembled");
    assert!(!scalar_ref.is_directive());
    assert_eq!(
        scalar_ref.parse(&registry, "Contact.Email").unwrap(),
        "Contact.Email"
    );
    assert_eq!(
        scalar_ref.normalize(&registry, "Acme.OrderNumber").unwrap(),
        "Acme.OrderNumber"
    );
    assert!(scalar_ref.validate(&registry, "Acme.ScalarRef").is_ok());
    let rejected = scalar_ref.validate(&registry, "Nope.Nope").unwrap_err();
    assert_eq!(rejected.kind, ErrorKind::Enum);

    // Handed the built-in registry instead, the same impl rejects the
    // extension names: the accept set is a property of the assembly.
    assert!(scalar_ref
        .validate(Registry::builtin(), "Acme.OrderNumber")
        .is_err());
    assert!(scalar_ref
        .validate(Registry::builtin(), "Contact.Email")
        .is_ok());
}

#[test]
fn builtin_scalars_behave_the_same_through_the_assembly() {
    let registry = assembled();
    let builtin = Registry::builtin();
    for (id, input) in [
        (ScalarId::CONTACT_EMAIL, " Alice@Example.com "),
        (ScalarId::DESIGN_COLOR, "#FF0000"),
        (
            ScalarId::IDENTITY_USER_ID,
            "550e8400-e29b-41d4-a716-446655440000",
        ),
        (ScalarId::TEMPORAL_DATE, "2026-01-15"),
        (ScalarId::FINANCE_MONEY, " 12345 "),
    ] {
        let through_assembly = registry.scalar(id).expect("slot").parse(&registry, input);
        let through_builtin = builtin.scalar(id).expect("slot").parse(builtin, input);
        assert_eq!(through_assembly, through_builtin, "{}", id.0);
    }
    let coerced = registry.coerce_lenient(&serde_json::json!(" 12345 "), "Finance.Money");
    assert_eq!(coerced.value, Some(serde_json::json!(12345)));
    let unknown = registry.coerce_lenient(&serde_json::json!("x"), "Acme.OrderNumber");
    assert_eq!(unknown.error.map(|e| e.kind), Some(ErrorKind::Pattern));
}

#[test]
fn the_builtin_registry_is_untouched_by_an_assembly() {
    let _registry = assembled();
    let builtin = Registry::builtin();
    assert_eq!(builtin.len(), 44);
    assert!(builtin.by_canonical("Acme.OrderNumber").is_none());
    assert!(builtin.def(ORDER_NUMBER).is_none());
}

#[test]
fn dump_attributes_extension_scalars_to_their_owner() {
    let dump = assembled().dump();
    let scalars = dump["scalars"].as_array().expect("array");
    assert_eq!(scalars.len(), 46);
    let order = &scalars[44];
    assert_eq!(order["canonical"], "Acme.OrderNumber");
    assert_eq!(order["extension"], "acme");
    assert_eq!(order["is_directive"], true);
    assert_eq!(scalars[45]["is_directive"], false);
    assert_eq!(dump["extensions"][1]["name"], "acme");
}
