"""Temporal.Duration scalar."""

from __future__ import annotations

from typing import TYPE_CHECKING

from . import _native

if TYPE_CHECKING:
    from . import ValidationError

_NAME = "Temporal.Duration"


def normalize_temporal_duration(input_value: str) -> str:
    """Normalize a Go-style duration string. Raises ``ValueError`` on bad input."""
    return _native.normalize(_NAME, input_value)


def parse_temporal_duration(input_value: str) -> str:
    """Parse a Go-style duration string. Raises ``ValueError`` on bad input."""
    return _native.parse(_NAME, input_value)


def validate_temporal_duration(input_value: str) -> list[ValidationError]:
    """Validate a Go-style duration string; empty list if valid, else one error."""
    from . import ValidationError

    try:
        _native.validate(_NAME, input_value)
    except ValueError as exc:
        return [ValidationError(validator="custom", message=str(exc))]
    return []
