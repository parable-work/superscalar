//! Registry assembly: every `AssemblyError` variant has a test that provokes
//! exactly it, the built-in name table is pinned, and the dispatch guards that
//! used to live in `dispatch_exhaustiveness.rs` run over `Registry::builtin()`.

mod common;

use common::{alias_def, def, TestExtension, Upper};
use superscalar::{names, AssemblyError, LegacyAlias, Registry, Scalar, ScalarTag};

fn assemble(exts: &[&TestExtension]) -> Result<Registry, AssemblyError> {
    let dyn_exts: Vec<&dyn superscalar::Extension> = exts
        .iter()
        .map(|e| *e as &dyn superscalar::Extension)
        .collect();
    Registry::try_assemble(&dyn_exts)
}

fn err(result: Result<Registry, AssemblyError>) -> AssemblyError {
    match result {
        Ok(_) => panic!("assembly unexpectedly succeeded"),
        Err(e) => e,
    }
}

static ONE_PATTERN: [superscalar::ScalarDef; 1] = [def(
    "Acme",
    "Acme.One",
    ScalarTag::PatternOnly,
    Some("^[a-z]+$"),
)];

#[test]
fn duplicate_canonical_names_both_owners() {
    static DEFS: [superscalar::ScalarDef; 1] = [def(
        "Contact",
        "Contact.Email",
        ScalarTag::PatternOnly,
        None,
    )];
    let ext = TestExtension::new("acme", &DEFS);
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
    static OTHER: [superscalar::ScalarDef; 1] =
        [def("Beta", "Beta.One", ScalarTag::PatternOnly, None)];
    let a = TestExtension::new("acme", &ONE_PATTERN);
    let b = TestExtension::new("acme", &OTHER);
    assert_eq!(
        err(assemble(&[&a, &b])),
        AssemblyError::DuplicateExtensionName { name: "acme" }
    );
    let stolen = TestExtension::new("builtin", &ONE_PATTERN);
    assert_eq!(
        err(assemble(&[&stolen])),
        AssemblyError::DuplicateExtensionName { name: "builtin" }
    );
}

#[test]
fn namespace_mismatch() {
    static DEFS: [superscalar::ScalarDef; 1] =
        [def("Nope", "Acme.One", ScalarTag::PatternOnly, None)];
    let ext = TestExtension::new("acme", &DEFS);
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
    static DEFS: [superscalar::ScalarDef; 1] = [alias_def("Acme", "Acme.Alias", "Acme.Missing")];
    let ext = TestExtension::new("acme", &DEFS);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::DanglingAlias {
            canonical: "Acme.Alias",
            alias_of: "Acme.Missing",
        }
    );
}

#[test]
fn alias_chain() {
    static DEFS: [superscalar::ScalarDef; 2] = [
        alias_def("Acme", "Acme.First", "Acme.Second"),
        alias_def("Acme", "Acme.Second", names::CONTACT_EMAIL),
    ];
    let ext = TestExtension::new("acme", &DEFS);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::AliasChain {
            canonical: "Acme.First",
            alias_of: "Acme.Second",
        }
    );
}

#[test]
fn foreign_impl() {
    fn impls() -> Vec<(&'static str, Box<dyn Scalar>)> {
        vec![(names::CONTACT_EMAIL, Box::new(Upper))]
    }
    let ext = TestExtension::new("acme", &ONE_PATTERN).with_impls(impls);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::ForeignImpl {
            extension: "acme",
            canonical: names::CONTACT_EMAIL,
        }
    );
}

/// The `is_directive` guard: a def tagged for a hand-written impl that has
/// none must not silently fall through to `DirectiveScalar`.
#[test]
fn missing_impl_for_custom_logic_def() {
    static DEFS: [superscalar::ScalarDef; 1] =
        [def("Acme", "Acme.Custom", ScalarTag::CustomLogic, None)];
    let ext = TestExtension::new("acme", &DEFS);
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
    fn impls() -> Vec<(&'static str, Box<dyn Scalar>)> {
        vec![("Acme.One", Box::new(Upper))]
    }
    let ext = TestExtension::new("acme", &ONE_PATTERN).with_impls(impls);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::UnexpectedImpl {
            canonical: "Acme.One",
        }
    );
}

/// An alias borrows its target's impl; one registered under the alias's own
/// name would be silently ignored by the build phase, so assembly rejects it.
#[test]
fn unexpected_impl_for_alias_def() {
    static DEFS: [superscalar::ScalarDef; 1] =
        [alias_def("Acme", "Acme.EmailAlias", names::CONTACT_EMAIL)];
    fn impls() -> Vec<(&'static str, Box<dyn Scalar>)> {
        vec![("Acme.EmailAlias", Box::new(Upper))]
    }
    let ext = TestExtension::new("acme", &DEFS).with_impls(impls);
    assert_eq!(
        err(assemble(&[&ext])),
        AssemblyError::UnexpectedImpl {
            canonical: "Acme.EmailAlias",
        }
    );
    // Without the impl the same alias assembles and serves the target's impl.
    let ext = TestExtension::new("acme", &DEFS);
    let registry = assemble(&[&ext]).expect("alias of a built-in assembles");
    let input = "  Someone@Example.COM ";
    assert_eq!(
        registry
            .scalar("Acme.EmailAlias")
            .expect("slot")
            .normalize(&registry, input),
        registry
            .scalar(names::CONTACT_EMAIL)
            .expect("slot")
            .normalize(&registry, input),
    );
    assert!(!registry
        .scalar("Acme.EmailAlias")
        .expect("slot")
        .is_directive());
}

#[test]
fn invalid_pattern() {
    static DEFS: [superscalar::ScalarDef; 1] = [def(
        "Acme",
        "Acme.Broken",
        ScalarTag::PatternOnly,
        Some("("),
    )];
    let ext = TestExtension::new("acme", &DEFS);
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
    let ext = TestExtension::new("acme", &ONE_PATTERN).with_aliases(&ALIASES);
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
    let ext = TestExtension::new("acme", &ONE_PATTERN).with_aliases(&GOOD);
    let registry = assemble(&[&ext]).expect("alias resolves");
    assert_eq!(
        registry.legacy_aliases().last().map(|a| a.name),
        Some("One"),
        "extension aliases follow the built-in ones"
    );
}

/// When two checks would both fail, the earlier one in the documented order
/// names the error. One case per phase boundary.
#[test]
fn earlier_check_wins_when_two_violations_hold() {
    // 1 (duplicate extension name) before 2 (duplicate canonical).
    static STOLEN_CANONICAL: [superscalar::ScalarDef; 1] =
        [def("Acme", "Contact.Email", ScalarTag::PatternOnly, None)];
    let stolen_name = TestExtension::new("builtin", &STOLEN_CANONICAL);
    assert_eq!(
        err(assemble(&[&stolen_name])),
        AssemblyError::DuplicateExtensionName { name: "builtin" }
    );

    // 2 (duplicate canonical) before 3 (namespace mismatch).
    let stolen = TestExtension::new("acme", &STOLEN_CANONICAL);
    assert_eq!(
        err(assemble(&[&stolen])),
        AssemblyError::DuplicateCanonical {
            canonical: "Contact.Email",
            first: "builtin",
            second: "acme",
        }
    );

    // 3 (namespace mismatch) before 7 (missing impl).
    static WRONG_NAMESPACE: [superscalar::ScalarDef; 1] =
        [def("Other", "Acme.Custom", ScalarTag::CustomLogic, None)];
    let wrong_namespace = TestExtension::new("acme", &WRONG_NAMESPACE);
    assert_eq!(
        err(assemble(&[&wrong_namespace])),
        AssemblyError::NamespaceMismatch {
            canonical: "Acme.Custom",
            namespace: "Other",
        }
    );

    // 7 (missing impl) before 8 (invalid pattern).
    static UNIMPLEMENTED_BROKEN: [superscalar::ScalarDef; 1] = [def(
        "Acme",
        "Acme.Custom",
        ScalarTag::CustomLogic,
        Some("("),
    )];
    let unimplemented = TestExtension::new("acme", &UNIMPLEMENTED_BROKEN);
    assert_eq!(
        err(assemble(&[&unimplemented])),
        AssemblyError::MissingImpl {
            canonical: "Acme.Custom",
            tag: ScalarTag::CustomLogic,
        }
    );

    // 8 (invalid pattern) before 9 (dangling legacy alias).
    static BROKEN: [superscalar::ScalarDef; 1] = [def(
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
    let broken = TestExtension::new("acme", &BROKEN).with_aliases(&DANGLING);
    assert!(matches!(
        err(assemble(&[&broken])),
        AssemblyError::InvalidPattern {
            canonical: "Acme.Broken",
            ..
        }
    ));
}

/// The built-in names. Names are append-only: adding a scalar adds its name
/// here, and this is the test that fails when a name is renamed or removed.
/// `names::ALL` must carry exactly the same list, so a constant cannot drift
/// from the catalog.
#[test]
fn builtin_registry_is_frozen() {
    const TABLE: [&str; 48] = [
        "AgentSkill.Name",
        "Auth.JWT",
        "Auth.Password",
        "Contact.Email",
        "Contact.PhoneNumber",
        "Crypto.RSAPrivateKey",
        "Crypto.RSAPublicKey",
        "Crypto.SHA256",
        "Design.Color",
        "Embedding.Vector",
        "File.SizeBytes",
        "Finance.Money",
        "Generic.Int64",
        "Generic.JSON",
        "Generic.Probability",
        "Generic.StringMap",
        "Geo.Location",
        "Git.PathPattern",
        "Identity.Name",
        "Identity.Slug",
        "Identity.UUID",
        "Identity.UserID",
        "Localization.Locale",
        "Network.DnsLabel",
        "Network.DomainName",
        "Network.IpAddress",
        "Network.Uri",
        "Network.Url",
        "Ordering.Rank",
        "Temporal.CronExpression",
        "Temporal.Date",
        "Temporal.DateTime",
        "Temporal.Days",
        "Temporal.Duration",
        "Temporal.Hours",
        "Temporal.Milliseconds",
        "Temporal.Minutes",
        "Temporal.Month",
        "Temporal.Quarter",
        "Temporal.QuarterYear",
        "Temporal.RecurrenceRule",
        "Temporal.Seconds",
        "Temporal.Time",
        "Temporal.TimeZone",
        "Temporal.Year",
        "Text.Markdown",
        "Text.Sql",
        "Version.SemVer",
    ];
    let registry = Registry::builtin();
    let got: Vec<&str> = registry.names().collect();
    assert_eq!(got, TABLE);
    assert_eq!(names::ALL, TABLE);
    assert_eq!(registry.extensions(), ["builtin"]);
    assert!(registry.legacy_aliases().is_empty());
    for name in registry.names() {
        assert_eq!(registry.owner(name), Some("builtin"));
    }
}

/// A scalar is served by the generic directive engine exactly when its
/// resolved tag is `PatternOnly`; every other tag carries a hand-written impl.
#[test]
fn every_def_dispatch_matches_its_tag() {
    let registry = Registry::builtin();
    for def in registry.defs() {
        let resolved = registry
            .def(registry.resolved(def.canonical))
            .expect("alias target exists");
        let is_directive = registry.scalar(def.canonical).expect("slot").is_directive();
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
                registry.scalar(def.canonical).expect("slot").is_directive(),
                registry.scalar(target).expect("slot").is_directive(),
                "{}: alias of {target} but directive-ness diverges",
                def.canonical,
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
        .scalar(names::IDENTITY_USER_ID)
        .expect("slot")
        .parse(registry, raw);
    let via_target = registry
        .scalar(names::IDENTITY_UUID)
        .expect("slot")
        .parse(registry, raw);
    assert_eq!(via_alias, via_target);
    assert!(via_target.is_ok(), "uuid impl should accept a valid uuid");
    assert_eq!(
        registry.resolved(names::IDENTITY_USER_ID),
        names::IDENTITY_UUID
    );
}

/// Iteration is by canonical name, whatever order the extensions and their
/// defs were declared in.
#[test]
fn assembled_registry_orders_names_ascending() {
    static ACME: [superscalar::ScalarDef; 1] =
        [def("Acme", "Acme.Legacy", ScalarTag::PatternOnly, None)];
    static BETA: [superscalar::ScalarDef; 2] = [
        def("Beta", "Beta.Two", ScalarTag::PatternOnly, None),
        def("Beta", "Beta.One", ScalarTag::PatternOnly, None),
    ];
    let acme = TestExtension::new("acme", &ACME);
    let beta = TestExtension::new("beta", &BETA);
    let registry = assemble(&[&beta, &acme]).expect("assembles");
    let got: Vec<&str> = registry.names().collect();
    let mut expected: Vec<&str> = Registry::builtin().names().collect();
    expected.extend(["Acme.Legacy", "Beta.One", "Beta.Two"]);
    expected.sort_unstable();
    assert_eq!(got, expected);
    assert_eq!(registry.extensions(), ["builtin", "beta", "acme"]);
    assert_eq!(registry.owner("Beta.One"), Some("beta"));
}

#[test]
fn unknown_names_resolve_to_none() {
    let registry = Registry::builtin();
    assert!(registry.def("contact.email").is_none());
    assert!(registry.scalar("contact.email").is_none());
    assert!(registry.owner("contact.email").is_none());
    assert!(registry.def("Contact.Email").is_some());
    assert_eq!(registry.resolved("No.Such"), "No.Such");
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
    assert_eq!(dump["dump_version"], 2);
    assert_eq!(dump["extensions"], serde_json::json!(["builtin"]));
    let scalars = dump["scalars"].as_array().expect("array");
    assert_eq!(scalars.len(), 48);
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
    assert_eq!(user_id["alias_of"], "Identity.UUID");
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
    static STOLEN: [superscalar::ScalarDef; 1] = [def(
        "Contact",
        "Contact.Email",
        ScalarTag::PatternOnly,
        None,
    )];
    let ext = TestExtension::new("acme", &STOLEN);
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
        message
            .contains(r#"canonical name "Contact.Email" is declared by both "builtin" and "acme""#),
        "{message}"
    );
}
