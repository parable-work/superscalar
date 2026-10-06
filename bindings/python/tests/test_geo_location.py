"""Geo.Location's typed shape: ``superscalar.GeoLocation`` is the TypedDict the
catalog's Python mapping names, and every canonical value the core writes
decodes into exactly that shape.
"""

import json
import pathlib
import typing

import pytest

import superscalar as ps

_VECTORS = json.loads(
    (pathlib.Path(__file__).resolve().parents[3] / "conformance" / "core-scalars.v2.json").read_text()
)["scalars"]["Geo.Location"]


def test_geo_location_is_the_mapped_typed_dict():
    assert ps.SCALAR_METADATA["Geo.Location"]["is_sortable"] is False
    assert typing.get_type_hints(ps.GeoLocation) == {"lat": float, "lon": float}
    assert ps.GeoLocation.__required_keys__ == frozenset({"lat", "lon"})


@pytest.mark.parametrize("case", _VECTORS["accepted"], ids=lambda case: case["input"])
def test_canonical_value_decodes_to_the_typed_shape(case):
    decoded = json.loads(ps.parse_geo_location(case["input"]))
    assert set(decoded) == set(typing.get_type_hints(ps.GeoLocation))
    assert all(isinstance(decoded[key], (int, float)) for key in decoded)


def test_typed_value_round_trips_through_the_core():
    location = ps.GeoLocation(lat=37.7749, lon=-122.4194)
    text = ps.parse_geo_location(json.dumps(location))
    assert text == '{"lat":37.7749,"lon":-122.4194}'
    assert json.loads(text) == location


def test_lat_lon_string_is_refused():
    with pytest.raises(ValueError, match="lat,lon"):
        ps.parse_geo_location("37.7749,-122.4194")
