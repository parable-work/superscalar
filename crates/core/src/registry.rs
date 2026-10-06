use crate::catalog::CATALOG;
use crate::definitions::Definitions;
use crate::directive::DirectiveScalar;
use crate::error::ScalarError;
use crate::extension::{AssemblyError, Extension, LegacyAlias};
use serde::Serialize;
use std::collections::BTreeMap;
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
    /// Text.
    (String, "String", "String"),
    /// A signed 64-bit integer.
    (Int, "Int", "Int"),
    /// A 64-bit float.
    (Float, "Float", "Float"),
    /// A boolean. Reserved: no built-in scalar is backed by it.
    (Bool, "Boolean", "Bool"),
    /// An object-shaped value. No built-in is backed by it: the built-ins
    /// whose value is a JSON object (`Generic.StringMap`, `Geo.Location`) are
    /// `String`, their canonical JSON text, with `json_schema_type: "object"`.
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
    /// The canonical prefix before the first '.', e.g. "Contact". Redundant with
    /// `canonical` and asserted equal at assembly; present so codegen and docs
    /// can group without string-splitting in three template languages.
    pub namespace: &'static str,
    /// The scalar's identity: `"Contact.Email"`. Unique within an assembly and
    /// append-only: a published name keeps its meaning and is never renamed.
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
    /// If set, the canonical name of the scalar whose implementation this one
    /// shares (e.g. Identity.UserID -> Identity.UUID). One impl, two names.
    pub alias_of: Option<&'static str>,
    /// When the registry's runtime `primitive` deliberately diverges from the
    /// not-yet-ratified schema declaration, this holds the DSL primitive the
    /// generated schema catalogs must still emit so they match the audited
    /// baseline. No built-in sets it. `None` means emit the runtime
    /// primitive.
    pub schema_primitive_override: Option<&'static str>,
    /// When true the scalar exists in the registry but is intentionally omitted
    /// from the generated schema catalogs (e.g. the TS runtime's
    /// `BUILTIN_SCALARS`) until its promotion is ratified.
    /// No built-in sets it; a downstream secret-reference scalar is the
    /// motivating case.
    pub schema_omit: bool,
    /// Named format hint, such as `semver` for `Version.SemVer`. The registry
    /// dump, the generated Rust and TypeScript metadata tables (`format`) and a
    /// downstream schema catalog read it from here; a def without one emits
    /// `None` or `""`.
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
    /// vector -- `Embedding.Vector`, `Generic.JSON`, `Generic.StringMap` and
    /// `Geo.Location` are all declared `PrimitiveKind::String`, so a
    /// primitive-only predicate would call them sortable and contradict the
    /// data-SDK contract.
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
///
/// Every definition lookup (`def`, `resolved`, `comparable_with`, `names`,
/// `defs`, `len`) is answered by the registry's
/// [`Definitions`], which a consumer that needs no implementation can
/// assemble on its own.
pub struct Registry {
    definitions: Definitions,
    slots: BTreeMap<&'static str, Slot>,
    legacy_aliases: Vec<LegacyAlias>,
    extensions: Vec<&'static str>,
}

static BUILTIN: LazyLock<Registry> = LazyLock::new(|| Registry::assemble(&[]));

/// A pending extension during assembly: everything the checks need, gathered
/// once so the trait methods are called exactly once each.
struct Pending {
    name: &'static str,
    defs: &'static [ScalarDef],
    impls: Vec<(&'static str, Box<dyn Scalar>)>,
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
        match Self::try_assemble(extensions) {
            Ok(registry) => registry,
            Err(err) => panic!("scalar registry assembly failed: {err}"),
        }
    }

    /// Non-panicking `assemble`, for tests and tooling. Runs the checks in the
    /// documented order and stops at the first failure.
    pub fn try_assemble(extensions: &[&dyn Extension]) -> Result<Registry, AssemblyError> {
        let mut pending: Vec<Pending> = Vec::with_capacity(extensions.len() + 1);
        pending.push(Pending {
            name: crate::builtin::NAME,
            defs: &CATALOG,
            impls: crate::builtin::impls(),
            aliases: &[],
        });
        for ext in extensions {
            pending.push(Pending {
                name: ext.name(),
                defs: ext.defs(),
                impls: ext.impls(),
                aliases: ext.aliases(),
            });
        }
        // The phases run in the documented order and each stops at its first
        // failure.
        check_extension_names(&pending)?;
        let definitions =
            Definitions::try_assemble_sources(pending.iter().map(|ext| (ext.name, ext.defs)))?;
        let defs_by_name = definitions.by_name();
        check_impls(&pending, defs_by_name)?;
        check_patterns_and_legacy_aliases(&pending, defs_by_name)?;

        let extensions: Vec<&'static str> = pending.iter().map(|ext| ext.name).collect();
        let legacy_aliases: Vec<LegacyAlias> = pending
            .iter()
            .flat_map(|ext| ext.aliases.iter().copied())
            .collect();
        Ok(Registry {
            definitions,
            slots: build_slots(pending),
            legacy_aliases,
            extensions,
        })
    }

    /// The definitions this registry was assembled from: every def lookup
    /// below delegates to it.
    pub fn definitions(&self) -> &Definitions {
        &self.definitions
    }

    /// The def named `canonical` (`"Contact.Email"`), or `None` for a name no
    /// extension declared. Exact and case-sensitive.
    pub fn def(&self, canonical: &str) -> Option<&'static ScalarDef> {
        self.definitions.def(canonical)
    }

    /// The implementation serving `canonical` (hand-written or directive
    /// engine), or `None` for a name no extension declared.
    pub fn scalar(&self, canonical: &str) -> Option<&dyn Scalar> {
        self.slots.get(canonical).map(|slot| &*slot.scalar)
    }

    /// The name of the extension that declared `canonical` (`"builtin"` for
    /// built-ins).
    pub fn owner(&self, canonical: &str) -> Option<&'static str> {
        self.slots.get(canonical).map(|slot| slot.owner)
    }

    /// Resolve an alias to the name that carries the implementation. A name
    /// that is not an alias, or not assembled, resolves to itself.
    pub fn resolved<'a>(&self, canonical: &'a str) -> &'a str {
        self.definitions.resolved(canonical)
    }

    /// Whether a comparison or join between a column of scalar `a` and one of
    /// `b` is semantically meaningful: [`Definitions::comparable_with`] over
    /// this registry's definitions, which documents the rule.
    pub fn comparable_with(&self, a: &str, b: &str) -> bool {
        self.definitions.comparable_with(a, b)
    }

    /// Every assembled canonical name, sorted.
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.definitions.names()
    }

    /// Every assembled def, sorted by canonical name.
    pub fn defs(&self) -> impl Iterator<Item = &'static ScalarDef> + '_ {
        self.definitions.defs()
    }

    /// Number of assembled scalars.
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    /// Whether the registry holds no scalars. Never true for an assembly that
    /// includes the built-ins; present for `clippy::len_without_is_empty`.
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    /// Legacy flat names the generated bindings alias, concatenated in
    /// extension order (built-ins first).
    pub fn legacy_aliases(&self) -> &[LegacyAlias] {
        &self.legacy_aliases
    }

    /// The names of the extensions in this assembly, `"builtin"` first.
    pub fn extensions(&self) -> &[&'static str] {
        &self.extensions
    }

    /// The whole assembled catalog as JSON with a fixed field order, for docs
    /// generation and downstream tooling. Scalars are sorted by canonical name.
    pub fn dump(&self) -> serde_json::Value {
        #[derive(Serialize)]
        struct Dump<'a> {
            dump_version: u32,
            superscalar_version: &'static str,
            extensions: &'a [&'static str],
            scalars: Vec<DumpScalar<'a>>,
            legacy_aliases: &'a [LegacyAlias],
        }
        #[derive(Serialize)]
        struct DumpScalar<'a> {
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
            alias_of: Option<&'a str>,
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
                    alias_of: def.alias_of,
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
            dump_version: 2,
            superscalar_version: env!("CARGO_PKG_VERSION"),
            extensions: &self.extensions,
            scalars,
            legacy_aliases: &self.legacy_aliases,
        })
        .expect("registry dump serializes")
    }
}

/// Assembly check 1: extension names are unique and none is `"builtin"`
/// (`DuplicateExtensionName`). Checks 2 to 5 run in `Definitions` assembly,
/// which needs no extension names beyond the owner it reports.
fn check_extension_names(pending: &[Pending]) -> Result<(), AssemblyError> {
    let exts = &pending[1..];
    for (i, ext) in exts.iter().enumerate() {
        let clashes = ext.name == crate::builtin::NAME
            || exts[..i].iter().any(|other| other.name == ext.name);
        if clashes {
            return Err(AssemblyError::DuplicateExtensionName { name: ext.name });
        }
    }
    Ok(())
}

/// Assembly checks 6 and 7: every registered impl names a def its own
/// extension declares (`ForeignImpl`); then exhaustiveness after alias
/// resolution: a def whose resolved tag is not `PatternOnly` has an impl
/// (`MissingImpl`), a resolved `PatternOnly` def has none, and an alias never
/// carries its own impl (both `UnexpectedImpl`).
fn check_impls(
    pending: &[Pending],
    defs_by_name: &BTreeMap<&'static str, &'static ScalarDef>,
) -> Result<(), AssemblyError> {
    for ext in pending {
        for (canonical, _) in &ext.impls {
            if !ext.defs.iter().any(|def| def.canonical == *canonical) {
                return Err(AssemblyError::ForeignImpl {
                    extension: ext.name,
                    canonical,
                });
            }
        }
    }
    let has_impl = |canonical: &str| {
        pending
            .iter()
            .any(|ext| ext.impls.iter().any(|(name, _)| *name == canonical))
    };
    for def in defs_by_name.values() {
        let resolved = def
            .alias_of
            .and_then(|target| defs_by_name.get(target).copied())
            .unwrap_or(def);
        if def.alias_of.is_some() && has_impl(def.canonical) {
            return Err(AssemblyError::UnexpectedImpl {
                canonical: def.canonical,
            });
        }
        let expects_impl = resolved.tag != ScalarTag::PatternOnly;
        if expects_impl && !has_impl(resolved.canonical) {
            return Err(AssemblyError::MissingImpl {
                canonical: def.canonical,
                tag: resolved.tag,
            });
        }
        if !expects_impl && has_impl(def.canonical) {
            return Err(AssemblyError::UnexpectedImpl {
                canonical: def.canonical,
            });
        }
    }
    Ok(())
}

/// Assembly checks 8 and 9: every declared `pattern` compiles
/// (`InvalidPattern`); every `LegacyAlias.target` is the generated symbol of
/// some def (`DanglingLegacyAlias`).
fn check_patterns_and_legacy_aliases(
    pending: &[Pending],
    defs_by_name: &BTreeMap<&'static str, &'static ScalarDef>,
) -> Result<(), AssemblyError> {
    for def in defs_by_name.values() {
        if let Some(pattern) = def.pattern {
            if let Err(err) = regex::Regex::new(pattern) {
                return Err(AssemblyError::InvalidPattern {
                    canonical: def.canonical,
                    message: err.to_string(),
                });
            }
        }
    }
    let symbols: Vec<String> = defs_by_name.values().map(|def| def.symbol()).collect();
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
/// first, then each alias borrows its target's implementation; a def with no
/// registered impl gets a `DirectiveScalar` over its own def.
fn build_slots(pending: Vec<Pending>) -> BTreeMap<&'static str, Slot> {
    let mut slots: BTreeMap<&'static str, Slot> = BTreeMap::new();
    let mut aliases: Vec<(&'static ScalarDef, &'static str)> = Vec::new();
    for ext in pending {
        let mut impls: BTreeMap<&'static str, Arc<dyn Scalar>> = ext
            .impls
            .into_iter()
            .map(|(canonical, boxed)| (canonical, Arc::from(boxed)))
            .collect();
        for def in ext.defs {
            if def.alias_of.is_some() {
                aliases.push((def, ext.name));
                continue;
            }
            let scalar: Arc<dyn Scalar> = match impls.remove(def.canonical) {
                Some(custom) => custom,
                None => Arc::new(DirectiveScalar::from_def(def)),
            };
            slots.insert(
                def.canonical,
                Slot {
                    def,
                    owner: ext.name,
                    scalar,
                },
            );
        }
    }
    for (def, owner) in aliases {
        let target = def.alias_of.expect("collected as an alias above");
        let scalar = Arc::clone(&slots[target].scalar);
        slots.insert(def.canonical, Slot { def, owner, scalar });
    }
    slots
}

/// Built-in catalog lookup by canonical name. Panics on a name the built-in
/// set does not hold; callers with an untrusted name use `Definitions::def` or
/// `Registry::def`. Reads [`Definitions::builtin`], so it links no scalar
/// implementation.
pub fn scalar_def(canonical: &str) -> &'static ScalarDef {
    Definitions::builtin()
        .def(canonical)
        .unwrap_or_else(|| panic!("{canonical:?} is not a built-in scalar"))
}

/// Dispatch to a built-in scalar's implementation (custom or directive
/// engine) by canonical name. Panics on a name the built-in registry does not
/// hold; callers with an untrusted name use `Registry::scalar`.
pub fn scalar_for(canonical: &str) -> &'static dyn Scalar {
    Registry::builtin()
        .scalar(canonical)
        .unwrap_or_else(|| panic!("{canonical:?} is not a built-in scalar"))
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
