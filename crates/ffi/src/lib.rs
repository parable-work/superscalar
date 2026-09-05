//! C ABI over superscalar: panics are caught and returned as an error code
//! (never unwind across C); the caller frees every returned string/result via
//! its `*_free`; values cross as UTF-8, objects as bincode.
//!
//! Two parts. The marshaling library (`abi`, re-exported here) holds the
//! result types, the `catch_unwind` guard, the C-string decoding, the per-hook
//! runners and the free functions; none of it is `#[no_mangle]`. The
//! `export_c_abi!` macro emits the exported symbols over a registry
//! constructor. With the default `builtin` feature this crate is also the
//! built-in assembly: the macro is applied below to `Registry::builtin()`. A
//! downstream applies the same macro to its own assembled registry, depends on
//! this crate with `default-features = false`, and gets the same nine symbols
//! plus whatever extra entry points it writes; the macro checks the feature at
//! compile time through `BUILTIN_ASSEMBLY`.

mod abi;
mod macros;

pub use abi::*;

/// True when this build of the crate carries the built-in assembly (the
/// `builtin` feature). `export_c_abi!` asserts it is false at every call site
/// outside this crate: with it true, a downstream expansion would put a second
/// definition of every exported symbol into the same archive.
pub const BUILTIN_ASSEMBLY: bool = cfg!(feature = "builtin");

#[cfg(feature = "builtin")]
crate::export_c_abi!(@emit registry = superscalar::Registry::builtin, extra = {});

#[cfg(all(test, feature = "builtin"))]
mod tests;
