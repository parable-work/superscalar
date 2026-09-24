---
title: Build an extension
description: Add your own scalars in one Rust crate, build C, Node, Python and WASM bindings for them, and generate typed wrappers, walking through the acme-scalars example.
sidebar:
  order: 2
---

An extension is a Rust crate that adds scalars to the registry without
forking the core. You declare definitions and implementations, assemble a
registry that holds the 48 built-ins plus yours, and apply four one-line
macros to produce native bindings that carry the whole assembled catalog.
The code generator then emits Go, Python and TypeScript wrappers for your
scalars from a configuration file you own.

This guide walks through `examples/acme-scalars/`, a complete extension that
CI builds through all four bindings on every pull request. Where the guide
and the example disagree, the example is right. The example covers steps 1
to 3 and 5. It stops at the native bindings and does not generate typed
wrappers; step 4 describes the configuration and the small binary a real
extension adds for that.

## What the example contains

```
examples/acme-scalars/
  Cargo.toml                    its own workspace: ext, ffi, napi, python, wasm
  README.md                     what the example proves, its layout, how to run it
  ext/src/lib.rs                the extension: two definitions, one impl, registry(), definitions()
  ext/examples/dump.rs          prints the assembled registry as JSON
  ext/tests/conformance.rs      both vector files through the assembled registry
  ext/tests/definitions.rs      definitions() answers every lookup as registry() does
  ffi/src/lib.rs                superscalar_ffi::export_c_abi!(acme_scalars::registry);
  ffi/tests/c_consumer.c        a C program linked against the static archive
  napi/src/lib.rs               superscalar_napi::export_napi!(acme_scalars::registry);
  python/src/lib.rs             superscalar_python::export_pymodule!(_native, acme_scalars::registry);
  wasm/src/lib.rs               superscalar_wasm::export_wasm!(acme_scalars::registry);
  conformance/acme-scalars.v2.json   vectors for the two scalars
  scripts/smoke.sh              builds the four bindings and runs both vector files through each
  scripts/run_vectors.py        vector runner for the C and Python bindings
  scripts/run_vectors.cjs       vector runner for the napi and wasm bindings
  scripts/check_third_scalar.sh adds a third scalar (third_scalar.patch) and checks the diff
```

`python/` is the PyO3 module crate, not a Python package. The example has no
`superscalar.toml`, no codegen binary and no generated Go, Python or
TypeScript package. The runners load each build output directly: the static
archive through the C program, the napi cdylib as a `.node` file, the PyO3
cdylib as `_native`, and the wasm-pack bundle. They look up scalar ids by
canonical name in the registry dump, which stands in for the tables generated
wrappers carry. The example's `README.md` says how a published extension
would package the same crates with `napi build`, `maturin` and `wasm-pack`.

The two scalars are chosen to cover both kinds of rule. `Acme.OrderNumber`
(id 4096) is a directive scalar with pattern `^ORD-[0-9]{6}$` and no
implementation. `Acme.ScalarRef` (id 4097) is a deep scalar whose `validate`
accepts any canonical scalar name present in the assembled registry, so its
vectors pass only if the binding really dispatches into the assembled
registry and not the built-in one.

## 1. Pick an id block

Built-in ids own `0..=4095`. An extension declares an `id_base` that is a
multiple of 4096 and at least 4096, and its ids live in
`[id_base, id_base + 4096)`. The example uses 4096
(`ScalarId::EXTENSION_BLOCK`), the first extension block. Once published,
your ids and canonical names are frozen under the same rule as the built-ins.

Two extensions with the same base cannot be assembled together, so if your
extension is meant to be combined with others, pick a base they do not use;
the first block is the one most likely to be taken. There is no central
allocation of blocks; this is a coordination problem the design leaves to
extension authors.

## 2. Declare the extension

`ext/src/lib.rs`, in outline:

```rust
use std::sync::LazyLock;
use superscalar::{
    ErrorKind, Extension, Registry, Scalar, ScalarDef, ScalarError, ScalarId, ScalarTag,
};

pub const NAME: &str = "acme";
pub const ID_BASE: u32 = ScalarId::EXTENSION_BLOCK; // 4096
pub const ORDER_NUMBER: ScalarId = ScalarId(ID_BASE);
pub const SCALAR_REF: ScalarId = ScalarId(ID_BASE + 1);

pub static DEFS: [ScalarDef; 2] = [
    ScalarDef { id: ORDER_NUMBER, namespace: "Acme", canonical: "Acme.OrderNumber",
                tag: ScalarTag::PatternOnly, pattern: Some("^ORD-[0-9]{6}$"),
                /* other fields ... */ },
    ScalarDef { id: SCALAR_REF, namespace: "Acme", canonical: "Acme.ScalarRef",
                tag: ScalarTag::CustomLogic, /* other fields ... */ },
];

pub struct ScalarRef;
impl Scalar for ScalarRef {
    fn id(&self) -> ScalarId { SCALAR_REF }
    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        if input.trim().is_empty() {
            return Err(ScalarError::new(ErrorKind::Empty, "empty scalar name"));
        }
        if registry.by_canonical(input).is_some() { return Ok(()); }
        Err(ScalarError::new(ErrorKind::Enum, format!("unknown scalar: {input}")))
    }
    fn parse(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.validate(registry, input).map(|_| input.to_string())
    }
    fn normalize(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.parse(registry, input)
    }
}

pub struct AcmeExtension;
impl Extension for AcmeExtension {
    fn name(&self) -> &'static str { NAME }
    fn id_base(&self) -> u32 { ID_BASE }
    fn defs(&self) -> &'static [ScalarDef] { &DEFS }
    fn impls(&self) -> Vec<(ScalarId, Box<dyn Scalar>)> {
        vec![(SCALAR_REF, Box::new(ScalarRef))]
    }
}

pub fn registry() -> &'static Registry {
    static REGISTRY: LazyLock<Registry> = LazyLock::new(|| Registry::assemble(&[&AcmeExtension]));
    &REGISTRY
}
```

`Registry::assemble` runs thirteen checks and panics with both owners named
on any conflict: an unaligned or reserved `id_base`, an id outside the block,
a duplicate id or canonical name across built-ins and extensions, a
namespace that does not match the canonical prefix, a dangling or chained
alias, an implementation registered for a definition the extension does not
own, a `CustomLogic` definition with no implementation, a `PatternOnly`
definition with one, or a pattern that does not compile. A non-panicking
`try_assemble` exists for tests. `Acme.OrderNumber` has no implementation and
gets the generic directive validator built from its pattern.

A consumer that only reads definitions (a def by id or name, alias
resolution, comparability) should not link the implementations at all. Give
it `Definitions`, assembled from the same static slice and never through the
`Extension` trait, whose `impls()` would pull every implementation in:

```rust
pub fn definitions() -> &'static Definitions {
    static DEFINITIONS: LazyLock<Definitions> =
        LazyLock::new(|| Definitions::assemble(&[(NAME, &DEFS)]));
    &DEFINITIONS
}
```

It answers every definition lookup exactly as the registry does; the example
pins that in `ext/tests/definitions.rs`.

All three `Scalar` hooks take the `&Registry`, because a `parse` that
delegates to `validate` needs it. `Extension::name()` is the owner string
that assembly errors and the registry dump report, so `Definitions` is
assembled under the same name.

## 3. Assemble the bindings

Each binding is a one-file `cdylib` crate that applies a macro to your
registry function. The macros expand to the same `extern "C"`, napi, PyO3
and wasm-bindgen entry points the built-in packages use, over your assembled
registry instead of `Registry::builtin()`:

```rust
// ffi/src/lib.rs        (crate-type staticlib + cdylib; what Go links)
superscalar_ffi::export_c_abi!(acme_scalars::registry);
// napi/src/lib.rs       (keeps a build.rs calling napi_build::setup())
superscalar_napi::export_napi!(acme_scalars::registry);
// python/src/lib.rs     (_native must match module-name in pyproject.toml)
superscalar_python::export_pymodule!(_native, acme_scalars::registry);
// wasm/src/lib.rs
superscalar_wasm::export_wasm!(acme_scalars::registry);
```

The result is one native artifact per process that carries the whole
assembled catalog: your Go program links one static archive that answers for
`Contact.Email` and `Acme.OrderNumber` alike. The C ABI is unchanged from the
built-in packages (a `u32` id and a UTF-8 string in, a `ScalarResult` out),
so the same nine entry points exist and the built-in header applies.
`napi`, `pyo3` and `wasm-bindgen` stay direct dependencies of your binding
crates because the macros expand to items carrying those frameworks'
attributes. They must be the same minor versions the superscalar binding
crates use (pyo3 0.29, napi 3, wasm-bindgen 0.2): PyO3 links the Python
library, and Cargo refuses a graph with two pyo3 minors in it. Each macro
may be invoked once per cdylib. A PyO3 module can add
extra functions with `extra = [f, g]`; the other three frameworks export
per-function, so you add extra functions by writing them beside the macro
call.

The C header is the generic `superscalar.h` that `superscalar-ffi` ships
beside its manifest (`crates/ffi/superscalar.h` here; the Go module carries
a copy as `go/include/superscalar.h`). The example's C consumer compiles
against it unchanged. An extension that exports extra C functions writes its
own header that includes it and declares them by hand.

## 4. Configure codegen

The example stops at step 3: its runners call the raw exports with numeric
ids. A real extension ships typed packages, with `ParseAcmeOrderNumber` in Go
next to the built-in `ParseContactEmail`, and those come from the code
generator. Nothing in this step is in the example; it is what you add next.

Codegen reads a `superscalar.toml` that names the package and output path for
each language. Relative paths resolve against the file's directory. For the
acme workspace it would be:

```toml
[package]
name = "acme-scalars"

[registry]
# The only accepted value. The xtask below passes the assembled registry in
# code and ignores this section.
source = "builtin"

[go]
out = "go/generated.go"
package = "acmescalars"
module = "example.com/acme-scalars/go"
header = "go/include/superscalar.h"
cgo_library = "acme_scalars_ffi"

[typescript]
out = "typescript/src/generated.ts"
package = "acme-scalars"
backend_module = "./backend"
validation_module = "./validation"
json_value_module = "./json-value"

[python]
out = "python/acme_scalars/_generated.py"
package = "acme_scalars"
native_module = "._native"
```

An emitter runs when its section is present; `enabled = false` turns one off
without deleting it. This repository's own config also has `[rust_metadata]`
and `[docs]`, for the core crate's metadata table and the reference pages; an
extension leaves them out. In `[go]`, `module`, `header` and `cgo_library`
are informational: the generated file does not use them, but they should
match your hand-written cgo file.

The generated TypeScript file imports three hand-written modules that your
TypeScript package provides beside it: the backend that loads your native
addon or wasm build (`backend_module`), the validation result types
(`validation_module`), and the `JSONValue` type with its `isJSONValue` guard
(`json_value_module`, default `./json-value`), which the wrappers of
`Generic.JSON` use. Every assembled registry carries that built-in, so the
import is always emitted. `bindings/typescript/src/` in this repository has
one of each to start from. Point `json_value_module` at your own module, not
at the `superscalar` package root: that entry loads superscalar's own
backend, and your package would then load two native addons.

The Go and Python outputs sit beside hand-written code too. The generated Go
file calls `callScalarParse` and its siblings, which `go/binding.go` in this
repository defines over cgo; the generated Python file imports the compiled
module that `native_module` names.

The CLI can only see the built-in registry, because extensions are compiled
in. An extension therefore runs codegen from its own small binary, an
`xtask` crate in its workspace that depends on the extension crate and on
`superscalar-codegen` and calls the code generator library over the
assembled registry:

```rust
// xtask/src/main.rs
use std::path::Path;
use std::process::ExitCode;
use superscalar_codegen::{builtin_emitters, run, Config, Context, Emitter, Mode};

fn main() -> ExitCode {
    let check = std::env::args().any(|arg| arg == "--check");
    let mode = if check { Mode::Check } else { Mode::Write };
    // superscalar.toml sits at the workspace root, one level above this crate.
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../superscalar.toml");
    let config = Config::load(&path).expect("superscalar.toml loads");
    let ctx = Context::new(acme_scalars::registry(), &config);
    let emitters = builtin_emitters();
    let refs: Vec<&dyn Emitter> = emitters.iter().map(|e| e.as_ref()).collect();
    let report = run(&refs, &ctx, mode).expect("codegen runs");
    for path in &report.drifted {
        eprintln!("out of date: {}", path.display());
    }
    if report.drifted.is_empty() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
```

`cargo run -p acme-xtask` writes the wrappers; `cargo run -p acme-xtask --
--check` exits non-zero if a committed file differs. Run the check in CI as
the drift gate, as `make codegen-check` does for this repository's packages.
Go output goes through `gofmt`: without it on the path the Go file is written
unformatted and skipped in check mode, which `report.skipped` lists. If other
tooling reads the registry, `superscalar_codegen::dump_to_string` renders it
as JSON, as the example's `dump` binary does. An extension that needs an
emitter the library does not ship (a project-specific table, say) implements
the `Emitter` trait and pushes it onto the same list. The generated wrappers
include the built-in scalars as well as yours, so your Go package has
`ParseContactEmail` next to `ParseAcmeOrderNumber`.

## 5. Write vectors and run conformance

`conformance/acme-scalars.v2.json` has the same shape as the built-in
corpus. `Acme.ScalarRef` accepts `"Contact.Email"` and `"Acme.OrderNumber"`
and rejects `"Nope.Nope"` and `"contact.email"`. The example runs both the
built-in file and its own at two levels, so the assembly proves that the 48
built-ins still behave and that its scalars behave:

- In Rust, `ext/tests/conformance.rs` runs every vector through
  `registry()`.
- Through each binding, `scripts/smoke.sh` builds the artifact and hands it
  to a runner with both files: `run_vectors.py` for the C program (one
  process per vector) and the PyO3 module, `run_vectors.cjs` for the napi
  addon and the wasm bundle. The runners map names to ids through the
  registry dump, and fail if an assembled scalar has no vectors or a vector
  names a scalar the assembly does not know.

See [conformance](/superscalar/guides/conformance/) for how vector files
merge.

The `acme-example` CI job runs `scripts/smoke.sh` (`make acme` locally):
`cargo fmt --check`, `cargo clippy`, `cargo test`, the registry dump, then
the C, napi, wasm and Python bindings, each over both vector files. It then
runs `scripts/check_third_scalar.sh` (`make acme-third-scalar`), which
applies a patch adding `Acme.Sku` (id 4098) with its vectors, reruns the
smoke, and fails if any file outside `examples/acme-scalars/` changed. That
check is the acceptance test for the whole model: a scalar was added without
touching any file under `crates/`.

Once you generate typed packages as in step 4, give each one a conformance
test over both vector files, as the built-in packages have, and run the
xtask in `--check` mode in the same job.

## Combining with other extensions

`Registry::assemble` takes a slice, so a downstream can assemble several
extensions into one registry as long as their id blocks and canonical names
do not collide. The generated wrappers and the native artifact then carry all
of them. Nothing in the design lets two separately built native artifacts be
merged at runtime; assembly happens in Rust, at build time.
