//! `registry dump` is a stable contract: two runs are byte-identical and the
//! keys come out in one fixed order, so a consumer (a schema toolchain, docs) can diff it.

use superscalar::Registry;
use superscalar_codegen::dump_to_string;

#[test]
fn two_dumps_are_byte_identical() {
    let first = dump_to_string(Registry::builtin());
    let second = dump_to_string(Registry::builtin());
    assert_eq!(first, second);
    assert!(first.ends_with("}\n"), "one trailing newline");
    assert!(!first.ends_with("\n\n"), "exactly one trailing newline");
}

#[test]
fn keys_are_in_a_fixed_order() {
    let text = dump_to_string(Registry::builtin());
    let value: serde_json::Value = serde_json::from_str(&text).expect("dump is JSON");
    let top = value.as_object().expect("object");
    let top_keys: Vec<&str> = top.keys().map(String::as_str).collect();
    assert_eq!(
        top_keys,
        [
            "dump_version",
            "superscalar_version",
            "extensions",
            "scalars",
            "legacy_aliases"
        ]
    );
    assert_eq!(top["dump_version"], 1);

    let scalars = top["scalars"].as_array().expect("scalars array");
    assert_eq!(scalars.len(), Registry::builtin().len());
    let ids: Vec<u64> = scalars
        .iter()
        .map(|scalar| scalar["id"].as_u64().expect("id"))
        .collect();
    assert!(ids.windows(2).all(|pair| pair[0] < pair[1]), "ids ascend");

    let first_keys: Vec<&str> = scalars[0]
        .as_object()
        .expect("scalar object")
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        first_keys,
        [
            "id",
            "canonical",
            "namespace",
            "extension",
            "primitive",
            "sql_type",
            "metadata_primitive",
            "json_schema_type",
            "tag",
            "pattern",
            "min_length",
            "max_length",
            "minimum",
            "maximum",
            "case_insensitive",
            "reserved_words",
            "reserved_words_case_insensitive",
            "reserved_words_match_partial",
            "examples",
            "description",
            "docstring",
            "type_mappings",
            "file_upload",
            "image_constraints",
            "alias_of",
            "schema_primitive_override",
            "schema_omit",
            "metadata_omit",
            "format",
            "comparability_class",
            "is_sortable",
            "is_directive",
            "hooks"
        ]
    );
    // Every scalar carries the same key set in the same order.
    for scalar in scalars {
        let keys: Vec<&str> = scalar
            .as_object()
            .expect("scalar object")
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(keys, first_keys, "{}", scalar["canonical"]);
    }

    let email = scalars
        .iter()
        .find(|scalar| scalar["canonical"] == "Contact.Email")
        .expect("Contact.Email");
    assert_eq!(email["id"], 8);
    assert_eq!(email["extension"], "builtin");
    assert_eq!(
        email["type_mappings"][0],
        serde_json::json!(["typescript", "string"])
    );
    assert_eq!(email["hooks"]["normalize"], true);
}
