use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};

fn normalize_vector(input: &str) -> Result<String, ScalarError> {
    let parsed: Vec<f32> = serde_json::from_str(input).map_err(|e| {
        ScalarError::new(
            ErrorKind::Parse,
            format!("expected a JSON float32 array: {e}"),
        )
    })?;
    // serde_json casts a number beyond the float32 range to infinity, and
    // `to_string` writes infinity as `null`; refuse it rather than emit `[null]`.
    if let Some(index) = parsed.iter().position(|v| !v.is_finite()) {
        return Err(ScalarError::new(
            ErrorKind::Parse,
            format!("element {index} is outside the float32 range"),
        ));
    }
    serde_json::to_string(&parsed)
        .map_err(|e| ScalarError::new(ErrorKind::Parse, format!("failed to serialize vector: {e}")))
}

pub struct EmbeddingVector;

impl Scalar for EmbeddingVector {
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

#[cfg(test)]
mod tests {
    use super::EmbeddingVector;
    use crate::error::ErrorKind;
    use crate::registry::{Registry, Scalar};

    #[test]
    fn rejects_elements_beyond_float32_range() {
        let registry = Registry::builtin();
        for input in ["[1e40]", "[-1e40]", "[3.5e38]"] {
            let err = EmbeddingVector.normalize(registry, input).unwrap_err();
            assert_eq!(err.kind, ErrorKind::Parse, "{input}");
            assert_eq!(err.message, "element 0 is outside the float32 range");
            assert!(EmbeddingVector.parse(registry, input).is_err(), "{input}");
            assert!(
                EmbeddingVector.validate(registry, input).is_err(),
                "{input}"
            );
        }
        let err = EmbeddingVector.parse(registry, "[0.5,1e40]").unwrap_err();
        assert_eq!(err.message, "element 1 is outside the float32 range");
    }

    #[test]
    fn keeps_float32_max_and_canonical_output_reparses() {
        let registry = Registry::builtin();
        for (input, canonical) in [
            (
                "[3.4028235e38,-3.4028235e38]",
                "[3.4028235e+38,-3.4028235e+38]",
            ),
            ("[0.5, 1.25]", "[0.5,1.25]"),
        ] {
            let got = EmbeddingVector.normalize(registry, input).unwrap();
            assert_eq!(got, canonical);
            assert_eq!(EmbeddingVector.parse(registry, &got).unwrap(), canonical);
        }
    }
}
