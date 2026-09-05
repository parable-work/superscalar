use crate::catalog::ScalarId;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};

fn normalize_json(input: &str) -> Result<String, ScalarError> {
    let parsed: serde_json::Value = serde_json::from_str(input)
        .map_err(|e| ScalarError::new(ErrorKind::Parse, format!("failed to parse JSON: {e}")))?;
    serde_json::to_string(&parsed)
        .map_err(|e| ScalarError::new(ErrorKind::Parse, format!("failed to serialize JSON: {e}")))
}

pub struct Json;

impl Scalar for Json {
    fn id(&self) -> ScalarId {
        ScalarId::GENERIC_JSON
    }

    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_json(input)
    }

    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_json(input)
    }

    fn validate(&self, _registry: &Registry, input: &str) -> Result<(), ScalarError> {
        normalize_json(input).map(|_| ())
    }
}
