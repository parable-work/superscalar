//! PyO3 binding over superscalar; bad input raises `ValueError`.
//!
//! Two parts. The library (`resolve`, `run_*`) does the work over an explicit
//! registry and carries no `#[pyfunction]` attribute. The `export_pymodule!`
//! macro emits the four `#[pyfunction]`s and the `#[pymodule]` that registers
//! them, over a registry constructor. With the default `builtin` feature this
//! crate is also the built-in assembly: the macro is applied below to
//! `Registry::builtin()` as module `_native`. A downstream applies it to its
//! own assembled registry under its own module name, with any extra
//! `#[pyfunction]`s it writes, and depends on this crate with
//! `default-features = false`; the macro checks that at compile time through
//! `BUILTIN_ASSEMBLY`.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule as PyModuleType};
use superscalar::{Registry, Scalar, ScalarId};

/// True when this build of the crate carries the built-in assembly (the
/// `builtin` feature). `export_pymodule!` asserts it is false at every call
/// site outside this crate: with it true, the downstream cdylib would hold two
/// `PyInit_*` symbols.
pub const BUILTIN_ASSEMBLY: bool = cfg!(feature = "builtin");

/// The scalar serving `scalar_id` in `registry`, or the `ValueError` the
/// functions raise for an id no extension declared.
pub fn resolve(registry: &Registry, scalar_id: u32) -> PyResult<&dyn Scalar> {
    registry
        .scalar(ScalarId(scalar_id))
        .ok_or_else(|| PyValueError::new_err("unknown scalar id"))
}

/// `parse` over `registry`; a core error becomes a `ValueError` with its
/// message.
pub fn run_parse(registry: &Registry, scalar_id: u32, value: &str) -> PyResult<String> {
    resolve(registry, scalar_id)?
        .parse(registry, value)
        .map_err(|err| PyValueError::new_err(err.to_string()))
}

/// `normalize` over `registry`.
pub fn run_normalize(registry: &Registry, scalar_id: u32, value: &str) -> PyResult<String> {
    resolve(registry, scalar_id)?
        .normalize(registry, value)
        .map_err(|err| PyValueError::new_err(err.to_string()))
}

/// `validate` over `registry`.
pub fn run_validate(registry: &Registry, scalar_id: u32, value: &str) -> PyResult<()> {
    resolve(registry, scalar_id)?
        .validate(registry, value)
        .map_err(|err| PyValueError::new_err(err.to_string()))
}

/// Lenient coercion over `registry`: a JSON document string in, a
/// `{"value": <decoded JSON | None>, "error": {"kind", "message"} | None}` dict
/// out. A coercion failure does not raise: it rides in `error`. An unknown id
/// or unparseable `json_value` raises `ValueError`. The coerced `value` is
/// round-tripped back to native Python via `json.loads`.
pub fn run_coerce_lenient<'py>(
    py: Python<'py>,
    registry: &Registry,
    scalar_id: u32,
    json_value: &str,
) -> PyResult<Bound<'py, PyDict>> {
    let def = registry
        .def(ScalarId(scalar_id))
        .ok_or_else(|| PyValueError::new_err("unknown scalar id"))?;
    let value: serde_json::Value =
        serde_json::from_str(json_value).map_err(|err| PyValueError::new_err(err.to_string()))?;
    let result = registry.coerce_lenient(&value, def.canonical);

    let out = PyDict::new(py);
    match result.value {
        Some(value) => {
            // Round-trip through json.loads so callers get native Python objects
            // (int/str/...), matching how the other _native fns carry JSON.
            let encoded = serde_json::to_string(&value)
                .map_err(|err| PyValueError::new_err(err.to_string()))?;
            let json_module = PyModuleType::import(py, "json")?;
            let decoded = json_module.getattr("loads")?.call1((encoded,))?;
            out.set_item("value", decoded)?;
        }
        None => out.set_item("value", py.None())?,
    }
    match result.error {
        Some(error) => {
            let error_dict = PyDict::new(py);
            error_dict.set_item("kind", error.kind.as_str())?;
            error_dict.set_item("message", error.message)?;
            out.set_item("error", error_dict)?;
        }
        None => out.set_item("error", py.None())?,
    }
    Ok(out)
}

/// Emit the Python module `$module` over the registry that `$registry()`
/// returns, a path to a `fn() -> &'static Registry`. The module holds the four
/// `#[pyfunction]`s `parse`, `normalize`, `validate`, `coerce_lenient`, plus
/// every function named in the optional `extra = [f, g]` list, which must be
/// `#[pyfunction]`s visible at the call site.
///
/// `#[pymodule]` names the Python module after the function identifier and a
/// cdylib holds exactly one `PyInit_*` symbol, so `$module` is a required
/// identifier that must equal the last segment of `module-name` in the
/// downstream `pyproject.toml` (`_native` for `superscalar._native`). One
/// cdylib invokes this macro once, at the crate root. The expansion names the
/// framework as `::pyo3`, so the calling crate depends on `pyo3` directly.
///
/// ```ignore
/// superscalar_python::export_pymodule!(_native, superscalar::Registry::builtin);
/// superscalar_python::export_pymodule!(
///     _native,
///     my_extension::registry,
///     extra = [my_hash, my_other_function]
/// );
/// ```
///
/// The public arms of the macro refuse to expand in a build of this crate that
/// already carries the built-in assembly (`BUILTIN_ASSEMBLY`, the `builtin`
/// feature): a second expansion is a compile error naming the fix, instead of
/// an archive with two definitions of every symbol. The `@emit` arm is the
/// crate's own entry for that assembly and skips the check; a downstream has no
/// reason to use it.
#[macro_export]
macro_rules! export_pymodule {
    ($module:ident, $registry:path) => {
        $crate::export_pymodule!($module, $registry, extra = []);
    };
    ($module:ident, $registry:path, extra = [$($extra:path),* $(,)?]) => {
        const _: () = ::core::assert!(
            !$crate::BUILTIN_ASSEMBLY,
            "superscalar-python was built with its `builtin` feature and already defines the `_native` module over Registry::builtin(); depend on it with default-features = false"
        );
        $crate::export_pymodule!(@emit $module, $registry, extra = [$($extra),*]);
    };
    (@emit $module:ident, $registry:path, extra = [$($extra:path),*]) => {
        /// Validate shape and return the canonical normalized value.
        #[::pyo3::pyfunction]
        fn parse(scalar_id: u32, value: &str) -> ::pyo3::PyResult<String> {
            $crate::run_parse($registry(), scalar_id, value)
        }

        /// Transform toward canonical form without enforcing shape.
        #[::pyo3::pyfunction]
        fn normalize(scalar_id: u32, value: &str) -> ::pyo3::PyResult<String> {
            $crate::run_normalize($registry(), scalar_id, value)
        }

        /// Enforce shape; returns None on success, raises ValueError on reject.
        #[::pyo3::pyfunction]
        fn validate(scalar_id: u32, value: &str) -> ::pyo3::PyResult<()> {
            $crate::run_validate($registry(), scalar_id, value)
        }

        /// Lenient ("flag, don't block") coercion: a JSON document string in, a
        /// `{"value": <decoded JSON | None>, "error": {"kind", "message"} | None}` dict
        /// out.
        ///
        /// Unlike `parse`/`normalize`/`validate`, a coercion failure does NOT raise: it
        /// rides in `error` so a caller can quarantine a row or null+flag a cell. An
        /// unknown id or unparseable `json_value` still raises `ValueError`. The coerced
        /// `value` is round-tripped back to native Python via `json.loads`.
        #[::pyo3::pyfunction]
        fn coerce_lenient<'py>(
            py: ::pyo3::Python<'py>,
            scalar_id: u32,
            json_value: &str,
        ) -> ::pyo3::PyResult<::pyo3::Bound<'py, ::pyo3::types::PyDict>> {
            $crate::run_coerce_lenient(py, $registry(), scalar_id, json_value)
        }

        // `add_function` is a trait method; the fully qualified form works at a
        // call site that has not imported `PyModuleMethods`.
        #[::pyo3::pymodule]
        fn $module(m: &::pyo3::Bound<'_, ::pyo3::types::PyModule>) -> ::pyo3::PyResult<()> {
            ::pyo3::types::PyModuleMethods::add_function(m, ::pyo3::wrap_pyfunction!(parse, m)?)?;
            ::pyo3::types::PyModuleMethods::add_function(m, ::pyo3::wrap_pyfunction!(normalize, m)?)?;
            ::pyo3::types::PyModuleMethods::add_function(m, ::pyo3::wrap_pyfunction!(validate, m)?)?;
            ::pyo3::types::PyModuleMethods::add_function(
                m,
                ::pyo3::wrap_pyfunction!(coerce_lenient, m)?,
            )?;
            $(::pyo3::types::PyModuleMethods::add_function(m, ::pyo3::wrap_pyfunction!($extra, m)?)?;)*
            Ok(())
        }
    };
}

#[cfg(feature = "builtin")]
crate::export_pymodule!(@emit _native, superscalar::Registry::builtin, extra = []);
