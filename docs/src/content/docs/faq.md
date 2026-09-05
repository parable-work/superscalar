---
title: FAQ
description: Why the library is written in Rust, why the Go binding uses cgo, and why there is a WASM build.
---

## Why is the core written in Rust?

Before the Rust core, the same rules were hand-written four times, in Go,
TypeScript, Python and Rust, and kept in step by a partial fixture and by
comments asking a human to keep the copies aligned. Two of the four lagged,
and a normalization mismatch on a scalar that is identity or key material is
silent data corruption, not a cosmetic bug. One implementation removes the
cause instead of testing for the symptom. Rust was the language that could be linked natively from a
Rust data engine, exposed through a C ABI to Go and Python, wrapped for Node
with napi-rs, and compiled to WebAssembly for browsers, all from one crate
with `#![forbid(unsafe_code)]` in the core.

## Why does the Go binding use cgo?

The Go binding calls the Rust core through a C ABI, which needs cgo and a C
toolchain at build time. The archive is linked statically, so the resulting
binary has no runtime shared-library dependency, and the archives are
prebuilt per platform and fetched, so no Rust toolchain is needed. The cost
is real: `CGO_ENABLED=1` and a compiler enter the build, and cross compiling
gets harder. A cgo-free backend that runs the WASM build from pure
Go, on the model of `ncruces/go-sqlite3`, is planned as a follow-up after
v0.1.0.

## Why is there a WASM build?

Browsers and edge runtimes cannot load a native addon, and the TypeScript
package is meant to work there with the same import and the same behaviour
as in Node. `wasm-bindgen` compiles the same core to WebAssembly, the
package's `exports` map selects it under the `browser`, `edge-light` and
`workerd` conditions, and the conformance corpus runs against both the napi
addon and the WASM build in CI to catch native-versus-WASM divergence. The
WASM build is also the fallback in Node on a platform without a prebuilt
addon, and the basis for the planned cgo-free Go backend.

## Why are ids numeric and frozen?

The C ABI dispatches on a `u32` id, generated bindings embed the id per
function, and stored data may carry it. Renumbering would change the meaning
of existing binaries and data. So ids are append-only forever, holes are
permanent, and extensions get their own 4096-wide blocks so they can never
collide with a future built-in. See
[ABI and versioning](/superscalar/policy/abi-and-versioning/).

## Why is the reference documentation generated?

Because a hand-written page can be missing or stale, and a generated one
cannot. `superscalar docs` writes one page per scalar from the registry and
the conformance vectors, and `--check` fails the build when a scalar has no
vectors or no description. Adding a scalar without documenting its accepted
and rejected inputs is therefore not possible. See the
[scalar reference](/superscalar/reference/).

## Can I add scalars without forking?

Yes. An extension is a Rust crate that declares definitions and
implementations in its own id block, assembles a registry with the built-ins,
and applies four one-line macros to get native bindings carrying the whole
catalog. The code generator then emits Go, Python and TypeScript wrappers
for your scalars from a configuration you own. See
[build an extension](/superscalar/guides/build-an-extension/).
