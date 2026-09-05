#!/usr/bin/env bash
# Build the ffi static archive into bindings/go/lib/<goos>_<goarch>, then run the Go
# binding's v2 conformance via cgo against it. The binding links the hermetic
# lib/<goos>_<goarch> (cgo_ldflags_<goos>_<goarch>.go), so this stages the archive
# there exactly as a release fetch would.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GO_DIR="$(dirname "$SCRIPT_DIR")"

bash "$SCRIPT_DIR/build_ffi.sh"

cd "$GO_DIR" || exit 1
CGO_ENABLED=1 go test ./... -count=1
