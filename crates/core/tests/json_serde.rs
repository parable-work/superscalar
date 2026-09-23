use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use superscalar::{scalar_for, scalars::json_scalar::serde as json_serde, Registry, ScalarId};

// Local adapter fixtures: a struct shaped like a generated consumer's, with
// Generic.JSON fields in every standard container.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Fields {
    #[serde(deserialize_with = "json_serde::deserialize")]
    value: Value,
    #[serde(
        default,
        deserialize_with = "json_serde::deserialize",
        skip_serializing_if = "Option::is_none"
    )]
    optional: Option<Value>,
    #[serde(deserialize_with = "json_serde::deserialize")]
    list: Vec<Value>,
    #[serde(default, deserialize_with = "json_serde::deserialize")]
    optional_list: Option<Vec<Value>>,
    #[serde(deserialize_with = "json_serde::deserialize")]
    map: HashMap<String, Value>,
    #[serde(default, deserialize_with = "json_serde::deserialize")]
    optional_map: Option<HashMap<String, Vec<Option<Value>>>>,
    count: u64,
    enabled: bool,
}

fn example() -> Value {
    json!({
        "marker_numbers": [{"$serde_json::private::Number":"1"}, {"$serde_json::private::Number":"not numeric","x":null}],
        "marker_raw": [{"$serde_json::private::RawValue":"true"}, {"$serde_json::private::RawValue":"not JSON","x":[]}],
        "precise": "12345678901234567890.123456789012345678901234567890".parse::<serde_json::Number>().unwrap(),
        "integer": "1234567890123456789012345678901234567890".parse::<serde_json::Number>().unwrap(),
        "null":null, "empty": ["",{},[]]
    })
}

#[test]
fn adapters_preserve_json_in_standard_containers_from_text_and_value() {
    let expected = example();
    let input = json!({"value":expected,"optional":expected,"list":[expected,null],"optional_list":[expected],"map":{"a":expected},"optional_map":{"a":[expected,null]},"count":42,"enabled":true});
    for fields in [
        serde_json::from_str::<Fields>(&input.to_string()).unwrap(),
        serde_json::from_value::<Fields>(input).unwrap(),
    ] {
        assert_eq!(fields.value, expected);
        assert_eq!(fields.optional, Some(expected.clone()));
        assert_eq!(fields.list, vec![expected.clone(), Value::Null]);
        assert_eq!(fields.optional_list, Some(vec![expected.clone()]));
        assert_eq!(fields.map["a"], expected);
        assert_eq!(
            fields.optional_map.as_ref().unwrap()["a"],
            vec![Some(expected.clone()), None]
        );
        assert_eq!(fields.count, 42);
        assert!(fields.enabled);
        let roundtrip: Fields =
            serde_json::from_str(&serde_json::to_string(&fields).unwrap()).unwrap();
        assert_eq!(roundtrip, fields);
    }
}

#[test]
fn absent_null_empty_and_wrong_container_shapes_keep_serde_semantics() {
    let input = json!({"value":null,"list":[],"map":{},"count":0,"enabled":false});
    let fields: Fields = serde_json::from_value(input.clone()).unwrap();
    assert_eq!(fields.value, Value::Null);
    assert!(fields.optional.is_none());
    assert!(fields.optional_list.is_none());
    assert!(fields.optional_map.is_none());
    assert!(fields.list.is_empty() && fields.map.is_empty());
    let mut explicit_nulls = input.clone();
    for key in ["optional", "optional_list", "optional_map"] {
        explicit_nulls[key] = Value::Null;
    }
    assert_eq!(
        serde_json::from_value::<Fields>(explicit_nulls).unwrap(),
        fields
    );
    for required in ["value", "list", "map", "count", "enabled"] {
        let mut missing = input.clone();
        missing.as_object_mut().unwrap().remove(required);
        assert!(
            serde_json::from_value::<Fields>(missing).is_err(),
            "missing {required}"
        );
    }
    for (key, wrong) in [
        ("list", Value::Null),
        ("list", json!({})),
        ("map", Value::Null),
        ("map", json!([])),
        ("optional_list", json!({})),
        ("optional_map", json!([])),
        ("count", json!("42")),
        ("enabled", json!(1)),
    ] {
        let mut invalid = input.clone();
        invalid[key] = wrong;
        assert!(
            serde_json::from_value::<Fields>(invalid).is_err(),
            "wrong shape for {key}"
        );
    }
}

#[test]
fn shared_json_parser_adds_no_size_limit_and_retains_serde_depth_and_duplicate_behavior() {
    let text = "x".repeat(1_048_577);
    assert_eq!(
        json_serde::parse_value(&serde_json::to_string(&text).unwrap()).unwrap(),
        Value::String(text)
    );
    for levels in [64, 96, 127, 128] {
        let text = format!("{}null{}", "[".repeat(levels), "]".repeat(levels));
        assert_eq!(
            json_serde::parse_value(&text).is_ok(),
            serde_json::from_str::<Value>(&text).is_ok(),
            "depth {levels}"
        );
    }
    assert_eq!(
        json_serde::parse_value(r#"{"a":1,"a":2}"#).unwrap(),
        json!({"a":2})
    );
}

#[test]
fn existing_generic_json_scalar_uses_the_same_lossless_parser() {
    let input = example();
    let text = input.to_string();
    let registry = Registry::builtin();
    let scalar = scalar_for(ScalarId::GENERIC_JSON);
    let parsed = scalar.parse(registry, &text).unwrap();
    assert_eq!(json_serde::parse_value(&parsed).unwrap(), input);
    scalar.validate(registry, &parsed).unwrap();
    assert_eq!(scalar.normalize(registry, &parsed).unwrap(), parsed);
}

#[test]
fn json_features_do_not_reinterpret_non_json_string_map_keys() {
    let registry = Registry::builtin();
    let scalar = scalar_for(ScalarId::GENERIC_STRING_MAP);
    for key in [
        "$serde_json::private::Number",
        "$serde_json::private::RawValue",
    ] {
        let input = json!({key: "literal string"}).to_string();
        assert_eq!(scalar.parse(registry, &input).unwrap(), input);
        scalar.validate(registry, &input).unwrap();
    }
}

/// The one built-in object scalar decodes into a typed struct, so the
/// private marker keys are ordinary unknown fields to it: they neither turn
/// the object into a number nor reach the canonical output.
#[test]
fn json_features_do_not_reinterpret_object_scalar_keys() {
    let registry = Registry::builtin();
    let scalar = scalar_for(ScalarId::GEO_LOCATION);
    for key in [
        "$serde_json::private::Number",
        "$serde_json::private::RawValue",
    ] {
        let input = json!({"lat": 1.5, "lon": -2.25, key: "literal string"}).to_string();
        assert_eq!(
            scalar.parse(registry, &input).unwrap(),
            r#"{"lat":1.5,"lon":-2.25}"#
        );
        scalar.validate(registry, &input).unwrap();
    }
}

#[test]
fn constructed_json_round_trips_cover_keys_escapes_numbers_and_containers() {
    let mut values = vec![
        Value::Null,
        json!(false),
        json!(true),
        json!(""),
        json!("\u{0}\n\"\\é中文"),
        json!([]),
        json!({}),
        example(),
    ];
    for _ in 0..4 {
        let previous = values.clone();
        for (index, value) in previous.into_iter().enumerate() {
            let key = [
                "$serde_json::private::Number",
                "$serde_json::private::RawValue",
                "normal",
                "\"é",
            ][index % 4];
            values.push(json!({key: value.clone()}));
            values.push(json!([value, index, null]));
        }
    }
    for value in values {
        assert_eq!(json_serde::parse_value(&value.to_string()).unwrap(), value);
    }
    for invalid in ["", "{", "[1,]", "{\"n\":NaN}", "01", "true false"] {
        assert!(json_serde::parse_value(invalid).is_err());
    }
}
