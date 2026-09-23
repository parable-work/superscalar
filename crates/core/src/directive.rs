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

    /// `case_insensitive` used to be read ONLY for reserved-word matching, so
    /// `normalize` was the identity for every non-Int primitive and
    /// normalizing `Identity.Slug` `"ACME"` returned `"ACME"` -- a value its own
    /// validator then rejected, because the pattern is lowercase-only. A
    /// normalize whose own validator refuses its output left every caller to
    /// lowercase first.
    ///
    /// WHICH SCALARS THIS MOVES. Only the directive-served ones: a
    /// `case_insensitive` scalar with a hand-written `Scalar` impl
    /// (`Contact.Email`, `Network.DnsLabel`) never reaches this function. A
    /// custom impl is also where a scalar whose case is SIGNIFICANT belongs
    /// (a camelCase identifier, say), since lowercasing would destroy the value
    /// rather than canonicalize it.
    ///
    /// AN ALREADY-VALID VALUE IS NEVER REWRITTEN. The fold applies only when the
    /// input fails this scalar's own validator and the lowercased form passes.
    /// That keeps `Temporal.Quarter` alone: its pattern is `^[Qq][1-4]$`, so
    /// `Q1` is already valid and stays `Q1`, the canonical spelling its
    /// conformance vector pins. `Identity.Slug` has a lowercase-only pattern,
    /// so `ACME` is invalid, `acme` is valid, and the fold fires.
    /// `Network.DomainName` admits either case and does not move.
    ///
    /// The rule is therefore not "case-insensitive means lowercase", which would
    /// silently restyle values whose canonical form nobody agreed to change. It
    /// is the narrower and checkable one: normalize must not emit something its
    /// own validator refuses.
    ///
    /// `parse` stays validate-then-normalize and therefore stays STRICT:
    /// parsing `Identity.Slug` `"ACME"` still fails. Only `normalize`
    /// canonicalizes, so a caller that means to accept mixed case normalizes
    /// first, then parses.
    fn normalize(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        match self.primitive {
            PrimitiveKind::Int => {
                let value: i64 = input.trim().parse().map_err(|_| {
                    ScalarError::new(ErrorKind::Parse, format!("not an integer: {input}"))
                })?;
                Ok(value.to_string())
            }
            // `validate`, not `validate_string`: this arm is a wildcard over every
            // non-Int primitive, and the rule being enforced is "normalize must
            // not emit what its OWN validator refuses", which is `validate`.
            _ if self.case_insensitive && self.validate(registry, input).is_err() => {
                let folded = input.to_lowercase();
                if self.validate(registry, &folded).is_ok() {
                    Ok(folded)
                } else {
                    Ok(input.to_string())
                }
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

#[cfg(test)]
mod case_fold_tests {
    use crate::catalog::ScalarId;
    use crate::registry::{scalar_for, Registry};

    /// The defect, stated as behaviour: `Identity.Slug` declares
    /// `case_insensitive` but its normalize did not fold, so it emitted a value
    /// its own validator rejected.
    #[test]
    fn identity_slug_normalize_output_passes_its_own_validator() {
        let registry = Registry::builtin();
        let slug = scalar_for(ScalarId::IDENTITY_SLUG);
        let normalized = slug
            .normalize(registry, "ACME")
            .expect("normalize does not refuse");
        assert_eq!(normalized, "acme");
        assert!(
            slug.validate(registry, &normalized).is_ok(),
            "normalize emitted {normalized:?}, which its own validator refuses"
        );
    }

    /// The general invariant, over every case-insensitive scalar the directive
    /// engine owns. This is the property; the test above is one instance.
    #[test]
    fn normalize_never_emits_what_validate_refuses() {
        let registry = Registry::builtin();
        for (id, input) in [
            (ScalarId::IDENTITY_SLUG, "ACME"),
            (ScalarId::IDENTITY_SLUG, "Q1-Sales"),
            (ScalarId::IDENTITY_SLUG, "HubSpot"),
            (ScalarId::NETWORK_DOMAIN_NAME, "Example.COM"),
            (ScalarId::TEMPORAL_QUARTER, "Q1"),
        ] {
            let scalar = scalar_for(id);
            let normalized = scalar
                .normalize(registry, input)
                .expect("normalize does not refuse");
            assert!(
                scalar.validate(registry, &normalized).is_ok(),
                "{id:?}: normalize({input:?}) = {normalized:?}, which validate refuses"
            );
        }
    }

    /// An already-valid value is returned untouched, which is what keeps the
    /// fold from restyling canonical forms nobody agreed to change.
    /// `Temporal.Quarter` accepts `^[Qq][1-4]$`, so `Q1` is valid as written and
    /// its conformance vector still reads `Q1`.
    #[test]
    fn an_already_valid_value_is_not_refolded() {
        let quarter = scalar_for(ScalarId::TEMPORAL_QUARTER);
        assert_eq!(
            quarter
                .normalize(Registry::builtin(), "Q1")
                .expect("normalize"),
            "Q1"
        );
    }

    /// `parse` stays strict. Only `normalize` canonicalizes, so a caller that
    /// means to accept mixed case normalizes first and then parses.
    #[test]
    fn parse_stays_strict_on_uppercase() {
        let registry = Registry::builtin();
        let slug = scalar_for(ScalarId::IDENTITY_SLUG);
        assert!(slug.parse(registry, "ACME").is_err());
        let normalized = slug.normalize(registry, "ACME").expect("normalize");
        assert!(slug.parse(registry, &normalized).is_ok());
    }
}
