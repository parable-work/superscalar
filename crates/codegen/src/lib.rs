//! Codegen over a scalar `Registry`: a library (`Emitter`, `Context`,
//! `render`, `run`) that an extension xtask can extend with its own emitters,
//! plus the built-in emitters for Go, TypeScript, Python and the Rust metadata
//! table. Every name, path and package literal comes from `superscalar.toml`
//! (`Config`), never from a template.

pub mod config;
pub mod docs;
mod emitters;

pub use config::{Config, ConfigError};
pub use emitters::builtin_emitters;

use minijinja::Environment;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
pub use superscalar::symbol_from_canonical;
use superscalar::{LegacyAlias, PrimitiveKind, Registry, ScalarDef, ScalarTag};

/// Per-scalar view handed to templates.
#[derive(Clone, Serialize)]
pub struct Entry {
    pub camel: String,
    pub snake: String,
    pub id: u32,
    pub canonical: String,
    pub canonical_literal: String,
    pub is_object: bool,
    pub is_metadata_alias: bool,
    /// The def is an alias or is aliased by another def, so Go emits it as a
    /// type alias of the shared Go type instead of a distinct named type.
    pub is_alias_member: bool,
    pub include_metadata: bool,
    pub primitive: &'static str,
    pub primitive_literal: String,
    pub description_literal: String,
    pub ts_type: String,
    pub ts_type_literal: String,
    pub ts_alias: String,
    pub go_type: String,
    pub go_type_literal: String,
    pub sql_type_literal: String,
    pub json_schema_type_literal: String,
    pub max_length: usize,
    pub min_length: usize,
    pub max_length_literal: String,
    pub min_length_literal: String,
    pub maximum_literal: String,
    pub minimum_literal: String,
    pub pattern: String,
    pub pattern_literal: String,
    pub has_pattern: bool,
    pub has_custom_normalize: bool,
    pub has_custom_parse: bool,
    pub has_custom_validate: bool,
    pub has_validator: bool,
    pub reserved_words: Vec<Literal>,
    pub examples: Vec<Literal>,
    /// True when this scalar is an `alias_of` another, which the generated
    /// `comparable_with` predicate must resolve before it reads a class off a
    /// metadata row.
    pub is_alias: bool,
    /// The alias target's canonical name as a quoted source literal, empty when
    /// this scalar is not an alias. ONE literal serves Go, TypeScript and
    /// Python because all three quote a string identically, the same reason
    /// `canonical_literal` is shared.
    pub alias_target_literal: String,
    /// Per-language source literal for `comparability_class`, shaped here
    /// rather than in the templates, matching how `alias_target_literal` and
    /// `pattern_literal` already work. Go encodes absent as the empty string,
    /// TypeScript as `null`, Python as `None`.
    pub comparability_class_go: String,
    pub comparability_class_ts: String,
    pub comparability_class_py: String,
    /// `ScalarDef::is_sortable()`, precomputed because the generated TS
    /// catalog carries neither `jsonSchemaType` nor `sqlType` and so cannot
    /// re-derive it.
    pub is_sortable: bool,
}

#[derive(Clone, Serialize)]
pub struct Literal {
    pub value: String,
    pub literal: String,
}

pub fn to_snake(symbol: &str) -> String {
    let chars: Vec<char> = symbol.chars().collect();
    let mut out = String::with_capacity(symbol.len() + 4);
    for (i, ch) in chars.iter().copied().enumerate() {
        if ch.is_ascii_uppercase() {
            let prev = i.checked_sub(1).and_then(|idx| chars.get(idx)).copied();
            let next = chars.get(i + 1).copied();
            let boundary_after_lower =
                prev.is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
            let boundary_before_word = prev.is_some_and(|c| c.is_ascii_uppercase())
                && next.is_some_and(|c| c.is_ascii_lowercase());
            if i != 0 && (boundary_after_lower || boundary_before_word) {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn literal(value: &str) -> Literal {
    Literal {
        value: value.to_string(),
        literal: format!("{value:?}"),
    }
}

/// The def's declared type for `language`, or `"string"` when it has none.
pub fn type_mapping(def: &ScalarDef, language: &str) -> String {
    def.type_mappings
        .iter()
        .find_map(|(candidate, ty)| (*candidate == language).then_some(*ty))
        .unwrap_or("string")
        .to_string()
}

/// The emitted TypeScript type: the def's mapping, renamed through
/// `[typescript.type_renames]`.
pub fn ts_type(def: &ScalarDef, config: &Config) -> String {
    let mapped = type_mapping(def, "typescript");
    config
        .typescript
        .as_ref()
        .and_then(|ts| ts.type_renames.get(&mapped).cloned())
        .unwrap_or(mapped)
}

fn ts_alias_type(canonical_literal: &str, ty: &str) -> String {
    if ty == "string" {
        format!("string & {{ readonly __brand: {canonical_literal} }}")
    } else {
        ty.to_string()
    }
}

/// The emitted Go type: `[go.type_overrides]` by canonical name, else the
/// def's mapping.
pub fn go_type(def: &ScalarDef, config: &Config) -> String {
    config
        .go
        .as_ref()
        .and_then(|go| go.type_overrides.get(def.canonical).cloned())
        .unwrap_or_else(|| type_mapping(def, "go"))
}

fn has_validator(def: &ScalarDef) -> bool {
    !matches!(def.tag, ScalarTag::Structural)
}

/// One template entry per assembled def, in `Registry::ids()` order.
pub fn entries(registry: &Registry, config: &Config) -> Vec<Entry> {
    registry
        .defs()
        .map(|def| {
            let id = def.id;
            let canonical = def.canonical.to_string();
            let camel = symbol_from_canonical(&canonical);
            let snake = to_snake(&camel);
            let canonical_literal = format!("{canonical:?}");
            let ts_type = ts_type(def, config);
            let go_type = go_type(def, config);
            let pattern = def.pattern.unwrap_or_default().to_string();
            let alias_target_literal = def
                .alias_of
                .and_then(|alias| registry.def(alias))
                .map(|target| format!("{:?}", target.canonical))
                .unwrap_or_default();
            let is_object = def.primitive == PrimitiveKind::Object;
            let is_alias_member =
                def.alias_of.is_some() || registry.defs().any(|other| other.alias_of == Some(id));
            Entry {
                snake,
                id: id.as_u32(),
                canonical,
                canonical_literal: canonical_literal.clone(),
                is_object,
                is_metadata_alias: is_object && !go_type.starts_with("struct{"),
                is_alias_member,
                include_metadata: !def.metadata_omit,
                primitive: def.metadata_primitive,
                primitive_literal: format!("{:?}", def.metadata_primitive),
                description_literal: format!("{:?}", def.description),
                ts_type_literal: format!("{ts_type:?}"),
                ts_alias: ts_alias_type(&canonical_literal, &ts_type),
                ts_type,
                go_type_literal: format!("{go_type:?}"),
                go_type,
                sql_type_literal: format!("{:?}", def.sql_type),
                json_schema_type_literal: format!("{:?}", def.json_schema_type),
                max_length: def.max_length.unwrap_or_default(),
                min_length: def.min_length.unwrap_or_default(),
                max_length_literal: optional_usize_literal(def.max_length),
                min_length_literal: optional_usize_literal(def.min_length),
                maximum_literal: optional_i64_ptr_literal(def.maximum),
                minimum_literal: optional_i64_ptr_literal(def.minimum),
                pattern_literal: format!("{pattern:?}"),
                pattern,
                has_pattern: def.pattern.is_some(),
                has_custom_normalize: def.hooks.normalize,
                has_custom_parse: def.hooks.parse,
                has_custom_validate: def.hooks.validate,
                has_validator: has_validator(def),
                reserved_words: def
                    .reserved_words
                    .iter()
                    .map(|word| literal(word))
                    .collect(),
                examples: def
                    .examples
                    .iter()
                    .map(|example| literal(example))
                    .collect(),
                is_alias: def.alias_of.is_some(),
                alias_target_literal,
                comparability_class_go: optional_lang_str_literal(def.comparability_class, "\"\""),
                comparability_class_ts: optional_lang_str_literal(def.comparability_class, "null"),
                comparability_class_py: optional_lang_str_literal(def.comparability_class, "None"),
                is_sortable: def.is_sortable(),
                camel,
            }
        })
        .collect()
}

/// Named anchors in the built-in templates where an extension places code
/// it renders itself. The generated file is byte-identical whether the
/// insert is empty or absent; an extension that needs, say, a Go allowlist
/// of its own next to the scalar ids renders it and sets the anchor. The
/// text is emitted verbatim, so an insert ends with its own newline.
pub const GO_AFTER_IDS: &str = "go.after_ids";
/// After the per-scalar `<Camel>Pattern` variables in the Go output.
pub const GO_AFTER_PATTERNS: &str = "go.after_patterns";

/// Everything an emitter renders from: the registry, the config, the shaped
/// entries, the legacy aliases the registry carries, and the inserts an
/// extension supplies for the template anchors.
pub struct Context<'a> {
    pub registry: &'a Registry,
    pub config: &'a Config,
    /// `registry.ids()` order.
    pub entries: Vec<Entry>,
    pub legacy_aliases: &'a [LegacyAlias],
    pub gen_note: &'a str,
    /// Anchor name (`GO_AFTER_IDS`, ...) -> rendered text. Empty for the
    /// built-in assembly.
    pub inserts: BTreeMap<String, String>,
}

impl<'a> Context<'a> {
    pub fn new(registry: &'a Registry, config: &'a Config) -> Context<'a> {
        Context {
            registry,
            config,
            entries: entries(registry, config),
            legacy_aliases: registry.legacy_aliases(),
            gen_note: &config.package.gen_note,
            inserts: BTreeMap::new(),
        }
    }

    /// Indexes into `entries` of the scalars with a metadata row.
    pub fn metadata_entries(&self) -> Vec<&Entry> {
        self.entries
            .iter()
            .filter(|entry| entry.include_metadata)
            .collect()
    }
}

/// Exactly one trailing newline, so the end-of-file-fixer hook cannot fight
/// the drift check.
pub fn finish(mut s: String) -> String {
    while s.ends_with('\n') {
        s.pop();
    }
    s.push('\n');
    s
}

#[derive(Debug)]
pub struct RenderError(pub String);

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for RenderError {}

/// minijinja over one template source.
pub fn render<T: Serialize>(name: &str, source: &str, context: &T) -> Result<String, RenderError> {
    let mut env = Environment::new();
    env.add_template(name, source)
        .map_err(|err| RenderError(format!("{name}: {err}")))?;
    env.get_template(name)
        .map_err(|err| RenderError(format!("{name}: {err}")))?
        .render(context)
        .map_err(|err| RenderError(format!("{name}: {err}")))
}

/// `Ok(Some)` formatted, `Ok(None)` gofmt absent, `Err` gofmt present but
/// failed (never silently emit unformatted Go).
pub fn gofmt(source: &str) -> Result<Option<String>, String> {
    let mut child = match Command::new("gofmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(format!("could not spawn gofmt: {err}")),
    };
    let mut stdin = child.stdin.take().ok_or("gofmt stdin unavailable")?;
    stdin
        .write_all(source.as_bytes())
        .map_err(|err| format!("could not write to gofmt: {err}"))?;
    drop(stdin);
    let output = child
        .wait_with_output()
        .map_err(|err| format!("could not read gofmt output: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "gofmt failed ({}):\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    String::from_utf8(output.stdout)
        .map(Some)
        .map_err(|err| format!("gofmt produced non-UTF-8 output: {err}"))
}

/// What `Emitter::postprocess` hands back. `Unverified` means the formatter
/// the emitter wanted is not installed: `run` still writes the text but skips
/// it in check mode, since the committed file went through the formatter.
pub enum Postprocessed {
    Ready(String),
    Unverified(String),
}

pub trait Emitter {
    /// `"go"`, `"typescript"`, ...
    fn name(&self) -> &str;
    /// Resolved output path, or `None` when the config disables the emitter.
    fn output(&self, config: &Config) -> Option<PathBuf>;
    fn render(&self, ctx: &Context) -> Result<String, RenderError>;
    fn postprocess(
        &self,
        rendered: String,
        _config: &Config,
    ) -> Result<Postprocessed, RenderError> {
        Ok(Postprocessed::Ready(rendered))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Write,
    Check,
}

#[derive(Debug, Default)]
pub struct Report {
    pub written: Vec<PathBuf>,
    pub drifted: Vec<PathBuf>,
    /// Outputs whose formatter was absent, so check mode could not compare.
    pub skipped: Vec<PathBuf>,
    /// Outputs whose config section is absent or disabled.
    pub disabled: Vec<String>,
}

#[derive(Debug)]
pub enum Error {
    Render {
        emitter: String,
        source: RenderError,
    },
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Render { emitter, source } => write!(f, "{emitter}: {source}"),
            Error::Io { path, source } => write!(f, "{}: {source}", path.display()),
        }
    }
}

impl std::error::Error for Error {}

/// Render every emitter, then write or diff. Check mode reports drift in
/// `Report::drifted` instead of exiting, so an xtask can add its own emitters
/// before deciding the exit code.
pub fn run(emitters: &[&dyn Emitter], ctx: &Context, mode: Mode) -> Result<Report, Error> {
    let mut report = Report::default();
    for emitter in emitters {
        let Some(path) = emitter.output(ctx.config) else {
            report.disabled.push(emitter.name().to_string());
            continue;
        };
        let render_error = |source| Error::Render {
            emitter: emitter.name().to_string(),
            source,
        };
        let rendered = finish(emitter.render(ctx).map_err(render_error)?);
        let content = match emitter
            .postprocess(rendered, ctx.config)
            .map_err(render_error)?
        {
            Postprocessed::Ready(content) => content,
            Postprocessed::Unverified(content) => {
                if mode == Mode::Check {
                    report.skipped.push(path);
                    continue;
                }
                content
            }
        };
        match mode {
            Mode::Check => {
                let current = fs::read_to_string(&path).unwrap_or_default();
                if current != content {
                    report.drifted.push(path);
                }
            }
            Mode::Write => {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent).map_err(|source| Error::Io {
                        path: parent.to_path_buf(),
                        source,
                    })?;
                }
                fs::write(&path, content).map_err(|source| Error::Io {
                    path: path.clone(),
                    source,
                })?;
                report.written.push(path);
            }
        }
    }
    Ok(report)
}

/// `Registry::dump()` pretty-printed plus one trailing newline: what
/// `registry dump` writes and what an xtask writes for its own tooling.
pub fn dump_to_string(registry: &Registry) -> String {
    let mut text =
        serde_json::to_string_pretty(&registry.dump()).expect("registry dump serializes");
    text.push('\n');
    text
}

pub fn optional_usize_literal(value: Option<usize>) -> String {
    match value {
        Some(n) => format!("Some({n})"),
        None => "None".to_string(),
    }
}

fn optional_i64_ptr_literal(value: Option<f64>) -> String {
    match value {
        Some(n) if n.fract() == 0.0 && n >= i64::MIN as f64 && n <= i64::MAX as f64 => {
            format!("scalarInt64Ptr({})", n as i64)
        }
        Some(n) => panic!("Go scalar metadata requires an integral i64 bound, got {n}"),
        None => "nil".to_string(),
    }
}

pub fn optional_str_literal(value: Option<&str>) -> String {
    match value {
        Some(s) => format!("Some({s:?})"),
        None => "None".to_string(),
    }
}

/// Shape an optional class name as a source literal for a target language.
/// `none_literal` is what that language writes for "absent": `"\"\""` for Go
/// (which encodes absent as the empty string), `"null"` for TypeScript,
/// `"None"` for Python. `{:?}` on a `&str` produces a double-quoted, escaped
/// literal that is valid in all three for the ASCII class names this field
/// will ever carry.
pub fn optional_lang_str_literal(value: Option<&str>, none_literal: &str) -> String {
    match value {
        Some(s) => format!("{s:?}"),
        None => none_literal.to_string(),
    }
}
