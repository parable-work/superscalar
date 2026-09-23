//! `Definitions`, the definitions-only assembly, against `Registry`: the two
//! answer every definition lookup identically, for the built-ins and for an
//! extension set, and for every def-level assembly failure they report the
//! same `AssemblyError`. The checks that need impls, patterns or id blocks
//! stay with `Registry`, and `Definitions` is shown not to run them.

mod common;

use common::{alias_def, def, ScalarRef, TestExtension};
use superscalar::{
    AssembleOptions, AssemblyError, DefSource, Definitions, Extension, Registry, Scalar, ScalarDef,
    ScalarId, ScalarTag,
};

const BLOCK: u32 = ScalarId::EXTENSION_BLOCK;

/// Every definition lookup `Registry` offers, asked of both and compared.
/// Defs are compared by address: both must hand back the very same `'static`
/// def, not an equal copy.
fn assert_agree(registry: &Registry, definitions: &Definitions) {
    assert_eq!(registry.len(), definitions.len());
    assert_eq!(registry.is_empty(), definitions.is_empty());
    assert_eq!(
        registry.ids().collect::<Vec<_>>(),
        definitions.ids().collect::<Vec<_>>()
    );
    let from_registry: Vec<*const ScalarDef> = registry.defs().map(std::ptr::from_ref).collect();
    let from_definitions: Vec<*const ScalarDef> =
        definitions.defs().map(std::ptr::from_ref).collect();
    assert_eq!(from_registry, from_definitions);

    // Every assembled id plus ids nobody declared: below, between and above.
    let mut probes: Vec<ScalarId> = registry.ids().collect();
    probes.extend([ScalarId(0), ScalarId(61), ScalarId(4095), ScalarId(9_999)]);
    for &id in &probes {
        assert_eq!(
            registry.def(id).map(std::ptr::from_ref),
            definitions.def(id).map(std::ptr::from_ref),
            "def({})",
            id.0
        );
        assert_eq!(
            registry.resolved(id),
            definitions.resolved(id),
            "resolved({})",
            id.0
        );
        for &other in &probes {
            assert_eq!(
                registry.comparable_with(id, other),
                definitions.comparable_with(id, other),
                "comparable_with({}, {})",
                id.0,
                other.0
            );
        }
    }
    let mut names: Vec<&str> = registry.defs().map(|def| def.canonical).collect();
    names.extend(["contact.email", "Contact", "", "Contact.Email ", "No.Such"]);
    for name in names {
        assert_eq!(
            registry.by_canonical(name).map(std::ptr::from_ref),
            definitions.by_canonical(name).map(std::ptr::from_ref),
            "by_canonical({name:?})"
        );
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
        ScalarId(BLOCK),
        "Acme",
        "Acme.OrderNumber",
        ScalarTag::PatternOnly,
        Some("^ORD-[0-9]{6}$"),
    ),
    def(
        ScalarId(BLOCK + 1),
        "Acme",
        "Acme.ScalarRef",
        ScalarTag::CustomLogic,
        None,
    ),
    classed(def(
        ScalarId(BLOCK + 2),
        "Acme",
        "Acme.Code",
        ScalarTag::PatternOnly,
        None,
    )),
    alias_def(
        ScalarId(BLOCK + 3),
        "Acme",
        "Acme.CodeAlias",
        ScalarId(BLOCK + 2),
    ),
    classed(def(
        ScalarId(BLOCK + 4),
        "Acme",
        "Acme.OtherCode",
        ScalarTag::PatternOnly,
        None,
    )),
];

const fn classed(mut def: ScalarDef) -> ScalarDef {
    def.comparability_class = Some("acme_code");
    def
}

fn acme_impls() -> Vec<(ScalarId, Box<dyn Scalar>)> {
    vec![(
        ScalarId(BLOCK + 1),
        Box::new(ScalarRef(ScalarId(BLOCK + 1))),
    )]
}

static BETA_DEFS: [ScalarDef; 1] = [def(
    ScalarId(2 * BLOCK),
    "Beta",
    "Beta.One",
    ScalarTag::PatternOnly,
    None,
)];

#[test]
fn extension_definitions_agree_with_the_extension_registry() {
    let acme = TestExtension::new("acme", BLOCK, &ACME_DEFS).with_impls(acme_impls);
    let beta = TestExtension::new("beta", 2 * BLOCK, &BETA_DEFS);
    let registry = Registry::assemble(&[&acme, &beta]);
    let definitions = Definitions::assemble(&[("acme", &ACME_DEFS), ("beta", &BETA_DEFS)]);
    assert_agree(&registry, &definitions);
    assert_agree(&registry, registry.definitions());

    // The alias and class arms were exercised, not vacuous.
    assert_eq!(
        definitions.resolved(ScalarId(BLOCK + 3)),
        ScalarId(BLOCK + 2)
    );
    assert!(definitions.comparable_with(ScalarId(BLOCK + 3), ScalarId(BLOCK + 4)));
    assert!(!definitions.comparable_with(ScalarId(BLOCK), ScalarId(BLOCK + 4)));

    // The built-in view is untouched by an extension assembly.
    assert!(Definitions::builtin().def(ScalarId(BLOCK)).is_none());
}

fn registry_error(exts: &[&TestExtension], options: AssembleOptions) -> AssemblyError {
    let dyn_exts: Vec<&dyn Extension> = exts.iter().map(|e| *e as &dyn Extension).collect();
    match Registry::try_assemble(&dyn_exts, options) {
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

const LEGACY: AssembleOptions = AssembleOptions {
    allow_legacy_ids: true,
};

static CLASH_WITH_BUILTIN_ID: [ScalarDef; 1] = [def(
    ScalarId::CONTACT_EMAIL,
    "Acme",
    "Acme.Clash",
    ScalarTag::PatternOnly,
    None,
)];
static FIRST: [ScalarDef; 1] = [def(
    ScalarId(BLOCK),
    "Acme",
    "Acme.One",
    ScalarTag::PatternOnly,
    None,
)];
static SAME_ID_OTHER_NAME: [ScalarDef; 1] = [def(
    ScalarId(BLOCK),
    "Beta",
    "Beta.One",
    ScalarTag::PatternOnly,
    None,
)];
static STOLEN_CANONICAL: [ScalarDef; 1] = [def(
    ScalarId(BLOCK),
    "Contact",
    "Contact.Email",
    ScalarTag::PatternOnly,
    None,
)];
static WRONG_NAMESPACE: [ScalarDef; 1] = [def(
    ScalarId(BLOCK),
    "Acme",
    "Other.Thing",
    ScalarTag::PatternOnly,
    None,
)];
static DANGLING: [ScalarDef; 1] = [alias_def(
    ScalarId(BLOCK),
    "Acme",
    "Acme.Dangling",
    ScalarId(BLOCK + 99),
)];
static CHAIN: [ScalarDef; 2] = [
    alias_def(
        ScalarId(BLOCK),
        "Acme",
        "Acme.Head",
        ScalarId::IDENTITY_USER_ID,
    ),
    def(
        ScalarId(BLOCK + 1),
        "Acme",
        "Acme.Filler",
        ScalarTag::PatternOnly,
        None,
    ),
];

/// Checks 4 to 8 are the ones `Definitions` runs. Each failure, provoked
/// through both assemblies, names the same error with the same owners.
#[test]
fn def_level_failures_report_the_same_error_through_both_assemblies() {
    let cases: Vec<(Vec<TestExtension>, AssembleOptions, Vec<DefSource>, &str)> = vec![
        (
            vec![TestExtension::new("acme", 0, &CLASH_WITH_BUILTIN_ID)],
            LEGACY,
            vec![("acme", &CLASH_WITH_BUILTIN_ID)],
            "DuplicateId against a built-in",
        ),
        (
            vec![
                TestExtension::new("acme", BLOCK, &FIRST),
                TestExtension::new("beta", BLOCK, &SAME_ID_OTHER_NAME),
            ],
            LEGACY,
            vec![("acme", &FIRST), ("beta", &SAME_ID_OTHER_NAME)],
            "DuplicateId across extensions",
        ),
        (
            vec![TestExtension::new("acme", BLOCK, &STOLEN_CANONICAL)],
            AssembleOptions::default(),
            vec![("acme", &STOLEN_CANONICAL)],
            "DuplicateCanonical",
        ),
        (
            vec![TestExtension::new("acme", BLOCK, &WRONG_NAMESPACE)],
            AssembleOptions::default(),
            vec![("acme", &WRONG_NAMESPACE)],
            "NamespaceMismatch",
        ),
        (
            vec![TestExtension::new("acme", BLOCK, &DANGLING)],
            AssembleOptions::default(),
            vec![("acme", &DANGLING)],
            "DanglingAlias",
        ),
        (
            vec![TestExtension::new("acme", BLOCK, &CHAIN)],
            AssembleOptions::default(),
            vec![("acme", &CHAIN)],
            "AliasChain",
        ),
    ];
    let mut seen = Vec::new();
    for (exts, options, sources, label) in &cases {
        let refs: Vec<&TestExtension> = exts.iter().collect();
        let from_registry = registry_error(&refs, *options);
        let from_definitions = definitions_error(sources);
        assert_eq!(from_registry, from_definitions, "{label}");
        seen.push(std::mem::discriminant(&from_definitions));
    }
    // One case per check, each a different variant, so none of the five is
    // covered only by accident of another.
    seen.dedup();
    assert_eq!(seen.len(), 5);
}

static BROKEN_PATTERN: [ScalarDef; 1] = [def(
    ScalarId(BLOCK),
    "Acme",
    "Acme.Broken",
    ScalarTag::PatternOnly,
    Some("(unclosed"),
)];
static CUSTOM_WITHOUT_IMPL: [ScalarDef; 1] = [def(
    ScalarId(BLOCK),
    "Acme",
    "Acme.Custom",
    ScalarTag::CustomLogic,
    None,
)];
static OUT_OF_BLOCK: [ScalarDef; 1] = [def(
    ScalarId(68),
    "Acme",
    "Acme.Legacy",
    ScalarTag::PatternOnly,
    None,
)];

/// Pattern compilation, the impl checks, the id-block checks and extension
/// naming are `Registry`'s. `Definitions` does not run them, which is what
/// keeps the regex engine and every impl out of a definitions-only build: the
/// same inputs that `Registry` refuses assemble here.
#[test]
fn registry_only_checks_do_not_run_in_definitions_assembly() {
    for (sources, registry_err) in [
        (
            [("acme", &BROKEN_PATTERN[..])],
            registry_error(
                &[&TestExtension::new("acme", BLOCK, &BROKEN_PATTERN)],
                AssembleOptions::default(),
            ),
        ),
        (
            [("acme", &CUSTOM_WITHOUT_IMPL[..])],
            registry_error(
                &[&TestExtension::new("acme", BLOCK, &CUSTOM_WITHOUT_IMPL)],
                AssembleOptions::default(),
            ),
        ),
        (
            [("acme", &OUT_OF_BLOCK[..])],
            registry_error(
                &[&TestExtension::new("acme", BLOCK, &OUT_OF_BLOCK)],
                AssembleOptions::default(),
            ),
        ),
        (
            [("builtin", &FIRST[..])],
            registry_error(
                &[&TestExtension::new("builtin", BLOCK, &FIRST)],
                AssembleOptions::default(),
            ),
        ),
    ] {
        let definitions = Definitions::try_assemble(&sources)
            .unwrap_or_else(|err| panic!("{err} is a Registry-only check"));
        assert_eq!(definitions.len(), Definitions::builtin().len() + 1);
        assert!(matches!(
            registry_err,
            AssemblyError::InvalidPattern { .. }
                | AssemblyError::MissingImpl { .. }
                | AssemblyError::IdOutOfBlock { .. }
                | AssemblyError::DuplicateExtensionName { .. }
        ));
    }
}

#[test]
#[should_panic(expected = "scalar definitions assembly failed")]
fn assemble_panics_with_the_error_text() {
    Definitions::assemble(&[("acme", &STOLEN_CANONICAL)]);
}
