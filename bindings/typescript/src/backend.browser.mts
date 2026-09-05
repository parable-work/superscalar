// Browser backend: a static ESM import of the bundler-target WASM (a CommonJS
// require cannot load a WASM module that uses top-level await), so client
// bundles never reference the napi `.node` addon. fix-esm-extensions.mjs
// rewrites this path to ../../wasm-bundler in the emitted dist/esm/ file.
import {
  scalar_coerce_lenient,
  scalar_normalize,
  scalar_parse,
  scalar_validate,
} from "../wasm-bundler/superscalar_wasm.js";

// ScalarBackend is inlined (not imported from ./backend) so the browser ESM
// compile never pulls in backend.ts and clobbers its CJS-emitted backend.js.
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

const wasmBackendInstance: ScalarBackend = {
  parse: (id: number, value: string): string => scalar_parse(id, value),
  normalize: (id: number, value: string): string => scalar_normalize(id, value),
  validate: (id: number, value: string): void => {
    scalar_validate(id, value);
  },
  coerceLenient: (id: number, jsonIn: string): string => scalar_coerce_lenient(id, jsonIn),
};

export function napiBackend(): ScalarBackend {
  throw new Error("napiBackend is not available in the browser build of superscalar");
}

export function wasmBackend(): ScalarBackend {
  return wasmBackendInstance;
}

export function loadBackend(): ScalarBackend {
  return wasmBackendInstance;
}

export const backend: ScalarBackend = wasmBackendInstance;
