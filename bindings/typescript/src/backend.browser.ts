// Browser backend: the wasm-bindgen `web` build with its module inlined as
// base64 (wasm-inline.js, which scripts/build-web.mjs writes from
// wasm-web/superscalar_wasm_bg.wasm), so any bundler takes it as plain
// JavaScript with no WebAssembly or asset support, and instantiated
// synchronously at import, so every export works as soon as the import
// resolves, as it does on the Node backend. Chrome allows synchronous
// compilation on the main thread for modules up to 8 MB (since 2023; this one
// is about 2.2 MB). A Content-Security-Policy must allow 'wasm-unsafe-eval'.
//
// The package's `browser` condition resolves `.`, `./scalars`, `./validation`,
// `./backend` and `./wasm` into dist/browser/, where scripts/build-web.mjs has
// renamed this file to backend.js and copied the wasm-bindgen glue beside it.
import * as wasm from "../wasm-web/superscalar_wasm.js";
import wasmBase64 from "./wasm-inline.js";
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

function decodeBase64(text: string): Uint8Array {
  const binary = atob(text);
  const bytes = new Uint8Array(binary.length);
  for (let index = 0; index < binary.length; index += 1) {
    bytes[index] = binary.charCodeAt(index);
  }
  return bytes;
}

try {
  wasm.initSync({ module: decodeBase64(wasmBase64) });
} catch (error) {
  throw new Error(
    "superscalar: cannot instantiate the WASM backend " +
      `(a Content-Security-Policy must allow 'wasm-unsafe-eval'): ${error instanceof Error ? error.message : String(error)}`,
  );
}

const instance: ScalarBackend = wrapWasm(wasm);

export function wasmBackend(): ScalarBackend {
  return instance;
}

export function loadBackend(): ScalarBackend {
  return instance;
}

export const backend: ScalarBackend = instance;
