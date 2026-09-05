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
//! consume; each hands it to its export macro.

use std::sync::LazyLock;
use superscalar::{
    ErrorKind, Extension, PrimitiveKind, Registry, Scalar, ScalarDef, ScalarError, ScalarHooks,
    ScalarId, ScalarTag,
};

/// First id of the Acme block. An extension's ids live in
/// `[id_base, id_base + 4096)`; the built-ins own `0..=4095`.
pub const ID_BASE: u32 = ScalarId::EXTENSION_BLOCK;

/// `Acme.OrderNumber`, frozen at 4096.
pub const ORDER_NUMBER: ScalarId = ScalarId(ID_BASE);
/// `Acme.ScalarRef`, frozen at 4097.
pub const SCALAR_REF: ScalarId = ScalarId(ID_BASE + 1);

/// The Acme catalog. Ids are append-only: a new scalar takes the next free id
/// in the block and nothing is ever renumbered.
pub static DEFS: [ScalarDef; 2] = [
    ScalarDef {
        id: ORDER_NUMBER,
        namespace: "Acme",
        canonical: "Acme.OrderNumber",
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
        id: SCALAR_REF,
        namespace: "Acme",
        canonical: "Acme.ScalarRef",
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

/// The extension: name, block, defs, and the one hand-written impl.
pub struct AcmeExtension;

impl Extension for AcmeExtension {
    fn name(&self) -> &'static str {
        "acme"
    }

    fn id_base(&self) -> u32 {
        ID_BASE
    }

    fn defs(&self) -> &'static [ScalarDef] {
        &DEFS
    }

    fn impls(&self) -> Vec<(ScalarId, Box<dyn Scalar>)> {
        vec![(SCALAR_REF, Box::new(ScalarRef))]
    }
}

/// `Acme.ScalarRef`: accepts exactly the canonical names the registry it was
/// assembled into knows. Case-sensitive, no normalization.
pub struct ScalarRef;

impl Scalar for ScalarRef {
    fn id(&self) -> ScalarId {
        SCALAR_REF
    }

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
        if registry.by_canonical(input).is_some() {
            return Ok(());
        }
        Err(ScalarError::new(
            ErrorKind::Enum,
            format!("unknown scalar: {input}"),
        ))
    }
}

static REGISTRY: LazyLock<Registry> = LazyLock::new(|| Registry::assemble(&[&AcmeExtension]));

/// The assembled registry: every built-in scalar plus the Acme block. Built
/// once, on first use; assembly panics on an id or name collision, which is a
/// programming error in the extension set and never a runtime condition.
pub fn registry() -> &'static Registry {
    &REGISTRY
}
