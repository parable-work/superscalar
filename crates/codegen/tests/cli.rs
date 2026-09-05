//! The CLI surface: the bare `--check` CI runs, the subcommands, and their
//! exit codes. Nothing here writes into the tree.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_superscalar"))
        .args(args)
        .output()
        .expect("run superscalar")
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("utf-8 stdout")
}

fn stderr(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("utf-8 stderr")
}

#[test]
fn bare_check_is_the_codegen_drift_gate() {
    let output = cli(&["--check"]);
    assert!(
        output.status.success(),
        "generated files drift: {}",
        stderr(&output)
    );
    assert!(stdout(&output).contains("codegen: 44 scalars"));

    let explicit = cli(&["codegen", "--check"]);
    assert!(explicit.status.success(), "{}", stderr(&explicit));
    assert_eq!(stdout(&explicit), stdout(&output));
}

#[test]
fn registry_dump_is_stable_across_runs() {
    let first = cli(&["registry", "dump"]);
    let second = cli(&["registry", "dump"]);
    assert!(first.status.success(), "{}", stderr(&first));
    assert_eq!(first.stdout, second.stdout);
    assert_eq!(
        stdout(&first),
        superscalar_codegen::dump_to_string(superscalar::Registry::builtin())
    );
    assert!(stdout(&first).starts_with("{\n  \"dump_version\": 1,\n"));
}

#[test]
fn docs_check_passes_and_docs_out_writes_pages() {
    let check = cli(&["docs", "--check"]);
    assert!(check.status.success(), "{}", stderr(&check));
    assert!(stdout(&check).contains("docs check: 44 scalars documented"));

    let out: PathBuf =
        std::env::temp_dir().join(format!("scalar-lib-cli-docs-{}", std::process::id()));
    let written = cli(&[
        "docs",
        "--out",
        out.to_str().expect("utf-8 path"),
        "--check",
    ]);
    assert!(written.status.success(), "{}", stderr(&written));
    assert!(stdout(&written).contains("docs: 44 pages plus index"));
    assert_eq!(fs::read_dir(&out).expect("out dir").count(), 45);
    fs::remove_dir_all(&out).expect("remove temp dir");
}

#[test]
fn bad_invocations_fail_with_usage() {
    let no_target = cli(&["docs"]);
    assert!(!no_target.status.success());
    assert!(stderr(&no_target).contains("docs needs --out DIR, --check, or both"));

    let unknown = cli(&["frobnicate"]);
    assert!(!unknown.status.success());
    assert!(stderr(&unknown).contains("unknown command frobnicate"));

    let stray_out = cli(&["--out", "/nowhere"]);
    assert!(!stray_out.status.success());
    assert!(stderr(&stray_out).contains("--out only applies to docs"));

    let missing_config = cli(&["--check", "--config", "/nowhere/superscalar.toml"]);
    assert!(!missing_config.status.success());
    assert!(stderr(&missing_config).contains("could not read"));

    let help = cli(&["--help"]);
    assert!(help.status.success());
    assert!(stdout(&help).contains("registry dump"));
}
