//! Shared fixtures: the checked-in `superscalar.toml`, a `Context` over the
//! built-in registry, and rendering by emitter name.

#![allow(dead_code)]

use std::path::{Path, PathBuf};
use superscalar::Registry;
use superscalar_codegen::{builtin_emitters, Config, Context, Emitter};

pub fn config_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .join("superscalar.toml")
}

pub fn config() -> Config {
    Config::load(&config_path()).expect("load superscalar.toml")
}

pub fn context(config: &Config) -> Context<'_> {
    Context::new(Registry::builtin(), config)
}

pub fn all_emitters() -> Vec<Box<dyn Emitter>> {
    builtin_emitters()
}

pub fn emitter(name: &str) -> Box<dyn Emitter> {
    all_emitters()
        .into_iter()
        .find(|emitter| emitter.name() == name)
        .unwrap_or_else(|| panic!("no emitter named {name}"))
}

/// The rendered (not postprocessed) text of one emitter.
pub fn rendered(ctx: &Context, name: &str) -> String {
    emitter(name)
        .render(ctx)
        .unwrap_or_else(|err| panic!("render {name}: {err}"))
}
