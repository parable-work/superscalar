use crate::catalog::ScalarId;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

const BASE62_ALPHABET: &[u8; 62] =
    b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn encode_base62(value: uuid::Uuid) -> String {
    let mut number = u128::from_be_bytes(*value.as_bytes());
    if number == 0 {
        return "0".to_string();
    }
    let mut encoded = Vec::new();
    while number > 0 {
        // `number % 62` is in 0..62, so narrowing to a usize index is exact; `try_from` stays panic-free without `as`.
        let remainder = usize::try_from(number % 62).unwrap_or(0);
        encoded.push(char::from(BASE62_ALPHABET[remainder]));
        number /= 62;
    }
    encoded.reverse();
    encoded.into_iter().collect()
}

fn decode_base62(value: &str) -> Result<uuid::Uuid, ScalarError> {
    let mut number: u128 = 0;
    for ch in value.bytes() {
        let position = BASE62_ALPHABET
            .iter()
            .position(|candidate| *candidate == ch)
            .ok_or_else(|| {
                ScalarError::new(
                    ErrorKind::Parse,
                    format!("invalid base62 character: {}", char::from(ch)),
                )
            })?;
        // `position` indexes the 62-byte alphabet; widening to u128 is always exact (no `From<usize> for u128`).
        let digit = u128::try_from(position).unwrap_or(0);
        number = number
            .checked_mul(62)
            .and_then(|acc| acc.checked_add(digit))
            .ok_or_else(|| {
                ScalarError::new(ErrorKind::Parse, "base62 value exceeds UUID max value")
            })?;
    }
    Ok(uuid::Uuid::from_bytes(number.to_be_bytes()))
}

fn parse_canonical(input: &str) -> Result<uuid::Uuid, ScalarError> {
    // A 36-char input is the hyphenated form (never base62); take it if it parses, else fall through.
    if let Some(parsed) = (input.len() == 36)
        .then(|| uuid::Uuid::parse_str(input).ok())
        .flatten()
    {
        return Ok(parsed);
    }
    if (1..=22).contains(&input.len()) {
        return decode_base62(input);
    }
    uuid::Uuid::parse_str(input)
        .map_err(|err| ScalarError::new(ErrorKind::Parse, format!("invalid UUID format: {err}")))
}

pub struct IdentityUuid;

impl Scalar for IdentityUuid {
    fn id(&self) -> ScalarId {
        ScalarId::IDENTITY_UUID
    }

    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        parse_canonical(input).map(encode_base62)
    }

    fn normalize(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.parse(registry, input)
    }

    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        self.parse(registry, input).map(|_| ())
    }
}

/// Typed Identity.UUID newtype; renders/serializes in canonical base62 form (all-zero -> "0"), accepting base62 or hyphenated input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Uuid(uuid::Uuid);

impl Uuid {
    pub fn new_v4() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn from_uuid(value: uuid::Uuid) -> Self {
        Self(value)
    }

    pub fn to_uuid(self) -> uuid::Uuid {
        self.0
    }

    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == uuid::Uuid::nil()
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}

impl From<Uuid> for uuid::Uuid {
    fn from(value: Uuid) -> Self {
        value.0
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&encode_base62(self.0))
    }
}

impl Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Uuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UuidVisitor;

        impl Visitor<'_> for UuidVisitor {
            type Value = Uuid;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a base62 UUID or canonical UUID string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                parse_canonical(value).map(Uuid).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(UuidVisitor)
    }
}

impl FromStr for Uuid {
    type Err = ScalarError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_canonical(s).map(Uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::Uuid;
    use std::str::FromStr;

    #[test]
    fn parses_canonical_uuid() {
        let value = Uuid::from_str("123e4567-e89b-12d3-a456-426614174000").expect("should parse");
        assert_eq!(
            value.to_uuid().to_string(),
            "123e4567-e89b-12d3-a456-426614174000"
        );
    }

    #[test]
    fn parses_base62_uuid() {
        let canonical =
            Uuid::from_str("123e4567-e89b-12d3-a456-426614174000").expect("should parse");
        let base62 = canonical.to_string();
        let reparsed = Uuid::from_str(&base62).expect("base62 should parse");
        assert_eq!(reparsed.to_uuid(), canonical.to_uuid());
    }

    #[test]
    fn nil_uuid_renders_as_zero() {
        let value = Uuid::from(uuid::Uuid::nil());
        assert!(value.is_zero());
        assert_eq!(value.to_string(), "0");
    }

    #[test]
    fn serializes_as_base62_json() {
        let value = Uuid::from_str("123e4567-e89b-12d3-a456-426614174000").expect("should parse");
        let serialized = serde_json::to_string(&value).expect("serialize should work");
        assert!(serialized.starts_with('"') && serialized.ends_with('"'));
        assert_ne!(
            serialized, "\"123e4567-e89b-12d3-a456-426614174000\"",
            "json output should be base62"
        );
    }

    #[test]
    fn deserializes_base62_json() {
        let value = Uuid::from_str("123e4567-e89b-12d3-a456-426614174000").expect("should parse");
        let base62_json = format!("\"{}\"", value);
        let decoded: Uuid = serde_json::from_str(&base62_json).expect("deserialize should work");
        assert_eq!(decoded.to_uuid(), value.to_uuid());
    }
}
