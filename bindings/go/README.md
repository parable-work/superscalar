# superscalar (Go)

Go binding of the SuperScalar core. The package calls the Rust core through
cgo against a prebuilt static archive, so a consumer needs a C toolchain but
no Rust toolchain, and the resulting binary has no runtime shared-library
dependency on superscalar.

Module: `github.com/parable-work/superscalar/go`, package `superscalar`.
Every generated scalar has `Parse<Name>`, `Normalize<Name>` and
`Validate<Name>`; see `generated.go` and the docs site.

## What the module contains and what it fetches

Committed in the module:

- `include/superscalar.h`: the C ABI header, a byte-identical copy of
  `crates/ffi/superscalar.h` kept in sync by
  `crates/ffi/scripts/check_header.sh` (CI fails on drift).
- `cgo_ldflags_<goos>_<goarch>.go`: the per-platform link directives (below).
- `release.pin`: the GitHub release the archives come from, the core commit
  they were built from, and the SHA-256 of that release's `manifest.json`.
  Written by the release pipeline, never by hand.

Fetched at build time:

- `lib/<goos>_<goarch>/libsuperscalar_ffi.a`: the static archive for your
  platform, a GitHub release asset. `lib/` is gitignored.

Supported platforms: `darwin_arm64`, `darwin_amd64`, `linux_amd64`,
`linux_arm64`. The Linux archives are built against musl and link into glibc
or musl programs alike.

## Getting the archive

### From a release (no Rust toolchain)

```
scripts/fetch_release_archive.sh
```

reads `release.pin`, downloads `manifest.json` and
`superscalar-<goos>_<goarch>.tar.gz` from that release, and verifies, in
order: the manifest's SHA-256 against the pin, the manifest's core commit
against the pin, that every artifact in the manifest was built from that one
commit, and the archive's SHA-256 against its manifest entry. Only then does
it place the archive under `lib/`. It uses the GitHub CLI when installed
(required while the repository is private), otherwise `curl`.

Inside a checkout of this repository that is all you need; `go build` and
`go test` in this directory link `lib/<platform>` through the committed cgo
flags.

If you installed the module with `go get`, its directory sits in the Go module
cache and is read-only, so give the script a writable output directory and
point cgo at it:

```
mod="$(go list -m -f '{{.Dir}}' github.com/parable-work/superscalar/go)"
bash "$mod/scripts/fetch_release_archive.sh" --out "$PWD/third_party/superscalar"
export CGO_LDFLAGS="-L$PWD/third_party/superscalar/darwin_arm64"   # your platform
go build ./...
```

Go's build cache does not hash the archive bytes, so if you replace an archive
at the same path, run `go clean -cache` or use a versioned output directory.

Optional: verify the release assets' build provenance before extracting them
with `gh attestation verify <asset> --repo parable-work/superscalar`.

### From source (Rust toolchain)

```
scripts/build_ffi.sh
```

builds `superscalar-ffi` for the host and stages it under `lib/`. This is the
development loop and what CI runs; `scripts/go_smoke.sh` builds and then runs
the conformance tests.

## cgo flags

The module sets these in `binding.go` and `cgo_ldflags_<goos>_<goarch>.go`;
a consumer normally sets nothing. Listed here so you can reproduce a link
outside the module (for example in a Bazel or CMake build):

| Platform     | CFLAGS                 | LDFLAGS                                                        |
| ------------ | ---------------------- | -------------------------------------------------------------- |
| darwin_arm64 | `-I<module>/include`   | `-L<module>/lib/darwin_arm64 -lsuperscalar_ffi -liconv`        |
| darwin_amd64 | `-I<module>/include`   | `-L<module>/lib/darwin_amd64 -lsuperscalar_ffi -liconv`        |
| linux_amd64  | `-I<module>/include`   | `-L<module>/lib/linux_amd64 -lsuperscalar_ffi -lm -ldl -lpthread` |
| linux_arm64  | `-I<module>/include`   | `-L<module>/lib/linux_arm64 -lsuperscalar_ffi -lm -ldl -lpthread` |

Requirements: `CGO_ENABLED=1` and a C compiler (`cc`) on the build machine.
Extra `-L` directories can be added through the `CGO_LDFLAGS` environment
variable; they are searched alongside the ones above.

For a fully static Linux binary:

```
go build -ldflags '-linkmode external -extldflags "-static"' ./...
```

## Versions and tags

The module shares the repository version. `go get
github.com/parable-work/superscalar/go@vX.Y.Z` resolves the tag `go/vX.Y.Z`,
which is cut on the commit that pins `release.pin` to the `vX.Y.Z` release
(one commit after the `vX.Y.Z` tag itself). CONTRIBUTING.md, "Releases",
describes the sequence.
