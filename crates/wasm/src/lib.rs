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

use superscalar::{Registry, Scalar};
use wasm_bindgen::{JsError, JsValue};

/// True when this build of the crate carries the built-in assembly (the
/// `builtin` feature). `export_wasm!` asserts it is false at every call site
/// outside this crate: with it true, the downstream bundle would carry every
/// export twice and fail to link.
pub const BUILTIN_ASSEMBLY: bool = cfg!(feature = "builtin");

/// The scalar named `scalar` (a canonical name such as `"Contact.Email"`) in
/// `registry`, or the JS error the exports throw for a name no extension
/// declared.
pub fn resolve<'r>(registry: &'r Registry, scalar: &str) -> Result<&'r dyn Scalar, JsError> {
    registry
        .scalar(scalar)
        .ok_or_else(|| unknown_scalar(scalar))
}

fn unknown_scalar(scalar: &str) -> JsError {
    JsError::new(&format!("unknown scalar {scalar:?}"))
}

/// The `name` of the JS error `scalar_parse` throws when the core rejects the
/// input. An unknown scalar name or a failed host call throws a plain `Error`,
/// so a caller can tell an invalid value from a runtime failure.
pub const SCALAR_PARSE_ERROR_NAME: &str = "ScalarParseError";

/// `parse` over `registry`. A rejected value becomes a JS `Error` named
/// `ScalarParseError` carrying the core's message; an unknown name stays a plain
/// `Error`. A consumer can then use the parser for advisory validation without
/// turning a runtime failure into an invalid value.
pub fn run_parse(registry: &Registry, scalar: &str, input: &str) -> Result<String, JsValue> {
    let scalar = resolve(registry, scalar)?;
    scalar.parse(registry, input).map_err(|err| {
        let error = js_sys::Error::new(&err.to_string());
        error.set_name(SCALAR_PARSE_ERROR_NAME);
        error.into()
    })
}

/// `normalize` over `registry`.
pub fn run_normalize(registry: &Registry, scalar: &str, input: &str) -> Result<String, JsError> {
    resolve(registry, scalar)?
        .normalize(registry, input)
        .map_err(|err| JsError::new(&err.to_string()))
}

/// `validate` over `registry`.
pub fn run_validate(registry: &Registry, scalar: &str, input: &str) -> Result<(), JsError> {
    resolve(registry, scalar)?
        .validate(registry, input)
        .map_err(|err| JsError::new(&err.to_string()))
}

/// Lenient coercion over `registry`: JSON document in, the serialized
/// `LenientCoerceResult` JSON out. Throws only on an unknown name or unparseable
/// `json_in`; a captured coercion failure is carried in the result's `error`.
pub fn run_coerce_lenient(
    registry: &Registry,
    scalar: &str,
    json_in: &str,
) -> Result<String, JsError> {
    let def = registry.def(scalar).ok_or_else(|| unknown_scalar(scalar))?;
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
        /// Validate shape and return the canonical normalized value. Throws on
        /// reject, with an error named `ScalarParseError`.
        #[::wasm_bindgen::prelude::wasm_bindgen]
        pub fn scalar_parse(
            scalar: &str,
            input: &str,
        ) -> Result<String, ::wasm_bindgen::JsValue> {
            $crate::run_parse($registry(), scalar, input)
        }

        /// Transform toward canonical form without enforcing shape.
        #[::wasm_bindgen::prelude::wasm_bindgen]
        pub fn scalar_normalize(
            scalar: &str,
            input: &str,
        ) -> Result<String, ::wasm_bindgen::JsError> {
            $crate::run_normalize($registry(), scalar, input)
        }

        /// Enforce shape; returns nothing on success, throws on reject.
        #[::wasm_bindgen::prelude::wasm_bindgen]
        pub fn scalar_validate(scalar: &str, input: &str) -> Result<(), ::wasm_bindgen::JsError> {
            $crate::run_validate($registry(), scalar, input)
        }

        /// Lenient ("flag, don't block") coercion: JSON document in, the serialized
        /// `LenientCoerceResult` JSON (`{"value":<json|null>,"error":<{kind,message}|null>}`)
        /// out. Throws only on an unknown name or unparseable `json_in`; a captured
        /// coercion failure is carried in the result's `error`, not thrown.
        #[::wasm_bindgen::prelude::wasm_bindgen]
        pub fn scalar_coerce_lenient(
            scalar: &str,
            json_in: &str,
        ) -> Result<String, ::wasm_bindgen::JsError> {
            $crate::run_coerce_lenient($registry(), scalar, json_in)
        }
    };
}

#[cfg(feature = "builtin")]
crate::export_wasm!(@emit superscalar::Registry::builtin);
