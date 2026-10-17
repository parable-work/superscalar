// A scalar whose TypeScript type is `number` must come back from its generated
// wrappers as a JavaScript number. The core's parse returns canonical text, and
// the strict wrappers once passed that text through, so
// parseOrderingRankStrict("1") returned the string "1". Every number scalar's
// conformance vectors run through the generated parse, normalize and strict
// wrappers here.
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

function expectNumber(call, got, want) {
  if (typeof got !== "number") {
    fail(`${call} returned ${typeof got} ${JSON.stringify(got)}, want a number`);
  } else if (got !== want) {
    fail(`${call} = ${got}, want ${want}`);
  }
}

const numberScalars = generated.SCALAR_METADATA.filter(meta => meta.tsType === "number");
const covered = new Set();
for (const { canonicalName, symbol } of numberScalars) {
  const vectors = corpus.scalars[canonicalName];
  if (!vectors) {
    fail(`${canonicalName} has a number type but no conformance vectors`);
    continue;
  }
  const strict = generated[`parse${symbol}Strict`];
  const normalizeStrict = generated[`normalize${symbol}Strict`];
  const lenient = generated[`parse${symbol}`];
  for (const c of vectors.accepted || []) {
    if (c.unresolved) continue;
    const want = Number(c.normalized ?? c.input);
    expectNumber(`parse${symbol}Strict(${JSON.stringify(c.input)})`, strict(c.input), want);
    expectNumber(`normalize${symbol}Strict(${JSON.stringify(c.input)})`, normalizeStrict(c.input), want);
    expectNumber(`parse${symbol}(${JSON.stringify(c.input)})`, lenient(c.input), want);
    expectNumber(`parse${symbol}(${want})`, lenient(want), want);
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

// The reported case, both as text and as a number.
expectNumber('parseOrderingRankStrict("1")', generated.parseOrderingRankStrict("1"), 1);
expectNumber("parseOrderingRank(1)", generated.parseOrderingRank(1), 1);
expectNumber('parseOrderingRank("1")', generated.parseOrderingRank("1"), 1);
if (!covered.has("Ordering.Rank")) fail("no accepted Ordering.Rank vector ran");

if (failures > 0) {
  console.error(`FAIL: ${failures} number wrapper failures`);
  process.exit(1);
}
console.log(`ts number wrappers: ok (${covered.size} scalars)`);
