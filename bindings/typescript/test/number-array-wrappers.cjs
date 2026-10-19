// A scalar whose TypeScript type is `number[]` must come back from its generated
// wrappers as an array of JavaScript numbers. The core hands back canonical JSON
// text on both paths: parse and normalize return text, and the lenient coerce
// returns text too because Embedding.Vector's metadata primitive is a string.
// The wrappers once passed that text through, so
// parseEmbeddingVectorStrict("[0.5,1.25]") returned the string "[0.5,1.25]".
// Every number[] scalar's conformance vectors run through the generated parse,
// normalize and strict wrappers here.
//
// Standalone CLI test runner: its console output is the report, so
// `no-console` is disabled for the whole file.
/* eslint-disable no-console */
const fs = require("node:fs");
const path = require("node:path");

const generated = require("../dist/generated.js");

const corpus = JSON.parse(
  fs.readFileSync(path.join(__dirname, "..", "..", "..", "conformance", "core-scalars.v2.json"), "utf8"),
);

let failures = 0;
function fail(message) {
  failures++;
  console.error(message);
}

function expectNumberArray(call, got, want) {
  if (!Array.isArray(got)) {
    fail(`${call} returned ${typeof got} ${JSON.stringify(got)}, want an array`);
    return;
  }
  const bad = got.findIndex(item => typeof item !== "number");
  if (bad !== -1) {
    fail(`${call} element ${bad} is ${typeof got[bad]} ${JSON.stringify(got[bad])}, want a number`);
    return;
  }
  if (got.length !== want.length || got.some((item, i) => item !== want[i])) {
    fail(`${call} = ${JSON.stringify(got)}, want ${JSON.stringify(want)}`);
  }
}

const arrayScalars = generated.SCALAR_METADATA.filter(meta => meta.tsType === "number[]");
const covered = new Set();
for (const { canonicalName, symbol } of arrayScalars) {
  const vectors = corpus.scalars[canonicalName];
  if (!vectors) {
    fail(`${canonicalName} has a number[] type but no conformance vectors`);
    continue;
  }
  const strict = generated[`parse${symbol}Strict`];
  const normalizeStrict = generated[`normalize${symbol}Strict`];
  const lenient = generated[`parse${symbol}`];
  const normalize = generated[`normalize${symbol}`];
  for (const c of vectors.accepted || []) {
    if (c.unresolved) continue;
    const want = JSON.parse(c.normalized ?? c.input);
    const text = JSON.stringify(c.input);
    expectNumberArray(`parse${symbol}Strict(${text})`, strict(c.input), want);
    expectNumberArray(`normalize${symbol}Strict(${text})`, normalizeStrict(c.input), want);
    expectNumberArray(`parse${symbol}(${text})`, lenient(c.input), want);
    expectNumberArray(`normalize${symbol}(${text})`, normalize(c.input), want);
    expectNumberArray(`parse${symbol}(${JSON.stringify(want)})`, lenient(want), want);
    expectNumberArray(`normalize${symbol}(${JSON.stringify(want)})`, normalize(want), want);
    covered.add(canonicalName);
  }
  for (const c of vectors.rejected || []) {
    if (c.unresolved) continue;
    let threw = false;
    try {
      strict(c.input);
    } catch {
      threw = true;
    }
    if (!threw) fail(`parse${symbol}Strict(${JSON.stringify(c.input)}) accepted a rejected vector`);
    if (lenient(c.input) !== null) {
      fail(`parse${symbol}(${JSON.stringify(c.input)}) accepted a rejected vector`);
    }
  }
}

// The reported case, as text and as an already-decoded array.
expectNumberArray('parseEmbeddingVectorStrict("[0.5,1.25]")', generated.parseEmbeddingVectorStrict("[0.5,1.25]"), [0.5, 1.25]);
expectNumberArray("parseEmbeddingVector([0.5, 1.25])", generated.parseEmbeddingVector([0.5, 1.25]), [0.5, 1.25]);
expectNumberArray('parseEmbeddingVector("[0.5,1.25]")', generated.parseEmbeddingVector("[0.5,1.25]"), [0.5, 1.25]);
if (!covered.has("Embedding.Vector")) fail("no accepted Embedding.Vector vector ran");

if (failures > 0) {
  console.error(`FAIL: ${failures} number[] wrapper failures`);
  process.exit(1);
}
console.log(`ts number[] wrappers: ok (${covered.size} scalars)`);
