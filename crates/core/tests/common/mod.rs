//! Shared fixtures for the assembly tests: a minimal `ScalarDef` constructor,
//! an `Extension` built from plain data, and two tiny hand-written scalars.

#![allow(dead_code)]

use superscalar::{
    ErrorKind, Extension, LegacyAlias, PrimitiveKind, Registry, Scalar, ScalarDef, ScalarError,
    ScalarHooks, ScalarId, ScalarTag,
};

/// A string def with every optional field at its default.
pub const fn def(
    id: ScalarId,
    namespace: &'static str,
    canonical: &'static str,
    tag: ScalarTag,
    pattern: Option<&'static str>,
) -> ScalarDef {
    ScalarDef {
        id,
        namespace,
        canonical,
        primitive: PrimitiveKind::String,
        sql_type: "TEXT",
        metadata_primitive: "String",
        json_schema_type: "string",
        tag,
        pattern,
        min_length: None,
        max_length: None,
        minimum: None,
        maximum: None,
        case_insensitive: false,
        reserved_words: &[],
        examples: &[],
        description: "",
        type_mappings: &[
            ("typescript", "string"),
            ("python", "str"),
            ("go", "string"),
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
    }
}

/// `def` with `alias_of` set.
pub const fn alias_def(
    id: ScalarId,
    namespace: &'static str,
    canonical: &'static str,
    alias_of: ScalarId,
) -> ScalarDef {
    let mut d = def(id, namespace, canonical, ScalarTag::PatternOnly, None);
    d.alias_of = Some(alias_of);
    d
}

pub type Impls = fn() -> Vec<(ScalarId, Box<dyn Scalar>)>;

pub fn no_impls() -> Vec<(ScalarId, Box<dyn Scalar>)> {
    Vec::new()
}

/// An extension assembled from plain data.
pub struct TestExtension {
    pub name: &'static str,
    pub id_base: u32,
    pub defs: &'static [ScalarDef],
    pub impls: Impls,
    pub aliases: &'static [LegacyAlias],
}

impl TestExtension {
    pub const fn new(name: &'static str, id_base: u32, defs: &'static [ScalarDef]) -> Self {
        TestExtension {
            name,
            id_base,
            defs,
            impls: no_impls,
            aliases: &[],
        }
    }

    pub const fn with_impls(mut self, impls: Impls) -> Self {
        self.impls = impls;
        self
    }

    pub const fn with_aliases(mut self, aliases: &'static [LegacyAlias]) -> Self {
        self.aliases = aliases;
        self
    }
}

impl Extension for TestExtension {
    fn name(&self) -> &'static str {
        self.name
    }
    fn id_base(&self) -> u32 {
        self.id_base
    }
    fn defs(&self) -> &'static [ScalarDef] {
        self.defs
    }
    fn impls(&self) -> Vec<(ScalarId, Box<dyn Scalar>)> {
        (self.impls)()
    }
    fn aliases(&self) -> &'static [LegacyAlias] {
        self.aliases
    }
}

/// A hand-written scalar that upper-cases its input. Exists to be
/// distinguishable from `DirectiveScalar` (`is_directive() == false`).
pub struct Upper(pub ScalarId);

impl Scalar for Upper {
    fn id(&self) -> ScalarId {
        self.0
    }
    fn parse(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.validate(registry, input)?;
        self.normalize(registry, input)
    }
    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        Ok(input.trim().to_ascii_uppercase())
    }
    fn validate(&self, _registry: &Registry, input: &str) -> Result<(), ScalarError> {
        if input.trim().is_empty() {
            return Err(ScalarError::new(ErrorKind::Empty, "empty"));
        }
        Ok(())
    }
}

/// A hand-written scalar whose accept set is the assembled registry's
/// canonical names: the "custom that consults the registry" case.
pub struct ScalarRef(pub ScalarId);

impl Scalar for ScalarRef {
    fn id(&self) -> ScalarId {
        self.0
    }
    fn parse(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.validate(registry, input)?;
        Ok(input.to_string())
    }
    fn normalize(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.parse(registry, input)
    }
    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        if registry.by_canonical(input).is_some() {
            Ok(())
        } else {
            Err(ScalarError::new(
                ErrorKind::Enum,
                format!("unknown scalar: {input}"),
            ))
        }
    }
}
