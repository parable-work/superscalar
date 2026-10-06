"""Geo.Location: a point as the JSON object ``{"lat": <number>, "lon": <number>}``.

``GeoLocation`` is the decoded value's shape, the type the catalog's Python
mapping (``superscalar.GeoLocation``) names. ``parse_geo_location`` and its
siblings take and return the JSON text, like every generated wrapper, so
``json.loads(parse_geo_location(text))`` is a ``GeoLocation``.
"""

from typing import TypedDict


class GeoLocation(TypedDict):
    """Decimal degrees: ``lat`` from -90 to 90, ``lon`` from -180 to 180."""

    lat: float
    lon: float
