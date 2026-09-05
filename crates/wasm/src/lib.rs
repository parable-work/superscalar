//! wasm-bindgen build of the scalar boundary; dispatch errors throw in JS.
//!
//! Two parts. The library (`resolve`, `run_*`) does the work over an explicit
//! registry and carries no `#[wasm_bindgen]` attribute. The `export_wasm!`
//! macro emits the four exported functions over a registry constructor. With
//! the default `builtin` feature this crate is also the built-in assembly: the
//! macro is applied below to `Registry::builtin()`. A downstream applies it to
//! its own assembled registry, depends on this crate with
//! `default-features = false`, and ships the same four exports; the macro
//! checks that at compile time through `BUILTIN_ASSEMBLY`.

use superscalar::{Registry, Scalar, ScalarId};
use wasm_bindgen::JsError;

/// True when this build of the crate carries the built-in assembly (the
/// `builtin` feature). `export_wasm!` asserts it is false at every call site
/// outside this crate: with it true, the downstream bundle would carry every
/// export twice and fail to link.
pub const BUILTIN_ASSEMBLY: bool = cfg!(feature = "builtin");

/// The scalar serving `scalar_id` in `registry`, or the JS error the exports
/// throw for an id no extension declared.
pub fn resolve(registry: &Registry, scalar_id: u32) -> Result<&dyn Scalar, JsError> {
    registry
        .scalar(ScalarId(scalar_id))
        .ok_or_else(|| JsError::new("unknown scalar id"))
}

/// `parse` over `registry`; a core error becomes a JS error with its message.
pub fn run_parse(registry: &Registry, scalar_id: u32, input: &str) -> Result<String, JsError> {
    resolve(registry, scalar_id)?
        .parse(registry, input)
        .map_err(|err| JsError::new(&err.to_string()))
}

/// `normalize` over `registry`.
pub fn run_normalize(registry: &Registry, scalar_id: u32, input: &str) -> Result<String, JsError> {
    resolve(registry, scalar_id)?
        .normalize(registry, input)
        .map_err(|err| JsError::new(&err.to_string()))
}

/// `validate` over `registry`.
pub fn run_validate(registry: &Registry, scalar_id: u32, input: &str) -> Result<(), JsError> {
    resolve(registry, scalar_id)?
        .validate(registry, input)
        .map_err(|err| JsError::new(&err.to_string()))
}

/// Lenient coercion over `registry`: JSON document in, the serialized
/// `LenientCoerceResult` JSON out. Throws only on an unknown id or unparseable
/// `json_in`; a captured coercion failure is carried in the result's `error`.
pub fn run_coerce_lenient(
    registry: &Registry,
    scalar_id: u32,
    json_in: &str,
) -> Result<String, JsError> {
    let def = registry
        .def(ScalarId(scalar_id))
        .ok_or_else(|| JsError::new("unknown scalar id"))?;
    let value: serde_json::Value =
        serde_json::from_str(json_in).map_err(|_| JsError::new("invalid json"))?;
    let result = registry.coerce_lenient(&value, def.canonical);
    serde_json::to_string(&result).map_err(|err| JsError::new(&err.to_string()))
}

/// Emit the four `#[wasm_bindgen]` exports (`scalar_parse`, `scalar_normalize`,
/// `scalar_validate`, `scalar_coerce_lenient`) over the registry that
/// `$registry()` returns, a path to a `fn() -> &'static Registry`.
///
/// `#[wasm_bindgen]` exports each function under its Rust name, so one cdylib
/// invokes this macro once, at the crate root. Extra exports are written beside
/// the call. The expansion names the framework as `::wasm_bindgen`, so the
/// calling crate depends on `wasm-bindgen` directly.
///
/// The public arms of the macro refuse to expand in a build of this crate that
/// already carries the built-in assembly (`BUILTIN_ASSEMBLY`, the `builtin`
/// feature): a second expansion is a compile error naming the fix, instead of
/// an archive with two definitions of every symbol. The `@emit` arm is the
/// crate's own entry for that assembly and skips the check; a downstream has no
/// reason to use it.
#[macro_export]
macro_rules! export_wasm {
    ($registry:path) => {
        const _: () = ::core::assert!(
            !$crate::BUILTIN_ASSEMBLY,
            "superscalar-wasm was built with its `builtin` feature and already exports the four functions over Registry::builtin(); depend on it with default-features = false"
        );
        $crate::export_wasm!(@emit $registry);
    };
    (@emit $registry:path) => {
        /// Validate shape and return the canonical normalized value. Throws on reject.
        #[::wasm_bindgen::prelude::wasm_bindgen]
        pub fn scalar_parse(
            scalar_id: u32,
            input: &str,
        ) -> Result<String, ::wasm_bindgen::JsError> {
            $crate::run_parse($registry(), scalar_id, input)
        }

        /// Transform toward canonical form without enforcing shape.
        #[::wasm_bindgen::prelude::wasm_bindgen]
        pub fn scalar_normalize(
            scalar_id: u32,
            input: &str,
        ) -> Result<String, ::wasm_bindgen::JsError> {
            $crate::run_normalize($registry(), scalar_id, input)
        }

        /// Enforce shape; returns nothing on success, throws on reject.
        #[::wasm_bindgen::prelude::wasm_bindgen]
        pub fn scalar_validate(scalar_id: u32, input: &str) -> Result<(), ::wasm_bindgen::JsError> {
            $crate::run_validate($registry(), scalar_id, input)
        }

        /// Lenient ("flag, don't block") coercion: JSON document in, the serialized
        /// `LenientCoerceResult` JSON (`{"value":<json|null>,"error":<{kind,message}|null>}`)
        /// out. Throws only on an unknown id or unparseable `json_in`; a captured
        /// coercion failure is carried in the result's `error`, not thrown.
        #[::wasm_bindgen::prelude::wasm_bindgen]
        pub fn scalar_coerce_lenient(
            scalar_id: u32,
            json_in: &str,
        ) -> Result<String, ::wasm_bindgen::JsError> {
            $crate::run_coerce_lenient($registry(), scalar_id, json_in)
        }
    };
}

#[cfg(feature = "builtin")]
crate::export_wasm!(@emit superscalar::Registry::builtin);
