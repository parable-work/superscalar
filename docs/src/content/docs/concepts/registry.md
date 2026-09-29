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
  --check` (`make docs-check`).
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
generated reference and in the codegen output. A definition also carries it
in its `namespace` field, and assembly rejects one that does not equal the
prefix of the canonical name.

## The open registry

A downstream crate adds scalars by implementing the `Extension` trait
(`extension.rs`):

- `name()`, the owner name (`"acme"`). It must be unique within an assembly
  and must not be `"builtin"`.
- `defs()`, a `&'static [ScalarDef]` declared the same way as the built-in
  catalog.
- `impls()`, hand-written `Scalar` implementations, each keyed by the
  canonical name of a def the extension declares. A def with no entry runs
  on the directive engine, built from its rules; an alias shares its
  target's implementation.
- `aliases()`, optional `LegacyAlias` entries: flat names the generated
  bindings alias to a canonical symbol (`Email` to `ContactEmail`). Empty by
  default.

`Registry::builtin()` is the 48 built-ins, assembled on first use.
`Registry::assemble(&[&dyn Extension])` assembles the built-ins plus the
given extensions and panics if a check fails; `Registry::try_assemble` runs
the same checks and returns the `AssemblyError` instead. The checks run in
this order and stop at the first failure:

1. An extension name repeats or is `"builtin"` (`DuplicateExtensionName`).
2. Two contributions declare the same canonical name (`DuplicateCanonical`).
3. A `namespace` is not the canonical prefix (`NamespaceMismatch`).
4. An `alias_of` names no assembled scalar (`DanglingAlias`).
5. An `alias_of` names another alias (`AliasChain`).
6. An extension registers an implementation for a name it does not declare
   (`ForeignImpl`).
7. After alias resolution, a def whose tag is not `PatternOnly` has no
   implementation (`MissingImpl`), or a `PatternOnly` def or an alias
   registers one (`UnexpectedImpl`).
8. A `pattern` does not compile (`InvalidPattern`).
9. A legacy alias targets a symbol no assembled scalar generates
   (`DanglingLegacyAlias`).

An assembled registry is read-only. It answers `def`, `scalar`, `owner`
(`"builtin"` for a built-in), `resolved`, `comparable_with`, `names`, `defs`,
`len`, `extensions` and `legacy_aliases`. The `Scalar` methods `parse`,
`normalize` and `validate` receive the registry, so a scalar can check input
against the assembled catalog rather than the built-in one. The free
functions `scalar_def` and `scalar_for` cover the built-ins only and panic on
any other name.

`Registry::dump()` returns the assembled registry as JSON with a fixed field
order: `dump_version` (2), `superscalar_version`, `extensions`, `scalars`
(each def's fields plus its owner, `is_sortable` and `is_directive`, sorted
by canonical name) and `legacy_aliases`. `superscalar registry dump`
(`make dump`) prints the built-in registry's dump; the acme example prints
its own and its vector runners read the scalar list from it. The code
generator and the docs generator read the `Registry` itself, not the dump.

The [build an extension](/superscalar/guides/build-an-extension/) guide
walks through `examples/acme-scalars/`, which exercises all of this.
`Definitions`, below, is the same catalog without implementations.

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

`Definitions` assembly runs checks 2 to 5 from the list above, the ones that
need only definitions, in the same order and with the same `AssemblyError`
as `Registry` assembly. Checks 1 and 6 to 9 need extension names,
implementations, compiled patterns or legacy aliases and run only in
`Registry` assembly, so a set of definitions can assemble here and still be
refused by `Registry`; validate the full extension through `Registry` in its
own tests.

