// Node backend selection: napi addon in Node, wasm in the browser (swapped via
// the `browser` package.json condition). Both throw on reject.
import { join } from "node:path";
import { loadNativeAddon } from "./native-addon";

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

// dist/backend.js -> ../native holds the locally built addon in a checkout;
// a published install resolves @superscalar/<triple> instead.
export function napiBackend(): ScalarBackend {
  return loadNativeAddon<ScalarBackend>(require, join(__dirname, "..", "native"));
}

export function wasmBackend(): ScalarBackend {
  const wasm = require("../wasm-node/superscalar_wasm.js") as WasmExports;
  return wrapWasm(wasm);
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

let cached: ScalarBackend | null = null;

// The native addon when this host has one installed, else the wasm-node bundle
// the main package ships; the two cores pass the same conformance corpus.
export function loadBackend(): ScalarBackend {
  if (cached === null) {
    try {
      cached = napiBackend();
    } catch {
      cached = wasmBackend();
    }
  }
  return cached;
}

export const backend: ScalarBackend = loadBackend();
