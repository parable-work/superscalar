# Contributing to SuperScalar

This file covers what you need before opening a pull request: the sign-off
requirement, the toolchain, the gates CI runs, and the rules that keep scalar
behaviour stable across releases.

## Sign your commits (DCO)

This project does not use a CLA. Every commit must carry a Developer
Certificate of Origin sign-off. Add it with the `-s` flag:

```
git commit -s -m "your message"
```

Git appends a line of the form `Signed-off-by: Your Name <you@example.com>`
using your configured `user.name` and `user.email`. To add sign-off to commits
you already made:

```
git rebase --signoff origin/main
```

By signing off you agree to the Developer Certificate of Origin 1.1, which is
reproduced below and also published at https://developercertificate.org/.
CI rejects a pull request if any commit is missing the line, and the
repository requires sign-off on commits made through the GitHub web UI.

```
Developer Certificate of Origin
Version 1.1

Copyright (C) 2004, 2006 The Linux Foundation and its contributors.

Everyone is permitted to copy and distribute verbatim copies of this
license document, but changing it is not allowed.


Developer's Certificate of Origin 1.1

By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I
    have the right to submit it under the open source license
    indicated in the file; or

(b) The contribution is based upon previous work that, to the best
    of my knowledge, is covered under an appropriate open source
    license and I have the right under that license to submit that
    work with modifications, whether created in whole or in part
    by me, under the same open source license (unless I am
    permitted to submit under a different license), as indicated
    in the file; or

(c) The contribution was provided directly to me by some other
    person who certified (a), (b) or (c) and I have not modified
    it.

(d) I understand and agree that this project and the contribution
    are public and that a record of the contribution (including all
    personal information I submit with it, including my sign-off) is
    maintained indefinitely and may be redistributed consistent with
    this project or the open source license(s) involved.
```

## Toolchain

| Tool      | Version                | How it is pinned                                  |
| --------- | ---------------------- | ------------------------------------------------- |
| Rust      | 1.95.0                 | `rust-toolchain.toml` (rustfmt, clippy, wasm32)   |
| cbindgen  | 0.29.2                 | `tools.env` (`CBINDGEN_VERSION`)                  |
| wasm-pack | 0.15.0                 | `tools.env` (`WASM_PACK_VERSION`)                 |
| Go        | 1.26.4                 | `tools.env` (`GO_VERSION`)                        |
| Node      | 24                     | `tools.env` (`NODE_VERSION`)                      |
| Python    | 3.9 or newer, maturin  | floor in `bindings/python/pyproject.toml`; CI tests on `tools.env` (`PYTHON_VERSION`) |

Setup on a fresh machine:

```
rustup toolchain install            # reads rust-toolchain.toml
rustup target add wasm32-unknown-unknown
. ./tools.env
cargo install cbindgen --version "$CBINDGEN_VERSION" --locked
cargo install wasm-pack --version "$WASM_PACK_VERSION" --locked
pip install maturin
```

Go, Node and a C compiler (`cc`) come from your OS package manager. The Go
binding needs `CGO_ENABLED=1`. The Miri gate additionally needs a nightly
toolchain with the `miri` component:

```
rustup toolchain install nightly --profile minimal --component miri
```

Bump a tool version in `tools.env` or `rust-toolchain.toml` only; workflows
read those files and never inline a version.

## Running the gates

The Makefile mirrors `.github/workflows/ci.yml`. A pull request must pass all
of them; run them locally before pushing.

| Target                | What it checks                                                     |
| --------------------- | ------------------------------------------------------------------ |
| `make test`           | `cargo test` for the core and ffi crates                           |
| `make lint`           | `cargo fmt --check`, clippy with `-D warnings` (host and wasm32)   |
| `make header`         | The committed C header matches cbindgen output                     |
| `make c-smoke`        | A small C program compiles against the header and links the staticlib |
| `make wasm`           | wasm-pack build and a Node smoke test                              |
| `make miri`           | `cargo +nightly miri test` on the ffi crate                        |
| `make codegen-check`  | Generated Go, Python and TypeScript files match the registry       |
| `make go`             | Go binding builds and passes the conformance vectors               |
| `make python`         | Python wheel builds and passes the conformance vectors             |
| `make ts`             | TypeScript builds against napi and WASM, both pass the vectors     |
| `make bindings`       | All of the above except `make miri`, which needs nightly           |

## Rules

### Behaviour freeze

A scalar's behaviour is its conformance vectors and its accept set (the inputs
it parses without error). Neither changes silently. A pull request that adds,
removes or edits a vector, or changes which inputs a scalar accepts, must
include a `CHANGELOG.md` entry and a version bump: loosening an accept set is
a minor bump, tightening one is a major bump. If you find a bug in a scalar,
file an issue and leave the vector as it is until the change is agreed.

### Frozen ids

Scalar ids (the `u32` values in the catalog) and canonical names are
append-only forever. Never renumber, reuse or delete an id. Holes are
permanent.

### Generated files are never hand-edited

Files produced by the codegen crate (the Go, Python and TypeScript wrappers,
the C header, the scalar reference docs) are regenerated, not edited. Change
the registry or the emitter, run `make codegen-check` (or the regenerate
command it prints), and commit the diff. CI fails on drift.

### Adding a built-in scalar

Add the module under `crates/core/src/scalars/`, register it with the next
free id, add vectors to `conformance/core-scalars.v2.json`, regenerate the
bindings and docs, and run `make bindings`. Prefer an extension crate for
scalars specific to one project; see the `examples/acme-scalars/` walkthrough.

### Prose

Plain ASCII in source, docs and commit messages. Keep comments and docs short
and concrete.

## Releases and trusted publishing

Releases are cut from a `vX.Y.Z` tag by `.github/workflows/release.yml`.
crates.io, npm and PyPI are configured for trusted publishing (OIDC); no
registry tokens are stored in this repository. Each registry's trusted
publisher entry is registered against this repository and the workflow
filename `release.yml`. If the workflow is ever renamed, every registry entry
must be updated to match or publishing stops.
