"""Identity.UUID -> stdlib uuid.UUID coercion for DB binding.

``parse_identity_uuid_to_uuid`` is the Python-binding-only helper that turns the
core's compact-base62 canonical form back into a stdlib ``uuid.UUID`` for asyncpg
binding (a manifest seal in a downstream ingestion job). Vectors are byte-identical to the old
pure-Python test so the consolidated tree preserves the parity guarantee.
"""

import uuid

import pytest
from superscalar import parse_identity_uuid_to_uuid

CANONICAL = "123e4567-e89b-12d3-a456-426614174000"
CANONICAL_BASE62 = "YQJpYwUwvbaLOwTUr4thA"


@pytest.mark.parametrize(
    ("input_value", "expected"),
    [
        (CANONICAL, uuid.UUID(CANONICAL)),
        (CANONICAL_BASE62, uuid.UUID(CANONICAL)),
        (uuid.UUID(CANONICAL), uuid.UUID(CANONICAL)),
    ],
)
def test_parse_identity_uuid_to_uuid_accepted(input_value, expected) -> None:
    assert parse_identity_uuid_to_uuid(input_value) == expected


@pytest.mark.parametrize("input_value", ["", "invalid-uuid"])
def test_parse_identity_uuid_to_uuid_rejected(input_value: str) -> None:
    with pytest.raises(ValueError):
        parse_identity_uuid_to_uuid(input_value)
