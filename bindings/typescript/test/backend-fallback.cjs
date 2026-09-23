// The Node backend prefers the napi addon and falls back to the wasm build when
// no addon loads: a host without a prebuilt binary for its platform (a musl
// Linux, where the published addons are glibc), or a bundle that resolves the
// package but not an addon. The fallback is proved in child processes against a
// copy of the package that has no native/ directory and no node_modules, for
// both the CJS and the ESM entry, so the test cannot pass by accident through
// the real addon that sits beside this file.
// Standalone CLI runner; its console output is the report.
/* eslint-disable no-console */
const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const pkg = path.resolve(__dirname, "..");
const { firstAvailableBackend, loadBackend, wasmBackend } = require("../dist/backend.js");

// Selection order and failure reporting, with stand-in loaders.
const wasm = wasmBackend();
assert.equal(
  firstAvailableBackend([() => { throw new Error("no addon"); }, () => wasm]),
  wasm,
  "the next loader is used when the first throws",
);
let laterCalls = 0;
assert.equal(
  firstAvailableBackend([() => wasm, () => { laterCalls += 1; return wasm; }]),
  wasm,
);
assert.equal(laterCalls, 0, "a later loader is not tried once one succeeded");
assert.throws(
  () => firstAvailableBackend([
    () => { throw new Error("no addon"); },
    () => { throw new Error("no wasm"); },
  ]),
  /no scalar backend could be loaded: no addon; no wasm/,
  "a total failure names every attempt",
);
assert.equal(typeof loadBackend().parse, "function");

// A package copy with dist/ and wasm-node/ but no native/: the relative addon
// path is missing, @superscalar/<triple> does not resolve (no node_modules
// above a temp directory), and wasm-node/ carries the backend.
// Serialized into the child, so it must be self-contained: 67 is AgentSkill.Name.
const probe = (backend) => {
  backend.validate(67, "careful-refactors");
  let refused = false;
  try {
    backend.validate(67, "Not A Skill Name!");
  } catch {
    refused = true;
  }
  if (!refused) throw new Error("the fallback backend accepted an invalid scalar");
};
const copy = fs.mkdtempSync(path.join(os.tmpdir(), "superscalar-fallback-"));
try {
  fs.mkdirSync(path.join(copy, "dist", "esm"), { recursive: true });
  for (const file of ["backend.js", "native-addon.js"]) {
    fs.copyFileSync(path.join(pkg, "dist", file), path.join(copy, "dist", file));
    fs.copyFileSync(path.join(pkg, "dist", "esm", file), path.join(copy, "dist", "esm", file));
  }
  fs.copyFileSync(path.join(pkg, "dist", "esm", "package.json"), path.join(copy, "dist", "esm", "package.json"));
  fs.cpSync(path.join(pkg, "wasm-node"), path.join(copy, "wasm-node"), { recursive: true });
  fs.writeFileSync(
    path.join(copy, "package.json"),
    JSON.stringify({ name: "superscalar-fallback-copy", private: true }),
  );
  const env = { ...process.env, NODE_PATH: "" };
  const cjs = execFileSync(process.execPath, [
    "-e",
    `const { loadBackend } = require(process.argv[1]); (${probe.toString()})(loadBackend()); console.log("cjs");`,
    path.join(copy, "dist", "backend.js"),
  ], { env, encoding: "utf8" });
  assert.equal(cjs.trim(), "cjs", "the CJS entry loads the wasm backend without an addon");
  const esm = execFileSync(process.execPath, [
    "--input-type=module",
    "-e",
    `const { loadBackend } = await import(process.argv[1]); (${probe.toString()})(loadBackend()); console.log("esm");`,
    `file://${path.join(copy, "dist", "esm", "backend.js")}`,
  ], { env, encoding: "utf8" });
  assert.equal(esm.trim(), "esm", "the ESM entry loads the wasm backend without an addon");
} finally {
  fs.rmSync(copy, { recursive: true, force: true });
}

// A bundled copy of the backend module: dist/ sits somewhere with no wasm-node/
// beside it, so the relative path misses, and the wasm bundle is reached by the
// package name through node_modules (the "./wasm-node/superscalar_wasm.js"
// export). Linked here the way a workspace or `npm link` links it.
const bundle = fs.mkdtempSync(path.join(os.tmpdir(), "superscalar-bundle-"));
try {
  fs.mkdirSync(path.join(bundle, "dist"), { recursive: true });
  for (const file of ["backend.js", "native-addon.js"]) {
    fs.copyFileSync(path.join(pkg, "dist", file), path.join(bundle, "dist", file));
  }
  fs.mkdirSync(path.join(bundle, "node_modules"), { recursive: true });
  fs.symlinkSync(pkg, path.join(bundle, "node_modules", "superscalar"), "dir");
  const env = { ...process.env, NODE_PATH: "" };
  const bundled = execFileSync(process.execPath, [
    "-e",
    `const { loadBackend } = require(process.argv[1]); (${probe.toString()})(loadBackend()); console.log("bundled");`,
    path.join(bundle, "dist", "backend.js"),
  ], { env, encoding: "utf8" });
  assert.equal(bundled.trim(), "bundled", "a bundled copy reaches the wasm bundle by package name");
} finally {
  fs.rmSync(bundle, { recursive: true, force: true });
}

console.log("ok - backend fallback: napi first, wasm when no addon loads (cjs + esm + bundled)");
