// The checks every backend of the TypeScript binding passes, shared by
// conformance.cjs (napi and wasm-node, in Node), web-conformance.mjs (the
// browser build, in Node) and the workerd smoke (the workerd build, in
// workerd). It uses no Node API, since workerd runs it, and its console output
// is the report, so `no-console` is disabled for the whole file by design.
/* eslint-disable no-console */

// The full v2 corpus through `be`, by canonical name, the only scalar identity
// (every built-in scalar). Returns the number of failures.
function runCorpus(name, be, corpus) {
  let accepted = 0;
  let rejected = 0;
  let skipped = 0;
  let failures = 0;
  for (const [canonical, data] of Object.entries(corpus.scalars)) {
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
        const got = be.parse(canonical, c.input);
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
        be.parse(canonical, c.input);
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

// An unknown or differently cased name throws.
function checkUnknownName(name, be) {
  let failures = 0;
  try {
    be.parse("contact.email", "a@b.com");
    failures++;
    console.error(`${name}: parse under an unknown name unexpectedly succeeded`);
  } catch (e) {
    if (!String(e.message).includes('unknown scalar "contact.email"')) {
      failures++;
      console.error(`${name}: unknown name threw ${JSON.stringify(e.message)}`);
    }
  }
  return failures;
}

// The generated wrappers over the backend that `api`, the package's generated
// module, was built with: the canonical forms they hand back and the inputs
// they refuse.
function checkWrappers(api) {
  const {
    parseContactEmail,
    parseGenericJSON,
    parseGenericJSONStrict,
    normalizeGenericJSON,
    validateGenericJSON,
    parseGenericStringMap,
    parseGenericStringMapStrict,
    parseGeoLocation,
    parseGeoLocationStrict,
    validateGeoLocation,
    validateNetworkUrl,
  } = api;
  let failures = 0;

  // A generated branded wrapper routes through the core.
  if (parseContactEmail("Foo@Bar.com") !== "foo@bar.com") {
    failures++;
    console.error("generated wrapper parseContactEmail did not route correctly");
  }

  // Generic.StringMap crosses the core as JSON text; the generated wrappers hand
  // back the canonical map as an object, not as that text.
  for (const [label, got] of [
    ["parseGenericStringMapStrict", parseGenericStringMapStrict('{"b":"2","a":"1"}')],
    ["parseGenericStringMap", parseGenericStringMap({ b: "2", a: "1" })],
  ]) {
    if (typeof got !== "object" || got === null || JSON.stringify(got) !== '{"a":"1","b":"2"}') {
      failures++;
      console.error(`generated wrapper ${label} returned ${JSON.stringify(got)}, want the object {"a":"1","b":"2"}`);
    }
  }

  // Geo.Location crosses the core as JSON text too: the wrappers take the
  // object or its text and hand back the canonical {lat, lon} object; anything
  // else, the old "lat,lon" string included, is refused.
  for (const [label, got] of [
    ["parseGeoLocationStrict", parseGeoLocationStrict(' { "lon": -122.4194, "lat": 37.7749 } ')],
    ["parseGeoLocation", parseGeoLocation({ lon: -122.4194, lat: 37.7749 })],
  ]) {
    if (typeof got !== "object" || got === null || JSON.stringify(got) !== '{"lat":37.7749,"lon":-122.4194}') {
      failures++;
      console.error(`generated wrapper ${label} returned ${JSON.stringify(got)}, want the object {"lat":37.7749,"lon":-122.4194}`);
    }
  }
  for (const [value, wantValid] of [
    [{ lat: 90, lon: -180 }, true],
    ['{"lat":0,"lon":0}', true],
    [{ lat: 90.5, lon: 0 }, false],
    [{ lat: 1, lon: 2, alt: 3 }, false],
    [{ lat: "1", lon: 2 }, false],
    ["37.7749,-122.4194", false],
    [[37.7749, -122.4194], false],
    [42, false],
  ]) {
    const [valid] = validateGeoLocation(value);
    if (valid !== wantValid) {
      failures++;
      console.error(`validateGeoLocation(${JSON.stringify(value)}) valid = ${valid}, want ${wantValid}`);
    }
  }
  if (parseGeoLocation("37.7749,-122.4194") !== null) {
    failures++;
    console.error('parseGeoLocation("37.7749,-122.4194") accepted the retired "lat,lon" string');
  }

  // Generic.JSON is any JSON value, explicit null included: the generated
  // wrappers keep every root, keep null distinct from absence (undefined), and
  // refuse host values that are not portable JSON.
  const genericJSONRoots = [
    [`{"k":1}`, { k: 1 }],
    [`[1,2]`, [1, 2]],
    [`"text"`, "text"],
    ["42", 42],
    ["true", true],
    ["null", null],
  ];
  for (const [encoded, expected] of genericJSONRoots) {
    const strict = parseGenericJSONStrict(encoded);
    if (JSON.stringify(strict) !== JSON.stringify(expected)) {
      failures++;
      console.error(
        `generated wrapper parseGenericJSONStrict(${JSON.stringify(encoded)}) = ${JSON.stringify(strict)}, want ${JSON.stringify(expected)}`,
      );
    }

    const parsed = parseGenericJSON(expected);
    const normalized = normalizeGenericJSON(expected);
    const [valid, validationErrors] = validateGenericJSON(expected);
    if (
      JSON.stringify(parsed) !== JSON.stringify(expected) ||
      JSON.stringify(normalized) !== JSON.stringify(expected) ||
      !valid ||
      validationErrors !== null
    ) {
      failures++;
      console.error(`generated Generic.JSON runtime wrapper rejected ${JSON.stringify(expected)}`);
    }
  }

  if (parseGenericJSON(null) !== null || parseGenericJSON(undefined) !== undefined) {
    failures++;
    console.error("generated Generic.JSON wrapper collapsed explicit null and absence");
  }

  try {
    parseGenericJSONStrict("{");
    failures++;
    console.error("generated Generic.JSON strict wrapper accepted malformed JSON text");
  } catch {
    // Expected: strict textual parsing keeps malformed input distinct from null.
  }

  const cyclic = {};
  cyclic.self = cyclic;
  const sparse = [];
  sparse.length = 1;
  const invalidGenericJSONValues = [
    ["NaN", Number.NaN],
    ["Infinity", Number.POSITIVE_INFINITY],
    ["bigint", 1n],
    ["set", new Set([1])],
    ["undefined property", { value: undefined }],
    ["sparse array", sparse],
    ["function", () => undefined],
    ["cycle", cyclic],
  ];
  for (const [name, value] of invalidGenericJSONValues) {
    const [valid] = validateGenericJSON(value);
    if (parseGenericJSON(value) !== undefined || valid) {
      failures++;
      console.error(`generated Generic.JSON wrapper accepted invalid ${name}`);
    }
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

  return failures;
}

module.exports = { runCorpus, checkUnknownName, checkWrappers };
