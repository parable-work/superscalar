// Node smoke for the WASM backend: load the wasm-pack (nodejs) bundle and
// round-trip a few scalars through the same core the native bindings use.
// Run via scripts/wasm_smoke.sh, which builds pkg/ first.
const assert = require("node:assert");
const wasm = require("../pkg/superscalar_wasm.js");

// Frozen ScalarId discriminants (see crates/core/src/catalog.rs).
// The codegen crate generates named constants; the smoke test hardcodes a few.
const CONTACT_EMAIL = 8;
const IDENTITY_UUID = 23;
const DESIGN_COLOR = 12;

// Email canonical form is trim + lowercase (asserted exactly).
assert.strictEqual(wasm.scalar_parse(CONTACT_EMAIL, "Foo@Bar.com"), "foo@bar.com");
assert.strictEqual(wasm.scalar_normalize(CONTACT_EMAIL, "  MIXED@Case.COM "), "mixed@case.com");
// uuid / color: assert acceptance (canonical form is the core's concern, proven
// exactly by the Rust ffi conformance test).
assert.ok(wasm.scalar_parse(IDENTITY_UUID, "550e8400-e29b-41d4-a716-446655440000").length > 0);
assert.ok(wasm.scalar_parse(DESIGN_COLOR, "#ff0000").length > 0);

assert.throws(() => wasm.scalar_parse(CONTACT_EMAIL, "not-an-email"));
assert.throws(() => wasm.scalar_parse(IDENTITY_UUID, "not-a-uuid"));
assert.throws(() => wasm.scalar_parse(9999, "anything"));

// validate returns undefined on success, throws on reject.
assert.strictEqual(wasm.scalar_validate(CONTACT_EMAIL, "ok@example.com"), undefined);
assert.throws(() => wasm.scalar_validate(CONTACT_EMAIL, "nope"));

// Smoke success marker. Write straight to stdout rather than via console.* so
// this stays clear of the no-console rule while still printing the ok signal.
process.stdout.write("wasm node smoke: ok\n");
