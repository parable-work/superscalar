//! The four napi exports (parse, normalize, validate, coerceLenient) over the
//! Acme assembly. Node loads the built cdylib as a .node addon.

superscalar_napi::export_napi!(acme_scalars::registry);
