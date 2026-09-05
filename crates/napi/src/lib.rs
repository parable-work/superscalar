//! napi-rs (Node) binding over the superscalar core; dispatch errors throw in JS.
//!
//! Two parts. The library (`resolve`, `run_*`) does the work over an explicit
//! registry and carries no `#[napi]` attribute. The `export_napi!` macro emits
//! the four exported functions over a registry constructor. With the default
//! `builtin` feature this crate is also the built-in assembly: the macro is
//! applied below to `Registry::builtin()`. A downstream applies it to its own
//! assembled registry, depends on this crate with `default-features = false`,
//! keeps a `build.rs` calling `napi_build::setup()`, and ships the same four
//! exports; the macro checks the feature at compile time through
//! `BUILTIN_ASSEMBLY`.

use napi::bindgen_prelude::*;
use superscalar::{Registry, Scalar, ScalarId};

/// True when this build of the crate carries the built-in assembly (the
/// `builtin` feature). `export_napi!` asserts it is false at every call site
/// outside this crate: with it true, the downstream addon links without an
/// error and registers both sets of exports, and the built-in registration
/// wins, so the extension's ids come back as "unknown scalar id".
pub const BUILTIN_ASSEMBLY: bool = cfg!(feature = "builtin");

/// The scalar serving `scalar_id` in `registry`, or the error the exports
/// throw for an id no extension declared.
pub fn resolve(registry: &Registry, scalar_id: u32) -> Result<&dyn Scalar> {
    registry
        .scalar(ScalarId(scalar_id))
        .ok_or_else(|| Error::from_reason("unknown scalar id"))
}

/// `parse` over `registry`; a core error becomes a JS error with its message.
pub fn run_parse(registry: &Registry, scalar_id: u32, value: &str) -> Result<String> {
    resolve(registry, scalar_id)?
        .parse(registry, value)
        .map_err(|err| Error::from_reason(err.to_string()))
}

/// `normalize` over `registry`.
pub fn run_normalize(registry: &Registry, scalar_id: u32, value: &str) -> Result<String> {
    resolve(registry, scalar_id)?
        .normalize(registry, value)
        .map_err(|err| Error::from_reason(err.to_string()))
}

/// `validate` over `registry`.
pub fn run_validate(registry: &Registry, scalar_id: u32, value: &str) -> Result<()> {
    resolve(registry, scalar_id)?
        .validate(registry, value)
        .map_err(|err| Error::from_reason(err.to_string()))
}

/// Lenient coercion over `registry`: a JSON document string in, the serialized
/// `LenientCoerceResult` JSON out. Throws only on an unknown id or unparseable
/// `json_in`; a captured coercion failure rides in the result's `error`.
pub fn run_coerce_lenient(registry: &Registry, scalar_id: u32, json_in: &str) -> Result<String> {
    let def = registry
        .def(ScalarId(scalar_id))
        .ok_or_else(|| Error::from_reason("unknown scalar id"))?;
    let value: serde_json::Value =
        serde_json::from_str(json_in).map_err(|_| Error::from_reason("invalid json"))?;
    let result = registry.coerce_lenient(&value, def.canonical);
    serde_json::to_string(&result).map_err(|err| Error::from_reason(err.to_string()))
}

/// Emit the four `#[napi]` exports (`parse`, `normalize`, `validate`,
/// `coerce_lenient`, which napi exposes as `coerceLenient`) over the registry
/// that `$registry()` returns, a path to a `fn() -> &'static Registry`.
///
/// `#[napi]` registers each export through `napi-build`, so the calling crate
/// keeps a `build.rs` with `napi_build::setup()` and depends on `napi` and
/// `napi-derive` directly (the expansion names them as `::napi` and
/// `::napi_derive`). One cdylib invokes this macro once, at the crate root;
/// extra exports are written beside the call.
///
/// The public arms of the macro refuse to expand in a build of this crate that
/// already carries the built-in assembly (`BUILTIN_ASSEMBLY`, the `builtin`
/// feature): a second expansion is a compile error naming the fix, instead of
/// an archive with two definitions of every symbol. The `@emit` arm is the
/// crate's own entry for that assembly and skips the check; a downstream has no
/// reason to use it.
#[macro_export]
macro_rules! export_napi {
    ($registry:path) => {
        const _: () = ::core::assert!(
            !$crate::BUILTIN_ASSEMBLY,
            "superscalar-napi was built with its `builtin` feature and already registers the four exports over Registry::builtin(); depend on it with default-features = false"
        );
        $crate::export_napi!(@emit $registry);
    };
    (@emit $registry:path) => {
        /// Validate shape and return the canonical normalized value (throws on reject).
        #[::napi_derive::napi]
        pub fn parse(scalar_id: u32, value: String) -> ::napi::Result<String> {
            $crate::run_parse($registry(), scalar_id, &value)
        }

        /// Transform toward canonical form without enforcing shape.
        #[::napi_derive::napi]
        pub fn normalize(scalar_id: u32, value: String) -> ::napi::Result<String> {
            $crate::run_normalize($registry(), scalar_id, &value)
        }

        /// Enforce shape; returns undefined on success, throws on reject.
        #[::napi_derive::napi]
        pub fn validate(scalar_id: u32, value: String) -> ::napi::Result<()> {
            $crate::run_validate($registry(), scalar_id, &value)
        }

        /// Lenient ("flag, don't block") coercion: a JSON document string in, the
        /// serialized `LenientCoerceResult` JSON
        /// (`{"value":<json|null>,"error":<{kind,message}|null>}`) out. Throws only on an
        /// unknown id or unparseable `json_in`; a captured coercion failure rides in the
        /// result's `error` rather than throwing.
        #[::napi_derive::napi]
        pub fn coerce_lenient(scalar_id: u32, json_in: String) -> ::napi::Result<String> {
            $crate::run_coerce_lenient($registry(), scalar_id, &json_in)
        }
    };
}

#[cfg(feature = "builtin")]
crate::export_napi!(@emit superscalar::Registry::builtin);
