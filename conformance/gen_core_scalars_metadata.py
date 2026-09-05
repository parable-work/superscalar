"""Regenerate the `metadata` section of conformance/core-scalars.v2.json.

The section is a TRANSCRIPT of the generated `crates/core/src/scalar_metadata.rs`
table, not a re-derivation of the sortability rule. That distinction is the
whole point: the Rust parity reader compares the corpus against the table, so a
corpus built by re-running the predicate would compare the rule against itself
and assert nothing. This script therefore parses the emitted Rust literal and
copies what it finds.

Run after `cargo run -p superscalar-codegen`:

    python3 conformance/gen_core_scalars_metadata.py

Regenerating must be a no-op unless the scalar catalog changed, matching
gen_schema_version_vectors.py. The script rewrites exactly one key. It re-reads
the file afterwards and fails if any byte outside the `metadata` value moved, so
the hand-maintained `scalars` section, `meta` and `metadata_excluded` are out of
reach by construction. If this script ever needs to touch those, the scoping
broke and that is a bug, not a feature request.

Row order follows `SCALAR_METADATA` (ScalarId::ALL order minus the excluded
scalar) so a regeneration diff lines up row-for-row with the Rust table. No
reader depends on order.
"""

import json
import pathlib
import re
import sys

HERE = pathlib.Path(__file__).resolve().parent
TABLE = HERE.parent / "crates" / "core" / "src" / "scalar_metadata.rs"
CORPUS = HERE / "core-scalars.v2.json"

ROW = re.compile(
    r"ScalarMetadata \{.*?"
    r'canonical_name: "(?P<canonical>[^"]+)".*?'
    r"comparability_class: (?P<klass>None|Some\(\"[^\"]*\"\)).*?"
    r"is_sortable: (?P<sortable>true|false)",
    re.S,
)


def parse_table(source: str) -> "dict[str, dict]":
    rows = {}
    for m in ROW.finditer(source):
        klass = m.group("klass")
        if klass == "None":
            value = None
        else:
            value = klass[len('Some("') : -len('")')]
        rows[m.group("canonical")] = {
            "comparability_class": value,
            "is_sortable": m.group("sortable") == "true",
        }
    return rows


def render_metadata(rows: "dict[str, dict]", indent: str = "  ") -> str:
    """Render the metadata value with one scalar per line, matching the file's style."""
    lines = []
    for canonical, row in rows.items():
        klass = "null" if row["comparability_class"] is None else json.dumps(row["comparability_class"])
        sortable = "true" if row["is_sortable"] else "false"
        lines.append(
            f'{indent}{indent}{json.dumps(canonical)}: '
            f'{{ "comparability_class": {klass}, "is_sortable": {sortable} }}'
        )
    return "{\n" + ",\n".join(lines) + f"\n{indent}}}"


def main() -> int:
    if not TABLE.exists():
        print(f"missing {TABLE}; run cargo run -p superscalar-codegen first", file=sys.stderr)
        return 1
    rows = parse_table(TABLE.read_text())
    if not rows:
        print(f"parsed zero rows from {TABLE}; the emitted shape changed", file=sys.stderr)
        return 1

    original = CORPUS.read_text()
    before = json.loads(original)
    rendered = render_metadata(rows)

    if '"metadata"' in original:
        # Replace the existing value in place, byte-preserving everything else.
        start = original.index('"metadata"')
        open_brace = original.index("{", start)
        depth, i = 0, open_brace
        while True:
            if original[i] == "{":
                depth += 1
            elif original[i] == "}":
                depth -= 1
                if depth == 0:
                    break
            i += 1
        updated = original[:open_brace] + rendered + original[i + 1 :]
    else:
        # First run: insert as a sibling of `scalars`, before it, so the two
        # identically-keyed sections read together.
        anchor = '  "scalars": {'
        if anchor not in original:
            print('could not find the `"scalars"` key to insert beside', file=sys.stderr)
            return 1
        updated = original.replace(
            anchor, f'  "metadata": {rendered},\n{anchor}', 1
        )

    CORPUS.write_text(updated)

    # The scoping assertion. Everything outside `metadata` must be unchanged.
    # The parse itself is inside the guard: a splice that produced invalid JSON
    # would otherwise raise here and leave the corrupted file on disk, which is
    # the one failure mode that makes committing this script unsafe.
    try:
        after = json.loads(CORPUS.read_text())
    except json.JSONDecodeError as exc:
        CORPUS.write_text(original)
        print(f"refusing to write: the splice produced invalid JSON ({exc}); restored the original", file=sys.stderr)
        return 1
    for key in set(before) | set(after):
        if key == "metadata":
            continue
        if before.get(key) != after.get(key):
            CORPUS.write_text(original)
            print(f"refusing to write: key {key!r} changed; restored the original", file=sys.stderr)
            return 1

    changed = before.get("metadata") != after["metadata"]
    print(f"metadata: {len(rows)} rows ({'updated' if changed else 'no change'})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
