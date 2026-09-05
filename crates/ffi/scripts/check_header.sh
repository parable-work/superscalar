#!/usr/bin/env bash
# Verify that the committed superscalar.h matches the exported C ABI.
#
#   crates/ffi/scripts/check_header.sh          # CI mode: fail if the header drifted
#   crates/ffi/scripts/check_header.sh --write  # regenerate the committed header (needs nightly)
#
# The exported functions are emitted by the export_c_abi! macro, so cbindgen has
# to expand macros before it can see them ([parse] expand in cbindgen.toml). That
# expansion is `cargo rustc -- -Zunpretty=expanded`, a nightly-only flag, so the
# script runs in one of two modes:
#
#   full     A nightly toolchain is installed. Regenerate with cbindgen on
#            nightly and diff byte for byte against the committed header. CI
#            runs this in the miri job, which already installs nightly. With
#            nightly installed and cbindgen missing the script fails instead of
#            downgrading: the reduced mode would hide drift the machine can
#            check.
#   reduced  No nightly. Compare the function names declared in the header with
#            the `pub unsafe extern "C" fn` names in crates/ffi/src/*.rs. Names only: a
#            changed signature or doc comment passes here and is caught by the
#            full mode. The gates job on stable runs this mode. Every message in
#            this mode says so.
#
# The committed header is the ABI contract the Go binding compiles against;
# regenerating it is a deliberate act, hence --write and never an implicit
# rewrite.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FFI_DIR="$(dirname "$SCRIPT_DIR")"
WORKSPACE="$(dirname "$(dirname "$FFI_DIR")")"
HEADER="$FFI_DIR/superscalar.h"
REDUCED_NOTE="reduced check: names only; no nightly toolchain for cbindgen expansion"

cd "$WORKSPACE" || exit 1

# `rustup toolchain list`, not `rustup run nightly ...`: rustup 1.28.1 and later
# auto-install a toolchain named on `run`, and a check must never download one.
have_nightly() {
  rustup toolchain list 2>/dev/null | grep -qE '^nightly-'
}

require_cbindgen() {
  if ! command -v cbindgen >/dev/null 2>&1; then
    echo "ERROR: cbindgen is not installed. Install the pinned version with 'make setup'" \
      "(cargo install cbindgen --version \$CBINDGEN_VERSION --locked, see tools.env)." >&2
    exit 1
  fi
}

# cbindgen builds the crate with nightly to expand it. Keep those artifacts out
# of the stable target dir so the two toolchains do not invalidate each other.
# cbindgen prints expansion warnings on stderr; they go to $2 and are shown only
# when the run fails.
run_cbindgen() {
  local out="$1" err="$2"
  RUSTUP_TOOLCHAIN=nightly \
    CARGO_EXPAND_TARGET_DIR="${CARGO_TARGET_DIR:-$WORKSPACE/target}/cbindgen-expand" \
    cbindgen --config crates/ffi/cbindgen.toml --crate superscalar-ffi --output "$out" 2>"$err"
}

if [[ "${1:-}" == "--write" ]]; then
  if ! have_nightly; then
    echo "ERROR: --write needs a nightly toolchain (rustup toolchain install nightly --profile minimal)." >&2
    exit 1
  fi
  require_cbindgen
  ERR="$(mktemp)"
  trap 'rm -f "$ERR"' EXIT
  if ! run_cbindgen "$HEADER" "$ERR"; then
    cat "$ERR" >&2
    echo "ERROR: cbindgen failed; $HEADER was not rewritten." >&2
    exit 1
  fi
  echo "wrote $HEADER"
  exit 0
fi

if have_nightly; then
  require_cbindgen
  TMP="$(mktemp)"
  ERR="$(mktemp)"
  trap 'rm -f "$TMP" "$ERR"' EXIT
  if ! run_cbindgen "$TMP" "$ERR"; then
    cat "$ERR" >&2
    echo "ERROR: cbindgen failed (full check: cbindgen on nightly)." >&2
    exit 1
  fi
  if ! diff -u "$HEADER" "$TMP"; then
    echo "ERROR: superscalar.h is out of date. Run: crates/ffi/scripts/check_header.sh --write" >&2
    exit 1
  fi
  echo "superscalar.h is up to date (full check: $(cbindgen --version) on nightly)."
  exit 0
fi

# Reduced mode. Header: every prototype is `<ReturnType> <name>(`, on one line,
# with the return type one of the three the ABI uses. Source: every exported
# function is `pub unsafe extern "C" fn <name>` at the start of a line, in the
# macro or beside it (the anchor skips the same text inside `///` examples).
DECLARED="$(grep -oE '^(ScalarResult|ScalarResultArray|void) [a-z_]+\(' "$HEADER" \
  | awk '{print $2}' | tr -d '(' | sort -u)"
EXPORTED="$(grep -ohE '^[[:space:]]*pub unsafe extern "C" fn [a-z_]+' "$FFI_DIR"/src/*.rs \
  | awk '{print $NF}' | sort -u)"

if [[ "$DECLARED" != "$EXPORTED" ]]; then
  echo "ERROR: superscalar.h declares a different function set than crates/ffi/src exports ($REDUCED_NOTE)." >&2
  echo "--- header" >&2
  echo "$DECLARED" >&2
  echo "--- source" >&2
  echo "$EXPORTED" >&2
  echo "Install nightly and run: crates/ffi/scripts/check_header.sh --write" >&2
  exit 1
fi
echo "superscalar.h declares the exported function set ($REDUCED_NOTE)."
