Cross-language scalar types: parse, normalize, validate. One Rust core; Go, Python, TypeScript and WASM bindings.

# SuperScalar

SuperScalar is a library of named scalar types such as `Contact.Email`,
`Identity.UUID`, `Temporal.DateTime` and `Design.Color`. Each scalar is
implemented once, in Rust, and every binding calls that implementation, so an
email address parses the same way in a Go service, a Python job, a Node process
and a browser. Scalars carry a stable numeric id and a canonical name, and a
corpus of conformance vectors runs against every binding in CI. Downstream
projects can add their own scalars in one Rust crate and receive generated
Go, Python and TypeScript bindings for them without forking the core.

Status: pre-release; APIs change until v0.1.0.

## The contract

Every scalar implements three operations over a string input. `parse` checks
the shape of the input and returns its canonical normalized form, or an error.
`normalize` moves the input toward canonical form without enforcing shape, so
a validate-only scalar returns its input unchanged, and `validate` enforces
shape and returns only success or an error.

## 30-second examples

Rust:

```rust
use superscalar::{Registry, ScalarId};

fn main() -> Result<(), superscalar::ScalarError> {
    let registry = Registry::builtin();

    let email = registry.scalar(ScalarId::CONTACT_EMAIL).expect("built-in scalar");
    let canonical = email.parse(registry, " Ada.Lovelace@Example.com ")?;
    println!("{canonical}");

    let uuid = registry.scalar(ScalarId::IDENTITY_UUID).expect("built-in scalar");
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

TypeScript (Node uses the native addon, browsers use the WASM build; same API):

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
version. Scalar ids and canonical names are append-only. Loosening a scalar's
accept set is a minor bump; tightening it is a major bump.

## History

This library was extracted from Parable's platform monorepo. The git history
stayed behind, so this repository starts from the extracted tree.

## License

Apache-2.0. See LICENSE.
