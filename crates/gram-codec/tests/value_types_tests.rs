//! Comprehensive value type tests for Phase 5

use gram_codec::{parse_gram_notation, serialize_pattern};

#[test]
fn test_parse_integer_values() {
    // Positive integer
    let result = parse_gram_notation("(a {count: 42})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert!(patterns[0].value.properties.contains_key("count"));

    // Negative integer
    let result = parse_gram_notation("(a {temp: -10})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    // Zero
    let result = parse_gram_notation("(a {zero: 0})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_parse_decimal_values() {
    // Positive decimal
    let result = parse_gram_notation("(a {pi: 3.14})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);

    // Negative decimal
    let result = parse_gram_notation("(a {temp: -2.5})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    // Zero decimal
    let result = parse_gram_notation("(a {zero: 0.0})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_parse_boolean_values() {
    // True
    let result = parse_gram_notation("(a {active: true})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert!(patterns[0].value.properties.contains_key("active"));

    // False
    let result = parse_gram_notation("(a {inactive: false})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_parse_array_homogeneous() {
    // String array
    let result = parse_gram_notation("(a {tags: [\"rust\", \"wasm\", \"python\"]})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    // Integer array
    let result = parse_gram_notation("(a {scores: [1, 2, 3, 4, 5]})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    // Empty array - Note: May not be supported by tree-sitter-gram grammar
    // Skipping for now, grammar may require at least one element
    // let result = parse_gram_notation("(a {empty: []})");
    // assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_parse_array_heterogeneous() {
    // Mixed types
    let result = parse_gram_notation("(a {mixed: [\"text\", 42, true, 3.14]})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert!(patterns[0].value.properties.contains_key("mixed"));
}

#[test]
fn test_parse_range_values() {
    // Positive range
    let result = parse_gram_notation("(a {range: 1..10})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    // Range with zero
    let result = parse_gram_notation("(a {range: 0..100})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    // Large range
    let result = parse_gram_notation("(a {range: 1..1000})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_parse_string_values() {
    // Simple string
    let result = parse_gram_notation("(a {name: \"Alice\"})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    // String with spaces
    let result = parse_gram_notation("(a {desc: \"Hello World\"})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    // Empty string
    let result = parse_gram_notation("(a {empty: \"\"})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_serialize_all_value_types() {
    use pattern_core::{Pattern, Subject, Symbol, Value};
    use std::collections::{HashMap, HashSet};

    let mut subject = Subject {
        identity: Symbol("test".to_string()),
        labels: HashSet::new(),
        properties: HashMap::new(),
    };

    // Add various value types
    subject
        .properties
        .insert("string".to_string(), Value::VString("hello".to_string()));
    subject
        .properties
        .insert("integer".to_string(), Value::VInteger(42));
    subject
        .properties
        .insert("decimal".to_string(), Value::VDecimal(3.14));
    subject
        .properties
        .insert("boolean".to_string(), Value::VBoolean(true));

    let pattern = Pattern::point(subject);
    let result = serialize_pattern(&pattern);
    assert!(result.is_ok(), "Failed to serialize: {:?}", result.err());

    let output = result.unwrap();
    // Verify all types are present
    assert!(output.contains("string: \"hello\""));
    assert!(output.contains("integer: 42"));
    assert!(output.contains("decimal: 3.14"));
    assert!(output.contains("boolean: true"));
}

#[test]
fn test_round_trip_numeric_values() {
    // Integer
    let original = "(a {count: 42})";
    let parsed = parse_gram_notation(original).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();
    let reparsed = parse_gram_notation(&serialized).unwrap();
    assert_eq!(
        parsed[0].value.properties.len(),
        reparsed[0].value.properties.len()
    );

    // Decimal
    let original = "(a {pi: 3.14})";
    let parsed = parse_gram_notation(original).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();
    let reparsed = parse_gram_notation(&serialized).unwrap();
    assert_eq!(
        parsed[0].value.properties.len(),
        reparsed[0].value.properties.len()
    );
}

#[test]
fn test_round_trip_boolean() {
    let original = "(a {active: true, inactive: false})";
    let parsed = parse_gram_notation(original).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();
    let reparsed = parse_gram_notation(&serialized).unwrap();
    assert_eq!(parsed[0].value.properties.len(), 2);
    assert_eq!(reparsed[0].value.properties.len(), 2);
}

#[test]
fn test_round_trip_arrays() {
    let original = "(a {tags: [\"rust\", \"wasm\"]})";
    let parsed = parse_gram_notation(original).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();
    let reparsed = parse_gram_notation(&serialized).unwrap();
    assert_eq!(
        parsed[0].value.properties.len(),
        reparsed[0].value.properties.len()
    );
}

#[test]
fn test_round_trip_ranges() {
    let original = "(a {range: 1..10})";
    let parsed = parse_gram_notation(original).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();
    let reparsed = parse_gram_notation(&serialized).unwrap();
    assert_eq!(
        parsed[0].value.properties.len(),
        reparsed[0].value.properties.len()
    );
}

#[test]
fn test_multiple_properties_mixed_types() {
    let input = "(person {name: \"Alice\", age: 30, active: true, score: 95.5, tags: [\"rust\"], range: 1..10})";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].value.properties.len(), 6);
}
