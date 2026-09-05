#!/usr/bin/env bash
# Build the napi addon and the WASM bundle, compile the TS, and run the full v2
# conformance through BOTH backends.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TS_DIR="$(dirname "$SCRIPT_DIR")"
WORKSPACE="$(dirname "$(dirname "$TS_DIR")")"

# WASM backend bundles: nodejs target (Node conformance) + bundler target
# (browser backend, backend.browser.ts). Both wrap the same superscalar core.
cd "$WORKSPACE/crates/wasm" || exit 1
wasm-pack build --target nodejs --out-dir pkg --dev
wasm-pack build --target bundler --out-dir pkg-bundler --dev

# Copy the wasm bundles into the TS package so it is self-contained for
# file:/npm installs (relative imports must resolve inside the package dir).
rm -rf "$TS_DIR/wasm-node" "$TS_DIR/wasm-bundler"
cp -R "$WORKSPACE/crates/wasm/pkg" "$TS_DIR/wasm-node"
cp -R "$WORKSPACE/crates/wasm/pkg-bundler" "$TS_DIR/wasm-bundler"

cd "$TS_DIR" || exit 1
npm install --silent
# napi addon (Node backend, crate crates/napi) -> native/index.js + native/*.node
npx napi build --platform --release --cargo-cwd ../../crates/napi native
# CJS wrappers (backend.js/napi, generated, index) + ESM browser backend (.mjs).
npx tsc -p tsconfig.json
npx tsc -p tsconfig.browser.json
node scripts/fix-esm-extensions.mjs
node test/conformance.cjs
node test/comparability.cjs
