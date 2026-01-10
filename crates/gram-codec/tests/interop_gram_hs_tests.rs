//! Interoperability tests with gram-hs canonical JSON format
//!
//! These tests verify that gram-rs AST output can be consumed by gram-hs
//! and vice versa, proving end-to-end interoperability.

use gram_codec::parse_to_ast;
use serde_json::Value as JsonValue;

/// Test that gram-rs AST matches gram-hs canonical format structure
///
/// This verifies:
/// 1. Field names match (subject, identity, labels, properties)
/// 2. Value types match (native JSON for numbers, tagged for complex types)
/// 3. Type discriminators are lowercase
#[test]
fn test_canonical_format_structure() {
    let ast = parse_to_ast("(alice:Person {name: \"Alice\", age: 30})").unwrap();
    let json = serde_json::to_value(&ast).unwrap();

    // Verify Pattern structure
    assert!(json.is_object());
    assert!(json.get("subject").is_some());
    assert!(json.get("elements").is_some());

    // Verify Subject structure
    let subject = json.get("subject").unwrap();
    assert!(subject.get("identity").is_some());
    assert!(subject.get("labels").is_some());
    assert!(subject.get("properties").is_some());

    // Verify identity is string
    assert!(subject.get("identity").unwrap().is_string());

    // Verify labels is array
    assert!(subject.get("labels").unwrap().is_array());

    // Verify properties is object
    let props = subject.get("properties").unwrap().as_object().unwrap();

    // Verify native JSON for numbers
    assert!(props.get("age").unwrap().is_number());
    assert_eq!(props.get("age").unwrap().as_i64(), Some(30));

    // Verify native JSON for strings
    assert!(props.get("name").unwrap().is_string());
}

/// Test that type discriminators use lowercase (matching gram-hs)
#[test]
fn test_lowercase_type_discriminators() {
    let gram = r#"(node {
        range: 1..10,
        measure: 5kg
    })"#;

    let ast = parse_to_ast(gram).unwrap();
    let json = serde_json::to_value(&ast).unwrap();

    let props = json["subject"]["properties"].as_object().unwrap();

    // Check range uses lowercase
    if let Some(range_val) = props.get("range") {
        assert_eq!(range_val["type"], "range");
    }

    // Check measurement uses lowercase
    if let Some(meas_val) = props.get("measure") {
        assert_eq!(meas_val["type"], "measurement");
    }
}

/// Test that integers and decimals are native JSON (not tagged)
#[test]
fn test_native_json_numbers() {
    let ast = parse_to_ast("(test {int: 42, decimal: 3.14})").unwrap();
    let json = serde_json::to_value(&ast).unwrap();

    let props = json["subject"]["properties"].as_object().unwrap();

    // Integer should be native JSON number
    let int_val = props.get("int").unwrap();
    assert!(int_val.is_number());
    assert!(!int_val.is_object()); // Not a tagged object
    assert_eq!(int_val.as_i64(), Some(42));

    // Decimal should be native JSON number
    let dec_val = props.get("decimal").unwrap();
    assert!(dec_val.is_number());
    assert!(!dec_val.is_object()); // Not a tagged object
    assert_eq!(dec_val.as_f64(), Some(3.14));
}

/// Test that complex nested structures match canonical format
#[test]
fn test_nested_structure_format() {
    let ast = parse_to_ast("[outer:Outer {version: 1} | [inner:Inner | (a), (b)], (c)]").unwrap();
    let json = serde_json::to_value(&ast).unwrap();

    // Verify top-level structure
    assert_eq!(json["subject"]["identity"], "outer");
    assert_eq!(json["subject"]["labels"][0], "Outer");
    assert_eq!(json["elements"].as_array().unwrap().len(), 2);

    // Verify nested structure
    let first_elem = &json["elements"][0];
    assert_eq!(first_elem["subject"]["identity"], "inner");
    assert_eq!(first_elem["subject"]["labels"][0], "Inner");
    assert_eq!(first_elem["elements"].as_array().unwrap().len(), 2);

    // Verify leaf nodes
    let leaf1 = &first_elem["elements"][0];
    assert_eq!(leaf1["subject"]["identity"], "a");
    assert_eq!(leaf1["elements"].as_array().unwrap().len(), 0);
}

/// Test that empty patterns match canonical format
#[test]
fn test_empty_pattern_format() {
    let ast = parse_to_ast("").unwrap();
    let json = serde_json::to_value(&ast).unwrap();

    // Empty pattern should have empty subject and empty elements
    assert_eq!(json["subject"]["identity"], "");
    assert_eq!(json["subject"]["labels"].as_array().unwrap().len(), 0);
    assert_eq!(json["subject"]["properties"].as_object().unwrap().len(), 0);
    assert_eq!(json["elements"].as_array().unwrap().len(), 0);
}

/// Test that arrays and maps are native JSON (not tagged)
#[test]
fn test_native_json_collections() {
    let ast = parse_to_ast("(test {arr: [1, 2, 3], map: {key: \"value\"}})").unwrap();
    let json = serde_json::to_value(&ast).unwrap();

    let props = json["subject"]["properties"].as_object().unwrap();

    // Array should be native JSON array
    let arr_val = props.get("arr").unwrap();
    assert!(arr_val.is_array());
    assert!(!arr_val.is_object()); // Not a tagged object
    assert_eq!(arr_val.as_array().unwrap().len(), 3);

    // Map should be native JSON object (without type field)
    let map_val = props.get("map").unwrap();
    assert!(map_val.is_object());
    let map_obj = map_val.as_object().unwrap();
    assert!(!map_obj.contains_key("type")); // Not a tagged type
    assert_eq!(map_obj.len(), 1);
    assert_eq!(map_obj.get("key").unwrap(), "value");
}
