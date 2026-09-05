//! The nine C entry points of superscalar.h over the Acme assembly. A C
//! consumer includes the generic header and links this archive.

superscalar_ffi::export_c_abi!(acme_scalars::registry);
