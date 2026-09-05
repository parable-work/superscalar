use std::fmt;

use serde::Serialize;

/// Why a scalar rejected an input. Stable string forms (`as_str`) are the
/// `validator` values in the parity vectors.
///
/// `snake_case` rename keeps the serialized variant strings identical to
/// `as_str()` (e.g. `Parse` -> `"parse"`), so a quarantine writer can emit a
/// `ScalarError` straight to JSON without re-deriving the kind string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    /// Input could not be parsed into the scalar's value space at all.
    Parse,
    /// Input failed the declared regex pattern.
    Pattern,
    /// Input violated a min/max length bound.
    Length,
    /// Numeric input fell outside the declared minimum/maximum.
    Range,
    /// Input is not a member of a declared value set (permission, scalar id).
    Enum,
    /// A scalar-specific rule rejected the input (e.g. phone number parsing).
    Custom,
    /// Input was empty or whitespace-only where that is disallowed.
    Empty,
}

impl ErrorKind {
    /// Stable lowercase identifier used as the parity `validator` value.
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorKind::Parse => "parse",
            ErrorKind::Pattern => "pattern",
            ErrorKind::Length => "length",
            ErrorKind::Range => "range",
            ErrorKind::Enum => "enum",
            ErrorKind::Custom => "custom",
            ErrorKind::Empty => "empty",
        }
    }
}

/// A scalar validation/normalization failure: a category plus a human message.
///
/// Serializes to `{"kind":"parse","message":"..."}` so a lenient-coercion
/// quarantine capture is just `serde_json::to_value(error)`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ScalarError {
    pub kind: ErrorKind,
    pub message: String,
}

impl ScalarError {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for ScalarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind.as_str(), self.message)
    }
}

impl std::error::Error for ScalarError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_kind_as_snake_case_and_message() {
        let error = ScalarError::new(ErrorKind::Parse, "bad value");
        let json = serde_json::to_value(&error).expect("ScalarError should serialize");
        assert_eq!(
            json,
            serde_json::json!({"kind": "parse", "message": "bad value"})
        );
    }

    #[test]
    fn error_kind_variants_serialize_to_as_str() {
        for kind in [
            ErrorKind::Parse,
            ErrorKind::Pattern,
            ErrorKind::Length,
            ErrorKind::Range,
            ErrorKind::Enum,
            ErrorKind::Custom,
            ErrorKind::Empty,
        ] {
            let json = serde_json::to_value(kind).expect("ErrorKind should serialize");
            assert_eq!(json, serde_json::Value::String(kind.as_str().to_string()));
        }
    }
}
