---
title: Scalar reference
description: One page per scalar, generated from the registry and the conformance vectors.
sidebar:
  order: 0
---

The pages in this section are generated. `superscalar docs` reads the
registry dump and the conformance corpus and writes one Markdown page per
scalar into this directory before the site is built. Each page carries the
scalar's canonical name and id, its primitive, SQL and JSON Schema types,
its pattern and bounds, examples, accepted and rejected inputs from the
corpus, its type mapping in each language and the generated function names.

Only this index is committed. The per-scalar pages are written in CI on every
deploy and are gitignored, so a local build without the generator shows this
page alone. To build the full reference locally, run the generator into
`docs/src/content/docs/reference/` first; the `docs/README.md` in the
repository has the command.

Because the pages come from the registry, a scalar cannot be missing from the
reference, and a scalar with no vectors or no description fails `superscalar
docs --check` in CI rather than shipping an empty page.

If a generated page is wrong, fix the registry entry or the vectors and
regenerate; do not edit the page.
