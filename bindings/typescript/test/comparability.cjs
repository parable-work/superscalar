// Standalone CLI test runner: mirror of
// comparability_across_distinct_scalars_is_exactly_the_temporal_instant_pair in
// core/tests/semantic_metadata.rs. The Rust core owns the rule; this pins that
// the TypeScript binding exposes it, because SCALAR_METADATA ships
// comparabilityClass and the obvious consumer implementation over that field is
// wrong.
/* eslint-disable no-console */
const assert = require('node:assert');

const { comparableWith, SCALAR_METADATA_BY_CANONICAL } = require('../dist/generated.js');

let failures = 0;

function test(name, fn) {
  try {
    fn();
    console.log(`ok - ${name}`);
  } catch (error) {
    failures += 1;
    console.error(`FAIL - ${name}`);
    console.error(error && error.stack ? error.stack : String(error));
  }
}

test('a scalar is comparable with itself', () => {
  assert.strictEqual(comparableWith('Contact.Email', 'Contact.Email'), true);
  assert.strictEqual(comparableWith('Contact.PhoneNumber', 'Contact.PhoneNumber'), true);
});

test('two class-less scalars are not comparable', () => {
  // The spec's named counter-example (0084-semantic-types.mdx). Naive field
  // equality answers true here because both classes are null. This is the
  // assertion the whole ticket exists for.
  assert.strictEqual(comparableWith('Contact.Email', 'Contact.PhoneNumber'), false);
  assert.strictEqual(comparableWith('Contact.PhoneNumber', 'Contact.Email'), false);
  assert.strictEqual(
    SCALAR_METADATA_BY_CANONICAL['Contact.Email'].comparabilityClass,
    SCALAR_METADATA_BY_CANONICAL['Contact.PhoneNumber'].comparabilityClass,
    'classes differ; the assertions above no longer prove field equality is wrong',
  );
});

test('an alias is the same scalar', () => {
  // One implementation under two ids. Identity.UserID -> Identity.UUID is the
  // catalog's only alias pair.
  assert.strictEqual(comparableWith('Identity.UserID', 'Identity.UUID'), true);
  assert.strictEqual(comparableWith('Identity.UUID', 'Identity.UserID'), true);
});

test('a name with no metadata row fails closed', () => {
  assert.strictEqual(comparableWith('Not.AScalar', 'Not.AScalar'), false);
  assert.strictEqual(comparableWith('Not.AScalar', 'Contact.Email'), false);
});

test('the temporal_instant pair is comparable', () => {
  // The one named class: Temporal.Date and Temporal.DateTime share
  // temporal_instant, so the predicate answers true across them.
  assert.strictEqual(comparableWith('Temporal.Date', 'Temporal.DateTime'), true);
  assert.strictEqual(comparableWith('Temporal.DateTime', 'Temporal.Date'), true);
});

test('an inherited object property name is not a scalar', () => {
  // SCALAR_ALIAS_TARGETS and SCALAR_METADATA_BY_CANONICAL are plain objects,
  // so a bare index on "constructor" or "toString" returns an Object.prototype
  // member instead of undefined. The predicate must look up own properties
  // only, or these names compare equal to themselves and to each other.
  for (const name of ['constructor', 'toString', '__proto__', 'hasOwnProperty']) {
    assert.strictEqual(comparableWith(name, name), false, `${name} against itself`);
    assert.strictEqual(comparableWith(name, 'Contact.Email'), false, `${name} against a scalar`);
    assert.strictEqual(comparableWith('Contact.Email', name), false, `a scalar against ${name}`);
  }
  assert.strictEqual(comparableWith('constructor', 'toString'), false);
  assert.strictEqual(comparableWith('toString', 'constructor'), false);
});

process.exit(failures === 0 ? 0 : 1);
