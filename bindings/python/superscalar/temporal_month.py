"""Temporal.Month scalar.

Canonical form: two-digit numeric ("01"-"12"). ``parse`` and ``normalize`` return
``None`` (rather than raising) for unrecognized months, and ``validate`` returns a
``list[ValidationError]``.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Optional

from . import _native
from ._native_ids import SCALAR_ID_BY_CANONICAL

if TYPE_CHECKING:
    from . import ValidationError

_ID = SCALAR_ID_BY_CANONICAL["Temporal.Month"]


def normalize_temporal_month(input_value: str) -> Optional[str]:
    """Normalize a month string to canonical "01"-"12". Returns ``None`` on failure."""
    if not isinstance(input_value, str):
        return None
    try:
        return _native.normalize(_ID, input_value)
    except ValueError:
        return None


def parse_temporal_month(input_value: str) -> Optional[str]:
    """Parse a month string into the canonical "01"-"12" form, or ``None``."""
    if not isinstance(input_value, str):
        return None
    try:
        return _native.parse(_ID, input_value)
    except ValueError:
        return None


def validate_temporal_month(input_value: str) -> list[ValidationError]:
    """Validate Month values; empty list if valid, else one error."""
    from . import ValidationError

    if normalize_temporal_month(input_value) is None:
        return [ValidationError(validator="custom", message="failed to parse Month")]
    return []
