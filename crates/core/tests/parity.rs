//! The parity gate: the core must reproduce every vector in
//! `conformance/core-scalars.v2.json`. Accepted inputs must `parse` to their declared
//! canonical form; rejected inputs must fail with the declared error category.
//!
//! This is the executable G2 contract -- the audit lives as asserted vectors,
//! not prose.

use serde::Deserialize;
use superscalar::{scalar_def, Registry};

const VECTORS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/core-scalars.v2.json"
));

#[derive(Deserialize)]
struct Vectors {
    meta: Meta,
    scalars: std::collections::BTreeMap<String, Cases>,
    // `default` is load-bearing during Red: without it the field is required and
    // `serde_json::from_str::<Vectors>` fails inside EVERY existing test in this
    // file while the corpus section is absent. With it, the section's absence
    // shows up as an empty map and fails only the new count assertion below --
    // and it keeps failing loudly if the section is ever deleted.
    #[serde(default)]
    metadata: std::collections::BTreeMap<String, MetadataVector>,
    #[serde(default)]
    metadata_excluded: Vec<String>,
}

/// One row of the corpus `metadata` section. `comparability_class` is
/// `Option<String>` because the corpus encodes "no class" as JSON `null`, which
/// is the shape the Rust and TS tables use; Go flattens it to `""` on its own
/// side (A16).
/// The corpus header. `non_sortable_count` is hand-maintained beside the
/// generated `metadata` section on purpose: an expectation derived from the
/// transcript it checks would assert nothing.
#[derive(Deserialize)]
struct Meta {
    non_sortable_count: usize,
    /// Class name -> sorted member canonical names. Hand-
    /// maintained on the same terms as `non_sortable_count`, and a member list
    /// rather than a count because a count cannot catch a class attached to the
    /// wrong scalar.
    comparability_classes: std::collections::BTreeMap<String, Vec<String>>,
}

#[derive(Deserialize)]
struct MetadataVector {
    comparability_class: Option<String>,
    is_sortable: bool,
}

#[derive(Deserialize)]
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

#[test]
fn core_reproduces_every_v2_vector() {
    let vectors: Vectors = serde_json::from_str(VECTORS).expect("v2 vectors parse");
    let mut failures: Vec<String> = Vec::new();
    let mut checked = 0usize;

    for (canonical, cases) in &vectors.scalars {
        let id = match Registry::builtin().by_canonical(canonical) {
            Some(def) => def.id,
            None => {
                failures.push(format!("unknown canonical scalar id: {canonical}"));
                continue;
            }
        };
        let registry = Registry::builtin();
        let scalar = registry.scalar(id).expect("assembled id has a scalar");

        for case in &cases.accepted {
            if case.unresolved {
                continue;
            }
            checked += 1;
            match scalar.parse(registry, &case.input) {
                Ok(got) => {
                    if let Some(expected) = case.normalized.as_ref().filter(|&e| &got != e) {
                        failures.push(format!(
                            "{canonical}: parse({:?}) = {:?}, expected {:?}",
                            case.input, got, expected
                        ));
                    }
                }
                Err(err) => failures.push(format!(
                    "{canonical}: parse({:?}) rejected ({}), expected accept",
                    case.input, err
                )),
            }
        }

        for case in &cases.rejected {
            if case.unresolved {
                continue;
            }
            checked += 1;
            match scalar.parse(registry, &case.input) {
                Ok(got) => failures.push(format!(
                    "{canonical}: parse({:?}) = {:?}, expected reject",
                    case.input, got
                )),
                Err(err) => {
                    if let Some(expected) =
                        case.validator.as_ref().filter(|&e| err.kind.as_str() != e)
                    {
                        failures.push(format!(
                            "{canonical}: parse({:?}) failed as {:?}, expected validator {:?}",
                            case.input,
                            err.kind.as_str(),
                            expected
                        ));
                    }
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} parity failures (of {checked} checked):\n{}",
        failures.len(),
        failures.join("\n")
    );
    assert!(checked > 0, "no vectors checked");
}

/// Canonical output must be a fixed point of `parse`: re-parsing an accepted
/// vector's canonical form yields the same value. Inherited from the retired
/// differential harness (the legacy runners it diffed against were deleted in
/// the per-language cutovers); the invariant itself is permanent.
#[test]
fn parse_is_idempotent_on_canonical_output() {
    let vectors: Vectors = serde_json::from_str(VECTORS).expect("v2 vectors parse");
    let mut failures: Vec<String> = Vec::new();

    for (canonical, cases) in &vectors.scalars {
        let id = Registry::builtin()
            .by_canonical(canonical)
            .expect("known canonical scalar id")
            .id;
        let registry = Registry::builtin();
        let scalar = registry.scalar(id).expect("assembled id has a scalar");
        for case in &cases.accepted {
            if case.unresolved {
                continue;
            }
            let Ok(value) = scalar.parse(registry, &case.input) else {
                // core_reproduces_every_v2_vector reports the acceptance failure.
                continue;
            };
            match scalar.parse(registry, &value) {
                Ok(reparsed) if reparsed == value => {}
                Ok(reparsed) => failures.push(format!(
                    "{canonical}: parse({:?}) = {:?} not a fixed point (reparse = {:?})",
                    case.input, value, reparsed
                )),
                Err(err) => failures.push(format!(
                    "{canonical}: canonical {:?} rejected on reparse ({err})",
                    value
                )),
            }
        }
    }

    assert!(
        failures.is_empty(),
        "core canonical form is not idempotent:\n{}",
        failures.join("\n")
    );
}

#[test]
fn v2_covers_every_catalog_scalar() {
    let vectors: Vectors = serde_json::from_str(VECTORS).expect("v2 vectors parse");
    let missing: Vec<&str> = Registry::builtin()
        .ids()
        .map(|id| scalar_def(id).canonical)
        .filter(|canonical| !vectors.scalars.contains_key(*canonical))
        .collect();
    assert!(
        missing.is_empty(),
        "v2 must cover all {} catalog scalars; missing: {missing:?}",
        Registry::builtin().len()
    );
}

/// A scalar's DECLARED shape must agree with the shape its implementation PRODUCES.
///
/// The catalog's `pattern`/`min_length`/`max_length` are not decoration: a schema toolchain bakes
/// them into the JSON Schema, the Go validators and the TypeScript validators, so they
/// are what an API request is checked against at the edge. The implementation is what
/// runs afterwards. Nothing forced the two to agree, and they silently drifted:
/// a downstream handle scalar was moved to camelCase while its declared pattern
/// stayed kebab, so every camelCase handle the client produced was rejected with a 400
/// before it ever reached the scalar that had just minted it.
///
/// Checking canonical output against the declared pattern closes that gap for all 57
/// scalars at once: if an implementation's output shape moves, this fails until the
/// declaration follows.
#[test]
fn declared_pattern_accepts_every_canonical_form() {
    use superscalar::scalar_metadata_by_canonical_name;

    let vectors: Vectors = serde_json::from_str(VECTORS).expect("v2 vectors parse");
    let mut failures: Vec<String> = Vec::new();
    let mut checked = 0usize;

    for (canonical, cases) in &vectors.scalars {
        // No metadata means the scalar emits no schema at all (`schema_omit`, a
        // secret reference for example), so it has no declaration to contradict.
        let Some(meta) = scalar_metadata_by_canonical_name(canonical) else {
            continue;
        };
        // `pattern`, `minLength` and `maxLength` are string-only JSON Schema keywords:
        // against an object-typed schema they are inert and gate nothing. Geo.Location
        // declares a pattern for its INPUT form while emitting an object, so asserting
        // it here would be checking a rule no request is ever held to.
        if meta.json_schema_type != Some("string") {
            continue;
        }

        let compiled = meta.pattern.map(|p| {
            regex::Regex::new(p)
                .unwrap_or_else(|e| panic!("{canonical}: declared pattern {p:?} must compile: {e}"))
        });

        for case in &cases.accepted {
            if case.unresolved {
                continue;
            }
            // The canonical form is what gets stored and sent back over the wire, so it
            // is the value that has to survive the declared pattern.
            let Some(value) = case.normalized.as_ref() else {
                continue;
            };
            checked += 1;

            if let Some(re) = compiled.as_ref() {
                if !re.is_match(value) {
                    failures.push(format!(
                        "{canonical}: parse({:?}) = {value:?}, which its DECLARED pattern {:?} rejects",
                        case.input,
                        meta.pattern.unwrap()
                    ));
                }
            }

            let len = value.chars().count();
            if let Some(max) = meta.max_length {
                if len > max {
                    failures.push(format!(
                        "{canonical}: canonical {value:?} is {len} chars, over declared max_length {max}"
                    ));
                }
            }
            if let Some(min) = meta.min_length {
                if len < min {
                    failures.push(format!(
                        "{canonical}: canonical {value:?} is {len} chars, under declared min_length {min}"
                    ));
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} declaration/implementation mismatches (of {checked} canonical forms checked):\n{}",
        failures.len(),
        failures.join("\n")
    );
    assert!(checked > 0, "no canonical forms checked");
}

/// The corpus is the cross-language oracle: Go, Python and both TS backends
/// assert against this same section. Rust asserting it here is what keeps the
/// corpus from silently drifting away from the generated table.
#[test]
fn v2_metadata_section_matches_the_generated_table() {
    use superscalar::{scalar_metadata_by_canonical_name, SCALAR_METADATA};

    let vectors: Vectors = serde_json::from_str(VECTORS).expect("v2 vectors parse");
    assert_eq!(
        vectors.metadata.len(),
        SCALAR_METADATA.len(),
        "metadata section must cover every metadata row"
    );
    for (canonical, want) in &vectors.metadata {
        let md = scalar_metadata_by_canonical_name(canonical)
            .unwrap_or_else(|| panic!("{canonical} has a metadata vector but no metadata row"));
        assert_eq!(
            md.comparability_class,
            want.comparability_class.as_deref(),
            "{canonical}"
        );
        assert_eq!(md.is_sortable, want.is_sortable, "{canonical}");
    }
    // One independent invariant at the corpus layer. The section is a
    // TRANSCRIPT of the emitted table, so if the emitter were wrong the
    // corpus would be wrong the same way and the per-row loop above would still
    // pass. Ten is pinned independently by the Phase 1 eleven-name set.
    assert_eq!(
        vectors.metadata.values().filter(|m| !m.is_sortable).count(),
        vectors.meta.non_sortable_count
    );
    // Same independence argument, for comparability. Rows with no class are
    // skipped rather than grouped under an empty key, so the derived map holds
    // only real classes.
    let mut derived: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    for (canonical, vector) in &vectors.metadata {
        if let Some(class) = &vector.comparability_class {
            derived
                .entry(class.clone())
                .or_default()
                .push(canonical.clone());
        }
    }
    for members in derived.values_mut() {
        members.sort();
    }
    assert_eq!(derived, vectors.meta.comparability_classes);
    // The two sections legitimately differ by one key, and the corpus now says
    // which one rather than leaving each reader to hardcode the name.
    // Both sides sorted, matching the Go and TypeScript readers. `scalars` is a
    // BTreeMap so its keys arrive sorted, but `metadata_excluded` is a Vec in
    // file order; comparing them directly is order-sensitive and would diverge
    // from the other readers the moment a second scalar is excluded.
    let mut missing: Vec<&str> = vectors
        .scalars
        .keys()
        .filter(|c| !vectors.metadata.contains_key(*c))
        .map(String::as_str)
        .collect();
    missing.sort_unstable();
    let mut declared: Vec<&str> = vectors
        .metadata_excluded
        .iter()
        .map(String::as_str)
        .collect();
    declared.sort_unstable();
    assert_eq!(
        missing, declared,
        "scalars minus metadata must equal the declared metadata_excluded"
    );
}

/// The shipped corpus carries `null` in all 60 rows, so the `Some(..)` arm of
/// `MetadataVector::comparability_class` is dead against it, and the first class
/// assignment would be the first thing to decode a real one. Feed the branch a literal
/// row instead of adding a synthetic corpus entry (which would break the
/// length assertion in all four readers). The same test exists in Go
/// (`TestMetadataVectorDecodesANamedClass`).
#[test]
fn a_named_class_decodes_from_a_metadata_vector() {
    let v: MetadataVector =
        serde_json::from_str(r#"{"comparability_class": "temporal_instant", "is_sortable": true}"#)
            .expect("vector parses");
    assert_eq!(v.comparability_class.as_deref(), Some("temporal_instant"));
    assert!(v.is_sortable);
}
