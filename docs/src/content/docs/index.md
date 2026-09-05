---
title: SuperScalar
description: Cross-language scalar types with one Rust implementation and Go, Python, TypeScript and WASM bindings.
---

SuperScalar is a library of named scalar types such as `Contact.Email`,
`Identity.UUID`, `Temporal.DateTime` and `Design.Color`. Each scalar's rule is
written once, in Rust, and every language binding calls that one
implementation. An email address therefore parses the same way in a Go
service, a Python job, a Node process and a browser tab, and a corpus of
conformance vectors proves it on every build.

The library was extracted from Parable's platform monorepo; the git history
stayed behind, so this repository starts from the extracted tree.

Status: pre-release. Package and crate names are fixed (see the install pages)
but nothing has been published to crates.io, npm, PyPI or as a Go module yet,
and APIs may change until v0.1.0. The `superscalar` CLI (`codegen`, `docs`,
`registry dump`) and the `Registry` API described on these pages are being
implemented; where today's names differ, the page says so.

## What a scalar is

A scalar is a string-shaped value with a name and a rule. The name is
canonical and dotted, `Namespace.Name`, and it never changes. The rule says
which inputs are accepted and what the accepted input looks like once it is
normalized. Every scalar also carries a stable numeric id (a `u32`), the
primitive it is stored as, its SQL and JSON Schema types, and the type it maps
to in each binding language. That record is the scalar's definition in the
registry; see [the registry](/superscalar/concepts/registry/).

There are 44 built-in scalars. They fall into three kinds by how their rule is
implemented: directive scalars are fully described by a pattern and length
bounds, deep scalars carry hand-written logic (colour math, phone numbers,
durations, base62 UUIDs), and structural scalars are object-shaped. The
[scalar kinds](/superscalar/concepts/deep-directive-structural/) page
explains the split.

## The contract: parse, normalize, validate

Every scalar implements three operations over a string input.

- `parse` checks the shape of the input and returns its canonical normalized
  form, or an error.
- `normalize` moves the input toward canonical form without enforcing shape.
  A validate-only scalar returns its input unchanged; a normalize-only scalar
  does its work here, and `parse` adds the shape check on top.
- `validate` enforces shape and returns only success or an error.

Errors carry a kind (for example `Pattern`, `Length`, `Parse`) and a message.
The bindings map them onto the language's idiom: Go returns an `error`,
Python raises `ValueError`, TypeScript returns `null` from the lenient
function and throws from the `Strict` one, and Rust returns
`Result<_, ScalarError>`.

## Canonical forms

`parse` returns the one spelling the library treats as canonical, and that
spelling is pinned by a conformance vector. Two examples from the built-in corpus:

- `Contact.Email`: `" User@Example.COM "` parses to `user@example.com`. The
  input is trimmed and lower-cased; `user@example` and `user@example.c` are
  rejected by the pattern.
- `Identity.UUID`: `123e4567-e89b-12d3-a456-426614174000` parses to
  `YQJpYwUwvbaLOwTUr4thA`. The canonical form is a base62 encoding of the 128
  bits; the hyphenated form is accepted as input, and the nil UUID parses to
  `0`.

Because the canonical form is part of the contract, changing it is a
versioned change. The rules are on the
[ABI and versioning](/superscalar/policy/abi-and-versioning/) page.

## One example

TypeScript, in Node or a browser (the package selects the native addon or
the WASM build for you):

```ts
import {
  parseContactEmail,
  parseContactEmailStrict,
  validateIdentityUUID,
} from "superscalar";

parseContactEmail(" User@Example.COM "); // "user@example.com"
parseContactEmail("user@example");       // null

parseContactEmailStrict("user@example"); // throws

const [ok, errors] = validateIdentityUUID("0f8fad5b-d9cb-469f-a165-70867728950e");
// ok === true, errors === null
```

The same two scalars in the other languages are on the install pages:
[Rust](/superscalar/install/rust/), [Go](/superscalar/install/go/),
[Python](/superscalar/install/python/),
[TypeScript](/superscalar/install/typescript/).

## Where to go next

- Adding a scalar to the library:
  [add a built-in scalar](/superscalar/guides/add-a-builtin-scalar/).
- Adding scalars that belong to your project, without forking:
  [build an extension](/superscalar/guides/build-an-extension/).
- Checking that your own build behaves like the reference:
  [conformance](/superscalar/guides/conformance/).
- Why the bindings are shaped the way they are: the
  [FAQ](/superscalar/faq/).
