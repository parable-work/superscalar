// Isolation conformance for the TypeScript binding: the full v2 corpus runs
// through BOTH backends (napi and wasm), catching native-vs-WASM divergence.
// Both backend factories are exported from the CJS Node entry (dist/backend.js):
//  - napiBackend() binds the napi addon (native/index.js).
//  - wasmBackend() binds the nodejs-target wasm bundle (wasm-node/).
// Running the full corpus through each proves both cores agree and the wasm
// path is live (reachable, not dead code). The browser/edge consumer resolves
// `./backend` to dist/esm/backend.browser.mjs via the package `exports` map;
// that ESM browser bundle is compiled by ts_smoke.sh to verify it builds.
// Routing is by canonical -> id from the generated map (covers every built-in scalar);
// a spot check proves the generated branded wrappers route through the core.
//
// This is a standalone CLI test runner: its console output IS the conformance
// report (pass/fail lines + the final ok/FAIL summary), so `no-console` is
// disabled for the whole file by design.
/* eslint-disable no-console */
const fs = require("node:fs");
const path = require("node:path");

const {
  scalarIdByCanonical,
  parseContactEmail,
  SCALAR_METADATA_BY_CANONICAL,
} = require("../dist/generated.js");
// validateNetworkUrl is the generated public wrapper over the Rust-backed core.
// Exercise it directly so the regression test covers the package export surface.
const { validateNetworkUrl } = require("../dist/generated.js");
// Both backend factories come from the CJS Node entry; wasmBackend() loads the
// nodejs-target wasm bundle so the wasm path is exercised end-to-end in Node.
const { napiBackend, wasmBackend } = require("../dist/backend.js");

const corpus = JSON.parse(
  fs.readFileSync(path.join(__dirname, "..", "..", "..", "conformance", "core-scalars.v2.json"), "utf8"),
);

function run(name, be) {
  let accepted = 0;
  let rejected = 0;
  let skipped = 0;
  let failures = 0;
  for (const [canonical, data] of Object.entries(corpus.scalars)) {
    const id = scalarIdByCanonical[canonical];
    for (const c of data.accepted || []) {
      // `unresolved` vectors are the corpus's single source of truth for cases
      // the core has not reconciled yet; the Rust parity runner (crates/core/tests/
      // parity.rs) skips them, so this binding conformance runner skips them too
      // to stay in lockstep with that gate.
      if (c.unresolved) {
        skipped++;
        continue;
      }
      try {
        const got = be.parse(id, c.input);
        if (c.normalized !== undefined && got !== c.normalized) {
          failures++;
          console.error(`${name} ${canonical} parse(${JSON.stringify(c.input)}) = ${JSON.stringify(got)}, want ${JSON.stringify(c.normalized)}`);
        }
        accepted++;
      } catch (e) {
        failures++;
        console.error(`${name} ${canonical} parse(${JSON.stringify(c.input)}) unexpected error: ${e.message}`);
      }
    }
    for (const c of data.rejected || []) {
      // Skip over-acceptance gaps (G2): the corpus pins the INTENDED rejection
      // but the scalar is still declared as unconstrained, so the core
      // legitimately accepts the input. The Rust parity gate skips these too
      // (crates/core/tests/parity.rs).
      if (c.unresolved) {
        skipped++;
        continue;
      }
      try {
        be.parse(id, c.input);
        failures++;
        console.error(`${name} ${canonical} parse(${JSON.stringify(c.input)}) unexpectedly accepted`);
      } catch (e) {
        rejected++;
      }
    }
  }
  console.log(`${name}: ${accepted} accepted, ${rejected} rejected, ${skipped} skipped (unresolved), ${failures} failures`);
  return failures;
}

let failures = 0;
// napiBackend() binds the napi addon; wasmBackend() binds the nodejs-target
// wasm bundle. Running the full corpus through each proves both cores agree and
// the wasm path is live (reachable, not dead code).
failures += run("napi", napiBackend());
failures += run("wasm", wasmBackend());

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

// Generated branded wrapper routes through the (napi) core.
if (parseContactEmail("Foo@Bar.com") !== "foo@bar.com") {
  failures++;
  console.error("generated wrapper parseContactEmail did not route correctly");
}

// Network.Url JS-path behavior + ReDoS regression. validateNetworkUrl returns
// [ok, errors]; ok is true when the value passes. These lock the JS RegExp so a
// future pattern edit cannot silently change accept/reject behavior.
function urlOk(value) {
  const result = validateNetworkUrl(value);
  return result[0] === true;
}
const urlAccept = [
  "https://www.example.com/example/path",
  "https://{subdomain}.example.com/v1?x=1#frag",
  "https://a.b:8080/p",
  "http://example.com",
];
const urlReject = ["ftp://x", "https://nodot", "noturl"];
for (const v of urlAccept) {
  if (!urlOk(v)) {
    failures++;
    console.error(`Network.Url JS path rejected a valid URL: ${v}`);
  }
}
for (const v of urlReject) {
  if (urlOk(v)) {
    failures++;
    console.error(`Network.Url JS path accepted an invalid URL: ${v}`);
  }
}

// ReDoS regression: the prior polynomial pattern hung on "http://-.-" followed
// by many "--" then a non-URL char (the trailing char forces backtracking; a
// matching input never backtracks). The linear rewrite rejects it immediately.
const redosInputs = [
  "http://-.-" + "--".repeat(50000) + " ",
  "http://-.-" + "-".repeat(100000) + " ",
];
for (const input of redosInputs) {
  const start = Date.now();
  const ok = urlOk(input);
  const elapsed = Date.now() - start;
  if (ok) {
    failures++;
    console.error("ReDoS input unexpectedly accepted as a valid URL");
  }
  if (elapsed > 100) {
    failures++;
    console.error(`ReDoS regression: validateNetworkUrl took ${elapsed}ms (want < 100ms)`);
  }
}

if (failures > 0) {
  console.error(`FAIL: ${failures} conformance failures`);
  process.exit(1);
}
console.log("ts conformance: ok (napi + wasm)");
