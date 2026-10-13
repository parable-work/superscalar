// Cloudflare Workers backend. workerd compiles no WebAssembly from bytes at
// run time (WebAssembly.compile and instantiate from a buffer are refused);
// it takes a module from the Worker's upload, where a `.wasm` import is a
// compiled WebAssembly.Module. wrangler's default module rules give exactly
// that, and so does any upload that lists superscalar_wasm_bg.wasm beside the
// bundle as a wasm module, such as `bun build --external '*.wasm'` with the
// file copied next to the output. Instantiating a compiled module is
// synchronous, so every export works as soon as the import resolves.
//
// The package's `workerd` condition resolves `.`, `./scalars`, `./validation`,
// `./backend` and `./wasm` into dist/workerd/, where scripts/build-web.mjs has
// renamed this file to backend.js and copied the glue and the module beside
// it, so the import below is `./superscalar_wasm_bg.wasm`.
import * as wasm from "../wasm-web/superscalar_wasm.js";
import wasmModule from "../wasm-web/superscalar_wasm_bg.wasm";
import { wrapWasm, type ScalarBackend } from "./web-backend";

export type { ScalarBackend, WasmExports } from "./web-backend";
export { firstAvailableBackend, napiBackend, wrapWasm } from "./web-backend";
// The raw wasm-bindgen exports: `superscalar/wasm` resolves here in this build.
export {
  scalar_coerce_lenient,
  scalar_normalize,
  scalar_parse,
  scalar_validate,
} from "../wasm-web/superscalar_wasm.js";

if (!(wasmModule instanceof WebAssembly.Module)) {
  throw new Error(
    "superscalar: superscalar_wasm_bg.wasm was not imported as a WebAssembly.Module; " +
      `got ${typeof wasmModule}. Bundle with wrangler, or keep the import external ` +
      "(bun build --external '*.wasm') and upload the file beside the bundle as a wasm module.",
  );
}
wasm.initSync({ module: wasmModule });

const instance: ScalarBackend = wrapWasm(wasm);

export function wasmBackend(): ScalarBackend {
  return instance;
}

export function loadBackend(): ScalarBackend {
  return instance;
}

export const backend: ScalarBackend = instance;
