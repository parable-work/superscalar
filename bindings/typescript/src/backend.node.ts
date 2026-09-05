// ESM napi backend: loads the CommonJS napi/wasm bundles via createRequire,
// since bare `require` is undefined under ESM. The browser swaps this module
// for backend.browser.mjs.
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);

export interface ScalarBackend {
  parse(id: number, value: string): string;
  normalize(id: number, value: string): string;
  validate(id: number, value: string): void;
  // Lenient ("flag, don't block") coercion: a JSON document string in, the
  // serialized LenientCoerceResult JSON
  // ({"value":<json|null>,"error":<{kind,message}|null>}) out. Throws only on an
  // unknown id or unparseable jsonIn; a captured coercion failure rides in the
  // result's error.
  coerceLenient(id: number, jsonIn: string): string;
}

export interface WasmExports {
  scalar_parse(id: number, value: string): string;
  scalar_normalize(id: number, value: string): string;
  scalar_validate(id: number, value: string): void;
  scalar_coerce_lenient(id: number, jsonIn: string): string;
}

export function wrapWasm(wasm: WasmExports): ScalarBackend {
  return {
    parse: (id: number, value: string): string => wasm.scalar_parse(id, value),
    normalize: (id: number, value: string): string => wasm.scalar_normalize(id, value),
    validate: (id: number, value: string): void => {
      wasm.scalar_validate(id, value);
    },
    coerceLenient: (id: number, jsonIn: string): string => wasm.scalar_coerce_lenient(id, jsonIn),
  };
}

export function napiBackend(): ScalarBackend {
  return require("../../native/index.js") as ScalarBackend;
}

export function wasmBackend(): ScalarBackend {
  const wasm = require("../../wasm-node/superscalar_wasm.js") as WasmExports;
  return wrapWasm(wasm);
}

let cached: ScalarBackend | null = null;

export function loadBackend(): ScalarBackend {
  if (cached === null) {
    cached = napiBackend();
  }
  return cached;
}

export const backend: ScalarBackend = loadBackend();
