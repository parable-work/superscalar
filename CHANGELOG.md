# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Every crate and package in this repository shares one version. Changes to a
scalar's conformance vectors or accept set are always listed here, with the
bump they require (minor when loosening, major when tightening).

## [Unreleased]

### Added

- Repository bootstrap: license, contribution guide, code of conduct,
  security policy, issue and pull request templates, dependency updates,
  CI, release and Scorecard workflows.
- Release pipeline: `release.yml` builds the static archives, napi addons,
  wheels, header and wasm bundles per platform, refuses a set not built from
  one commit, creates the GitHub release with provenance attestations and an
  SBOM, and publishes to crates.io, npm and PyPI through trusted publishing.
  `release-pr.yml` and `scripts/bump_version.py` set the one shared version;
  `go-module-tag.yml` cuts `go/vX.Y.Z` once the Go module is pinned to a
  release. The npm native platform packages are scoped:
  `@superscalar/darwin-arm64`, `@superscalar/darwin-x64`,
  `@superscalar/linux-x64-gnu`, `@superscalar/linux-arm64-gnu`; the main
  package `superscalar` loads the one for the host and falls back to its
  WASM build when none is installed.
- Go module: the module lives at `go/` so that its path
  `github.com/parable-work/superscalar/go` and its `go/vX.Y.Z` tags resolve;
  `include/superscalar.h` is committed inside the module, `release.pin`
  records the release the archives come from, and
  `scripts/fetch_release_archive.sh` downloads and verifies them against the
  pinned manifest digest.
- Conformance: `Temporal.Date` gains rejected vectors for bare epoch strings
  (`1736899200000`, `173689920000`, `1736899200`). The scalar is
  calendar-date-only and already rejected them; the vectors pin that so a
  consumer that converts epoch inputs itself at a storage boundary can rely
  on the rejection. No accept-set change, no bump required. Carried from
  parable-platform PR #5870.
- Core: `PrimitiveKind` is declared once with both of its spellings.
  `PrimitiveKind::ALL` lists every member; `from_dsl_name` inverts
  `dsl_name`; `type_ref_name` and `from_type_ref_name` give the schema-IR
  spelling (`Boolean` for `Bool`, `JSON` for `Object`). The Go binding emits
  `PRIMITIVE_KINDS` and `PrimitiveDSLNameByTypeRefName` from the same table.
- Core: the comparability rule behind `Registry::comparable_with` is one
  function over an alias-and-class lookup, table-tested for alias inheritance,
  transitivity over a three-member class and single-hop resolution. The class
  invariants also reject a class spanning two SQL types with no coercion
  between them and a class declared on an alias. No relation changes for any
  built-in.
- Scalars: `Ordering.Rank` (id 63, a positive JavaScript-safe integer),
  `Version.SemVer` (64, canonical Semantic Versioning 2.0.0),
  `Git.PathPattern` (66, a repository-rooted gitignore-style pattern, a deep
  scalar) and `AgentSkill.Name` (67, an Agent Skills directory and
  frontmatter name). Ids 61, 62 and 65 are held by a downstream extension and
  join the permanent holes; the next free built-in id is 68. New accept sets
  with vectors, a minor bump.
- Go: `ScalarMetadata` carries `TypeScriptType`, `PythonType` and `RustType`
  beside `GoType`; `GenericJSON` implements `driver.Valuer` and `sql.Scanner`
  and keeps SQL NULL distinct from an explicit JSON `null`.
- TypeScript: `JSONValue` and `isJSONValue` (`src/json-value.ts`), exported
  from the package root and from the generated module. Codegen reads the
  module path from the optional `[typescript] json_value_module` key
  (default `./json-value`).
- WASM: `scalar_parse` throws an error named `ScalarParseError` when the core
  rejects the input; an unknown scalar id still throws a plain `Error`, so a
  caller can tell an invalid value from a runtime failure. `run_parse` and
  the `scalar_parse` export of `export_wasm!` return `Result<String, JsValue>`
  (was `JsError`); a downstream that applies the macro needs no change.
- TypeScript: `firstAvailableBackend(loaders)`, used by `loadBackend` to try
  the native addon and then the WASM build, and to name every failed attempt
  when neither loads. The WASM bundle is found by package name
  (`superscalar/wasm-node/superscalar_wasm.js`) when a bundler has moved the
  backend module away from the package; `superscalar/wasm` exports the raw
  WASM bindings.
- Core: `scalars::json_scalar::serde`, serde adapters for `Generic.JSON`
  fields. `deserialize_with = "superscalar::scalars::json_scalar::serde::deserialize"`
  on a `serde_json::Value`, `Option`, `Vec` or `HashMap` field parses the
  value losslessly from text or from `serde_json::from_value`, and
  `parse_value` is the parser behind it.
- Core: `Definitions`, the assembled scalar definitions without
  implementations. `Definitions::builtin()`, `Definitions::assemble(&[(owner,
  defs)])` and `try_assemble` build it from static `ScalarDef` slices, never
  through `Extension`, so a consumer that only reads definitions links no
  scalar implementation (a size-capped WASM bundle is the motivating case).
  It answers `def`, `by_canonical`, `resolved`, `comparable_with`, `ids`,
  `defs`, `len` and `is_empty` with the `Registry` semantics, and runs the
  def-level assembly checks (duplicate id, duplicate canonical name,
  namespace, dangling and chained alias) with the same `AssemblyError`.
  `Registry` assembles one first and delegates those lookups to it;
  `Registry::definitions()` returns it. `scalar_def` reads
  `Definitions::builtin()`. Additive; no `Registry` behaviour changes.

### Changed

- Directive engine: `normalize` on a `case_insensitive` scalar now lowercases
  an input that fails the scalar's own `validate` when the lowercased form
  passes, so normalize never emits a value its validator refuses. An
  already-valid input is returned unchanged. Of the built-ins this moves
  `Identity.Slug` (`ACME` normalizes to `acme`); `Temporal.Quarter` and
  `Network.DomainName` accept either case and do not move, and scalars with a
  hand-written impl are unaffected. `parse` stays strict. No accept-set
  change and no vector change; normalize output changes, a minor bump.
- `Generic.JSON` is any JSON value, not only an object: `json_schema_type` is
  `any`, the TypeScript type is `JSONValue`, the Python type is `Any`, and the
  description, docstring and examples say so. The core already accepted every
  JSON root, so the accept set does not change. The generated TypeScript
  wrappers change shape: `parseGenericJSON` and `normalizeGenericJSON` take a
  decoded host value and return `GenericJSON | undefined`, keeping an explicit
  `null` as a value and reporting an absent or non-portable input (`NaN`, a
  cycle, a `Set`) as `undefined`; `validateGenericJSON` rejects such inputs.
  A breaking change for TypeScript callers of those wrappers. `Generic.JSON`
  and `Git.PathPattern` set the `validate` hook.
- `Generic.StringMap` sets the `parse` hook, so a generated schema runtime
  hands its value to the core parser. The TypeScript strict wrappers decode
  the core's canonical JSON into a `Record<string, string>` instead of
  returning the JSON text; the converter is chosen from the def (`parse` hook
  plus an object JSON shape), not from the scalar's name. No accept-set
  change.
- `Generic.JSON` and `Generic.StringMap` parse through
  `scalars::json_scalar::serde::parse_value`: numbers keep their exact
  digits instead of a floating-point round trip, and object keys that match
  serde_json's private number and raw-value markers stay literal. Two new
  accepted `Generic.JSON` vectors pin both. The accept set does not change;
  the canonical form of a number beyond `f64` precision does, a minor bump.
  The core's `serde_json` dependency now enables `arbitrary_precision` and
  `raw_value`. Cargo unifies features, so every crate that shares the
  `serde_json` build with the core sees them too.

[Unreleased]: https://github.com/parable-work/superscalar/commits/main
