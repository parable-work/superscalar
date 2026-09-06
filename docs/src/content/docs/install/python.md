---
title: Python
description: Install the superscalar wheel, which bundles the Rust core as a native extension module.
sidebar:
  order: 3
---

The Python binding is the PyPI distribution `superscalar`, imported as
`superscalar`. The wheel bundles the Rust core as a native extension module
built with PyO3 and maturin; there is nothing to compile on install.

## Requirements

Python 3.9 or newer. Wheels use the stable ABI (`abi3-py39`), so one wheel
per platform covers every supported interpreter version.

Wheel matrix: manylinux (2_17) on x86_64 and aarch64, macOS arm64 and
x86_64, plus a source distribution. Installing from the source distribution
needs a Rust toolchain and maturin. Pre-releases are uploaded as PEP 440
pre-release versions (`0.1.0a1`), which `pip` skips unless asked for `--pre`.

## Install

Nothing is on PyPI yet. From v0.1.0:

```
pip install superscalar
```

## Quickstart

```python
import superscalar

email = superscalar.parse_contact_email(" User@Example.COM ")
assert email == "user@example.com"

superscalar.validate_identity_uuid("0f8fad5b-d9cb-469f-a165-70867728950e")

loose = superscalar.normalize_contact_email("  Not An Email  ")
assert loose == "not an email"

try:
    superscalar.parse_contact_email("user@example")
except ValueError as err:
    print(err)
```

Every scalar has three generated functions: `parse_<name>(value: str) ->
str`, `normalize_<name>(value: str) -> str` and `validate_<name>(value: str)
-> None`. `<name>` is the canonical name in snake case with the dot dropped,
so `Contact.Email` becomes `contact_email` and `Identity.UUID` becomes
`identity_uuid`. A rejected input raises `ValueError` with the core's error
message.

The module also exposes `SCALAR_ID_BY_CANONICAL`, a mapping from canonical
name to numeric id, which the conformance runner uses to route vectors; see
[conformance](/superscalar/guides/conformance/).
