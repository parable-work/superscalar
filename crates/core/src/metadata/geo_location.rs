use crate::catalog::ScalarId;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

fn decode(input: &str) -> Result<Location, ScalarError> {
    let trimmed = input.trim();
    if trimmed.starts_with('{') {
        return serde_json::from_str(trimmed).map_err(|e| {
            ScalarError::new(
                ErrorKind::Parse,
                format!("invalid Geo.Location object: {e}"),
            )
        });
    }
    // Legacy "lat,lon" string input.
    let parts: Vec<&str> = trimmed.split(',').collect();
    if parts.len() == 2 {
        let lat = parts[0]
            .trim()
            .parse::<f64>()
            .map_err(|_| ScalarError::new(ErrorKind::Parse, "invalid latitude"))?;
        let lon = parts[1]
            .trim()
            .parse::<f64>()
            .map_err(|_| ScalarError::new(ErrorKind::Parse, "invalid longitude"))?;
        return Ok(Location { lat, lon });
    }
    Err(ScalarError::new(
        ErrorKind::Parse,
        "Geo.Location must be {lat,lon} object or 'lat,lon' string",
    ))
}

fn reserialize(loc: &Location) -> Result<String, ScalarError> {
    serde_json::to_string(loc)
        .map_err(|e| ScalarError::new(ErrorKind::Parse, format!("failed to re-serialize: {e}")))
}

pub struct GeoLocation;

impl Scalar for GeoLocation {
    fn id(&self) -> ScalarId {
        ScalarId::GEO_LOCATION
    }

    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        reserialize(&decode(input)?)
    }

    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        reserialize(&decode(input)?)
    }

    fn validate(&self, _registry: &Registry, input: &str) -> Result<(), ScalarError> {
        decode(input).map(|_| ())
    }
}
