//! Two scalars in the `Acme` namespace, added to the built-in registry from
//! outside the core crate.
//!
//! `Acme.OrderNumber` is directive-only: a pattern and a length, no Rust code.
//! The core's directive engine serves it from the def alone. `Acme.ScalarRef`
//! is a hand-written impl whose accept set is the canonical names of the
//! registry it was assembled into, so it accepts `Acme.OrderNumber` through
//! this assembly and rejects it through the built-in registry. That difference
//! is what the conformance vectors pin: a binding that passes them dispatches
//! into the assembled registry, not the built-in one.
//!
//! `registry()` is the one thing the four binding crates in this workspace
//! consume; each hands it to its export macro. `definitions()` is the same
//! catalog without implementations, for a consumer that only reads defs.

use std::sync::LazyLock;
use superscalar::{
    Definitions, ErrorKind, Extension, PrimitiveKind, Registry, Scalar, ScalarDef, ScalarError,
    ScalarHooks, ScalarTag,
};

/// `Acme.OrderNumber`. A scalar's canonical name is its identity; names are
/// append-only and never renamed.
pub const ORDER_NUMBER: &str = "Acme.OrderNumber";
/// `Acme.ScalarRef`.
pub const SCALAR_REF: &str = "Acme.ScalarRef";

/// The Acme catalog. A new scalar adds its name above and its def here; order
/// carries no meaning.
pub static DEFS: [ScalarDef; 2] = [
    ScalarDef {
        namespace: "Acme",
        canonical: ORDER_NUMBER,
        primitive: PrimitiveKind::String,
        sql_type: "TEXT",
        metadata_primitive: "String",
        json_schema_type: "string",
        tag: ScalarTag::PatternOnly,
        pattern: Some("^ORD-[0-9]{6}$"),
        min_length: Some(10),
        max_length: Some(10),
        minimum: None,
        maximum: None,
        case_insensitive: false,
        reserved_words: &[],
        examples: &["ORD-000123"],
        description: "An Acme order number: ORD- followed by six digits",
        type_mappings: &[
            ("typescript", "string"),
            ("python", "str"),
            ("go", "string"),
            ("rust", "String"),
            ("sql", "TEXT"),
            ("json_schema", "string"),
        ],
        file_upload: None,
        image_constraints: None,
        docstring: "",
        alias_of: None,
        schema_primitive_override: None,
        schema_omit: false,
        format: None,
        reserved_words_case_insensitive: false,
        reserved_words_match_partial: false,
        comparability_class: None,
        hooks: ScalarHooks::NONE,
        metadata_omit: false,
    },
    ScalarDef {
        namespace: "Acme",
        canonical: SCALAR_REF,
        primitive: PrimitiveKind::String,
        sql_type: "TEXT",
        metadata_primitive: "String",
        json_schema_type: "string",
        tag: ScalarTag::CustomLogic,
        pattern: None,
        min_length: None,
        max_length: None,
        minimum: None,
        maximum: None,
        case_insensitive: false,
        reserved_words: &[],
        examples: &["Contact.Email", "Acme.OrderNumber"],
        description: "The canonical name of a scalar in the assembled registry",
        type_mappings: &[
            ("typescript", "string"),
            ("python", "str"),
            ("go", "string"),
            ("rust", "String"),
            ("sql", "TEXT"),
            ("json_schema", "string"),
        ],
        file_upload: None,
        image_constraints: None,
        docstring: "",
        alias_of: None,
        schema_primitive_override: None,
        schema_omit: false,
        format: None,
        reserved_words_case_insensitive: false,
        reserved_words_match_partial: false,
        comparability_class: None,
        hooks: ScalarHooks {
            parse: false,
            normalize: false,
            validate: true,
        },
        metadata_omit: false,
    },
];

/// The extension: name, defs, and the one hand-written impl.
pub struct AcmeExtension;

impl Extension for AcmeExtension {
    fn name(&self) -> &'static str {
        NAME
    }

    fn defs(&self) -> &'static [ScalarDef] {
        &DEFS
    }

    fn impls(&self) -> Vec<(&'static str, Box<dyn Scalar>)> {
        vec![(SCALAR_REF, Box::new(ScalarRef))]
    }
}

/// `Acme.ScalarRef`: accepts exactly the canonical names the registry it was
/// assembled into knows. Case-sensitive, no normalization.
pub struct ScalarRef;

impl Scalar for ScalarRef {
    fn parse(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.validate(registry, input)?;
        Ok(input.to_string())
    }

    fn normalize(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.parse(registry, input)
    }

    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        if input.trim().is_empty() {
            return Err(ScalarError::new(ErrorKind::Empty, "empty scalar name"));
        }
        if registry.def(input).is_some() {
            return Ok(());
        }
        Err(ScalarError::new(
            ErrorKind::Enum,
            format!("unknown scalar: {input}"),
        ))
    }
}

static REGISTRY: LazyLock<Registry> = LazyLock::new(|| Registry::assemble(&[&AcmeExtension]));

/// The assembled registry: every built-in scalar plus the Acme scalars. Built
/// once, on first use; assembly panics on a name collision, which is a
/// programming error in the extension set and never a runtime condition.
pub fn registry() -> &'static Registry {
    &REGISTRY
}

/// The owner name the extension reports, shared by the registry and the
/// definitions-only assembly below.
pub const NAME: &str = "acme";

static DEFINITIONS: LazyLock<Definitions> =
    LazyLock::new(|| Definitions::assemble(&[(NAME, &DEFS)]));

/// The Acme definitions without their implementations: every built-in def plus
/// the Acme defs, for a consumer that only looks defs up by canonical name,
/// resolves aliases or asks about comparability. It never calls
/// `AcmeExtension::impls`, so a build that uses only this links no scalar
/// implementation, which is what keeps a size-capped WASM bundle small.
/// Answers every definition question exactly as `registry()` does.
pub fn definitions() -> &'static Definitions {
    &DEFINITIONS
}
