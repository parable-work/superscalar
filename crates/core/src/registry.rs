use crate::catalog::{ScalarId, CATALOG};
use crate::directive::DirectiveScalar;
use crate::error::ScalarError;
use crate::extension::{AssembleOptions, AssemblyError, Extension, ExtensionInfo, LegacyAlias};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, LazyLock};

/// Declares `PrimitiveKind` and both of the spellings each member is written in.
///
/// One invocation owns the whole (member, IR type-ref name, DSL name) triple.
/// A member cannot exist outside this list, so it cannot exist without both
/// names, and the four accessors below are generated from the same row rather
/// than restating it. That is the property a producer and a consumer on two
/// sides of a wire need: one writes the IR spelling into an emitted schema,
/// the other reads it back out as a DSL primitive, and neither can see the
/// other's mapping.
macro_rules! primitive_kinds {
    ($(
        $(#[$member_doc:meta])*
        ($member:ident, $type_ref_name:literal, $dsl_name:literal)
    ),+ $(,)?) => {
        /// Primitive backing of a scalar's canonical value. The DSL's
        /// object-shaped `"Type"` maps to `Object`; `Bool` is reserved (no
        /// current scalar uses it).
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
        pub enum PrimitiveKind {
            $(
                $(#[$member_doc])*
                $member,
            )+
        }

        impl PrimitiveKind {
            /// Every member, in declaration order. Generated with the enum, so
            /// it can never fall behind it.
            pub const ALL: &'static [PrimitiveKind] = &[$(PrimitiveKind::$member),+];

            /// The DSL spelling, used by the schema catalog emitters
            /// (`Object` -> `"Type"`).
            pub fn dsl_name(self) -> &'static str {
                match self {
                    $(PrimitiveKind::$member => $dsl_name,)+
                }
            }

            /// The inverse of [`PrimitiveKind::dsl_name`], `None` for any other
            /// spelling.
            ///
            /// A consumer reading a primitive back off the wire would otherwise
            /// write its own match over these names and become a mirror of
            /// this enum that drifts the first time it gains a member.
            pub fn from_dsl_name(name: &str) -> Option<Self> {
                match name {
                    $($dsl_name => Some(PrimitiveKind::$member),)+
                    _ => None,
                }
            }

            /// The schema-IR type-ref spelling, used when a primitive is named
            /// as a `typeRef` inside an emitted schema.
            ///
            /// It is NOT [`PrimitiveKind::dsl_name`]: the IR writes `Boolean`
            /// where the DSL writes `Bool`, and `JSON` where the DSL writes
            /// `Type`. Both spellings are real and neither is a typo, so they
            /// are declared side by side instead of being converted. `JSON` is
            /// the BARE object spelling, not the dotted `Generic.JSON`: that is
            /// a catalog scalar, and a column whose type was never resolved to
            /// a scalar must not name one.
            pub fn type_ref_name(self) -> &'static str {
                match self {
                    $(PrimitiveKind::$member => $type_ref_name,)+
                }
            }

            /// The inverse of [`PrimitiveKind::type_ref_name`], `None` for any
            /// other spelling -- including a `scalars/{canonical}` reference,
            /// which names a scalar rather than a bare primitive and is resolved
            /// through the scalar catalog instead.
            pub fn from_type_ref_name(name: &str) -> Option<Self> {
                match name {
                    $($type_ref_name => Some(PrimitiveKind::$member),)+
                    _ => None,
                }
            }
        }
    };
}

primitive_kinds!(
    (String, "String", "String"),
    (Int, "Int", "Int"),
    (Float, "Float", "Float"),
    (Bool, "Boolean", "Bool"),
    (Object, "JSON", "Type"),
);

/// How a scalar's behavior is realized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ScalarTag {
    /// Hand-written parse/normalize/validate beyond directive checks.
    CustomLogic,
    /// Fully described by its directives; validator generated from the def.
    PatternOnly,
    /// Object-shaped value with a typed metadata struct.
    Structural,
    /// Valid values are an enumerated set from an external source.
    SetValued,
}

/// Upload constraints for a file-shaped scalar, mirrored verbatim into the
/// generated per-language catalogs (e.g. the TS runtime's `BUILTIN_SCALARS`).
#[derive(Serialize)]
pub struct FileUploadConfig {
    pub max_size: u64,
    /// Permitted MIME types; an empty slice means "any" (emitted as `[]`).
    pub allowed_types: &'static [&'static str],
    pub category: &'static str,
}

/// Additional constraints for an image-shaped scalar, mirrored verbatim into
/// the generated per-language catalogs. Every field is optional; only the set
/// ones are emitted. Today only `require_transparency` is exercised.
#[derive(Serialize)]
pub struct ImageConstraints {
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub min_aspect_ratio: Option<f64>,
    pub max_aspect_ratio: Option<f64>,
    pub require_transparency: Option<bool>,
}

/// Which hooks generated schema runtimes must delegate to the core instead of
/// running primitive-only behaviour. Data on the def, read by codegen; it does
/// not change how the core dispatches.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct ScalarHooks {
    pub parse: bool,
    pub normalize: bool,
    pub validate: bool,
}

impl ScalarHooks {
    /// No hook set: the generated runtimes run primitive-only behaviour.
    pub const NONE: ScalarHooks = ScalarHooks {
        parse: false,
        normalize: false,
        validate: false,
    };
}

/// Catalog definition of a scalar -- the metadata formerly declared as
/// directives in the deleted `scalars.graphql` catalog. Per-language bindings
/// are generated from these.
pub struct ScalarDef {
    pub id: ScalarId,
    /// The canonical prefix before the first '.', e.g. "Contact". Redundant with
    /// `canonical` and asserted equal at assembly; present so codegen and docs
    /// can group without string-splitting in three template languages.
    pub namespace: &'static str,
    pub canonical: &'static str,
    pub primitive: PrimitiveKind,
    pub sql_type: &'static str,
    pub metadata_primitive: &'static str,
    pub json_schema_type: &'static str,
    pub tag: ScalarTag,
    pub pattern: Option<&'static str>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub case_insensitive: bool,
    pub reserved_words: &'static [&'static str],
    pub examples: &'static [&'static str],
    pub description: &'static str,
    /// Per-language type representations, in canonical emission order
    /// (typescript, python, go, rust, sql, json_schema). `sql` is absent for
    /// the two scalars that have no SQL mapping. Each tuple is (language, type).
    pub type_mappings: &'static [(&'static str, &'static str)],
    /// Upload constraints for file-shaped scalars.
    pub file_upload: Option<&'static FileUploadConfig>,
    /// Extra image constraints.
    pub image_constraints: Option<&'static ImageConstraints>,
    /// The scalar's long-form documentation block. Empty string means no
    /// docstring.
    pub docstring: &'static str,
    /// If set, this id shares the target's implementation (e.g.
    /// Identity.UserID -> Identity.UUID). One impl, two ids.
    pub alias_of: Option<ScalarId>,
    /// When the registry's runtime `primitive` deliberately diverges from the
    /// not-yet-ratified schema declaration, this holds the DSL primitive the
    /// generated schema catalogs must still emit so they match the audited
    /// baseline. Geo.Location is the only case today: runtime `Object`
    /// but schema-declared `String`. `None` means emit the
    /// runtime primitive.
    pub schema_primitive_override: Option<&'static str>,
    /// When true the scalar exists in the registry but is intentionally omitted
    /// from the generated schema catalogs (e.g. the TS runtime's
    /// `BUILTIN_SCALARS`) until its promotion is ratified.
    /// No built-in sets it; a downstream secret-reference scalar is the
    /// motivating case.
    pub schema_omit: bool,
    /// Named format hint carried into the TS builtin catalog (`format`).
    /// No scalar declares one today; the registry field exists so the emitter
    /// sources it here instead of hardcoding the empty default.
    pub format: Option<&'static str>,
    /// When true, reserved-word matching is case-insensitive. Carried into the
    /// TS builtin catalog (`reservedWordsCaseInsensitive`).
    pub reserved_words_case_insensitive: bool,
    /// When true, reserved-word matching also rejects partial matches. Carried
    /// into the TS builtin catalog (`reservedWordsMatchPartial`); same
    /// rationale as `reserved_words_case_insensitive`.
    pub reserved_words_match_partial: bool,
    /// Equivalence class for cross-scalar comparison. `None`, the default and
    /// the value every built-in row but two carries, means self-comparable
    /// only: `Contact.Email = Contact.Email` is legal, `Contact.Email =
    /// Contact.PhoneNumber` is not, even though both are `Utf8`. Named classes
    /// are for scalars that legitimately compare across types. The first and
    /// so far only one is `temporal_instant`, over `Temporal.Date` and
    /// `Temporal.DateTime`. The relation is a DURABLE CONTRACT: saved queries
    /// validate against it, so tightening a class breaks stored queries on
    /// their next scheduled run. Classes are easy to add and painful to
    /// remove.
    ///
    /// The class STRING SET IS DELIBERATELY OPEN: this is free-form, unvalidated
    /// text, and there is no `COMPARABILITY_CLASSES` constant to check a new
    /// value against. One class is a thin basis for freezing a four-language
    /// vocabulary, and tightening it later is a codegen break plus a corpus
    /// rewrite, so the set stays open until more classes exist. The bound a
    /// closed enum would add already exists outside Rust and is stronger for
    /// being independent: `meta.comparability_classes` in
    /// `conformance/core-scalars.v2.json` is HAND-WRITTEN, and
    /// `conformance/gen_core_scalars_metadata.py` cannot write it (it rewrites
    /// exactly one other key and aborts if any other byte moved). A hand-typed
    /// member list cross-checked by four readers against a machine-generated
    /// transcript catches the one thing the in-Rust invariants cannot: two
    /// scalars sharing a misspelling.
    ///
    /// Five standing invariants in `crates/core/tests/semantic_metadata.rs`
    /// bound what an open set can silently get wrong: a class name must match
    /// `[a-z0-9_]+` and may not be empty; no class may have exactly one member;
    /// no class may span two `PrimitiveKind`s; no class may span two SQL types
    /// with no coercion between them (`PrimitiveKind` is the backing of the
    /// canonical STRING form and cannot see that `temporal_instant` spans `DATE`
    /// and `TIMESTAMPTZ`); and an alias may not declare a class of its own,
    /// since the class belongs to the alias target. All five are table-tested
    /// against tables the catalog cannot produce, because a walk over an
    /// almost-empty class table asserts nothing.
    ///
    /// A sixth bound lives outside Rust: `meta.comparability_classes` pins each
    /// class name to its exact member list, and all four bindings assert it.
    pub comparability_class: Option<&'static str>,
    /// Which hooks the generated schema runtimes delegate to the core.
    pub hooks: ScalarHooks,
    /// Excluded from every binding's SCALAR_METADATA table and from the corpus
    /// `metadata` section. No built-in sets it; the motivating case is a
    /// secret reference, which is never a queryable data column.
    pub metadata_omit: bool,
}

/// The sortability rule, as a fail-closed allowlist over a scalar's declared
/// JSON shape. Split out from `ScalarDef::is_sortable` so it can be table-tested
/// against shapes the catalog does not contain today -- a mistyped `"json"`, a
/// capitalized `"Object"`, an empty declaration.
fn json_shape_is_sortable(json_schema_type: &str) -> bool {
    matches!(
        json_schema_type,
        "string" | "integer" | "number" | "boolean"
    )
}

/// The comparability relation over an alias-and-class TABLE, extracted from
/// [`Registry::comparable_with`] so both of its arms can be table-tested.
///
/// `lookup` answers, for one key, that row's two RAW fields as a pair:
/// `(alias_of, comparability_class)`. Resolving the alias is this function's job
/// and deliberately not the caller's. That is the entire reason the extraction
/// exists: the defect this guards against is reading the class off the raw row
/// instead of off the alias TARGET, so if the caller did the resolving, the
/// mutation would live outside the unit under test and a table test would prove
/// nothing.
///
/// Undischargeable against the built-in catalog, which is why it takes a lookup
/// rather than reading a registry directly. Two separate holes, one cause -- a
/// guard whose input the catalog cannot produce:
///
/// * The ALIAS arm. `Identity.UserID` -> `Identity.UUID` is the only alias pair
///   and neither row carries a class, so the arm compares `None` against `None`
///   and answers the same either way.
/// * The TRANSITIVITY arm. With exactly one two-member class there is no
///   pairwise-distinct triple `a ~ b`, `b ~ c`, so the transitivity loop in
///   `comparability_is_an_equivalence_relation_over_the_catalog` can only reach
///   the reflexive and symmetric cases it already covers.
///
/// A mutation probe measured both: swapping the resolved read for a raw-field
/// read produced ZERO test failures across the workspace. Synthetic tables in
/// `comparability_rule_tests` close them. Same remedy, and the same reason for
/// it, as `json_shape_is_sortable` above and `check_class_invariants` in
/// `crates/core/tests/semantic_metadata.rs`.
///
/// Alias resolution is SINGLE-HOP, matching [`Registry::resolved`] exactly
/// rather than improving on it. A chain-following version here would be a
/// second, more permissive rule than the one production runs, and the point of
/// the extraction is that the tested rule and the shipped rule are one function.
fn comparable_in<K, F>(left: K, right: K, lookup: F) -> bool
where
    K: Copy + PartialEq,
    F: Fn(K) -> (Option<K>, Option<&'static str>),
{
    let left = lookup(left).0.unwrap_or(left);
    let right = lookup(right).0.unwrap_or(right);
    if left == right {
        return true;
    }
    match (lookup(left).1, lookup(right).1) {
        (Some(left), Some(right)) => left == right,
        _ => false,
    }
}

/// The dot-stripped symbol a canonical name becomes in every generated
/// binding: `"Contact.Email"` -> `"ContactEmail"`. Legacy aliases target it.
pub fn symbol_from_canonical(canonical: &str) -> String {
    canonical.chars().filter(|ch| *ch != '.').collect()
}

impl ScalarDef {
    /// Whether a column of this scalar may carry an ORDER BY. Sortable exactly
    /// when the declared JSON shape is a JSON scalar: string, integer, number
    /// or boolean. Object- and array-shaped values have no total order, and
    /// neither do the `String`-primitive scalars whose PAYLOAD is JSON or a
    /// vector -- `Embedding.Vector`, `Generic.JSON` and `Generic.StringMap` are
    /// all declared `PrimitiveKind::String`, so a primitive-only predicate would
    /// call them sortable and contradict the data-SDK contract.
    ///
    /// Reads `json_schema_type` and nothing else, ALLOWLIST not denylist. A
    /// denylist fails open: a scalar whose shape was mistyped ships sortable and
    /// the SDK generates a `sort` method that cannot be withdrawn without a
    /// breaking change. This form answers `false` for anything undeclared.
    /// `primitive` is not read: every `PrimitiveKind::Object` scalar also
    /// declares `json_schema_type: "object"`, so a primitive clause would be
    /// unreachable; that implication is asserted directly in
    /// `crates/core/tests/semantic_metadata.rs` instead.
    ///
    /// A secret-reference scalar that declares `json_schema_type: ""`
    /// answers `false`. Such a scalar is `metadata_omit`, so no binding
    /// consumes that answer; it is the conservative one for a schema-omitted
    /// secret reference. The registry predicate is not the binding contract -- a
    /// scalar with no metadata row has no sortability answer in any binding.
    pub fn is_sortable(&self) -> bool {
        json_shape_is_sortable(self.json_schema_type)
    }

    /// The generated-binding symbol for this scalar (`symbol_from_canonical`).
    pub fn symbol(&self) -> String {
        symbol_from_canonical(self.canonical)
    }
}

/// The parse/normalize/validate contract every scalar implements. The input is
/// the raw primitive (a string; object scalars take canonical JSON). Output is
/// the canonical normalized form.
///
/// Every hook receives the registry the scalar was assembled into, so a scalar
/// whose rule depends on the catalog (one that accepts exactly the
/// canonical names the registry knows) reads the assembled set, not a
/// compile-time one. Implementations with no such dependency ignore it.
///
/// `Send + Sync` so the registry can hold `Arc<dyn Scalar>` and hand out
/// `&'static dyn Scalar` from a process-wide assembly.
pub trait Scalar: Send + Sync {
    fn id(&self) -> ScalarId;
    /// Validate shape, then return the canonical normalized form.
    fn parse(&self, registry: &Registry, input: &str) -> Result<String, ScalarError>;
    /// Transform toward canonical form without enforcing shape. A validate-only
    /// scalar leaves this as identity; a normalize-only scalar does the work
    /// here and `parse` adds the shape check.
    fn normalize(&self, registry: &Registry, input: &str) -> Result<String, ScalarError>;
    /// Enforce shape without returning a value.
    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError>;
    /// True only for the generic directive engine. Hand-written impls inherit
    /// the `false` default; the assembly exhaustiveness check uses this to
    /// catch a tagged-custom scalar that silently fell through to
    /// `DirectiveScalar`.
    fn is_directive(&self) -> bool {
        false
    }
}

/// One assembled scalar: its def, the extension that declared it, and the
/// implementation that serves it (hand-written or `DirectiveScalar`).
struct Slot {
    def: &'static ScalarDef,
    owner: &'static str,
    scalar: Arc<dyn Scalar>,
}

/// The assembled catalog: built-ins plus zero or more extensions, checked for
/// collisions once and then read-only. The only thing bindings and codegen
/// consume; `Registry::builtin()` is the process-wide built-in assembly.
pub struct Registry {
    slots: BTreeMap<u32, Slot>,
    by_canonical: HashMap<&'static str, ScalarId>,
    legacy_aliases: Vec<LegacyAlias>,
    extensions: Vec<ExtensionInfo>,
}

static BUILTIN: LazyLock<Registry> = LazyLock::new(|| Registry::assemble(&[]));

/// A pending extension during assembly: everything the checks need, gathered
/// once so the trait methods are called exactly once each.
struct Pending {
    name: &'static str,
    id_base: u32,
    defs: &'static [ScalarDef],
    impls: Vec<(ScalarId, Box<dyn Scalar>)>,
    aliases: &'static [LegacyAlias],
}

impl Registry {
    /// The built-in scalars, no extensions, assembled on first use.
    pub fn builtin() -> &'static Registry {
        &BUILTIN
    }

    /// Built-ins plus `extensions`. Panics on any `AssemblyError`, naming both
    /// owners: an assembly that fails is a programming error in the extension
    /// set, never a runtime condition.
    pub fn assemble(extensions: &[&dyn Extension]) -> Registry {
        Self::assemble_with(extensions, AssembleOptions::default())
    }

    /// `assemble` with options (see `AssembleOptions::allow_legacy_ids`).
    pub fn assemble_with(extensions: &[&dyn Extension], options: AssembleOptions) -> Registry {
        match Self::try_assemble(extensions, options) {
            Ok(registry) => registry,
            Err(err) => panic!("scalar registry assembly failed: {err}"),
        }
    }

    /// Non-panicking `assemble_with`, for tests and tooling. Runs the checks in
    /// the documented order and stops at the first failure.
    pub fn try_assemble(
        extensions: &[&dyn Extension],
        options: AssembleOptions,
    ) -> Result<Registry, AssemblyError> {
        let mut pending: Vec<Pending> = Vec::with_capacity(extensions.len() + 1);
        pending.push(Pending {
            name: crate::builtin::NAME,
            id_base: 0,
            defs: &CATALOG,
            impls: crate::builtin::impls(),
            aliases: &[],
        });
        for ext in extensions {
            pending.push(Pending {
                name: ext.name(),
                id_base: ext.id_base(),
                defs: ext.defs(),
                impls: ext.impls(),
                aliases: ext.aliases(),
            });
        }
        // The built-in block is index 0; the id-block checks are about
        // extensions only and skip it. The phases run in the documented order
        // (extension-model.md section 2.6) and each stops at its first failure.
        check_id_blocks(&pending[1..], options)?;
        let by_canonical = check_unique_identities(&pending)?;
        let defs_by_id: BTreeMap<u32, &'static ScalarDef> = pending
            .iter()
            .flat_map(|ext| ext.defs.iter())
            .map(|def| (def.id.0, def))
            .collect();
        check_def_consistency(&defs_by_id)?;
        check_impls(&pending, &defs_by_id)?;
        check_patterns_and_legacy_aliases(&pending, &defs_by_id)?;

        let mut legacy_aliases: Vec<LegacyAlias> = Vec::new();
        let mut infos: Vec<ExtensionInfo> = Vec::with_capacity(pending.len());
        for ext in &pending {
            let legacy_ids = ext.id_base < ScalarId::EXTENSION_BLOCK
                || ext.defs.iter().any(|def| !in_block(def.id, ext.id_base));
            infos.push(ExtensionInfo {
                name: ext.name,
                id_base: ext.id_base,
                legacy_ids: legacy_ids && ext.name != crate::builtin::NAME,
            });
            legacy_aliases.extend(ext.aliases.iter().copied());
        }
        Ok(Registry {
            slots: build_slots(pending),
            by_canonical,
            legacy_aliases,
            extensions: infos,
        })
    }

    /// The def for `id`, or `None` for an id no extension declared.
    pub fn def(&self, id: ScalarId) -> Option<&'static ScalarDef> {
        self.slots.get(&id.0).map(|slot| slot.def)
    }

    /// Look up a def by its canonical identity string, e.g. `"Contact.Email"`.
    /// Exact and case-sensitive.
    pub fn by_canonical(&self, name: &str) -> Option<&'static ScalarDef> {
        self.by_canonical.get(name).and_then(|id| self.def(*id))
    }

    /// The implementation serving `id` (hand-written or directive engine).
    pub fn scalar(&self, id: ScalarId) -> Option<&dyn Scalar> {
        self.slots.get(&id.0).map(|slot| &*slot.scalar)
    }

    /// The name of the extension that declared `id` (`"builtin"` for built-ins).
    pub fn owner(&self, id: ScalarId) -> Option<&'static str> {
        self.slots.get(&id.0).map(|slot| slot.owner)
    }

    /// Resolve an alias to the id that carries the implementation. An unknown
    /// id resolves to itself.
    pub fn resolved(&self, id: ScalarId) -> ScalarId {
        self.def(id).and_then(|def| def.alias_of).unwrap_or(id)
    }

    /// Whether a comparison or join between a column of scalar `a` and one of
    /// `b` is semantically meaningful. THIS is the comparability contract;
    /// `ScalarDef::comparability_class` is the data it reads, not the relation
    /// itself.
    ///
    /// The distinction matters because the obvious implementation is wrong.
    /// `a.comparability_class == b.comparability_class` answers `true` for two
    /// class-less scalars, and `None` is the value almost every scalar carries,
    /// so naive field equality makes almost the entire catalog mutually comparable --
    /// including `Contact.Email` against `Contact.PhoneNumber`, which
    /// the semantic-types design names as the case the relation exists to
    /// reject. Go has the sharper version of the same trap: it encodes absent
    /// as `""`, which is also the zero value a failed map lookup returns.
    ///
    /// The rule: a scalar is always comparable with itself, and two DISTINCT
    /// scalars are comparable only when both declare the SAME named class.
    /// `None` on either side is self-comparable-only and never matches across.
    /// An alias resolves first, since `alias_of` means one implementation under
    /// two ids (`Identity.UserID` and `Identity.UUID` are the only such pair),
    /// so they are the same scalar for this purpose. The class is read through
    /// the RESOLVED def so the alias inherits the target's class; reading the
    /// raw field on each side would break transitivity one hop out.
    ///
    /// Equivalence classes, not a subtype lattice. Reflexive, symmetric and
    /// transitive, asserted over the whole catalog by
    /// `comparability_is_an_equivalence_relation_over_the_catalog`.
    ///
    /// This answers ONE binary question: may these two be compared. It is not
    /// the validator's severity rule, which is three-valued --
    /// a query validator distinguishes different-class (error),
    /// typed-vs-unknown (warn) and unknown-vs-unknown (no diagnostic), and this
    /// predicate collapses the last two into `false`. The validator must read
    /// the classes itself to tell those apart; what it must NOT do is treat raw
    /// field equality as the comparability answer, which is the trap this
    /// exists for.
    ///
    /// The rule itself is [`comparable_in`], which resolves the alias before it
    /// reads either class. This method only supplies the registry as that
    /// function's lookup. The indirection is load-bearing rather than stylistic:
    /// reading the class off the raw row instead of off the alias TARGET breaks
    /// transitivity one hop out (`UserID ~ UUID` true by the identity
    /// short-circuit, `UUID ~ Other` true by the shared class, `UserID ~ Other`
    /// false), and no walk over the built-in catalog can see that, because the
    /// one alias pair carries no class on either side. Putting the resolution
    /// inside the rule is what lets a synthetic table catch it.
    pub fn comparable_with(&self, a: ScalarId, b: ScalarId) -> bool {
        comparable_in(a, b, |id| {
            self.def(id)
                .map_or((None, None), |def| (def.alias_of, def.comparability_class))
        })
    }

    /// Every assembled id, ascending.
    pub fn ids(&self) -> impl Iterator<Item = ScalarId> + '_ {
        self.slots.keys().map(|id| ScalarId(*id))
    }

    /// Every assembled def, in ascending id order.
    pub fn defs(&self) -> impl Iterator<Item = &'static ScalarDef> + '_ {
        self.slots.values().map(|slot| slot.def)
    }

    /// Number of assembled scalars.
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    /// Whether the registry holds no scalars. Never true for an assembly that
    /// includes the built-ins; present for `clippy::len_without_is_empty`.
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    /// Legacy flat names the generated bindings alias, concatenated in
    /// extension order (built-ins first).
    pub fn legacy_aliases(&self) -> &[LegacyAlias] {
        &self.legacy_aliases
    }

    /// The extensions in this assembly, built-ins first.
    pub fn extensions(&self) -> &[ExtensionInfo] {
        &self.extensions
    }

    /// The whole assembled catalog as JSON with a fixed field order, for docs
    /// generation and downstream tooling. Scalars are in `ids()` order.
    pub fn dump(&self) -> serde_json::Value {
        #[derive(Serialize)]
        struct Dump<'a> {
            dump_version: u32,
            superscalar_version: &'static str,
            extensions: &'a [ExtensionInfo],
            scalars: Vec<DumpScalar<'a>>,
            legacy_aliases: &'a [LegacyAlias],
        }
        #[derive(Serialize)]
        struct DumpScalar<'a> {
            id: u32,
            canonical: &'a str,
            namespace: &'a str,
            extension: &'a str,
            primitive: PrimitiveKind,
            sql_type: &'a str,
            metadata_primitive: &'a str,
            json_schema_type: &'a str,
            tag: ScalarTag,
            pattern: Option<&'a str>,
            min_length: Option<usize>,
            max_length: Option<usize>,
            minimum: Option<f64>,
            maximum: Option<f64>,
            case_insensitive: bool,
            reserved_words: &'a [&'a str],
            reserved_words_case_insensitive: bool,
            reserved_words_match_partial: bool,
            examples: &'a [&'a str],
            description: &'a str,
            docstring: &'a str,
            type_mappings: &'a [(&'a str, &'a str)],
            file_upload: Option<&'a FileUploadConfig>,
            image_constraints: Option<&'a ImageConstraints>,
            alias_of: Option<u32>,
            schema_primitive_override: Option<&'a str>,
            schema_omit: bool,
            metadata_omit: bool,
            format: Option<&'a str>,
            comparability_class: Option<&'a str>,
            is_sortable: bool,
            is_directive: bool,
            hooks: ScalarHooks,
        }
        let scalars = self
            .slots
            .values()
            .map(|slot| {
                let def = slot.def;
                DumpScalar {
                    id: def.id.0,
                    canonical: def.canonical,
                    namespace: def.namespace,
                    extension: slot.owner,
                    primitive: def.primitive,
                    sql_type: def.sql_type,
                    metadata_primitive: def.metadata_primitive,
                    json_schema_type: def.json_schema_type,
                    tag: def.tag,
                    pattern: def.pattern,
                    min_length: def.min_length,
                    max_length: def.max_length,
                    minimum: def.minimum,
                    maximum: def.maximum,
                    case_insensitive: def.case_insensitive,
                    reserved_words: def.reserved_words,
                    reserved_words_case_insensitive: def.reserved_words_case_insensitive,
                    reserved_words_match_partial: def.reserved_words_match_partial,
                    examples: def.examples,
                    description: def.description,
                    docstring: def.docstring,
                    type_mappings: def.type_mappings,
                    file_upload: def.file_upload,
                    image_constraints: def.image_constraints,
                    alias_of: def.alias_of.map(|id| id.0),
                    schema_primitive_override: def.schema_primitive_override,
                    schema_omit: def.schema_omit,
                    metadata_omit: def.metadata_omit,
                    format: def.format,
                    comparability_class: def.comparability_class,
                    is_sortable: def.is_sortable(),
                    is_directive: slot.scalar.is_directive(),
                    hooks: def.hooks,
                }
            })
            .collect();
        serde_json::to_value(Dump {
            dump_version: 1,
            superscalar_version: env!("CARGO_PKG_VERSION"),
            extensions: &self.extensions,
            scalars,
            legacy_aliases: &self.legacy_aliases,
        })
        .expect("registry dump serializes")
    }
}

/// Assembly checks 1 and 2: every extension's `id_base` is block-aligned
/// (`IdBaseNotAligned`); then, unless `allow_legacy_ids`, no extension sits in
/// the reserved built-in block (`IdBaseReserved`) and every def id lies inside
/// its extension's block (`IdOutOfBlock`). `exts` excludes the built-in block.
fn check_id_blocks(exts: &[Pending], options: AssembleOptions) -> Result<(), AssemblyError> {
    for ext in exts {
        if ext.id_base % ScalarId::EXTENSION_BLOCK != 0 {
            return Err(AssemblyError::IdBaseNotAligned {
                extension: ext.name,
                id_base: ext.id_base,
            });
        }
    }
    if options.allow_legacy_ids {
        return Ok(());
    }
    for ext in exts {
        if ext.id_base < ScalarId::EXTENSION_BLOCK {
            return Err(AssemblyError::IdBaseReserved {
                extension: ext.name,
            });
        }
        for def in ext.defs {
            if !in_block(def.id, ext.id_base) {
                return Err(AssemblyError::IdOutOfBlock {
                    extension: ext.name,
                    id: def.id,
                    canonical: def.canonical,
                });
            }
        }
    }
    Ok(())
}

/// Assembly checks 3, 4 and 5: extension names are unique and none is
/// `"builtin"` (`DuplicateExtensionName`); no two defs share an id
/// (`DuplicateId`); no two defs share a canonical name, compared exact and
/// case-sensitive (`DuplicateCanonical`). Returns the canonical-to-id index the
/// registry keeps, which is well defined only once check 5 has passed.
fn check_unique_identities(
    pending: &[Pending],
) -> Result<HashMap<&'static str, ScalarId>, AssemblyError> {
    let exts = &pending[1..];
    for (i, ext) in exts.iter().enumerate() {
        let clashes = ext.name == crate::builtin::NAME
            || exts[..i].iter().any(|other| other.name == ext.name);
        if clashes {
            return Err(AssemblyError::DuplicateExtensionName { name: ext.name });
        }
    }
    let mut owner_by_id: BTreeMap<u32, &'static str> = BTreeMap::new();
    for ext in pending {
        for def in ext.defs {
            if let Some(first) = owner_by_id.insert(def.id.0, ext.name) {
                return Err(AssemblyError::DuplicateId {
                    id: def.id,
                    first,
                    second: ext.name,
                });
            }
        }
    }
    let mut by_canonical: HashMap<&'static str, ScalarId> = HashMap::new();
    let mut owner_by_canonical: HashMap<&'static str, &'static str> = HashMap::new();
    for ext in pending {
        for def in ext.defs {
            if let Some(first) = owner_by_canonical.insert(def.canonical, ext.name) {
                return Err(AssemblyError::DuplicateCanonical {
                    canonical: def.canonical,
                    first,
                    second: ext.name,
                });
            }
            by_canonical.insert(def.canonical, def.id);
        }
    }
    Ok(by_canonical)
}

/// Assembly checks 6, 7 and 8, over every def in id order: `namespace` is the
/// canonical prefix before the first `.` (`NamespaceMismatch`); every
/// `alias_of` target exists (`DanglingAlias`) and is not itself an alias
/// (`AliasChain`).
fn check_def_consistency(
    defs_by_id: &BTreeMap<u32, &'static ScalarDef>,
) -> Result<(), AssemblyError> {
    for def in defs_by_id.values() {
        let prefix = def.canonical.split('.').next().unwrap_or_default();
        if def.namespace != prefix {
            return Err(AssemblyError::NamespaceMismatch {
                canonical: def.canonical,
                namespace: def.namespace,
            });
        }
    }
    for def in defs_by_id.values() {
        let Some(target) = def.alias_of else {
            continue;
        };
        let Some(target_def) = defs_by_id.get(&target.0) else {
            return Err(AssemblyError::DanglingAlias {
                canonical: def.canonical,
                alias_of: target,
            });
        };
        if target_def.alias_of.is_some() {
            return Err(AssemblyError::AliasChain {
                canonical: def.canonical,
                alias_of: target,
            });
        }
    }
    Ok(())
}

/// Assembly checks 9, 10 and 11: every registered impl names a def its own
/// extension owns (`ForeignImpl`) and reports that id (`ImplIdMismatch`); then
/// exhaustiveness after alias resolution: a def whose resolved tag is not
/// `PatternOnly` has an impl (`MissingImpl`), a resolved `PatternOnly` def has
/// none, and an alias never carries its own impl (both `UnexpectedImpl`).
fn check_impls(
    pending: &[Pending],
    defs_by_id: &BTreeMap<u32, &'static ScalarDef>,
) -> Result<(), AssemblyError> {
    for ext in pending {
        for (id, scalar) in &ext.impls {
            if !ext.defs.iter().any(|def| def.id == *id) {
                return Err(AssemblyError::ForeignImpl {
                    extension: ext.name,
                    id: *id,
                });
            }
            if scalar.id() != *id {
                return Err(AssemblyError::ImplIdMismatch {
                    registered: *id,
                    reported: scalar.id(),
                });
            }
        }
    }
    let has_impl = |id: ScalarId| {
        pending
            .iter()
            .any(|ext| ext.impls.iter().any(|(impl_id, _)| *impl_id == id))
    };
    for def in defs_by_id.values() {
        let resolved = def
            .alias_of
            .and_then(|target| defs_by_id.get(&target.0).copied())
            .unwrap_or(def);
        if def.alias_of.is_some() && has_impl(def.id) {
            return Err(AssemblyError::UnexpectedImpl {
                canonical: def.canonical,
            });
        }
        let expects_impl = resolved.tag != ScalarTag::PatternOnly;
        if expects_impl && !has_impl(resolved.id) {
            return Err(AssemblyError::MissingImpl {
                canonical: def.canonical,
                tag: resolved.tag,
            });
        }
        if !expects_impl && has_impl(def.id) {
            return Err(AssemblyError::UnexpectedImpl {
                canonical: def.canonical,
            });
        }
    }
    Ok(())
}

/// Assembly checks 12 and 13: every declared `pattern` compiles
/// (`InvalidPattern`); every `LegacyAlias.target` is the generated symbol of
/// some def (`DanglingLegacyAlias`).
fn check_patterns_and_legacy_aliases(
    pending: &[Pending],
    defs_by_id: &BTreeMap<u32, &'static ScalarDef>,
) -> Result<(), AssemblyError> {
    for def in defs_by_id.values() {
        if let Some(pattern) = def.pattern {
            if let Err(err) = regex::Regex::new(pattern) {
                return Err(AssemblyError::InvalidPattern {
                    canonical: def.canonical,
                    message: err.to_string(),
                });
            }
        }
    }
    let symbols: Vec<String> = defs_by_id.values().map(|def| def.symbol()).collect();
    for ext in pending {
        for alias in ext.aliases {
            if !symbols.iter().any(|symbol| symbol == alias.target) {
                return Err(AssemblyError::DanglingLegacyAlias {
                    name: alias.name,
                    target: alias.target,
                });
            }
        }
    }
    Ok(())
}

/// The build step after every check has passed. Non-alias slots are filled
/// first so an alias can borrow its target's `Arc` regardless of id order; a
/// def with no registered impl gets a `DirectiveScalar` over its own def.
fn build_slots(pending: Vec<Pending>) -> BTreeMap<u32, Slot> {
    let mut slots: BTreeMap<u32, Slot> = BTreeMap::new();
    for ext in pending {
        let mut impls: BTreeMap<u32, Arc<dyn Scalar>> = ext
            .impls
            .into_iter()
            .map(|(id, boxed)| (id.0, Arc::from(boxed)))
            .collect();
        for def in ext.defs.iter().filter(|def| def.alias_of.is_none()) {
            let scalar: Arc<dyn Scalar> = match impls.remove(&def.id.0) {
                Some(custom) => custom,
                None => Arc::new(DirectiveScalar::from_def(def)),
            };
            slots.insert(
                def.id.0,
                Slot {
                    def,
                    owner: ext.name,
                    scalar,
                },
            );
        }
        for def in ext.defs.iter().filter(|def| def.alias_of.is_some()) {
            slots.insert(
                def.id.0,
                Slot {
                    def,
                    owner: ext.name,
                    // Filled in the alias pass below once every target
                    // slot exists; a placeholder directive engine over the
                    // alias's own def keeps the map total meanwhile.
                    scalar: Arc::new(DirectiveScalar::from_def(def)),
                },
            );
        }
    }
    let alias_ids: Vec<u32> = slots
        .values()
        .filter(|slot| slot.def.alias_of.is_some())
        .map(|slot| slot.def.id.0)
        .collect();
    for alias_id in alias_ids {
        let target = slots[&alias_id]
            .def
            .alias_of
            .expect("filtered to aliases above");
        let target_slot = &slots[&target.0];
        let scalar: Arc<dyn Scalar> = if target_slot.scalar.is_directive() {
            Arc::new(DirectiveScalar::from_def_as(
                ScalarId(alias_id),
                target_slot.def,
            ))
        } else {
            Arc::clone(&target_slot.scalar)
        };
        if let Some(slot) = slots.get_mut(&alias_id) {
            slot.scalar = scalar;
        }
    }
    slots
}

fn in_block(id: ScalarId, id_base: u32) -> bool {
    id.0 >= id_base && id.0 < id_base.saturating_add(ScalarId::EXTENSION_BLOCK)
}

/// Built-in catalog lookup. Panics on an id the built-in registry does not
/// hold; callers with an untrusted id use `Registry::def`.
pub fn scalar_def(id: ScalarId) -> &'static ScalarDef {
    Registry::builtin()
        .def(id)
        .unwrap_or_else(|| panic!("scalar id {} is not a built-in scalar", id.0))
}

/// Dispatch to a built-in scalar's implementation (custom or directive
/// engine). Panics on an id the built-in registry does not hold; callers with
/// an untrusted id use `Registry::scalar`.
pub fn scalar_for(id: ScalarId) -> &'static dyn Scalar {
    Registry::builtin()
        .scalar(id)
        .unwrap_or_else(|| panic!("scalar id {} is not a built-in scalar", id.0))
}

#[cfg(test)]
mod sortability_rule_tests {
    use super::json_shape_is_sortable;

    /// Table test over shapes the catalog does not contain, which is the point:
    /// the catalog-walking tests cannot reach a mistyped or absent declaration,
    /// and those are the inputs the fail-closed direction exists for (H3, F1).
    #[test]
    fn allowlist_admits_json_scalars_and_rejects_everything_else() {
        for shape in ["string", "integer", "number", "boolean"] {
            assert!(json_shape_is_sortable(shape), "{shape} must be sortable");
        }
        for shape in [
            "object", "array", "any", "", "json", "Object", "objects", "String", "null",
        ] {
            assert!(
                !json_shape_is_sortable(shape),
                "{shape} must not be sortable"
            );
        }
    }
}

#[cfg(test)]
mod primitive_kind_tests {
    use super::PrimitiveKind;

    /// Round trip over EVERY member, driven by `PrimitiveKind::ALL` rather than
    /// by the catalog: a member no scalar currently uses -- `Bool` is one -- is
    /// still a spelling an emitted schema can carry, so a list bounded by the
    /// catalog would leave it unchecked.
    #[test]
    fn from_dsl_name_inverts_dsl_name_over_every_primitive() {
        for primitive in PrimitiveKind::ALL {
            assert_eq!(
                PrimitiveKind::from_dsl_name(primitive.dsl_name()),
                Some(*primitive),
                "{primitive:?} did not round trip"
            );
        }
        for unknown in ["Object", "Boolean", "int", "", "Typeq"] {
            assert_eq!(PrimitiveKind::from_dsl_name(unknown), None, "{unknown}");
        }
    }

    /// The other half of the pair `primitive_kinds!` declares together. `JSON`
    /// is the IR spelling, `Type` is the DSL one, and reading the wrong one is
    /// how a derived column arrives at its consumer as an unknown type
    /// reference.
    #[test]
    fn from_type_ref_name_inverts_type_ref_name_over_every_primitive() {
        for primitive in PrimitiveKind::ALL {
            assert_eq!(
                PrimitiveKind::from_type_ref_name(primitive.type_ref_name()),
                Some(*primitive),
                "{primitive:?} did not round trip"
            );
        }
        // A scalar-typed column is `scalars/{canonical}` and is resolved through
        // the catalog, not here; the DSL spellings that differ from the IR ones
        // are not type-ref names either. Nor is the dotted `Generic.JSON`: it
        // names a catalog scalar, and a bare primitive must not read as one.
        for unknown in [
            "scalars/Contact.Email",
            "Bool",
            "Type",
            "boolean",
            "Generic.JSON",
            "",
        ] {
            assert_eq!(
                PrimitiveKind::from_type_ref_name(unknown),
                None,
                "{unknown}"
            );
        }
    }

    /// Both spellings are declared, and neither is empty or duplicated. A member
    /// added to `primitive_kinds!` without real names would compile -- the macro
    /// only requires two literals -- and this is what refuses it.
    #[test]
    fn every_primitive_declares_two_usable_spellings() {
        let mut type_refs = std::collections::BTreeSet::new();
        let mut dsl_names = std::collections::BTreeSet::new();
        for primitive in PrimitiveKind::ALL {
            assert!(
                !primitive.type_ref_name().is_empty(),
                "{primitive:?} has no IR type-ref name"
            );
            assert!(
                !primitive.dsl_name().is_empty(),
                "{primitive:?} has no DSL name"
            );
            assert!(
                type_refs.insert(primitive.type_ref_name()),
                "{primitive:?} repeats an IR type-ref name"
            );
            assert!(
                dsl_names.insert(primitive.dsl_name()),
                "{primitive:?} repeats a DSL name"
            );
        }
        assert_eq!(type_refs.len(), PrimitiveKind::ALL.len());
        assert_eq!(dsl_names.len(), PrimitiveKind::ALL.len());
    }
}

/// The comparability relation against tables the catalog cannot produce.
///
/// Every test here is a table test on purpose. The catalog-walking versions in
/// `crates/core/tests/semantic_metadata.rs` stay as the standing checks over
/// shipped data, but neither of the two arms below is DISCHARGEABLE by them: the
/// one alias pair carries no class, and one two-member class admits no
/// pairwise-distinct triple. These tables supply both, so neither guard depends
/// on any particular scalar being classed and both keep working when the class
/// table changes.
#[cfg(test)]
mod comparability_rule_tests {
    use super::comparable_in;

    /// `(name, alias_of, class)`. The shape [`comparable_in`] reads, as raw rows.
    type Row = (&'static str, Option<&'static str>, Option<&'static str>);

    fn lookup(
        rows: &[Row],
    ) -> impl Fn(&'static str) -> (Option<&'static str>, Option<&'static str>) + '_ {
        move |name| {
            rows.iter()
                .find(|(row, _, _)| *row == name)
                .map(|(_, alias_of, class)| (*alias_of, *class))
                .unwrap_or((None, None))
        }
    }

    /// The alias arm, which the catalog cannot exercise.
    ///
    /// `Alias` declares NO class of its own and points at `Target`, which is
    /// classed alongside `Other`. The relation must read `Alias`'s class through
    /// `Target`. Reading the raw field instead answers `None` for `Alias`, so
    /// `Alias ~ Other` comes back false while `Alias ~ Target` stays true on the
    /// identity short-circuit -- the inconsistency is invisible at the row where
    /// it was introduced and only surfaces one hop out, which is exactly why the
    /// shipped catalog cannot catch it.
    #[test]
    fn an_alias_inherits_its_targets_class_one_hop_out() {
        let rows: &[Row] = &[
            ("Identity.UserID", Some("Identity.UUID"), None),
            ("Identity.UUID", None, Some("opaque_id")),
            ("Other.Id", None, Some("opaque_id")),
        ];
        let comparable = |a, b| comparable_in(a, b, lookup(rows));

        assert!(
            comparable("Identity.UserID", "Other.Id"),
            "Identity.UserID is an alias of Identity.UUID, which shares the \
             opaque_id class with Other.Id; reading the class off the raw alias \
             row instead of off the alias target breaks this pair and leaves \
             Identity.UserID ~ Identity.UUID true, so the break only shows one \
             hop out"
        );
        assert!(comparable("Identity.UserID", "Identity.UUID"), "alias pair");
        assert!(
            comparable("Other.Id", "Identity.UserID"),
            "and symmetrically"
        );
    }

    /// The transitivity arm, which the catalog cannot exercise either: it needs a
    /// pairwise-distinct triple, and one two-member class has none.
    #[test]
    fn transitivity_holds_over_a_three_member_class() {
        let rows: &[Row] = &[
            ("A.One", None, Some("shared")),
            ("A.Two", None, Some("shared")),
            ("A.Three", None, Some("shared")),
            ("B.One", None, Some("other")),
            ("B.Two", None, Some("other")),
            ("C.Unclassed", None, None),
        ];
        let names = ["A.One", "A.Two", "A.Three", "B.One", "B.Two", "C.Unclassed"];
        let comparable = |a, b| comparable_in(a, b, lookup(rows));

        let mut distinct_triples = 0;
        for &a in &names {
            assert!(comparable(a, a), "{a} reflexive");
            for &b in &names {
                assert_eq!(comparable(a, b), comparable(b, a), "{a} / {b} symmetric");
                if !comparable(a, b) {
                    continue;
                }
                for &c in &names {
                    if !comparable(b, c) {
                        continue;
                    }
                    if a != b && b != c && a != c {
                        distinct_triples += 1;
                    }
                    assert!(
                        comparable(a, c),
                        "transitivity: {a} ~ {b} and {b} ~ {c} but not {a} ~ {c}"
                    );
                }
            }
        }
        assert!(
            distinct_triples > 0,
            "the transitivity loop must reach a pairwise-distinct triple, which \
             is precisely what the shipped catalog cannot supply"
        );
    }

    /// Two classed scalars in DIFFERENT classes never compare, and an unclassed
    /// scalar is self-comparable only. The second half is the trap the doc
    /// comment on `comparable_with` names: raw field equality answers true for
    /// two class-less scalars, and `None` is what almost every catalog row
    /// carries.
    #[test]
    fn distinct_classes_and_absent_classes_never_match_across() {
        let rows: &[Row] = &[
            ("A.One", None, Some("shared")),
            ("B.One", None, Some("other")),
            ("C.Unclassed", None, None),
            ("D.Unclassed", None, None),
        ];
        let comparable = |a, b| comparable_in(a, b, lookup(rows));

        assert!(!comparable("A.One", "B.One"), "different classes");
        assert!(
            !comparable("C.Unclassed", "D.Unclassed"),
            "two class-less scalars are NOT comparable; field equality would say \
             they are, and that is the majority of the catalog"
        );
        assert!(
            !comparable("A.One", "C.Unclassed"),
            "classed against absent"
        );
        assert!(comparable("C.Unclassed", "C.Unclassed"), "self");
    }

    /// Resolution is SINGLE-HOP, and this is the table that says so.
    ///
    /// [`comparable_in`]'s doc comment claims it matches `Registry::resolved`
    /// exactly rather than improving on it, and that claim was undischargeable
    /// until this table: the catalog's one alias pair is a single hop, so
    /// replacing `unwrap_or` with a fixpoint loop passes every other test here.
    ///
    /// The chain is `A -> B -> C` with the class on `C`, and single-hop answers
    /// FALSE for every pair that crosses it -- including `A ~ B`, the adjacent
    /// alias pair. `A` resolves one hop to `B`; `B` resolves one hop to `C`, so
    /// the two sides are not equal and the identity short-circuit does not fire;
    /// `B` carries no class of its own because the alias arm of
    /// `check_class_invariants` forbids it. So a chain does not merely fail to
    /// propagate a class, it breaks the alias relation at its own first link.
    ///
    /// Assembly refuses a chain today (`AssemblyError::AliasChain`), so no
    /// assembled registry reaches this shape. The test pins the rule rather than
    /// the catalog: it is what fails if the assembly check is ever relaxed while
    /// a chain-following `comparable_in` and a single-hop `resolved()` would
    /// silently disagree about the same registry.
    #[test]
    fn resolution_is_single_hop_and_a_chain_breaks_at_its_first_link() {
        let rows: &[Row] = &[
            ("A.Head", Some("B.Middle"), None),
            ("B.Middle", Some("C.Tail"), None),
            ("C.Tail", None, Some("shared")),
            ("D.Other", None, Some("shared")),
        ];
        let comparable = |a, b| comparable_in(a, b, lookup(rows));

        assert!(
            !comparable("A.Head", "B.Middle"),
            "SINGLE-HOP: A.Head resolves to B.Middle and B.Middle resolves to \
             C.Tail, so the identity short-circuit does not fire and B.Middle \
             carries no class. A chain breaks even its own adjacent pair, which \
             is why the catalog must not grow one without deciding first"
        );
        assert!(
            !comparable("A.Head", "C.Tail"),
            "and it does not reach the class at the end of the chain; a fixpoint \
             resolver would answer true here and diverge from Registry::resolved"
        );
        assert!(!comparable("A.Head", "D.Other"));
        assert!(
            comparable("C.Tail", "D.Other"),
            "the classed pair past the chain is unaffected"
        );

        // The contrast: the same shape WITHOUT a chain resolves cleanly, so the
        // assertions above are about the chain and not about aliases generally.
        let flat: &[Row] = &[
            ("A.Head", Some("C.Tail"), None),
            ("C.Tail", None, Some("shared")),
            ("D.Other", None, Some("shared")),
        ];
        assert!(comparable_in("A.Head", "C.Tail", lookup(flat)));
        assert!(comparable_in("A.Head", "D.Other", lookup(flat)));
    }

    /// An alias whose target is UNCLASSED stays self-comparable-only. Guards the
    /// over-correction: resolving the alias must not invent a class.
    #[test]
    fn an_alias_of_an_unclassed_target_matches_nothing_else() {
        let rows: &[Row] = &[
            ("Identity.UserID", Some("Identity.UUID"), None),
            ("Identity.UUID", None, None),
            ("Other.Id", None, Some("opaque_id")),
        ];
        let comparable = |a, b| comparable_in(a, b, lookup(rows));

        assert!(comparable("Identity.UserID", "Identity.UUID"), "alias pair");
        assert!(!comparable("Identity.UserID", "Other.Id"));
    }
}
