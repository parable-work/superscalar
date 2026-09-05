---
title: Conformance
description: The vector corpus, what it proves, and how to run it against your own build or extension.
sidebar:
  order: 3
---

The conformance corpus is a JSON file of inputs and expected outcomes for
every scalar. The Rust core, the Go, Python and TypeScript bindings and the
WASM build all run it in CI. With one implementation there is no cross-language
drift to catch; what the corpus proves is that each binding exposes the core
faithfully, that the WASM build agrees with the native build, and that a
change to a scalar's behaviour is visible as a change to a vector.

If you build the library yourself, or assemble an extension, running the
corpus against your build is how you know it behaves like the reference.

## The file

`conformance/core-scalars.v2.json` holds the vectors for the 44 built-in
scalars. Its top level:

- `meta`: `version` (2), `source_language` (`rust-core`), a description, and
  `non_sortable_count`, a hand-maintained expectation checked against the
  `metadata` section.
- `scalars`: a map from canonical name to `{ accepted, rejected }`.
- `metadata`: a map from canonical name to `{ comparability_class,
  is_sortable }`, a transcript of the generated metadata table.
- `metadata_excluded`: canonical names that appear in `scalars` but have no
  metadata row. Empty for the built-in corpus.

An accepted vector is `{ "input": ..., "normalized": ... }`: `parse(input)`
must succeed and return exactly `normalized`. A rejected vector is `{ "input":
..., "validator": ... }`: `parse(input)` must fail, and `validator` names the
error kind that fires (`pattern`, `empty`, `length`, `parse`, and so on).

A vector may carry `"unresolved": true`. That flags intended behaviour the
core does not yet enforce; every runner skips it and counts the skips, so the
suite stays green while the gap is visible. Do not add unresolved vectors to
paper over a failing case; they exist to record an agreed tightening that has
not shipped.

## What each runner checks

Each runner iterates `scalars`, resolves the canonical name to a numeric id
through the generated table of the binding under test (`ScalarIDByCanonical`
in Go, `scalarIdByCanonical` in TypeScript, `SCALAR_ID_BY_CANONICAL` in
Python, `Registry::by_canonical` in Rust), and calls the binding's parse. A
key the table lacks fails the run, which is how a vector file from the wrong
assembly is caught.

Beyond parse, the runners assert the metadata section: every scalar in
`scalars` has a row in `metadata` or is listed in `metadata_excluded`, each
row's `is_sortable` and `comparability_class` match the binding's table, and
the count of non-sortable rows equals `meta.non_sortable_count`.

The TypeScript runner goes through both backends, the napi addon and the
WASM build, and reports each separately.

## Running against the reference build

From the repository root, with the toolchain from `CONTRIBUTING.md`:

```
cargo test                 # core: every vector, plus the metadata tests
make go                    # builds the Go binding and runs go test ./...
make python                # builds the wheel and runs pytest
make ts                    # builds napi and WASM and runs the Node runner on both
make bindings              # all of the above plus lint, header and codegen drift
```

The Go, Python and TypeScript runners live in `bindings/go/conformance_test.go`,
`bindings/python/tests/test_conformance.py` and
`bindings/typescript/test/conformance.cjs`. Each prints a pass and fail
count per bucket.

## Running against your own build or extension

Multi-file loading is part of the extension work for v0.1.0; today's runners
read the one built-in file. Once it lands, every runner takes a list of
vector files rather than one hard-coded path.
Loading merges the files: `scalars` keys must be disjoint across files (the
runner fails otherwise), `metadata` keys likewise, `metadata_excluded` lists
concatenate, and `non_sortable_count` values sum. The total case count is
asserted per assembly so a missing file is noticed.

The file lists in use:

- Built-in bindings in this repository: `conformance/core-scalars.v2.json`.
- The `examples/acme-scalars/` extension: `conformance/core-scalars.v2.json`
  plus `conformance/acme-scalars.v2.json`.
- Your extension: the built-in file at the version you depend on, plus one
  file per extension you assemble.

Copy a runner from the acme example into your extension, set its file list,
and run it in your CI against each binding you ship. If the built-in vectors
fail through your assembly, the fault is in how the registry was assembled or
how the binding was built, not in the built-ins.

## Vectors are the versioning unit

A pull request that adds, removes or edits a vector, or changes which inputs a
scalar accepts, is a versioned behaviour change: loosening is a minor bump,
tightening is a major bump, and either needs a changelog entry. The
[ABI and versioning](/superscalar/policy/abi-and-versioning/) page states
the rule; `CONTRIBUTING.md` explains how to propose a change.
