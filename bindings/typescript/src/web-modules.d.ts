// Modules the browser and workerd builds import that tsc does not compile.

// dist/browser/wasm-inline.js, which scripts/build-web.mjs writes: the
// WebAssembly module (wasm-web/superscalar_wasm_bg.wasm) as base64.
declare module "*/wasm-inline.js" {
  const wasmBase64: string;
  export default wasmBase64;
}

// A `.wasm` import in a Worker: workerd hands the importer a compiled module.
declare module "*.wasm" {
  const wasmModule: WebAssembly.Module;
  export default wasmModule;
}
