#!/usr/bin/env python3
"""Build one package in release mode and print the path of one of its artifacts.

    cargo_artifact.py <package> <kind>      kind: staticlib | cdylib

Cargo's own artifact record names the file, so this works wherever the target
directory is (CARGO_TARGET_DIR, --target-dir, a config.toml). Diagnostics stay
on stderr in their usual rendered form; only the path goes to stdout.
"""

import json
import subprocess
import sys

EXTENSIONS = {
    "staticlib": (".a", ".lib"),
    "cdylib": (".dylib", ".so", ".dll"),
}


def main() -> int:
    if len(sys.argv) != 3 or sys.argv[2] not in EXTENSIONS:
        print(__doc__, file=sys.stderr)
        return 2
    package, kind = sys.argv[1], sys.argv[2]
    proc = subprocess.run(
        [
            "cargo",
            "build",
            "--release",
            "-p",
            package,
            "--message-format=json-render-diagnostics",
        ],
        stdout=subprocess.PIPE,
        check=False,
        text=True,
    )
    if proc.returncode != 0:
        return proc.returncode
    for line in proc.stdout.splitlines():
        try:
            message = json.loads(line)
        except ValueError:
            continue
        if message.get("reason") != "compiler-artifact":
            continue
        if message.get("target", {}).get("name", "").replace("-", "_") != package.replace("-", "_"):
            continue
        for path in message.get("filenames") or []:
            if path.endswith(EXTENSIONS[kind]):
                print(path)
                return 0
    print(f"cargo build reported no {kind} artifact for {package}", file=sys.stderr)
    return 1


if __name__ == "__main__":
    sys.exit(main())
