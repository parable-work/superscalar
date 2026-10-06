//! `Geo.Location`: a point as the JSON object `{"lat": <number>, "lon": <number>}`.
//!
//! `lat` and `lon` are decimal degrees, `lat` in [-90, 90] and `lon` in
//! [-180, 180], each a JSON number. The object has exactly those two members;
//! a missing, duplicate or unknown key is refused, and so is the old
//! `"lat,lon"` string. The canonical text is `{"lat":<lat>,"lon":<lon>}` with
//! each number written as `JSON.stringify` and Go's `encoding/json` write it
//! (`90`, not `90.0`), and negative zero written as `0`.
//!
//! In SQL the scalar is a Postgres `POINT`, which is `(x, y)`: x is `lon` and
//! y is `lat`, so `{"lat":37.7749,"lon":-122.4194}` is `POINT(-122.4194, 37.7749)`.

use std::fmt;
use std::str::FromStr;

use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::value::RawValue;

use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};

/// A `Geo.Location` value. `Location::new`, `FromStr` and serde
/// deserialization apply the scalar's rules; `Display` writes the canonical
/// text. serde serialization writes the same object, though serde_json
/// spells an integral number as `90.0`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(try_from = "LocationMembers")]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

impl Location {
    /// A location from degrees, refused with `ErrorKind::Range` when `lat` is
    /// outside [-90, 90] or `lon` outside [-180, 180] (NaN and the infinities
    /// included). Negative zero becomes zero.
    pub fn new(lat: f64, lon: f64) -> Result<Location, ScalarError> {
        Ok(Location {
            lat: degrees("lat", lat, 90.0)?,
            lon: degrees("lon", lon, 180.0)?,
        })
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("{\"lat\":")?;
        write_number(f, self.lat)?;
        f.write_str(",\"lon\":")?;
        write_number(f, self.lon)?;
        f.write_str("}")
    }
}

impl FromStr for Location {
    type Err = ScalarError;

    fn from_str(input: &str) -> Result<Location, ScalarError> {
        decode(input)
    }
}

/// What serde reads before `Location::new` checks the ranges.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LocationMembers {
    lat: f64,
    lon: f64,
}

impl TryFrom<LocationMembers> for Location {
    type Error = ScalarError;

    fn try_from(members: LocationMembers) -> Result<Location, ScalarError> {
        Location::new(members.lat, members.lon)
    }
}

fn degrees(key: &str, value: f64, bound: f64) -> Result<f64, ScalarError> {
    if !(-bound..=bound).contains(&value) {
        return Err(ScalarError::new(
            ErrorKind::Range,
            format!("{key:?} must be from -{bound} to {bound}, got {value}"),
        ));
    }
    // -0.0 + 0.0 is 0.0, so both zeros have one canonical form.
    Ok(value + 0.0)
}

/// ECMAScript's Number::toString for a finite value under 1e21, which is what
/// `JSON.stringify` and Go's `encoding/json` write: the shortest digits that
/// read back to the same f64, in positional notation unless the magnitude is
/// below 1e-6. Rust's `Display` and `LowerExp` for f64 give those digits.
fn write_number(f: &mut fmt::Formatter<'_>, value: f64) -> fmt::Result {
    if value == 0.0 {
        f.write_str("0")
    } else if value.abs() < 1e-6 {
        write!(f, "{value:e}")
    } else {
        write!(f, "{value}")
    }
}

fn decode(input: &str) -> Result<Location, ScalarError> {
    let root: &RawValue = serde_json::from_str(input).map_err(|e| {
        if is_lat_lon_string(input) {
            ScalarError::new(
                ErrorKind::Parse,
                "the \"lat,lon\" string is not accepted; a Geo.Location is the JSON object \
                 {\"lat\": <number>, \"lon\": <number>}",
            )
        } else {
            ScalarError::new(ErrorKind::Parse, format!("expected JSON: {e}"))
        }
    })?;
    let text = root.get();
    if !text.starts_with('{') {
        return Err(ScalarError::new(
            ErrorKind::Custom,
            format!(
                "expected a JSON object {{\"lat\": <number>, \"lon\": <number>}}, got {}",
                json_type(text)
            ),
        ));
    }
    let members = serde_json::Deserializer::from_str(text)
        .deserialize_map(MembersVisitor)
        .map_err(|e| ScalarError::new(ErrorKind::Parse, format!("expected JSON: {e}")))?;

    let (mut lat, mut lon) = (None, None);
    for (key, value) in members {
        let slot = match key.as_str() {
            "lat" => &mut lat,
            "lon" => &mut lon,
            _ => {
                return Err(ScalarError::new(
                    ErrorKind::Custom,
                    format!("unknown key {key:?}: a Geo.Location has only \"lat\" and \"lon\""),
                ))
            }
        };
        if slot.replace(value).is_some() {
            return Err(ScalarError::new(
                ErrorKind::Custom,
                format!("duplicate key {key:?}"),
            ));
        }
    }
    let lat = lat.ok_or_else(|| missing("lat"))?;
    let lon = lon.ok_or_else(|| missing("lon"))?;
    Location::new(number("lat", lat)?, number("lon", lon)?)
}

fn missing(key: &str) -> ScalarError {
    ScalarError::new(ErrorKind::Custom, format!("missing key {key:?}"))
}

/// The member's value as an f64, refused unless it is a JSON number. Read
/// from the raw text, so an object that spells one of serde_json's private
/// number markers is still an object here.
fn number(key: &str, value: &RawValue) -> Result<f64, ScalarError> {
    let text = value.get();
    if !text.starts_with(|c: char| c == '-' || c.is_ascii_digit()) {
        return Err(ScalarError::new(
            ErrorKind::Custom,
            format!("{key:?} must be a JSON number, got {}", json_type(text)),
        ));
    }
    // JSON's number grammar is a subset of Rust's, so this always parses; a
    // magnitude beyond f64 reads as infinity and fails the range check.
    text.parse::<f64>()
        .map_err(|_| ScalarError::new(ErrorKind::Parse, format!("{key:?} is not a number: {text}")))
}

/// The JSON type of a valid JSON text, from its first character.
fn json_type(text: &str) -> &'static str {
    match text.as_bytes().first() {
        Some(b'{') => "an object",
        Some(b'[') => "an array",
        Some(b'"') => "a string",
        Some(b't' | b'f') => "a boolean",
        Some(b'n') => "null",
        _ => "a number",
    }
}

/// Whether `input` is two numbers separated by a comma, the string form this
/// scalar no longer accepts. Only used to explain the refusal.
fn is_lat_lon_string(input: &str) -> bool {
    let mut parts = input.split(',');
    match (parts.next(), parts.next(), parts.next()) {
        (Some(lat), Some(lon), None) => {
            lat.trim().parse::<f64>().is_ok() && lon.trim().parse::<f64>().is_ok()
        }
        _ => false,
    }
}

/// Every member of an object in order, duplicates included, each value as
/// its raw JSON text.
struct MembersVisitor;

impl<'de> Visitor<'de> for MembersVisitor {
    type Value = Vec<(String, &'de RawValue)>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON object")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut entries: M) -> Result<Self::Value, M::Error> {
        let mut members = Vec::new();
        while let Some(member) = entries.next_entry::<String, &'de RawValue>()? {
            members.push(member);
        }
        Ok(members)
    }
}

pub struct GeoLocation;

impl Scalar for GeoLocation {
    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        decode(input).map(|location| location.to_string())
    }

    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        decode(input).map(|location| location.to_string())
    }

    fn validate(&self, _registry: &Registry, input: &str) -> Result<(), ScalarError> {
        decode(input).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn refusal(input: &str) -> ScalarError {
        decode(input).expect_err(input)
    }

    #[test]
    fn canonical_text_matches_json_stringify() {
        for (input, canonical) in [
            (
                r#"{"lat":37.7749,"lon":-122.4194}"#,
                r#"{"lat":37.7749,"lon":-122.4194}"#,
            ),
            (
                r#" { "lon" : -180 , "lat" : 90 } "#,
                r#"{"lat":90,"lon":-180}"#,
            ),
            (r#"{"lat":-0,"lon":-0.0}"#, r#"{"lat":0,"lon":0}"#),
            (r#"{"lat":1.50,"lon":1E2}"#, r#"{"lat":1.5,"lon":100}"#),
            (
                r#"{"lat":0.000001,"lon":1.5e-7}"#,
                r#"{"lat":0.000001,"lon":1.5e-7}"#,
            ),
            (r#"{"lat":1e-400,"lon":0.1}"#, r#"{"lat":0,"lon":0.1}"#),
        ] {
            assert_eq!(decode(input).unwrap().to_string(), canonical, "{input}");
        }
    }

    #[test]
    fn each_refusal_has_its_kind() {
        for (input, kind, message) in [
            ("", ErrorKind::Parse, "expected JSON"),
            ("37.7749,-122.4194", ErrorKind::Parse, "\"lat,lon\" string"),
            (" 1 , 2 ", ErrorKind::Parse, "\"lat,lon\" string"),
            ("{\"lat\":1,", ErrorKind::Parse, "expected JSON"),
            ("null", ErrorKind::Custom, "got null"),
            ("[37.7749,-122.4194]", ErrorKind::Custom, "got an array"),
            ("37.7749", ErrorKind::Custom, "got a number"),
            ("\"37.7749,-122.4194\"", ErrorKind::Custom, "got a string"),
            ("{\"lat\":1}", ErrorKind::Custom, "missing key \"lon\""),
            ("{\"lon\":1}", ErrorKind::Custom, "missing key \"lat\""),
            (
                "{\"lat\":1,\"lon\":2,\"alt\":3}",
                ErrorKind::Custom,
                "unknown key \"alt\"",
            ),
            (
                "{\"lat\":1,\"lat\":2,\"lon\":3}",
                ErrorKind::Custom,
                "duplicate key \"lat\"",
            ),
            (
                "{\"lat\":\"1\",\"lon\":2}",
                ErrorKind::Custom,
                "\"lat\" must be a JSON number, got a string",
            ),
            (
                "{\"lat\":1,\"lon\":null}",
                ErrorKind::Custom,
                "\"lon\" must be a JSON number, got null",
            ),
            (
                "{\"lat\":90.0001,\"lon\":0}",
                ErrorKind::Range,
                "\"lat\" must be from -90 to 90",
            ),
            (
                "{\"lat\":0,\"lon\":-180.5}",
                ErrorKind::Range,
                "\"lon\" must be from -180 to 180",
            ),
            ("{\"lat\":1e400,\"lon\":0}", ErrorKind::Range, "got inf"),
        ] {
            let error = refusal(input);
            assert_eq!(error.kind, kind, "{input}: {error}");
            assert!(error.message.contains(message), "{input}: {error}");
        }
    }

    /// A nested object that spells serde_json's private number marker is an
    /// object to this scalar, never a number.
    #[test]
    fn a_number_marker_is_not_a_number() {
        let input = r#"{"lat":{"$serde_json::private::Number":"5"},"lon":1}"#;
        let error = refusal(input);
        assert_eq!(error.kind, ErrorKind::Custom);
        assert!(error.message.contains("got an object"), "{error}");
    }

    #[test]
    fn serde_round_trips_and_applies_the_rules() {
        let location = Location::new(37.7749, -122.4194).unwrap();
        let text = serde_json::to_string(&location).unwrap();
        assert_eq!(text, r#"{"lat":37.7749,"lon":-122.4194}"#);
        assert_eq!(serde_json::from_str::<Location>(&text).unwrap(), location);
        assert_eq!(location.to_string().parse::<Location>().unwrap(), location);
        for refused in [
            r#"{"lat":91,"lon":0}"#,
            r#"{"lat":1,"lon":2,"alt":3}"#,
            r#"{"lat":1}"#,
            r#"{"lat":"1","lon":2}"#,
            r#"{"lat":1,"lat":2,"lon":3}"#,
        ] {
            assert!(
                serde_json::from_str::<Location>(refused).is_err(),
                "{refused}"
            );
        }
    }

    #[test]
    fn new_refuses_non_finite_degrees() {
        for (lat, lon) in [
            (f64::NAN, 0.0),
            (0.0, f64::INFINITY),
            (f64::NEG_INFINITY, 0.0),
        ] {
            assert_eq!(Location::new(lat, lon).unwrap_err().kind, ErrorKind::Range);
        }
    }
}
