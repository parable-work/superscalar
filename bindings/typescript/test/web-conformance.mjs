// The browser build (dist/browser) through the same checks as the Node
// backends (corpus-runner.cjs), run in Node, which has every API that build
// uses (atob, WebAssembly, TextEncoder). Also checks what the browser and
// workerd builds may import, and which build each export condition resolves
// to. The workerd build runs these checks in workerd itself
// (scripts/web_smoke.sh).
// Standalone CLI runner; its console output is the report.
/* eslint-disable no-console */
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readdirSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);
const { runCorpus, checkUnknownName, checkWrappers } = require("./corpus-runner.cjs");
const pkg = fileURLToPath(new URL("..", import.meta.url));
const corpus = JSON.parse(
  readFileSync(new URL("../../../conformance/core-scalars.v2.json", import.meta.url), "utf8"),
);

let failures = 0;

// Neither build reaches Node or outside its own directory: every static
// specifier names a sibling file, and nothing requires.
for (const variant of ["browser", "workerd"]) {
  const dir = new URL(`../dist/${variant}/`, import.meta.url);
  for (const file of readdirSync(dir).filter((name) => name.endsWith(".js"))) {
    const text = readFileSync(new URL(file, dir), "utf8");
    for (const [, spec] of text.matchAll(/\b(?:from|import)\s*["']([^"']+)["']/g)) {
      if (!/^\.\/[^/]+\.(js|wasm)$/.test(spec)) {
        failures++;
        console.error(`dist/${variant}/${file} imports ${JSON.stringify(spec)}, not a sibling module`);
      }
    }
    if (/\brequire\s*\(|["']node:/.test(text)) {
      failures++;
      console.error(`dist/${variant}/${file} requires a module or names a node: builtin`);
    }
  }
}

// Each condition takes every subpath into its own build; Node and Bun, which
// set neither, stay on the napi backend.
const subpaths = ["superscalar", "superscalar/scalars", "superscalar/validation", "superscalar/backend", "superscalar/wasm"];
const want = {
  browser: ["browser/index.js", "browser/generated.js", "browser/validation.js", "browser/backend.js", "browser/backend.js"],
  workerd: ["workerd/index.js", "workerd/generated.js", "workerd/validation.js", "workerd/backend.js", "workerd/backend.js"],
  "": ["esm/index.js", "esm/generated.js", "esm/validation.js", "esm/backend.js", "../wasm-node/superscalar_wasm.js"],
};
for (const [condition, files] of Object.entries(want)) {
  const flags = condition ? [`--conditions=${condition}`] : [];
  const resolved = JSON.parse(
    execFileSync(
      process.execPath,
      [...flags, "--input-type=module", "-e", `console.log(JSON.stringify(${JSON.stringify(subpaths)}.map((s) => import.meta.resolve(s))))`],
      { cwd: pkg, encoding: "utf8" },
    ),
  );
  resolved.forEach((url, index) => {
    const expected = new URL(`../dist/${files[index]}`, import.meta.url).href;
    if (url !== expected) {
      failures++;
      console.error(`${condition || "no condition"}: ${subpaths[index]} resolves to ${url}, want ${expected}`);
    }
  });
}

const started = performance.now();
const api = await import("../dist/browser/index.js");
const backendModule = await import("../dist/browser/backend.js");
console.log(`browser build: imported and instantiated in ${(performance.now() - started).toFixed(1)}ms`);

// One backend instance behind every entry.
assert.equal(api.loadBackend(), backendModule.backend);
assert.equal(api.wasmBackend(), backendModule.backend);
assert.throws(() => api.napiBackend(), /native addon loads only in Node/);

failures += checkUnknownName("browser", backendModule.backend);
failures += runCorpus("browser", backendModule.backend, corpus);
failures += checkWrappers(api);

// `superscalar/wasm` in this build: the raw exports, already instantiated.
assert.equal(backendModule.scalar_parse("Contact.Email", "Foo@Bar.com"), "foo@bar.com");
assert.throws(() => backendModule.scalar_parse("Contact.Email", "not-an-email"), { name: "ScalarParseError" });

if (failures > 0) {
  console.error(`FAIL: ${failures} browser build failures`);
  process.exit(1);
}
console.log("ts web conformance: ok (browser build in Node; export conditions)");
