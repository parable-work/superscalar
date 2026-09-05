---
title: The registry
description: The single source of truth for which scalars exist, what each one is called, and how it behaves.
sidebar:
  order: 1
---

The registry is the one place that says which scalars exist and how each one
behaves. Every binding, the code generator and the docs generator read it;
nothing else defines a scalar. If you want to know whether a name is a scalar,
what its id is, or what type it maps to in Go, the answer is a registry entry.

It is two Rust files in the core crate. `registry.rs` holds the `ScalarDef`
struct, the `PrimitiveKind` and `ScalarTag` enums and the `Scalar` trait.
`catalog.rs` holds the `ScalarId` type and the 44 built-in definitions.

## A definition

Each `ScalarDef` records, among other fields:

- `id`, a stable `u32`, and `canonical`, the dotted name (`Contact.Email`).
  Both are frozen once published; see
  [ABI and versioning](/superscalar/policy/abi-and-versioning/).
- `primitive` (`String`, `Int`, `Float`, `Bool`, `Object`), the storage
  primitive, plus `sql_type` (`CITEXT` for `Contact.Email`) and
  `json_schema_type`.
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
- `alias_of`, for a scalar that shares another scalar's implementation.
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

## Ids are frozen, holes are permanent

Built-in ids are `u32` values in the block `0..=4095`. The 44 built-ins do
not occupy `0..=43`: the ids `0` to `4`, `7`, `30` to `38`, `51` and `57` are
unused. Those ids belonged to scalars that were removed from the built-in set
when the library was extracted and now live in an extension, and they are
never reused. The next free built-in id is `61`.

The reason is byte compatibility. Generated bindings and stored data carry
the numeric id, and a renumbering would change what an existing consumer
means by `8`. Holes cost nothing.

## Namespaces

The part of the canonical name before the dot is the namespace: `Contact`,
`Identity`, `Temporal`, `Network`, and so on. It groups scalars in the
generated reference and in the codegen output. Under the open registry below,
a definition carries its namespace as a field, and assembly rejects one that
does not equal the prefix of the canonical name.

## The open registry

The registry described above is closed: the built-in catalog is the only
catalog. The extension model opens it so a downstream project can add scalars
in its own crate. The design is agreed and is being implemented; the shape is:

- `ScalarId` becomes a newtype `ScalarId(pub u32)` with associated constants
  for the built-ins (`ScalarId::CONTACT_EMAIL`), keeping every current value.
- An `Extension` trait supplies a name, an `id_base` that is a multiple of
  4096 and at least 4096, a static slice of `ScalarDef`s, hand-written
  `Scalar` implementations, and optional legacy aliases for generated
  symbols.
- `Registry::builtin()` returns the 44 built-ins.
  `Registry::assemble(&[&dyn Extension])` returns built-ins plus extensions
  and panics on any conflict: overlapping ids, duplicate canonical names, a
  definition outside its extension's block, a `CustomLogic` definition with
  no implementation, or a pattern that does not compile.
- Lookups (`def`, `by_canonical`, `scalar`) move from free functions onto the
  `Registry` value, and the `Scalar` trait hooks receive the registry so a
  scalar can validate against the assembled catalog rather than the built-in
  one.
- `Registry::dump()` serialises the assembled registry to JSON in a stable
  field order. The code generator and the docs generator consume the dump.

The [build an extension](/superscalar/guides/build-an-extension/) guide
walks through the example extension that exercises all of this. Points the
design has not closed are marked there.
