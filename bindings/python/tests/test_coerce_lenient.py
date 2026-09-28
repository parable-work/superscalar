"""Smoke test for the lenient ("flag, don't block") coercion binding.

`_native.coerce_lenient(scalar, json_value)` is the one generic coercion
entry point (the engine calls by canonical scalar name, not per-scalar). It takes a JSON
document string and returns a ``{"value": <decoded JSON | None>, "error":
{"kind", "message"} | None}`` dict: a clean coercion fills ``value`` and leaves
``error`` None; a failed coercion leaves ``value`` None and fills ``error``
instead of raising.
"""

import json

import pytest

from superscalar import _native


def test_money_string_is_trimmed_and_coerced():
    # Finance.Money is an Int scalar: a padded numeric string coerces to the
    # bare integer with no error captured.
    money = "Finance.Money"
    result = _native.coerce_lenient(money, json.dumps(" 12345 "))
    assert result["value"] == 12345
    assert result["error"] is None


def test_bad_date_captures_an_error_instead_of_raising():
    # Temporal.Date is a String scalar: an unparseable date yields a null value
    # plus a captured error (no exception).
    date = "Temporal.Date"
    result = _native.coerce_lenient(date, json.dumps("not-a-date"))
    assert result["value"] is None
    assert result["error"] is not None
    assert result["error"]["kind"]
    assert result["error"]["message"]


def test_fractional_temporal_integer_captures_parse_error():
    seconds = "Temporal.Seconds"

    for value in (1.5, "1.5"):
        result = _native.coerce_lenient(seconds, json.dumps(value))
        assert result["value"] is None
        assert result["error"]["kind"] == "parse"


def test_unknown_scalar_name_raises():
    # Names are exact and case-sensitive; an unknown one raises rather than
    # riding in `error`, because no coercion was attempted.
    with pytest.raises(ValueError, match='unknown scalar "finance.money"'):
        _native.coerce_lenient("finance.money", json.dumps("1"))
