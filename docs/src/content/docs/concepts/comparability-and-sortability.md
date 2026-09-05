---
title: Comparability and sortability
description: Two durable facts a query layer reads from the registry, when two scalars may be compared and whether a column may be ordered.
sidebar:
  order: 3
---

Two fields of scalar metadata exist for query engines built on top of the
registry rather than for parsing. A query layer that lets users filter and
sort typed columns needs to know which comparisons are meaningful and which
columns have an order at all, and it needs those answers to be stable,
because a query saved today runs again tomorrow against the registry of
tomorrow. Both facts are emitted into every binding's metadata table
(`SCALAR_METADATA` in Rust, the equivalent generated table in Go, Python and
TypeScript) and into the conformance corpus, so every consumer reads the same
values.

## Comparability class

`comparability_class` is the equivalence class two scalars must share before a
comparison or join between them is meaningful. The value every built-in but
two carries is `None`, which means self-comparable only: `Contact.Email =
Contact.Email` is legal, `Contact.Email = Contact.PhoneNumber` is not, even
though both are stored as strings. A named class exists for scalars that
legitimately compare across types. The first and so far only one is
`temporal_instant`, over `Temporal.Date` and `Temporal.DateTime`: both are
points on the timeline, so a comparison between them means something. A class
is admitted only when a comparison between its members is semantically
meaningful, judged on the scalars' declared intent, not on a shared primitive
or on one pattern happening to subsume another. These are equivalence classes,
not a subtype lattice.

The relation is a durable contract. A saved query that compares two columns
validated against it when it was saved; tightening a class later breaks that
query on its next run. Classes are easy to add and painful to remove, so add
one only with that cost in view.

The field is unversioned, and that is safe for a specific reason. From an
all-`None` baseline, assigning a class can only loosen: `None` is
self-comparable only, so a named class adds legal comparisons and never
removes one. A ruleset version becomes load-bearing only at the first class
removal or narrowing, and none has happened. Do not add a private version
scheme before then.

The set of class names is deliberately open: free-form text, no enum, no
constant list. One class is a thin basis for freezing a four-language
vocabulary, and tightening it after values exist is a codegen break plus a
corpus rewrite. Two tests bound what an open set can silently get wrong: no
class may have exactly one member (a singleton is indistinguishable from
`None` and is almost always a typo), and no class may span two
`PrimitiveKind`s. A third bound lives outside Rust, because those two cannot
catch a class attached to the wrong scalar: `meta.comparability_classes` in
the conformance corpus is hand-maintained beside the generated `metadata`
section and pins each class name to its exact member list, and every binding
derives the same map from the corpus rows and asserts it.

## Comparability predicate

The class is data; `comparable_with` is the contract, and every binding that
ships the data ships the predicate: `Registry::comparable_with` (Rust),
`ComparableWith` (Go), `comparableWith` (TypeScript), `comparable_with`
(Python). All four take two scalars and answer one question, may these be
compared or joined, by the same rule: aliases resolve first, a scalar is
always comparable with itself, and two distinct scalars are comparable only
when both declare the same named class.

The three binding copies are generated from the codegen templates, one alias
table plus one predicate body each; a hand-maintained copy in three languages
is exactly the mirror the library avoids.

Do not reimplement it as class equality. `a.comparability_class ==
b.comparability_class` answers true for two class-less scalars, and
class-less is what almost every scalar is, so field equality makes the whole
catalog mutually comparable, `Contact.Email` against `Contact.PhoneNumber`
included, the one comparison the relation exists to reject. Go is the
sharpest case: it encodes absent as `""`, which is also the zero value a
`ScalarMetadataByCanonical` miss yields, so field equality there cannot tell
"no class" from "no such scalar". Each binding carries the negative test that
catches this.

The bindings fail closed where the core does not, and the difference is
deliberate. The core answers over `ScalarDef`, so every registered scalar has
an answer. The bindings answer over canonical name strings against the
metadata table, which excludes any scalar whose def sets `metadata_omit`, so
the predicate reports false for such a name and for any unknown name, even
against itself. Same direction as the `is_sortable` allowlist: a shape nobody
declared answers "no".

## Doctored fixtures use a value the catalog cannot produce

Several codegen tests prove that a field travels from `ScalarDef` through the
shaper into the Go, TypeScript and Python templates by doctoring one entry and
re-emitting. Those tests were first written with the placeholder
`"temporal_instant"`, which later shipped as a real class name, and both
stopped discriminating on the spot: the render now contained genuine
occurrences, so one count assertion broke loudly and one unscoped `contains`
check kept passing while asserting nothing.

So doctor with a value the catalog cannot produce. `never_a_real_class` is the
one in use: it matches the `[a-z0-9_]+` name domain, so it is a legal class
shape, and no catalog row carries it. Raising an expected count to absorb the
real rows is the wrong repair; it turns a template-wiring test into a
hand-maintained mirror of the catalog's contents. The rule is about tests that
render against the live registry; a table test that takes its rows explicitly
never reads the catalog and is free to use a realistic name as a fixture.

## Sortability

`is_sortable` says whether a column of this scalar may carry an `ORDER BY`. It
is derived, never declared, and the rule is about JSON shape, not meaningful
order: a scalar is sortable when its `json_schema_type` is `string`,
`integer`, `number` or `boolean`, and not sortable when it is `object`,
`array`, empty, or anything else.

It is not a claim that the order means something. `Design.Color`,
`Identity.UUID` and `Network.IpAddress` are all sortable, and lexicographic
order over each is equally meaningless. It is a claim that a total order
exists. `Embedding.Vector`, `Generic.JSON` and `Generic.StringMap` show the
gap the rule closes: their primitive is `String` but their JSON shape is an
array or an object, so a primitive-only rule would call them sortable. Among
the 44 built-ins, four are non-sortable: `Geo.Location` (the structural
scalar) and those three.

The rule is an allowlist and fails closed on purpose. A shape nobody has
declared, or a mistyped one, answers "not sortable". That direction matters
because a downstream data SDK may generate a sort method only for sortable
columns and enforce the same predicate on its read path, and withdrawing a
generated method is a breaking change. The obligation therefore runs the
other way too: a scalar whose payload is JSON, a vector, or otherwise has no
total order must declare `json_schema_type` as `object` or `array`. Three
tests catch a mistake: the declared domain is closed, every `Structural`
scalar and every `Object`-primitive scalar declares `object`, and every scalar
with a metadata row declares something non-empty.

`is_sortable` is a durable contract in the same sense as the comparability
relation. Flipping a scalar from
sortable to non-sortable removes a generated method and is a breaking change;
the safe direction is non-sortable to sortable.

## Absent metadata

Every built-in scalar has a metadata row. The extension model lets a
definition set `metadata_omit`, which excludes it from the metadata table and
from the corpus `metadata` section; an extension might do that for a scalar
whose value must never be echoed. For such a scalar there is no sortability
answer in any binding, and a lookup must handle absence: Go returns the zero
value with `ok == false`, Python raises `KeyError`, TypeScript throws on
`undefined`. Sortability is a shape question, never an authorization one;
secret-bearing built-ins such as `Auth.Password`, `Auth.JWT` and
`Crypto.RSAPrivateKey` declare `string` and sort under the same rule as
everything else.
