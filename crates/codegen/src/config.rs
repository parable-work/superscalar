//! `superscalar.toml`: every package name, import path, output path and
//! native entry name the templates used to carry as literals. Paths are
//! relative to the directory that holds the config file. Maps deserialize
//! into `BTreeMap`s and emitters only look keys up, so map order in the file
//! cannot change output.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ConfigError {
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    Parse {
        path: PathBuf,
        message: String,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Read { path, source } => {
                write!(f, "could not read {}: {source}", path.display())
            }
            ConfigError::Parse { path, message } => {
                write!(f, "could not parse {}: {message}", path.display())
            }
        }
    }
}

impl std::error::Error for ConfigError {}

fn default_gen_note() -> String {
    "@generated; do not edit".to_string()
}

fn default_true() -> bool {
    true
}

fn default_scalar_list_type() -> String {
    "string".to_string()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageConfig {
    /// The package this config generates for. Informational: no template
    /// reads it yet.
    pub name: String,
    #[serde(default = "default_gen_note")]
    pub gen_note: String,
}

/// Where the CLI's registry comes from. Extensions are compiled in, so the
/// CLI can only see `Registry::builtin()` and `builtin` is the one value; an
/// extension gets codegen over its assembled registry by calling the library
/// from its own xtask, which supplies the registry in code and ignores this
/// section. Any other value is a parse error rather than a runtime refusal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegistrySource {
    Builtin,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistryConfig {
    pub source: RegistrySource,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub out: PathBuf,
    /// The `package <name>` line.
    pub package: String,
    /// Informational: docs and headers. No template reads it.
    pub module: String,
    /// Informational: the C header the hand-written cgo directives include.
    /// No template reads it.
    pub header: PathBuf,
    /// Informational: the `-l<name>` library in those directives. No template
    /// reads it.
    pub cgo_library: String,
    /// Run gofmt over the rendered file. An absent gofmt skips the Go drift
    /// check; a failing one is an error.
    #[serde(default = "default_true")]
    pub gofmt: bool,
    /// Canonical name -> Go type, replacing the def's `type_mappings["go"]`.
    #[serde(default)]
    pub type_overrides: BTreeMap<String, String>,
    /// The element type of the generated `VALID_SCALARS` list of canonical
    /// names. `string` by default; a package with a named scalar-name type
    /// (one of its own scalars) names it here.
    #[serde(default = "default_scalar_list_type")]
    pub scalar_list_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypeImport {
    pub module: String,
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypeScriptConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub out: PathBuf,
    pub package: String,
    /// `import { backend } from "<backend_module>"`.
    pub backend_module: String,
    /// `import type { ScalarValidationResult, ValidationError } from "<validation_module>"`.
    pub validation_module: String,
    /// `type_mappings["typescript"]` value -> emitted TS type.
    #[serde(default)]
    pub type_renames: BTreeMap<String, String>,
    /// Type-only imports re-exported from the generated module, for object
    /// scalars whose TS type is a hand-written interface. A legacy alias
    /// whose name is one of these is emitted without its `export type` line,
    /// since the re-export already declares the name.
    #[serde(default)]
    pub type_imports: Vec<TypeImport>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PythonConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub out: PathBuf,
    pub package: String,
    /// The compiled module, as a relative import: `._native` renders as
    /// `from . import _native`.
    pub native_module: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RustMetadataConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub out: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocsConfig {
    /// Conformance vector files, merged; each is the v2 corpus shape.
    pub vectors: Vec<PathBuf>,
    /// Absolute link prefix for the generated pages, site base included.
    pub link_prefix: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub package: PackageConfig,
    pub registry: RegistryConfig,
    #[serde(default)]
    pub go: Option<GoConfig>,
    #[serde(default)]
    pub typescript: Option<TypeScriptConfig>,
    #[serde(default)]
    pub python: Option<PythonConfig>,
    #[serde(default)]
    pub rust_metadata: Option<RustMetadataConfig>,
    #[serde(default)]
    pub docs: Option<DocsConfig>,
    /// Settings an extension xtask owns, opaque to this library. An xtask
    /// deserializes the tables it defines from here and gets the same
    /// config-relative path handling as the sections above; the library
    /// never reads them, so a downstream can keep everything in one file.
    #[serde(default)]
    pub extension: toml::Table,
    /// Directory every relative path resolves against: the config file's.
    #[serde(skip)]
    pub root: PathBuf,
}

impl Config {
    pub fn load(path: &Path) -> Result<Config, ConfigError> {
        let text = std::fs::read_to_string(path).map_err(|source| ConfigError::Read {
            path: path.to_path_buf(),
            source,
        })?;
        let root = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        Config::parse(&text, root).map_err(|err| match err {
            ConfigError::Parse { message, .. } => ConfigError::Parse {
                path: path.to_path_buf(),
                message,
            },
            other => other,
        })
    }

    /// Parse config text with relative paths resolved against `root`.
    pub fn parse(text: &str, root: PathBuf) -> Result<Config, ConfigError> {
        let mut config: Config = toml::from_str(text).map_err(|err| ConfigError::Parse {
            path: root.join("superscalar.toml"),
            message: err.to_string(),
        })?;
        config.root = root;
        Ok(config)
    }

    /// A config path resolved against the config directory.
    pub fn resolve(&self, relative: &Path) -> PathBuf {
        if relative.is_absolute() {
            relative.to_path_buf()
        } else {
            self.root.join(relative)
        }
    }

    /// One `[extension.<name>]` table deserialized into the xtask's own type.
    /// `Ok(None)` when the table is absent; a present table that does not fit
    /// `T` is a parse error naming the table.
    pub fn extension_table<T: serde::de::DeserializeOwned>(
        &self,
        name: &str,
    ) -> Result<Option<T>, ConfigError> {
        let Some(value) = self.extension.get(name) else {
            return Ok(None);
        };
        value
            .clone()
            .try_into()
            .map(Some)
            .map_err(|err| ConfigError::Parse {
                path: self.root.join("superscalar.toml"),
                message: format!("[extension.{name}]: {err}"),
            })
    }
}
