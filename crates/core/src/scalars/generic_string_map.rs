use std::collections::BTreeMap;

use crate::catalog::ScalarId;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};

// Non-JSON fails as Parse; valid JSON that is not a string-to-string object
// fails as Custom. The parity corpus pins both error kinds.
fn normalize_string_map(input: &str) -> Result<String, ScalarError> {
    let value: serde_json::Value = serde_json::from_str(input)
        .map_err(|e| ScalarError::new(ErrorKind::Parse, format!("expected JSON: {e}")))?;
    let object = value.as_object().ok_or_else(|| {
        ScalarError::new(
            ErrorKind::Custom,
            "expected a JSON object of string values".to_string(),
        )
    })?;
    let mut map: BTreeMap<&str, &str> = BTreeMap::new();
    for (key, entry) in object {
        let text = entry.as_str().ok_or_else(|| {
            ScalarError::new(
                ErrorKind::Custom,
                format!("value for key {key:?} is not a string"),
            )
        })?;
        map.insert(key, text);
    }
    serde_json::to_string(&map)
        .map_err(|e| ScalarError::new(ErrorKind::Parse, format!("failed to serialize map: {e}")))
}

pub struct GenericStringMap;

impl Scalar for GenericStringMap {
    fn id(&self) -> ScalarId {
        ScalarId::GENERIC_STRING_MAP
    }

    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_string_map(input)
    }

    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_string_map(input)
    }

    fn validate(&self, _registry: &Registry, input: &str) -> Result<(), ScalarError> {
        normalize_string_map(input).map(|_| ())
    }
}
