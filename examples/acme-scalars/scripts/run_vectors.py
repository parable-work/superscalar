#!/usr/bin/env python3
"""Run conformance vectors through the C or the Python binding of the Acme assembly.

    run_vectors.py --registry registry.json --backend c --c-consumer <bin> <vectors.json>...
    run_vectors.py --registry registry.json --backend python --module-dir <dir> <vectors.json>...

`registry.json` is the assembled registry dump (ext/examples/dump.rs); it plays
the part of the generated canonical-name-to-id table a real binding ships. Every
vector key must resolve in it, and every non-built-in scalar in it must have
vectors, so a scalar added without vectors fails here. Vectors flagged
`unresolved` are skipped, as the built-in runners skip them.

The C backend spawns the consumer once per vector (`c_consumer <id> <input>`,
exit 0 with the value on stdout, exit 1 on reject). The Python backend imports
`_native` from `--module-dir`, the cdylib copied under an extension-module
name, and calls `_native.parse`.
"""

import argparse
import importlib
import json
import subprocess
import sys


def load_vectors(paths):
    scalars = {}
    for path in paths:
        with open(path, encoding="utf-8") as handle:
            data = json.load(handle)
        for canonical, cases in data["scalars"].items():
            if canonical in scalars:
                raise SystemExit(f"{canonical} appears in more than one vector file")
            scalars[canonical] = cases
    return scalars


def make_c_backend(consumer):
    def parse(scalar_id, value):
        proc = subprocess.run(
            [consumer, str(scalar_id), value],
            stdout=subprocess.PIPE,
            check=False,
        )
        out = proc.stdout.decode("utf-8", errors="replace")
        if proc.returncode == 0:
            return True, out
        if proc.returncode == 1:
            return False, out
        raise SystemExit(f"c_consumer exited {proc.returncode} for id {scalar_id}: {out}")

    return parse


def make_python_backend(module_dir):
    sys.path.insert(0, module_dir)
    native = importlib.import_module("_native")
    for name in ("parse", "normalize", "validate", "coerce_lenient"):
        if not hasattr(native, name):
            raise SystemExit(f"_native lacks {name}")

    def parse(scalar_id, value):
        try:
            return True, native.parse(scalar_id, value)
        except ValueError as err:
            return False, str(err)

    return parse


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--registry", required=True)
    parser.add_argument("--backend", choices=["c", "python"], required=True)
    parser.add_argument("--c-consumer")
    parser.add_argument("--module-dir")
    parser.add_argument("vectors", nargs="+")
    args = parser.parse_args()

    with open(args.registry, encoding="utf-8") as handle:
        dump = json.load(handle)
    ids = {s["canonical"]: s["id"] for s in dump["scalars"]}
    owners = {s["canonical"]: s["extension"] for s in dump["scalars"]}

    if args.backend == "c":
        if not args.c_consumer:
            parser.error("--c-consumer is required for the C backend")
        parse = make_c_backend(args.c_consumer)
    else:
        if not args.module_dir:
            parser.error("--module-dir is required for the Python backend")
        parse = make_python_backend(args.module_dir)

    scalars = load_vectors(args.vectors)
    missing = sorted(set(ids) - set(scalars))
    if missing:
        raise SystemExit(f"assembled scalars without vectors: {missing}")
    unknown = sorted(set(scalars) - set(ids))
    if unknown:
        raise SystemExit(f"vectors for scalars the assembly does not know: {unknown}")

    accepted = rejected = skipped = failures = 0
    exercised_extension_scalars = set()
    for canonical, cases in sorted(scalars.items()):
        scalar_id = ids[canonical]
        if owners[canonical] != "builtin":
            exercised_extension_scalars.add(canonical)
        for case in cases.get("accepted", []):
            if case.get("unresolved"):
                skipped += 1
                continue
            ok, got = parse(scalar_id, case["input"])
            if not ok:
                failures += 1
                print(f"{canonical} parse({case['input']!r}) rejected: {got}", file=sys.stderr)
            elif "normalized" in case and got != case["normalized"]:
                failures += 1
                print(
                    f"{canonical} parse({case['input']!r}) = {got!r}, want {case['normalized']!r}",
                    file=sys.stderr,
                )
            else:
                accepted += 1
        for case in cases.get("rejected", []):
            if case.get("unresolved"):
                skipped += 1
                continue
            ok, got = parse(scalar_id, case["input"])
            if ok:
                failures += 1
                print(f"{canonical} parse({case['input']!r}) accepted as {got!r}", file=sys.stderr)
            else:
                rejected += 1

    if not exercised_extension_scalars:
        raise SystemExit("no extension scalar was exercised; the vectors cover built-ins only")
    print(
        f"{args.backend}: {accepted} accepted, {rejected} rejected, {skipped} skipped (unresolved), "
        f"{failures} failures; extension scalars: {', '.join(sorted(exercised_extension_scalars))}"
    )
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
