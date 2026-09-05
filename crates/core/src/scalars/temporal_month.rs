use crate::catalog::ScalarId;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use chrono::{Datelike, Month, NaiveDate};
use std::str::FromStr;

fn normalize_temporal_month(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }
    // Numeric path accepts 1-2 digit forms only (rejects "001", "-1"); anything else falls through to name parsing.
    if let Some(value) = (trimmed.len() <= 2)
        .then(|| trimmed.parse::<u32>().ok())
        .flatten()
    {
        if (1..=12).contains(&value) {
            return Some(format!("{value:02}"));
        }
        return None;
    }
    let lower = trimmed.to_lowercase();
    let month = Month::from_str(&lower).ok()?;
    let number = NaiveDate::from_ymd_opt(2000, month.number_from_month(), 1)?.month();
    Some(format!("{number:02}"))
}

pub struct TemporalMonth;

impl Scalar for TemporalMonth {
    fn id(&self) -> ScalarId {
        ScalarId::TEMPORAL_MONTH
    }

    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_temporal_month(input).ok_or_else(|| {
            ScalarError::new(ErrorKind::Parse, format!("failed to parse Month: {input}"))
        })
    }

    fn normalize(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.parse(registry, input)
    }

    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        self.parse(registry, input).map(|_| ())
    }
}
