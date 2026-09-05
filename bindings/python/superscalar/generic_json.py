"""JSON scalar (Generic.JSON).

Public functions accept ``Any`` (a JSON string or an already-decoded Python value);
non-string inputs are first serialized to a JSON string, then handed to the core for
canonical compaction.
"""

from __future__ import annotations

import json
from typing import TYPE_CHECKING, Any

from . import _native
from ._native_ids import SCALAR_ID_BY_CANONICAL

if TYPE_CHECKING:
    from . import ValidationError

_ID = SCALAR_ID_BY_CANONICAL["Generic.JSON"]


def _as_json_text(input_value: Any) -> str:
    if isinstance(input_value, str):
        return input_value
    return json.dumps(input_value, separators=(",", ":"), ensure_ascii=False)


def normalize_generic_json(input_value: Any) -> str:
    """Normalize any valid JSON token (string or decoded value) to compact text."""
    return _native.normalize(_ID, _as_json_text(input_value))


def parse_generic_json(input_value: Any) -> str:
    """Parse and normalize any valid JSON token to compact text. Raises on bad input."""
    return _native.parse(_ID, _as_json_text(input_value))


def validate_generic_json(input_value: Any) -> list[ValidationError]:
    """Validate that input represents a valid JSON token; empty list if valid."""
    from . import ValidationError

    try:
        _native.validate(_ID, _as_json_text(input_value))
    except (TypeError, ValueError) as exc:
        return [ValidationError(validator="custom", message=str(exc))]
    return []
