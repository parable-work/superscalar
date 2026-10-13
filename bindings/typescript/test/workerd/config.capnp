# The Worker scripts/web_smoke.sh runs with `workerd test`: the bundle of
# test/web/worker.mjs and the WebAssembly module it imports as
# ./superscalar_wasm_bg.wasm, listed as a wasm module the way a Worker upload
# lists it. Both are written to out/ by web_smoke.sh.
using Workerd = import "/workerd/workerd.capnp";

const config :Workerd.Config = (
  services = [(name = "main", worker = .smoke)],
);

const smoke :Workerd.Worker = (
  modules = [
    (name = "worker.js", esModule = embed "out/worker.js"),
    (name = "superscalar_wasm_bg.wasm", wasm = embed "out/superscalar_wasm_bg.wasm"),
  ],
  compatibilityDate = "2026-09-01",
);
