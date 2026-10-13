// Isolation conformance for the TypeScript binding: the full v2 corpus runs
// through BOTH Node backends (napi and wasm), catching native-vs-WASM
// divergence. Both backend factories are exported from the CJS Node entry
// (dist/backend.js):
//  - napiBackend() binds the napi addon (native/superscalar-napi.<triple>.node
//    in this checkout; @superscalar/<triple> in a published install).
//  - wasmBackend() binds the nodejs-target wasm bundle (wasm-node/).
// Running the full corpus through each proves both cores agree and the wasm
// path is live (reachable, not dead code). The browser and workerd builds run
// the same checks (corpus-runner.cjs) in web-conformance.mjs and the workerd
// smoke. Routing is by canonical name, the only scalar identity (covers every
// built-in scalar); a spot check proves the generated branded wrappers route
// through the core.
//
// This is a standalone CLI test runner: its console output IS the conformance
// report (pass/fail lines + the final ok/FAIL summary), so `no-console` is
// disabled for the whole file by design.
/* eslint-disable no-console */
const fs = require("node:fs");
const path = require("node:path");

const generated = require("../dist/generated.js");
const { VALID_SCALARS, SCALAR_METADATA_BY_CANONICAL } = generated;
// Both backend factories come from the CJS Node entry; wasmBackend() loads the
// nodejs-target wasm bundle so the wasm path is exercised end-to-end in Node.
const { napiBackend, wasmBackend } = require("../dist/backend.js");
const { runCorpus, checkUnknownName, checkWrappers } = require("./corpus-runner.cjs");

const corpus = JSON.parse(
  fs.readFileSync(path.join(__dirname, "..", "..", "..", "conformance", "core-scalars.v2.json"), "utf8"),
);

let failures = 0;
// The corpus names exactly the registry, so runCorpus reaches every scalar.
const corpusNames = Object.keys(corpus.scalars).sort();
if (JSON.stringify(corpusNames) !== JSON.stringify([...VALID_SCALARS])) {
  failures++;
  console.error(`corpus scalars ${JSON.stringify(corpusNames)} differ from VALID_SCALARS ${JSON.stringify(VALID_SCALARS)}`);
}
// An unknown or differently cased name throws through both backends.
for (const [name, be] of [["napi", napiBackend()], ["wasm", wasmBackend()]]) {
  failures += checkUnknownName(name, be);
}
// napiBackend() binds the napi addon; wasmBackend() binds the nodejs-target
// wasm bundle. Running the full corpus through each proves both cores agree and
// the wasm path is live (reachable, not dead code).
failures += runCorpus("napi", napiBackend(), corpus);
failures += runCorpus("wasm", wasmBackend(), corpus);

// Scalar metadata is a static generated table in dist/generated.js, shared by
// both backends -- napi and wasm differ only in the parse/normalize/validate
// core they bind. Asserting it per backend would check the same object twice
// and imply a backend dependency that does not exist.
const metaVectors = corpus.metadata || {};
if (Object.keys(metaVectors).length !== Object.keys(SCALAR_METADATA_BY_CANONICAL).length) {
  failures++;
  console.error("metadata vector count does not match the generated table");
}
let nonSortable = 0;
for (const [canonical, want] of Object.entries(metaVectors)) {
  const got = SCALAR_METADATA_BY_CANONICAL[canonical];
  if (!got) {
    failures++;
    console.error(`no metadata row for ${canonical}`);
    continue;
  }
  if (got.comparabilityClass !== want.comparability_class) {
    failures++;
    console.error(`${canonical} comparabilityClass = ${JSON.stringify(got.comparabilityClass)}, want ${JSON.stringify(want.comparability_class)}`);
  }
  if (got.isSortable !== want.is_sortable) {
    failures++;
    console.error(`${canonical} isSortable = ${got.isSortable}, want ${want.is_sortable}`);
  }
  if (got.caseInsensitive !== want.case_insensitive) {
    failures++;
    console.error(`${canonical} caseInsensitive = ${got.caseInsensitive}, want ${want.case_insensitive}`);
  }
  if (JSON.stringify(got.reservedWords) !== JSON.stringify(want.reserved_words)) {
    failures++;
    console.error(`${canonical} reservedWords = ${JSON.stringify(got.reservedWords)}, want ${JSON.stringify(want.reserved_words)}`);
  }
  if (got.reservedWordsCaseInsensitive !== want.reserved_words_case_insensitive) {
    failures++;
    console.error(`${canonical} reservedWordsCaseInsensitive = ${got.reservedWordsCaseInsensitive}, want ${want.reserved_words_case_insensitive}`);
  }
  if (got.reservedWordsMatchPartial !== want.reserved_words_match_partial) {
    failures++;
    console.error(`${canonical} reservedWordsMatchPartial = ${got.reservedWordsMatchPartial}, want ${want.reserved_words_match_partial}`);
  }
  if (!want.is_sortable) {
    nonSortable++;
  }
}
// Independent of the row loop above, which compares a transcript against the
// table it was transcribed from.
if (nonSortable !== corpus.meta.non_sortable_count) {
  failures++;
  console.error(`non-sortable metadata rows = ${nonSortable}, want ${corpus.meta.non_sortable_count}`);
}
// Same independence argument, for comparability. Rows with no class are skipped
// rather than grouped under a null key, so the derived map holds only real
// classes.
// Prototype-free: a comparability_class literally named __proto__ or constructor would
// otherwise resolve through Object.prototype, so ||= would keep the inherited value and
// .push() would throw before the parity check could report the mismatch.
const derivedClasses = Object.create(null);
for (const [canonical, want] of Object.entries(metaVectors)) {
  if (want.comparability_class === null || want.comparability_class === undefined) {
    continue;
  }
  (derivedClasses[want.comparability_class] ||= []).push(canonical);
}
for (const members of Object.values(derivedClasses)) {
  members.sort();
}
// Guard the corpus lookup: an absent `meta.comparability_classes` must become a
// COUNTED failure, not a TypeError out of Object.keys(undefined), or the runner
// dies without reporting which check died. Removing a class is by construction
// the next thing that edits this key.
const handClasses = corpus.meta.comparability_classes;
if (handClasses === undefined || handClasses === null) {
  failures++;
  console.error("corpus meta.comparability_classes is missing; it is hand-maintained and the generator must never remove it");
} else {
  const derivedClassesJson = JSON.stringify(derivedClasses, Object.keys(derivedClasses).sort());
  const handClassesJson = JSON.stringify(handClasses, Object.keys(handClasses).sort());
  if (derivedClassesJson !== handClassesJson) {
    failures++;
    console.error(`comparability classes = ${derivedClassesJson}, want ${handClassesJson}`);
  }
}
// Same independence argument, for the case and reserved-word fields, with the
// same guard against an absent hand-maintained key.
const caseInsensitive = Object.keys(metaVectors)
  .filter((canonical) => metaVectors[canonical].case_insensitive === true)
  .sort();
const handCaseInsensitive = corpus.meta.case_insensitive_scalars;
if (!Array.isArray(handCaseInsensitive)) {
  failures++;
  console.error("corpus meta.case_insensitive_scalars is missing; it is hand-maintained and the generator must never remove it");
} else if (JSON.stringify(caseInsensitive) !== JSON.stringify(handCaseInsensitive)) {
  failures++;
  console.error(`case-insensitive rows = ${JSON.stringify(caseInsensitive)}, want ${JSON.stringify(handCaseInsensitive)}`);
}
const reservedRule = (row) => ({
  reserved_words: row.reserved_words,
  reserved_words_case_insensitive: row.reserved_words_case_insensitive,
  reserved_words_match_partial: row.reserved_words_match_partial,
});
const derivedReserved = Object.create(null);
for (const canonical of Object.keys(metaVectors).sort()) {
  const row = metaVectors[canonical];
  if (row.reserved_words.length > 0 || row.reserved_words_case_insensitive || row.reserved_words_match_partial) {
    derivedReserved[canonical] = reservedRule(row);
  }
}
const handReserved = corpus.meta.reserved_word_scalars;
if (handReserved === undefined || handReserved === null) {
  failures++;
  console.error("corpus meta.reserved_word_scalars is missing; it is hand-maintained and the generator must never remove it");
} else {
  const handReservedSorted = Object.create(null);
  for (const canonical of Object.keys(handReserved).sort()) {
    handReservedSorted[canonical] = reservedRule(handReserved[canonical]);
  }
  const derivedReservedJson = JSON.stringify(derivedReserved);
  const handReservedJson = JSON.stringify(handReservedSorted);
  if (derivedReservedJson !== handReservedJson) {
    failures++;
    console.error(`reserved-word rows = ${derivedReservedJson}, want ${handReservedJson}`);
  }
}
// set(scalars) - set(metadata) === set(metadata_excluded), not the weaker
// disjointness check: disjointness alone passes if a scalar is missing from
// `metadata` without being declared excluded, which is the drift that matters.
const missingFromMetadata = Object.keys(corpus.scalars)
  .filter((canonical) => !(canonical in metaVectors))
  .sort();
const declaredExcluded = (corpus.metadata_excluded || []).slice().sort();
if (missingFromMetadata.join(",") !== declaredExcluded.join(",")) {
  failures++;
  console.error(`scalars minus metadata = ${JSON.stringify(missingFromMetadata)}, want declared metadata_excluded ${JSON.stringify(declaredExcluded)}`);
}

// The generated wrappers route through the (napi) core.
failures += checkWrappers(generated);

if (failures > 0) {
  console.error(`FAIL: ${failures} conformance failures`);
  process.exit(1);
}
console.log("ts conformance: ok (napi + wasm)");
