use crate::catalog::ScalarId;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use once_cell::sync::Lazy;
use regex::Regex;

static EMAIL_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").expect("email pattern compiles")
});

const MAX_LENGTH: usize = 255;

pub struct Email;

impl Email {
    fn canonical(input: &str) -> String {
        input.trim().to_lowercase()
    }
}

impl Scalar for Email {
    fn id(&self) -> ScalarId {
        ScalarId::CONTACT_EMAIL
    }

    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        Ok(Self::canonical(input))
    }

    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        self.parse(registry, input).map(|_| ())
    }

    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        let normalized = Self::canonical(input);
        if normalized.is_empty() {
            return Err(ScalarError::new(
                ErrorKind::Empty,
                "email must not be empty",
            ));
        }
        if !EMAIL_PATTERN.is_match(&normalized) {
            return Err(ScalarError::new(
                ErrorKind::Pattern,
                format!("invalid email format: {input}"),
            ));
        }
        if normalized.chars().count() > MAX_LENGTH {
            return Err(ScalarError::new(
                ErrorKind::Length,
                format!("email exceeds {MAX_LENGTH} characters"),
            ));
        }
        Ok(normalized)
    }
}
