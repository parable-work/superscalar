---
title: Security
description: How to report a vulnerability, what is in scope, and which versions receive fixes.
sidebar:
  order: 2
---

The security policy lives in the repository as
[`SECURITY.md`](https://github.com/parable-work/superscalar/blob/main/SECURITY.md),
which carries the private reporting address. That file is the authority; this
page says where to look and what the release pipeline does. Response times,
disclosure timelines and the supported-version window are stated in
`SECURITY.md` and not repeated here.

## Reporting

Do not open a public issue for a security problem. Use the private address in
`SECURITY.md`, or GitHub private vulnerability reporting on the repository's
Security tab where it is enabled. Include the affected package and version, a
minimal reproduction and the impact you believe it has.

## Supported versions

Until v0.1.0 there is no supported release; pre-release tags carry no
compatibility or security-fix promise. From v0.1.0, `SECURITY.md` states
which release lines receive fixes.

## Scope

In scope: the Rust crates, the Go, Python, TypeScript and WASM bindings, the
codegen CLI, and the release pipeline in the repository. Out of scope:
vulnerabilities in third-party dependencies that do not affect this library's
behaviour (report those upstream), and issues that require a compromised
build machine or registry account.

## Supply chain

Releases are published from one tagged workflow run. crates.io and PyPI use
trusted publishing (OIDC), so no long-lived tokens for those registries are
stored in the repository; npm packages are published with provenance. GitHub
release assets carry build provenance attestations, and `manifest.json` pins
every archive in a release to one core commit. The Go module's fetch script
verifies a downloaded archive against that manifest before it is used.
