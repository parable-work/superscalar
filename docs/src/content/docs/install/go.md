---
title: Go
description: Install the Go module, which links a prebuilt static archive of the Rust core through cgo.
sidebar:
  order: 2
---

The Go binding is the module `github.com/parable-work/superscalar/go`,
package `superscalar`. It calls the Rust core through cgo against a static
archive built for your platform, so `go build` needs a C toolchain but your
binary has no runtime shared-library dependency.

## Requirements

- `CGO_ENABLED=1` and a C compiler (`cc`) on the build machine.
- One of: macOS arm64, macOS amd64, Linux arm64, Linux amd64. The Linux
  archives are built against musl.
- No Rust toolchain. The archive is prebuilt and fetched.

## Install

Nothing is published yet. From v0.1.0 the module is tagged `go/v0.1.0`
alongside the repository tag `v0.1.0`, and the install is:

```
go get github.com/parable-work/superscalar/go@v0.1.0
```

Then fetch the static archive for your platform. The module commits the C
header and the per-platform cgo link directives; the archive itself is a
GitHub release asset, fetched and verified by a script committed in the
module:

```
go/scripts/fetch.sh v0.1.0
```

The script downloads `superscalar-<platform>.tar.gz` from the matching GitHub
release and checks it against `manifest.json`, which pins every archive in a
release to one core commit. A release whose archives disagree on that commit
is refused at publish time, so a verified archive is guaranteed to match the
header and the other bindings of the same version.

For a fully static Linux binary, add external linking flags:

```
go build -ldflags '-linkmode external -extldflags "-static"' ./...
```

## Quickstart

```go
package main

import (
    "fmt"
    "log"

    superscalar "github.com/parable-work/superscalar/go"
)

func main() {
    email, err := superscalar.ParseContactEmail(" User@Example.COM ")
    if err != nil {
        log.Fatal(err)
    }
    fmt.Println(email) // user@example.com

    if err := superscalar.ValidateIdentityUUID("0f8fad5b-d9cb-469f-a165-70867728950e"); err != nil {
        log.Fatal(err)
    }

    loose, _ := superscalar.NormalizeContactEmail("  Not An Email  ")
    fmt.Println(loose) // not an email
}
```

Every scalar has the same three generated functions: `Parse<Name>` returns
`(string, error)`, `Normalize<Name>` returns `(string, error)`, and
`Validate<Name>` returns `error`. `<Name>` is the canonical name with the dot
removed, so `Contact.Email` becomes `ContactEmail` and `Identity.UUID` becomes
`IdentityUUID`. The generated file also declares a named string type per
scalar (`type ContactEmail string`) with `Validate` and `ValidateRequired`
methods.

## Why cgo, and the alternative

The FAQ explains [why the Go binding uses cgo](/superscalar/faq/#why-does-the-go-binding-use-cgo).
A cgo-free backend that runs the WASM build from pure Go is planned as a
follow-up after v0.1.0 and is not part of the first release.
