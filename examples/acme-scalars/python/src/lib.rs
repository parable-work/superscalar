//! The Python module `_native` (parse, normalize, validate, coerce_lenient)
//! over the Acme assembly. The module identifier is the last segment of the
//! import path a packaging tool would give it (`acme_scalars._native`); the
//! cdylib exports exactly one `PyInit__native`.

superscalar_python::export_pymodule!(_native, acme_scalars::registry);
