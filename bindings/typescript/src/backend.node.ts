// ESM napi backend: loads the CommonJS napi/wasm bundles via createRequire,
// since bare `require` is undefined under ESM. The browser swaps this module
// for backend.browser.mjs.
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { loadNativeAddon } from "./native-addon";

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

// See backend.ts: the package name is the fallback for a bundled copy of this
// module, whose relative path no longer points at the package, and a double
// failure names both attempts.
function loadBundle(relativeEntry: string, packageEntry: string): unknown {
  try {
    return require(relativeEntry);
  } catch (relativeError) {
    try {
      return require(packageEntry);
    } catch (packageError) {
      throw new Error(
        `${errorMessage(relativeError)}; package fallback ${packageEntry}: ${errorMessage(packageError)}`,
      );
    }
  }
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

// dist/esm/backend.js -> ../../native holds the locally built addon in a
// checkout; a published install resolves @superscalar/<triple> instead.
export function napiBackend(): ScalarBackend {
  const here = dirname(fileURLToPath(import.meta.url));
  return loadNativeAddon<ScalarBackend>(require, join(here, "..", "..", "native"));
}

export function wasmBackend(): ScalarBackend {
  const wasm = loadBundle(
    "../../wasm-node/superscalar_wasm.js",
    "superscalar/wasm-node/superscalar_wasm.js",
  ) as WasmExports;
  return wrapWasm(wasm);
}

// See backend.ts: the first loader that succeeds wins, and a total failure
// names every attempt.
export function firstAvailableBackend(loaders: ReadonlyArray<() => ScalarBackend>): ScalarBackend {
  const failures: string[] = [];
  for (const load of loaders) {
    try {
      return load();
    } catch (error) {
      failures.push(errorMessage(error));
    }
  }
  throw new Error(`superscalar: no scalar backend could be loaded: ${failures.join("; ")}`);
}

let cached: ScalarBackend | null = null;

// The native addon when this host has one installed, else the wasm-node bundle
// the main package ships; the two cores pass the same conformance corpus.
export function loadBackend(): ScalarBackend {
  if (cached === null) {
    cached = firstAvailableBackend([napiBackend, wasmBackend]);
  }
  return cached;
}

export const backend: ScalarBackend = loadBackend();
