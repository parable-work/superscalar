//! The four wasm exports (scalar_parse, scalar_normalize, scalar_validate,
//! scalar_coerce_lenient) over the Acme assembly. Built with wasm-pack.

superscalar_wasm::export_wasm!(acme_scalars::registry);
