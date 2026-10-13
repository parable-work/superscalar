// What the browser and workerd backends share: both wrap the wasm-bindgen
// `web` build of the core (wasm-web/), and differ only in how they hand it its
// module. Neither imports anything from Node.

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

export interface WasmExports {
  scalar_parse(scalar: string, value: string): string;
  scalar_normalize(scalar: string, value: string): string;
  scalar_validate(scalar: string, value: string): void;
  scalar_coerce_lenient(scalar: string, jsonIn: string): string;
}

export function wrapWasm(wasm: WasmExports): ScalarBackend {
  return {
    parse: (scalar: string, value: string): string => wasm.scalar_parse(scalar, value),
    normalize: (scalar: string, value: string): string => wasm.scalar_normalize(scalar, value),
    validate: (scalar: string, value: string): void => {
      wasm.scalar_validate(scalar, value);
    },
    coerceLenient: (scalar: string, jsonIn: string): string => wasm.scalar_coerce_lenient(scalar, jsonIn),
  };
}

export function napiBackend(): ScalarBackend {
  throw new Error("superscalar: the native addon loads only in Node; this build runs the WASM backend");
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
