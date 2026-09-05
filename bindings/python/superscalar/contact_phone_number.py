"""PhoneNumber scalar (Contact.PhoneNumber)."""

from __future__ import annotations

from typing import TYPE_CHECKING

from . import _native
from ._native_ids import SCALAR_ID_BY_CANONICAL

if TYPE_CHECKING:
    from . import ValidationError

_ID = SCALAR_ID_BY_CANONICAL["Contact.PhoneNumber"]


def normalize_contact_phone_number(input_value: str) -> str:
    """Normalize a phone number to E.164. Raises ``ValueError`` on bad input."""
    return _native.normalize(_ID, input_value)


def parse_contact_phone_number(input_value: str) -> str:
    """Parse and normalize a phone number to E.164. Raises on bad input."""
    return _native.parse(_ID, input_value)


def validate_contact_phone_number(input_value: str) -> list[ValidationError]:
    """Validate an international phone number; empty list if valid."""
    from . import ValidationError

    try:
        _native.validate(_ID, input_value)
    except ValueError as exc:
        return [ValidationError(validator="custom", message=str(exc))]
    return []
