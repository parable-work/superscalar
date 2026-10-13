#!/usr/bin/env bash
# Build the napi addon and the WASM bundles, compile the TS, and run the full v2
# conformance through the napi and wasm-node backends in Node and through the
# browser build in Node. scripts/web_smoke.sh then bundles the browser and
# workerd builds with bun and runs the workerd one in workerd.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TS_DIR="$(dirname "$SCRIPT_DIR")"
WORKSPACE="$(dirname "$(dirname "$TS_DIR")")"

# WASM backend bundles: the nodejs target (the Node fallback) and the web
# target (the browser and workerd builds). Both wrap the same superscalar core.
cd "$WORKSPACE/crates/wasm" || exit 1
wasm-pack build --target nodejs --out-dir pkg --dev
"$SCRIPT_DIR/build_wasm_web.sh"

# Copy the nodejs bundle into the TS package so it is self-contained for
# file:/npm installs (relative imports must resolve inside the package dir).
rm -rf "$TS_DIR/wasm-node"
cp -R "$WORKSPACE/crates/wasm/pkg" "$TS_DIR/wasm-node"

cd "$TS_DIR" || exit 1
npm install --silent
# napi addon (Node backend, crate crates/napi) -> native/superscalar-napi.<triple>.node,
# which src/native-addon.ts loads ahead of the @superscalar/<triple> package.
npx napi build --platform --release --manifest-path ../../crates/napi/Cargo.toml --output-dir native
# CJS wrappers (backend.js/napi, generated, index), the ESM Node entry
# (dist/esm), and the browser and workerd builds (dist/browser, dist/workerd):
# the same passes as build:ts.
npx tsc -p tsconfig.json
npx tsc -p tsconfig.esm.json
node scripts/fix-esm-extensions.mjs
npx tsc -p tsconfig.browser.json
npx tsc -p tsconfig.workerd.json
node scripts/build-web.mjs
node test/native-addon.cjs
node test/conformance.cjs
node test/comparability.cjs
node test/backend-fallback.cjs
node test/web-conformance.mjs
