// Finishes an ESM tree tsc emitted with moduleResolution: bundler: appends
// `.js` to extensionless relative imports, which Node ESM rejects at runtime
// (ERR_MODULE_NOT_FOUND), moves a backend authored under a variant name into
// place, and marks the directory as ESM. generated.js is @generated and must
// not be hand-edited at the source, so the fix is applied to the emitted
// output. Idempotent: a specifier that already ends in .js, .mjs, .json or
// .wasm is skipped. Used by fix-esm-extensions.mjs (dist/esm) and
// build-web.mjs (dist/browser, dist/workerd).
import { readdirSync, readFileSync, writeFileSync, statSync, renameSync, existsSync } from "node:fs";
import { join, dirname, resolve } from "node:path";

function fixFile(file) {
  const original = readFileSync(file, "utf8");
  const fromDir = dirname(file);
  // Match `from "./x"` / `from "../x"` without an extension. Append `/index.js`
  // when the specifier resolves to a directory, otherwise `.js`.
  const fixed = original.replace(
    /(from\s+["'])(\.\.?\/[^"']*?)(["'])/g,
    (match, prefix, spec, suffix) => {
      if (/\.(m?js|json|wasm)$/.test(spec)) {
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

// Renames each emitted [from, to] pair inside `dir` (a backend authored as
// backend.<variant>.ts becomes backend.js, which generated.js imports), marks
// the directory as ESM so Node reads its .js files as modules without the
// MODULE_TYPELESS_PACKAGE_JSON reparse warning (the package root stays CJS),
// and fixes every relative specifier.
export function fixEsmTree(dir, renames) {
  for (const [from, to] of renames) {
    const src = join(dir, from);
    if (existsSync(src)) {
      renameSync(src, join(dir, to));
    }
  }
  writeFileSync(join(dir, "package.json"), '{\n  "type": "module"\n}\n');
  walk(dir);
}
