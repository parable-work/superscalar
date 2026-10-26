"""Decimal scalar (Generic.Decimal). The canonical form is plain-notation text.

The catalog maps the scalar to ``decimal.Decimal`` in Python, so every function
here takes the text or a ``decimal.Decimal``. A Decimal is written in plain
notation first (``format(value, "f")``; ``str`` would write ``1E-7``) and then
parsed by the core like text, so ``Decimal("1.50")`` gives ``"1.5"`` and
``Decimal("NaN")`` is refused as the text ``NaN`` is. A float is refused: it
may already have lost digits.
"""

from __future__ import annotations

import decimal as _decimal

from . import _native

_NAME = "Generic.Decimal"


def _text(input_value: str | _decimal.Decimal) -> str:
    if isinstance(input_value, str):
        return input_value
    if isinstance(input_value, _decimal.Decimal):
        return format(input_value, "f")
    raise ValueError("Generic.Decimal input must be a string or decimal.Decimal")


def parse_generic_decimal(input_value: str | _decimal.Decimal) -> str:
    """Parse to the canonical text, such as ``"-12.5"``. Raises ``ValueError`` on bad input."""
    return _native.parse(_NAME, _text(input_value))


def normalize_generic_decimal(input_value: str | _decimal.Decimal) -> str:
    """The canonical text, as ``parse_generic_decimal``. Raises ``ValueError`` on bad input."""
    return _native.normalize(_NAME, _text(input_value))


def validate_generic_decimal(input_value: str | _decimal.Decimal) -> None:
    """Raise ``ValueError`` unless the input is a decimal the scalar accepts."""
    _native.validate(_NAME, _text(input_value))


def parse_generic_decimal_to_decimal(input_value: str | _decimal.Decimal) -> _decimal.Decimal:
    """Parse into a ``decimal.Decimal`` holding the canonical value, exactly."""
    return _decimal.Decimal(parse_generic_decimal(input_value))
