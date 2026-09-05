#!/usr/bin/env bash
# Prove the pin gate REFUSES a mismatched artifact set (the guarantee:
# never publish native/WASM/header built from different core commits). No Rust
# build needed -- this exercises the REAL assemble path the release job runs:
# per-platform gen_manifest.py -> merge_manifests.py -> verify_pinned.py.
#
# The load-bearing case is "two runners, two commits": each per-platform
# manifest is generated with its own core_sha, merge_manifests.py preserves that
# provenance (it does NOT re-stamp a single SHA), and verify_pinned.py then
# aborts. A regen-based assemble (rm manifest.json; gen_manifest.py DIST ONE_SHA)
# would stamp one SHA over everything and let the mismatch through -- so this
# test would fail against that broken design, which is the point.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

GOOD_SHA="deadbeefcafef00d"
OTHER_SHA="0000000000000000"

# --- Lay out two per-platform dist slices, as the matrix runners would. ---
# darwin runner: staticlibs only (STATICLIB_ONLY=1 in the real build).
mkdir -p "$TMP/dist-darwin/lib/darwin_arm64" "$TMP/dist-darwin/lib/darwin_amd64"
printf 'fake-darwin-arm64' >"$TMP/dist-darwin/lib/darwin_arm64/libsuperscalar_ffi.a"
printf 'fake-darwin-amd64' >"$TMP/dist-darwin/lib/darwin_amd64/libsuperscalar_ffi.a"
# linux runner: staticlibs + the shared header/wasm bundle.
mkdir -p "$TMP/dist-linux/lib/linux_amd64" "$TMP/dist-linux/lib/linux_arm64" \
  "$TMP/dist-linux/include" "$TMP/dist-linux/wasm"
printf 'fake-linux-amd64' >"$TMP/dist-linux/lib/linux_amd64/libsuperscalar_ffi.a"
printf 'fake-linux-arm64' >"$TMP/dist-linux/lib/linux_arm64/libsuperscalar_ffi.a"
printf 'fake-header' >"$TMP/dist-linux/include/superscalar.h"
printf 'fake-wasm' >"$TMP/dist-linux/wasm/superscalar_wasm_bg.wasm"

require="darwin_arm64,darwin_amd64,linux_amd64,linux_arm64"

# Helper: run the real assemble path (merge per-platform manifests + verify)
# into a fresh DIST and echo PASS/FAIL based on verify_pinned's exit code.
assemble_and_verify() {
  local dist="$1"
  rm -rf "$dist"
  mkdir -p "$dist"
  cp -R "$TMP/dist-darwin/." "$dist/"
  cp -R "$TMP/dist-linux/." "$dist/"
  python3 "$SCRIPT_DIR/merge_manifests.py" "$dist/manifest.json" \
    "$TMP/dist-darwin/manifest.json" "$TMP/dist-linux/manifest.json"
  python3 "$SCRIPT_DIR/verify_pinned.py" "$dist/manifest.json" --require-platforms "$require"
}

echo "--- a matched set (both runners on $GOOD_SHA) must verify ---"
python3 "$SCRIPT_DIR/gen_manifest.py" "$TMP/dist-darwin" "$GOOD_SHA"
python3 "$SCRIPT_DIR/gen_manifest.py" "$TMP/dist-linux" "$GOOD_SHA"
assemble_and_verify "$TMP/dist"

echo "--- a SHA mismatch across runners must be rejected by the assemble path ---"
# Regenerate the linux slice as if that runner built from a DIFFERENT commit.
python3 "$SCRIPT_DIR/gen_manifest.py" "$TMP/dist-linux" "$OTHER_SHA" >/dev/null
if assemble_and_verify "$TMP/dist"; then
  echo "FAIL: assemble+verify accepted artifacts built from two different commits" >&2
  exit 1
fi
# Restore the matched linux slice for the remaining case.
python3 "$SCRIPT_DIR/gen_manifest.py" "$TMP/dist-linux" "$GOOD_SHA" >/dev/null

echo "--- a corrupted artifact (sha256 drift) must be rejected ---"
assemble_and_verify "$TMP/dist" >/dev/null
printf 'tampered' >>"$TMP/dist/lib/darwin_arm64/libsuperscalar_ffi.a"
if python3 "$SCRIPT_DIR/verify_pinned.py" "$TMP/dist/manifest.json" --require-platforms "$require"; then
  echo "FAIL: verifier accepted a corrupted artifact" >&2
  exit 1
fi

echo "PASS: assemble path rejects cross-runner SHA mismatch and artifact corruption"
