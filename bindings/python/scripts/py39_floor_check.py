"""Execute the generated module on the declared requires-python floor.

`pyproject.toml` declares ">=3.9" and consumers import `superscalar`, but
`py_smoke.sh` and the CI job both stand up 3.12, so a construct that only fails on
3.9 (a PEP 604 annotation evaluated at class-creation time, for instance) would go
green everywhere. Run this with a 3.9 interpreter; it evaluates the module body
with a stubbed `_native`, so it needs no compiled extension.

Limitation, since the stub is the reason this is cheap: `_native` is an empty
`types.ModuleType`, so this check would NOT catch a generated module that touched
`_native` at import time. That is fine -- the shape this pins is a module whose
body is data and function definitions only, which is exactly what it emits today.
If the generated module ever calls into `_native` at import, this stub has to grow
or the check has to build a wheel.
"""

import pathlib
import sys
import types

if sys.version_info[:2] != (3, 9):
    raise SystemExit(f"run with python 3.9, got {sys.version_info[0]}.{sys.version_info[1]}")

source = pathlib.Path(__file__).resolve().parents[1] / "superscalar" / "_generated.py"

package = types.ModuleType("superscalar")
package.__path__ = []
native = types.ModuleType("superscalar._native")
package._native = native
sys.modules["superscalar"] = package
sys.modules["superscalar._native"] = native

module = types.ModuleType("superscalar._generated")
module.__package__ = "superscalar"
exec(compile(source.read_text(), str(source), "exec"), module.__dict__)  # noqa: S102

# Derived, not hardcoded (F3, M6): this script's job is "does this module evaluate
# on 3.9", not "is the catalog complete". The catalog size is pinned in Rust
# (core/tests/scalar_metadata_complete.rs) and in all four parity readers. Every
# built-in carries a metadata row.
assert len(module.SCALAR_METADATA) == len(module.SCALAR_ID_BY_CANONICAL), (
    len(module.SCALAR_METADATA),
    len(module.SCALAR_ID_BY_CANONICAL),
)
# Two named spot-checks, on `is_sortable` only. Deliberately NOT
# `== {"comparability_class": None, "is_sortable": True}`: the all-None state is
# pinned in exactly one place (`every_scalar_starts_with_no_comparability_class` in
# core/tests/semantic_metadata.rs), and re-pinning it here would give the first class
# assignment a second site to edit in a file that has nothing to do with
# comparability classes.
assert "comparability_class" in module.SCALAR_METADATA["Contact.Email"]
assert module.SCALAR_METADATA["Contact.Email"]["is_sortable"] is True
assert module.SCALAR_METADATA["Embedding.Vector"]["is_sortable"] is False
print("py39 floor: ok")
