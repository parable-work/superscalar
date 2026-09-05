"""Email scalar (Contact.Email)."""

from __future__ import annotations

from typing import TYPE_CHECKING

from . import _native
from ._native_ids import SCALAR_ID_BY_CANONICAL

if TYPE_CHECKING:
    from . import ValidationError

_ID = SCALAR_ID_BY_CANONICAL["Contact.Email"]


def normalize_contact_email(input_value: str) -> str:
    """Normalize an email address to canonical form (lowercase, trimmed)."""
    return _native.normalize(_ID, input_value)


def parse_contact_email(input_value: str) -> str:
    """Parse and normalize an email address. Raises ``ValueError`` on bad input."""
    return _native.parse(_ID, input_value)


def validate_contact_email(input_value: str) -> list[ValidationError]:
    """Validate an email address; empty list if valid, else one error."""
    from . import ValidationError

    try:
        _native.validate(_ID, input_value)
    except ValueError as exc:
        return [ValidationError(validator="custom", message=str(exc))]
    return []
