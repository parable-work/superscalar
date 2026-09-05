// Append `.js` to extensionless relative imports in the ESM build (dist/esm).
// tsc with moduleResolution: bundler emits extensionless specifiers, which Node
// ESM rejects at runtime (ERR_MODULE_NOT_FOUND). generated.js is @generated and
// must not be hand-edited at the source, so the fix is applied to the emitted
// output. Idempotent: a specifier that already ends in .js or .mjs is skipped.
import { readdirSync, readFileSync, writeFileSync, statSync, renameSync, existsSync } from "node:fs";
import { join, dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const esmDir = join(dirname(fileURLToPath(import.meta.url)), "..", "dist", "esm");

// The ESM napi backend is authored as backend.node.ts so the CJS pass can skip
// it; generated.js imports `./backend.js`, so rename the emitted file (and its
// declaration) into place before fixing extensions.
for (const [from, to] of [
  ["backend.node.js", "backend.js"],
  ["backend.node.d.ts", "backend.d.ts"],
]) {
  const src = join(esmDir, from);
  if (existsSync(src)) {
    renameSync(src, join(esmDir, to));
  }
}

function fixFile(file) {
  const original = readFileSync(file, "utf8");
  const fromDir = dirname(file);
  // Match `from "./x"` / `from "../x"` without an extension. Append `/index.js`
  // when the specifier resolves to a directory, otherwise `.js`.
  const fixed = original.replace(
    /(from\s+["'])(\.\.?\/[^"']*?)(["'])/g,
    (match, prefix, spec, suffix) => {
      if (/\.(m?js|json)$/.test(spec)) {
        return match;
      }
      const resolved = resolve(fromDir, spec);
      if (existsSync(resolved) && statSync(resolved).isDirectory()) {
        return `${prefix}${spec}/index.js${suffix}`;
      }
      return `${prefix}${spec}.js${suffix}`;
    }
  );
  if (fixed !== original) {
    writeFileSync(file, fixed);
  }
}

function walk(dir) {
  for (const entry of readdirSync(dir)) {
    const full = join(dir, entry);
    if (statSync(full).isDirectory()) {
      walk(full);
    } else if (full.endsWith(".js")) {
      fixFile(full);
    }
  }
}

// backend.browser.mjs lives in dist/esm/ (one level deeper than the source's
// src/), so its source-relative ../wasm-bundler must become ../../wasm-bundler.
const browserBackend = join(esmDir, "backend.browser.mjs");
if (existsSync(browserBackend)) {
  const text = readFileSync(browserBackend, "utf8").replace(
    /(["'])\.\.\/wasm-bundler\//g,
    "$1../../wasm-bundler/"
  );
  writeFileSync(browserBackend, text);
}

// Mark dist/esm as ESM so Node treats the .js files as modules without the
// MODULE_TYPELESS_PACKAGE_JSON reparse warning (the package root stays CJS).
writeFileSync(join(esmDir, "package.json"), '{\n  "type": "module"\n}\n');

walk(esmDir);
