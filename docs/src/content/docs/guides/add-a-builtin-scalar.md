---
title: Add a built-in scalar
description: Module, definition, vectors, regeneration and the docs check, in that order.
sidebar:
  order: 1
---

A built-in scalar is one that ships in the `superscalar` crate and in every
published binding. Add one when the type is generic: an email, a colour, a
duration. If the type belongs to one project (its own identifiers, its own
error codes), put it in an extension instead; see
[build an extension](/superscalar/guides/build-an-extension/).

The work is one Rust module (for a deep scalar), one registry entry, one
block of conformance vectors, a regeneration, and a docs check. Nothing is
written per language: the Go, Python and TypeScript wrappers, the C header
constants and the reference page are generated.

Before starting, read the behaviour freeze in `CONTRIBUTING.md`: a scalar's
accept set and canonical form are versioned, so a new scalar lands in a minor
release and an edit to an existing one is either a minor bump (loosening) or a
major bump (tightening).

The `superscalar` CLI used in steps 5 and 6 is being implemented for v0.1.0;
the `make` targets exist today. Steps 1 to 4 do not depend on it.

## 1. Decide the kind

Pattern plus bounds: a directive scalar, no module. Parsing or transformation:
a deep scalar, one module. Object-shaped: a structural scalar, a metadata type
under `crates/core/src/metadata/`. The
[scalar kinds](/superscalar/concepts/deep-directive-structural/) page has the
criteria.

## 2. Write the module (deep and structural only)

Create `crates/core/src/scalars/<name>.rs` and implement the `Scalar` trait:
`id`, `parse`, `normalize`, `validate`. Keep the three consistent with the
contract: `parse` returns the canonical form or an error, `normalize` moves
toward canonical form without enforcing shape, `validate` is `parse` without
the value. The email scalar is a small model to copy from: `normalize` trims
and lower-cases, `parse` normalizes then checks empty, pattern and length,
`validate` delegates to `parse`.

Errors use `ScalarError::new(ErrorKind::..., message)`. Pick the kind the
input actually failed (`Empty`, `Pattern`, `Length`, `Parse`, and so on);
the conformance vectors record it.

Register the module in `crates/core/src/scalars/mod.rs`. Under the open
registry, the implementation is listed in the built-in extension's `impls()`;
until that lands, it is one arm in the dispatch table. The assembly guard
fails the build if a `CustomLogic` definition has no implementation, so a
forgotten registration is caught.

## 3. Add the definition

Append a `ScalarDef` to the built-in catalog in `crates/core/src/catalog.rs`
with the next free id. Ids are append-only; the next free built-in id is 61,
and the holes below it are not reusable. Fill every field: canonical name and
namespace, primitive, `sql_type`, `json_schema_type`, tag, the declarative
rules, `examples` (at least one), a non-empty `description`, and
`type_mappings` for typescript, python, go, rust, sql and json_schema.

Two fields deserve care. `json_schema_type` decides sortability: a value that
is a JSON array or object must say so, or the scalar will be reported as
sortable. `comparability_class` stays `None` unless the scalar genuinely
compares with an existing class; see
[comparability and sortability](/superscalar/concepts/comparability-and-sortability/).

## 4. Add conformance vectors

Add a block keyed by the canonical name to `conformance/core-scalars.v2.json`
under `scalars`:

```json
"Contact.Email": {
  "accepted": [
    { "input": " User@Example.COM ", "normalized": "user@example.com" }
  ],
  "rejected": [
    { "input": "user@example", "validator": "pattern" },
    { "input": "", "validator": "empty" }
  ]
}
```

Each accepted vector states the canonical form `parse` must return. Each
rejected vector names the validator that fires. Cover the boundaries you
chose in the module: trimming, case, each length bound, each error kind.
Add the scalar's row to the corpus `metadata` section (its
`comparability_class` and `is_sortable`), and if it is non-sortable, increase
`meta.non_sortable_count` by hand; that count is a check on the generated
section and must not be derived from it.

Every binding runs these vectors, so this block is what makes the Go, Python,
TypeScript and WASM behaviour a tested fact rather than an assumption.

## 5. Regenerate

Run the code generator over the built-in registry using the checked-in
configuration:

```
cargo run -p superscalar-codegen
```

This rewrites the generated Go, Python and TypeScript wrappers and the Rust
metadata table. Commit the diff; never edit those files by hand. CI runs the
same command with `--check` and fails on drift. The C header is separately
drift-checked (`make header`).

## 6. Run the docs check and the gates

```
superscalar docs --check
make bindings
```

`superscalar docs --check` fails when a scalar has no vectors or no
description, which is the guarantee that every scalar gets a reference page
with real accepted and rejected inputs. `make bindings` runs the core tests,
lint, header drift, codegen drift, the C smoke test, the WASM smoke test and
the Go, Python and TypeScript conformance runs. All of them must pass before
you open a pull request.

## 7. Changelog

Add a line under the unreleased heading in `CHANGELOG.md` naming the scalar
and its id. A new scalar is a minor version change.

## Checklist

- Module implements `parse`, `normalize`, `validate` consistently (deep and
  structural only).
- Definition appended with the next free id; every field filled;
  `description` non-empty.
- Vectors cover accept, reject per validator, and the canonical form.
- Metadata row added; `non_sortable_count` adjusted if needed.
- `cargo run -p superscalar-codegen` run and diff
  committed.
- `superscalar docs --check` and `make bindings` pass.
- `CHANGELOG.md` updated.
