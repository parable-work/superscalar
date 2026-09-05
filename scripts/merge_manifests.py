#!/usr/bin/env python3
"""Merge per-platform scalar-lib manifests into one release manifest.

Each matrix runner builds its slice of the artifact set from one pinned core
commit and writes its own manifest.json (gen_manifest.py stamps every artifact
with the core_sha that runner built from). This script concatenates those
per-platform artifact lists WITHOUT re-stamping the SHA, so each artifact keeps
the real provenance it was built with. The merged manifest's top-level core_sha
is taken from the first input; verify_pinned.py then refuses the set if any
artifact's preserved core_sha disagrees -- the "never publish a
mismatched set" gate. Regenerating instead (rm manifest.json; gen_manifest.py
DIST ONE_SHA) would stamp a single SHA over everything and defeat that gate.

Usage: merge_manifests.py <out_manifest> <platform_manifest> [platform_manifest ...]
"""
import json
import sys
from pathlib import Path


def main() -> int:
    if len(sys.argv) < 3:
        print(
            "usage: merge_manifests.py <out_manifest> <platform_manifest> [platform_manifest ...]",
            file=sys.stderr,
        )
        return 2

    out_path = Path(sys.argv[1])
    inputs = [Path(p) for p in sys.argv[2:]]

    top_sha = None
    schema = 1
    artifacts = []
    seen_files = set()
    for src in inputs:
        manifest = json.loads(src.read_text())
        if top_sha is None:
            top_sha = manifest.get("core_sha")
            schema = manifest.get("schema", 1)
        for art in manifest.get("artifacts", []):
            ref = art.get("file")
            # The header + wasm bundle is platform-independent and is built once
            # (on the linux runner), so it only appears in one input. Guard
            # against accidental duplicates from overlapping inputs.
            if ref in seen_files:
                continue
            seen_files.add(ref)
            artifacts.append(art)

    artifacts.sort(key=lambda a: a.get("file", ""))
    merged = {"schema": schema, "core_sha": top_sha, "artifacts": artifacts}
    out_path.write_text(json.dumps(merged, indent=2) + "\n")
    print(
        f"merged {len(inputs)} manifests -> {out_path} "
        f"({len(artifacts)} artifacts, core_sha {str(top_sha)[:12]})"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
