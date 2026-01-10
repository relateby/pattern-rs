//! AST round-trip tests
//!
//! Tests that AST serialization/deserialization is lossless:
//! gram → AST → JSON → AST → (verify equivalence)

use gram_codec::{parse_to_ast, AstPattern};
use serde_json;

/// Test that AST can be serialized to JSON and deserialized back without loss
fn assert_ast_json_roundtrip(input: &str) {
    // Parse gram to AST
    let ast1 =
        parse_to_ast(input).unwrap_or_else(|e| panic!("Parse failed for '{}': {}", input, e));

    // Serialize AST to JSON
    let json = serde_json::to_string(&ast1)
        .unwrap_or_else(|e| panic!("JSON serialization failed for '{}': {}", input, e));

    // Deserialize JSON back to AST
    let ast2: AstPattern = serde_json::from_str(&json)
        .unwrap_or_else(|e| panic!("JSON deserialization failed for '{}': {}", json, e));

    // Verify equivalence
    assert_eq!(
        ast1, ast2,
        "AST round-trip failed for '{}':\n  AST1: {:?}\n  AST2: {:?}\n  JSON: {}",
        input, ast1, ast2, json
    );
}

#[test]
fn test_ast_json_roundtrip_simple_node() {
    assert_ast_json_roundtrip("(hello)");
}

#[test]
fn test_ast_json_roundtrip_node_with_label() {
    assert_ast_json_roundtrip("(alice:Person)");
}

#[test]
fn test_ast_json_roundtrip_node_with_properties() {
    assert_ast_json_roundtrip("(alice {name: \"Alice\", age: 30})");
}

#[test]
fn test_ast_json_roundtrip_node_full() {
    assert_ast_json_roundtrip("(alice:Person {name: \"Alice\", age: 30, active: true})");
}

#[test]
fn test_ast_json_roundtrip_with_elements() {
    assert_ast_json_roundtrip("[team | (alice), (bob)]");
}

#[test]
fn test_ast_json_roundtrip_nested() {
    assert_ast_json_roundtrip("[outer | [inner | (a), (b)], (c)]");
}

#[test]
fn test_ast_json_roundtrip_all_value_types() {
    // Test with all supported value types
    let gram = r#"(node {
        int: 42,
        decimal: 3.14,
        bool: true,
        str: "hello",
        array: [1, 2, 3],
        map: {key: "value"}
    })"#;

    assert_ast_json_roundtrip(gram);
}

#[test]
fn test_ast_json_roundtrip_empty() {
    assert_ast_json_roundtrip("");
}

#[test]
fn test_ast_json_roundtrip_path_notation() {
    assert_ast_json_roundtrip("(alice)-->(bob)");
}

#[test]
fn test_ast_json_preserves_structure() {
    let input = "(alice:Person {name: \"Alice\", age: 30})";
    let ast1 = parse_to_ast(input).unwrap();

    // Serialize and deserialize
    let json = serde_json::to_string(&ast1).unwrap();
    let ast2: AstPattern = serde_json::from_str(&json).unwrap();

    // Verify structure preserved
    assert_eq!(ast1.subject.identity, ast2.subject.identity);
    assert_eq!(ast1.subject.labels, ast2.subject.labels);
    assert_eq!(ast1.subject.properties.len(), ast2.subject.properties.len());
    assert_eq!(ast1.elements.len(), ast2.elements.len());

    // Verify property values preserved
    for (key, val1) in &ast1.subject.properties {
        let val2 = ast2
            .subject
            .properties
            .get(key)
            .unwrap_or_else(|| panic!("Property '{}' missing after round-trip", key));
        assert_eq!(val1, val2, "Property '{}' value changed", key);
    }
}
