#![forbid(unsafe_code)]

mod builtin;
pub mod catalog;
pub mod coerce;
pub mod directive;
pub mod error;
pub mod extension;
pub mod metadata;
pub mod registry;
pub mod scalar_metadata;
pub mod scalars;
pub mod temporal_format;

pub use catalog::ScalarId;
pub use coerce::{coerce_bool, coerce_float, coerce_int, coerce_lenient, LenientCoerceResult};
pub use error::{ErrorKind, ScalarError};
pub use extension::{AssembleOptions, AssemblyError, Extension, ExtensionInfo, LegacyAlias};
pub use registry::{
    scalar_def, scalar_for, symbol_from_canonical, PrimitiveKind, Registry, Scalar, ScalarDef,
    ScalarHooks, ScalarTag,
};
pub use scalar_metadata::{
    flat_scalar_name, scalar_metadata_by_canonical_name, ScalarMetadata, SCALAR_METADATA,
};
pub use scalars::datetime::DateTime;
pub use scalars::uuid_scalar::Uuid;
pub use temporal_format::{
    epoch_timestamp_micros, normalize_epoch, TemporalFormat, TemporalMicrosScale,
};
