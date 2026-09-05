//! The extension contract: how a downstream adds scalars to the registry, and
//! the errors assembly raises when two contributions disagree.

use crate::catalog::ScalarId;
use crate::registry::{Scalar, ScalarDef, ScalarTag};
use serde::Serialize;
use std::fmt;

/// A legacy flat name that generated bindings alias to a canonical symbol
/// (`Email` -> `ContactEmail`). `parse_target` names the generated parse
/// function the alias re-exports when it differs from the default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct LegacyAlias {
    pub name: &'static str,
    pub target: &'static str,
    pub parse_target: Option<&'static str>,
}

/// A compiled-in set of scalars. `Registry::assemble` merges any number of
/// these with the built-ins.
pub trait Extension: Sync {
    /// Short lowercase identifier, unique within an assembly (`"acme"`,
    /// `"widgets"`). Names owners in assembly errors and the dump.
    fn name(&self) -> &'static str;
    /// Multiple of `ScalarId::EXTENSION_BLOCK` and at least `EXTENSION_BLOCK`;
    /// the extension's ids live in `[id_base, id_base + EXTENSION_BLOCK)`. Both
    /// rules are waived by `AssembleOptions::allow_legacy_ids`.
    fn id_base(&self) -> u32;
    /// `'static` because extensions are compiled in: a downstream declares
    /// `static DEFS: [ScalarDef; N]` exactly as the built-in catalog does.
    fn defs(&self) -> &'static [ScalarDef];
    /// Hand-written impls, keyed by the def id each serves. A def with no impl
    /// gets `DirectiveScalar`; assembly rejects a non-`PatternOnly` def without
    /// one.
    fn impls(&self) -> Vec<(ScalarId, Box<dyn Scalar>)>;
    /// Legacy flat names the generated bindings alias to this extension's
    /// symbols.
    fn aliases(&self) -> &'static [LegacyAlias] {
        &[]
    }
}

/// Knobs for `Registry::assemble_with`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AssembleOptions {
    /// Skip the reserved-block and block-range checks (assembly check 2);
    /// collision checks still run. For an extension that keeps ids it was
    /// assigned before the block scheme existed. Never set for a new
    /// extension.
    pub allow_legacy_ids: bool,
}

/// One extension in an assembled registry, as reported by
/// `Registry::extensions` and the dump.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ExtensionInfo {
    pub name: &'static str,
    pub id_base: u32,
    /// True when the extension declares ids outside its block (only possible
    /// under `allow_legacy_ids`).
    pub legacy_ids: bool,
}

/// Why `Registry::try_assemble` refused an extension set. The variants are in
/// the order the checks run; assembly stops at the first failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssemblyError {
    IdBaseNotAligned {
        extension: &'static str,
        id_base: u32,
    },
    IdBaseReserved {
        extension: &'static str,
    },
    IdOutOfBlock {
        extension: &'static str,
        id: ScalarId,
        canonical: &'static str,
    },
    DuplicateId {
        id: ScalarId,
        first: &'static str,
        second: &'static str,
    },
    DuplicateCanonical {
        canonical: &'static str,
        first: &'static str,
        second: &'static str,
    },
    DuplicateExtensionName {
        name: &'static str,
    },
    NamespaceMismatch {
        canonical: &'static str,
        namespace: &'static str,
    },
    DanglingAlias {
        canonical: &'static str,
        alias_of: ScalarId,
    },
    AliasChain {
        canonical: &'static str,
        alias_of: ScalarId,
    },
    ForeignImpl {
        extension: &'static str,
        id: ScalarId,
    },
    ImplIdMismatch {
        registered: ScalarId,
        reported: ScalarId,
    },
    MissingImpl {
        canonical: &'static str,
        tag: ScalarTag,
    },
    UnexpectedImpl {
        canonical: &'static str,
    },
    InvalidPattern {
        canonical: &'static str,
        message: String,
    },
    DanglingLegacyAlias {
        name: &'static str,
        target: &'static str,
    },
}

impl fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblyError::IdBaseNotAligned { extension, id_base } => write!(
                f,
                "extension {extension:?}: id_base {id_base} is not a multiple of {}",
                ScalarId::EXTENSION_BLOCK
            ),
            AssemblyError::IdBaseReserved { extension } => write!(
                f,
                "extension {extension:?}: id_base 0 is the built-in block; extensions start at {}",
                ScalarId::EXTENSION_BLOCK
            ),
            AssemblyError::IdOutOfBlock {
                extension,
                id,
                canonical,
            } => write!(
                f,
                "extension {extension:?}: {canonical} has id {} outside the extension's block",
                id.0
            ),
            AssemblyError::DuplicateId { id, first, second } => write!(
                f,
                "id {} is declared by both {first:?} and {second:?}",
                id.0
            ),
            AssemblyError::DuplicateCanonical {
                canonical,
                first,
                second,
            } => write!(
                f,
                "canonical name {canonical:?} is declared by both {first:?} and {second:?}"
            ),
            AssemblyError::DuplicateExtensionName { name } => {
                write!(f, "extension name {name:?} is used more than once")
            }
            AssemblyError::NamespaceMismatch {
                canonical,
                namespace,
            } => write!(
                f,
                "{canonical}: namespace {namespace:?} is not the canonical prefix"
            ),
            AssemblyError::DanglingAlias {
                canonical,
                alias_of,
            } => write!(
                f,
                "{canonical}: alias_of {} names no assembled scalar",
                alias_of.0
            ),
            AssemblyError::AliasChain {
                canonical,
                alias_of,
            } => write!(f, "{canonical}: alias_of {} is itself an alias", alias_of.0),
            AssemblyError::ForeignImpl { extension, id } => write!(
                f,
                "extension {extension:?} registers an impl for id {}, which it does not declare",
                id.0
            ),
            AssemblyError::ImplIdMismatch {
                registered,
                reported,
            } => write!(
                f,
                "impl registered for id {} reports id {}",
                registered.0, reported.0
            ),
            AssemblyError::MissingImpl { canonical, tag } => write!(
                f,
                "{canonical} is tagged {tag:?} but has no hand-written impl"
            ),
            AssemblyError::UnexpectedImpl { canonical } => write!(
                f,
                "{canonical} is PatternOnly or an alias and must not register an impl"
            ),
            AssemblyError::InvalidPattern { canonical, message } => {
                write!(f, "{canonical}: pattern does not compile: {message}")
            }
            AssemblyError::DanglingLegacyAlias { name, target } => write!(
                f,
                "legacy alias {name:?} targets {target:?}, which is no assembled scalar's symbol"
            ),
        }
    }
}

impl std::error::Error for AssemblyError {}
