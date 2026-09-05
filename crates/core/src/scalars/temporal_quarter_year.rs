use crate::catalog::ScalarId;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use once_cell::sync::Lazy;
use regex::Regex;

static YEAR_FIRST: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d{4})-[Qq]([1-4])$").expect("re"));
static SLASH: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[Qq]([1-4])/(\d{4})$").expect("re"));
static DASH_FULL_YEAR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[Qq]([1-4])-(\d{4})$").expect("re"));
static DASH_TWO_YEAR: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[Qq]([1-4])-(\d{2})$").expect("re"));

fn normalize_temporal_quarter_year(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(caps) = YEAR_FIRST.captures(trimmed) {
        return Some(format!("{}-Q{}", &caps[1], &caps[2]));
    }
    if let Some(caps) = SLASH.captures(trimmed) {
        return Some(format!("{}-Q{}", &caps[2], &caps[1]));
    }
    if let Some(caps) = DASH_FULL_YEAR.captures(trimmed) {
        return Some(format!("{}-Q{}", &caps[2], &caps[1]));
    }
    if let Some(caps) = DASH_TWO_YEAR.captures(trimmed) {
        return Some(format!("20{}-Q{}", &caps[2], &caps[1]));
    }
    None
}

pub struct TemporalQuarterYear;

impl Scalar for TemporalQuarterYear {
    fn id(&self) -> ScalarId {
        ScalarId::TEMPORAL_QUARTER_YEAR
    }

    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_temporal_quarter_year(input).ok_or_else(|| {
            ScalarError::new(
                ErrorKind::Parse,
                format!("failed to parse QuarterYear: {input}"),
            )
        })
    }

    fn normalize(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.parse(registry, input)
    }

    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        self.parse(registry, input).map(|_| ())
    }
}
