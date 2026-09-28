//! The extension contract: how a downstream adds scalars to the registry, and
//! the errors assembly raises when two contributions disagree.

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
/// these with the built-ins. A scalar is identified by its canonical name
/// alone; assembly refuses a name two contributions both declare.
pub trait Extension: Sync {
    /// Short lowercase identifier, unique within an assembly (`"acme"`,
    /// `"widgets"`). Names owners in assembly errors and the dump.
    fn name(&self) -> &'static str;
    /// `'static` because extensions are compiled in: a downstream declares
    /// `static DEFS: [ScalarDef; N]` exactly as the built-in catalog does.
    fn defs(&self) -> &'static [ScalarDef];
    /// Hand-written impls, keyed by the canonical name of the def each serves.
    /// A def with no impl gets `DirectiveScalar`; assembly rejects a
    /// non-`PatternOnly` def without one.
    fn impls(&self) -> Vec<(&'static str, Box<dyn Scalar>)>;
    /// Legacy flat names the generated bindings alias to this extension's
    /// symbols.
    fn aliases(&self) -> &'static [LegacyAlias] {
        &[]
    }
}

/// Why `Registry::try_assemble` refused an extension set. The variants are in
/// the order the checks run; assembly stops at the first failure.
/// `Definitions::try_assemble` runs the def-level subset (`DuplicateCanonical`
/// through `AliasChain`) and reports the same variant for the same input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssemblyError {
    DuplicateExtensionName {
        name: &'static str,
    },
    DuplicateCanonical {
        canonical: &'static str,
        first: &'static str,
        second: &'static str,
    },
    NamespaceMismatch {
        canonical: &'static str,
        namespace: &'static str,
    },
    DanglingAlias {
        canonical: &'static str,
        alias_of: &'static str,
    },
    AliasChain {
        canonical: &'static str,
        alias_of: &'static str,
    },
    ForeignImpl {
        extension: &'static str,
        canonical: &'static str,
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
            AssemblyError::DuplicateExtensionName { name } => {
                write!(f, "extension name {name:?} is used more than once")
            }
            AssemblyError::DuplicateCanonical {
                canonical,
                first,
                second,
            } => write!(
                f,
                "canonical name {canonical:?} is declared by both {first:?} and {second:?}"
            ),
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
                "{canonical}: alias_of {alias_of:?} names no assembled scalar"
            ),
            AssemblyError::AliasChain {
                canonical,
                alias_of,
            } => write!(f, "{canonical}: alias_of {alias_of:?} is itself an alias"),
            AssemblyError::ForeignImpl {
                extension,
                canonical,
            } => write!(
                f,
                "extension {extension:?} registers an impl for {canonical:?}, which it does not declare"
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
