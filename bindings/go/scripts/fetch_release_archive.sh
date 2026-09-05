#!/usr/bin/env bash
# Fetch the prebuilt superscalar-ffi static archive for one platform from a
# GitHub release of parable-work/superscalar and verify it before it is linked.
#
#   fetch_release_archive.sh [--tag vX.Y.Z] [--platform <goos>_<goarch>] [--out DIR]
#                            [--unpinned] [--from-dir DIR]
#
# Defaults: the tag, core commit and manifest digest come from release.pin
# next to this module (written by the release pipeline, see that file); the
# platform is the host's `go env GOOS`_`GOARCH`; the archive is placed at
# <module>/lib/<platform>/libsuperscalar_ffi.a, which is where the cgo
# LDFLAGS in cgo_ldflags_<goos>_<goarch>.go point.
#
# Verification, in order:
#   1. sha256(manifest.json) equals release.pin's manifest_sha256, and the
#      manifest's core_sha equals release.pin's core_sha. A different tag than
#      the pinned one needs --unpinned, which skips only this step.
#   2. every artifact in the manifest carries the manifest's core_sha (the
#      whole release set was built from one commit).
#   3. sha256 of the extracted archive equals the manifest entry for it.
#
# The module directory in the Go module cache ($GOMODCACHE) is read-only, so a
# consumer that installed the module with `go get` passes --out to put the
# archive somewhere writable and adds `-L<dir>/<platform>` to CGO_LDFLAGS; the
# script prints the exact line. Inside a checkout the default --out works.
#
# --from-dir DIR reads manifest.json and the archive from a directory instead
# of downloading (a release downloaded earlier, or a local build packed the
# same way); every verification step still runs.
#
# Needs: bash, tar, python3 (hashing and JSON), and either the GitHub CLI (gh,
# required while the repository is private) or curl. Set GITHUB_TOKEN for curl
# against a private repository.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GO_DIR="$(dirname "$SCRIPT_DIR")"
REPO="parable-work/superscalar"
PIN="$GO_DIR/release.pin"

TAG=""
PLATFORM=""
OUT=""
UNPINNED=0
FROM_DIR=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --tag) TAG="$2"; shift 2 ;;
    --platform) PLATFORM="$2"; shift 2 ;;
    --out) OUT="$2"; shift 2 ;;
    --unpinned) UNPINNED=1; shift ;;
    --from-dir) FROM_DIR="$2"; shift 2 ;;
    -h|--help) sed -n '2,30p' "$0"; exit 0 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done

pin_value() {
  sed -n "s/^$1=//p" "$PIN" | head -n 1
}

PIN_TAG="$(pin_value version)"
PIN_CORE_SHA="$(pin_value core_sha)"
PIN_MANIFEST_SHA="$(pin_value manifest_sha256)"

if [[ -z "$TAG" ]]; then
  TAG="$PIN_TAG"
  if [[ -z "$TAG" && -n "$FROM_DIR" ]]; then
    TAG="local"
  fi
  if [[ -z "$TAG" ]]; then
    echo "error: release.pin has no version yet (no release has been cut); pass --tag, or build" >&2
    echo "       the archive from source with scripts/build_ffi.sh" >&2
    exit 1
  fi
fi
if [[ "$TAG" != "$PIN_TAG" && "$UNPINNED" -ne 1 ]]; then
  echo "error: --tag $TAG differs from the pinned release ${PIN_TAG:-<none>}; add --unpinned to skip the pin check" >&2
  exit 1
fi

if [[ -z "$PLATFORM" ]]; then
  PLATFORM="$(go env GOOS)_$(go env GOARCH)"
fi
case "$PLATFORM" in
  darwin_arm64|darwin_amd64|linux_amd64|linux_arm64) ;;
  *) echo "error: no prebuilt archive for platform $PLATFORM (darwin_arm64, darwin_amd64, linux_amd64, linux_arm64)" >&2; exit 1 ;;
esac

if [[ -z "$OUT" ]]; then
  OUT="$GO_DIR/lib"
  if ! mkdir -p "$OUT" 2>/dev/null || [[ ! -w "$OUT" ]]; then
    echo "error: $OUT is not writable (module cache?); pass --out DIR and add -L<DIR>/$PLATFORM to CGO_LDFLAGS" >&2
    exit 1
  fi
fi

ARCHIVE="superscalar-$PLATFORM.tar.gz"
DL="$(mktemp -d)"
trap 'rm -rf "$DL"' EXIT

if [[ -n "$FROM_DIR" ]]; then
  echo "==> reading $FROM_DIR: manifest.json + $ARCHIVE"
  cp "$FROM_DIR/manifest.json" "$FROM_DIR/$ARCHIVE" "$DL/"
elif command -v gh >/dev/null 2>&1; then
  echo "==> downloading $TAG: manifest.json + $ARCHIVE"
  gh release download "$TAG" --repo "$REPO" --dir "$DL" --pattern manifest.json --pattern "$ARCHIVE"
else
  echo "==> downloading $TAG: manifest.json + $ARCHIVE"
  auth=()
  if [[ -n "${GITHUB_TOKEN:-}" ]]; then auth=(-H "Authorization: Bearer $GITHUB_TOKEN"); fi
  for asset in manifest.json "$ARCHIVE"; do
    curl -fsSL "${auth[@]}" -o "$DL/$asset" "https://github.com/$REPO/releases/download/$TAG/$asset"
  done
fi

sha256() {
  python3 -c 'import hashlib, sys; print(hashlib.sha256(open(sys.argv[1], "rb").read()).hexdigest())' "$1"
}

if [[ "$UNPINNED" -ne 1 ]]; then
  actual="$(sha256 "$DL/manifest.json")"
  if [[ "$actual" != "$PIN_MANIFEST_SHA" ]]; then
    echo "error: manifest.json sha256 $actual does not match release.pin ($PIN_MANIFEST_SHA)" >&2
    exit 1
  fi
fi

# Steps 1 (core_sha) and 2: one pass over the manifest in python; prints the
# expected archive digest on success.
EXPECTED="$(python3 - "$DL/manifest.json" "lib/$PLATFORM/libsuperscalar_ffi.a" "$PIN_CORE_SHA" "$UNPINNED" <<'PY'
import json, sys
manifest = json.load(open(sys.argv[1]))
want_file, pin_core, unpinned = sys.argv[2], sys.argv[3], sys.argv[4] == "1"
core = manifest.get("core_sha")
errors = []
if not core:
    errors.append("manifest has no core_sha")
if not unpinned and core != pin_core:
    errors.append(f"manifest core_sha {core} != release.pin core_sha {pin_core}")
entry = None
for art in manifest.get("artifacts", []):
    if art.get("core_sha") != core:
        errors.append(f"{art.get('file')}: core_sha {art.get('core_sha')} != manifest {core}")
    if art.get("file") == want_file:
        entry = art
if entry is None:
    errors.append(f"manifest has no entry for {want_file}")
if errors:
    print("manifest rejected:", file=sys.stderr)
    for e in errors:
        print("  - " + e, file=sys.stderr)
    sys.exit(1)
print(entry["sha256"])
PY
)"

mkdir -p "$DL/x"
tar -xzf "$DL/$ARCHIVE" -C "$DL/x"
EXTRACTED="$DL/x/lib/$PLATFORM/libsuperscalar_ffi.a"
if [[ ! -f "$EXTRACTED" ]]; then
  echo "error: $ARCHIVE does not contain lib/$PLATFORM/libsuperscalar_ffi.a" >&2
  exit 1
fi
actual="$(sha256 "$EXTRACTED")"
if [[ "$actual" != "$EXPECTED" ]]; then
  echo "error: archive sha256 $actual does not match the manifest ($EXPECTED)" >&2
  exit 1
fi

mkdir -p "$OUT/$PLATFORM"
cp "$EXTRACTED" "$OUT/$PLATFORM/libsuperscalar_ffi.a"
cp "$DL/manifest.json" "$OUT/manifest.json"
echo "verified $ARCHIVE ($TAG, core $(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["core_sha"][:12])' "$DL/manifest.json"))"
echo "staged $OUT/$PLATFORM/libsuperscalar_ffi.a"

if [[ "$OUT" == "$GO_DIR/lib" ]]; then
  # Same stamp scripts/build_ffi.sh writes: the digest goes into a real .go
  # file so Go's build cache relinks when the archive changes (the LDFLAGS
  # text alone does not change).
  GOOS="${PLATFORM%_*}"
  GOARCH="${PLATFORM#*_}"
  cat > "$GO_DIR/archive_stamp_${GOOS}_${GOARCH}.go" <<STAMP
//go:build ${GOOS} && ${GOARCH}

package superscalar

// Code generated by scripts/build_ffi.sh. DO NOT EDIT.
//
// The SHA-256 of the static archive this package links. Its only job is to change when
// the archive changes, so Go's build cache cannot reuse a link against a stale one.
const archiveSHA256 = "${actual}"
STAMP
  echo "stamped archive digest ${actual:0:12}... -> archive_stamp_${GOOS}_${GOARCH}.go"
else
  echo
  echo "Add the archive directory to the cgo link flags when building your program:"
  echo "  export CGO_LDFLAGS=\"-L$OUT/$PLATFORM\""
fi
