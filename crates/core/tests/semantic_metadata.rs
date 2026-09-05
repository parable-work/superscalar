//! The semantic facts the registry carries: comparability class (one class
//! assigned, `None` on every other built-in row) and the derived sortability
//! predicate, plus the invariants both depend on.

use superscalar::{scalar_def, PrimitiveKind, Registry, ScalarId, ScalarTag};

/// The first class assignment replaced the all-`None` tripwire that used to
/// live here: the catalog is no longer class-free, so the assertion is now the
/// exact map rather than its absence. `catalog.rs` is the source of truth;
/// `meta.comparability_classes` in the conformance corpus restates it and every
/// binding asserts the two agree.
#[test]
fn the_v1_comparability_class_is_exactly_temporal_instant() {
    const EXPECTED: &[(&str, &str)] = &[
        ("Temporal.Date", "temporal_instant"),
        ("Temporal.DateTime", "temporal_instant"),
    ];
    let mut got: Vec<(&str, &str)> = Registry::builtin()
        .ids()
        .filter_map(|id| {
            scalar_def(id)
                .comparability_class
                .map(|c| (scalar_def(id).canonical, c))
        })
        .collect();
    got.sort_unstable();
    assert_eq!(got, EXPECTED, "the v1 comparability class assignment");
}

/// The relation the class exists to create. Before the first class this
/// answered `false`, which is what made
/// `comparability_is_an_equivalence_relation_over_the_catalog` vacuous.
/// Symmetry is covered by that walk and by the exact-pair-set assertion in
/// `comparability_across_distinct_scalars_is_exactly_the_temporal_instant_pair`;
/// do not remove either without replacing the symmetry check.
#[test]
fn temporal_date_and_datetime_are_comparable() {
    assert!(
        Registry::builtin().comparable_with(ScalarId::TEMPORAL_DATE, ScalarId::TEMPORAL_DATE_TIME),
        "Temporal.Date ~ Temporal.DateTime"
    );
}

/// The exact non-sortable set over the WHOLE catalog: the four scalars whose
/// JSON shape is not a JSON scalar. The rule is a JSON-shape rule and nothing
/// else: secret-bearing scalars such as `Auth.Password`, `Auth.JWT` and
/// `Crypto.RSAPrivateKey` all declare `"string"` and all sort. Whether a column
/// SHOULD be exposed is an authorization question this predicate does not
/// answer.
///
/// A generated SDK emits `sort` only for sortable columns and a read path
/// enforces the same predicate, so a silent move in this set changes a public
/// API surface. Every built-in carries a metadata row, so this is also the
/// number every corpus reader asserts as `non_sortable_count`.
#[test]
fn non_sortable_set_is_exactly_the_four_table_scalars() {
    const EXPECTED: &[&str] = &[
        "Embedding.Vector",
        "Generic.JSON",
        "Generic.StringMap",
        "Geo.Location",
    ];
    let mut got: Vec<&str> = Registry::builtin()
        .ids()
        .filter(|id| !scalar_def(*id).is_sortable())
        .map(|id| scalar_def(id).canonical)
        .collect();
    got.sort_unstable();
    assert_eq!(got, EXPECTED);
    assert!(Registry::builtin().defs().all(|def| !def.metadata_omit));
}

/// A primitive-only predicate (the ticket's literal wording) would call these
/// three sortable. The SDK spec requires them not to be. This is the regression
/// test for that divergence: it fails the moment the allowlist is widened to
/// admit their declared JSON shape, or inverted back to a primitive rule.
#[test]
fn string_primitive_json_scalars_are_not_sortable() {
    for canonical in ["Embedding.Vector", "Generic.JSON", "Generic.StringMap"] {
        let def = Registry::builtin()
            .by_canonical(canonical)
            .expect("catalogued");
        assert_eq!(
            def.primitive,
            PrimitiveKind::String,
            "{canonical} primitive"
        );
        assert!(!def.is_sortable(), "{canonical} must not be sortable");
    }
}

/// The spec states the sortability rule CATEGORICALLY ("structural columns are
/// not sortable"); `is_sortable()` implements a proxy over one declared string.
/// This is the standing cross-check that keeps the proxy honest -- it reads
/// `tag`, which the predicate does not, so it is a genuinely independent
/// oracle. It catches a Structural scalar whose `json_schema_type` was
/// mistyped, which the name pin cannot (that test only fires when the set
/// GROWS, never when a member fails to join it). It does not catch a brand new
/// JSON-ish scalar; the domain test below and the CONTEXT.md obligation are
/// what cover that (C2, F6, D2).
#[test]
fn structural_and_object_scalars_declare_an_object_json_shape() {
    let mut structural = 0;
    let mut objects = 0;
    for id in Registry::builtin().ids() {
        let def = scalar_def(id);
        if def.tag == ScalarTag::Structural {
            structural += 1;
            assert_eq!(
                def.json_schema_type,
                "object",
                "{} tag",
                scalar_def(id).canonical
            );
            assert!(
                !def.is_sortable(),
                "{} structural must not sort",
                scalar_def(id).canonical
            );
        }
        if def.primitive == PrimitiveKind::Object {
            objects += 1;
            assert_eq!(
                def.json_schema_type,
                "object",
                "{} primitive",
                scalar_def(id).canonical
            );
        }
    }
    // Not a coverage assertion for its own sake: if either count reaches zero
    // the loop above asserts nothing and the cross-check goes silently vacuous.
    //
    // Deliberately literal, unlike `checked` above. Deriving these from the
    // same walk that produced them would make them tautological -- the whole
    // job of this pair is to be an outside statement about the catalog that a
    // change to the catalog has to come and update. A new Structural scalar
    // SHOULD fail here and be looked at. Geo.Location is the one built-in of
    // each kind.
    assert_eq!(structural, 1);
    assert_eq!(objects, 1);
}

/// `json_schema_type` is a plain `&'static str` with no enum and no domain
/// constraint, and after this ticket it alone decides a public API surface.
/// Pin the domain so a new scalar cannot introduce a value the allowlist has
/// never seen without this test saying so (F1, C1).
#[test]
fn json_schema_type_domain_is_closed() {
    const KNOWN: &[&str] = &[
        "string", "integer", "number", "boolean", "object", "array", "",
    ];
    for id in Registry::builtin().ids() {
        let declared = scalar_def(id).json_schema_type;
        assert!(
            KNOWN.contains(&declared),
            "{} declares json_schema_type {declared:?}, which is outside the known \
             domain {KNOWN:?}; a value the sortability allowlist has never seen is \
             non-sortable by default, which may or may not be what you meant",
            scalar_def(id).canonical
        );
    }
}

/// Only a metadata-omitted scalar may declare an empty `json_schema_type`.
/// Every scalar that reaches a binding's metadata table declares a real one.
/// Without this, a metadata-bearing scalar with `""` would emit
/// `json_schema_type: None` into the table (Step 2.2) and read as a scalar
/// with no declared JSON shape in four languages (C1a).
#[test]
fn every_metadata_bearing_scalar_declares_a_json_schema_type() {
    for id in Registry::builtin().ids() {
        if scalar_def(id).metadata_omit {
            continue;
        }
        assert!(
            !scalar_def(id).json_schema_type.is_empty(),
            "{} has a metadata row and must declare a json_schema_type",
            scalar_def(id).canonical
        );
    }
}

/// Three views of one fact must agree for every scalar with a metadata row:
///   1. the registry predicate (`ScalarDef::is_sortable`),
///   2. the emitted bool on the metadata row (what every binding reads),
///   3. a re-derivation from only the fields the metadata row exposes.
///
/// (3) is what catches an emitter that stopped reading the predicate. It is NOT
/// an independent oracle for the rule itself -- both sides read the same
/// declared `json_schema_type`, so a mis-declared field makes both wrong
/// together. The independent oracles are the eleven-name pin and
/// `structural_and_object_scalars_declare_an_object_json_shape`, which reads
/// `tag`. Stated plainly here so nobody mistakes this test for more than it is
/// (C1, C2).
///
/// The row-side derivation reads `json_schema_type` alone, matching
/// `ScalarDef::is_sortable`. It does NOT check `md.primitive != "Type"`: over
/// the table `primitive == "Type"` is a strict subset of
/// `json_schema_type == "object"`, so that clause was redundant and untestable.
/// `Geo.Location` is the case that makes the row-side derivation non-trivial:
/// it carries a `schema_primitive_override` so its metadata primitive is
/// "String" while its registry primitive is `Object`, and only the JSON shape
/// gets it right on both sides.
#[test]
fn registry_predicate_emitted_field_and_row_derivation_agree() {
    use superscalar::scalar_metadata_by_canonical_name;

    let mut checked = 0usize;
    for id in Registry::builtin().ids() {
        let Some(md) = scalar_metadata_by_canonical_name(scalar_def(id).canonical) else {
            // No metadata row (a metadata_omit def); nothing to agree with.
            continue;
        };
        let from_row = matches!(
            md.json_schema_type,
            Some("string" | "integer" | "number" | "boolean")
        );
        assert_eq!(
            scalar_def(id).is_sortable(),
            md.is_sortable,
            "{}: registry predicate vs emitted field",
            scalar_def(id).canonical
        );
        assert_eq!(
            md.is_sortable,
            from_row,
            "{}: emitted field vs re-derivation from the row's own fields",
            scalar_def(id).canonical
        );
        assert_eq!(md.comparability_class, scalar_def(id).comparability_class);
        checked += 1;
    }
    // Without this the loop passes vacuously if the metadata table is ever
    // empty. Derived rather than literal (the file computes `cross_pairs` the
    // same way): every scalar without `metadata_omit` has a row.
    let with_rows = Registry::builtin()
        .defs()
        .filter(|def| !def.metadata_omit)
        .count();
    assert_eq!(checked, with_rows);
}

/// The relation `comparability_class` encodes is NOT what naive equality over
/// the field computes, and that gap is the whole reason this predicate exists.
///
/// `None` means self-comparable ONLY. But `a.comparability_class ==
/// b.comparability_class` -- the obvious implementation, in any of the four
/// languages -- answers `true` for two class-less scalars, which is every
/// class-less pair in the catalog. That includes
/// `Contact.Email` against `Contact.PhoneNumber`, the exact comparison
/// the semantic-types design names as the thing the relation must reject. Go is
/// the worst case: it flattens absent to `""`, which is also the map-miss zero
/// value, so a naive Go implementation cannot even distinguish "no class" from
/// "scalar not found".
///
/// So the field ships with the predicate that reads it, and the predicate is
/// the contract. A query validator and a schema data layer must call this
/// rather than compare the raw field.
///
/// The first class assignment upgraded the guarantee below from "no two
/// distinct scalars are comparable" to "exactly these are". The walk stays
/// exhaustive either way, because an exact-set assertion over every ordered
/// pair is what catches `comparable_with` collapsing into field equality: that
/// collapse reports thousands of pairs here, not two.
#[test]
fn comparability_across_distinct_scalars_is_exactly_the_temporal_instant_pair() {
    let email = scalar_def(ScalarId::CONTACT_EMAIL);
    let phone = scalar_def(ScalarId::CONTACT_PHONE_NUMBER);

    // Self-comparable, which is what `None` means.
    assert!(Registry::builtin().comparable_with(email.id, email.id));
    assert!(Registry::builtin().comparable_with(phone.id, phone.id));

    // The spec's named counter-example. Naive field equality says `true` here,
    // because both sides are `None`. That is the bug this test exists for.
    assert!(!Registry::builtin().comparable_with(email.id, phone.id));
    assert!(!Registry::builtin().comparable_with(phone.id, email.id));
    assert_eq!(
        email.comparability_class, phone.comparability_class,
        "both are None today, which is exactly why raw field equality is the \
         wrong implementation"
    );

    // An alias shares one implementation under two ids, so it is the same
    // scalar for comparison purposes. `Identity.UserID -> Identity.UUID` is the
    // only alias pair in the catalog.
    let user_id = scalar_def(ScalarId::IDENTITY_USER_ID);
    let uuid = scalar_def(ScalarId::IDENTITY_UUID);
    assert_eq!(user_id.alias_of, Some(ScalarId::IDENTITY_UUID));
    assert!(Registry::builtin().comparable_with(user_id.id, uuid.id));
    assert!(Registry::builtin().comparable_with(uuid.id, user_id.id));

    // Exhaustive: the only comparable DISTINCT pairs are the ones the v1 class
    // table declares. Both directions are listed, so this doubles as the
    // symmetry check on the new pair.
    let distinct_pairs: Vec<(ScalarId, ScalarId)> = Registry::builtin()
        .ids()
        .flat_map(|a| Registry::builtin().ids().map(move |b| (a, b)))
        .filter(|(a, b)| Registry::builtin().resolved(*a) != Registry::builtin().resolved(*b))
        .collect();
    let cross_pairs = distinct_pairs.len();
    let mut comparable: Vec<(&str, &str)> = distinct_pairs
        .iter()
        .filter(|(a, b)| Registry::builtin().comparable_with(*a, *b))
        .map(|(a, b)| (scalar_def(*a).canonical, scalar_def(*b).canonical))
        .collect();
    comparable.sort_unstable();
    assert_eq!(
        comparable,
        vec![
            ("Temporal.Date", "Temporal.DateTime"),
            ("Temporal.DateTime", "Temporal.Date"),
        ],
        "exactly the temporal_instant pair, both directions, is comparable \
         across distinct scalars"
    );
    // Derived, not hardcoded: ordered pairs over the catalog, minus the pairs
    // that resolve to the same scalar (the self-pairs plus both directions
    // of the single alias pair). Adding a scalar updates this on its own.
    let n = Registry::builtin().len();
    let same_scalar = Registry::builtin()
        .ids()
        .flat_map(|a| Registry::builtin().ids().map(move |b| (a, b)))
        .filter(|(a, b)| Registry::builtin().resolved(*a) == Registry::builtin().resolved(*b))
        .count();
    assert_eq!(cross_pairs, n * n - same_scalar);
    assert!(cross_pairs > 0, "the pair loop must not be vacuous");
}

/// The three invariants a comparability class table must satisfy, extracted so
/// they can be tested against tables the catalog cannot produce yet.
///
/// The extraction is the point, not a style choice. When every catalog value was
/// `None`, a catalog walk guarding on `if let Some(class)` never entered its
/// body: the assertions inside could be arbitrarily corrupted -- `>= 2` written
/// as `>= 99`, a primitive comparison inverted -- and every test still passed. A
/// mutation probe during review confirmed exactly that. Without the extraction,
/// the first class assignment would have inherited guards that do not guard, at the moment it
/// most needs them. Same remedy as `json_shape_is_sortable` in `registry.rs`:
/// put the rule in a function, table-test the function against synthetic
/// input, and have the catalog walk call the same function.
///
/// Returns the first violation as a message, so the table test can assert
/// WHICH rule fired rather than merely that something did.
fn check_class_invariants(
    rows: &[(Option<&'static str>, PrimitiveKind, &'static str)],
) -> Result<(), String> {
    let mut members: std::collections::BTreeMap<&str, Vec<&str>> = Default::default();
    let mut primitives: std::collections::BTreeMap<&str, (PrimitiveKind, &str)> =
        Default::default();

    for &(class, primitive, canonical) in rows {
        let Some(class) = class else {
            continue;
        };

        // Shape. Two encoding facts bound what the open string may carry: Go
        // flattens absent to `""`, which would make a class named `""`
        // indistinguishable from "no class"; and the per-language shaper is
        // Rust `Debug` escaping, which is not valid Go, Python or TypeScript
        // escaping for non-ASCII.
        if class.is_empty() {
            return Err(format!(
                "{canonical}: the empty string is how Go encodes absent; it cannot \
                 also be a class name"
            ));
        }
        if !class
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        {
            return Err(format!(
                "{canonical}: comparability class {class:?} must match [a-z0-9_]+; \
                 the shaper emits Rust Debug escaping, which is not valid Go, \
                 Python or TypeScript escaping for non-ASCII"
            ));
        }

        // Primitive homogeneity: comparing a `String` scalar
        // against an `Int` scalar is not made legal by sharing a class name,
        // and a SQL validator would emit a comparison the engine cannot
        // execute. Reads the REGISTRY primitive, not `metadata_primitive`. The
        // two disagree only for `Geo.Location`, and the registry value is the
        // right one because comparability asks whether two RUNTIME values can
        // be compared, while `metadata_primitive` exists to keep the generated
        // schema catalogs matching an audited baseline.
        let (first_primitive, first_canonical) =
            *primitives.entry(class).or_insert((primitive, canonical));
        if first_primitive != primitive {
            return Err(format!(
                "comparability class {class:?} spans two primitives: \
                 {first_canonical} is {first_primitive:?}, {canonical} is {primitive:?}"
            ));
        }

        members.entry(class).or_default().push(canonical);
    }

    // Singleton. Under equivalence-class semantics a class with exactly one
    // member is indistinguishable from `None`, so a lone typo --
    // "temporal-instant" against "temporal_instant" -- silently degrades that
    // scalar to self-comparable-only.
    //
    // What this does NOT catch, stated so the guarantee is not read as broader
    // than it is: two scalars sharing the SAME misspelling form a perfectly
    // valid two-member class. The relation then splits in a way no assertion
    // here can see, because a split relation is structurally identical to a
    // deliberate one. The shape rule above narrows the space a typo can land
    // in; it does not eliminate it. Reviewing the class table stays a human job.
    for (class, scalars) in &members {
        if scalars.len() < 2 {
            return Err(format!(
                "comparability class {class:?} has one member ({scalars:?}); a \
                 singleton class is indistinguishable from None and is almost \
                 always a typo"
            ));
        }
    }

    Ok(())
}

/// The shipped catalog satisfies all three. It was vacuous by construction until
/// the first class assigned `temporal_instant`; with one two-member class the
/// name-shape and primitive-homogeneity arms now execute, and the singleton arm
/// still does not fire, which is why the rule it delegates to stays tested
/// separately below against tables the catalog cannot produce.
#[test]
fn the_catalog_satisfies_the_comparability_class_invariants() {
    let rows: Vec<(Option<&'static str>, PrimitiveKind, &'static str)> = Registry::builtin()
        .ids()
        .map(|id| {
            let def = scalar_def(id);
            (
                def.comparability_class,
                def.primitive,
                scalar_def(id).canonical,
            )
        })
        .collect();
    assert_eq!(rows.len(), Registry::builtin().len());
    if let Err(violation) = check_class_invariants(&rows) {
        panic!("{violation}");
    }
}

/// The rule itself, against tables the catalog cannot produce until
/// classes are assigned. This is what makes the three invariants real
/// today rather than prose that happens to compile: corrupt any of them and
/// this test fails, even though the catalog walk above would not notice.
#[test]
fn class_invariants_reject_the_tables_the_catalog_cannot_produce_yet() {
    let ok: &[(Option<&'static str>, PrimitiveKind, &'static str)] = &[
        (
            Some("temporal_instant"),
            PrimitiveKind::String,
            "Temporal.DateTime",
        ),
        (
            Some("temporal_instant"),
            PrimitiveKind::String,
            "Temporal.Date",
        ),
        (None, PrimitiveKind::Int, "Generic.Int64"),
    ];
    assert!(
        check_class_invariants(ok).is_ok(),
        "a valid two-member class alongside an unclassed scalar"
    );

    let singleton: &[(Option<&'static str>, PrimitiveKind, &'static str)] = &[(
        Some("temporal_instant"),
        PrimitiveKind::String,
        "Temporal.DateTime",
    )];
    assert!(check_class_invariants(singleton)
        .unwrap_err()
        .contains("has one member"));

    let cross_primitive: &[(Option<&'static str>, PrimitiveKind, &'static str)] = &[
        (Some("mixed"), PrimitiveKind::String, "Temporal.DateTime"),
        (Some("mixed"), PrimitiveKind::Int, "Temporal.Seconds"),
    ];
    assert!(check_class_invariants(cross_primitive)
        .unwrap_err()
        .contains("spans two primitives"));

    let empty_name: &[(Option<&'static str>, PrimitiveKind, &'static str)] = &[
        (Some(""), PrimitiveKind::String, "Temporal.DateTime"),
        (Some(""), PrimitiveKind::String, "Temporal.Date"),
    ];
    assert!(check_class_invariants(empty_name)
        .unwrap_err()
        .contains("cannot also be a class name"));

    for bad in [
        "Temporal-Instant",
        "temporal instant",
        "temporal.instant",
        "temporal\u{e9}",
    ] {
        let rows: &[(Option<&'static str>, PrimitiveKind, &'static str)] = &[
            (Some(bad), PrimitiveKind::String, "A.One"),
            (Some(bad), PrimitiveKind::String, "A.Two"),
        ];
        assert!(
            check_class_invariants(rows)
                .unwrap_err()
                .contains("must match [a-z0-9_]+"),
            "{bad:?} must be rejected"
        );
    }
}

/// An alias must not carry a comparability class of its own. `alias_of` means
/// one implementation under two ids, so the class belongs to the target and the
/// alias inherits it -- which is what `comparable_with` reads.
///
/// Without this, the first class assignment makes an ordinary oversight -- class the target,
/// miss the alias row, or the reverse -- and the relation stops being an
/// equivalence relation. The alias pair itself still answers `true`, because
/// the identity short-circuit fires first, so the inconsistency is invisible at
/// the row where it was introduced and only surfaces one hop out, against some
/// third scalar. Still structurally undischargeable: the one alias pair
/// (`Identity.UserID` -> `Identity.UUID`) carries no class, so the loop body
/// runs but asserts `None == None`. It fires the moment an alias target is
/// classed.
#[test]
fn an_alias_never_declares_its_own_comparability_class() {
    for id in Registry::builtin().ids() {
        let def = scalar_def(id);
        let Some(target) = def.alias_of else {
            continue;
        };
        assert_eq!(
            def.comparability_class,
            None,
            "{} is an alias of {}; the class belongs to the target and the alias \
             inherits it, so declaring one here silently does nothing",
            scalar_def(id).canonical,
            scalar_def(target).canonical
        );
    }
}

/// Transitivity, which the doc comment on `comparable_with` claims and which the
/// alias short-circuit is the one thing that could break.
///
/// The failing shape: class the alias TARGET, leave the alias row alone, and ask
/// about the alias against a third scalar in the same class. Reading the raw
/// field on each def rather than the resolved one gives
/// `UserID ~ UUID = true`, `UUID ~ Other = true`, `UserID ~ Other = false`.
/// This test walks the real catalog, so it is the standing version; the
/// counter-example above is what it exists to prevent.
#[test]
fn comparability_is_an_equivalence_relation_over_the_catalog() {
    for a in Registry::builtin().ids() {
        assert!(
            Registry::builtin().comparable_with(a, a),
            "{} reflexive",
            scalar_def(a).canonical
        );
        for b in Registry::builtin().ids() {
            assert_eq!(
                Registry::builtin().comparable_with(a, b),
                Registry::builtin().comparable_with(b, a),
                "{} / {} symmetric",
                scalar_def(a).canonical,
                scalar_def(b).canonical
            );
            if !Registry::builtin().comparable_with(a, b) {
                continue;
            }
            for c in Registry::builtin().ids() {
                if !Registry::builtin().comparable_with(b, c) {
                    continue;
                }
                assert!(
                    Registry::builtin().comparable_with(a, c),
                    "transitivity: {} ~ {} and {} ~ {} but not {} ~ {}",
                    scalar_def(a).canonical,
                    scalar_def(b).canonical,
                    scalar_def(b).canonical,
                    scalar_def(c).canonical,
                    scalar_def(a).canonical,
                    scalar_def(c).canonical
                );
            }
        }
    }
}
