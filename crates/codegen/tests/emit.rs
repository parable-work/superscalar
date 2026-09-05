//! Emitter correctness: every per-language artifact derives from the core
//! registry, with one parse/normalize/validate trio per scalar, the frozen u32
//! ids carried verbatim, exact function names, and the TS brand string equal to
//! the canonical scalar form. We assert on structure, not whole-file snapshots
//! (the drift gate -- `cargo run -p superscalar-codegen -- --check` -- and
//! tests/golden.rs own byte-exactness).

mod common;

use common::rendered;
use std::fs;
use std::path::Path;
use superscalar::{scalar_def, PrimitiveKind, Registry, ScalarId};
use superscalar_codegen::{optional_lang_str_literal, Context};

fn with_context<T>(f: impl FnOnce(&Context) -> T) -> T {
    let config = common::config();
    let ctx = common::context(&config);
    f(&ctx)
}

#[test]
fn one_parse_wrapper_per_scalar() {
    with_context(|ctx| {
        let n = Registry::builtin().len();
        assert_eq!(
            ctx.entries.len(),
            n,
            "entries() must cover every registry def"
        );

        // Object-primitive scalars (canonical form is a Go struct) are skipped by
        // the Go emitter only; Python and TS still emit a wrapper for every scalar.
        let object_count = Registry::builtin()
            .defs()
            .filter(|def| def.primitive == PrimitiveKind::Object)
            .count();
        assert_eq!(object_count, 1, "object-primitive scalar count");

        let py = rendered(ctx, "python");
        let go = rendered(ctx, "go");
        let ts = rendered(ctx, "typescript");

        assert_eq!(py.matches("def parse_").count(), n, "python parse_ count");
        assert_eq!(
            go.matches("func Parse").count(),
            n - object_count,
            "go Parse count (object scalars skipped)"
        );
        assert_eq!(
            ts.matches("export function parse").count(),
            n * 2,
            "ts parse count (lenient + strict)"
        );

        // A representative non-object scalar is present in Go; the object
        // scalar is absent from Go but present in Python and TS.
        assert!(
            go.contains("func ParseContactEmail"),
            "go keeps non-object scalars"
        );
        assert!(
            !go.contains("func ParseGeoLocation"),
            "go omits object scalars"
        );
        assert!(
            py.contains("def parse_geo_location"),
            "python keeps object scalars"
        );
        assert!(
            ts.contains("export function parseGeoLocation"),
            "ts keeps object scalars"
        );
    });
}

#[test]
fn frozen_id_carried_verbatim() {
    with_context(|ctx| {
        let py = rendered(ctx, "python");
        let expected = format!("\"Contact.Email\": {}", ScalarId::CONTACT_EMAIL.as_u32());
        assert!(
            py.contains(&expected),
            "python output must carry the frozen id verbatim: {expected:?}"
        );
    });
}

#[test]
fn package_and_module_literals_come_from_config() {
    with_context(|ctx| {
        let go = rendered(ctx, "go");
        let ts = rendered(ctx, "typescript");
        let py = rendered(ctx, "python");

        assert!(go.starts_with("// @generated; do not edit\n\npackage superscalar\n"));
        assert!(ts.contains("import { backend } from \"./backend\";"));
        assert!(py.contains("\nfrom . import _native\n"));
    });

    // The same templates render another layout without a code change.
    let text = fs::read_to_string(common::config_path()).expect("read config");
    let text = text
        .replace(
            "package = \"superscalar\"\nmodule",
            "package = \"scalars\"\nmodule",
        )
        .replace(
            "backend_module = \"./backend\"",
            "backend_module = \"./native\"",
        )
        .replace(
            "native_module = \"._native\"",
            "native_module = \"superscalar._native\"",
        );
    let config = superscalar_codegen::Config::parse(&text, common::config().root).expect("parse");
    let ctx = common::context(&config);
    let go = rendered(&ctx, "go");
    let ts = rendered(&ctx, "typescript");
    let py = rendered(&ctx, "python");
    assert!(go.contains("\npackage scalars\n"));
    assert!(ts.contains("import { backend } from \"./native\";"));
    assert!(py.contains("\nfrom superscalar import _native\n"));
    assert!(!go.contains("package superscalar"));
}

#[test]
fn type_imports_add_a_reexport_block_and_silence_alias_types() {
    with_context(|ctx| {
        let ts = rendered(ctx, "typescript");
        assert!(!ts.contains("./platform"));
        assert!(ts.contains("export type JSDate = globalThis.Date;"));
    });
    let text = fs::read_to_string(common::config_path()).expect("read config");
    let text = text.replace(
        "[typescript.type_renames]",
        "[[typescript.type_imports]]\nmodule = \"./platform\"\nnames = [\"Permission\"]\n\n[typescript.type_renames]",
    );
    let config = superscalar_codegen::Config::parse(&text, common::config().root).expect("parse");
    let ctx = common::context(&config);
    let ts = rendered(&ctx, "typescript");
    assert!(ts.contains("import type {\n  Permission,\n} from \"./platform\";"));
    assert!(ts.contains("export type {\n  Permission,\n} from \"./platform\";"));
}

#[test]
fn representative_function_names_are_exact() {
    with_context(|ctx| {
        let py = rendered(ctx, "python");
        let go = rendered(ctx, "go");
        let ts = rendered(ctx, "typescript");

        assert!(
            py.contains("def parse_contact_email"),
            "python snake-case fn name"
        );
        assert!(
            go.contains("func ParseContactEmail"),
            "go PascalCase fn name"
        );
        assert!(
            ts.contains("export function parseContactEmail"),
            "ts camelCase fn name"
        );
    });
}

#[test]
fn ts_brand_equals_canonical() {
    with_context(|ctx| {
        let ts = rendered(ctx, "typescript");
        assert!(
            ts.contains("__brand: \"Contact.Email\""),
            "ts brand string must equal the canonical scalar form"
        );
    });
}

#[test]
fn symbols_are_acronym_preserving() {
    with_context(|ctx| {
        let go = rendered(ctx, "go");
        let ts = rendered(ctx, "typescript");

        assert!(
            go.contains("func ParseAuthJWT"),
            "go symbol must preserve JWT acronym"
        );
        assert!(
            !go.contains("ParseAuthJwt"),
            "go must not derive symbols from Rust enum Debug casing"
        );
        assert!(
            ts.contains("export function parseAuthJWT"),
            "ts symbol must preserve JWT acronym"
        );
        assert!(
            !ts.contains("parseAuthJwt"),
            "ts must not derive symbols from Rust enum Debug casing"
        );
        assert!(
            ts.contains("symbol: \"CryptoRSAPrivateKey\""),
            "metadata symbol must preserve RSA acronym"
        );
    });
}

#[test]
fn object_scalars_use_their_mapped_types() {
    with_context(|ctx| {
        let go = rendered(ctx, "go");
        let ts = rendered(ctx, "typescript");

        assert!(
            ts.contains("export type GeoLocation = { lat: number; lon: number };"),
            "ts object scalars use the mapped shape"
        );
        assert!(
            !ts.contains("export type GeoLocation = string &"),
            "ts object scalars must not be string branded"
        );
        assert!(
            go.contains("type GeoLocation struct{ Lat float64; Lon float64 }"),
            "go object scalars use the mapped struct"
        );
    });
}

#[test]
fn uuid_derived_scalars_remain_go_aliases() {
    with_context(|ctx| {
        let go = rendered(ctx, "go");

        assert!(
            go.contains("type IdentityUUID = UUID"),
            "Identity.UUID must remain a Go alias so generated schema UUID aliases keep UUID methods"
        );
        assert!(
            go.contains("type IdentityUserID = UUID"),
            "Identity.UserID must remain a Go alias so UserID and UUID stay assignable"
        );
    });

    // The alias comes from [go.type_overrides], not from the emitter.
    let text = fs::read_to_string(common::config_path()).expect("read config");
    let text = text.replace("\"Identity.UUID\" = \"UUID\"\n", "");
    let config = superscalar_codegen::Config::parse(&text, common::config().root).expect("parse");
    let ctx = common::context(&config);
    let go = rendered(&ctx, "go");
    assert!(go.contains("type IdentityUUID = uuid.UUID"));
}

#[test]
fn consolidated_metadata_tables_cover_non_excluded_scalars() {
    with_context(|ctx| {
        let expected = Registry::builtin()
            .defs()
            .filter(|def| !def.metadata_omit)
            .count();
        let go = rendered(ctx, "go");
        let ts = rendered(ctx, "typescript");

        assert_eq!(
            ts.matches("    canonicalName:").count(),
            expected,
            "ts metadata table must cover every non-excluded scalar"
        );
        assert!(
            ts.contains("canonicalName: \"Contact.Email\""),
            "ts metadata includes representative scalar"
        );
        assert_eq!(
            go.matches("CanonicalName:").count(),
            expected,
            "go metadata table must cover every non-excluded scalar"
        );
        assert!(
            go.contains("CanonicalName:  \"Contact.Email\""),
            "go metadata includes representative scalar"
        );
    });
}

#[test]
fn examples_and_allowlists_are_generated() {
    with_context(|ctx| {
        let go = rendered(ctx, "go");
        let ts = rendered(ctx, "typescript");

        assert!(
            ts.contains("examples: [\"test@example.com\"]"),
            "ts metadata carries registry examples"
        );
        assert!(
            go.contains("var VALID_SCALARS = []string"),
            "go emits scalar allowlist"
        );
        assert!(
            go.contains("string(\"Contact.Email\")"),
            "go scalar allowlist contains representative scalar"
        );
        // No built-in declares reserved words, so no constant is emitted.
        assert!(!ts.contains("ReservedWords = ["));
        assert!(!go.contains("ReservedWords = map[string]bool"));
    });
}

#[test]
fn legacy_aliases_render_from_the_registry() {
    // The built-in registry carries no legacy aliases; the alias blocks are
    // empty rather than absent so an extension's aliases land in the same
    // place.
    with_context(|ctx| {
        assert!(ctx.legacy_aliases.is_empty());
        let go = rendered(ctx, "go");
        let ts = rendered(ctx, "typescript");
        assert!(!go.contains("is a legacy scalar alias"));
        assert!(!ts.contains("export const parseEmail = parseContactEmail"));
    });
}

#[test]
fn phase3_cutover_removes_preview_and_compat_outputs() {
    with_context(|ctx| {
        let go = rendered(ctx, "go");
        let compat_template =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("templates/generated_compat.go.jinja");

        assert!(
            !go.contains("scalar_codegen_preview"),
            "Phase 3 promotes go/generated.go to the default build"
        );
        assert!(
            !compat_template.exists(),
            "Phase 3 removes the generated_compat.go template"
        );
    });
}

#[test]
fn generated_validators_preserve_public_contracts_without_regex_reimplementation() {
    with_context(|ctx| {
        let go = rendered(ctx, "go");
        let ts = rendered(ctx, "typescript");

        assert!(
            ts.contains(
                "import type { ScalarValidationResult, ValidationError } from \"./validation\";"
            ),
            "TS generated validators must keep the public ScalarValidationResult contract"
        );
        assert!(
            ts.contains("export function validateContactEmail(value: unknown | null | undefined): ScalarValidationResult"),
            "TS validate wrappers return the legacy tuple shape"
        );
        assert!(
            ts.contains("return validateWithBackend(8, value);"),
            "TS validate wrappers delegate to the Rust-backed backend helper"
        );
        assert!(
            !ts.contains("new RegExp"),
            "TS generated validators must not reimplement regex validation"
        );

        assert!(
            go.contains("func (v ContactEmail) Validate() (bool, []ValidationError)"),
            "Go generated scalar types keep the structural Validate interface"
        );
        assert!(
            go.contains("func (v ContactEmail) ValidateRequired() (bool, []ValidationError)"),
            "Go generated scalar types keep the structural ValidateRequired interface"
        );
        assert!(
            go.contains("return validateScalarValue(8, string(v))"),
            "Go receiver validators delegate to the Rust-backed core helper"
        );
        assert!(
            !go.contains("regexp.MustCompile"),
            "Go generated validators must not reimplement regex validation"
        );
    });
}

#[test]
fn scalar_metadata_emits_expected_table_shape() {
    with_context(|ctx| {
        let metadata = rendered(ctx, "rust_metadata");
        let expected = Registry::builtin()
            .defs()
            .filter(|def| !def.metadata_omit)
            .count();

        assert_eq!(
            metadata.matches("    ScalarMetadata {").count(),
            expected,
            "metadata output must cover every non-excluded scalar"
        );
        assert!(
            metadata.contains("pub struct ScalarMetadata"),
            "metadata output must define the public table row type"
        );
        assert!(
            metadata.contains("canonical_name: \"Contact.Email\""),
            "metadata output keeps representative scalar metadata"
        );
    });
}

/// Both semantic fields reach every generated table, with the same count as
/// the metadata table itself, and Python gains the metadata surface it never had.
///
/// Every substring below is anchored on a ROW, not on the type declaration the
/// same emitter writes above the rows. `pub is_sortable: bool,` (4-space struct
/// indent) and `isSortable: boolean;` (2-space interface indent) would otherwise
/// each add one more match than the table has rows. The anchors therefore pin
/// the template's literal indentation: if the template's row indent moves,
/// this test fails, which is the intended coupling.
///
/// The `comparability_class` anchors count the KEY and not the value, so they
/// survived the first class assignment unchanged. The all-`None` state used to
/// be asserted twice in `core/tests/semantic_metadata.rs`: once as a FIELD, in
/// the tripwire, and once as a RELATION, in the exhaustive cross-pair walk,
/// which a grep for `comparability_class` does not surface. The first class
/// assignment rewrote both into positive assertions.
#[test]
fn semantic_metadata_fields_reach_every_generated_table() {
    with_context(|ctx| {
        let expected = Registry::builtin()
            .defs()
            .filter(|def| !def.metadata_omit)
            .count();
        // Derived from the registry, never hardcoded: the emitted booleans are
        // compared against the predicate that produced them, so a constant-emitting
        // template fails, and adding a scalar updates one place -- the catalog.
        let non_sortable = Registry::builtin()
            .defs()
            .filter(|def| !def.metadata_omit)
            .filter(|def| !def.is_sortable())
            .count();
        let sortable = expected - non_sortable;
        let rust = rendered(ctx, "rust_metadata");
        let go = rendered(ctx, "go");
        let ts = rendered(ctx, "typescript");
        let py = rendered(ctx, "python");

        // Rust rows are indented 8 spaces inside `ScalarMetadata { .. }`; the struct
        // declaration is at 4 and reads `pub is_sortable: bool,`.
        assert_eq!(
            rust.matches("        comparability_class: ").count(),
            expected
        );
        assert_eq!(rust.matches("        is_sortable: true,").count(), sortable);
        assert_eq!(
            rust.matches("        is_sortable: false,").count(),
            non_sortable
        );
        // Go rows are tab-tab indented; a Go struct field declaration carries no colon,
        // so `IsSortable:` is already row-only.
        assert_eq!(go.matches("\t\tComparabilityClass: ").count(), expected);
        assert_eq!(go.matches("\t\tIsSortable: true,").count(), sortable);
        assert_eq!(go.matches("\t\tIsSortable: false,").count(), non_sortable);
        // TS rows are indented 4 spaces; the interface declaration is at 2.
        assert_eq!(ts.matches("    comparabilityClass: ").count(), expected);
        assert_eq!(ts.matches("    isSortable: true,").count(), sortable);
        assert_eq!(ts.matches("    isSortable: false,").count(), non_sortable);
        // Python: ONE row-shaped dict of dicts, matching the corpus row shape
        // exactly. Each row is
        //   "Contact.Email": {"comparability_class": None, "is_sortable": True},
        assert!(py.contains("SCALAR_METADATA = {"));
        assert_eq!(py.matches("\"comparability_class\": ").count(), expected);
        assert_eq!(py.matches("\"is_sortable\": True},").count(), sortable);
        assert_eq!(py.matches("\"is_sortable\": False},").count(), non_sortable);
        // Python floor guard: the generated module must evaluate on the declared
        // 3.9 floor, so it imports nothing beyond `_native` and annotates with
        // builtins only. `comparable_with(a: str, b: str) -> bool` annotates with
        // builtins and so evaluates on 3.9; what stays banned is an annotation
        // needing `typing` or PEP 604, which would raise at def time there.
        // `python/scripts/py39_floor_check.py` execs the module on a real 3.9 and
        // is the proof; these two are the cheap in-suite tripwire.
        assert!(!py.contains("from typing"));
        assert!(!py.contains("__future__"));
        // No metadata row may carry a meaningless empty JSON Schema type.
        assert!(!rust.contains("json_schema_type: Some(\"\")"));
    });
}

/// The count assertions above would also pass on a template that hardcoded the
/// empty/None literal and never read the entry. Doctor one entry and re-emit, so
/// the class provably travels from `Entry` into the Go, TS and Python templates.
///
/// The doctored literal must be a string the catalog CANNOT produce. It was
/// `"temporal_instant"` until that shipped as a real class name, at which point
/// the unscoped `contains` checks on the Go and TS output were satisfied by the
/// genuine `Temporal.Date` / `Temporal.DateTime` rows and the doctoring stopped
/// carrying the test: deleting it left this green. Only the Python assertion
/// survived, because it is scoped to the `Contact.Email` key.
#[test]
fn a_named_comparability_class_reaches_the_go_ts_and_python_templates() {
    // The recurrence detector, rendered BEFORE the doctoring: re-emit without
    // doctoring and require the sentinel to be absent. If someone ever picks a
    // sentinel the catalog CAN produce, this fails immediately, instead of the
    // assertions below quietly ceasing to discriminate the way they did when
    // the sentinel was "temporal_instant".
    with_context(|ctx| {
        for language in ["go", "typescript", "python"] {
            assert!(
                !rendered(ctx, language).contains("never_a_real_class"),
                "the {language} render carries the sentinel without doctoring, so the \
                 sentinel is a class the catalog can produce and this test no longer \
                 proves the field travels; pick one the catalog cannot produce"
            );
        }
    });

    let config = common::config();
    let mut ctx = common::context(&config);
    let target = ctx
        .entries
        .iter_mut()
        .find(|e| e.canonical == "Contact.Email")
        .expect("Contact.Email is catalogued and carries a metadata row");
    target.comparability_class_go = "\"never_a_real_class\"".to_string();
    target.comparability_class_ts = "\"never_a_real_class\"".to_string();
    target.comparability_class_py = "\"never_a_real_class\"".to_string();

    let go = rendered(&ctx, "go");
    let ts = rendered(&ctx, "typescript");
    let py = rendered(&ctx, "python");

    // `count() == 1`, not `contains`. An unscoped `contains` is what let this
    // test decay silently once the sentinel became a real class name: the render
    // carried genuine occurrences and the assertion passed without the doctoring.
    assert_eq!(
        go.matches("ComparabilityClass: \"never_a_real_class\",")
            .count(),
        1
    );
    assert_eq!(
        ts.matches("comparabilityClass: \"never_a_real_class\",")
            .count(),
        1
    );
    assert_eq!(
        py.matches("\"Contact.Email\": {\"comparability_class\": \"never_a_real_class\",")
            .count(),
        1
    );
}

/// The doctored-entry test above assigns the ALREADY-SHAPED fields, so it proves
/// the templates read them but never exercises the shaper or the `entries`
/// population that calls it. Every real catalog value was `None` when this was
/// written, so the `Some(..)` branch was dead until the first class was
/// assigned -- which is the exact situation the doctored test was added to
/// prevent, one hop upstream. This closes the `ScalarDef -> Entry` hop, which
/// is what makes the acceptance criterion "a non-None class is proven to travel
/// end to end" true.
#[test]
fn the_class_shaper_produces_each_language_none_literal_and_quotes_a_name() {
    assert_eq!(optional_lang_str_literal(None, "\"\""), "\"\"");
    assert_eq!(optional_lang_str_literal(None, "null"), "null");
    assert_eq!(optional_lang_str_literal(None, "None"), "None");
    for none_literal in ["\"\"", "null", "None"] {
        assert_eq!(
            optional_lang_str_literal(Some("temporal_instant"), none_literal),
            "\"temporal_instant\""
        );
    }
    // The shipped population wires the right none-literal to the right language.
    with_context(|ctx| {
        let contact = ctx
            .entries
            .iter()
            .find(|e| e.canonical == "Contact.Email")
            .expect("Contact.Email is catalogued");
        assert_eq!(contact.comparability_class_go, "\"\"");
        assert_eq!(contact.comparability_class_ts, "null");
        assert_eq!(contact.comparability_class_py, "None");
        assert!(contact.is_sortable);
    });
}

/// `comparability_class` is data; the predicate that reads it is the
/// contract, and the bindings must ship both. The predicate is GENERATED, not
/// hand-written per language, so this asserts on the emitter output rather than
/// on the checked-in files.
///
/// Each anchor is the function signature plus the two clauses a naive
/// reimplementation gets wrong: the miss check that fails closed, and the
/// non-empty class guard that keeps two class-less scalars apart. Anchoring on
/// the signature alone would pass on a body that is `return a == b`.
#[test]
fn the_comparability_predicate_reaches_every_binding() {
    with_context(|ctx| {
        let go = rendered(ctx, "go");
        let ts = rendered(ctx, "typescript");
        let py = rendered(ctx, "python");

        assert!(go.contains("func ComparableWith(a, b string) bool {"));
        assert!(go.contains("if metaA == nil || metaB == nil {"));
        assert!(go.contains("return metaA.ComparabilityClass != \"\" && metaA.ComparabilityClass == metaB.ComparabilityClass"));

        assert!(ts.contains("export function comparableWith(a: string, b: string): boolean {"));
        assert!(ts.contains(
            "!Object.prototype.hasOwnProperty.call(SCALAR_METADATA_BY_CANONICAL, left) ||"
        ));
        assert!(ts.contains("metaLeft.comparabilityClass !== null &&"));

        assert!(py.contains("def comparable_with(a: str, b: str) -> bool:"));
        assert!(py.contains("if meta_a is None or meta_b is None:"));
        assert!(py
            .contains("return class_a is not None and class_a == meta_b[\"comparability_class\"]"));
    });
}

/// The predicate resolves an alias before it reads a class, so each binding
/// needs the alias table too. `Identity.UserID -> Identity.UUID` is the only
/// pair in the catalog; the rows are derived from `alias_of` so a second pair
/// updates one place, the catalog.
#[test]
fn the_alias_table_reaches_every_binding() {
    let aliases: Vec<(&str, &str)> = Registry::builtin()
        .defs()
        .filter_map(|def| {
            def.alias_of
                .map(|target| (def.canonical, scalar_def(target).canonical))
        })
        .collect();
    assert!(!aliases.is_empty(), "the assertions below would be vacuous");
    with_context(|ctx| {
        let go = rendered(ctx, "go");
        let ts = rendered(ctx, "typescript");
        let py = rendered(ctx, "python");

        assert!(go.contains("var scalarAliasTargets = map[string]string{"));
        assert!(ts.contains("const SCALAR_ALIAS_TARGETS: Record<string, string> = {"));
        // Underscore-prefixed so `from ._generated import *` does not export it.
        assert!(py.contains("_SCALAR_ALIAS_TARGETS = {"));
        assert!(!py.contains("\nSCALAR_ALIAS_TARGETS = {"));

        // Row indentation differs per language: Go a tab, TS 2 spaces, Python 4.
        for (alias, target) in &aliases {
            let row = format!("{alias:?}: {target:?},");
            assert_eq!(
                go.matches(&format!("\t{row}")).count(),
                1,
                "go row for {alias}"
            );
            assert_eq!(
                ts.matches(&format!("  {row}")).count(),
                1,
                "ts row for {alias}"
            );
            assert_eq!(
                py.matches(&format!("    {row}")).count(),
                1,
                "py row for {alias}"
            );
        }
    });
}

/// The emitted alias table above would also pass on a template that hardcoded
/// the one pair, so close the `ScalarDef -> Entry` hop the same way the
/// comparability-class shaper test does: the flag and the literal come from
/// `alias_of`, for every scalar, not just the one that has it.
#[test]
fn the_alias_shaper_marks_exactly_the_catalogued_aliases() {
    with_context(|ctx| {
        for entry in &ctx.entries {
            let def = Registry::builtin()
                .by_canonical(&entry.canonical)
                .expect("every entry is a catalogued scalar");
            match def.alias_of {
                Some(target) => {
                    assert!(entry.is_alias, "{} is an alias", entry.canonical);
                    assert_eq!(
                        entry.alias_target_literal,
                        format!("{:?}", scalar_def(target).canonical)
                    );
                }
                None => {
                    assert!(!entry.is_alias, "{} is not an alias", entry.canonical);
                    assert!(entry.alias_target_literal.is_empty());
                }
            }
        }
    });
}
