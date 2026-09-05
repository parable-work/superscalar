#!/usr/bin/env bash
# Build the PyO3 binding with maturin and run the full v2 conformance suite.
# Uses a uv-managed venv pinned to 3.12 (abi3 wheel works on 3.9+).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PY_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PY_DIR" || exit 1
uv venv --python 3.12 --clear .venv
# shellcheck disable=SC1091
source .venv/bin/activate
uv pip install --quiet maturin pytest
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --release
pytest -q tests

# The venv above is 3.12, as is the CI job, so nothing here would notice a
# generated module that only fails on the floor pyproject.toml declares
# (requires-python = ">=3.9"). Evaluate it on a real 3.9 too. Uses the uv this
# script already drives; --no-project keeps it off the 3.12 venv.
uv run --python 3.9 --no-project python scripts/py39_floor_check.py
