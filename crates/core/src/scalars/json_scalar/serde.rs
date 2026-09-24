//! Serde adapters for the existing `Generic.JSON` scalar and its containers.
//!
//! Public generated fields remain `serde_json::Value`, `Option`, `Vec`, and
//! `HashMap`. Capturing JSON through `RawValue` distinguishes actual JSON objects
//! from serde_json's internal number/raw-value map representation, including
//! when the input deserializer is `serde_json::from_value`.

use std::{collections::HashMap, hash::Hash};

use ::serde::{
    de::{DeserializeOwned, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::{value::RawValue, Map, Value};

/// Deserialize a generated Generic.JSON field without changing its Rust type.
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: JsonField,
{
    T::deserialize_json(deserializer)
}

/// Standard Rust containers supported by the generated field adapter.
pub trait JsonField: Sized {
    /// Deserialize the container, reading every `serde_json::Value` in it as a
    /// Generic.JSON value: object keys stay literal and, with `lossless-json`,
    /// numbers keep their digits.
    fn deserialize_json<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>;
}

struct Field<T>(T);

impl<'de, T: JsonField> Deserialize<'de> for Field<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize_json(deserializer).map(Self)
    }
}

impl JsonField for Value {
    fn deserialize_json<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(deserializer)?;
        parse_raw(&raw, 0).map_err(::serde::de::Error::custom)
    }
}

impl<T: JsonField> JsonField for Option<T> {
    fn deserialize_json<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Option::<Field<T>>::deserialize(deserializer).map(|value| value.map(|field| field.0))
    }
}

impl<T: JsonField> JsonField for Vec<T> {
    fn deserialize_json<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Vec::<Field<T>>::deserialize(deserializer)
            .map(|values| values.into_iter().map(|field| field.0).collect())
    }
}

impl<K: DeserializeOwned + Eq + Hash, T: JsonField> JsonField for HashMap<K, T> {
    fn deserialize_json<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        HashMap::<K, Field<T>>::deserialize(deserializer).map(|values| {
            values
                .into_iter()
                .map(|(key, value)| (key, value.0))
                .collect()
        })
    }
}

/// Parse the Generic.JSON value using serde_json's grammar, keeping numbers
/// exact when the `lossless-json` feature is on.
///
/// Cost: one pass over `text`, unless the text could spell one of serde_json's
/// private marker names as an object key (see [`may_spell_a_marker`]). Such
/// text takes the marker-safe path, where each container re-reads its own
/// slice, so the cost is proportional to the size times the nesting depth,
/// which serde_json's recursion limit caps. Application-specific size and
/// depth limits belong to the caller.
pub fn parse_value(text: &str) -> serde_json::Result<Value> {
    if !may_spell_a_marker(text) {
        return serde_json::from_str(text);
    }
    let raw = serde_json::from_str::<&RawValue>(text)?;
    parse_raw(raw, 0)
}

/// Whether an object key in `text` could decode to one of serde_json's private
/// marker names (`$serde_json::private::Number`, `...::RawValue`), which its
/// `Value` deserializer reads as a number or raw value instead of an object.
/// A key spells a marker either literally or through `\u` escapes: the other
/// JSON escapes decode to `"`, `\`, `/` or control characters, none of which
/// a marker contains. Text with neither can go through serde_json directly.
fn may_spell_a_marker(text: &str) -> bool {
    text.contains("serde_json::private") || text.contains("\\u")
}

fn parse_raw(raw: &RawValue, depth: usize) -> serde_json::Result<Value> {
    // Match serde_json's default recursion budget. This is independent of any
    // smaller depth or byte limit an application applies on top.
    if depth >= 127 && raw.get().starts_with(['{', '[']) {
        return Err(<serde_json::Error as ::serde::de::Error>::custom(
            "recursion limit exceeded",
        ));
    }
    match raw.get().as_bytes()[0] {
        b'{' => {
            let mut deserializer = serde_json::Deserializer::from_str(raw.get());
            deserializer.deserialize_map(ObjectVisitor { depth })
        }
        b'[' => serde_json::from_str::<Vec<Box<RawValue>>>(raw.get())?
            .into_iter()
            .map(|value| parse_raw(&value, depth + 1))
            .collect::<serde_json::Result<Vec<_>>>()
            .map(Value::Array),
        _ => serde_json::from_str(raw.get()),
    }
}

struct ObjectVisitor {
    depth: usize,
}

impl<'de> Visitor<'de> for ObjectVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a JSON object")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut entries: M) -> Result<Value, M::Error> {
        let mut fields = Map::new();
        while let Some((name, raw)) = entries.next_entry::<String, Box<RawValue>>()? {
            // Retain serde_json's existing duplicate-key/ordering behavior.
            fields.insert(
                name,
                parse_raw(&raw, self.depth + 1).map_err(::serde::de::Error::custom)?,
            );
        }
        Ok(Value::Object(fields))
    }
}

#[cfg(test)]
mod tests {
    use super::{may_spell_a_marker, parse_raw, parse_value};
    use serde_json::{value::RawValue, Value};

    fn single_pass(text: &str) -> serde_json::Result<Value> {
        serde_json::from_str(text)
    }

    fn marker_safe(text: &str) -> serde_json::Result<Value> {
        parse_raw(serde_json::from_str::<&RawValue>(text)?, 0)
    }

    fn assert_paths_agree(text: &str) {
        match (single_pass(text), marker_safe(text)) {
            (Ok(fast), Ok(safe)) => assert_eq!(fast, safe, "{text:?}"),
            (Err(_), Err(_)) => {}
            (fast, safe) => panic!("{text:?}: single pass {fast:?}, marker-safe {safe:?}"),
        }
    }

    /// Text that cannot spell a marker goes through serde_json in one pass. The
    /// marker-safe path must give the same value and the same accept set for it,
    /// or the fast path would change what Generic.JSON accepts.
    #[test]
    fn both_paths_agree_on_text_without_markers() {
        for text in [
            r#"{"z":1,"a":2}"#,
            r#"{"a":1,"a":2}"#,
            r#"[1,-0,1.5e300,12345678901234567890.123456789012345678901234567890]"#,
            r#"{"s":"line\nbreak \"q\" \\ \/","n":null,"t":true,"e":[],"o":{}}"#,
            "  [ 1 , 2 ]  ",
            "\"text\"",
            "42",
            "null",
            r#"{"a":[{"b":[{"c":{}}]}]}"#,
            "[1,]",
            "{\"a\"}",
            "[1] x",
            "",
            "01",
            "NaN",
        ] {
            assert!(!may_spell_a_marker(text), "{text:?} takes the single pass");
            assert_paths_agree(text);
        }
    }

    #[test]
    fn both_paths_share_the_depth_limit() {
        for levels in 120..=135 {
            for (open, close) in [("[", "]"), ("{\"k\":", "}")] {
                assert_paths_agree(&format!(
                    "{}null{}",
                    open.repeat(levels),
                    close.repeat(levels)
                ));
            }
        }
    }

    /// A key can spell a marker literally or through `\u` escapes; both take
    /// the marker-safe path and stay objects.
    #[test]
    fn a_key_that_could_spell_a_marker_stays_an_object() {
        for text in [
            r#"{"$serde_json::private::Number":"1"}"#,
            r#"{"$serde_json::private::Number":"1"}"#,
            r#"{"$serde_json::private::RawValue":"true"}"#,
        ] {
            assert!(may_spell_a_marker(text), "{text:?}");
            let value = parse_value(text).expect("valid JSON");
            assert!(value.is_object(), "{text:?} parsed as {value:?}");
        }
    }
}
