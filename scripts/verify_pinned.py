#!/usr/bin/env python3
"""Verify a scalar-lib release manifest is commit-pinned and intact.

Fails (exit 1) if any artifact's core_sha disagrees with the manifest's, or --
when the artifact files are present next to the manifest -- if any sha256 no
longer matches. This is the gate the release job runs before publishing: a set
whose native/WASM/header artifacts were not all built from one core commit is
refused.

Usage:
  verify_pinned.py <manifest.json> [--require-platforms p1,p2,...]
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


def main() -> int:
    args = sys.argv[1:]
    require = []
    if "--require-platforms" in args:
        i = args.index("--require-platforms")
        require = [p for p in args[i + 1].split(",") if p]
        del args[i : i + 2]
    if len(args) != 1:
        print("usage: verify_pinned.py <manifest.json> [--require-platforms a,b]", file=sys.stderr)
        return 2

    manifest_path = Path(args[0])
    manifest = json.loads(manifest_path.read_text())
    dist = manifest_path.parent
    top_sha = manifest.get("core_sha")
    errors = []

    if not top_sha:
        errors.append("manifest has no core_sha")

    seen_platforms = set()
    for art in manifest.get("artifacts", []):
        ref = art.get("file", "<?>")
        if art.get("core_sha") != top_sha:
            errors.append(f"{ref}: core_sha {art.get('core_sha')} != manifest {top_sha}")
        if art.get("kind") == "staticlib":
            seen_platforms.add(art.get("platform"))
        f = dist / art["file"]
        if f.is_file():
            actual = sha256(f)
            if actual != art.get("sha256"):
                errors.append(f"{ref}: sha256 {actual[:12]} != recorded {str(art.get('sha256'))[:12]}")

    for p in require:
        if p not in seen_platforms:
            errors.append(f"missing required staticlib platform: {p}")

    if errors:
        print("PINNED MANIFEST INVALID:", file=sys.stderr)
        for e in errors:
            print(f"  - {e}", file=sys.stderr)
        return 1

    print(f"pinned manifest OK: {len(manifest.get('artifacts', []))} artifacts, core_sha {str(top_sha)[:12]}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
