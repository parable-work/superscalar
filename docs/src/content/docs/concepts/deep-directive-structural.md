---
title: Deep, directive and structural scalars
description: The three ways a scalar's rule is implemented, and which built-ins fall into each.
sidebar:
  order: 2
---

Every scalar exposes the same three operations, but the rule behind them is
implemented in one of three ways. The registry records the choice in the
definition's `tag` field, and the kind decides what you have to write when
you add a scalar.

## Directive scalars

Tag `PatternOnly`. A directive scalar is fully described by its definition:
a regular expression, length bounds, numeric bounds, case-insensitivity and
reserved words. It has no hand-written code. The core builds a generic
validator from those fields at assembly time, so the definition is the whole
rule.

`parse` on a directive scalar is strict: it validates, then returns the
canonical form (an integer rewritten as plain decimal, anything else as
given). `normalize` does not validate. On a `case_insensitive` scalar it
lowercases an input that fails validation when the lowercased form passes,
so `Identity.Slug` normalizes `ACME` to `acme` while `parse("ACME")` still
fails. An input that is already valid is returned unchanged, which is why
`Temporal.Quarter` keeps `Q1`.

30 of the 48 built-ins are directive scalars, including `Auth.JWT`,
`Identity.Slug`, `Network.IpAddress`, `Ordering.Rank`, `Temporal.Time`,
`Text.Markdown` and `Version.SemVer`.

Adding one is a registry entry plus conformance vectors; there is no module
to write.

## Deep scalars

Tag `CustomLogic`. A deep scalar needs code beyond a pattern: `Design.Color`
converts CSS colour syntax to RGBA, `Contact.PhoneNumber` parses phone
numbers with a phone-number library, `Temporal.Duration` and
`Temporal.DateTime` parse grammars, `Identity.UUID` accepts hyphenated and
base62 forms and canonicalises to base62, `Temporal.RecurrenceRule` parses
recurrence rules. The rule lives in one Rust module under
`crates/core/src/scalars/`, implemented once and called by every binding.

18 of the 48 built-ins are deep scalars: `Contact.Email`,
`Contact.PhoneNumber`, `Design.Color`, `Embedding.Vector`, `Generic.JSON`,
`Generic.StringMap`, `Geo.Location`, `Git.PathPattern`, `Identity.UUID`,
`Identity.UserID` (an alias of `Identity.UUID`), `Network.Url`,
`Network.DnsLabel`, `Temporal.Date`, `Temporal.DateTime`, `Temporal.Duration`,
`Temporal.Month`, `Temporal.QuarterYear` and `Temporal.RecurrenceRule`.

A deep scalar whose value is JSON keeps the `String` primitive, because its
input and canonical form are JSON text, and declares the value's shape in
`json_schema_type`: `object` for `Generic.StringMap` and `Geo.Location`,
`array` for `Embedding.Vector`, `any` for `Generic.JSON`. Such a scalar
declares no `pattern` or length bounds, which are rules on a string; a schema
toolchain that sees one treats the value as a string. `Generic.StringMap` and
`Geo.Location` also set the `parse` hook, so a generated runtime hands the
value to the core and decodes the canonical JSON it returns into a native
object.

A deep scalar may still declare a `pattern` in its definition. `Contact.Email`
does; the pattern documents the shape and feeds the generated reference,
while the module decides acceptance.

Assembly enforces the tag: a `CustomLogic` definition with no registered
implementation fails, and a `PatternOnly` definition with one also fails.
That guard is what stops a hand-written scalar from silently falling back to
the generic validator.

## Structural scalars

Tag `Structural`. A structural scalar is object-shaped: its value is a JSON
object with a defined set of fields rather than a string with a pattern. Like
every scalar it crosses the C ABI as text, here the object's JSON. The
generated bindings give a structural scalar no validator (`HasValidator` and
`hasValidator` are false, and Go has no `Validate` method for it).

No built-in is structural: `Geo.Location`, an object, is a deep scalar so
that every binding validates it through the core (its module and value type
are under `crates/core/src/metadata/`). The extension model keeps the
`Structural` tag and the `file_upload` and `image_constraints` definition
fields for structural scalars an extension adds, such as file or image
metadata with upload limits.

Structural scalars are never sortable, because a JSON object has no total
order. See
[comparability and sortability](/superscalar/concepts/comparability-and-sortability/).

## Which kind to pick

If the accept set can be written as a regular expression plus bounds, make a
directive scalar. If acceptance depends on parsing (dates, durations,
numbers with units, encodings), make a deep scalar; that includes a JSON
value the core checks, such as `Geo.Location`'s object. If the value is a
record the bindings carry without a validator, make a structural scalar. The
[add a built-in scalar](/superscalar/guides/add-a-builtin-scalar/) guide
covers each path.
