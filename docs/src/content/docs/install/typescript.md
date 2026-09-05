---
title: TypeScript
description: Install the superscalar npm package for Node (native addon) and browsers (WASM) behind one import.
sidebar:
  order: 4
---

The TypeScript binding is the npm package `superscalar`. One import works in
Node and in browsers: Node loads a native addon built with napi-rs, and
browsers and edge runtimes load a WebAssembly build of the same core. The
public functions, the branded types and the behaviour are identical on both
paths, and the conformance corpus runs against both in CI.

## Install

Nothing is on npm yet. From v0.1.0:

```
npm install superscalar
```

The main package depends on per-platform optional packages
(`@superscalar/darwin-arm64` and the like) that carry the native addon; npm
installs only the one for your platform. The main package also ships a WASM
fallback, so a platform without a prebuilt addon still works in Node.

## Node or browser

Selection is by the package's `exports` map. The `browser`, `edge-light` and
`workerd` conditions resolve the backend module to the WASM build; everything
else resolves to the native addon. Bundlers and runtimes that honour those
conditions need no configuration. You never import a backend directly.

## Quickstart

```ts
import {
  parseContactEmail,
  parseContactEmailStrict,
  normalizeContactEmailStrict,
  validateIdentityUUID,
  type ContactEmail,
} from "superscalar";

const email: ContactEmail | null = parseContactEmail(" User@Example.COM ");
// "user@example.com"; null when the input is rejected

const strict: ContactEmail = parseContactEmailStrict("user@example.com");
// throws when the input is rejected

const loose = normalizeContactEmailStrict("  Not An Email  ");
// "not an email": trimmed and lower-cased, shape not enforced

const [ok, errors] = validateIdentityUUID("0f8fad5b-d9cb-469f-a165-70867728950e");
// ok: boolean; errors: ValidationError[] | null
```

Every scalar has the same generated surface. `parse<Name>(value: unknown)`
returns the branded type or `null`; `parse<Name>Strict(value: string)`
returns the branded type or throws; `normalize<Name>` has the same two
forms. The two normalize forms differ in more than error handling: the
lenient `normalize<Name>` runs the input through the core's lenient coercion
and returns `null` for a rejected shape, while `normalize<Name>Strict` calls
the core's `normalize` directly and does not enforce shape.
`validate<Name>(value)` returns a `[boolean, ValidationError[] | null]`
tuple. `<Name>` is the canonical name with the dot removed (`ContactEmail`,
`IdentityUUID`).

Branded types are plain strings at runtime with a phantom `__brand` field, so
a `ContactEmail` cannot be passed where an `IdentityUUID` is expected without
going through a parse function.
