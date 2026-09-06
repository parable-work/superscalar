// The native addon loader (dist/native-addon.js) in its two layouts:
//  - checkout: native/superscalar-napi.<triple>.node exists and is loaded first;
//  - published: the addon comes from the platform package @superscalar/<triple>,
//    named exactly as the release pipeline publishes it.
// Standalone CLI runner; its console output is the report.
/* eslint-disable no-console */
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");

const {
  NATIVE_BINARY_NAME,
  NATIVE_PACKAGE_SCOPE,
  NATIVE_TRIPLES,
  loadNativeAddon,
  nativePackageName,
  nativeTriple,
} = require("../dist/native-addon.js");
const { napiBackend, loadBackend } = require("../dist/backend.js");

// The four platform packages, exactly as release.yml packs and publishes them.
assert.equal(NATIVE_PACKAGE_SCOPE, "@superscalar");
assert.deepEqual(
  NATIVE_TRIPLES.map(nativePackageName),
  [
    "@superscalar/darwin-arm64",
    "@superscalar/darwin-x64",
    "@superscalar/linux-x64-gnu",
    "@superscalar/linux-arm64-gnu",
  ],
);

// This host is one of the four, and the .node the napi CLI built is in place.
const triple = nativeTriple();
assert.ok(NATIVE_TRIPLES.includes(triple), triple);
const nativeDir = path.join(__dirname, "..", "native");
const local = path.join(nativeDir, `${NATIVE_BINARY_NAME}.${triple}.node`);
assert.ok(fs.existsSync(local), `missing ${local}; run ts_smoke.sh first`);

// Checkout layout: the local file wins and is a working addon.
const addon = loadNativeAddon(require, nativeDir);
assert.equal(typeof addon.parse, "function");
assert.equal(typeof napiBackend().parse, "function");
assert.equal(typeof loadBackend().coerceLenient, "function");

// Published layout: no local file, so the loader requires @superscalar/<triple>.
// The package is not installed here; the error must name it and nothing else.
const missing = path.join(__dirname, "no-such-native-dir");
assert.throws(
  () => loadNativeAddon(require, missing),
  (err) => {
    assert.match(err.message, new RegExp(`^superscalar: cannot load the native addon package ${nativePackageName(triple)} `));
    assert.doesNotMatch(err.message, /superscalar-(darwin|linux)-/);
    return true;
  },
);

console.log(`ts native-addon: ok (${triple} -> ${nativePackageName(triple)})`);
