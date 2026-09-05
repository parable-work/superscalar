//! Lenient ("flag, don't block") scalar coercion.
//!
//! The strict path (`Scalar::parse` / `normalize` / `validate`) rejects bad
//! input by returning `Err`. Lenient coercion instead captures the failure:
//! it returns a [`LenientCoerceResult`] carrying either the canonical value or
//! a [`ScalarError`], so a writer can quarantine a bad row (or null a single
//! cell and flag it) rather than aborting the whole batch.
//!
//! The primitive helpers ([`coerce_int`], [`coerce_float`], [`coerce_bool`])
//! are the trim-and-parse rules shared with the JSON-value coercion path; they
//! are public because downstream DataFusion UDFs need them directly.

use serde_json::{Number, Value};

use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};

/// Coerce a JSON value to an `i64`. With `strict`, only a JSON number is
/// accepted; otherwise a trimmed numeric string is parsed too (integer first,
/// then a float truncated toward zero).
pub fn coerce_int(value: &Value, strict: bool) -> Option<i64> {
    match value {
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Some(i)
            } else {
                n.as_f64().map(|f| f as i64)
            }
        }
        Value::String(s) if !strict => {
            let t = s.trim();
            if let Ok(i) = t.parse::<i64>() {
                return Some(i);
            }
            t.parse::<f64>().ok().map(|f| f as i64)
        }
        _ => None,
    }
}

/// Coerce a JSON value to an `f64`. With `strict`, only a JSON number is
/// accepted; otherwise a trimmed numeric string is parsed too.
pub fn coerce_float(value: &Value, strict: bool) -> Option<f64> {
    match value {
        Value::Number(n) => n.as_f64(),
        Value::String(s) if !strict => s.trim().parse::<f64>().ok(),
        _ => None,
    }
}

/// Coerce a JSON value to a `bool`. With `strict`, only a JSON bool is
/// accepted; otherwise a trimmed, lowercased `"true"`/`"false"` string is
/// accepted too.
///
/// No catalog scalar is backed by `Bool` today, so the scalar-level bool branch
/// in [`coerce_lenient`] is unreachable. This helper is exported anyway because
/// downstream DataFusion UDFs coerce booleans directly without going through a scalar id.
pub fn coerce_bool(value: &Value, strict: bool) -> Option<bool> {
    match value {
        Value::Bool(b) => Some(*b),
        Value::String(s) if !strict => match s.trim().to_lowercase().as_str() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        },
        _ => None,
    }
}

/// The outcome of a lenient coercion: the canonical value, an error, or both
/// absent (a null passthrough).
///
/// Serializes so a quarantine writer can persist the capture directly.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct LenientCoerceResult {
    pub value: Option<Value>,
    pub error: Option<ScalarError>,
}

impl LenientCoerceResult {
    /// A successful coercion to `value`.
    pub fn ok(value: Value) -> Self {
        Self {
            value: Some(value),
            error: None,
        }
    }

    /// A null passthrough: no value, no error.
    pub fn null() -> Self {
        Self {
            value: None,
            error: None,
        }
    }

    /// A failed coercion: no value, carrying `error`.
    pub fn failed(error: ScalarError) -> Self {
        Self {
            value: None,
            error: Some(error),
        }
    }

    /// Continue with `next` only on a clean success; pass any value/error
    /// through unchanged otherwise.
    fn and_then(self, next: impl FnOnce(Value) -> Self) -> Self {
        match (self.value, self.error) {
            (Some(value), None) => next(value),
            (value, error) => Self { value, error },
        }
    }
}

/// Coerce `value` toward the canonical form of the built-in scalar named
/// `scalar_name`, capturing failures instead of returning `Err`. Equivalent to
/// `Registry::builtin().coerce_lenient(value, scalar_name)`.
pub fn coerce_lenient(value: &Value, scalar_name: &str) -> LenientCoerceResult {
    Registry::builtin().coerce_lenient(value, scalar_name)
}

impl Registry {
    /// Coerce `value` toward the canonical form of the scalar named
    /// `scalar_name`, capturing failures instead of returning `Err`.
    ///
    /// - A null input is a passthrough: `{value: None, error: None}`.
    /// - An unknown scalar name fails with [`ErrorKind::Enum`].
    /// - Otherwise the scalar's `metadata_primitive` selects the coercion:
    ///   `Int` and `Float` parse the numeric value (range-checked by the
    ///   scalar's `validate`), while `String` and `Type` run the scalar's
    ///   `parse` to get the canonical string.
    pub fn coerce_lenient(&self, value: &Value, scalar_name: &str) -> LenientCoerceResult {
        if value.is_null() {
            return LenientCoerceResult::null();
        }

        let Some(def) = self.by_canonical(scalar_name) else {
            return LenientCoerceResult::failed(ScalarError::new(
                ErrorKind::Enum,
                format!("unknown scalar: {scalar_name}"),
            ));
        };
        let scalar = self
            .scalar(def.id)
            .expect("every assembled def has a scalar slot");

        match def.metadata_primitive {
            "Int" => coerce_lenient_int(self, scalar, value, scalar_name),
            "Float" => coerce_lenient_float(self, scalar, value, scalar_name),
            "String" | "Type" => coerce_lenient_string(self, scalar, value),
            primitive => LenientCoerceResult::failed(ScalarError::new(
                ErrorKind::Custom,
                format!("unsupported scalar primitive {primitive} for {scalar_name}"),
            )),
        }
    }
}

fn coerce_lenient_int(
    registry: &Registry,
    scalar: &dyn Scalar,
    value: &Value,
    scalar_name: &str,
) -> LenientCoerceResult {
    let canonical = match scalar.parse(registry, &value_to_string(value)) {
        Ok(canonical) => canonical,
        Err(error) => return LenientCoerceResult::failed(error),
    };
    match canonical.parse::<i64>() {
        Ok(value) => LenientCoerceResult::ok(Value::Number(Number::from(value))),
        Err(_) => LenientCoerceResult::failed(ScalarError::new(
            ErrorKind::Parse,
            format!("failed to coerce {canonical:?} to {scalar_name}"),
        )),
    }
}

fn coerce_lenient_float(
    registry: &Registry,
    scalar: &dyn Scalar,
    value: &Value,
    scalar_name: &str,
) -> LenientCoerceResult {
    let Some(n) = coerce_float(value, false).and_then(Number::from_f64) else {
        return LenientCoerceResult::failed(ScalarError::new(
            ErrorKind::Parse,
            format!("failed to coerce {value:?} to {scalar_name}"),
        ));
    };

    LenientCoerceResult::ok(Value::Number(n.clone())).and_then(|coerced| {
        match scalar.validate(registry, &n.to_string()) {
            Ok(()) => LenientCoerceResult::ok(coerced),
            Err(error) => LenientCoerceResult::failed(error),
        }
    })
}

fn coerce_lenient_string(
    registry: &Registry,
    scalar: &dyn Scalar,
    value: &Value,
) -> LenientCoerceResult {
    match scalar.parse(registry, &value_to_string(value)) {
        Ok(canonical) => LenientCoerceResult::ok(Value::String(canonical)),
        Err(error) => LenientCoerceResult::failed(error),
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(raw) => raw.clone(),
        Value::Number(raw) => raw.to_string(),
        Value::Bool(raw) => raw.to_string(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn coerces_money_to_integer_value() {
        let result = coerce_lenient(&json!(" 12345 "), "Finance.Money");

        assert_eq!(result.value, Some(json!(12345)));
        assert_eq!(result.error, None);
    }

    #[test]
    fn coerces_datetime_to_canonical_string() {
        let result = coerce_lenient(&json!("2026-01-15 12:00:00"), "Temporal.DateTime");

        assert_eq!(result.value, Some(json!("2026-01-15T12:00:00Z")));
        assert_eq!(result.error, None);
    }

    #[test]
    fn rejects_unix_ms_digit_string_datetime() {
        // Bare epoch counts are never guessed from digit count.
        // A column that carries them declares x-temporal-format and decodes
        // via temporal_format::normalize_epoch; undeclared counts fail loudly.
        let result = coerce_lenient(&json!("1786550463000"), "Temporal.DateTime");
        assert_eq!(result.value, None);
        assert!(result.error.is_some());
    }

    #[test]
    fn returns_null_and_error_on_lenient_failure() {
        let result = coerce_lenient(&json!("not-a-date"), "Temporal.Date");

        assert_eq!(result.value, None);
        assert!(result.error.is_some());
        assert_eq!(result.error.unwrap().kind, ErrorKind::Parse);
    }

    #[test]
    fn keeps_null_without_error() {
        let result = coerce_lenient(&Value::Null, "Contact.Email");

        assert_eq!(result, LenientCoerceResult::null());
        assert_eq!(result.value, None);
        assert_eq!(result.error, None);
    }

    #[test]
    fn round_trips_uuid_to_canonical() {
        // Identity.UUID is the catalog's uuid scalar (base62-or-hyphenated v4).
        let result = coerce_lenient(
            &json!("123e4567-e89b-12d3-a456-426614174000"),
            "Identity.UUID",
        );

        assert_eq!(result.error, None);
        let canonical = result.value.expect("uuid should coerce");
        assert!(canonical.is_string());
        // Re-coercing the canonical form is idempotent (no error).
        let again = coerce_lenient(&canonical, "Identity.UUID");
        assert_eq!(again.error, None);
        assert_eq!(again.value, Some(canonical));
    }

    #[test]
    fn fails_unknown_scalar_with_enum_kind() {
        let result = coerce_lenient(&json!("anything"), "Not.AScalar");

        assert_eq!(result.value, None);
        let error = result.error.expect("unknown scalar should fail");
        assert_eq!(error.kind, ErrorKind::Enum);
    }

    #[test]
    fn out_of_range_int_captures_validate_error() {
        // Finance.Money is non-negative; a negative coerces but fails validate.
        let result = coerce_lenient(&json!("-5"), "Finance.Money");

        assert_eq!(result.value, None);
        assert!(result.error.is_some());
    }

    #[test]
    fn fractional_temporal_integer_captures_parse_error() {
        for value in [json!(1.5), json!("1.5")] {
            let result = coerce_lenient(&value, "Temporal.Seconds");

            assert_eq!(result.value, None);
            assert_eq!(
                result.error.expect("fraction should fail").kind,
                ErrorKind::Parse
            );
        }
    }

    #[test]
    fn failed_result_serializes_with_snake_case_kind() {
        let result = coerce_lenient(&json!("not-a-date"), "Temporal.Date");
        let json = serde_json::to_value(&result).expect("result should serialize");

        assert_eq!(json["value"], Value::Null);
        assert_eq!(json["error"]["kind"], json!("parse"));
        assert!(json["error"]["message"].is_string());
    }

    /// A metadata_omit def declares no metadata primitive. Before the
    /// registry, coerce_lenient looked the name up in SCALAR_METADATA and
    /// panicked on the missing row (which the FFI guard surfaced as a
    /// panic-category error). Reading the def instead makes it an ordinary
    /// failed result. No built-in is metadata_omit, so the def comes from an
    /// inline extension.
    #[test]
    fn metadata_omitted_scalar_fails_without_panicking() {
        use crate::extension::Extension;
        use crate::registry::{PrimitiveKind, Scalar, ScalarDef, ScalarHooks, ScalarTag};
        use crate::ScalarId;

        static DEFS: [ScalarDef; 1] = [ScalarDef {
            id: ScalarId(ScalarId::EXTENSION_BLOCK),
            namespace: "Acme",
            canonical: "Acme.SecretRef",
            primitive: PrimitiveKind::String,
            sql_type: "TEXT",
            metadata_primitive: "",
            json_schema_type: "",
            tag: ScalarTag::PatternOnly,
            pattern: Some("^projects/[^/]+/secrets/[^/]+/versions/[^/]+$"),
            min_length: None,
            max_length: None,
            minimum: None,
            maximum: None,
            case_insensitive: false,
            reserved_words: &[],
            examples: &[],
            description: "",
            type_mappings: &[],
            file_upload: None,
            image_constraints: None,
            docstring: "",
            alias_of: None,
            schema_primitive_override: None,
            schema_omit: true,
            format: None,
            reserved_words_case_insensitive: false,
            reserved_words_match_partial: false,
            comparability_class: None,
            hooks: ScalarHooks::NONE,
            metadata_omit: true,
        }];
        struct Acme;
        impl Extension for Acme {
            fn name(&self) -> &'static str {
                "acme"
            }
            fn id_base(&self) -> u32 {
                ScalarId::EXTENSION_BLOCK
            }
            fn defs(&self) -> &'static [ScalarDef] {
                &DEFS
            }
            fn impls(&self) -> Vec<(ScalarId, Box<dyn Scalar>)> {
                Vec::new()
            }
        }
        let registry = Registry::assemble(&[&Acme]);
        let result =
            registry.coerce_lenient(&json!("projects/p/secrets/s/versions/1"), "Acme.SecretRef");

        assert_eq!(result.value, None);
        let error = result.error.expect("no coercion is defined for the scalar");
        assert_eq!(error.kind, ErrorKind::Custom);
        assert!(
            error.message.contains("Acme.SecretRef"),
            "{}",
            error.message
        );
    }

    #[test]
    fn coerce_int_primitive_trims_and_parses() {
        assert_eq!(coerce_int(&json!(" 42 "), false), Some(42));
        assert_eq!(coerce_int(&json!("42"), true), None);
        assert_eq!(coerce_int(&json!(7), true), Some(7));
    }

    #[test]
    fn coerce_bool_primitive_trims_and_parses() {
        assert_eq!(coerce_bool(&json!(" TRUE "), false), Some(true));
        assert_eq!(coerce_bool(&json!("false"), false), Some(false));
        assert_eq!(coerce_bool(&json!("maybe"), false), None);
        assert_eq!(coerce_bool(&json!(true), true), Some(true));
    }
}
