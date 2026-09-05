#!/usr/bin/env bash
# Build the WASM backend with wasm-pack (nodejs target) and run the Node smoke.
# Proves the wasm crate builds for wasm32 and round-trips through the core in a
# JS runtime.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WASM_DIR="$(dirname "$SCRIPT_DIR")"

cd "$WASM_DIR" || exit 1
wasm-pack build --target nodejs --out-dir pkg --dev
node scripts/node_smoke.cjs
