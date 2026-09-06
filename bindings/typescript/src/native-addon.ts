// Locates the napi addon for this host. Two layouts:
//  - source checkout: `napi build --platform ... native` leaves
//    native/superscalar-napi.<triple>.node next to the package; loaded first.
//  - published package: the addon ships in the platform package
//    @superscalar/<triple>, an optionalDependency of `superscalar`; npm installs
//    only the one whose os/cpu/libc match.
// The napi CLI's generated index.js can only name platform packages
// `<package>-<triple>`, so this loader replaces it. Node-only: both the CJS
// (backend.ts) and ESM (backend.node.ts) backends call it with their own
// `require` and their own path to native/.
import { existsSync } from "node:fs";
import { join } from "node:path";

export const NATIVE_PACKAGE_SCOPE = "@superscalar";
// package.json "napi.name": the basename of the .node file the CLI emits.
export const NATIVE_BINARY_NAME = "superscalar-napi";

// `${process.platform}-${process.arch}` -> napi platformArchABI. Exactly the
// triples in package.json "napi.triples"; linux is glibc only.
const TRIPLES: Readonly<Record<string, string>> = {
  "darwin-arm64": "darwin-arm64",
  "darwin-x64": "darwin-x64",
  "linux-x64": "linux-x64-gnu",
  "linux-arm64": "linux-arm64-gnu",
};

export const NATIVE_TRIPLES: readonly string[] = Object.values(TRIPLES);

interface ProcessReportHeader {
  header?: { glibcVersionRuntime?: string };
}

function isMusl(): boolean {
  if (process.platform !== "linux" || process.report === undefined) {
    return false;
  }
  const report = process.report.getReport() as ProcessReportHeader;
  if (report.header === undefined) {
    return false;
  }
  return report.header.glibcVersionRuntime === undefined;
}

// The platform package suffix for this host, or a throw naming the host.
export function nativeTriple(): string {
  const key = `${process.platform}-${process.arch}`;
  const triple = TRIPLES[key];
  if (triple === undefined) {
    throw new Error(
      `superscalar: no native addon is built for ${key}; supported: ${NATIVE_TRIPLES.join(", ")}. ` +
        "The WASM backend (superscalar/backend wasmBackend) runs everywhere.",
    );
  }
  if (isMusl()) {
    throw new Error(
      `superscalar: the ${triple} addon is linked against glibc; this Node runs on musl. ` +
        "The WASM backend (superscalar/backend wasmBackend) runs everywhere.",
    );
  }
  return triple;
}

export function nativePackageName(triple: string): string {
  return `${NATIVE_PACKAGE_SCOPE}/${triple}`;
}

// `req` is the caller's CommonJS require (createRequire under ESM); `nativeDir`
// is the caller's absolute path to the checkout's native/ directory.
export function loadNativeAddon<T>(req: (id: string) => unknown, nativeDir: string): T {
  const triple = nativeTriple();
  const local = join(nativeDir, `${NATIVE_BINARY_NAME}.${triple}.node`);
  // A local file that exists but does not load (stale, truncated, wrong ABI)
  // must not hide an installed platform package; its error rides along.
  let localDetail = "";
  if (existsSync(local)) {
    try {
      return req(local) as T;
    } catch (err) {
      localDetail = `; the checkout's ${local} failed to load: ${errorMessage(err)}`;
    }
  }
  const pkg = nativePackageName(triple);
  try {
    return req(pkg) as T;
  } catch (err) {
    throw new Error(
      `superscalar: cannot load the native addon package ${pkg} (${errorMessage(err)}). ` +
        "npm installs it as an optional dependency of superscalar; check that optional " +
        `dependencies were not skipped (npm install --no-optional / --omit=optional)${localDetail}.`,
    );
  }
}

function errorMessage(err: unknown): string {
  return err instanceof Error ? err.message : String(err);
}
