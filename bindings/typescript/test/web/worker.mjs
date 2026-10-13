// A Worker's use of the package, which scripts/web_smoke.sh bundles with
// `bun build --conditions=workerd --external '*.wasm'` and runs with
// `workerd test` (test/workerd/config.capnp): the full corpus and the
// generated wrappers through the workerd build, the checks
// test/conformance.cjs runs on the Node backends.
import corpus from "../../../../conformance/core-scalars.v2.json";
import * as api from "superscalar";
import { backend } from "superscalar/backend";
import { scalar_parse } from "superscalar/wasm";
import { checkUnknownName, checkWrappers, runCorpus } from "../corpus-runner.cjs";

export default {
  async test() {
    let failures = 0;
    // workerd refuses to compile WebAssembly from bytes, which is why the
    // browser build, which does, cannot run here.
    try {
      new WebAssembly.Module(new Uint8Array([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]));
      failures++;
      console.error("workerd compiled WebAssembly from bytes; the browser build would run here");
    } catch {
      // Expected.
    }
    if (api.loadBackend() !== backend) {
      failures++;
      console.error("superscalar and superscalar/backend hold different backends");
    }
    if (scalar_parse("Contact.Email", "Foo@Bar.com") !== "foo@bar.com") {
      failures++;
      console.error("superscalar/wasm did not route through the core");
    }
    failures += checkUnknownName("workerd", backend);
    failures += runCorpus("workerd", backend, corpus);
    failures += checkWrappers(api);
    if (failures > 0) {
      throw new Error(`FAIL: ${failures} workerd build failures`);
    }
    console.log("ts workerd conformance: ok");
  },

  async fetch() {
    return new Response(api.parseContactEmail(" Ada.Lovelace@Example.COM ") ?? "rejected");
  },
};
