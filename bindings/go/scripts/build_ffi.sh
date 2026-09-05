#!/usr/bin/env bash
# Build the superscalar-ffi static archive for the HOST platform and stage it
# at bindings/go/lib/<goos>_<goarch>/libsuperscalar_ffi.a, where the cgo binding
# (cgo_ldflags_<goos>_<goarch>.go) links it. This is the local/CI host build path;
# cross-platform release archives come from the release fetch script.
# A Rust-free consumer build just needs this archive present.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GO_DIR="$(dirname "$SCRIPT_DIR")"
WORKSPACE="$(dirname "$(dirname "$GO_DIR")")"

GOOS="$(go env GOOS)"
GOARCH="$(go env GOARCH)"
DEST="$GO_DIR/lib/${GOOS}_${GOARCH}"

cd "$WORKSPACE"

# Where the build output lands is cargo's answer, not ours: CARGO_TARGET_DIR, --target-dir,
# build.target-dir in a config.toml and --target <triple> all move it, and a relative
# CARGO_TARGET_DIR resolves against the workspace root rather than $PWD. A path assembled
# here rather than asked for is a silent staleness bug the moment any of those is set --
# the copy picks up whatever archive was left at the assumed path and exits 0, and the
# digest stamp below then agrees with those stale bytes, so nothing downstream can tell the
# service is linking an old Rust core. The worktree tooling sets CARGO_TARGET_DIR.
TARGET_DIR="$(cargo metadata --format-version 1 --no-deps \
  | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')"

BUILD_MARKER="$(mktemp)"
BUILD_LOG="$(mktemp)"
trap 'rm -f "$BUILD_MARKER" "$BUILD_LOG"' EXIT

# The marker's mtime is the "before" edge of the staleness check further down. Read it in
# nanoseconds rather than comparing with bash's `-nt`: the bash 3.2 that macOS ships
# compares whole seconds, so a relink that finished inside the same wall-clock second as
# the marker looked older than the marker and the check rejected a correct build. A warm
# target dir relinks this crate in well under a second, so back-to-back go_smoke.sh runs
# tripped it more often than not (5 of 8 on APFS, 2026-09-03). A filesystem that stores
# whole-second mtimes has the same ambiguity however the value is read, so on one of those
# wait for the second to roll over before cargo starts: anything cargo writes after that
# sorts strictly after the marker even at one-second granularity.
MARKER_MTIME_NS="$(python3 -c 'import os, sys; print(os.stat(sys.argv[1]).st_mtime_ns)' "$BUILD_MARKER")"
if [ $((MARKER_MTIME_NS % 1000000000)) -eq 0 ]; then
  while [ "$(date +%s)" -le $((MARKER_MTIME_NS / 1000000000)) ]; do sleep 0.2; done
fi

cargo build -p superscalar-ffi --release --message-format=json-render-diagnostics >"$BUILD_LOG"

# json-render-diagnostics keeps warnings and errors on stderr in their usual rendered form
# and puts artifact records on stdout. Those records name the exact archive cargo produced,
# which beats reassembling "$TARGET_DIR/release/..." here because the profile and
# target-triple subdirectories are cargo's to name. They also carry `fresh`, and that is
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

if [ -z "$ARTIFACT" ]; then
  echo "ERROR: cargo build reported no libsuperscalar_ffi.a artifact." >&2
  echo "       resolved target directory: $TARGET_DIR" >&2
  echo "       CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-<unset>}" >&2
  exit 1
fi

FRESHNESS="${ARTIFACT%%$'\t'*}"
ARCHIVE="${ARTIFACT#*$'\t'}"

if [ ! -f "$ARCHIVE" ]; then
  echo "ERROR: cargo named an archive that is not on disk: $ARCHIVE" >&2
  echo "       resolved target directory: $TARGET_DIR" >&2
  echo "       CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-<unset>}" >&2
  exit 1
fi

# cargo's artifact record is the authority for which archive this build produced. `fresh`
# means the fingerprint matched and the bytes on disk are the ones cargo verified, so there
# is nothing to compare against. `relinked` means cargo says it just wrote them, and the
# mtime comparison is a cross-check on that claim only: an archive older than the marker
# is one cargo did not write in this build. Nanosecond mtimes, see the marker above.
if [ "$FRESHNESS" = relinked ]; then
  ARCHIVE_MTIME_NS="$(python3 -c 'import os, sys; print(os.stat(sys.argv[1]).st_mtime_ns)' "$ARCHIVE")"
  if [ "$ARCHIVE_MTIME_NS" -lt "$MARKER_MTIME_NS" ]; then
    echo "ERROR: cargo relinked the archive but $ARCHIVE predates this build." >&2
    echo "       archive mtime (ns): $ARCHIVE_MTIME_NS  build marker (ns): $MARKER_MTIME_NS" >&2
    echo "       resolved target directory: $TARGET_DIR" >&2
    echo "       CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-<unset>}" >&2
    exit 1
  fi
fi

mkdir -p "$DEST"
cp "$ARCHIVE" "$DEST/libsuperscalar_ffi.a"
echo "staged ${GOOS}_${GOARCH} archive -> $DEST/libsuperscalar_ffi.a"

# Stamp the archive's digest into a real .go file in the linking package.
#
# Go's build cache keys a cgo link on the package build ID plus the LDFLAGS *text*; it
# never hashes the bytes of the archive those flags point at. So replacing the .a does
# not invalidate the link, and the service goes on running a binary that contains the
# PREVIOUS Rust core. That is how a downstream handle scalar moved to camelCase in the
# core while its API kept rejecting camelCase handles as "invalid format" across several
# rebuilds and restarts -- the running binary was still linking the kebab-era core.
#
# Writing the digest into a real .go file puts the archive's CONTENT into the package's
# source hash, so a new archive changes the build ID and Go relinks on its own. The
# const is declared and never referenced, so a build without this file (CI fetching a
# prebuilt archive) still compiles.
DIGEST="$(shasum -a 256 "$DEST/libsuperscalar_ffi.a" | awk '{print $1}')"
cat > "$GO_DIR/archive_stamp_${GOOS}_${GOARCH}.go" <<STAMP
//go:build ${GOOS} && ${GOARCH}

package superscalar

// Code generated by scripts/build_ffi.sh. DO NOT EDIT.
//
// The SHA-256 of the static archive this package links. Its only job is to change when
// the archive changes, so Go's build cache cannot reuse a link against a stale one.
const archiveSHA256 = "${DIGEST}"
STAMP
echo "stamped archive digest ${DIGEST:0:12}... -> $GO_DIR/archive_stamp_${GOOS}_${GOARCH}.go"
