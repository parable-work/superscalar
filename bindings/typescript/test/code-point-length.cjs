// codePointLength is the count a generated TypeScript validator uses for a
// `minLength` or `maxLength` rule. The core counts code points, and so must
// every language: `String.prototype.length` counts UTF-16 units, which is two
// for a character outside the Basic Multilingual Plane.
//
// Standalone CLI test runner: its console output is the report, so
// `no-console` is disabled for the whole file.
/* eslint-disable no-console */
const assert = require("node:assert");

const { codePointLength } = require("../dist/validation.js");
const { backend } = require("../dist/backend.js");

// One character outside the Basic Multilingual Plane (U+1F600): two UTF-16
// units, one code point.
const ASTRAL = "\u{1F600}";

const cases = [
  "",
  "abc",
  "\u00e9",
  "e\u0301",
  "\u65e5\u672c\u8a9e",
  ASTRAL,
  ASTRAL + "a" + ASTRAL,
  "\ud800",
  "\udc00",
  "\ud800a",
  "a\udc00\ud800",
];
for (const value of cases) {
  assert.strictEqual(codePointLength(value), [...value].length, JSON.stringify(value));
}
assert.strictEqual(ASTRAL.length, 2);
assert.strictEqual(codePointLength(ASTRAL), 1);

// The count agrees with the core. Auth.Password needs 8 characters: eight
// astral characters are 16 UTF-16 units and 8 code points, and the core
// accepts them as a password and refuses seven.
const eight = ASTRAL.repeat(8);
assert.strictEqual(codePointLength(eight), 8);
assert.doesNotThrow(() => backend.validate("Auth.Password", eight));
assert.throws(() => backend.validate("Auth.Password", ASTRAL.repeat(7)));

console.log(`ts codePointLength: ok (${cases.length} strings)`);
