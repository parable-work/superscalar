# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Every crate and package in this repository shares one version. Changes to a
scalar's conformance vectors or accept set are always listed here, with the
bump they require (minor when loosening, major when tightening).

## [Unreleased]

### Added

- Repository bootstrap: license, contribution guide, code of conduct,
  security policy, issue and pull request templates, dependency updates,
  CI, release and Scorecard workflows.
- Release pipeline: `release.yml` builds the static archives, napi addons,
  wheels, header and wasm bundles per platform, refuses a set not built from
  one commit, creates the GitHub release with provenance attestations and an
  SBOM, and publishes to crates.io, npm and PyPI through trusted publishing.
  `release-pr.yml` and `scripts/bump_version.py` set the one shared version;
  `go-module-tag.yml` cuts `go/vX.Y.Z` once the Go module is pinned to a
  release.
- Go module: the module lives at `go/` so that its path
  `github.com/parable-work/superscalar/go` and its `go/vX.Y.Z` tags resolve;
  `include/superscalar.h` is committed inside the module, `release.pin`
  records the release the archives come from, and
  `scripts/fetch_release_archive.sh` downloads and verifies them against the
  pinned manifest digest.

[Unreleased]: https://github.com/parable-work/superscalar/commits/main
