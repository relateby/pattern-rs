//! Round-trip tests for gram notation
//!
//! Tests semantic equivalence: gram -> pattern -> gram -> pattern
//! The second pattern should equal the first pattern, proving round-trip correctness.

use gram_codec::{parse_gram, serialize_patterns};

/// Helper to test round-trip semantic equivalence
fn assert_round_trip_equivalent(input: &str) {
    // First parse: gram1 -> pattern1
    let patterns1 =
        parse_gram(input).unwrap_or_else(|e| panic!("First parse failed for '{}': {}", input, e));

    // Serialize: pattern1 -> gram2
    let gram2 = serialize_patterns(&patterns1)
        .unwrap_or_else(|e| panic!("Serialization failed for '{}': {}", input, e));

    // Second parse: gram2 -> pattern2
    let patterns2 = parse_gram(&gram2).unwrap_or_else(|e| {
        panic!(
            "Second parse failed for '{}' (from '{}'): {}",
            gram2, input, e
        )
    });

    // Verify semantic equivalence: pattern1 == pattern2
    assert_eq!(
        patterns1, patterns2,
        "Round-trip semantic equivalence failed:\n  Original: {}\n  Serialized: {}\n  Pattern1: {:?}\n  Pattern2: {:?}",
        input, gram2, patterns1, patterns2
    );
}

#[test]
fn test_round_trip_simple_node() {
    assert_round_trip_equivalent("(hello)");
}

#[test]
fn test_round_trip_node_with_label() {
    assert_round_trip_equivalent("(alice:Person)");
}

#[test]
fn test_round_trip_node_with_multiple_labels() {
    assert_round_trip_equivalent("(alice:Person:User)");
}

#[test]
fn test_round_trip_node_with_properties() {
    assert_round_trip_equivalent("(alice {name: \"Alice\"})");
}

#[test]
fn test_round_trip_node_full() {
    assert_round_trip_equivalent("(alice:Person {name: \"Alice\", age: 30})");
}

#[test]
fn test_round_trip_relationship_simple() {
    assert_round_trip_equivalent("(a)-->(b)");
}

// Note: Additional relationship arrow types and labeled edges are future enhancements
// Current parser only supports `-->` arrow type
// TODO: Add tests for `--`, `<--`, `<-->` when parser implements them
// TODO: Add tests for `-[:LABEL]->` syntax when parser implements it

#[test]
fn test_round_trip_path_simple() {
    assert_round_trip_equivalent("(a)-->(b)-->(c)");
}

// TODO: Add test for paths with labeled edges when parser implements `-[:LABEL]->` syntax

#[test]
fn test_round_trip_subject_pattern_simple() {
    assert_round_trip_equivalent("[team | (alice), (bob)]");
}

#[test]
fn test_round_trip_subject_pattern_with_labels() {
    assert_round_trip_equivalent("[team:Group | (alice:Person), (bob:Person)]");
}

#[test]
fn test_round_trip_annotation_simple() {
    assert_round_trip_equivalent("@deprecated (old_node)");
}

// TODO: Add test for annotations with values when parser implements `@key(value)` syntax

#[test]
fn test_round_trip_multiple_patterns() {
    assert_round_trip_equivalent("(a) (b) (c)");
}

#[test]
fn test_round_trip_value_string() {
    assert_round_trip_equivalent("(node {name: \"value\"})");
}

#[test]
fn test_round_trip_value_integer() {
    assert_round_trip_equivalent("(node {count: 42})");
}

#[test]
fn test_round_trip_value_decimal() {
    assert_round_trip_equivalent("(node {price: 19.99})");
}

#[test]
fn test_round_trip_value_boolean_true() {
    assert_round_trip_equivalent("(node {active: true})");
}

#[test]
fn test_round_trip_value_boolean_false() {
    assert_round_trip_equivalent("(node {active: false})");
}

#[test]
fn test_round_trip_value_array() {
    assert_round_trip_equivalent("(node {tags: [\"a\", \"b\", \"c\"]})");
}

#[test]
fn test_round_trip_value_range_full() {
    assert_round_trip_equivalent("(node {range: 0..10})");
}

// TODO: Add tests for partial ranges (`5..`, `..10`, `..`) when parser implements them

#[test]
fn test_round_trip_complex_pattern() {
    // Test complex node with labels and properties
    assert_round_trip_equivalent("(alice:Person {name: \"Alice\", age: 30})");
}

#[test]
fn test_round_trip_nested_subject() {
    assert_round_trip_equivalent("[outer | [inner | (a), (b)], (c)]");
}

/// Test that whitespace differences don't affect semantic equivalence
#[test]
fn test_round_trip_whitespace_normalization() {
    let input1 = "(a)-->(b)";
    let input2 = "(a) --> (b)";
    let input3 = "( a )-->( b )";

    let patterns1 = parse_gram(input1).unwrap();
    let patterns2 = parse_gram(input2).unwrap();
    let patterns3 = parse_gram(input3).unwrap();

    // All should parse to the same semantic structure
    assert_eq!(
        patterns1, patterns2,
        "Whitespace should not affect semantics"
    );
    assert_eq!(
        patterns1, patterns3,
        "Whitespace should not affect semantics"
    );
}

/// Test that serialization is idempotent after first round-trip
#[test]
fn test_round_trip_idempotent() {
    let input = "(alice:Person {name: \"Alice\"})-->(bob:Person)";

    // First round-trip: gram1 -> pattern1 -> gram2
    let patterns1 = parse_gram(input).unwrap();
    let gram2 = serialize_patterns(&patterns1).unwrap();
    let patterns2 = parse_gram(&gram2).unwrap();

    // Second round-trip: gram2 -> pattern2 -> gram3
    let gram3 = serialize_patterns(&patterns2).unwrap();
    let patterns3 = parse_gram(&gram3).unwrap();

    // Third round-trip: gram3 -> pattern3 -> gram4
    let gram4 = serialize_patterns(&patterns3).unwrap();

    // After stabilization, output should be identical
    assert_eq!(
        gram3, gram4,
        "Serialization should be idempotent after first round-trip"
    );
    assert_eq!(
        patterns2, patterns3,
        "Patterns should be identical after stabilization"
    );
}
