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

`conformance/core-scalars.v2.json` holds the vectors for the 48 built-in
scalars. Its top level:

- `meta`: `version` (2), `source_language` (`rust-core`), a description, and
  four hand-maintained expectations checked against the `metadata` section:
  `non_sortable_count`, `comparability_classes` (each class and its sorted
  members), `case_insensitive_scalars` (the sorted names of the rows with
  `case_insensitive` set) and `reserved_word_scalars` (each row that reserves a
  word or sets either reserved-word flag, with those three fields).
- `scalars`: a map from canonical name to `{ accepted, rejected }`.
- `metadata`: a map from canonical name to `{ comparability_class,
  is_sortable, case_insensitive, reserved_words,
  reserved_words_case_insensitive, reserved_words_match_partial }`, a
  transcript of the generated metadata table. Regenerate it with
  `python3 conformance/gen_core_scalars_metadata.py` after running the code
  generator.
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

Each runner iterates `scalars` and calls the binding's parse with the
canonical name, the scalar's only identity. A key the binding does not know
fails the run (`KnownScalar` in Go, `VALID_SCALARS` in TypeScript and Python,
`Registry::scalar` in Rust), which is how a vector file from the wrong
assembly is caught.

Beyond parse, the runners assert the metadata section: every scalar in
`scalars` has a row in `metadata` or is listed in `metadata_excluded`, every
field of each row matches the binding's table, and the expectations in `meta`
agree with what the rows give: the count of non-sortable rows, the class
members, the case-insensitive scalars and the reserved-word settings.

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

The Go, Python and TypeScript runners live in `go/conformance_test.go`,
`bindings/python/tests/test_conformance.py` and
`bindings/typescript/test/conformance.cjs`. Each prints a pass and fail
count per bucket.

## Running against your own build or extension

The Go, Python and TypeScript runners above read one fixed path,
`conformance/core-scalars.v2.json`. An extension runs the built-in file and
its own files together through its assembled registry. The
`examples/acme-scalars/` extension does it in two places, each over
`conformance/core-scalars.v2.json` plus
`examples/acme-scalars/conformance/acme-scalars.v2.json`:

- `ext/tests/conformance.rs` merges the two files and runs every vector
  through the assembled registry. `scalars` keys and `metadata` keys must be
  disjoint across files, and `metadata_excluded` lists concatenate. Every
  assembled scalar must have vectors, every vector must name an assembled
  scalar, and each metadata row must match its def.
- `scripts/run_vectors.py` (C and Python) and `scripts/run_vectors.cjs`
  (napi and WASM) take a registry dump (`--registry`) and any number of
  vector files. They merge `scalars` the same way and fail when a key is in
  two files, an assembled scalar has no vectors, a vector names a scalar the
  assembly does not know, or no extension scalar was exercised. They check
  parse results only, not the metadata section. `scripts/smoke.sh`
  (`make acme`) builds each binding and runs them over both files.

A vector file left off the list fails the run, because its scalars then have
no vectors. No multi-file runner reads `non_sortable_count`; only the
single-file runners check it.

For your extension, the list is the built-in file at the version you depend
on plus one file per extension you assemble. Copy the acme runners and
`ext/examples/dump.rs`, point them at your registry and files, and run them
in CI against each binding you ship. The acme example has no Go runner; a
generated Go package needs its own test over the same list. If the built-in
vectors fail through your assembly, the fault is in how the registry was
assembled or how the binding was built, not in the built-ins.

## Vectors are the versioning unit

A pull request that adds, removes or edits a vector, or changes which inputs a
scalar accepts, is a versioned behaviour change: loosening is a minor bump,
tightening is a major bump, and either needs a changelog entry. The
[ABI and versioning](/superscalar/policy/abi-and-versioning/) page states
the rule; `CONTRIBUTING.md` explains how to propose a change.
