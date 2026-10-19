---
title: TypeScript
description: Install the superscalar npm package for Node (native addon), browsers and Cloudflare Workers (WASM) behind one import.
sidebar:
  order: 4
---

The TypeScript binding is the npm package `superscalar`. One import works in
Node, in browsers and in Cloudflare Workers: Node loads a native addon built
with napi-rs, and browsers and Workers load a WebAssembly build of the same
core. The public functions, the branded types and the behaviour are identical
on every path, and the conformance corpus runs against each in CI: the
browser build in Node, the Workers build in workerd.

## Install

Nothing is on npm yet. From v0.1.0:

```
npm install superscalar
```

The main package depends on four optional platform packages under the
`@superscalar` scope (`@superscalar/darwin-arm64`, `@superscalar/darwin-x64`,
`@superscalar/linux-x64-gnu`, `@superscalar/linux-arm64-gnu`) that carry the
native addon; npm installs only the one whose os, cpu and libc match. Never
depend on a platform package directly: the main package requires the right
one for the host at load time. Pre-releases are published under the `next`
dist-tag (`npm install superscalar@next`). The main package also ships a WASM
build of the same core; when no platform package is installed (another
platform, musl, or optional dependencies skipped), Node falls back to it. If
neither backend loads, the error names both attempts. A bundle that inlines
the backend module finds the WASM build through the package name
(`superscalar/wasm-node/superscalar_wasm.js`) when the relative path no
longer points into the package.

## Node, browsers and Workers

Selection is by the package's `exports` map, on every entry (`superscalar`,
`superscalar/scalars`, `superscalar/validation`, `superscalar/backend` and
`superscalar/wasm`):

| Condition | Build | Backend |
| --------- | ----- | ------- |
| `workerd` | `dist/workerd/` | WebAssembly, the module imported as `./superscalar_wasm_bg.wasm` |
| `browser` | `dist/browser/` | WebAssembly, the module inlined as base64 |
| anything else (Node, Bun) | `dist/esm/`, `dist/` | the native addon, else the WASM build in `wasm-node/` |

`workerd` comes before `browser`, so wrangler, which sets both, takes the
Workers build. Neither build imports anything from Node, and each holds every
file it needs in its own directory.

Both WebAssembly builds instantiate synchronously when they are imported, as
the Node backend loads, so every function works as soon as the import
resolves; there is nothing to await.

- **Browsers.** The module rides inline in the JavaScript, so a bundler takes
  it as plain JavaScript, with no WebAssembly or asset support and no plugin;
  CI bundles with `bun build --target=browser`. The page compiles it
  synchronously on the main thread, which Chrome allows for modules up to 8 MB
  (since 2023; the module is 2.2 MB). A Content-Security-Policy must allow
  `'wasm-unsafe-eval'`.
- **Cloudflare Workers.** workerd compiles no WebAssembly from bytes at run
  time, so the Workers build imports the module as a `.wasm` file, which a
  Worker's upload carries as a compiled module. wrangler does this with no
  configuration. With another bundler, keep the import external and upload the
  file beside the bundle as a wasm module named `superscalar_wasm_bg.wasm`:

  ```
  bun build worker.ts --target=browser --conditions=workerd --external '*.wasm' --outdir out
  cp node_modules/superscalar/dist/workerd/superscalar_wasm_bg.wasm out/
  ```

  A bundler that turns the import into a file path instead (bun without
  `--external`) fails at startup with an error that says so.

Sizes, from the `wasm-release` profile (opt-level `"s"`, LTO):

| Artifact | Raw | Gzipped |
| -------- | --- | ------- |
| WebAssembly module | 2.2 MB | 0.66 MB |
| browser bundle of a small site (`bun build --target=browser`) | 3.0 MB | 1.0 MB |
| wrangler upload of a small Worker | 2.2 MB | 0.68 MB |

A Worker may be 64 MiB uncompressed on the Free and Paid plans.

`superscalar/wasm` exposes the raw WASM exports (`scalar_parse` and friends)
for a caller that wants that build explicitly; in a browser or a Worker they
are already instantiated. Its `scalar_parse` throws an error named
`ScalarParseError` when the core rejects the input, and a plain `Error` for an
unknown scalar name, so a caller can tell an invalid value from a runtime
failure.

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

`Generic.JSON` is the one exception, because `null` is one of its values:
its lenient `parse` and `normalize` take an already-decoded JSON value and
return `undefined`, not `null`, for an absent or non-portable input (`NaN`, a
cycle, a `Set`). The strict forms take JSON text. `JSONValue` and the guard
`isJSONValue` are exported beside them.

Branded types are plain strings at runtime with a phantom `__brand` field, so
a `ContactEmail` cannot be passed where an `IdentityUUID` is expected without
going through a parse function.

A scalar whose type is not a string comes back as that type on every path.
A `number` scalar (`OrderingRank`, `FinanceMoney`) returns a number, and
`EmbeddingVector` returns a `number[]`, from both the lenient and the strict
wrappers, though the core hands back canonical text.
