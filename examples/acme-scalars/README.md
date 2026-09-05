# acme-scalars

A complete downstream extension of the scalar core: two scalars in the `Acme`
namespace, assembled with the built-in registry and shipped through all four
bindings (C, Node via napi, Node via wasm, Python) without a change to any
crate outside this directory.

## What it proves

- An extension is a crate that implements `Extension` (`ext/src/lib.rs`): a
  name, an id block, a static array of `ScalarDef`s, and hand-written impls
  for the scalars that need code. `Registry::assemble(&[&AcmeExtension])`
  merges it with the built-ins and checks for collisions.
- A scalar with only a pattern and lengths (`Acme.OrderNumber`) needs no Rust
  code; the core's directive engine serves it from the def.
- A scalar whose rule depends on the catalog (`Acme.ScalarRef`, which accepts
  the canonical name of any scalar in the registry) sees the registry it was
  assembled into: it accepts `Acme.OrderNumber` through this assembly and
  rejects it through the built-in registry. Its vectors therefore pass only if
  a binding dispatches into the assembled registry.
- Each binding is one line: an export macro applied to `acme_scalars::registry`
  (`ffi/src/lib.rs`, `napi/src/lib.rs`, `python/src/lib.rs`, `wasm/src/lib.rs`).
  The exported symbols, and the C header, are the same as the built-in
  bindings' because the ABI does not depend on the registry behind it.
- The built-in conformance corpus still passes through the assembly, and the
  Acme corpus (`conformance/acme-scalars.v2.json`, same shape) passes through
  every binding.

## Layout

```
Cargo.toml                 own workspace (excluded from the parent), members below
ext/                       crate acme-scalars: AcmeExtension, DEFS, ScalarRef, registry()
  examples/dump.rs         prints Registry::dump() as JSON
  tests/conformance.rs     built-in + Acme vectors through the assembled registry
ffi/                       crate acme-scalars-ffi: export_c_abi!(acme_scalars::registry)
  tests/c_consumer.c       one scalar_parse call per process, driven by run_vectors.py
napi/                      crate acme-scalars-napi: export_napi!(...) plus the napi build.rs
python/                    crate acme-scalars-python: export_pymodule!(_native, ...)
wasm/                      crate acme-scalars-wasm: export_wasm!(...)
conformance/               acme-scalars.v2.json
scripts/smoke.sh           builds the four bindings and runs both corpora through each
scripts/run_vectors.py     vector runner for the C and Python bindings
scripts/run_vectors.cjs    vector runner for the napi and wasm bindings
scripts/cargo_artifact.py  asks cargo where it put a staticlib or cdylib
scripts/check_third_scalar.sh  applies third_scalar.patch, reruns the smoke, checks the diff
```

## Running it

```
examples/acme-scalars/scripts/smoke.sh
```

Needs the pinned Rust toolchain with the `wasm32-unknown-unknown` target,
`wasm-pack`, a C compiler, `node` and `python3`. The script installs nothing.
It runs `cargo test` (assembly, both corpora in Rust), dumps the registry to
get the canonical-name-to-id table, then for each binding builds the artifact
and runs `run_vectors.*` over `conformance/core-scalars.v2.json` and
`conformance/acme-scalars.v2.json`. Every non-built-in scalar in the dump must
have vectors, so a scalar added without them fails the run.

The bindings are loaded without their packaging tools: the napi cdylib is
loaded as a `.node` file, the PyO3 cdylib is copied under an extension-module
suffix and imported as `_native`, the wasm bundle comes from `wasm-pack`. A
published extension would wrap the same crates with `napi build` (package.json
`napi.name`), `maturin` (pyproject `module-name = "acme_scalars._native"`, which
is why the module identifier is `_native`), and `wasm-pack` for the browser
target. The C header is the generic `superscalar.h`; an extension with
extra C entry points writes its own header that includes it.

## Adding a scalar

Three files, all in this directory:

1. `ext/src/lib.rs`: a `ScalarId` constant at the next free id in the block
   (ids are append-only, never renumbered), a `ScalarDef` in `DEFS`, and an
   impl registered in `impls()` if the tag is not `PatternOnly`.
2. `conformance/acme-scalars.v2.json`: accepted and rejected vectors, and a
   `metadata` row (`comparability_class`, `is_sortable`), for the new name.
3. `README.md`, if the scalar is worth describing.

`scripts/check_third_scalar.sh` does exactly this with `scripts/third_scalar.patch`
(`Acme.Sku` at 4098), reruns the smoke, and fails if the run left a change
anywhere outside `examples/acme-scalars/`. That is the acceptance test for the
extension model: nothing under the core or binding crates moves when a
downstream scalar is added.

## Ids

An extension declares an `id_base` that is a multiple of 4096 and at least
4096; its ids live in `[id_base, id_base + 4096)`. The built-ins own
`0..=4095`. Acme uses 4096 (`Acme.OrderNumber`) and 4097 (`Acme.ScalarRef`).
Two extensions in one assembly need different blocks; assembly panics on an
overlap or a duplicate canonical name.
