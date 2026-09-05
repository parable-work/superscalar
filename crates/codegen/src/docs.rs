//! The generated scalar reference: one Markdown page per scalar from the
//! registry and the conformance vectors, plus an index, with Starlight
//! frontmatter (`title`, `description`, `sidebar.order`). `check` is the gate
//! that fails a scalar with no vectors or no description.

use crate::config::Config;
use crate::{go_type, symbol_from_canonical, to_snake, ts_type, type_mapping};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use superscalar::{PrimitiveKind, Registry, ScalarDef};

#[derive(Debug)]
pub enum DocsError {
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    Parse {
        path: PathBuf,
        message: String,
    },
    DuplicateScalar {
        canonical: String,
        path: PathBuf,
    },
    Write {
        path: PathBuf,
        source: std::io::Error,
    },
    NoDocsSection,
}

impl fmt::Display for DocsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocsError::Read { path, source } => {
                write!(f, "could not read {}: {source}", path.display())
            }
            DocsError::Parse { path, message } => {
                write!(f, "could not parse {}: {message}", path.display())
            }
            DocsError::DuplicateScalar { canonical, path } => write!(
                f,
                "{} repeats vectors for {canonical}, which an earlier file already holds",
                path.display()
            ),
            DocsError::Write { path, source } => {
                write!(f, "could not write {}: {source}", path.display())
            }
            DocsError::NoDocsSection => f.write_str("superscalar.toml has no [docs] section"),
        }
    }
}

impl std::error::Error for DocsError {}

#[derive(Debug, Clone, Deserialize)]
pub struct AcceptedVector {
    pub input: String,
    #[serde(default)]
    pub normalized: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RejectedVector {
    pub input: String,
    #[serde(default)]
    pub validator: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ScalarVectors {
    #[serde(default)]
    pub accepted: Vec<AcceptedVector>,
    #[serde(default)]
    pub rejected: Vec<RejectedVector>,
}

impl ScalarVectors {
    pub fn is_empty(&self) -> bool {
        self.accepted.is_empty() && self.rejected.is_empty()
    }
}

/// The `scalars` sections of one or more v2 corpus files, merged. Each scalar
/// may appear in exactly one file.
#[derive(Debug, Clone, Default)]
pub struct Vectors {
    pub scalars: BTreeMap<String, ScalarVectors>,
}

#[derive(Deserialize)]
struct Corpus {
    #[serde(default)]
    scalars: BTreeMap<String, ScalarVectors>,
}

impl Vectors {
    pub fn load(paths: &[PathBuf]) -> Result<Vectors, DocsError> {
        let mut merged = Vectors::default();
        for path in paths {
            let text = fs::read_to_string(path).map_err(|source| DocsError::Read {
                path: path.clone(),
                source,
            })?;
            let corpus: Corpus = serde_json::from_str(&text).map_err(|err| DocsError::Parse {
                path: path.clone(),
                message: err.to_string(),
            })?;
            for (canonical, vectors) in corpus.scalars {
                if merged.scalars.contains_key(&canonical) {
                    return Err(DocsError::DuplicateScalar {
                        canonical,
                        path: path.clone(),
                    });
                }
                merged.scalars.insert(canonical, vectors);
            }
        }
        Ok(merged)
    }

    pub fn for_scalar(&self, canonical: &str) -> Option<&ScalarVectors> {
        self.scalars.get(canonical)
    }
}

/// Load the vector files named by `[docs].vectors`.
pub fn load_vectors(config: &Config) -> Result<Vectors, DocsError> {
    let docs = config.docs.as_ref().ok_or(DocsError::NoDocsSection)?;
    let paths: Vec<PathBuf> = docs.vectors.iter().map(|p| config.resolve(p)).collect();
    Vectors::load(&paths)
}

/// `Crypto.RSAPrivateKey` -> `crypto-rsa-private-key`.
pub fn page_slug(canonical: &str) -> String {
    to_snake(&symbol_from_canonical(canonical)).replace('_', "-")
}

/// Every reason the reference cannot be generated completely, one line per
/// scalar: a missing description, or no vectors. Also flags vectors that
/// name no registered scalar, since that is a typo the corpus runners would
/// otherwise never see.
pub fn check(registry: &Registry, vectors: &Vectors) -> Vec<String> {
    let mut problems = Vec::new();
    for def in registry.defs() {
        if def.description.trim().is_empty() {
            problems.push(format!("{}: no description", def.canonical));
        }
        match vectors.for_scalar(def.canonical) {
            Some(v) if !v.is_empty() => {}
            _ => problems.push(format!("{}: no conformance vectors", def.canonical)),
        }
    }
    for canonical in vectors.scalars.keys() {
        if registry.by_canonical(canonical).is_none() {
            problems.push(format!(
                "{canonical}: vectors name a scalar the registry does not hold"
            ));
        }
    }
    problems
}

/// Per-page limit on an input shown in a vector table. Longer inputs (the RSA
/// key fixtures) are cut with the full length noted.
const MAX_CELL_CHARS: usize = 160;

/// A value as an inline code span safe inside a GFM table cell: pipes
/// escaped, newlines shown as `\n`, a fence longer than any backtick run
/// inside, and a note when the value was cut.
fn code_cell(value: &str) -> String {
    if value.is_empty() {
        return "(empty)".to_string();
    }
    let mut shown: String = value.chars().take(MAX_CELL_CHARS).collect();
    let cut = shown.chars().count() < value.chars().count();
    // Backslashes stay as they are: a code span shows them literally, so the
    // pattern `\.` reads as written. Control characters become their escapes.
    shown = shown
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
        .replace('|', "\\|");
    let longest_run = shown.split(|c| c != '`').map(str::len).max().unwrap_or(0);
    let fence = "`".repeat(longest_run + 1);
    let pad = if shown.starts_with('`') || shown.ends_with('`') {
        " "
    } else {
        ""
    };
    let mut cell = format!("{fence}{pad}{shown}{pad}{fence}");
    if cut {
        cell.push_str(&format!(" (cut; {} characters)", value.chars().count()));
    }
    cell
}

/// Prose from the registry or the corpus (descriptions, docstrings, vector
/// notes) rendered literally: the characters that would open emphasis, a
/// link, raw HTML or a blockquote are backslash-escaped outside backtick code
/// spans, and code spans are kept as written so a docstring's `like_this`
/// still renders as code. Pages are `.md`, so braces need no escaping.
fn escape_prose(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 8);
    let mut in_code = false;
    for ch in value.chars() {
        if ch == '`' {
            in_code = !in_code;
        } else if !in_code && matches!(ch, '\\' | '*' | '_' | '<' | '>' | '[' | ']') {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}

/// Prose inside a GFM table cell: escaped, pipes escaped, one line.
fn text_cell(value: &str) -> String {
    escape_prose(value).replace('|', "\\|").replace('\n', " ")
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn number(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 9.3e18 {
        format!("{}", value as i64)
    } else {
        value.to_string()
    }
}

fn frontmatter(title: &str, description: &str, order: usize) -> String {
    // A JSON string is a valid YAML double-quoted string, so the description
    // (which may carry quotes, colons or backslashes) goes through serde_json.
    let description = serde_json::to_string(description).expect("string serializes");
    format!("---\ntitle: {title}\ndescription: {description}\nsidebar:\n  order: {order}\n---\n")
}

/// One scalar's reference page.
pub fn render_page(
    def: &ScalarDef,
    registry: &Registry,
    config: &Config,
    vectors: Option<&ScalarVectors>,
    order: usize,
) -> String {
    let canonical = def.canonical;
    let symbol = symbol_from_canonical(canonical);
    let snake = to_snake(&symbol);
    let owner = registry.owner(def.id).unwrap_or("builtin");
    let mut out = frontmatter(canonical, def.description, order);

    out.push('\n');
    out.push_str(&escape_prose(def.description.trim()));
    out.push('\n');
    if !def.docstring.trim().is_empty() {
        out.push('\n');
        out.push_str(&escape_prose(def.docstring.trim()));
        out.push('\n');
    }

    out.push_str("\n## Identity\n\n| Field | Value |\n| --- | --- |\n");
    out.push_str(&format!("| Canonical name | `{canonical}` |\n"));
    out.push_str(&format!("| Id | {} |\n", def.id.as_u32()));
    out.push_str(&format!("| Namespace | {} |\n", def.namespace));
    out.push_str(&format!("| Owner | {owner} |\n"));
    out.push_str(&format!("| Primitive | {:?} |\n", def.primitive));
    out.push_str(&format!("| Kind | {:?} |\n", def.tag));
    out.push_str(&format!("| SQL type | {} |\n", code_cell(def.sql_type)));
    out.push_str(&format!(
        "| JSON Schema type | {} |\n",
        code_cell(def.json_schema_type)
    ));
    if let Some(alias) = def.alias_of {
        let target = registry
            .def(alias)
            .map(|target| target.canonical)
            .unwrap_or("unknown");
        out.push_str(&format!("| Alias of | `{target}` |\n"));
    }
    out.push_str(&format!(
        "| Case-insensitive | {} |\n",
        yes_no(def.case_insensitive)
    ));
    out.push_str(&format!("| Sortable | {} |\n", yes_no(def.is_sortable())));
    out.push_str(&format!(
        "| Comparability class | {} |\n",
        def.comparability_class
            .unwrap_or("none (self-comparable only)")
    ));

    out.push_str("\n## Constraints\n\n");
    let has_constraint = def.pattern.is_some()
        || def.min_length.is_some()
        || def.max_length.is_some()
        || def.minimum.is_some()
        || def.maximum.is_some()
        || !def.reserved_words.is_empty();
    if has_constraint {
        out.push_str("| Constraint | Value |\n| --- | --- |\n");
        if let Some(pattern) = def.pattern {
            out.push_str(&format!("| Pattern | {} |\n", code_cell(pattern)));
        }
        if let Some(n) = def.min_length {
            out.push_str(&format!("| Min length | {n} |\n"));
        }
        if let Some(n) = def.max_length {
            out.push_str(&format!("| Max length | {n} |\n"));
        }
        if let Some(n) = def.minimum {
            out.push_str(&format!("| Minimum | {} |\n", number(n)));
        }
        if let Some(n) = def.maximum {
            out.push_str(&format!("| Maximum | {} |\n", number(n)));
        }
        if !def.reserved_words.is_empty() {
            let words: Vec<String> = def.reserved_words.iter().map(|w| code_cell(w)).collect();
            out.push_str(&format!("| Reserved words | {} |\n", words.join(", ")));
            out.push_str(&format!(
                "| Reserved words case-insensitive | {} |\n",
                yes_no(def.reserved_words_case_insensitive)
            ));
            out.push_str(&format!(
                "| Reserved words match partial | {} |\n",
                yes_no(def.reserved_words_match_partial)
            ));
        }
    } else {
        out.push_str(
            "No pattern or bounds are declared; the scalar's own parse and validate hooks decide.\n",
        );
    }

    out.push_str("\n## Examples\n\n");
    if def.examples.is_empty() {
        out.push_str("None declared.\n");
    } else {
        for example in def.examples {
            out.push_str(&format!("- {}\n", code_cell(example)));
        }
    }

    out.push_str("\n## Accepted inputs\n\n");
    match vectors {
        Some(v) if !v.accepted.is_empty() => {
            out.push_str("| Input | Canonical form | Note |\n| --- | --- | --- |\n");
            for vec in &v.accepted {
                let normalized = vec.normalized.as_deref().unwrap_or(&vec.input);
                out.push_str(&format!(
                    "| {} | {} | {} |\n",
                    code_cell(&vec.input),
                    code_cell(normalized),
                    text_cell(vec.note.as_deref().unwrap_or(""))
                ));
            }
        }
        _ => out.push_str("No accepted vectors in the conformance corpus.\n"),
    }

    out.push_str("\n## Rejected inputs\n\n");
    match vectors {
        Some(v) if !v.rejected.is_empty() => {
            out.push_str("| Input | Validator | Note |\n| --- | --- | --- |\n");
            for vec in &v.rejected {
                out.push_str(&format!(
                    "| {} | {} | {} |\n",
                    code_cell(&vec.input),
                    text_cell(vec.validator.as_deref().unwrap_or("")),
                    text_cell(vec.note.as_deref().unwrap_or(""))
                ));
            }
        }
        _ => out.push_str("No rejected vectors in the conformance corpus.\n"),
    }

    out.push_str("\n## Per-language\n\n| Language | Type | Functions |\n| --- | --- | --- |\n");
    let go_functions = if def.primitive == PrimitiveKind::Object {
        "none (object-valued; use the hand-written Go type)".to_string()
    } else {
        format!("`Parse{symbol}`, `Normalize{symbol}`, `Validate{symbol}`")
    };
    out.push_str(&format!(
        "| Go | {} | {go_functions} |\n",
        code_cell(&go_type(def, config))
    ));
    let ts = ts_type(def, config);
    let ts_shown = if ts == "string" {
        format!("`string` branded as `{symbol}`")
    } else {
        code_cell(&ts)
    };
    out.push_str(&format!(
        "| TypeScript | {ts_shown} | `parse{symbol}`, `normalize{symbol}`, `validate{symbol}`, `parse{symbol}Strict`, `normalize{symbol}Strict` |\n"
    ));
    out.push_str(&format!(
        "| Python | {} | `parse_{snake}`, `normalize_{snake}`, `validate_{snake}` |\n",
        code_cell(&type_mapping(def, "python"))
    ));
    out.push_str(&format!(
        "| Rust | {} | `Registry::scalar(ScalarId({}))` |\n",
        code_cell(&type_mapping(def, "rust")),
        def.id.as_u32()
    ));
    out
}

/// The index page: an intro and one table per namespace linking every page.
pub fn render_index(registry: &Registry, config: &Config) -> String {
    let link_prefix = config
        .docs
        .as_ref()
        .map(|docs| docs.link_prefix.trim_end_matches('/').to_string())
        .unwrap_or_default();
    let mut out = frontmatter(
        "Scalar reference",
        "One page per scalar, generated from the registry and the conformance vectors.",
        0,
    );
    out.push_str(&format!(
        "\nThe {} pages in this section are generated from the registry and the\n\
         conformance corpus; do not edit them. Each carries the scalar's canonical\n\
         name and id, its primitive, SQL and JSON Schema types, its pattern and\n\
         bounds, examples, accepted and rejected inputs from the corpus, its type\n\
         in each language and the generated function names. If a page is wrong,\n\
         fix the registry entry or the vectors and regenerate.\n",
        registry.len()
    ));

    let mut by_namespace: BTreeMap<&str, Vec<&ScalarDef>> = BTreeMap::new();
    for def in registry.defs() {
        by_namespace.entry(def.namespace).or_default().push(def);
    }
    for (namespace, defs) in by_namespace {
        out.push_str(&format!(
            "\n## {namespace}\n\n| Scalar | Id | Primitive | Description |\n| --- | --- | --- | --- |\n"
        ));
        for def in defs {
            out.push_str(&format!(
                "| [{}]({link_prefix}/{}/) | {} | {:?} | {} |\n",
                def.canonical,
                page_slug(def.canonical),
                def.id.as_u32(),
                def.primitive,
                text_cell(def.description.trim())
            ));
        }
    }
    out
}

#[derive(Debug, Default)]
pub struct DocsReport {
    /// Per-scalar pages written, index excluded.
    pub pages: usize,
    pub written: Vec<PathBuf>,
}

/// Write one page per scalar plus `index.md` into `out_dir`.
pub fn write(
    registry: &Registry,
    config: &Config,
    vectors: &Vectors,
    out_dir: &Path,
) -> Result<DocsReport, DocsError> {
    fs::create_dir_all(out_dir).map_err(|source| DocsError::Write {
        path: out_dir.to_path_buf(),
        source,
    })?;
    let mut report = DocsReport::default();
    let mut write_file = |path: PathBuf, content: String| -> Result<(), DocsError> {
        fs::write(&path, content).map_err(|source| DocsError::Write {
            path: path.clone(),
            source,
        })?;
        report.written.push(path);
        Ok(())
    };
    for (position, def) in registry.defs().enumerate() {
        let page = render_page(
            def,
            registry,
            config,
            vectors.for_scalar(def.canonical),
            position + 1,
        );
        write_file(
            out_dir.join(format!("{}.md", page_slug(def.canonical))),
            page,
        )?;
    }
    write_file(out_dir.join("index.md"), render_index(registry, config))?;
    report.pages = report.written.len() - 1;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugs_split_acronyms_and_lowercase() {
        assert_eq!(page_slug("Crypto.RSAPrivateKey"), "crypto-rsa-private-key");
        assert_eq!(page_slug("Contact.Email"), "contact-email");
        assert_eq!(page_slug("Generic.Int64"), "generic-int64");
    }

    #[test]
    fn code_cells_survive_pipes_newlines_and_backticks() {
        assert_eq!(code_cell(""), "(empty)");
        assert_eq!(code_cell("a|b"), "`a\\|b`");
        assert_eq!(code_cell("a\nb"), "`a\\nb`");
        assert_eq!(code_cell("a`b"), "``a`b``");
        assert_eq!(code_cell("`x"), "`` `x ``");
        let long = "x".repeat(MAX_CELL_CHARS + 5);
        assert!(code_cell(&long).ends_with(&format!("(cut; {} characters)", MAX_CELL_CHARS + 5)));
    }

    #[test]
    fn prose_escapes_markdown_outside_code_spans() {
        assert_eq!(
            escape_prose("projects/*/secrets/*"),
            "projects/\\*/secrets/\\*"
        );
        assert_eq!(
            escape_prose("append \"_<hexsuffix>\""),
            "append \"\\_\\<hexsuffix\\>\""
        );
        assert_eq!(
            escape_prose("see `q1_sales` or q1_sales"),
            "see `q1_sales` or q1\\_sales"
        );
        assert_eq!(text_cell("a|b\nc [x]"), "a\\|b c \\[x\\]");
    }

    #[test]
    fn frontmatter_quotes_the_description_as_yaml() {
        let fm = frontmatter("A.B", "says \"hi\": ok", 3);
        assert_eq!(
            fm,
            "---\ntitle: A.B\ndescription: \"says \\\"hi\\\": ok\"\nsidebar:\n  order: 3\n---\n"
        );
    }
}
