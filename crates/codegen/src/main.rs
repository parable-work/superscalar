//! The codegen CLI.
//!
//!   superscalar [codegen] [--check] [--config PATH]
//!   superscalar registry dump [--config PATH]
//!   superscalar docs [--out DIR] [--check] [--config PATH]
//!
//! No subcommand means `codegen`, so the bare invocation and the bare
//! `--check` that CI, the Makefile and the Dockerfiles run keep working.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use superscalar::Registry;
use superscalar_codegen::config::RegistrySource;
use superscalar_codegen::{builtin_emitters, docs, dump_to_string, Config, Context, Emitter, Mode};

const USAGE: &str = "usage:
  superscalar [codegen] [--check] [--config PATH]
  superscalar registry dump [--config PATH]
  superscalar docs [--out DIR] [--check] [--config PATH]

  --config PATH  superscalar.toml to read (default: the one at the workspace root)
  --check        codegen: report drift and exit 1 instead of writing
                 docs: exit 1 when a scalar has no vectors or no description
  --out DIR      docs: directory to write one Markdown page per scalar plus index.md";

enum Command {
    Codegen { check: bool },
    RegistryDump,
    Docs { out: Option<PathBuf>, check: bool },
    Help,
}

struct Args {
    command: Command,
    config: PathBuf,
}

fn default_config_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .join("superscalar.toml")
}

fn parse_args(args: &[String]) -> Result<Args, String> {
    let mut config = default_config_path();
    let mut check = false;
    let mut out: Option<PathBuf> = None;
    let mut positional: Vec<&str> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--check" => check = true,
            "--config" => {
                i += 1;
                config = PathBuf::from(
                    args.get(i)
                        .ok_or_else(|| "--config needs a path".to_string())?,
                );
            }
            "--out" => {
                i += 1;
                out = Some(PathBuf::from(
                    args.get(i)
                        .ok_or_else(|| "--out needs a directory".to_string())?,
                ));
            }
            "--help" | "-h" | "help" => {
                return Ok(Args {
                    command: Command::Help,
                    config,
                })
            }
            flag if flag.starts_with('-') => return Err(format!("unknown flag {flag}")),
            word => positional.push(word),
        }
        i += 1;
    }
    if out.is_some() && positional.as_slice() != ["docs"] {
        return Err("--out only applies to docs".to_string());
    }
    let command = match positional.as_slice() {
        [] | ["codegen"] => Command::Codegen { check },
        ["registry", "dump"] => Command::RegistryDump,
        ["docs"] => Command::Docs { out, check },
        other => return Err(format!("unknown command {}", other.join(" "))),
    };
    Ok(Args { command, config })
}

fn load_registry(config: &Config) -> &'static Registry {
    match config.registry.source {
        RegistrySource::Builtin => Registry::builtin(),
    }
}

fn codegen(config: &Config, registry: &Registry, check: bool) -> Result<ExitCode, String> {
    let ctx = Context::new(registry, config);
    let emitters = builtin_emitters();
    let refs: Vec<&dyn Emitter> = emitters.iter().map(|e| e.as_ref()).collect();
    let mode = if check { Mode::Check } else { Mode::Write };
    let report = superscalar_codegen::run(&refs, &ctx, mode).map_err(|err| err.to_string())?;
    for path in &report.written {
        println!("wrote {}", path.display());
    }
    for path in &report.skipped {
        eprintln!(
            "note: skipping {} drift check (gofmt not found)",
            path.display()
        );
    }
    for path in &report.drifted {
        eprintln!("DRIFT: {} is out of date", path.display());
    }
    if check && !report.drifted.is_empty() {
        eprintln!("Run: cargo run -p superscalar-codegen");
        return Ok(ExitCode::FAILURE);
    }
    println!("codegen: {} scalars", ctx.entries.len());
    Ok(ExitCode::SUCCESS)
}

fn docs_command(
    config: &Config,
    registry: &Registry,
    out: Option<&Path>,
    check: bool,
) -> Result<ExitCode, String> {
    if out.is_none() && !check {
        return Err("docs needs --out DIR, --check, or both".to_string());
    }
    let vectors = docs::load_vectors(config).map_err(|err| err.to_string())?;
    if let Some(out_dir) = out {
        let report =
            docs::write(registry, config, &vectors, out_dir).map_err(|err| err.to_string())?;
        println!(
            "docs: {} pages plus index in {}",
            report.pages,
            out_dir.display()
        );
    }
    if check {
        let problems = docs::check(registry, &vectors);
        if !problems.is_empty() {
            for problem in &problems {
                eprintln!("docs check: {problem}");
            }
            return Ok(ExitCode::FAILURE);
        }
        println!("docs check: {} scalars documented", registry.len());
    }
    Ok(ExitCode::SUCCESS)
}

fn run(args: Args) -> Result<ExitCode, String> {
    if matches!(args.command, Command::Help) {
        println!("{USAGE}");
        return Ok(ExitCode::SUCCESS);
    }
    let config = Config::load(&args.config).map_err(|err| err.to_string())?;
    let registry = load_registry(&config);
    match args.command {
        Command::Codegen { check } => codegen(&config, registry, check),
        Command::RegistryDump => {
            print!("{}", dump_to_string(registry));
            Ok(ExitCode::SUCCESS)
        }
        Command::Docs { out, check } => docs_command(&config, registry, out.as_deref(), check),
        Command::Help => unreachable!("handled above"),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let parsed = match parse_args(&args) {
        Ok(parsed) => parsed,
        Err(message) => {
            eprintln!("error: {message}\n{USAGE}");
            return ExitCode::FAILURE;
        }
    };
    match run(parsed) {
        Ok(code) => code,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}
