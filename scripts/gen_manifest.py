#!/usr/bin/env python3
"""Generate a commit-pinned artifact manifest for a scalar-lib release.

Walks a dist directory (lib/<platform>/*.a, include/*.h, wasm/*.wasm), records
each artifact's sha256, size, and the core commit SHA it was built from, and
writes manifest.json. The release job builds each platform from one pinned core
commit and passes that SHA here; verify_pinned.py then refuses any set whose
artifacts disagree on the SHA (never publish a mismatched set).

Usage: gen_manifest.py <dist_dir> <core_sha>
"""
import hashlib
import json
import sys
from pathlib import Path


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def kind_for(rel: str) -> str:
    if rel.endswith(".a"):
        return "staticlib"
    if rel.endswith(".h"):
        return "header"
    if rel.endswith(".wasm"):
        return "wasm"
    if rel.endswith((".whl",)):
        return "wheel"
    if rel.endswith((".node",)):
        return "napi"
    return "other"


def platform_for(rel: str) -> str:
    parts = rel.split("/")
    if parts[0] == "lib" and len(parts) >= 2:
        return parts[1]
    return "any"


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: gen_manifest.py <dist_dir> <core_sha>", file=sys.stderr)
        return 2
    dist = Path(sys.argv[1])
    core_sha = sys.argv[2]

    artifacts = []
    for path in sorted(dist.rglob("*")):
        if not path.is_file() or path.name == "manifest.json":
            continue
        rel = path.relative_to(dist).as_posix()
        artifacts.append(
            {
                "kind": kind_for(rel),
                "platform": platform_for(rel),
                "file": rel,
                "sha256": sha256(path),
                "bytes": path.stat().st_size,
                "core_sha": core_sha,
            }
        )

    manifest = {"schema": 1, "core_sha": core_sha, "artifacts": artifacts}
    (dist / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"wrote {dist / 'manifest.json'} ({len(artifacts)} artifacts, core_sha {core_sha[:12]})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
