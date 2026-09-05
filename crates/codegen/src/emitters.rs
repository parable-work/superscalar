//! The built-in emitters: Go, TypeScript, Python and the Rust metadata table.
//! An extension adds its own by implementing `Emitter` in its xtask and
//! passing them to `run` beside these.

use crate::config::{Config, TypeImport};
use crate::{
    gofmt, optional_str_literal, render, Context, Emitter, Entry, Postprocessed, RenderError,
    GO_AFTER_IDS, GO_AFTER_PATTERNS,
};
use serde::Serialize;
use std::path::PathBuf;
use superscalar::{LegacyAlias, Registry};

/// Go, TypeScript, Python and the Rust metadata table. Each reads its own
/// `[section]` of the config and is disabled when the section is absent or
/// has `enabled = false`.
pub fn builtin_emitters() -> Vec<Box<dyn Emitter>> {
    vec![
        Box::new(Python),
        Box::new(TypeScript),
        Box::new(RustMetadata),
        Box::new(Go),
    ]
}

#[derive(Serialize)]
struct GoView<'a> {
    package: &'a str,
    scalar_list_type: &'a str,
    /// Extension-rendered text for the two anchors, empty when unset.
    after_ids: &'a str,
    after_patterns: &'a str,
}

#[derive(Serialize)]
struct TypeScriptView<'a> {
    backend_module: &'a str,
    validation_module: &'a str,
    type_imports: &'a [TypeImport],
    /// Every name across `type_imports`; a legacy alias with one of these
    /// names skips its `export type` line.
    imported_type_names: Vec<&'a str>,
}

#[derive(Serialize)]
struct PythonView<'a> {
    package: &'a str,
    native_from: &'a str,
    native_name: &'a str,
}

/// The shared template context for the three binding emitters.
#[derive(Serialize)]
struct EntriesView<'a> {
    gen_note: &'a str,
    entries: &'a [Entry],
    metadata_entries: Vec<&'a Entry>,
    legacy_aliases: &'a [LegacyAlias],
    go: Option<GoView<'a>>,
    typescript: Option<TypeScriptView<'a>>,
    python: Option<PythonView<'a>>,
}

impl<'a> EntriesView<'a> {
    fn new(ctx: &'a Context) -> EntriesView<'a> {
        EntriesView {
            gen_note: ctx.gen_note,
            entries: &ctx.entries,
            metadata_entries: ctx.metadata_entries(),
            legacy_aliases: ctx.legacy_aliases,
            go: None,
            typescript: None,
            python: None,
        }
    }
}

/// `._native` -> (".", "_native"); `pkg._native` -> ("pkg", "_native").
fn split_native_module(module: &str) -> Result<(&str, &str), RenderError> {
    let Some(dot) = module.rfind('.') else {
        return Err(RenderError(format!(
            "[python] native_module {module:?} must be a dotted path such as \"._native\""
        )));
    };
    let from = if dot == 0 { "." } else { &module[..dot] };
    Ok((from, &module[dot + 1..]))
}

pub struct Python;

impl Emitter for Python {
    fn name(&self) -> &str {
        "python"
    }
    fn output(&self, config: &Config) -> Option<PathBuf> {
        let py = config.python.as_ref().filter(|py| py.enabled)?;
        Some(config.resolve(&py.out))
    }
    fn render(&self, ctx: &Context) -> Result<String, RenderError> {
        let py = ctx
            .config
            .python
            .as_ref()
            .ok_or_else(|| RenderError("[python] section missing".to_string()))?;
        let (native_from, native_name) = split_native_module(&py.native_module)?;
        let mut view = EntriesView::new(ctx);
        view.python = Some(PythonView {
            package: &py.package,
            native_from,
            native_name,
        });
        render(
            "_generated.py.jinja",
            include_str!("../templates/_generated.py.jinja"),
            &view,
        )
    }
}

pub struct TypeScript;

impl Emitter for TypeScript {
    fn name(&self) -> &str {
        "typescript"
    }
    fn output(&self, config: &Config) -> Option<PathBuf> {
        let ts = config.typescript.as_ref().filter(|ts| ts.enabled)?;
        Some(config.resolve(&ts.out))
    }
    fn render(&self, ctx: &Context) -> Result<String, RenderError> {
        let ts = ctx
            .config
            .typescript
            .as_ref()
            .ok_or_else(|| RenderError("[typescript] section missing".to_string()))?;
        let mut view = EntriesView::new(ctx);
        view.typescript = Some(TypeScriptView {
            backend_module: &ts.backend_module,
            validation_module: &ts.validation_module,
            type_imports: &ts.type_imports,
            imported_type_names: ts
                .type_imports
                .iter()
                .flat_map(|imp| imp.names.iter().map(String::as_str))
                .collect(),
        });
        render(
            "generated.ts.jinja",
            include_str!("../templates/generated.ts.jinja"),
            &view,
        )
    }
}

pub struct Go;

impl Emitter for Go {
    fn name(&self) -> &str {
        "go"
    }
    fn output(&self, config: &Config) -> Option<PathBuf> {
        let go = config.go.as_ref().filter(|go| go.enabled)?;
        Some(config.resolve(&go.out))
    }
    fn render(&self, ctx: &Context) -> Result<String, RenderError> {
        let go = ctx
            .config
            .go
            .as_ref()
            .ok_or_else(|| RenderError("[go] section missing".to_string()))?;
        let insert = |anchor: &str| ctx.inserts.get(anchor).map_or("", String::as_str);
        let mut view = EntriesView::new(ctx);
        view.go = Some(GoView {
            package: &go.package,
            scalar_list_type: &go.scalar_list_type,
            after_ids: insert(GO_AFTER_IDS),
            after_patterns: insert(GO_AFTER_PATTERNS),
        });
        render(
            "generated.go.jinja",
            include_str!("../templates/generated.go.jinja"),
            &view,
        )
    }
    /// gofmt when `[go] gofmt = true`. An absent gofmt leaves the text
    /// unverified; a failing one is an error, never unformatted output.
    fn postprocess(&self, rendered: String, config: &Config) -> Result<Postprocessed, RenderError> {
        if !config.go.as_ref().is_some_and(|go| go.gofmt) {
            return Ok(Postprocessed::Ready(rendered));
        }
        match gofmt(&rendered) {
            Ok(Some(formatted)) => Ok(Postprocessed::Ready(formatted)),
            Ok(None) => Ok(Postprocessed::Unverified(rendered)),
            Err(message) => Err(RenderError(message)),
        }
    }
}

#[derive(Serialize)]
pub(crate) struct MetadataEntry {
    pub(crate) canonical_literal: String,
    pub(crate) primitive_literal: String,
    pub(crate) sql_type_literal: String,
    pub(crate) json_schema_type_literal: String,
    pub(crate) max_length_literal: String,
    pub(crate) min_length_literal: String,
    pub(crate) pattern_literal: String,
    /// Rust `Option` form: `None` or `Some("...")`.
    pub(crate) comparability_class_literal: String,
    pub(crate) is_sortable: bool,
}

#[derive(Serialize)]
pub(crate) struct MetadataView<'a> {
    pub(crate) gen_note: &'a str,
    pub(crate) entries: Vec<MetadataEntry>,
}

pub(crate) fn metadata_entries(registry: &Registry) -> Vec<MetadataEntry> {
    registry
        .defs()
        .filter(|def| !def.metadata_omit)
        .map(|def| MetadataEntry {
            canonical_literal: format!("{:?}", def.canonical),
            primitive_literal: format!("{:?}", def.metadata_primitive),
            sql_type_literal: format!("{:?}", def.sql_type),
            json_schema_type_literal: optional_str_literal(
                Some(def.json_schema_type).filter(|s| !s.is_empty()),
            ),
            max_length_literal: crate::optional_usize_literal(def.max_length),
            min_length_literal: crate::optional_usize_literal(def.min_length),
            pattern_literal: optional_str_literal(def.pattern),
            comparability_class_literal: optional_str_literal(def.comparability_class),
            is_sortable: def.is_sortable(),
        })
        .collect()
}

pub(crate) const METADATA_TEMPLATE: &str = include_str!("../templates/scalar_metadata.rs.jinja");

pub struct RustMetadata;

impl Emitter for RustMetadata {
    fn name(&self) -> &str {
        "rust_metadata"
    }
    fn output(&self, config: &Config) -> Option<PathBuf> {
        let meta = config.rust_metadata.as_ref().filter(|meta| meta.enabled)?;
        Some(config.resolve(&meta.out))
    }
    fn render(&self, ctx: &Context) -> Result<String, RenderError> {
        render(
            "scalar_metadata.rs.jinja",
            METADATA_TEMPLATE,
            &MetadataView {
                gen_note: ctx.gen_note,
                entries: metadata_entries(ctx.registry),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The emit test's count assertions would also pass on a template that
    /// hardcoded `comparability_class: None`. Render the Rust metadata template
    /// with one doctored row so the wiring is proven independently of what the
    /// catalog happens to contain.
    ///
    /// The doctored literal is a string the catalog CANNOT produce, so the
    /// assertion below stays about template wiring and never about the
    /// catalog's contents. It was `"temporal_instant"` until that shipped as a
    /// real class name, at which point the render carried the doctored
    /// occurrence plus two real ones and the count assertion broke. Raising the
    /// expected count instead would have turned a wiring test into a
    /// hand-maintained mirror of the catalog's classed-row count.
    #[test]
    fn a_named_class_reaches_the_rust_metadata_table() {
        // Recurrence detector: render WITHOUT doctoring and require the
        // sentinel to be absent, so a sentinel the catalog can produce fails
        // here instead of silently weakening the assertion below.
        let undoctored = render(
            "scalar_metadata.rs.jinja",
            METADATA_TEMPLATE,
            &MetadataView {
                gen_note: "@generated; do not edit",
                entries: metadata_entries(Registry::builtin()),
            },
        )
        .expect("render");
        assert!(
            !undoctored.contains("never_a_real_class"),
            "the undoctored render carries the sentinel, so it is a class the \
             catalog can produce and the count assertion below no longer proves \
             the field travels; pick one the catalog cannot produce"
        );

        let mut entries = metadata_entries(Registry::builtin());
        entries[0].comparability_class_literal = optional_str_literal(Some("never_a_real_class"));
        let rendered = render(
            "scalar_metadata.rs.jinja",
            METADATA_TEMPLATE,
            &MetadataView {
                gen_note: "@generated; do not edit",
                entries,
            },
        )
        .expect("render");
        assert_eq!(
            rendered
                .matches("        comparability_class: Some(\"never_a_real_class\"),")
                .count(),
            1
        );
    }

    #[test]
    fn native_module_splits_relative_and_absolute_forms() {
        assert_eq!(split_native_module("._native").unwrap(), (".", "_native"));
        assert_eq!(
            split_native_module("superscalar._native").unwrap(),
            ("superscalar", "_native")
        );
        assert!(split_native_module("_native").is_err());
    }
}
