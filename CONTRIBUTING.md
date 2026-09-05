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
| `make docs-check`     | Every scalar has conformance vectors and a description             |
| `make go`             | Go binding builds and passes the conformance vectors               |
| `make python`         | Python wheel builds and passes the conformance vectors             |
| `make ts`             | TypeScript builds against napi and WASM, both pass the vectors     |
| `make acme`           | The `examples/acme-scalars/` extension builds all four bindings and passes both corpora |
| `make acme-third-scalar` | Adding a scalar to the example changes nothing outside `examples/acme-scalars/` |
| `make bindings`       | All of the above except `make miri` (needs nightly) and `make acme-third-scalar` |

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

## Releases

One version for everything: the six crates, the npm package and its platform
packages, the PyPI distribution, the Go module and the `examples/acme-scalars`
workspace all carry the same SemVer version, and `scripts/bump_version.py`
is the only thing that writes it. `bump_version.py check` fails when any site
disagrees; the release workflow runs it before building anything.

Two workflows (`release-pr` and the `go-pin` job of `release.yml`) open pull
requests with the workflow token, which the repository setting "Allow GitHub
Actions to create and approve pull requests" (Settings -> Actions -> General)
must permit; it is off by default on a new repository.

A release is four steps, each started by a person:

1. Open the release pull request: run the `release-pr` workflow (Actions ->
   release-pr -> Run workflow) with the version, without the leading `v`
   (`0.1.0-alpha.1`, `0.1.0`). It runs `bump_version.py set`, which writes the
   version everywhere and cuts the `Unreleased` section of `CHANGELOG.md`
   into a dated `[X.Y.Z]` section, then opens `release/vX.Y.Z`. Review the
   changelog. The pull request is opened with the workflow token, which does
   not start CI: close and reopen it once so `ci-pass` runs, then merge it.
   (Without the workflow: `python3 scripts/bump_version.py set X.Y.Z` on a
   branch and open the pull request yourself.)
2. Cut the tag: on a clean checkout of `main` at that merge, run
   `scripts/tag_release.sh`. It re-checks every version site, creates the
   annotated tag `vX.Y.Z` and pushes it. The push starts
   `.github/workflows/release.yml`.
3. `release.yml` runs the full CI, builds the static archives, napi addons,
   wheels, header and wasm bundles on macOS and Linux runners, merges the
   per-runner manifests and refuses a set whose artifacts were not all built
   from the tag's commit, creates the GitHub release with build provenance
   and an SBOM, publishes to crates.io, npm and PyPI, and opens a second pull
   request that pins `go/release.pin` to the release's manifest
   digest. Close and reopen that one too, then merge it.
4. Merging the pin pull request cuts the Go module tag `go/vX.Y.Z`
   (`.github/workflows/go-module-tag.yml`) on the pin commit. `go get
   github.com/parable-work/superscalar/go@vX.Y.Z` resolves that tag, and the
   module's fetch script then downloads and verifies the archives of the
   `vX.Y.Z` release. The two tags are always cut together in this order; do
   not create either by hand.

Pre-releases: `vX.Y.Z-alpha.N`, `-beta.N` and `-rc.N` are the supported
forms. The GitHub release is marked as a pre-release, npm publishes under the
`next` dist-tag instead of `latest`, and PyPI receives the PEP 440 spelling
(`0.1.0a1`), which `pip` skips unless asked for `--pre`. The first release is
`v0.1.0-alpha.1`.

A dry run of the build on any branch: Actions -> release -> Run workflow with
`dry_run` checked (or `gh workflow run release.yml --ref <branch> -f
dry_run=true`). It runs the verify, build and assemble jobs and uploads the
assembled release set as the `release-assets` workflow artifact; nothing is
released, published or deployed.

### Trusted publishing

crates.io, npm and PyPI are configured for trusted publishing (OIDC); no
registry tokens are stored in this repository. Each registry's trusted
publisher entry is registered against the repository `parable-work/superscalar`
and the workflow filename `release.yml`, with the GitHub environment named in
the job (`crates-io`, `npm`, `pypi`). If the workflow is renamed or an
environment name changes, every registry entry must be updated to match or
publishing stops. The three publish jobs are gated on the repository variable
`RELEASE_PUBLISH_ENABLED`; it is set to `true` once the registrations exist.
crates.io only accepts a trusted publisher for a crate that already exists, so
the first version of each crate is published by hand with a personal token
before the publisher is registered; the exact registrations are written at
the top of each publish job in `release.yml`.

### Behaviour changes and the changelog

A change to a scalar's conformance vectors or accept set never ships without
a `CHANGELOG.md` entry under `Unreleased` that names the bump it requires
(see "Behaviour freeze" above). `release-pr` refuses to cut a release whose
`Unreleased` section is empty, and `release.yml` refuses a tag whose version
has no changelog section.
