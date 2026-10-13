// Append `.js` to extensionless relative imports in the Node ESM build
// (dist/esm); see esm-tree.mjs. The ESM napi backend is authored as
// backend.node.ts so the CJS pass can skip it; generated.js imports
// `./backend.js`, so the emitted file (and its declaration) moves into place
// before the extensions are fixed.
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { fixEsmTree } from "./esm-tree.mjs";

fixEsmTree(join(dirname(fileURLToPath(import.meta.url)), "..", "dist", "esm"), [
  ["backend.node.js", "backend.js"],
  ["backend.node.d.ts", "backend.d.ts"],
]);
