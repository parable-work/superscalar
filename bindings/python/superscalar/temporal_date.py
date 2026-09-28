"""Temporal.Date scalar.

Canonical form: ``YYYY-MM-DD``. ``parse`` and ``normalize`` return ``None`` (rather than
raising) when the input is not a recognizable date, and ``validate`` returns a
``list[ValidationError]``.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Optional

from . import _native

if TYPE_CHECKING:
    from . import ValidationError

_NAME = "Temporal.Date"


def normalize_temporal_date(input_value: str) -> Optional[str]:
    """Normalize a date input to ``YYYY-MM-DD``. Returns ``None`` on failure."""
    if not isinstance(input_value, str):
        return None
    try:
        return _native.normalize(_NAME, input_value)
    except ValueError:
        return None


def parse_temporal_date(input_value: str) -> Optional[str]:
    """Parse a date string and return its ``YYYY-MM-DD`` form, or ``None``."""
    if not isinstance(input_value, str):
        return None
    try:
        return _native.parse(_NAME, input_value)
    except ValueError:
        return None


def validate_temporal_date(input_value: str) -> list[ValidationError]:
    """Validate Date values; empty list if valid, else one error."""
    from . import ValidationError

    if normalize_temporal_date(input_value) is None:
        return [ValidationError(validator="custom", message="failed to parse Date")]
    return []
