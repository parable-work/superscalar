---
title: The registry
description: The single source of truth for which scalars exist, what each one is called, and how it behaves.
sidebar:
  order: 1
---

The registry is the one place that says which scalars exist and how each one
behaves. Every binding, the code generator and the docs generator read it;
nothing else defines a scalar. If you want to know whether a name is a scalar,
what it accepts, or what type it maps to in Go, the answer is a registry entry.

It is three Rust files in the core crate. `registry.rs` holds the
`ScalarDef` struct, the `PrimitiveKind` and `ScalarTag` enums, the `Scalar`
trait and `Registry`. `catalog.rs` holds the `names` constants and the 48
built-in definitions. `definitions.rs` holds `Definitions`, the assembled
definitions without implementations, and the comparability rule.

## A definition

Each `ScalarDef` records, among other fields:

- `canonical`, the dotted name (`Contact.Email`). It is the scalar's only
  identity and is frozen once published; see
  [ABI and versioning](/superscalar/policy/abi-and-versioning/).
- `primitive` (`String`, `Int`, `Float`, `Bool`, `Object`), the storage
  primitive, plus `sql_type` (`CITEXT` for `Contact.Email`) and
  `json_schema_type`. `PrimitiveKind::ALL` lists every member, and each one
  carries two spellings: `dsl_name` for a schema DSL (`Bool`, `Type`) and
  `type_ref_name` for a type reference in an emitted schema IR (`Boolean`,
  `JSON`), with `from_dsl_name` and `from_type_ref_name` as the inverses. The
  Go binding emits both as `PRIMITIVE_KINDS` and
  `PrimitiveDSLNameByTypeRefName`, so no consumer restates the mapping.
- `tag`: `PatternOnly`, `CustomLogic` or `Structural`, which decides how the
  scalar is implemented. See
  [scalar kinds](/superscalar/concepts/deep-directive-structural/).
- The declarative rules a directive scalar is built from: `pattern`,
  `min_length`, `max_length`, `minimum`, `maximum`, `case_insensitive`,
  `reserved_words`.
- `examples` and `description`, which the generated reference and the docs
  check read. A scalar with an empty description fails `superscalar docs
  --check` (part of the CLI being implemented for v0.1.0).
- `type_mappings`, the type the scalar becomes in each target: for
  `Contact.Email`, `string` in TypeScript, `str` in Python, `string` in Go,
  `String` in Rust, `CITEXT` in SQL and `string` in JSON Schema.
- `alias_of`, the name of the scalar whose implementation this one shares.
  The one built-in alias is `Identity.UserID`, which resolves to
  `Identity.UUID`.
- `comparability_class`, an optional equivalence class for cross-scalar
  comparison. See
  [comparability and sortability](/superscalar/concepts/comparability-and-sortability/).
- `file_upload` and `image_constraints`, optional limits a structural scalar
  may declare. No built-in sets them; the fields exist so an extension can.

The `Scalar` trait is the behaviour side: `parse`, `normalize` and
`validate`, each taking the input string. A hand-written implementation
exists for deep and structural scalars; directive scalars get a generic
implementation built from the definition's rules.

## The name is the identity

A scalar is identified by its canonical name and nothing else. The C ABI,
the WASM, napi and PyO3 exports, the generated wrappers and the conformance
vectors all name a scalar by it, and stored data records it. Names are
append-only: a published name keeps its meaning and is never renamed or
reused. In Rust, `names::CONTACT_EMAIL` and its siblings are constants for
the built-in names, so a misspelled built-in fails to compile; lookups take
any `&str`.

Every lookup is exact and case-sensitive. `Registry::names()` and
`Registry::defs()` iterate in name order, and so do the generated files and
the dump, so where a definition sits in the catalog carries no meaning.

Earlier versions also gave each scalar a frozen `u32` id. Nothing stored the
id, every durable consumer already keyed on the name, and a second identity
let two contributions collide on a number while meaning different scalars.
It was removed before the first release.

## Namespaces

The part of the canonical name before the dot is the namespace: `Contact`,
`Identity`, `Temporal`, `Network`, and so on. It groups scalars in the
generated reference and in the codegen output. Under the open registry below,
a definition carries its namespace as a field, and assembly rejects one that
does not equal the prefix of the canonical name.

## The open registry

The built-in catalog is not the only one: a downstream project can add
scalars in its own crate.

- An `Extension` trait supplies a name, a static slice of `ScalarDef`s,
  hand-written `Scalar` implementations keyed by canonical name, and optional
  legacy aliases for generated symbols.
- `Registry::builtin()` returns the 48 built-ins.
  `Registry::assemble(&[&dyn Extension])` returns built-ins plus extensions
  and panics on any conflict: a canonical name two contributions declare, a
  `CustomLogic` definition with no implementation, or a pattern that does not
  compile, among others.
- Lookups (`def`, `scalar`, `owner`, `resolved`) live on the `Registry`
  value, and the `Scalar` trait hooks receive the registry so a scalar can
  validate against the assembled catalog rather than the built-in one.
- `Registry::dump()` serialises the assembled registry to JSON in a stable
  field order. The code generator and the docs generator consume the dump.
- `Definitions` is the same catalog without implementations; see below.

The [build an extension](/superscalar/guides/build-an-extension/) guide
walks through the example extension that exercises all of this.

## Definitions without implementations

`Registry` holds an implementation for every scalar, and assembling one calls
the built-in implementation table and every extension's `impls()`. A
consumer that only reads definitions pays for all of it anyway: the
phone-number library with its metadata, the regex engine and every
hand-written scalar are linked into its binary. For a WASM bundle with a size
budget that is the difference between fitting and not.

`Definitions` is the definitions-only view. It is assembled from static
`ScalarDef` slices, never from an `Extension`, so no implementation is
reachable from it:

```rust
use superscalar::Definitions;

// The built-ins.
let builtin = Definitions::builtin();
let email = builtin.def("Contact.Email").expect("built-in");

// The built-ins plus an extension's defs, each as an (owner, defs) pair.
let assembled = Definitions::assemble(&[("acme", &acme_scalars::DEFS)]);
assert!(assembled.comparable_with(email.canonical, email.canonical));
```

It answers `def`, `resolved`, `comparable_with`, `names`, `defs`, `len` and
`is_empty` with exactly the semantics of the `Registry`
methods of the same names. That is by construction: a `Registry` assembles a
`Definitions` first and delegates every one of those lookups to it
(`Registry::definitions()` returns it), so the lookup and comparability rules
live in one place. The free function `scalar_def` reads
`Definitions::builtin()` for the same reason.

`Definitions` assembly runs the checks that need only definitions, in the
same order and with the same `AssemblyError` as `Registry` assembly:
duplicate canonical names, a namespace that is not the canonical prefix, and
a dangling or chained alias. Pattern compilation, the implementation checks
and extension naming need implementations or a compiled pattern and stay
with `Registry`, so a set of
definitions can assemble here and still be refused by `Registry`; validate
the full extension through `Registry` in its own tests.

