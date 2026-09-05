//! `superscalar.toml` parsing: unknown keys are rejected so a typo cannot
//! silently disable an emitter, relative paths resolve against the config
//! file's directory rather than the working directory, the CLI-only
//! `[registry]` section accepts nothing but `builtin`, and `[extension.*]`
//! tables are carried opaquely for an xtask to read.

mod common;

use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use superscalar_codegen::{Config, ConfigError};

fn checked_in_text() -> String {
    fs::read_to_string(common::config_path()).expect("read superscalar.toml")
}

#[test]
fn unknown_keys_are_rejected_at_every_level() {
    let root = PathBuf::from("/cfg");
    let top = format!(
        "{}\n[docz]\nvectors = []\nlink_prefix = \"/x\"\n",
        checked_in_text()
    );
    let err = Config::parse(&top, root.clone()).expect_err("unknown section");
    assert!(matches!(err, ConfigError::Parse { .. }), "{err}");
    assert!(err.to_string().contains("docz"), "{err}");

    let nested = checked_in_text().replace("gofmt = true", "gofmt = true\nenbled = false");
    let err = Config::parse(&nested, root.clone()).expect_err("unknown key in [go]");
    assert!(err.to_string().contains("enbled"), "{err}");

    let table_entry = checked_in_text().replace(
        "[typescript.type_renames]",
        "[[typescript.type_imports]]\nmodule = \"./platform\"\nnames = []\nfrom = \"x\"\n\n[typescript.type_renames]",
    );
    let err = Config::parse(&table_entry, root).expect_err("unknown key in type_imports");
    assert!(err.to_string().contains("from"), "{err}");
}

#[test]
fn registry_source_accepts_only_builtin() {
    let text = checked_in_text().replace(
        "source = \"builtin\"",
        "source = \"dump\"\ndump = \"target/registry.json\"",
    );
    let err = Config::parse(&text, PathBuf::from("/cfg")).expect_err("dump is not a CLI source");
    assert!(err.to_string().contains("dump"), "{err}");
}

#[test]
fn relative_paths_resolve_against_the_config_directory() {
    let root = PathBuf::from("/somewhere/else");
    let config = Config::parse(&checked_in_text(), root.clone()).expect("parse");
    assert_eq!(config.root, root);
    assert_eq!(
        config.resolve(Path::new("go/generated.go")),
        root.join("go/generated.go")
    );
    assert_eq!(
        config.resolve(Path::new("/abs/out.go")),
        PathBuf::from("/abs/out.go")
    );
    let go = common::emitter("go").output(&config).expect("go enabled");
    assert_eq!(go, root.join("bindings/go/generated.go"));

    // Config::load takes the root from the file's own directory, wherever
    // the process runs.
    let dir = std::env::temp_dir().join(format!("scalar-lib-config-{}", std::process::id()));
    fs::create_dir_all(&dir).expect("create temp dir");
    let path = dir.join("superscalar.toml");
    fs::write(&path, checked_in_text()).expect("write config copy");
    let loaded = Config::load(&path).expect("load");
    assert_eq!(loaded.root, dir);
    assert_eq!(
        common::emitter("python")
            .output(&loaded)
            .expect("python enabled"),
        dir.join("bindings/python/superscalar/_generated.py")
    );
    fs::remove_dir_all(&dir).expect("remove temp dir");
}

#[test]
fn go_output_without_inserts_has_no_extension_blocks() {
    // The built-in layout supplies no inserts, so the Go output carries only
    // what the template itself emits, with the scalar list typed as string.
    let config = common::config();
    let ctx = common::context(&config);
    assert!(ctx.inserts.is_empty());
    let go = common::rendered(&ctx, "go");
    assert!(!go.contains("VALID_PERMISSIONS"));
    assert!(go.contains("\n// VALID_SCALARS lists"));
    assert!(go.contains("var VALID_SCALARS = []string{"));
    assert!(!go.contains("var SlugPattern"));
}

#[test]
fn go_inserts_land_at_their_anchors_and_the_list_type_is_configurable() {
    let text = checked_in_text().replace(
        "gofmt = true",
        "gofmt = true\nscalar_list_type = \"ScalarName\"",
    );
    let config = Config::parse(&text, common::config().root).expect("parse");
    let mut ctx = common::context(&config);
    ctx.inserts.insert(
        superscalar_codegen::GO_AFTER_IDS.to_string(),
        "var AFTER_IDS = 1\n\n".to_string(),
    );
    ctx.inserts.insert(
        superscalar_codegen::GO_AFTER_PATTERNS.to_string(),
        "var AFTER_PATTERNS = 2\n\n".to_string(),
    );
    let go = common::rendered(&ctx, "go");
    let ids = go.find("var AFTER_IDS = 1\n").expect("after_ids insert");
    let list = go.find("// VALID_SCALARS lists").expect("scalar list");
    assert!(ids < list, "after_ids sits before the scalar list");
    assert!(go.contains("var VALID_SCALARS = []ScalarName{"));
    assert!(go.contains("\tScalarName(\"Contact.Email\"),"));
    let patterns = go
        .find("var AFTER_PATTERNS = 2\n")
        .expect("after_patterns insert");
    let errors = go
        .find("func validationErrorsFromError")
        .expect("error helper");
    assert!(
        patterns < errors,
        "after_patterns sits before the error helper"
    );
}

#[test]
fn extension_tables_are_opaque_to_the_library_and_typed_for_an_xtask() {
    #[derive(Deserialize, PartialEq, Debug)]
    #[serde(deny_unknown_fields)]
    struct Perms {
        source: PathBuf,
        outputs: Vec<String>,
    }
    let text = format!(
        "{}\n[extension.allowlist]\nsource = \"allowlist.yml\"\noutputs = [\"a\", \"b\"]\n\n[extension.other]\nanything = 1\n",
        checked_in_text()
    );
    let config = Config::parse(&text, PathBuf::from("/cfg")).expect("extension tables parse");
    let perms: Perms = config
        .extension_table("allowlist")
        .expect("well-formed")
        .expect("present");
    assert_eq!(
        perms,
        Perms {
            source: PathBuf::from("allowlist.yml"),
            outputs: vec!["a".to_string(), "b".to_string()],
        }
    );
    let absent: Option<Perms> = config.extension_table("missing").expect("absent is Ok");
    assert!(absent.is_none());
    let err = config
        .extension_table::<Perms>("other")
        .expect_err("wrong shape is a parse error");
    assert!(err.to_string().contains("[extension.other]"), "{err}");
    // The built-in config declares no extension tables.
    assert!(common::config().extension.is_empty());
}
