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
- Scalars: `Ordering.Rank` (a positive JavaScript-safe integer),
  `Version.SemVer` (canonical Semantic Versioning 2.0.0),
  `Git.PathPattern` (a repository-rooted gitignore-style pattern, a deep
  scalar whose escape state is the parity of each backslash run, so `\\[b]`
  opens a class and `\\ ` leaves a bare trailing space) and
  `AgentSkill.Name` (an Agent Skills directory and frontmatter name). New
  accept sets with vectors, a minor bump.
- Go: `ScalarMetadata` carries `TypeScriptType`, `PythonType` and `RustType`
  beside `GoType`; `GenericJSON` implements `driver.Valuer` and `sql.Scanner`
  and keeps SQL NULL distinct from an explicit JSON `null`.
- TypeScript: `JSONValue` and `isJSONValue` (`src/json-value.ts`), exported
  from the package root and from the generated module. Codegen reads the
  module path from the optional `[typescript] json_value_module` key
  (default `./json-value`).
- WASM: `scalar_parse` throws an error named `ScalarParseError` when the core
  rejects the input; an unknown scalar name still throws a plain `Error`, so a
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
  It answers `def`, `resolved`, `comparable_with`, `names`, `defs`, `len`
  and `is_empty` with the `Registry` semantics, and runs the def-level
  assembly checks (duplicate canonical name, namespace, dangling and chained
  alias) with the same `AssemblyError`.
  `Registry` assembles one first and delegates those lookups to it;
  `Registry::definitions()` returns it. `scalar_def` reads
  `Definitions::builtin()`. Additive; no `Registry` behaviour changes.

### Changed

- `Geo.Location` is a JSON-object scalar, `{"lat": <number>, "lon": <number>}`,
  and its definition agrees with itself. Breaking: the accept set tightens and
  the canonical form changes, a major bump.
  - Accept set: the `"lat,lon"` string is refused by `parse`, `normalize` and
    `validate` (it was accepted and normalized to the object); so are an
    unknown or duplicate key (unknown keys were ignored), `lat` outside
    [-90, 90] or `lon` outside [-180, 180] (unchecked before), and a member
    that is not a JSON number. Text that is not JSON fails as `parse` (with a
    message naming the object form when it is a `"lat,lon"` string); another
    JSON type, a missing, unknown or duplicate key, or a member that is not a
    number fails as `custom`; an out-of-range degree fails as `range`.
  - Canonical form: `{"lat":<lat>,"lon":<lon>}` with each number as
    `JSON.stringify` and Go's `encoding/json` write it: `{"lat":90,"lon":-180}`
    (was `90.0` and `-180.0`), negative zero as `0`, exponent form below
    1e-6.
  - Definition: primitive `String` (was `Object`), tag `CustomLogic` (was
    `Structural`, which turned the generated validators off), no `pattern`
    (was the `lat,lon` regex), no `schema_primitive_override` (was `String`,
    so the emitted schema primitive is still `String`), the `parse` hook set,
    the example `{"lat":37.7749,"lon":-122.4194}`, and a docstring stating
    that a Postgres `POINT` is (x, y) with x = `lon` and y = `lat`. The row now
    has the same shape as `Generic.StringMap`'s. Type mappings: Go
    ``struct{ Lat float64 `json:"lat"`; Lon float64 `json:"lon"` }``, Python
    `superscalar.GeoLocation` (was `dict`), Rust
    `superscalar::metadata::geo_location::Location` (was a struct
    declaration, not a type); TypeScript, SQL `POINT` and JSON Schema `object`
    are unchanged. No built-in is `Structural` or `PrimitiveKind::Object` now.
  - Go: `GeoLocation` has JSON tags and marshals as `{"lat":...,"lon":...}`
    (was `{"Lat":...,"Lon":...}`). It gains `ParseGeoLocation`,
    `NormalizeGeoLocation`, `ValidateGeoLocation`, the `String`, `Validate`
    and `ValidateRequired` methods and a `ValidatorFor` case; its metadata row
    has `HasValidator` and `HasCustomParse` true. `GeoLocationPattern` is
    removed. A zero `GeoLocation` is the point (0, 0), not a missing required
    value.
  - TypeScript: the metadata row has `hasValidator` true and no pattern; the
    `GeoLocation` type and wrappers are unchanged.
  - Python: `superscalar.GeoLocation`, a `TypedDict` with `lat` and `lon`.
  - Rust: `metadata::geo_location::Location` deserializes under the scalar's
    rules (unknown and duplicate keys, ranges) and gains `Location::new`,
    `FromStr` and `Display`, which writes the canonical text.
  - Conformance: the `"lat,lon"` vector moves from accepted to rejected, and
    new vectors cover the inclusive bounds, integers, negative values, member
    order, number spelling, negative zero, and the refusals above. Two new
    core tests keep definitions honest: a JSON-valued scalar declares no
    `pattern` or length bounds, and every example parses to itself.
- A scalar's canonical name is its only identity; numeric scalar ids are
  gone. Every C ABI entry point takes the name where it took a `uint32_t`
  (`scalar_parse(const char *scalar, const char *input)` and the other eight),
  and so do the WASM, napi and PyO3 exports. An unknown name fails with
  `unknown scalar "<name>"`. In Rust, `ScalarId` gives way to `&str` names and
  a `names` module of built-in constants (`names::CONTACT_EMAIL`); `Registry`
  and `Definitions` look scalars up by name (`def`, `scalar`, `owner`,
  `resolved`, `comparable_with`, `scalar_for`, `scalar_def`), iterate in name
  order (`names()` replaces `ids()`), and `by_canonical` folds into `def`.
  `ScalarDef::id` is removed and `alias_of` holds the target's name.
  `Extension` loses `id_base` and keys `impls` by name. `Scalar::id`,
  `AssembleOptions`, `allow_legacy_ids`, `ExtensionInfo` and the id assembly
  errors (`DuplicateId`, `IdBaseNotAligned`, `IdBaseReserved`,
  `IdOutOfBlock`, `ImplIdMismatch`) are removed; `ForeignImpl`,
  `DanglingAlias` and `AliasChain` carry names, and `Registry::extensions()`
  returns extension names. The registry dump is `dump_version` 2: no `id`,
  `alias_of` by name, `extensions` as a list of names. The generated
  bindings key on names: Go drops `ScalarIDByCanonical` (`VALID_SCALARS` and
  `KnownScalar` remain), TypeScript replaces `scalarIdByCanonical` and Python
  replaces `SCALAR_ID_BY_CANONICAL` with `VALID_SCALARS`, and Python drops
  `_native_ids`. The Go codegen anchor `go.after_ids` is now
  `go.before_scalar_list`. Generated files and the dump list scalars in name
  order. Breaking for every consumer, and made before the first release so
  the ids never become part of the ABI. No accept-set or vector change.
- `superscalar-python` builds with pyo3 0.29 (was 0.25). An extension's
  PyO3 module crate must depend on pyo3 0.29 too, since Cargo refuses two
  pyo3 minors in one graph; the acme example moves with it.

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
  Exact numbers come from the `lossless-json` feature, on by default, which
  enables serde_json's `arbitrary_precision`; `raw_value` is always on. Cargo
  unifies features, so every crate that shares the `serde_json` build with
  the core sees them too, and serde_json then fails on floats inside
  `flatten` fields and untagged or tagged enums. A downstream with such types
  depends on the core with `default-features = false`; the language bindings
  always enable the feature. Text that cannot spell a serde_json marker name
  as an object key (no `serde_json::private`, no `\u` escape) parses in one
  pass; other text re-reads each container's slice, so its cost grows with
  nesting depth.
- The generated Rust and TypeScript metadata tables carry
  `ScalarDef::format` (`semver` for `Version.SemVer`) instead of an empty
  value. No accept-set change.

### Fixed

- The build-an-extension guide describes `examples/acme-scalars/` as it
  ships: the extension crate, the four binding crates, the smoke and
  third-scalar scripts. It no longer claims the example
  has a `superscalar.toml`, an xtask or generated Go, Python and TypeScript
  packages; codegen is presented as the next step for a real extension. The
  sample `superscalar.toml` now parses (`[registry] source = "builtin"`,
  `[go] module` set, no `[rust_metadata]` stub) and the sample xtask is
  complete. Documentation only.
- The registry page's open-registry section describes what ships: the four
  `Extension` methods, `assemble` and `try_assemble`, the nine assembly
  checks in the order they run with their `AssemblyError` variants, the
  lookups on `Registry`, and the dump's fields. It no longer says the code
  generator and the docs generator consume the dump; they read the
  `Registry`. The build-an-extension guide adds the legacy alias check to its
  list. The conformance guide no longer calls multi-file loading future work:
  it says which runners read one file and which merge several, and drops the
  claims that `non_sortable_count` sums across files and that a total case
  count is asserted. Documentation only.
- `Contact.PhoneNumber`'s example is `+14155552671`, a number the scalar
  accepts in its canonical E.164 form. The old example, `+1234567890`, failed
  the scalar's own validation, yet the generated Go and TypeScript metadata
  and the reference docs showed it. Metadata only: no vector or accept-set
  change, no bump required.

[Unreleased]: https://github.com/parable-work/superscalar/commits/main
