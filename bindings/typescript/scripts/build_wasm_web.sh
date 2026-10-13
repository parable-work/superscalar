#!/usr/bin/env bash
# Build crates/wasm for the browser and workerd builds: the wasm-pack `web`
# target, with the size-tuned wasm-release profile (the workspace Cargo.toml),
# into wasm-web/, which scripts/build-web.mjs reads. Needs wasm-pack and the
# wasm32-unknown-unknown target that rust-toolchain.toml installs.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TS_DIR="$(dirname "$SCRIPT_DIR")"
WORKSPACE="$(dirname "$(dirname "$TS_DIR")")"

rm -rf "$TS_DIR/wasm-web"
wasm-pack build "$WORKSPACE/crates/wasm" --target web --out-dir "$TS_DIR/wasm-web" --profile wasm-release
