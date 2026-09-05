"""Temporal.DateTime scalar.

Canonical form: RFC3339. ``parse`` and ``normalize`` return ``None`` (rather than
raising) when the input is not a recognizable datetime, and ``validate`` returns a
``list[ValidationError]``.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Optional

from . import _native
from ._native_ids import SCALAR_ID_BY_CANONICAL

if TYPE_CHECKING:
    from . import ValidationError

_ID = SCALAR_ID_BY_CANONICAL["Temporal.DateTime"]


def normalize_temporal_date_time(input_value: str) -> Optional[str]:
    """Normalize a DateTime input to RFC3339. Returns ``None`` on failure."""
    if not isinstance(input_value, str):
        return None
    try:
        return _native.normalize(_ID, input_value)
    except ValueError:
        return None


def parse_temporal_date_time(input_value: str) -> Optional[str]:
    """Parse a DateTime string and return its RFC3339 form, or ``None``."""
    if not isinstance(input_value, str):
        return None
    try:
        return _native.parse(_ID, input_value)
    except ValueError:
        return None


def validate_temporal_date_time(input_value: str) -> list[ValidationError]:
    """Validate DateTime values; empty list if valid, else one error."""
    from . import ValidationError

    if normalize_temporal_date_time(input_value) is None:
        return [ValidationError(validator="custom", message="failed to parse DateTime")]
    return []
