"""Isolation conformance: the Python binding reproduces the full v2 corpus.

Every accepted input parses to its canonical normalized form; every rejected
input raises ValueError. Routing is by canonical -> id (covers every built-in scalar);
a spot check proves the generated per-scalar wrappers route through the core.

Vectors flagged ``"unresolved": true`` are skipped, exactly as the Rust parity
gate (``crates/core/tests/parity.rs``) and the TS runner do. These pin INTENDED
behavior that the core does not yet enforce (G2 over-acceptance gaps for
unconstrained ``String`` scalars: Asset.FilePath "", Embedding.Vector
"notavector"/"[1,2,", Generic.StringMap "not json"/"[1,2,3]", Text.Markdown
""). Honoring the flag keeps the three runners in lockstep so the suite is
green until those scalars tighten.
"""

import json
import pathlib

import pytest

import superscalar as ps
from superscalar import _native, SCALAR_ID_BY_CANONICAL

CORPUS = json.loads(
    (pathlib.Path(__file__).resolve().parents[3] / "conformance" / "core-scalars.v2.json").read_text()
)

# Count of vectors skipped because they are flagged unresolved; asserted below
# so the skip stays visible rather than silently shrinking the suite.
SKIPPED_UNRESOLVED = sum(
    1
    for data in CORPUS["scalars"].values()
    for bucket in ("accepted", "rejected")
    for case in data.get(bucket, [])
    if case.get("unresolved")
)


def _accepts():
    for canonical, data in CORPUS["scalars"].items():
        for case in data.get("accepted", []):
            if case.get("unresolved"):
                continue
            # A few identity-normalizing scalars (Permission, Scalar) omit
            # `normalized`; for those we only assert acceptance.
            yield pytest.param(
                canonical, case["input"], case.get("normalized"), id=f"{canonical}:{case['input']!r}"
            )


def _rejects():
    for canonical, data in CORPUS["scalars"].items():
        for case in data.get("rejected", []):
            # Skip over-acceptance gaps: the corpus pins the INTENDED
            # rejection but the scalar is still declared as unconstrained, so
            # the core legitimately accepts the input. The Rust parity gate
            # skips these too (crates/core/tests/parity.rs).
            if case.get("unresolved"):
                continue
            yield pytest.param(canonical, case["input"], id=f"{canonical}:{case['input']!r}")


@pytest.mark.parametrize("canonical,inp,normalized", list(_accepts()))
def test_accept(canonical, inp, normalized):
    sid = SCALAR_ID_BY_CANONICAL[canonical]
    got = _native.parse(sid, inp)
    if normalized is not None:
        assert got == normalized
    else:
        assert isinstance(got, str)


@pytest.mark.parametrize("canonical,inp", list(_rejects()))
def test_reject(canonical, inp):
    sid = SCALAR_ID_BY_CANONICAL[canonical]
    with pytest.raises(ValueError):
        _native.parse(sid, inp)


def test_generated_wrappers_route():
    from superscalar import _generated

    assert ps.parse_contact_email("Foo@Bar.com") == "foo@bar.com"
    assert ps.parse_identity_uuid("00000000-0000-0000-0000-000000000000") == "0"

    # Top-level validate_* exposes the RICH contract the six consumers and the
    # cutover suite require: empty list on valid, a non-empty list of
    # ValidationError on invalid (never raises).
    assert ps.validate_contact_email("Foo@Bar.com") == []
    errors = ps.validate_contact_email("nope")
    assert isinstance(errors, list)
    assert errors and all(isinstance(e, ps.ValidationError) for e in errors)

    # The thin raise-on-invalid behavior still lives on the generated per-scalar
    # wrapper, which routes straight through the native core.
    assert _generated.validate_contact_email("Foo@Bar.com") is None
    with pytest.raises(ValueError):
        _generated.validate_contact_email("nope")


def test_unresolved_skip_count_pinned():
    # The corpus enforces every vector; nothing is skipped. A newly added
    # unresolved vector must be acknowledged here, not silently absorbed.
    assert SKIPPED_UNRESOLVED == 0


def test_datetime_subsecond_preserved():
    # Enriched-core behavior: sub-seconds preserved in Go
    # RFC3339Nano minimal form (trailing zeros trimmed), not dropped (legacy)
    # nor zero-padded (JS toISOString).
    sid = SCALAR_ID_BY_CANONICAL["Temporal.DateTime"]
    assert _native.parse(sid, "2026-01-15T12:00:00.5Z") == "2026-01-15T12:00:00.5Z"


def test_metadata_parity():
    """The generated Python metadata reproduces the corpus metadata section."""
    from superscalar import SCALAR_METADATA

    # `.get`, not `[...]`: the Red for this test must be the count assertion
    # below, not a KeyError, so the failure names the problem.
    vectors = CORPUS.get("metadata", {})
    assert len(vectors) == len(SCALAR_METADATA)
    for canonical, want in vectors.items():
        # Named-field assertion BEFORE the whole-row equality, not after. The row
        # equality subsumes it, so ordered the other way this never fires and the
        # "better failure message" it exists for is unreachable. First failure
        # wins, so the specific check has to run first to be worth having.
        assert (
            SCALAR_METADATA[canonical]["comparability_class"]
            == want["comparability_class"]
        ), canonical
        assert SCALAR_METADATA[canonical] == want, canonical
    # Independent of the row comparison above.
    assert (
        sum(1 for row in vectors.values() if not row["is_sortable"])
        == CORPUS["meta"]["non_sortable_count"]
    )
    # Same independence argument, for comparability. Rows with no class are
    # skipped rather than grouped under a None key, so the derived map holds only
    # real classes.
    derived_classes: dict = {}
    for canonical, row in vectors.items():
        klass = row["comparability_class"]
        if klass is None:
            continue
        derived_classes.setdefault(klass, []).append(canonical)
    for members in derived_classes.values():
        members.sort()
    # `.get`, not `[...]`, for the same reason the row lookup above uses it: an
    # absent `meta.comparability_classes` must fail as the comparison below with
    # a message naming both sides, not as a KeyError that names only the key.
    # This is the guard the Go and TypeScript readers already have.
    assert derived_classes == CORPUS["meta"].get("comparability_classes")
    # set(scalars) - set(metadata) == set(metadata_excluded), not the weaker
    # disjointness check: disjointness alone passes if a scalar is missing from
    # `metadata` without being declared excluded, which is the drift that
    # matters.
    missing = set(CORPUS["scalars"]) - set(vectors)
    assert missing == set(CORPUS.get("metadata_excluded", []))
