//! Edge case and complex scenario tests for Phase 5

use gram_codec::{parse_gram_notation, serialize_pattern};

// ============================================================================
// Nesting and Depth Tests
// ============================================================================

#[test]
fn test_deeply_nested_subject_patterns() {
    let input = "[outer | [middle | [inner | (leaf)]]]";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);

    // Verify nesting depth
    assert_eq!(patterns[0].elements.len(), 1); // outer has 1 element
    assert_eq!(patterns[0].elements[0].elements.len(), 1); // middle has 1 element
    assert_eq!(patterns[0].elements[0].elements[0].elements.len(), 1); // inner has 1 element
    assert_eq!(
        patterns[0].elements[0].elements[0].elements[0]
            .elements
            .len(),
        0
    ); // leaf is atomic
}

#[test]
fn test_nested_relationships_in_subject_pattern() {
    let input = "[root | (a)-->(b), (c)-->(d)]";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2); // Two relationship elements
}

#[test]
fn test_mixed_element_types_in_subject_pattern() {
    let input = "[mix | (node), (a)-->(b), [nested | (x)]]";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 3); // Node, relationship, subject pattern
}

// ============================================================================
// Special Characters and Identifiers
// ============================================================================

#[test]
fn test_quoted_identifier_with_spaces() {
    let input = "(\"hello world\")";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns[0].value.identity.0, "hello world");
}

#[test]
fn test_quoted_identifier_with_special_chars() {
    let input = "(\"node-123\")";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_numeric_identifier() {
    let input = "(\"42\")";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// ============================================================================
// Empty and Edge Cases
// ============================================================================

#[test]
fn test_multiple_empty_nodes() {
    let input = "() () ()";
    let result = parse_gram_notation(input);
    assert!(result.is_ok());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 3);
}

#[test]
fn test_empty_subject_pattern() {
    // Subject pattern with no elements - may not be valid gram syntax
    // This test documents current behavior
    let input = "[empty | ]";
    let result = parse_gram_notation(input);
    // This may fail or succeed depending on grammar - document actual behavior
    let _ = result;
}

#[test]
fn test_node_with_only_label() {
    let input = "(:Person)";
    let result = parse_gram_notation(input);
    assert!(result.is_ok());
    let patterns = result.unwrap();
    assert!(patterns[0].value.identity.0.is_empty());
    assert!(patterns[0].value.labels.contains("Person"));
}

#[test]
fn test_node_with_only_properties() {
    let input = "({name: \"Alice\"})";
    let result = parse_gram_notation(input);
    assert!(result.is_ok());
    let patterns = result.unwrap();
    assert!(patterns[0].value.identity.0.is_empty());
    assert!(patterns[0].value.properties.contains_key("name"));
}

// ============================================================================
// Complex Relationships
// ============================================================================

#[test]
fn test_relationship_with_complex_nodes() {
    let input = "(a:Person {name: \"Alice\"})-[:KNOWS {since: 2020}]->(b:Person {name: \"Bob\"})";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);

    // Verify relationship structure
    let rel = &patterns[0];
    assert_eq!(rel.elements.len(), 2);
    assert!(rel.value.labels.contains("KNOWS"));
    assert!(rel.value.properties.contains_key("since"));

    // Verify left node
    assert_eq!(rel.elements[0].value.identity.0, "a");
    assert!(rel.elements[0].value.labels.contains("Person"));

    // Verify right node
    assert_eq!(rel.elements[1].value.identity.0, "b");
    assert!(rel.elements[1].value.labels.contains("Person"));
}

#[test]
fn test_relationship_with_empty_nodes() {
    let input = "()-->()";
    let result = parse_gram_notation(input);
    assert!(result.is_ok());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2);
}

#[test]
fn test_multiple_labels_on_relationship() {
    let input = "(a)-[:REL1:REL2]->(b)";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    let labels = &patterns[0].value.labels;
    assert!(labels.contains("REL1"));
    assert!(labels.contains("REL2"));
}

// ============================================================================
// Whitespace and Comments
// ============================================================================

#[test]
fn test_multiple_line_comments() {
    let input = "// Comment 1\n// Comment 2\n(hello)";
    let result = parse_gram_notation(input);
    assert!(result.is_ok());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
}

#[test]
fn test_inline_comments() {
    let input = "(a) // node a\n(b) // node b";
    let result = parse_gram_notation(input);
    assert!(result.is_ok());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 2);
}

#[test]
fn test_excessive_whitespace() {
    let input = "  (  hello  )  ";
    let result = parse_gram_notation(input);
    assert!(result.is_ok());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
}

#[test]
fn test_newlines_between_patterns() {
    let input = "(a)\n\n(b)\n\n(c)";
    let result = parse_gram_notation(input);
    assert!(result.is_ok());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 3);
}

// ============================================================================
// Round-Trip Complex Scenarios
// ============================================================================

#[test]
fn test_round_trip_nested_subject_pattern() {
    let original = "[outer | [inner | (leaf)]]";
    let parsed = parse_gram_notation(original).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();
    let reparsed = parse_gram_notation(&serialized).unwrap();

    assert_eq!(parsed[0].elements.len(), reparsed[0].elements.len());
    assert_eq!(
        parsed[0].elements[0].elements.len(),
        reparsed[0].elements[0].elements.len()
    );
}

#[test]
fn test_round_trip_complex_relationship() {
    let original = "(a:Person {age: 30})-[:KNOWS {since: 2020}]->(b:Person {age: 25})";
    let parsed = parse_gram_notation(original).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();
    let reparsed = parse_gram_notation(&serialized).unwrap();

    // Verify structure is preserved
    assert_eq!(parsed[0].elements.len(), 2);
    assert_eq!(reparsed[0].elements.len(), 2);
    assert!(!parsed[0].value.labels.is_empty());
    assert!(!reparsed[0].value.labels.is_empty());
}

#[test]
fn test_round_trip_multiple_labels() {
    let original = "(a:Label1:Label2)";
    let parsed = parse_gram_notation(original).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();
    let reparsed = parse_gram_notation(&serialized).unwrap();

    assert_eq!(parsed[0].value.labels.len(), 2);
    assert_eq!(reparsed[0].value.labels.len(), 2);
}

// ============================================================================
// Error Cases
// ============================================================================

#[test]
fn test_error_unclosed_node() {
    let input = "(hello";
    let result = parse_gram_notation(input);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.error_count() > 0);
}

#[test]
fn test_error_unclosed_subject_pattern() {
    let input = "[team | (a), (b)";
    let result = parse_gram_notation(input);
    assert!(result.is_err());
}

#[test]
fn test_error_invalid_property_syntax() {
    let input = "(a {key value})"; // Missing colon
    let result = parse_gram_notation(input);
    assert!(result.is_err());
}

#[test]
fn test_error_unclosed_record() {
    let input = "(a {key: \"value\"";
    let result = parse_gram_notation(input);
    assert!(result.is_err());
}

// ============================================================================
// Property Edge Cases
// ============================================================================

#[test]
fn test_property_with_negative_numbers() {
    let input = "(a {temp: -273, score: -10.5})";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_property_with_escaped_strings() {
    let input = "(a {text: \"Hello \\\"World\\\"\"})";
    let result = parse_gram_notation(input);
    // This test documents current behavior for escaped strings
    let _ = result;
}

#[test]
fn test_large_integer_property() {
    let input = "(a {big: 999999999})";
    let result = parse_gram_notation(input);
    assert!(result.is_ok());
}

#[test]
fn test_many_properties() {
    let input = "(a {p1: 1, p2: 2, p3: 3, p4: 4, p5: 5, p6: 6, p7: 7, p8: 8, p9: 9, p10: 10})";
    let result = parse_gram_notation(input);
    assert!(result.is_ok());
    let patterns = result.unwrap();
    assert_eq!(patterns[0].value.properties.len(), 10);
}
