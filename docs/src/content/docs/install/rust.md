---
title: Rust
description: Add the superscalar crate and call the core directly, with no FFI layer.
sidebar:
  order: 1
---

Rust consumers link the core crate itself. There is no binding layer: the
crate is pure Rust, declares `#![forbid(unsafe_code)]`, and is the same code
every other language calls through the C ABI or WASM.

## Add the dependency

The crate name is `superscalar`. Nothing is on crates.io yet; once v0.1.0 is
published:

```
cargo add superscalar
```

Until the first crates.io publish, depend on a git tag. The first pre-release
tag is planned as `v0.1.0-alpha.1`; check the repository's releases page for
the tag that actually exists.

```toml
[dependencies]
superscalar = { git = "https://github.com/parable-work/superscalar", tag = "v0.1.0-alpha.1" }
```

No minimum supported Rust version has been declared yet. The repository
itself builds with the toolchain pinned in its `rust-toolchain.toml`.

## Quickstart

```rust
use superscalar::{Registry, ScalarId};

fn main() -> Result<(), superscalar::ScalarError> {
    let registry = Registry::builtin();

    let email = registry.scalar(ScalarId::CONTACT_EMAIL).expect("built-in scalar");
    let canonical = email.parse(registry, " User@Example.COM ")?;
    assert_eq!(canonical, "user@example.com");

    let uuid = registry.scalar(ScalarId::IDENTITY_UUID).expect("built-in scalar");
    uuid.validate(registry, "0f8fad5b-d9cb-469f-a165-70867728950e")?;

    // normalize moves toward canonical form without enforcing shape.
    let loose = email.normalize(registry, "  Not An Email  ")?;
    assert_eq!(loose, "not an email");
    Ok(())
}
```

`Registry::builtin()` returns the process-wide registry of built-in scalars.
`registry.scalar(id)` returns `Option<&dyn Scalar>`, the trait with `parse`,
`normalize` and `validate`; each hook takes the registry as its first argument
so a scalar can consult other scalars. `ScalarId` is a `u32` newtype with one
associated constant per built-in scalar (`ScalarId::CONTACT_EMAIL`). The
`scalar_for(id)` free function is a convenience over the built-in registry that
panics on an unknown id. `ScalarError` has two public fields, `kind: ErrorKind`
and `message: String`, and implements `Display`.

Downstream projects assemble their own registry from the built-ins plus their
extensions; see the guide on building an extension.

## Behaviour is the conformance corpus

`cargo test` in the repository runs every vector in
`conformance/core-scalars.v2.json` against the core. If you depend on a
specific canonical form, find the vector that pins it; if there is none, the
form is not yet guaranteed. See [conformance](/superscalar/guides/conformance/).
