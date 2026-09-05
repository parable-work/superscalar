// Run conformance vectors through the napi addon or the wasm bundle of the Acme
// assembly.
//
//   node run_vectors.cjs --registry registry.json --backend napi --addon <file.node> <vectors.json>...
//   node run_vectors.cjs --registry registry.json --backend wasm --bundle <pkg/acme_scalars_wasm.js> <vectors.json>...
//
// registry.json is the assembled registry dump (ext/examples/dump.rs), standing
// in for the generated canonical-name-to-id table a real binding ships. Every
// vector key must resolve in it and every non-built-in scalar must have
// vectors. Vectors flagged `unresolved` are skipped, as the built-in runners
// skip them. This is a CLI test runner: its output is the report.
/* eslint-disable no-console */
const fs = require("node:fs");
const path = require("node:path");

function parseArgs(argv) {
  const args = { vectors: [] };
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (arg.startsWith("--")) {
      args[arg.slice(2)] = argv[++i];
    } else {
      args.vectors.push(arg);
    }
  }
  return args;
}

function loadBackend(args) {
  if (args.backend === "napi") {
    if (!args.addon) throw new Error("--addon is required for the napi backend");
    const addon = require(path.resolve(args.addon));
    for (const name of ["parse", "normalize", "validate", "coerceLenient"]) {
      if (typeof addon[name] !== "function") throw new Error(`addon lacks ${name}`);
    }
    return (id, value) => addon.parse(id, value);
  }
  if (args.backend === "wasm") {
    if (!args.bundle) throw new Error("--bundle is required for the wasm backend");
    const wasm = require(path.resolve(args.bundle));
    for (const name of ["scalar_parse", "scalar_normalize", "scalar_validate", "scalar_coerce_lenient"]) {
      if (typeof wasm[name] !== "function") throw new Error(`wasm bundle lacks ${name}`);
    }
    return (id, value) => wasm.scalar_parse(id, value);
  }
  throw new Error(`unknown backend ${args.backend}`);
}

function loadVectors(paths) {
  const scalars = new Map();
  for (const file of paths) {
    const data = JSON.parse(fs.readFileSync(file, "utf8"));
    for (const [canonical, cases] of Object.entries(data.scalars)) {
      if (scalars.has(canonical)) throw new Error(`${canonical} appears in more than one vector file`);
      scalars.set(canonical, cases);
    }
  }
  return scalars;
}

function main() {
  const args = parseArgs(process.argv.slice(2));
  if (!args.registry || !args.backend || args.vectors.length === 0) {
    console.error("usage: run_vectors.cjs --registry <dump.json> --backend napi|wasm (--addon <file>|--bundle <file>) <vectors.json>...");
    return 2;
  }
  const dump = JSON.parse(fs.readFileSync(args.registry, "utf8"));
  const ids = new Map(dump.scalars.map((s) => [s.canonical, s.id]));
  const owners = new Map(dump.scalars.map((s) => [s.canonical, s.extension]));
  const parse = loadBackend(args);
  const scalars = loadVectors(args.vectors);

  const missing = [...ids.keys()].filter((c) => !scalars.has(c)).sort();
  if (missing.length > 0) throw new Error(`assembled scalars without vectors: ${missing.join(", ")}`);
  const unknown = [...scalars.keys()].filter((c) => !ids.has(c)).sort();
  if (unknown.length > 0) throw new Error(`vectors for scalars the assembly does not know: ${unknown.join(", ")}`);

  let accepted = 0;
  let rejected = 0;
  let skipped = 0;
  let failures = 0;
  const exercised = new Set();
  for (const [canonical, cases] of [...scalars.entries()].sort()) {
    const id = ids.get(canonical);
    if (owners.get(canonical) !== "builtin") exercised.add(canonical);
    for (const c of cases.accepted || []) {
      if (c.unresolved) {
        skipped++;
        continue;
      }
      try {
        const got = parse(id, c.input);
        if (c.normalized !== undefined && got !== c.normalized) {
          failures++;
          console.error(`${canonical} parse(${JSON.stringify(c.input)}) = ${JSON.stringify(got)}, want ${JSON.stringify(c.normalized)}`);
        } else {
          accepted++;
        }
      } catch (e) {
        failures++;
        console.error(`${canonical} parse(${JSON.stringify(c.input)}) rejected: ${e.message}`);
      }
    }
    for (const c of cases.rejected || []) {
      if (c.unresolved) {
        skipped++;
        continue;
      }
      try {
        const got = parse(id, c.input);
        failures++;
        console.error(`${canonical} parse(${JSON.stringify(c.input)}) accepted as ${JSON.stringify(got)}`);
      } catch (e) {
        rejected++;
      }
    }
  }
  if (exercised.size === 0) throw new Error("no extension scalar was exercised; the vectors cover built-ins only");
  console.log(
    `${args.backend}: ${accepted} accepted, ${rejected} rejected, ${skipped} skipped (unresolved), ${failures} failures; extension scalars: ${[...exercised].sort().join(", ")}`,
  );
  return failures > 0 ? 1 : 0;
}

process.exit(main());
