"""Generic.Decimal through the package's public functions, as text and as decimal.Decimal.

The catalog maps the scalar to ``decimal.Decimal``, so the functions in
``superscalar.generic_decimal`` take either. Every corpus vector runs through
them as text; each accepted vector runs again as a Decimal built from its input,
which must give the same canonical text, and ``parse_generic_decimal_to_decimal``
must return the canonical value exactly.
"""

from __future__ import annotations

import decimal
import json
import pathlib

import pytest

import superscalar as ps

_VECTORS = json.loads(
    (pathlib.Path(__file__).resolve().parents[3] / "conformance" / "core-scalars.v2.json").read_text()
)["scalars"]["Generic.Decimal"]

_ACCEPTED = [(c["input"], c["normalized"]) for c in _VECTORS["accepted"] if not c.get("unresolved")]
_REJECTED = [c["input"] for c in _VECTORS["rejected"] if not c.get("unresolved")]


@pytest.mark.parametrize(("inp", "normalized"), _ACCEPTED)
def test_accepts_text_and_decimal(inp: str, normalized: str) -> None:
    assert ps.parse_generic_decimal(inp) == normalized
    assert ps.normalize_generic_decimal(inp) == normalized
    ps.validate_generic_decimal(inp)

    value = decimal.Decimal(inp.strip())
    assert ps.parse_generic_decimal(value) == normalized

    got = ps.parse_generic_decimal_to_decimal(inp)
    assert isinstance(got, decimal.Decimal)
    assert format(got, "f") == normalized
    assert got == value


@pytest.mark.parametrize("inp", _REJECTED)
def test_rejects(inp: str) -> None:
    with pytest.raises(ValueError):
        ps.parse_generic_decimal(inp)
    with pytest.raises(ValueError):
        ps.validate_generic_decimal(inp)
    with pytest.raises(ValueError):
        ps.parse_generic_decimal_to_decimal(inp)


@pytest.mark.parametrize(
    ("value", "normalized"),
    [
        (decimal.Decimal("1E-7"), "0.0000001"),
        (decimal.Decimal("1.2E+3"), "1200"),
        (decimal.Decimal("-0.00"), "0"),
        (decimal.Decimal("12.500"), "12.5"),
    ],
)
def test_decimal_is_written_in_plain_notation(value: decimal.Decimal, normalized: str) -> None:
    # str() would give "1E-7", an exponent form the scalar refuses.
    assert ps.parse_generic_decimal(value) == normalized


@pytest.mark.parametrize(
    "value",
    [decimal.Decimal("NaN"), decimal.Decimal("sNaN"), decimal.Decimal("Infinity"), decimal.Decimal("-Infinity")],
)
def test_decimal_specials_are_refused(value: decimal.Decimal) -> None:
    with pytest.raises(ValueError):
        ps.parse_generic_decimal(value)


@pytest.mark.parametrize("value", [1.5, 2, None, b"1.5"])
def test_other_types_are_refused(value: object) -> None:
    with pytest.raises(ValueError):
        ps.parse_generic_decimal(value)  # type: ignore[arg-type]
