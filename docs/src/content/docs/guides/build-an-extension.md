---
title: Build an extension
description: Add your own scalars in one Rust crate and get Go, Python, TypeScript and WASM bindings for them, walking through the acme-scalars example.
sidebar:
  order: 2
---

An extension is a Rust crate that adds scalars to the registry without
forking the core. You declare definitions and implementations, assemble a
registry that holds the 44 built-ins plus yours, and apply four one-line
macros to produce native bindings that carry the whole assembled catalog.
The code generator then emits Go, Python and TypeScript wrappers for your
scalars from a configuration file you own.

The extension work adds a complete example at `examples/acme-scalars/`, and
this guide walks through it as designed. The extension model is agreed and
being implemented; where a detail is still open it is marked below, and once
the example is in the repository it is the authority when the two disagree.

## What the example contains

```
examples/acme-scalars/
  Cargo.toml                    workspace: ext, ffi, napi, python, wasm, xtask
  superscalar.toml              codegen configuration for the three languages
  ext/src/lib.rs                the extension: two definitions, one impl, a registry
  ffi/src/lib.rs                superscalar_ffi::export_c_abi!(acme_scalars::registry);
  napi/src/lib.rs               superscalar_napi::export_napi!(acme_scalars::registry);
  python/src/lib.rs             superscalar_python::export_pymodule!(_native, acme_scalars::registry);
  wasm/src/lib.rs               superscalar_wasm::export_wasm!(acme_scalars::registry);
  xtask/src/main.rs             dumps the registry and runs the code generator
  conformance/acme-scalars.v2.json   vectors for the two scalars
  go/  typescript/  python/     generated output plus the conformance runner
```

The two scalars are chosen to cover both kinds of rule. `Acme.OrderNumber`
(id 8192) is a directive scalar with pattern `^ORD-[0-9]{6}$` and no
implementation. `Acme.ScalarRef` (id 8193) is a deep scalar whose `validate`
accepts any canonical scalar name present in the assembled registry, so its
vectors pass only if the binding really dispatches into the assembled
registry and not the built-in one.

## 1. Pick an id block

Built-in ids own `0..=4095`. An extension declares an `id_base` that is a
multiple of 4096 and at least 4096, and its ids live in
`[id_base, id_base + 4096)`. The example uses 8192. Once published, your ids
and canonical names are frozen under the same rule as the built-ins.

Two extensions with the same base cannot be assembled together, so if your
extension is meant to be combined with others, pick a base they do not use.
There is no central allocation of blocks; this is a coordination problem the
design leaves to extension authors.

## 2. Declare the extension

`ext/src/lib.rs`, in outline:

```rust
use std::sync::LazyLock;
use superscalar::{Extension, Registry, Scalar, ScalarDef, ScalarError, ScalarId};

pub const ACME_ORDER_NUMBER: ScalarId = ScalarId(8192);
pub const ACME_SCALAR_REF: ScalarId = ScalarId(8193);

static DEFS: [ScalarDef; 2] = [
    ScalarDef { id: ACME_ORDER_NUMBER, namespace: "Acme", canonical: "Acme.OrderNumber",
                pattern: Some("^ORD-[0-9]{6}$"), /* tag PatternOnly, other fields ... */ },
    ScalarDef { id: ACME_SCALAR_REF, namespace: "Acme", canonical: "Acme.ScalarRef",
                /* tag CustomLogic, other fields ... */ },
];

struct ScalarRef;
impl Scalar for ScalarRef {
    fn id(&self) -> ScalarId { ACME_SCALAR_REF }
    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        if registry.by_canonical(input).is_some() { Ok(()) } else { Err(/* ... */) }
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
    fn name(&self) -> &'static str { "acme" }
    fn id_base(&self) -> u32 { 8192 }
    fn defs(&self) -> &'static [ScalarDef] { &DEFS }
    fn impls(&self) -> Vec<(ScalarId, Box<dyn Scalar>)> {
        vec![(ACME_SCALAR_REF, Box::new(ScalarRef))]
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

Open points, each with the answer the example follows until decided:

- Whether all three `Scalar` hooks take `&Registry` or only `validate`. The
  example passes it to all three, because a `parse` that delegates to
  `validate` needs it.
- Whether `Extension` has a `name()` method. The example has one; assembly
  errors and the registry dump need an owner string.

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
attributes. Each macro may be invoked once per cdylib. A PyO3 module can add
extra functions with `extra = [f, g]`; the other three frameworks export
per-function, so you add extra functions by writing them beside the macro
call.

Open point: the C header for macro-generated functions. The recommended
answer, which the example follows, is that `superscalar-ffi` commits
`include/superscalar.h` for the nine generic entry points and a downstream
header includes it and declares any extra functions by hand.

## 4. Configure codegen

`superscalar.toml` names the packages and output paths per language. The
example's values:

```toml
[package]
name = "acme-scalars"

[registry]
source = "dump"
dump = "target/registry.json"

[go]
enabled = true
out = "go/generated.go"
package = "acmescalars"
header = "../ffi/include/acme_scalars.h"
cgo_library = "acme_scalars_ffi"

[typescript]
enabled = true
out = "typescript/src/generated.ts"
package = "acme-scalars"
backend_module = "./backend"
validation_module = "./validation"

[python]
enabled = true
out = "python/acme_scalars/_generated.py"
package = "acme_scalars"
native_module = "._native"

[rust_metadata]
enabled = false
```

The CLI can only see the built-in registry, because extensions are compiled
in. An extension therefore runs codegen from its own small binary, the
`xtask`, which dumps the assembled registry to the path named in
`[registry].dump` and calls the code generator library:

```rust
// xtask/src/main.rs, in outline
let registry = acme_scalars::registry();
std::fs::write("target/registry.json", superscalar_codegen::dump_to_string(registry))?;
let config = superscalar_codegen::Config::load("superscalar.toml".as_ref())?;
let ctx = superscalar_codegen::Context::new(registry, &config);
let emitters = superscalar_codegen::builtin_emitters();
let refs: Vec<&dyn Emitter> = emitters.iter().map(|e| e.as_ref()).collect();
let report = superscalar_codegen::run(&refs, &ctx, mode)?;   // Mode::Write or Mode::Check
```

`cargo run -p acme-xtask` writes the wrappers; `cargo run -p acme-xtask --
--check` fails if the committed files differ, which is the drift gate the
example's CI runs. An extension that needs an emitter the library does not
ship (a project-specific table, say) implements the `Emitter` trait and
pushes it onto the same list. The generated wrappers include the built-in
scalars as well as yours, so your Go package has `ParseContactEmail` next to
`ParseAcmeOrderNumber`.

## 5. Write vectors and run conformance

`conformance/acme-scalars.v2.json` has the same shape as the built-in
corpus. `Acme.ScalarRef` accepts `"Contact.Email"` and `"Acme.OrderNumber"`
and rejects `"Nope.Nope"`. The runners in `go/`, `typescript/` and `python/`
take a list of vector files and run both the built-in file and yours, so a
downstream assembly proves that the 44 built-ins still behave and that your
scalars behave. See [conformance](/superscalar/guides/conformance/).

The example's CI job runs, in order: `cargo test -p acme-scalars`, the xtask
in `--check` mode, the ffi static archive build plus `go test ./...`,
`maturin develop` plus `pytest`, and `napi build` plus `wasm-pack build
--target nodejs` followed by the Node conformance runner on both backends.
That job is the acceptance test for the whole model: a scalar was added
without touching any file under `crates/`.

## Combining with other extensions

`Registry::assemble` takes a slice, so a downstream can assemble several
extensions into one registry as long as their id blocks and canonical names
do not collide. The generated wrappers and the native artifact then carry all
of them. Nothing in the design lets two separately built native artifacts be
merged at runtime; assembly happens in Rust, at build time.
