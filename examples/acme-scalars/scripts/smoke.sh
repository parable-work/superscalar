#!/usr/bin/env bash
# Build the Acme example end to end and run the conformance vectors (built-in
# corpus plus the Acme corpus) through each of the four bindings.
#
#   examples/acme-scalars/scripts/smoke.sh
#
# Needs: the pinned Rust toolchain with the wasm32-unknown-unknown target,
# wasm-pack, a C compiler, node, python3. Nothing is installed from the network.
# Build output stays in the example's own target dir; the built artifacts the
# runners load are copied to a temp dir that is removed on exit.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXAMPLE_DIR="$(dirname "$SCRIPT_DIR")"
REPO_ROOT="$(cd "$EXAMPLE_DIR/../.." && pwd)"
CORE_VECTORS="$REPO_ROOT/conformance/core-scalars.v2.json"
ACME_VECTORS="$EXAMPLE_DIR/conformance/acme-scalars.v2.json"
CC="${CC:-cc}"

OUT="$(mktemp -d)"
trap 'rm -rf "$OUT"' EXIT

cd "$EXAMPLE_DIR" || exit 1

echo "==> rust: format, clippy, assembly and conformance tests"
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test

echo "==> registry dump (the canonical-name-to-id table the runners use)"
cargo run --quiet -p acme-scalars --example dump >"$OUT/registry.json"
python3 - "$OUT/registry.json" <<'PY'
import json, sys
dump = json.load(open(sys.argv[1], encoding="utf-8"))
names = [e["name"] for e in dump["extensions"]]
assert names == ["builtin", "acme"], names
acme = [s for s in dump["scalars"] if s["extension"] == "acme"]
assert acme and all(4096 <= s["id"] < 8192 for s in acme), acme
print(f"dump: {len(dump['scalars'])} scalars, {len(acme)} in the acme block")
PY

echo "==> C: static archive + generic header + tiny consumer"
STATICLIB="$(python3 scripts/cargo_artifact.py acme-scalars-ffi staticlib)"
NATIVE_LIBS="$(cargo rustc --release -p acme-scalars-ffi --crate-type staticlib -- \
  --print native-static-libs 2>&1 | sed -n 's/.*native-static-libs: //p' | head -1)"
read -ra NATIVE_LIB_ARGS <<<"$NATIVE_LIBS"
"$CC" -std=c11 -Wall -Wextra -Werror \
  -I "$REPO_ROOT/crates/ffi" \
  ffi/tests/c_consumer.c "$STATICLIB" "${NATIVE_LIB_ARGS[@]}" \
  -o "$OUT/c_consumer"
python3 scripts/run_vectors.py --registry "$OUT/registry.json" \
  --backend c --c-consumer "$OUT/c_consumer" "$CORE_VECTORS" "$ACME_VECTORS"

echo "==> Node (napi): cdylib loaded as a .node addon"
NAPI_LIB="$(python3 scripts/cargo_artifact.py acme-scalars-napi cdylib)"
cp "$NAPI_LIB" "$OUT/acme_scalars_napi.node"
node scripts/run_vectors.cjs --registry "$OUT/registry.json" \
  --backend napi --addon "$OUT/acme_scalars_napi.node" "$CORE_VECTORS" "$ACME_VECTORS"

echo "==> Node (wasm): wasm-pack nodejs bundle"
(cd wasm && wasm-pack build --quiet --target nodejs --out-dir "$OUT/wasm-pkg" --dev)
node scripts/run_vectors.cjs --registry "$OUT/registry.json" \
  --backend wasm --bundle "$OUT/wasm-pkg/acme_scalars_wasm.js" "$CORE_VECTORS" "$ACME_VECTORS"

echo "==> Python: cdylib imported as the extension module _native"
PY_LIB="$(PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 python3 scripts/cargo_artifact.py acme-scalars-python cdylib)"
# CPython finds an extension module by <name><suffix>; the abi3 suffix is the
# one a maturin-built abi3 wheel would carry.
PY_SUFFIX="$(python3 -c 'import importlib.machinery as m; print([s for s in m.EXTENSION_SUFFIXES if "abi3" in s][0])')"
mkdir -p "$OUT/py"
cp "$PY_LIB" "$OUT/py/_native$PY_SUFFIX"
python3 scripts/run_vectors.py --registry "$OUT/registry.json" \
  --backend python --module-dir "$OUT/py" "$CORE_VECTORS" "$ACME_VECTORS"

echo "acme-scalars smoke: ok (C, napi, wasm, python)"
