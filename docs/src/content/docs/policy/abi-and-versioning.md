---
title: ABI and versioning
description: What is frozen forever, what is a minor change, what is a major change, and how versions and tags are cut.
sidebar:
  order: 1
---

SuperScalar's consumers include compiled Go programs linking a static
archive, Python wheels, npm packages and stored data that carries scalar
ids and canonical values. This page states what they may rely on across
releases.

## The policy

The policy, as decided before the first release:

> Versioning: SemVer on the repo tag `vX.Y.Z`; all crates and packages share
> the version. ABI policy: u32 ids and canonical names are append-only
> forever; changing a scalar's accept set is a minor bump if loosening and a
> major bump if tightening.

Everything below follows from that.

## Frozen forever

- Scalar ids. Every scalar has a `u32` id. Once a release carries it, the id
  means that scalar in every later release. Ids are never renumbered, reused
  or deleted; the holes in the built-in block (`0` to `4`, `7`, `30` to `38`,
  `51`, `57`) are permanent.
- Canonical names. `Contact.Email` is id 8 and always will be. There is no
  rename: a scalar under a different name is a new scalar with a new id.
- The id partition. Built-ins own `0..=4095`. Extensions declare a base that
  is a multiple of 4096 and at least 4096.
- The C ABI shape. Every entry point takes a `u32` id and a UTF-8 string
  and returns a `ScalarResult`. The nine generic entry points are
  `scalar_parse`, `scalar_normalize`, `scalar_validate`,
  `scalar_coerce_lenient`, `scalar_result_free`, `scalar_parse_batch`,
  `scalar_normalize_batch`, `scalar_validate_batch` and
  `scalar_result_array_free`. Adding an entry point is a minor change;
  changing or removing one is a major change.

## Minor changes

- A new scalar, with the next free id in its block.
- Loosening a scalar's accept set: an input that was rejected is now
  accepted. Existing stored values remain valid.
- Assigning a comparability class to a scalar that had none. From an
  all-`None` baseline this only adds legal comparisons.
- Flipping a scalar from non-sortable to sortable.
- A new function on a binding's public surface.

## Major changes

- Tightening a scalar's accept set: an input that was accepted is now
  rejected. Stored values may no longer validate.
- Changing a scalar's canonical form. A value that parsed to `X` now parses
  to `Y`, so equality against stored values breaks.
- Removing or narrowing a comparability class. Saved queries that compared
  two columns under the old class break on their next run.
- Flipping a scalar from sortable to non-sortable. A downstream that
  generated a sort method from the old value loses it.
- Removing or changing a C ABI entry point, or a binding's public function.

If a scalar has a bug that makes it accept something it should not, fixing
it is a tightening and therefore a major bump, however small the input set.
File the issue, leave the vector as it is, and agree the change before
editing.

## How a change is detected

A scalar's behaviour is its conformance vectors. A pull request that adds,
removes or edits a vector, or changes which inputs a scalar accepts, must
carry a `CHANGELOG.md` entry and the matching version bump. CI runs the
corpus through every binding, so a behaviour change without a vector change
fails, and a vector change is visible in review.

## Versions and tags

One version for everything. The repository tag `vX.Y.Z` is the version of the
`superscalar` crate, its sibling crates (`superscalar-ffi`, `superscalar-wasm`,
`superscalar-codegen`, `superscalar-napi`, `superscalar-python`), the npm
package, the PyPI distribution and the Go module. The npm and PyPI versions
are set from the tag in CI, not committed. The Go module is additionally
tagged `go/vX.Y.Z` so `go get` resolves the subdirectory module.

One release job cuts every artifact from that tag and pins them to one core
commit through `manifest.json`. A set whose archives disagree on the commit is
refused, so the header, the static archives, the WASM bundle and the packages
of one version are always built from the same source.

Pre-release versions (`v0.1.0-alpha.1` and the like) carry no compatibility
promise; until v0.1.0 any of the above may change. From v0.1.0 the rules on
this page apply. How the minor-versus-major rule is applied while the major
version is still 0 has not been decided; until it is, read "major bump" as a
breaking change that the changelog calls out as such.
