#!/usr/bin/env bash
# Build the superscalar release artifacts for one or more Rust targets and write a
# commit-pinned manifest. The release workflow runs this per matrix runner
# (macOS builds the darwin archives, ubuntu the linux-musl archives + wasm) and
# merges the manifests; verify_pinned.py then refuses any mismatched set.
#
#   build_release_artifacts.sh <out_dir> [rust-target ...]
# With no target, builds the host. Staticlibs need no C toolchain (cargo
# archives objects; there is no link step), so the matrix cross-builds freely.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE="$(dirname "$SCRIPT_DIR")"
cd "$WORKSPACE" || { echo "error: cannot cd to the superscalar workspace $WORKSPACE" >&2; exit 1; }

OUT="${1:?usage: build_release_artifacts.sh <out_dir> [target ...]}"
shift || true
TARGETS=("$@")
WASM_PROFILE="${WASM_PROFILE:-release}"
CORE_SHA="$(git rev-parse HEAD)"

# Where the build output lands is cargo's answer, not ours: CARGO_TARGET_DIR, --target-dir
# and build.target-dir in a config.toml all move the output root, a relative
# CARGO_TARGET_DIR resolves against the workspace root rather than $PWD, and --target adds a
# triple subdirectory. A path assembled here rather than asked for ships whatever leftover
# sits at the assumed location, and on this path that leftover becomes a published release
# asset that gen_manifest.py then pins as if it were the commit under CORE_SHA.
TARGET_DIR="$(cargo metadata --format-version 1 --no-deps \
  | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')"

BUILD_SCRATCH="$(mktemp -d)"
trap 'rm -rf "$BUILD_SCRATCH"' EXIT

mkdir -p "$OUT/include" "$OUT/wasm"

platform_for() {
  case "$1" in
    aarch64-apple-darwin) echo darwin_arm64 ;;
    x86_64-apple-darwin) echo darwin_amd64 ;;
    x86_64-unknown-linux-musl) echo linux_amd64 ;;
    aarch64-unknown-linux-musl) echo linux_arm64 ;;
    *) echo "unknown-$1" ;;
  esac
}

strip_archive() {
  # Best-effort: strip debug info with whatever native stripper fits the target.
  local target="$1" src="$2" dst="$3"
  case "$target" in
    *-apple-darwin)
      if [[ "$(uname)" == "Darwin" ]]; then strip -S "$src" -o "$dst"; return; fi ;;
    *-linux-*)
      if command -v strip >/dev/null && [[ "$(uname)" == "Linux" ]]; then
        cp "$src" "$dst"; strip --strip-debug "$dst"; return
      fi ;;
  esac
  echo "  (note: no native stripper for $target on $(uname); shipping unstripped)" >&2
  cp "$src" "$dst"
}

# Run one cargo build and set RESOLVED_STATICLIB to the archive cargo reported producing.
# json-render-diagnostics keeps warnings and errors on stderr in their usual rendered form
# and puts artifact records on stdout; the record names the file cargo wrote, so the profile
# and triple subdirectories stay cargo's to name. `fresh` is load-bearing: cargo leaves the
# .a untouched when nothing changed, so mtime on its own cannot separate a legitimate no-op
# from a build whose output went somewhere this script is not looking. The result goes to a
# global because a bare `exit 1` inside a command substitution kills only the subshell,
# which would let a failed resolve fall through into strip_archive.
RESOLVED_STATICLIB=""
resolve_staticlib() {
  local marker="$BUILD_SCRATCH/marker" log="$BUILD_SCRATCH/build.json" artifact freshness
  : >"$marker"
  cargo build --message-format=json-render-diagnostics "$@" >"$log"
  artifact="$(python3 - "$log" <<'PY'
import json, sys

for line in open(sys.argv[1], encoding="utf-8"):
    try:
        message = json.loads(line)
    except ValueError:
        continue
    if message.get("reason") != "compiler-artifact":
        continue
    for path in message.get("filenames") or []:
        if path.endswith("/libsuperscalar_ffi.a"):
            print("fresh" if message.get("fresh") else "relinked", path, sep="\t")
            sys.exit(0)
PY
)"
  if [[ -z "$artifact" ]]; then
    echo "ERROR: cargo build reported no libsuperscalar_ffi.a artifact for: $*" >&2
    echo "       resolved target directory: $TARGET_DIR" >&2
    echo "       CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-<unset>}" >&2
    exit 1
  fi
  freshness="${artifact%%$'\t'*}"
  RESOLVED_STATICLIB="${artifact#*$'\t'}"
  if [[ ! -f "$RESOLVED_STATICLIB" ]]; then
    echo "ERROR: cargo named an archive that is not on disk: $RESOLVED_STATICLIB" >&2
    echo "       resolved target directory: $TARGET_DIR" >&2
    echo "       CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-<unset>}" >&2
    exit 1
  fi
  if [[ "$freshness" == relinked && ! "$RESOLVED_STATICLIB" -nt "$marker" ]]; then
    echo "ERROR: cargo relinked the archive but $RESOLVED_STATICLIB predates this build." >&2
    echo "       resolved target directory: $TARGET_DIR" >&2
    echo "       CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-<unset>}" >&2
    exit 1
  fi
}

build_one() {
  local target="$1" platform
  platform="$(platform_for "$target")"
  echo "==> staticlib $target -> $platform"
  rustup target add "$target" >/dev/null 2>&1 || true
  resolve_staticlib --release --target "$target" -p superscalar-ffi
  mkdir -p "$OUT/lib/$platform"
  strip_archive "$target" "$RESOLVED_STATICLIB" "$OUT/lib/$platform/libsuperscalar_ffi.a"
}

if [[ ${#TARGETS[@]} -eq 0 ]]; then
  echo "==> staticlib (host)"
  resolve_staticlib --release -p superscalar-ffi
  host="$(rustc -vV | sed -n 's/host: //p')"
  mkdir -p "$OUT/lib/$(platform_for "$host")"
  strip_archive "$host" "$RESOLVED_STATICLIB" \
    "$OUT/lib/$(platform_for "$host")/libsuperscalar_ffi.a"
else
  for t in "${TARGETS[@]}"; do build_one "$t"; done
fi

# The header + WASM bundle are platform-independent: the matrix builds them once
# (on the linux runner). Set STATICLIB_ONLY=1 to skip them (the macOS runner).
if [[ -z "${STATICLIB_ONLY:-}" ]]; then
  # Header (cbindgen): regenerate in place so the shipped header is guaranteed
  # to match the pinned core (--write), not merely drift-checked against the
  # committed copy.
  crates/ffi/scripts/check_header.sh --write
  cp crates/ffi/superscalar.h "$OUT/include/superscalar.h"

  # WASM bundle (browser target). Keep stderr: this is a release build, so a
  # wasm-pack failure must surface its diagnostics, not be swallowed.
  echo "==> wasm ($WASM_PROFILE)"
  wasm-pack build crates/wasm --target web --out-dir "$(cd "$OUT" && pwd)/wasm" "--$WASM_PROFILE"
  # Keep only the portable bits in the manifest set.
  find "$OUT/wasm" -maxdepth 1 -type f ! -name "*.wasm" ! -name "*.js" ! -name "*.d.ts" -delete 2>/dev/null || true
fi

python3 "$SCRIPT_DIR/gen_manifest.py" "$OUT" "$CORE_SHA"
python3 "$SCRIPT_DIR/verify_pinned.py" "$OUT/manifest.json"
