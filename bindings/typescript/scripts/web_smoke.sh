#!/usr/bin/env bash
# Bundle the browser and workerd builds with `bun build`, as a site and a
# Worker would, and run them:
#   - test/web/site.mjs with --target=browser and no plugin, run in Node:
#     the bundle carries no Node API, so Node runs what a browser runs
#     (test/web/index.html loads the same bundle in a browser, by hand);
#   - test/web/worker.mjs with the workerd condition and the .wasm import
#     kept external, run with `workerd test` beside the module it imports:
#     the full corpus and the generated wrappers in workerd.
# Prints each bundle's size, raw and gzipped. Needs bun, node and a built
# package (scripts/ts_smoke.sh, or npm run build:wasm-web and build:ts);
# installs the pinned workerd (test/workerd/package-lock.json) with npm ci.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TS_DIR="$(dirname "$SCRIPT_DIR")"
cd "$TS_DIR" || exit 1

size() {
  printf '%s: %s bytes, %s gzipped\n' "$1" "$(wc -c < "$2" | tr -d ' ')" "$(gzip -9c "$2" | wc -c | tr -d ' ')"
}

no_node() {
  if grep -nE '"node:|require\(' "$1"; then
    echo "web smoke: $1 reaches for Node" >&2
    exit 1
  fi
}

echo "==> site bundle (bun build --target=browser)"
site_out="$TS_DIR/test/web/out"
rm -rf "$site_out"
bun build --target=browser --outdir "$site_out" test/web/site.mjs
no_node "$site_out/site.js"
node "$site_out/site.js"
size "site bundle" "$site_out/site.js"

echo "==> worker bundle (bun build --conditions=workerd --external '*.wasm')"
worker_out="$TS_DIR/test/workerd/out"
rm -rf "$worker_out"
bun build --target=browser --conditions=workerd --external '*.wasm' --outdir "$worker_out" test/web/worker.mjs
no_node "$worker_out/worker.js"
grep -q 'from "./superscalar_wasm_bg.wasm"' "$worker_out/worker.js" || {
  echo "web smoke: the worker bundle does not import ./superscalar_wasm_bg.wasm" >&2
  exit 1
}
cp dist/workerd/superscalar_wasm_bg.wasm "$worker_out/"
size "worker bundle" "$worker_out/worker.js"
size "wasm module" "$worker_out/superscalar_wasm_bg.wasm"

(cd test/workerd && npm ci --silent)
test/workerd/node_modules/.bin/workerd test test/workerd/config.capnp
echo "ts web smoke: ok (site bundle in Node, worker bundle in workerd)"
