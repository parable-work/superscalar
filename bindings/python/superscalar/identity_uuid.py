"""UUID scalar (Identity.UUID). Canonical form is compact base62.

Note: Python services import
``normalize_identity_uuid`` from this module path directly, so the module is
preserved (not folded into ``__init__``).
"""

from __future__ import annotations

import uuid as _uuidlib
from typing import TYPE_CHECKING

from . import _native
from ._native_ids import SCALAR_ID_BY_CANONICAL

if TYPE_CHECKING:
    from . import ValidationError

_ID = SCALAR_ID_BY_CANONICAL["Identity.UUID"]

# base62 alphabet mirrors the shared core (core/src/scalars/uuid_scalar.rs:9). The
# core owns encode/decode for cross-language parity; this Python copy exists only
# to turn the core's compact-base62 output into a stdlib uuid.UUID for DB binding
# (asyncpg), a Python-binding-only need Go/TS do not have. Guarded by the parity
# vectors in tests/test_identity_uuid_to_uuid.py.
_BASE62_ALPHABET = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
_BASE62_INDEX = {ch: idx for idx, ch in enumerate(_BASE62_ALPHABET)}
_MAX_UUID_VALUE = (1 << 128) - 1


def _decode_base62(value: str) -> int:
    result = 0
    for ch in value:
        idx = _BASE62_INDEX.get(ch)
        if idx is None:
            raise ValueError(f"invalid base62 character: {ch}")
        result = result * 62 + idx
    if result > _MAX_UUID_VALUE:
        raise ValueError("base62 value exceeds UUID max value")
    return result


def normalize_identity_uuid(input_value: str) -> str:
    """Normalize a UUID (canonical or base62) to compact base62. Raises on bad input."""
    # Strip surrounding whitespace to match the old pure-Python impl (main:identity_uuid.py:41).
    # Go/TS bindings never stripped, so this stays in the Python binding, not the shared core.
    return _native.normalize(_ID, input_value.strip())


def parse_identity_uuid(input_value: str) -> str:
    """Parse a UUID from canonical or base62 form to compact base62. Raises on bad input."""
    return _native.parse(_ID, input_value.strip())


def parse_identity_uuid_to_uuid(input_value: str | _uuidlib.UUID) -> _uuidlib.UUID:
    """Parse Identity.UUID input into a stdlib UUID for database binding."""
    if isinstance(input_value, _uuidlib.UUID):
        return input_value
    if not isinstance(input_value, str):
        raise ValueError("Identity.UUID input must be a string or uuid.UUID")

    # parse_identity_uuid (native core) validates + normalizes to compact base62.
    normalized = parse_identity_uuid(input_value)
    return _uuidlib.UUID(int=_decode_base62(normalized))


def validate_identity_uuid(input_value: str) -> list[ValidationError]:
    """Validate a UUID-like string; empty list if valid, else one error."""
    from . import ValidationError

    try:
        _native.validate(_ID, input_value.strip())
    except ValueError as exc:
        return [ValidationError(validator="custom", message=str(exc))]
    return []
