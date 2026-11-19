Cross-language scalar types: parse, normalize, validate. One Rust core; Go, Python, TypeScript and WASM bindings.

# SuperScalar

SuperScalar is a library of named scalar types such as `Contact.Email`,
`Identity.UUID`, `Temporal.DateTime` and `Design.Color`. Each scalar is
implemented once, in Rust, and every binding calls that implementation, so an
email address parses the same way in a Go service, a Python job, a Node process
and a browser. A scalar's canonical name is its identity: every binding and
the C ABI name the scalar by it. A corpus of conformance vectors runs against
every binding in CI. Downstream
projects can add their own scalars in one Rust crate and receive generated
Go, Python and TypeScript bindings for them without forking the core.

Status: pre-release; APIs change until v0.1.0.

## Data components for agent-written code

Ask a coding agent for a signup form, the API handler behind it and the
table it writes to, and it will write the email check three times: once per
layer, in whatever language that layer uses. Each copy is cheap to generate
and expensive to trust. Someone has to review it, test it and keep it in
step with the other two, and the next project starts over.

Front ends solved this with components. Nobody has an agent rebuild a date
picker for every screen; it imports one. A scalar is the same kind of unit
for a value. `Contact.Email` packages the validation, the coercion of loose
input, the canonical form and the marshaling facts (SQL type, JSON Schema
type, the type in each language), and an agent uses it by name instead of
writing it again.

- Less to generate. One name replaces a pattern, a normalizer, a codec and
  their tests in every language the project touches.
- Nothing to keep in step. The Go service, the Python job and the browser
  call the same Rust implementation, and the conformance vectors run against
  every binding in CI, so two layers cannot disagree about a value.
- Less to review. The diff says `Contact.Email`. A reviewer checks that the
  name is the right one, not that a regular expression is.
- A catalog an agent can read. `superscalar registry dump` prints every
  scalar's rules and types as JSON, each binding carries the same metadata
  as a table, and the generated reference lists accepted and rejected inputs
  per scalar. The agent looks a value type up instead of guessing at it.
- Your own components. A value that belongs to your domain goes into an
  extension crate once and comes back as generated bindings in each
  language, so every later session reuses it.

SuperScalar is also the base library of
[superschematic](https://github.com/parable-work/superschematic), a schema
compiler that generates tables, servers and SDKs in Go, TypeScript, Python
and Rust. The field types in a superschematic schema are SuperScalar scalars,
which is why a generated SDK refuses the request the generated server would
refuse.

## The contract

Every scalar implements three operations over a string input. `parse` checks
the shape of the input and returns its canonical normalized form, or an error.
`normalize` moves the input toward canonical form without enforcing shape, so
a validate-only scalar returns its input unchanged, and `validate` enforces
shape and returns only success or an error.

## 30-second examples

Rust:

```rust
use superscalar::{names, Registry};

fn main() -> Result<(), superscalar::ScalarError> {
    let registry = Registry::builtin();

    let email = registry.scalar(names::CONTACT_EMAIL).expect("built-in scalar");
    let canonical = email.parse(registry, " Ada.Lovelace@Example.com ")?;
    println!("{canonical}");

    let uuid = registry.scalar("Identity.UUID").expect("built-in scalar");
    uuid.validate(registry, "0f8fad5b-d9cb-469f-a165-70867728950e")?;
    Ok(())
}
```

Go (cgo; links the prebuilt static archive for your platform):

```go
package main

import (
    "fmt"
    "log"

    superscalar "github.com/parable-work/superscalar/go"
)

func main() {
    email, err := superscalar.ParseContactEmail(" Ada.Lovelace@Example.com ")
    if err != nil {
        log.Fatal(err)
    }
    fmt.Println(email)

    if err := superscalar.ValidateIdentityUUID("0f8fad5b-d9cb-469f-a165-70867728950e"); err != nil {
        log.Fatal(err)
    }
}
```

Python (bad input raises `ValueError`):

```python
import superscalar

email = superscalar.parse_contact_email(" Ada.Lovelace@Example.com ")
print(email)

superscalar.validate_identity_uuid("0f8fad5b-d9cb-469f-a165-70867728950e")
```

TypeScript (Node uses the native addon, browsers and Cloudflare Workers use the WASM build; same API):

```ts
import {
  parseContactEmail,
  parseContactEmailStrict,
  validateIdentityUUID,
} from "superscalar";

const email = parseContactEmail(" Ada.Lovelace@Example.com "); // null when invalid
const strict = parseContactEmailStrict("ada@example.com"); // throws when invalid

const [ok, errors] = validateIdentityUUID("0f8fad5b-d9cb-469f-a165-70867728950e");
```

## Install

No packages are published yet; the first release is v0.1.0. Until then, build
from source (see CONTRIBUTING.md).

| Language   | Package                                     | Registry  |
| ---------- | ------------------------------------------- | --------- |
| Rust       | `superscalar`                               | crates.io |
| Go         | `github.com/parable-work/superscalar/go`    | Go module |
| Python     | `superscalar` (import `superscalar`)        | PyPI      |
| TypeScript | `superscalar`                               | npm       |

The Go module requires `CGO_ENABLED=1` and a C toolchain; the static archives
are fetched from the matching GitHub release and verified against a
commit-pinned manifest.

## Documentation

Docs site: https://parable-work.github.io/superscalar/ (placeholder; the site
is published from the release workflow and this URL is not live yet).

Planned pages: install and quickstart per language, the scalar reference,
adding a built-in scalar, building an extension, running the conformance
vectors against your own build, and the ABI and versioning policy.

## Versioning

SemVer on the repository tag `vX.Y.Z`; every crate and package shares the
version. Canonical scalar names are append-only. Loosening a scalar's
accept set is a minor bump; tightening it is a major bump.

## History

This library was extracted from Parable's platform monorepo. The git history
stayed behind, so this repository starts from the extracted tree.

## License

Apache-2.0. See LICENSE.
