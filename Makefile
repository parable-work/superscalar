# superscalar (the generic scalar library). Convenience targets for the core,
# binding and codegen crates and the three generated packages. Mirrors the
# CI workflow gates (.github/workflows/ci.yml).

.PHONY: test lint fmt header c-smoke wasm miri acme acme-third-scalar all

all: test lint header

test:
	cargo test

lint:
	cargo clippy --all-targets -- -D warnings
	cargo clippy -p superscalar-wasm --target wasm32-unknown-unknown -- -D warnings
	cargo clippy -p superscalar-python -p superscalar-napi --all-targets -- -D warnings
	cargo fmt --check

fmt:
	cargo fmt

# Header drift check. With a nightly toolchain installed: regenerate with cbindgen
# (macro expansion needs nightly) and diff byte for byte; --write to update.
# Without one: compare the declared function names to the exported ones.
header:
	crates/ffi/scripts/check_header.sh

# Compile + run the tiny C consumer against the static library and header.
c-smoke:
	crates/ffi/scripts/c_smoke.sh

# Build the wasm-pack (nodejs) bundle and run the Node smoke.
wasm:
	crates/wasm/scripts/wasm_smoke.sh

# Free-discipline / UB proof for the ffi ABI under miri (needs nightly + miri).
miri:
	cargo +nightly miri test -p superscalar-ffi

# The downstream-extension example: build its four bindings over the assembled
# registry and run the built-in + Acme vectors through each.
acme:
	examples/acme-scalars/scripts/smoke.sh

# Add a temporary third scalar to the example, rerun the smoke, and fail if
# anything outside examples/acme-scalars/ changed.
acme-third-scalar:
	examples/acme-scalars/scripts/check_third_scalar.sh

#   make setup && make bindings   (make -k bindings to run past failures)

.PHONY: setup go python ts codegen codegen-check dump docs docs-check bindings

# The Rust pin is rust-toolchain.toml (channel, rustfmt, clippy, wasm32 target);
# the other tool pins are tools.env. Both are read here and by CI, never inlined.
setup:
	@set -a; . ./tools.env; set +a; \
	echo "==> rust toolchain from rust-toolchain.toml (channel, components, wasm32 target)"; \
	rustup toolchain install; \
	rustup show active-toolchain; \
	if ! cbindgen --version 2>/dev/null | grep -q "$$CBINDGEN_VERSION"; then \
		echo "==> installing cbindgen $$CBINDGEN_VERSION"; \
		cargo install cbindgen --version "$$CBINDGEN_VERSION" --locked; \
	fi; \
	if ! wasm-pack --version 2>/dev/null | grep -q "$$WASM_PACK_VERSION"; then \
		echo "==> installing wasm-pack $$WASM_PACK_VERSION"; \
		cargo install wasm-pack --version "$$WASM_PACK_VERSION" --locked; \
	fi; \
	missing=0; \
	for tool in go node uv cc; do \
		command -v $$tool >/dev/null 2>&1 || { echo "MISSING: $$tool (host-managed; needed by the binding smokes)"; missing=1; }; \
	done; \
	[ $$missing -eq 0 ] || { echo "install the tools above, then re-run: make setup"; exit 1; }; \
	echo "==> setup ok"

go:
	bindings/go/scripts/go_smoke.sh

python:
	bindings/python/scripts/py_smoke.sh

ts:
	bindings/typescript/scripts/ts_smoke.sh

# Codegen reads superscalar.toml (package names, import paths, output paths).
# `codegen` rewrites the generated files; `codegen-check` is the drift gate.
codegen:
	cargo run -p superscalar-codegen

codegen-check:
	cargo run -p superscalar-codegen -- --check

# The built-in registry as stable JSON on stdout (fixed key order).
dump:
	cargo run -q -p superscalar-codegen -- registry dump

# Per-scalar Markdown reference pages. DOCS_OUT defaults to a scratch dir; the
# docs site build writes them into docs/src/content/docs/reference/ in CI.
DOCS_OUT ?= target/docs-reference
docs:
	cargo run -p superscalar-codegen -- docs --out "$(DOCS_OUT)"

# Fails when a scalar has no conformance vectors or no description.
docs-check:
	cargo run -p superscalar-codegen -- docs --check

bindings: test lint header codegen-check docs-check c-smoke wasm go python ts acme
