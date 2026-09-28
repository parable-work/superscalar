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
  parse(scalar: string, value: string): string;
  normalize(scalar: string, value: string): string;
  validate(scalar: string, value: string): void;
  // Lenient ("flag, don't block") coercion: a JSON document string in, the
  // serialized LenientCoerceResult JSON
  // ({"value":<json|null>,"error":<{kind,message}|null>}) out. Throws only on an
  // unknown scalar name or unparseable jsonIn; a captured coercion failure rides in the
  // result's error.
  coerceLenient(scalar: string, jsonIn: string): string;
}

const wasmBackendInstance: ScalarBackend = {
  parse: (scalar: string, value: string): string => scalar_parse(scalar, value),
  normalize: (scalar: string, value: string): string => scalar_normalize(scalar, value),
  validate: (scalar: string, value: string): void => {
    scalar_validate(scalar, value);
  },
  coerceLenient: (scalar: string, jsonIn: string): string => scalar_coerce_lenient(scalar, jsonIn),
};

export function napiBackend(): ScalarBackend {
  throw new Error("napiBackend is not available in the browser build of superscalar");
}

export function wasmBackend(): ScalarBackend {
  return wasmBackendInstance;
}

// Same contract as the Node entries: the first loader that succeeds wins,
// and a total failure names every attempt.
export function firstAvailableBackend(loaders: ReadonlyArray<() => ScalarBackend>): ScalarBackend {
  const failures: string[] = [];
  for (const load of loaders) {
    try {
      return load();
    } catch (error) {
      failures.push(error instanceof Error ? error.message : String(error));
    }
  }
  throw new Error(`superscalar: no scalar backend could be loaded: ${failures.join("; ")}`);
}

export function loadBackend(): ScalarBackend {
  return wasmBackendInstance;
}

export const backend: ScalarBackend = wasmBackendInstance;
