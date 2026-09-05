#!/usr/bin/env bash
# Compile and run the tiny C consumer against the generated header and the
# static library. Proves superscalar.h is valid C and the ABI links + runs.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FFI_DIR="$(dirname "$SCRIPT_DIR")"
WORKSPACE="$(dirname "$(dirname "$FFI_DIR")")"
CC="${CC:-cc}"

cd "$WORKSPACE" || exit 1

# Build the static library and ask rustc for the system libs it needs at link
# time (portable across linux/macos rather than hardcoding).
#
# The archive path comes from cargo's own artifact record. Globbing under a relative
# "target" assumed two things that do not hold: that cargo writes there at all, when
# CARGO_TARGET_DIR, --target-dir and build.target-dir in a config.toml each move it, and
# that it is a real directory, when in a git worktree it is a symlink that find will not
# descend without -L. Either way the glob picks an arbitrary match or nothing, so this smoke
# test could link a stale archive and report a pass for a core it never exercised.
TARGET_DIR="$(cargo metadata --format-version 1 --no-deps \
  | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')"

BUILD_MARKER="$(mktemp)"
BUILD_LOG="$(mktemp)"
trap 'rm -f "$BUILD_MARKER" "$BUILD_LOG"' EXIT

cargo build -p superscalar-ffi --quiet --message-format=json-render-diagnostics >"$BUILD_LOG"

# --quiet drops the progress lines but leaves the artifact records intact. `fresh` is
# load-bearing: cargo leaves the .a untouched when nothing changed, so mtime on its own
# cannot separate a legitimate no-op from a build whose output went somewhere else.
ARTIFACT="$(python3 - "$BUILD_LOG" <<'PY'
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

if [[ -z "$ARTIFACT" ]]; then
  echo "ERROR: cargo build reported no libsuperscalar_ffi.a artifact." >&2
  echo "       resolved target directory: $TARGET_DIR" >&2
  echo "       CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-<unset>}" >&2
  exit 1
fi

FRESHNESS="${ARTIFACT%%$'\t'*}"
STATICLIB="${ARTIFACT#*$'\t'}"

if [[ ! -f "$STATICLIB" ]]; then
  echo "ERROR: cargo named a static library that is not on disk: $STATICLIB" >&2
  echo "       resolved target directory: $TARGET_DIR" >&2
  echo "       CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-<unset>}" >&2
  exit 1
fi

if [[ "$FRESHNESS" == relinked && ! "$STATICLIB" -nt "$BUILD_MARKER" ]]; then
  echo "ERROR: cargo relinked the archive but $STATICLIB predates this build." >&2
  echo "       resolved target directory: $TARGET_DIR" >&2
  echo "       CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-<unset>}" >&2
  exit 1
fi

NATIVE_LIBS="$(cargo rustc -p superscalar-ffi --crate-type staticlib -- \
  --print native-static-libs 2>&1 | sed -n 's/.*native-static-libs: //p' | head -1)"

# rustc prints the system libs as one space-separated string (e.g.
# "-lSystem -lc -lm"); split it into a proper argv array so each flag reaches
# cc as its own argument without relying on unquoted word-splitting.
read -ra NATIVE_LIB_ARGS <<<"$NATIVE_LIBS"

OUT="$(mktemp -d)"
trap 'rm -rf "$OUT" "$BUILD_MARKER" "$BUILD_LOG"' EXIT

"$CC" -std=c11 -Wall -Wextra -Werror \
  -I "$FFI_DIR" \
  "$FFI_DIR/tests/c_consumer.c" \
  "$STATICLIB" \
  "${NATIVE_LIB_ARGS[@]}" \
  -o "$OUT/c_consumer"

"$OUT/c_consumer"
