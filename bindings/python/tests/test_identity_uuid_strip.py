"""Regression: Identity.UUID wrapper strips surrounding whitespace (parity restore).

The pure-Python ``normalize_identity_uuid`` this binding replaced did
``input_value.strip()`` before parsing; the core rewrite dropped it, so a
whitespace-padded UUID raised ValueError. Some Python callers
pass UUIDs that are NOT pre-stripped. Old Go ParseUUID and old TS normalizeUUID
never stripped, so this is restored at the Python binding layer only (the core
and the Go/TS bindings are left unchanged).
"""

from superscalar.identity_uuid import (
    normalize_identity_uuid,
    parse_identity_uuid,
    validate_identity_uuid,
)

CANONICAL = "123e4567-e89b-12d3-a456-426614174000"
# base62 of CANONICAL, byte-identical to old Python (ingestion differential harness).
CANONICAL_BASE62 = "YQJpYwUwvbaLOwTUr4thA"


def test_normalize_output_is_byte_identical_to_main():
    assert normalize_identity_uuid(CANONICAL) == CANONICAL_BASE62


def test_normalize_strips_surrounding_whitespace():
    assert normalize_identity_uuid(f"  {CANONICAL}  ") == CANONICAL_BASE62


def test_parse_strips_surrounding_whitespace():
    assert parse_identity_uuid(f"\t{CANONICAL}\n") == CANONICAL_BASE62


def test_validate_accepts_padded_uuid():
    assert validate_identity_uuid(f"  {CANONICAL}  ") == []


def test_padded_base62_round_trips():
    assert normalize_identity_uuid(f"  {CANONICAL_BASE62}  ") == CANONICAL_BASE62
