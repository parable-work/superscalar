---
title: Data components for agent-written code
description: Why one cross-language package for validation, coercion and marshaling matters more when coding agents write the code, and how an agent finds, uses and extends the catalog.
sidebar:
  order: 0
---

Most code is now drafted by a coding agent and reviewed by a person. That
changes what a scalar library is for. The cost of typing a validator has
gone to nearly nothing; the cost of trusting one has not moved. This page
explains what SuperScalar gives a project where agents write the code, and
how to point an agent at it.

## What an agent rewrites without it

Take one field, an email address, in a service with a web client. Without a
shared library an agent produces all of this:

- a pattern and a trim-and-lowercase step in the browser form;
- the same check again in the API handler, in the server's language;
- a decoder that turns the JSON string into the server's type, and an
  encoder for the way back;
- a column type and a decision about case sensitivity in the migration;
- a JSON Schema or OpenAPI fragment describing the field;
- tests for each of the above.

None of it is hard, and an agent writes it in seconds. The trouble is that
each copy is written from scratch, in a separate step, often in a separate
session, and nothing ties them together. The browser accepts an address the
server refuses. The server lowercases what the job that reads the table does
not. For a value that is identity or key material, that is silent data
corruption. Every copy also has to be read by a reviewer, and the whole set
is produced again in the next project.

## Components, for values

Front-end development met the same problem and answered it with components.
An agent building a React screen does not write a date picker; it imports
one that is already tested, and spends its effort on the screen. Nobody
reviews the date picker again.

A scalar is that unit for a value. Each one packages what the list above
spreads across layers:

| Concern | What the scalar carries |
| --- | --- |
| Validation | `validate`, the one rule for which inputs are accepted. |
| Coercion | `normalize` and `parse`, which move loose input to the canonical form, and lenient coercion, which takes a JSON value (a number, a bool, a padded string) toward the scalar's primitive. |
| Marshaling | The canonical form itself, the SQL type, the JSON Schema type and the type in each binding language. The Go types also implement the JSON and `database/sql` interfaces where the value needs them. |
| Proof | Conformance vectors: accepted and rejected inputs that every binding runs in CI. |

The rule is written once, in Rust, and the Go, Python, TypeScript and WASM
bindings all call it. So the agent's job for the email field shrinks to
choosing `Contact.Email` in each layer. The layers cannot drift, because
there is one implementation behind all of them.

## What it saves

- Generation. A name stands in for a pattern, a normalizer, a codec and
  their tests, in every language the project uses. The agent spends its
  context and its turns on the logic that is specific to the project.
- Review. A reviewer sees `Contact.Email` in the diff and checks that it is
  the right type for the field. They do not read a regular expression and
  wonder which addresses it misses.
- Rework. A rule that changes is changed in one place and versioned; see
  [ABI and versioning](/superscalar/policy/abi-and-versioning/). No agent
  has to find and edit every copy.
- Guessing. An agent that writes a phone-number or duration parser from
  memory gets the common cases right and the edge cases wrong. The built-in
  one is pinned by vectors.

## How an agent finds a scalar

The catalog is built to be read by a program, which means an agent can read
it too:

- `superscalar registry dump` (`make dump`) prints the assembled registry as
  JSON: every scalar's canonical name, description, pattern and bounds,
  primitive, SQL and JSON Schema types and per-language types. See
  [the registry](/superscalar/concepts/registry/).
- Every binding carries the same facts as a generated metadata table
  (`SCALAR_METADATA` in Rust, TypeScript and Python,
  `ScalarMetadataByCanonical` in Go), so a tool can look a scalar up at run
  time without restating the rule.
- The [scalar reference](/superscalar/reference/) has one generated page per
  scalar with its accepted and rejected inputs, which is the fastest way for
  an agent to check what a scalar does with a particular input.

Canonical names are append-only and never renamed, so a name an agent
learned from one project means the same thing in the next.

## Telling an agent to use it

An agent reaches for what its instructions name. A few lines in the
project's agent instructions file (`CLAUDE.md`, `AGENTS.md` or the
equivalent) are enough:

```
Typed values (emails, IDs, dates, money, URLs, colors) use a SuperScalar
scalar. Do not write a validator, normalizer or codec for one.
- Find the scalar: run `superscalar registry dump` or read the scalar
  reference.
- Use the generated function for it in this language, for example
  ParseContactEmail in Go or parseContactEmail in TypeScript.
- If no scalar fits, add one to our extension crate with conformance
  vectors. Do not validate the value inline.
```

## Your own components

A project has values of its own: an order number, a tenant slug, a part
code. Left to itself an agent re-derives each one wherever it appears. An
extension gives them the same treatment as the built-ins. You declare the
scalar once in a Rust crate, add vectors, and the code generator emits Go,
Python and TypeScript wrappers for it. From then on every agent session in
the project uses the name. See
[build an extension](/superscalar/guides/build-an-extension/).

The vectors are also what makes an agent's change to a scalar checkable. A
change in behaviour shows up as a change to a vector, in a file a reviewer
can read; see [conformance](/superscalar/guides/conformance/).

## The base library of superschematic

[superschematic](https://github.com/parable-work/superschematic) takes the
same idea up a level. It is a schema compiler: you or an agent describe
tables, operations and environments in a schema, and it generates the
database tables, servers, SDKs and deploys. SuperScalar is its base library.
The field types in a superschematic schema are SuperScalar scalars, and the
validation in every server and SDK it generates is this library, which is
why a generated SDK refuses exactly the request the generated server would
refuse.

Use SuperScalar on its own when you want the value types in code you or an
agent write by hand. Use superschematic when you want the layers around
them generated as well.
