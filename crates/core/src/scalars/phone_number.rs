use crate::catalog::ScalarId;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use phonenumber::{parse, PhoneNumber};

fn err(msg: impl Into<String>) -> ScalarError {
    ScalarError::new(ErrorKind::Custom, msg)
}

fn parse_international(input: &str) -> Result<PhoneNumber, ScalarError> {
    parse(None, input).map_err(|e| {
        err(format!(
            "Invalid phone number format - must include country code: {e}"
        ))
    })
}

fn normalize_phone_number(input: &str) -> Result<String, ScalarError> {
    let num = parse_international(input)?;
    if !num.is_valid() {
        return Err(err("Invalid phone number - number failed validation"));
    }
    if num.code().value() == 0 {
        return Err(err("Invalid phone number - missing country code"));
    }
    Ok(num.format().mode(phonenumber::Mode::E164).to_string())
}

pub struct PhoneNumberScalar;

impl Scalar for PhoneNumberScalar {
    fn id(&self) -> ScalarId {
        ScalarId::CONTACT_PHONE_NUMBER
    }

    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_phone_number(input)
    }

    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_phone_number(input)
    }

    fn validate(&self, _registry: &Registry, input: &str) -> Result<(), ScalarError> {
        normalize_phone_number(input).map(|_| ())
    }
}
