use crate::catalog::ScalarId;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};

fn normalize_vector(input: &str) -> Result<String, ScalarError> {
    let parsed: Vec<f32> = serde_json::from_str(input).map_err(|e| {
        ScalarError::new(
            ErrorKind::Parse,
            format!("expected a JSON float32 array: {e}"),
        )
    })?;
    serde_json::to_string(&parsed)
        .map_err(|e| ScalarError::new(ErrorKind::Parse, format!("failed to serialize vector: {e}")))
}

pub struct EmbeddingVector;

impl Scalar for EmbeddingVector {
    fn id(&self) -> ScalarId {
        ScalarId::EMBEDDING_VECTOR
    }

    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_vector(input)
    }

    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_vector(input)
    }

    fn validate(&self, _registry: &Registry, input: &str) -> Result<(), ScalarError> {
        normalize_vector(input).map(|_| ())
    }
}
