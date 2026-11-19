//! The six napi exports (parse, normalize, validate, magnitude, parts,
//! coerceLenient) over the Acme assembly. Node loads the built cdylib as a
//! .node addon.

superscalar_napi::export_napi!(acme_scalars::registry);
