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

27 of the 44 built-ins are directive scalars, including `Auth.JWT`,
`Identity.Slug`, `Network.IpAddress`, `Temporal.Time` and `Text.Markdown`.

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

16 of the 44 built-ins are deep scalars: `Contact.Email`,
`Contact.PhoneNumber`, `Design.Color`, `Embedding.Vector`, `Generic.JSON`,
`Generic.StringMap`, `Identity.UUID`, `Identity.UserID` (an alias of
`Identity.UUID`), `Network.Url`, `Network.DnsLabel`, `Temporal.Date`,
`Temporal.DateTime`, `Temporal.Duration`, `Temporal.Month`,
`Temporal.QuarterYear` and `Temporal.RecurrenceRule`.

A deep scalar may still declare a `pattern` in its definition. `Contact.Email`
does; the pattern documents the shape and feeds the generated reference,
while the module decides acceptance.

Assembly enforces the tag: a `CustomLogic` definition with no registered
implementation fails, and a `PatternOnly` definition with one also fails.
That guard is what stops a hand-written scalar from silently falling back to
the generic validator.

## Structural scalars

Tag `Structural`. A structural scalar is object-shaped: its value is a JSON
object with a defined set of fields rather than a string with a pattern. The
shape and its metadata type live in the core under
`crates/core/src/metadata/`, and the value crosses the C ABI as a bincode
buffer rather than a UTF-8 string.

`Geo.Location` is the one built-in structural scalar. The extension model
keeps the `file_upload` and `image_constraints` definition fields for
structural scalars an extension adds, such as file or image metadata with
upload limits.

Structural scalars are never sortable, because a JSON object has no total
order. See
[comparability and sortability](/superscalar/concepts/comparability-and-sortability/).

## Which kind to pick

If the accept set can be written as a regular expression plus bounds, make a
directive scalar. If acceptance depends on parsing (dates, durations,
numbers with units, encodings), make a deep scalar. If the value is a record
rather than a string, make a structural scalar. The
[add a built-in scalar](/superscalar/guides/add-a-builtin-scalar/) guide
covers each path.
