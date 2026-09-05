"""Public API contract: the historical ``parse_*``/``normalize_*``/``validate_*``
surface maps onto the core binding.

Consumers depend on these signatures: ``str``-and-raise or ``Optional[str]``
parse/normalize and ``validate_*`` returning ``list[ValidationError]``. This
suite replays the canonical v2 corpus (conformance/core-scalars.v2.json) through
that public surface. Full-corpus binding conformance lives in test_conformance.py.
"""

from __future__ import annotations

import json
import pathlib

import pytest

import superscalar as ps

_CORPUS = json.loads(
    (pathlib.Path(__file__).resolve().parents[3] / "conformance" / "core-scalars.v2.json").read_text()
)["scalars"]

# Scalars the package's public function surface exposes, mapped to their
# function stem and the signature contract of parse/normalize:
#   "raise"    -> parse/normalize return str, raise ValueError on bad input
#   "optional" -> parse/normalize return Optional[str], return None on bad input
_PUBLIC = {
    "Contact.Email": ("contact_email", "raise"),
    "Contact.PhoneNumber": ("contact_phone_number", "raise"),
    "Design.Color": ("design_color", "raise"),
    "Generic.JSON": ("generic_json", "raise"),
    "Identity.UUID": ("identity_uuid", "raise"),
    "Identity.UserID": ("identity_user_id", "raise"),
    "Temporal.Date": ("temporal_date", "optional"),
    "Temporal.DateTime": ("temporal_date_time", "optional"),
    "Temporal.Duration": ("temporal_duration", "raise"),
    "Temporal.Month": ("temporal_month", "optional"),
    "Temporal.QuarterYear": ("temporal_quarter_year", "optional"),
}


def _public_accepts():
    for canonical, (stem, contract) in _PUBLIC.items():
        for case in _CORPUS[canonical].get("accepted", []):
            if case.get("unresolved"):
                continue
            yield pytest.param(
                canonical, stem, contract, case["input"], case.get("normalized"),
                id=f"{stem}:{case['input']!r}",
            )


def _public_rejects():
    for canonical, (stem, contract) in _PUBLIC.items():
        for case in _CORPUS[canonical].get("rejected", []):
            if case.get("unresolved"):
                continue
            yield pytest.param(
                canonical, stem, contract, case["input"], id=f"{stem}:{case['input']!r}"
            )


@pytest.mark.parametrize("canonical,stem,contract,inp,normalized", list(_public_accepts()))
def test_public_api_accept(canonical, stem, contract, inp, normalized):
    parse_fn = getattr(ps, f"parse_{stem}")
    validate_fn = getattr(ps, f"validate_{stem}")

    got = parse_fn(inp)
    if normalized is not None:
        assert got == normalized
    else:
        assert got is not None

    assert validate_fn(inp) == []


@pytest.mark.parametrize("canonical,stem,contract,inp", list(_public_rejects()))
def test_public_api_reject(canonical, stem, contract, inp):
    parse_fn = getattr(ps, f"parse_{stem}")
    validate_fn = getattr(ps, f"validate_{stem}")

    if contract == "optional":
        assert parse_fn(inp) is None
    else:
        with pytest.raises(ValueError):
            parse_fn(inp)

    assert validate_fn(inp), f"expected validation errors for {inp!r}"


def test_validation_error_shape():
    """validate_* returns a list of ValidationError with the documented fields."""
    errs = ps.validate_contact_email("not-an-email")
    assert len(errs) == 1
    assert isinstance(errs[0], ps.ValidationError)
    assert errs[0].validator
    assert errs[0].message
