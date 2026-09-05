"""Color scalar (Design.Color). Canonical form is 8-digit ``#RRGGBBAA``."""

from __future__ import annotations

from typing import TYPE_CHECKING

from . import _native
from ._native_ids import SCALAR_ID_BY_CANONICAL

if TYPE_CHECKING:
    from . import ValidationError

_ID = SCALAR_ID_BY_CANONICAL["Design.Color"]


def normalize_design_color(input_value: str) -> str:
    """Normalize a CSS color string to ``#RRGGBBAA``. Raises on bad input."""
    return _native.normalize(_ID, input_value)


def parse_design_color(input_value: str) -> str:
    """Parse and normalize a CSS color string to ``#RRGGBBAA``. Raises on bad input."""
    return _native.parse(_ID, input_value)


def validate_design_color(input_value: str) -> list[ValidationError]:
    """Validate a Color value; empty list if valid, else one error."""
    from . import ValidationError

    try:
        _native.validate(_ID, input_value)
    except ValueError as exc:
        return [ValidationError(validator="custom", message=str(exc))]
    return []
