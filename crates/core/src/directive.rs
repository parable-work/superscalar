use crate::catalog::ScalarId;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{PrimitiveKind, Registry, Scalar, ScalarDef};
use regex::Regex;

/// A scalar whose entire behavior is its declared pattern/length/range,
/// plus any declared reserved words.
pub struct DirectiveScalar {
    id: ScalarId,
    primitive: PrimitiveKind,
    pattern: Option<Regex>,
    min_length: Option<usize>,
    max_length: Option<usize>,
    minimum: Option<f64>,
    maximum: Option<f64>,
    reserved_words: &'static [&'static str],
    case_insensitive: bool,
}

impl DirectiveScalar {
    /// Build the engine for `def`'s own id.
    pub fn from_def(def: &ScalarDef) -> Self {
        Self::from_def_as(def.id, def)
    }

    /// Build the engine reporting `id` but using `def`'s rules. Used for aliases
    /// (Identity.UserID reports its own id but borrows Identity.UUID's rules).
    ///
    /// Panics on a pattern that does not compile; `Registry::try_assemble`
    /// checks every declared pattern first, so an assembled registry never
    /// reaches this panic.
    pub fn from_def_as(id: ScalarId, def: &ScalarDef) -> Self {
        let pattern = def.pattern.map(|p| {
            Regex::new(p)
                .unwrap_or_else(|e| panic!("catalog pattern for {} invalid: {e}", def.canonical))
        });
        Self {
            id,
            primitive: def.primitive,
            pattern,
            min_length: def.min_length,
            max_length: def.max_length,
            minimum: def.minimum,
            maximum: def.maximum,
            reserved_words: def.reserved_words,
            case_insensitive: def.case_insensitive,
        }
    }

    fn check_range(&self, value: f64) -> Result<(), ScalarError> {
        if let Some(min) = self.minimum.filter(|&min| value < min) {
            return Err(ScalarError::new(
                ErrorKind::Range,
                format!("value {value} below minimum {min}"),
            ));
        }
        if let Some(max) = self.maximum.filter(|&max| value > max) {
            return Err(ScalarError::new(
                ErrorKind::Range,
                format!("value {value} above maximum {max}"),
            ));
        }
        Ok(())
    }

    fn validate_string(&self, input: &str) -> Result<(), ScalarError> {
        let len = input.chars().count();
        if let Some(min) = self.min_length.filter(|&min| len < min) {
            return Err(ScalarError::new(
                ErrorKind::Length,
                format!("length {len} below minimum {min}"),
            ));
        }
        if let Some(max) = self.max_length.filter(|&max| len > max) {
            return Err(ScalarError::new(
                ErrorKind::Length,
                format!("length {len} above maximum {max}"),
            ));
        }
        if self.pattern.as_ref().is_some_and(|p| !p.is_match(input)) {
            return Err(ScalarError::new(
                ErrorKind::Pattern,
                format!("input does not match pattern: {input}"),
            ));
        }
        let is_reserved = if self.case_insensitive {
            self.reserved_words
                .iter()
                .any(|word| word.eq_ignore_ascii_case(input))
        } else {
            self.reserved_words.contains(&input)
        };
        if is_reserved {
            return Err(ScalarError::new(
                ErrorKind::Enum,
                format!("reserved word: {input}"),
            ));
        }
        Ok(())
    }
}

impl Scalar for DirectiveScalar {
    fn id(&self) -> ScalarId {
        self.id
    }

    fn validate(&self, _registry: &Registry, input: &str) -> Result<(), ScalarError> {
        match self.primitive {
            PrimitiveKind::Int => {
                let value: i64 = input.trim().parse().map_err(|_| {
                    ScalarError::new(ErrorKind::Parse, format!("not an integer: {input}"))
                })?;
                // i64 -> f64 widening: catalog Int bounds are within +/-2^53, exact in range; no lossless From<f64>.
                self.check_range(value as f64)
            }
            PrimitiveKind::Float => {
                let value: f64 = input.trim().parse().map_err(|_| {
                    ScalarError::new(ErrorKind::Parse, format!("not a number: {input}"))
                })?;
                if !value.is_finite() {
                    return Err(ScalarError::new(
                        ErrorKind::Parse,
                        format!("not finite: {input}"),
                    ));
                }
                self.check_range(value)
            }
            _ => self.validate_string(input),
        }
    }

    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        match self.primitive {
            PrimitiveKind::Int => {
                let value: i64 = input.trim().parse().map_err(|_| {
                    ScalarError::new(ErrorKind::Parse, format!("not an integer: {input}"))
                })?;
                Ok(value.to_string())
            }
            _ => Ok(input.to_string()),
        }
    }

    fn parse(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.validate(registry, input)?;
        self.normalize(registry, input)
    }

    fn is_directive(&self) -> bool {
        true
    }
}
