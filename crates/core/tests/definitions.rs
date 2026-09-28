//! `Definitions`, the definitions-only assembly, against `Registry`: the two
//! answer every definition lookup identically, for the built-ins and for an
//! extension set, and for every def-level assembly failure they report the
//! same `AssemblyError`. The checks that need impls or patterns, and
//! extension naming, stay with `Registry`, and `Definitions` is shown not to
//! run them.

mod common;

use common::{alias_def, def, ScalarRef, TestExtension};
use superscalar::{
    names, AssemblyError, DefSource, Definitions, Extension, Registry, Scalar, ScalarDef, ScalarTag,
};

/// Every definition lookup `Registry` offers, asked of both and compared.
/// Defs are compared by address: both must hand back the very same `'static`
/// def, not an equal copy.
fn assert_agree(registry: &Registry, definitions: &Definitions) {
    assert_eq!(registry.len(), definitions.len());
    assert_eq!(registry.is_empty(), definitions.is_empty());
    assert_eq!(
        registry.names().collect::<Vec<_>>(),
        definitions.names().collect::<Vec<_>>()
    );
    let from_registry: Vec<*const ScalarDef> = registry.defs().map(std::ptr::from_ref).collect();
    let from_definitions: Vec<*const ScalarDef> =
        definitions.defs().map(std::ptr::from_ref).collect();
    assert_eq!(from_registry, from_definitions);

    // Every assembled name plus names nobody declared: a case variant, a bare
    // namespace, the empty string, a trailing space and an unknown name.
    let mut probes: Vec<&str> = registry.names().collect();
    probes.extend(["contact.email", "Contact", "", "Contact.Email ", "No.Such"]);
    for &name in &probes {
        assert_eq!(
            registry.def(name).map(std::ptr::from_ref),
            definitions.def(name).map(std::ptr::from_ref),
            "def({name:?})"
        );
        assert_eq!(
            registry.resolved(name),
            definitions.resolved(name),
            "resolved({name:?})"
        );
        for &other in &probes {
            assert_eq!(
                registry.comparable_with(name, other),
                definitions.comparable_with(name, other),
                "comparable_with({name:?}, {other:?})"
            );
        }
    }
}

#[test]
fn builtin_definitions_agree_with_the_builtin_registry() {
    assert_agree(Registry::builtin(), Definitions::builtin());
    assert_agree(Registry::builtin(), Registry::builtin().definitions());
    assert_eq!(Definitions::builtin().len(), 48);
}

/// An extension set that exercises every arm of the lookups: a directive def,
/// a custom def, an alias of an extension def, and a two-member comparability
/// class, so the alias and class arms of `comparable_with` see real input.
static ACME_DEFS: [ScalarDef; 5] = [
    def(
        "Acme",
        "Acme.OrderNumber",
        ScalarTag::PatternOnly,
        Some("^ORD-[0-9]{6}$"),
    ),
    def("Acme", "Acme.ScalarRef", ScalarTag::CustomLogic, None),
    classed(def("Acme", "Acme.Code", ScalarTag::PatternOnly, None)),
    alias_def("Acme", "Acme.CodeAlias", "Acme.Code"),
    classed(def("Acme", "Acme.OtherCode", ScalarTag::PatternOnly, None)),
];

const fn classed(mut def: ScalarDef) -> ScalarDef {
    def.comparability_class = Some("acme_code");
    def
}

fn acme_impls() -> Vec<(&'static str, Box<dyn Scalar>)> {
    vec![("Acme.ScalarRef", Box::new(ScalarRef))]
}

static BETA_DEFS: [ScalarDef; 1] = [def("Beta", "Beta.One", ScalarTag::PatternOnly, None)];

#[test]
fn extension_definitions_agree_with_the_extension_registry() {
    let acme = TestExtension::new("acme", &ACME_DEFS).with_impls(acme_impls);
    let beta = TestExtension::new("beta", &BETA_DEFS);
    let registry = Registry::assemble(&[&acme, &beta]);
    let definitions = Definitions::assemble(&[("acme", &ACME_DEFS), ("beta", &BETA_DEFS)]);
    assert_agree(&registry, &definitions);
    assert_agree(&registry, registry.definitions());

    // The alias and class arms were exercised, not vacuous.
    assert_eq!(definitions.resolved("Acme.CodeAlias"), "Acme.Code");
    assert!(definitions.comparable_with("Acme.CodeAlias", "Acme.OtherCode"));
    assert!(!definitions.comparable_with("Acme.OrderNumber", "Acme.OtherCode"));

    // The built-in view is untouched by an extension assembly.
    assert!(Definitions::builtin().def("Acme.OrderNumber").is_none());
}

fn registry_error(exts: &[&TestExtension]) -> AssemblyError {
    let dyn_exts: Vec<&dyn Extension> = exts.iter().map(|e| *e as &dyn Extension).collect();
    match Registry::try_assemble(&dyn_exts) {
        Ok(_) => panic!("registry assembly unexpectedly succeeded"),
        Err(err) => err,
    }
}

fn definitions_error(sources: &[DefSource]) -> AssemblyError {
    match Definitions::try_assemble(sources) {
        Ok(_) => panic!("definitions assembly unexpectedly succeeded"),
        Err(err) => err,
    }
}

static FIRST: [ScalarDef; 1] = [def("Acme", "Acme.One", ScalarTag::PatternOnly, None)];
static SAME_NAME: [ScalarDef; 1] = [def("Acme", "Acme.One", ScalarTag::PatternOnly, None)];
static STOLEN_CANONICAL: [ScalarDef; 1] = [def(
    "Contact",
    "Contact.Email",
    ScalarTag::PatternOnly,
    None,
)];
static WRONG_NAMESPACE: [ScalarDef; 1] = [def("Acme", "Other.Thing", ScalarTag::PatternOnly, None)];
static DANGLING: [ScalarDef; 1] = [alias_def("Acme", "Acme.Dangling", "Acme.Missing")];
static CHAIN: [ScalarDef; 2] = [
    alias_def("Acme", "Acme.Head", names::IDENTITY_USER_ID),
    def("Acme", "Acme.Filler", ScalarTag::PatternOnly, None),
];

/// Checks 2 to 5 are the ones `Definitions` runs. Each failure, provoked
/// through both assemblies, names the same error with the same owners.
#[test]
fn def_level_failures_report_the_same_error_through_both_assemblies() {
    let cases: Vec<(Vec<TestExtension>, Vec<DefSource>, &str)> = vec![
        (
            vec![TestExtension::new("acme", &STOLEN_CANONICAL)],
            vec![("acme", &STOLEN_CANONICAL)],
            "DuplicateCanonical against a built-in",
        ),
        (
            vec![
                TestExtension::new("acme", &FIRST),
                TestExtension::new("beta", &SAME_NAME),
            ],
            vec![("acme", &FIRST), ("beta", &SAME_NAME)],
            "DuplicateCanonical across extensions",
        ),
        (
            vec![TestExtension::new("acme", &WRONG_NAMESPACE)],
            vec![("acme", &WRONG_NAMESPACE)],
            "NamespaceMismatch",
        ),
        (
            vec![TestExtension::new("acme", &DANGLING)],
            vec![("acme", &DANGLING)],
            "DanglingAlias",
        ),
        (
            vec![TestExtension::new("acme", &CHAIN)],
            vec![("acme", &CHAIN)],
            "AliasChain",
        ),
    ];
    let mut seen = Vec::new();
    for (exts, sources, label) in &cases {
        let refs: Vec<&TestExtension> = exts.iter().collect();
        let from_registry = registry_error(&refs);
        let from_definitions = definitions_error(sources);
        assert_eq!(from_registry, from_definitions, "{label}");
        seen.push(std::mem::discriminant(&from_definitions));
    }
    // One variant per check, so none of the four is covered only by accident
    // of another.
    seen.dedup();
    assert_eq!(seen.len(), 4);
}

static BROKEN_PATTERN: [ScalarDef; 1] = [def(
    "Acme",
    "Acme.Broken",
    ScalarTag::PatternOnly,
    Some("(unclosed"),
)];
static CUSTOM_WITHOUT_IMPL: [ScalarDef; 1] =
    [def("Acme", "Acme.Custom", ScalarTag::CustomLogic, None)];

/// Pattern compilation, the impl checks and extension naming are
/// `Registry`'s. `Definitions` does not run them, which is what keeps the
/// regex engine and every impl out of a definitions-only build: the same
/// inputs that `Registry` refuses assemble here.
#[test]
fn registry_only_checks_do_not_run_in_definitions_assembly() {
    for (sources, registry_err) in [
        (
            [("acme", &BROKEN_PATTERN[..])],
            registry_error(&[&TestExtension::new("acme", &BROKEN_PATTERN)]),
        ),
        (
            [("acme", &CUSTOM_WITHOUT_IMPL[..])],
            registry_error(&[&TestExtension::new("acme", &CUSTOM_WITHOUT_IMPL)]),
        ),
        (
            [("builtin", &FIRST[..])],
            registry_error(&[&TestExtension::new("builtin", &FIRST)]),
        ),
    ] {
        let definitions = Definitions::try_assemble(&sources)
            .unwrap_or_else(|err| panic!("{err} is a Registry-only check"));
        assert_eq!(definitions.len(), Definitions::builtin().len() + 1);
        assert!(matches!(
            registry_err,
            AssemblyError::InvalidPattern { .. }
                | AssemblyError::MissingImpl { .. }
                | AssemblyError::DuplicateExtensionName { .. }
        ));
    }
}

#[test]
#[should_panic(expected = "scalar definitions assembly failed")]
fn assemble_panics_with_the_error_text() {
    Definitions::assemble(&[("acme", &STOLEN_CANONICAL)]);
}
