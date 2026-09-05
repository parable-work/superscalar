//! Byte identity: every generated file in the tree is exactly what the
//! checked-in `superscalar.toml` produces. This is the acceptance test for
//! moving the template literals into config, and the same comparison
//! `scalar-codegen --check` runs.

mod common;

use superscalar_codegen::{run, Emitter, Mode};

#[test]
fn every_generated_file_matches_superscalar_toml() {
    let config = common::config();
    let ctx = common::context(&config);
    let emitters = common::all_emitters();
    let refs: Vec<&dyn Emitter> = emitters.iter().map(|e| e.as_ref()).collect();

    let report = run(&refs, &ctx, Mode::Check).expect("check run");

    assert!(
        report.drifted.is_empty(),
        "generated files drift from superscalar.toml; run `cargo run -p superscalar-codegen`: {:?}",
        report.drifted
    );
    assert!(report.written.is_empty(), "check mode must not write");
    assert!(
        report.disabled.is_empty(),
        "the in-repo config enables every emitter: {:?}",
        report.disabled
    );
    // gofmt is the only formatter; without it the Go file is skipped, not
    // compared, and CI (which installs Go) covers it.
    assert!(
        report.skipped.len() <= 1,
        "only the Go output may be skipped: {:?}",
        report.skipped
    );
}

#[test]
fn config_names_every_output_the_cli_wrote_before_it_was_configurable() {
    let config = common::config();
    let mut outputs: Vec<String> = common::all_emitters()
        .iter()
        .filter_map(|emitter| emitter.output(&config))
        .map(|path| {
            path.strip_prefix(&config.root)
                .expect("under the config root")
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    outputs.sort();
    assert_eq!(
        outputs,
        [
            "bindings/go/generated.go",
            "bindings/python/superscalar/_generated.py",
            "bindings/typescript/src/generated.ts",
            "crates/core/src/scalar_metadata.rs",
        ]
    );
}

#[test]
fn a_disabled_section_drops_its_emitter() {
    let text = std::fs::read_to_string(common::config_path()).expect("read config");
    let text = text.replace("[rust_metadata]\n", "[rust_metadata]\nenabled = false\n");
    let config =
        superscalar_codegen::Config::parse(text.as_str(), common::config().root).expect("parse");
    assert!(common::emitter("rust_metadata").output(&config).is_none());
    assert!(common::emitter("go").output(&config).is_some());
}
