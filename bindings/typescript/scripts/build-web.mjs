// Finishes the browser and workerd builds that `tsc -p tsconfig.browser.json`
// and `tsc -p tsconfig.workerd.json` emit into dist/browser and dist/workerd
// (`npm run build:web` runs all three). Needs wasm-web/, the wasm-pack `web`
// build of crates/wasm (scripts/build_wasm_web.sh).
//
// Each tree gets its backend moved to backend.js and its specifiers fixed
// (esm-tree.mjs), and a copy of the wasm-bindgen glue beside the backend, which
// imports it from ../wasm-web/ in the source. dist/workerd also gets the
// module itself, which its backend imports as ./superscalar_wasm_bg.wasm, and
// dist/browser gets wasm-inline.js, the module as base64. Neither tree reaches
// outside its own directory, so a bundler or a Worker upload takes it whole.
import { copyFileSync, existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { fixEsmTree } from "./esm-tree.mjs";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const wasmWeb = join(root, "wasm-web");
const GLUE = "superscalar_wasm.js";
const MODULE = "superscalar_wasm_bg.wasm";

for (const file of [GLUE, MODULE]) {
  if (!existsSync(join(wasmWeb, file))) {
    throw new Error(
      `build-web: ${join(wasmWeb, file)} is missing; wasm-web/ holds the wasm-pack web build ` +
        "(scripts/build_wasm_web.sh)",
    );
  }
}

// The glue's async init, which neither backend calls, defaults to fetching the
// module from new URL('superscalar_wasm_bg.wasm', import.meta.url). webpack and
// Vite resolve that URL when they bundle, and webpack fails when no file sits
// there, as in dist/browser. The default goes; initializing without a module
// throws instead.
const DEFAULT_URL = "module_or_path = new URL('superscalar_wasm_bg.wasm', import.meta.url);";
const glueSource = readFileSync(join(wasmWeb, GLUE), "utf8");
if (!glueSource.includes(DEFAULT_URL)) {
  throw new Error(
    `build-web: ${GLUE} no longer contains ${JSON.stringify(DEFAULT_URL)}; ` +
      "check what this wasm-bindgen version's init does without a module and update this script",
  );
}
const glue = glueSource.replace(
  DEFAULT_URL,
  'throw new Error("superscalar: its backend module initializes the WASM build; init needs a module");',
);

for (const variant of ["browser", "workerd"]) {
  const dir = join(root, "dist", variant);
  const emitted = join(dir, `backend.${variant}.js`);
  if (!existsSync(emitted)) {
    throw new Error(`build-web: ${emitted} is missing; run tsc -p tsconfig.${variant}.json first`);
  }
  fixEsmTree(dir, [[`backend.${variant}.js`, "backend.js"]]);
  const backendPath = join(dir, "backend.js");
  const backend = readFileSync(backendPath, "utf8");
  const local = backend.replace(/(["'])\.\.\/wasm-web\//g, "$1./");
  if (local === backend) {
    throw new Error(`build-web: ${backendPath} imports nothing from ../wasm-web/`);
  }
  writeFileSync(backendPath, local);
  writeFileSync(join(dir, GLUE), glue);
}

copyFileSync(join(wasmWeb, MODULE), join(root, "dist", "workerd", MODULE));

const bytes = readFileSync(join(wasmWeb, MODULE));
writeFileSync(
  join(root, "dist", "browser", "wasm-inline.js"),
  `// Written by scripts/build-web.mjs from wasm-web/${MODULE} (${bytes.length} bytes):\n` +
    "// the WebAssembly module as base64, which backend.js decodes and instantiates.\n" +
    `export default "${bytes.toString("base64")}";\n`,
);
