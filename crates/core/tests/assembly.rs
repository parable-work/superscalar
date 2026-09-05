//! Registry assembly: every `AssemblyError` variant has a test that provokes
//! exactly it, the built-in id table is pinned, and the dispatch guards that
//! used to live in `dispatch_exhaustiveness.rs` run over `Registry::builtin()`.

mod common;

use common::{alias_def, def, TestExtension, Upper};
use superscalar::{
    AssembleOptions, AssemblyError, LegacyAlias, Registry, Scalar, ScalarId, ScalarTag,
};

const BLOCK: u32 = ScalarId::EXTENSION_BLOCK;
const NEXT_BUILTIN: u32 = 61;

fn assemble(exts: &[&TestExtension]) -> Result<Registry, AssemblyError> {
    let dyn_exts: Vec<&dyn superscalar::Extension> = exts
        .iter()
        .map(|e| *e as &dyn superscalar::Extension)
        .collect();
    Registry::try_assemble(&dyn_exts, AssembleOptions::default())
}

fn assemble_legacy(exts: &[&TestExtension]) -> Result<Registry, AssemblyError> {
    let dyn_exts: Vec<&dyn superscalar::Extension> = exts
        .iter()
        .map(|e| *e as &dyn superscalar::Extension)
        .collect();
    Registry::try_assemble(
        &dyn_exts,
        AssembleOptions {
            allow_legacy_ids: true,
        },
    )
}

fn err(result: Result<Registry, AssemblyError>) -> AssemblyError {
    match result {
        Ok(_) => panic!("assembly unexpectedly succeeded"),
        Err(e) => e,
    }
}

static ONE_PATTERN: [superscalar::ScalarDef; 1] = [def(
    ScalarId(BLOCK),
    "Acme",
    "Acme.One",
    ScalarTag::PatternOnly,
    Some("^[a-z]+$"),
)];

#[test]
fn id_base_not_aligned() {
    let ext = TestExtension::new("acme", BLOCK + 4, &ONE_PATTERN);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::IdBaseNotAligned {
            extension: "acme",
            id_base: BLOCK + 4,
        }
    );
}

#[test]
fn id_base_reserved() {
    let ext = TestExtension::new("acme", 0, &ONE_PATTERN);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::IdBaseReserved { extension: "acme" }
    );
}

#[test]
fn id_out_of_block() {
    static DEFS: [superscalar::ScalarDef; 1] = [def(
        ScalarId(2 * BLOCK),
        "Acme",
        "Acme.Far",
        ScalarTag::PatternOnly,
        None,
    )];
    let ext = TestExtension::new("acme", BLOCK, &DEFS);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::IdOutOfBlock {
            extension: "acme",
            id: ScalarId(2 * BLOCK),
            canonical: "Acme.Far",
        }
    );
}

#[test]
fn duplicate_id_names_both_owners() {
    static OTHER: [superscalar::ScalarDef; 1] = [def(
        ScalarId(BLOCK),
        "Beta",
        "Beta.One",
        ScalarTag::PatternOnly,
        None,
    )];
    let a = TestExtension::new("acme", BLOCK, &ONE_PATTERN);
    let b = TestExtension::new("beta", BLOCK, &OTHER);
    assert_eq!(
        err(assemble(&[&a, &b])),
        AssemblyError::DuplicateId {
            id: ScalarId(BLOCK),
            first: "acme",
            second: "beta",
        }
    );
}

#[test]
fn duplicate_canonical_names_both_owners() {
    static DEFS: [superscalar::ScalarDef; 1] = [def(
        ScalarId(BLOCK),
        "Contact",
        "Contact.Email",
        ScalarTag::PatternOnly,
        None,
    )];
    let ext = TestExtension::new("acme", BLOCK, &DEFS);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::DuplicateCanonical {
            canonical: "Contact.Email",
            first: "builtin",
            second: "acme",
        }
    );
}

#[test]
fn duplicate_extension_name() {
    static OTHER: [superscalar::ScalarDef; 1] = [def(
        ScalarId(2 * BLOCK),
        "Beta",
        "Beta.One",
        ScalarTag::PatternOnly,
        None,
    )];
    let a = TestExtension::new("acme", BLOCK, &ONE_PATTERN);
    let b = TestExtension::new("acme", 2 * BLOCK, &OTHER);
    assert_eq!(
        err(assemble(&[&a, &b])),
        AssemblyError::DuplicateExtensionName { name: "acme" }
    );
    let stolen = TestExtension::new("builtin", BLOCK, &ONE_PATTERN);
    assert_eq!(
        err(assemble(&[&stolen])),
        AssemblyError::DuplicateExtensionName { name: "builtin" }
    );
}

#[test]
fn namespace_mismatch() {
    static DEFS: [superscalar::ScalarDef; 1] = [def(
        ScalarId(BLOCK),
        "Nope",
        "Acme.One",
        ScalarTag::PatternOnly,
        None,
    )];
    let ext = TestExtension::new("acme", BLOCK, &DEFS);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::NamespaceMismatch {
            canonical: "Acme.One",
            namespace: "Nope",
        }
    );
}

#[test]
fn dangling_alias() {
    static DEFS: [superscalar::ScalarDef; 1] = [alias_def(
        ScalarId(BLOCK),
        "Acme",
        "Acme.Alias",
        ScalarId(9_999),
    )];
    let ext = TestExtension::new("acme", BLOCK, &DEFS);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::DanglingAlias {
            canonical: "Acme.Alias",
            alias_of: ScalarId(9_999),
        }
    );
}

#[test]
fn alias_chain() {
    static DEFS: [superscalar::ScalarDef; 2] = [
        alias_def(ScalarId(BLOCK), "Acme", "Acme.First", ScalarId(BLOCK + 1)),
        alias_def(
            ScalarId(BLOCK + 1),
            "Acme",
            "Acme.Second",
            ScalarId::CONTACT_EMAIL,
        ),
    ];
    let ext = TestExtension::new("acme", BLOCK, &DEFS);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::AliasChain {
            canonical: "Acme.First",
            alias_of: ScalarId(BLOCK + 1),
        }
    );
}

#[test]
fn foreign_impl() {
    fn impls() -> Vec<(ScalarId, Box<dyn Scalar>)> {
        vec![(
            ScalarId::CONTACT_EMAIL,
            Box::new(Upper(ScalarId::CONTACT_EMAIL)),
        )]
    }
    let ext = TestExtension::new("acme", BLOCK, &ONE_PATTERN).with_impls(impls);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::ForeignImpl {
            extension: "acme",
            id: ScalarId::CONTACT_EMAIL,
        }
    );
}

#[test]
fn impl_id_mismatch() {
    static DEFS: [superscalar::ScalarDef; 1] = [def(
        ScalarId(BLOCK),
        "Acme",
        "Acme.Custom",
        ScalarTag::CustomLogic,
        None,
    )];
    fn impls() -> Vec<(ScalarId, Box<dyn Scalar>)> {
        vec![(ScalarId(BLOCK), Box::new(Upper(ScalarId(BLOCK + 1))))]
    }
    let ext = TestExtension::new("acme", BLOCK, &DEFS).with_impls(impls);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::ImplIdMismatch {
            registered: ScalarId(BLOCK),
            reported: ScalarId(BLOCK + 1),
        }
    );
}

/// The `is_directive` guard: a def tagged for a hand-written impl that has
/// none must not silently fall through to `DirectiveScalar`.
#[test]
fn missing_impl_for_custom_logic_def() {
    static DEFS: [superscalar::ScalarDef; 1] = [def(
        ScalarId(BLOCK),
        "Acme",
        "Acme.Custom",
        ScalarTag::CustomLogic,
        None,
    )];
    let ext = TestExtension::new("acme", BLOCK, &DEFS);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::MissingImpl {
            canonical: "Acme.Custom",
            tag: ScalarTag::CustomLogic,
        }
    );
}

#[test]
fn unexpected_impl_for_pattern_only_def() {
    fn impls() -> Vec<(ScalarId, Box<dyn Scalar>)> {
        vec![(ScalarId(BLOCK), Box::new(Upper(ScalarId(BLOCK))))]
    }
    let ext = TestExtension::new("acme", BLOCK, &ONE_PATTERN).with_impls(impls);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::UnexpectedImpl {
            canonical: "Acme.One",
        }
    );
}

/// An alias borrows its target's impl; one registered under the alias's own
/// id would be silently ignored by the build phase, so assembly rejects it.
#[test]
fn unexpected_impl_for_alias_def() {
    static DEFS: [superscalar::ScalarDef; 1] = [alias_def(
        ScalarId(BLOCK),
        "Acme",
        "Acme.EmailAlias",
        ScalarId::CONTACT_EMAIL,
    )];
    fn impls() -> Vec<(ScalarId, Box<dyn Scalar>)> {
        vec![(ScalarId(BLOCK), Box::new(Upper(ScalarId(BLOCK))))]
    }
    let ext = TestExtension::new("acme", BLOCK, &DEFS).with_impls(impls);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::UnexpectedImpl {
            canonical: "Acme.EmailAlias",
        }
    );
    // Without the impl the same alias assembles and shares the target's impl.
    let ext = TestExtension::new("acme", BLOCK, &DEFS);
    let registry = assemble(&[&ext]).expect("alias of a built-in assembles");
    assert_eq!(
        registry.scalar(ScalarId(BLOCK)).expect("slot").id(),
        ScalarId::CONTACT_EMAIL
    );
}

#[test]
fn invalid_pattern() {
    static DEFS: [superscalar::ScalarDef; 1] = [def(
        ScalarId(BLOCK),
        "Acme",
        "Acme.Broken",
        ScalarTag::PatternOnly,
        Some("("),
    )];
    let ext = TestExtension::new("acme", BLOCK, &DEFS);
    match err(assemble(&[&ext])) {
        AssemblyError::InvalidPattern { canonical, message } => {
            assert_eq!(canonical, "Acme.Broken");
            assert!(!message.is_empty());
        }
        other => panic!("expected InvalidPattern, got {other:?}"),
    }
}

#[test]
fn dangling_legacy_alias() {
    static ALIASES: [LegacyAlias; 1] = [LegacyAlias {
        name: "One",
        target: "NoSuchSymbol",
        parse_target: None,
    }];
    let ext = TestExtension::new("acme", BLOCK, &ONE_PATTERN).with_aliases(&ALIASES);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::DanglingLegacyAlias {
            name: "One",
            target: "NoSuchSymbol",
        }
    );
    static GOOD: [LegacyAlias; 1] = [LegacyAlias {
        name: "One",
        target: "AcmeOne",
        parse_target: Some("ParseAcmeOne"),
    }];
    let ext = TestExtension::new("acme", BLOCK, &ONE_PATTERN).with_aliases(&GOOD);
    let registry = assemble(&[&ext]).expect("alias resolves");
    assert_eq!(
        registry.legacy_aliases().last().map(|a| a.name),
        Some("One"),
        "extension aliases follow the built-in ones"
    );
}

/// The shape a downstream with pre-block ids takes: `id_base` 0 plus
/// `allow_legacy_ids`. The legacy id is the first one past the highest
/// built-in, so it lands in the built-in block without hitting a hole.
#[test]
fn allow_legacy_ids_admits_holes_and_still_rejects_collisions() {
    static LEGACY: [superscalar::ScalarDef; 2] = [
        def(
            ScalarId(NEXT_BUILTIN),
            "Acme",
            "Acme.Legacy",
            ScalarTag::PatternOnly,
            None,
        ),
        def(
            ScalarId(BLOCK),
            "Acme",
            "Acme.New",
            ScalarTag::PatternOnly,
            None,
        ),
    ];
    let legacy = TestExtension::new("acme", 0, &LEGACY);
    let registry = assemble_legacy(&[&legacy]).expect("legacy shape assembles with the flag");
    assert_eq!(registry.len(), 46);
    assert_eq!(
        registry.extensions()[1],
        superscalar::ExtensionInfo {
            name: "acme",
            id_base: 0,
            legacy_ids: true,
        }
    );
    assert_eq!(
        err(assemble(&[&legacy])),
        AssemblyError::IdBaseReserved { extension: "acme" }
    );

    static OUT_OF_BLOCK: [superscalar::ScalarDef; 1] = [def(
        ScalarId(NEXT_BUILTIN),
        "Acme",
        "Acme.Legacy",
        ScalarTag::PatternOnly,
        None,
    )];
    let blocked = TestExtension::new("acme", BLOCK, &OUT_OF_BLOCK);
    assert_eq!(
        err(assemble(&[&blocked])),
        AssemblyError::IdOutOfBlock {
            extension: "acme",
            id: ScalarId(NEXT_BUILTIN),
            canonical: "Acme.Legacy",
        }
    );
    assemble_legacy(&[&blocked]).expect("the flag also waives the block range");

    static COLLIDING: [superscalar::ScalarDef; 1] = [def(
        ScalarId::CONTACT_EMAIL,
        "Acme",
        "Acme.Clash",
        ScalarTag::PatternOnly,
        None,
    )];
    let colliding = TestExtension::new("acme", 0, &COLLIDING);
    assert_eq!(
        err(assemble_legacy(&[&colliding])),
        AssemblyError::DuplicateId {
            id: ScalarId::CONTACT_EMAIL,
            first: "builtin",
            second: "acme",
        },
        "the flag never waives collision checks"
    );
}

/// When two checks would both fail, the earlier one in the documented order
/// names the error. One case per phase boundary, plus the cross-extension
/// order inside checks 1 and 2: alignment is checked over every extension
/// before any reserved-block check runs.
#[test]
fn earlier_check_wins_when_two_violations_hold() {
    // 1 (alignment) before 2 (out of block) and 4 (duplicate id). The legacy
    // flag waives 2 and never 1.
    static MISALIGNED: [superscalar::ScalarDef; 1] = [def(
        ScalarId::CONTACT_EMAIL,
        "Acme",
        "Acme.Clash",
        ScalarTag::PatternOnly,
        None,
    )];
    let misaligned = TestExtension::new("beta", BLOCK + 1, &MISALIGNED);
    let expected = AssemblyError::IdBaseNotAligned {
        extension: "beta",
        id_base: BLOCK + 1,
    };
    assert_eq!(err(assemble(&[&misaligned])), expected);
    assert_eq!(err(assemble_legacy(&[&misaligned])), expected);

    // Check 1 over a later extension outranks check 2 over an earlier one.
    static RESERVED: [superscalar::ScalarDef; 1] = [def(
        ScalarId(NEXT_BUILTIN),
        "Acme",
        "Acme.Legacy",
        ScalarTag::PatternOnly,
        None,
    )];
    let reserved = TestExtension::new("alpha", 0, &RESERVED);
    assert_eq!(err(assemble(&[&reserved, &misaligned])), expected);

    // 5 (duplicate canonical) before 6 (namespace mismatch).
    static STOLEN_CANONICAL: [superscalar::ScalarDef; 1] = [def(
        ScalarId(BLOCK),
        "Acme",
        "Contact.Email",
        ScalarTag::PatternOnly,
        None,
    )];
    let stolen = TestExtension::new("acme", BLOCK, &STOLEN_CANONICAL);
    assert_eq!(
        err(assemble(&[&stolen])),
        AssemblyError::DuplicateCanonical {
            canonical: "Contact.Email",
            first: "builtin",
            second: "acme",
        }
    );

    // 6 (namespace mismatch) before 11 (missing impl).
    static WRONG_NAMESPACE: [superscalar::ScalarDef; 1] = [def(
        ScalarId(BLOCK),
        "Other",
        "Acme.Custom",
        ScalarTag::CustomLogic,
        None,
    )];
    let wrong_namespace = TestExtension::new("acme", BLOCK, &WRONG_NAMESPACE);
    assert_eq!(
        err(assemble(&[&wrong_namespace])),
        AssemblyError::NamespaceMismatch {
            canonical: "Acme.Custom",
            namespace: "Other",
        }
    );

    // 11 (missing impl) before 12 (invalid pattern).
    static UNIMPLEMENTED_BROKEN: [superscalar::ScalarDef; 1] = [def(
        ScalarId(BLOCK),
        "Acme",
        "Acme.Custom",
        ScalarTag::CustomLogic,
        Some("("),
    )];
    let unimplemented = TestExtension::new("acme", BLOCK, &UNIMPLEMENTED_BROKEN);
    assert_eq!(
        err(assemble(&[&unimplemented])),
        AssemblyError::MissingImpl {
            canonical: "Acme.Custom",
            tag: ScalarTag::CustomLogic,
        }
    );

    // 12 (invalid pattern) before 13 (dangling legacy alias).
    static BROKEN: [superscalar::ScalarDef; 1] = [def(
        ScalarId(BLOCK),
        "Acme",
        "Acme.Broken",
        ScalarTag::PatternOnly,
        Some("("),
    )];
    static DANGLING: [LegacyAlias; 1] = [LegacyAlias {
        name: "Broken",
        target: "NoSuchSymbol",
        parse_target: None,
    }];
    let broken = TestExtension::new("acme", BLOCK, &BROKEN).with_aliases(&DANGLING);
    assert!(matches!(
        err(assemble(&[&broken])),
        AssemblyError::InvalidPattern {
            canonical: "Acme.Broken",
            ..
        }
    ));
}

/// The frozen id table. This is the test that fails when someone renumbers.
#[test]
fn builtin_registry_is_frozen() {
    const TABLE: [(u32, &str); 44] = [
        (5, "Auth.JWT"),
        (6, "Auth.Password"),
        (8, "Contact.Email"),
        (9, "Contact.PhoneNumber"),
        (10, "Crypto.RSAPrivateKey"),
        (11, "Crypto.RSAPublicKey"),
        (12, "Design.Color"),
        (13, "Embedding.Vector"),
        (14, "File.SizeBytes"),
        (15, "Finance.Money"),
        (16, "Generic.Int64"),
        (17, "Generic.JSON"),
        (18, "Generic.Probability"),
        (19, "Generic.StringMap"),
        (20, "Geo.Location"),
        (21, "Identity.Name"),
        (22, "Identity.Slug"),
        (23, "Identity.UUID"),
        (24, "Identity.UserID"),
        (25, "Localization.Locale"),
        (26, "Network.DomainName"),
        (27, "Network.IpAddress"),
        (28, "Network.Uri"),
        (29, "Network.Url"),
        (39, "Temporal.CronExpression"),
        (40, "Temporal.Date"),
        (41, "Temporal.DateTime"),
        (42, "Temporal.Duration"),
        (43, "Temporal.Milliseconds"),
        (44, "Temporal.Month"),
        (45, "Temporal.Quarter"),
        (46, "Temporal.QuarterYear"),
        (47, "Temporal.Time"),
        (48, "Temporal.TimeZone"),
        (49, "Temporal.Year"),
        (50, "Text.Markdown"),
        (52, "Temporal.Seconds"),
        (53, "Temporal.Minutes"),
        (54, "Temporal.Hours"),
        (55, "Temporal.Days"),
        (56, "Text.Sql"),
        (58, "Crypto.SHA256"),
        (59, "Network.DnsLabel"),
        (60, "Temporal.RecurrenceRule"),
    ];
    let registry = Registry::builtin();
    let got: Vec<(u32, &str)> = registry
        .defs()
        .map(|def| (def.id.as_u32(), def.canonical))
        .collect();
    assert_eq!(got, TABLE);
    assert_eq!(registry.extensions().len(), 1);
    assert_eq!(registry.extensions()[0].name, "builtin");
    assert!(!registry.extensions()[0].legacy_ids);
    assert!(registry.legacy_aliases().is_empty());
}

#[test]
fn builtin_ids_stay_in_block() {
    for id in Registry::builtin().ids() {
        assert!(
            id.is_builtin_block(),
            "{} is outside the built-in block",
            id.0
        );
        assert_eq!(Registry::builtin().owner(id), Some("builtin"));
    }
}

/// A scalar is served by the generic directive engine exactly when its
/// resolved tag is `PatternOnly`; every other tag carries a hand-written impl.
#[test]
fn every_def_dispatch_matches_its_tag() {
    let registry = Registry::builtin();
    for def in registry.defs() {
        let resolved = registry
            .def(registry.resolved(def.id))
            .expect("alias target exists");
        let is_directive = registry.scalar(def.id).expect("slot").is_directive();
        assert_eq!(
            is_directive,
            resolved.tag == ScalarTag::PatternOnly,
            "{}: is_directive={is_directive} but tag {:?}",
            def.canonical,
            resolved.tag
        );
    }
}

#[test]
fn alias_borrows_target_directiveness() {
    let registry = Registry::builtin();
    for def in registry.defs() {
        if let Some(target) = def.alias_of {
            assert_eq!(
                registry.scalar(def.id).expect("slot").is_directive(),
                registry.scalar(target).expect("slot").is_directive(),
                "{}: alias of {} but directive-ness diverges",
                def.canonical,
                target.0
            );
        }
    }
}

#[test]
fn alias_delegates_to_target_impl() {
    // Identity.UserID is an alias of Identity.UUID; parsing a UUID through the
    // alias must produce the target impl's canonical output, not a fallback.
    let registry = Registry::builtin();
    let raw = "550e8400-e29b-41d4-a716-446655440000";
    let via_alias = registry
        .scalar(ScalarId::IDENTITY_USER_ID)
        .expect("slot")
        .parse(registry, raw);
    let via_target = registry
        .scalar(ScalarId::IDENTITY_UUID)
        .expect("slot")
        .parse(registry, raw);
    assert_eq!(via_alias, via_target);
    assert!(via_target.is_ok(), "uuid impl should accept a valid uuid");
    // The shared impl reports the target id, as the dispatch table always did.
    assert_eq!(
        registry
            .scalar(ScalarId::IDENTITY_USER_ID)
            .expect("slot")
            .id(),
        ScalarId::IDENTITY_UUID
    );
    assert_eq!(
        registry.resolved(ScalarId::IDENTITY_USER_ID),
        ScalarId::IDENTITY_UUID
    );
}

#[test]
fn assembled_registry_orders_ids_ascending() {
    static LEGACY: [superscalar::ScalarDef; 1] = [def(
        ScalarId(NEXT_BUILTIN),
        "Acme",
        "Acme.Legacy",
        ScalarTag::PatternOnly,
        None,
    )];
    static NEW: [superscalar::ScalarDef; 2] = [
        def(
            ScalarId(BLOCK + 1),
            "Beta",
            "Beta.Two",
            ScalarTag::PatternOnly,
            None,
        ),
        def(
            ScalarId(BLOCK),
            "Beta",
            "Beta.One",
            ScalarTag::PatternOnly,
            None,
        ),
    ];
    let legacy = TestExtension::new("acme", 0, &LEGACY);
    let new = TestExtension::new("beta", BLOCK, &NEW);
    let registry = assemble_legacy(&[&new, &legacy]).expect("assembles");
    let ids: Vec<u32> = registry.ids().map(|id| id.0).collect();
    let mut expected: Vec<u32> = Registry::builtin().ids().map(|id| id.0).collect();
    expected.push(NEXT_BUILTIN);
    expected.push(BLOCK);
    expected.push(BLOCK + 1);
    assert_eq!(ids, expected);
}

#[test]
fn unknown_ids_and_names_resolve_to_none() {
    let registry = Registry::builtin();
    assert!(registry.def(ScalarId(9_999)).is_none());
    assert!(registry.scalar(ScalarId(9_999)).is_none());
    assert!(registry.by_canonical("contact.email").is_none());
    assert!(registry.by_canonical("Contact.Email").is_some());
    assert_eq!(registry.resolved(ScalarId(9_999)), ScalarId(9_999));
}

#[test]
fn dump_has_the_documented_shape() {
    let dump = Registry::builtin().dump();
    let keys: Vec<&str> = dump
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        keys,
        [
            "dump_version",
            "superscalar_version",
            "extensions",
            "scalars",
            "legacy_aliases"
        ]
    );
    assert_eq!(dump["dump_version"], 1);
    assert_eq!(
        dump["extensions"],
        serde_json::json!([{ "name": "builtin", "id_base": 0, "legacy_ids": false }])
    );
    let scalars = dump["scalars"].as_array().expect("array");
    assert_eq!(scalars.len(), 44);
    let email = scalars
        .iter()
        .find(|scalar| scalar["canonical"] == "Contact.Email")
        .expect("Contact.Email is built in");
    let email_keys: Vec<&str> = email
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        email_keys,
        [
            "id",
            "canonical",
            "namespace",
            "extension",
            "primitive",
            "sql_type",
            "metadata_primitive",
            "json_schema_type",
            "tag",
            "pattern",
            "min_length",
            "max_length",
            "minimum",
            "maximum",
            "case_insensitive",
            "reserved_words",
            "reserved_words_case_insensitive",
            "reserved_words_match_partial",
            "examples",
            "description",
            "docstring",
            "type_mappings",
            "file_upload",
            "image_constraints",
            "alias_of",
            "schema_primitive_override",
            "schema_omit",
            "metadata_omit",
            "format",
            "comparability_class",
            "is_sortable",
            "is_directive",
            "hooks"
        ]
    );
    assert_eq!(email["id"], 8);
    assert_eq!(email["canonical"], "Contact.Email");
    assert_eq!(email["namespace"], "Contact");
    assert_eq!(email["extension"], "builtin");
    assert_eq!(email["primitive"], "String");
    assert_eq!(email["tag"], "CustomLogic");
    assert_eq!(
        email["type_mappings"][0],
        serde_json::json!(["typescript", "string"])
    );
    assert_eq!(
        email["hooks"],
        serde_json::json!({ "parse": false, "normalize": true, "validate": true })
    );
    let user_id = scalars
        .iter()
        .find(|scalar| scalar["canonical"] == "Identity.UserID")
        .expect("Identity.UserID is built in");
    assert_eq!(user_id["alias_of"], 23);
    assert_eq!(email["metadata_omit"], false);
    assert!(email["file_upload"].is_null());
    assert_eq!(dump["legacy_aliases"].as_array().expect("array").len(), 0);
    // Stable: two dumps of the same registry are byte-identical.
    assert_eq!(
        serde_json::to_string(&dump).unwrap(),
        serde_json::to_string(&Registry::builtin().dump()).unwrap()
    );
}

#[test]
fn assemble_panics_with_the_error_text() {
    let ext = TestExtension::new("acme", 0, &ONE_PATTERN);
    let result = std::panic::catch_unwind(|| Registry::assemble(&[&ext]));
    let payload = match result {
        Ok(_) => panic!("assembly must panic"),
        Err(payload) => payload,
    };
    let message = payload
        .downcast_ref::<String>()
        .cloned()
        .expect("panic carries a String");
    assert!(
        message.contains("id_base 0 is the built-in block"),
        "{message}"
    );
}
