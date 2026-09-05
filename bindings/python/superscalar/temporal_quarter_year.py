"""Temporal.QuarterYear scalar.

Canonical form: ``YYYY-Q#`` (lexically sortable). ``parse`` and ``normalize`` return
``None`` (rather than raising) for unrecognized inputs, and ``validate`` returns a
``list[ValidationError]``.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Optional

from . import _native
from ._native_ids import SCALAR_ID_BY_CANONICAL

if TYPE_CHECKING:
    from . import ValidationError

_ID = SCALAR_ID_BY_CANONICAL["Temporal.QuarterYear"]


def normalize_temporal_quarter_year(input_value: str) -> Optional[str]:
    """Normalize a quarter+year string to canonical ``YYYY-Q#``, or ``None``."""
    if not isinstance(input_value, str):
        return None
    try:
        return _native.normalize(_ID, input_value)
    except ValueError:
        return None


def parse_temporal_quarter_year(input_value: str) -> Optional[str]:
    """Parse a quarter-year string into canonical form, or ``None`` on failure."""
    if not isinstance(input_value, str):
        return None
    try:
        return _native.parse(_ID, input_value)
    except ValueError:
        return None


def validate_temporal_quarter_year(input_value: str) -> list[ValidationError]:
    """Validate QuarterYear values; empty list if valid, else one error."""
    from . import ValidationError

    if normalize_temporal_quarter_year(input_value) is None:
        return [ValidationError(validator="custom", message="failed to parse QuarterYear")]
    return []
