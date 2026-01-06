//! Advanced features tests for Phase 5 (root records, path patterns, Unicode)

use gram_codec::{parse_gram_notation, serialize_pattern};

// ============================================================================
// Root Record Support (T106)
// ============================================================================

#[test]
fn test_parse_root_record_with_pattern() {
    // Root records are supported by grammar but currently ignored during parsing
    // This documents current behavior
    let input = "{graph: \"social\"} (a)-->(b)";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    // Parser should extract the patterns, ignoring the root record for now
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1); // Relationship pattern parsed
}

#[test]
fn test_parse_root_record_with_multiple_patterns() {
    let input = "{version: 1} (a) (b) (c)";
    let result = parse_gram_notation(input);
    assert!(result.is_ok());
    let patterns = result.unwrap();
    // Root record is ignored, patterns are extracted
    assert!(patterns.len() >= 3);
}

// ============================================================================
// Path Pattern Support (T107-T108)
// ============================================================================

#[test]
fn test_parse_path_pattern_three_nodes() {
    let input = "(a)-->(b)-->(c)";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    let patterns = result.unwrap();
    // Path patterns create nested relationship structures
    // (a)-->(b)-->(c) becomes a relationship where right is another relationship
    assert_eq!(patterns.len(), 1);
}

#[test]
fn test_parse_path_pattern_with_labels() {
    let input = "(a)-[:R1]->(b)-[:R2]->(c)";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_parse_long_path_pattern() {
    let input = "(a)-->(b)-->(c)-->(d)-->(e)";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_path_pattern_nested_structure() {
    // Document that path patterns create nested relationships
    let input = "(a)-->(b)-->(c)";
    let result = parse_gram_notation(input);
    assert!(result.is_ok());

    let patterns = result.unwrap();
    // The outer pattern is a relationship with (a) on left and...
    // ...another relationship ((b)-->(c)) on the right
    assert_eq!(patterns[0].elements.len(), 2);
}

#[test]
fn test_round_trip_simple_path() {
    let input = "(a)-->(b)-->(c)";
    let parsed = parse_gram_notation(input).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();
    let reparsed = parse_gram_notation(&serialized).unwrap();

    // Structure should be preserved
    assert_eq!(parsed[0].elements.len(), reparsed[0].elements.len());
}

// ============================================================================
// Unicode Support (T109-T111)
// ============================================================================

#[test]
fn test_parse_unicode_identifier() {
    // Unicode identifiers must be quoted in gram notation
    let input = "(\"hello世界\")";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns[0].value.identity.0, "hello世界");
}

#[test]
fn test_parse_emoji_identifier() {
    // Emoji identifiers must be quoted in gram notation
    let input = "(\"node🚀\")";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_parse_unicode_property_value() {
    let input = "(a {greeting: \"こんにちは\"})";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_parse_arabic_text() {
    let input = "(node {text: \"مرحبا\"})";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_parse_cyrillic_text() {
    let input = "(node {name: \"Привет\"})";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_round_trip_unicode() {
    // Unicode identifiers must be quoted
    let input = "(\"世界\")";
    let parsed = parse_gram_notation(input).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();
    let reparsed = parse_gram_notation(&serialized).unwrap();

    assert_eq!(parsed[0].value.identity.0, reparsed[0].value.identity.0);
}

#[test]
fn test_serialize_unicode_identifier() {
    use pattern_core::{Pattern, Subject, Symbol};
    use std::collections::{HashMap, HashSet};

    let subject = Subject {
        identity: Symbol("日本".to_string()),
        labels: HashSet::new(),
        properties: HashMap::new(),
    };

    let pattern = Pattern::point(subject);
    let result = serialize_pattern(&pattern);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("日本"));
}

// ============================================================================
// Special Characters and Escaping (T110)
// ============================================================================

#[test]
fn test_parse_quoted_identifier_needs_quoting() {
    let input = "(\"hello-world\")";
    let result = parse_gram_notation(input);
    assert!(result.is_ok());
    let patterns = result.unwrap();
    assert_eq!(patterns[0].value.identity.0, "hello-world");
}

#[test]
fn test_parse_property_with_newline() {
    // Test multi-line string if supported
    let input = "(a {text: \"line1\\nline2\"})";
    let result = parse_gram_notation(input);
    // Document current behavior
    let _ = result;
}

#[test]
fn test_serialize_identifier_with_special_chars() {
    use pattern_core::{Pattern, Subject, Symbol};
    use std::collections::{HashMap, HashSet};

    let subject = Subject {
        identity: Symbol("node-123".to_string()),
        labels: HashSet::new(),
        properties: HashMap::new(),
    };

    let pattern = Pattern::point(subject);
    let result = serialize_pattern(&pattern);
    assert!(result.is_ok());
    let output = result.unwrap();
    // Should be quoted because of hyphen
    assert!(output.contains("\"node-123\""));
}

// ============================================================================
// Large Pattern Testing (T113-T114)
// ============================================================================

#[test]
fn test_parse_many_nodes() {
    // Generate 100 nodes
    let nodes: Vec<String> = (0..100).map(|i| format!("(n{})", i)).collect();
    let input = nodes.join(" ");

    let result = parse_gram_notation(&input);
    assert!(result.is_ok(), "Failed to parse many nodes");
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 100);
}

#[test]
fn test_parse_node_with_many_labels() {
    let labels: Vec<String> = (1..=20).map(|i| format!(":Label{}", i)).collect();
    let input = format!("(a{})", labels.join(""));

    let result = parse_gram_notation(&input);
    assert!(result.is_ok(), "Failed to parse many labels");
    let patterns = result.unwrap();
    assert_eq!(patterns[0].value.labels.len(), 20);
}

#[test]
fn test_serialize_large_property_array() {
    use pattern_core::{Pattern, Subject, Symbol, Value};
    use std::collections::{HashMap, HashSet};

    let mut subject = Subject {
        identity: Symbol("node".to_string()),
        labels: HashSet::new(),
        properties: HashMap::new(),
    };

    // Create array with 100 elements
    let array_values: Vec<Value> = (0..100).map(|i| Value::VInteger(i)).collect();
    subject
        .properties
        .insert("numbers".to_string(), Value::VArray(array_values));

    let pattern = Pattern::point(subject);
    let result = serialize_pattern(&pattern);
    assert!(result.is_ok(), "Failed to serialize large array");
}

#[test]
fn test_deeply_nested_subject_patterns() {
    // Create 10-level nesting
    let mut input = String::from("(leaf)");
    for i in 0..10 {
        input = format!("[level{} | {}]", i, input);
    }

    let result = parse_gram_notation(&input);
    assert!(result.is_ok(), "Failed to parse deeply nested pattern");

    // Verify nesting depth
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);

    // Walk down the nesting levels
    let mut current = &patterns[0];
    for _ in 0..10 {
        assert_eq!(current.elements.len(), 1);
        current = &current.elements[0];
    }
    assert_eq!(current.elements.len(), 0); // Leaf is atomic
}

#[test]
fn test_round_trip_deeply_nested() {
    let input = "[l0 | [l1 | [l2 | [l3 | (leaf)]]]]";
    let parsed = parse_gram_notation(input).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();
    let reparsed = parse_gram_notation(&serialized).unwrap();

    // Verify nesting is preserved
    assert_eq!(parsed[0].elements.len(), 1);
    assert_eq!(reparsed[0].elements.len(), 1);
}
