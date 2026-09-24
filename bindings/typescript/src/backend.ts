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

// The wasm bundle lives beside this package, not beside whatever file this
// module was bundled into. A bundler that inlines this module carries the
// relative path along, so the package-relative path is tried first and the
// package name second: the name resolves through node_modules from wherever
// the bundle runs, which any consumer that installs or links superscalar can
// satisfy. The error of the relative attempt is the one reported.
function loadBundle(relativeEntry: string, packageEntry: string): unknown {
  try {
    return require(relativeEntry);
  } catch (relativeError) {
    try {
      return require(packageEntry);
    } catch {
      throw relativeError;
    }
  }
}

// dist/backend.js -> ../native holds the locally built addon in a checkout;
// a published install resolves @superscalar/<triple> instead.
export function napiBackend(): ScalarBackend {
  return loadNativeAddon<ScalarBackend>(require, join(__dirname, "..", "native"));
}

export function wasmBackend(): ScalarBackend {
  const wasm = loadBundle(
    "../wasm-node/superscalar_wasm.js",
    "superscalar/wasm-node/superscalar_wasm.js",
  ) as WasmExports;
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

// Tries each loader in order and returns the first backend that loads. The
// failure names every attempt, because "no addon" alone hides which host,
// bundle, or package layout was wrong.
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

let cached: ScalarBackend | null = null;

// The native addon when this host has one installed, else the wasm-node bundle
// the main package ships; the two cores pass the same conformance corpus. The
// wasm build is the fallback for a host without a prebuilt addon for its
// platform (a musl Linux, say) and for a bundle that resolves the package but
// not an addon.
export function loadBackend(): ScalarBackend {
  if (cached === null) {
    cached = firstAvailableBackend([napiBackend, wasmBackend]);
  }
  return cached;
}

export const backend: ScalarBackend = loadBackend();
