//! Arrow type and relationship pattern tests for Phase 5

use gram_codec::{parse_gram_notation, serialize_pattern};

#[test]
fn test_right_arrow_simple() {
    let result = parse_gram_notation("(a)-->(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2);
}

#[test]
fn test_left_arrow_element_reversal() {
    let result = parse_gram_notation("(a)<--(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2);

    // Left arrow reverses element order
    // (a)<--(b) means b-->a
    assert_eq!(patterns[0].elements[0].value.identity.0, "b");
    assert_eq!(patterns[0].elements[1].value.identity.0, "a");
}

#[test]
fn test_bidirectional_arrow() {
    let result = parse_gram_notation("(a)<-->(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2);
}

#[test]
fn test_squiggle_undirected() {
    let result = parse_gram_notation("(a)~~(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
}

#[test]
#[ignore] // Not supported by current tree-sitter-gram grammar
fn test_squiggle_directed() {
    let result = parse_gram_notation("(a)~>(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
}

#[test]
fn test_relationship_with_label() {
    let result = parse_gram_notation("(a)-[:KNOWS]->(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert!(patterns[0].value.labels.contains("KNOWS"));
}

#[test]
fn test_relationship_with_properties() {
    let result = parse_gram_notation("(a)-[:KNOWS {since: 2020}]->(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert!(!patterns[0].value.properties.is_empty());
}

#[test]
fn test_relationship_with_label_and_properties() {
    let result = parse_gram_notation("(a:Person)-[:KNOWS {since: 2020}]->(b:Person)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);

    // Check edge has label and properties
    assert!(patterns[0].value.labels.contains("KNOWS"));
    assert!(patterns[0].value.properties.contains_key("since"));

    // Check nodes have labels
    assert!(patterns[0].elements[0].value.labels.contains("Person"));
    assert!(patterns[0].elements[1].value.labels.contains("Person"));
}

#[test]
fn test_round_trip_left_arrow() {
    let original = "(a)<--(b)";
    let parsed = parse_gram_notation(original).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();

    // After parsing (a)<--(b), we get b-->a
    // After serializing, we get (b)-->(a)
    assert!(serialized.contains("(b)"));
    assert!(serialized.contains("(a)"));
}

#[test]
fn test_round_trip_bidirectional() {
    let original = "(a)<-->(b)";
    let parsed = parse_gram_notation(original).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();
    let reparsed = parse_gram_notation(&serialized).unwrap();

    assert_eq!(parsed[0].elements.len(), 2);
    assert_eq!(reparsed[0].elements.len(), 2);
}

#[test]
fn test_round_trip_squiggle() {
    let original = "(a)~~(b)";
    let parsed = parse_gram_notation(original).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();
    let reparsed = parse_gram_notation(&serialized).unwrap();

    assert_eq!(parsed[0].elements.len(), 2);
    assert_eq!(reparsed[0].elements.len(), 2);
}

#[test]
fn test_round_trip_labeled_relationship() {
    let original = "(a)-[:KNOWS]->(b)";
    let parsed = parse_gram_notation(original).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();

    // Should preserve label
    assert!(serialized.contains("KNOWS"));

    let reparsed = parse_gram_notation(&serialized).unwrap();
    assert!(reparsed[0].value.labels.contains("KNOWS"));
}

#[test]
fn test_relationship_empty_edge() {
    let result = parse_gram_notation("(a)-->(b)");
    assert!(result.is_ok());
    let patterns = result.unwrap();

    // Empty edge has no identifier, labels, or properties
    assert!(patterns[0].value.identity.0.is_empty());
    assert!(patterns[0].value.labels.is_empty());
    assert!(patterns[0].value.properties.is_empty());
}

#[test]
fn test_chained_relationships() {
    // Path pattern: (a)-->(b)-->(c)
    let result = parse_gram_notation("(a)-->(b)-->(c)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();

    // Chained relationships are flattened
    // This creates a root pattern with multiple relationship elements
    assert!(patterns.len() >= 1);
}

#[test]
fn test_complex_nodes_in_relationship() {
    let input = "(a:Person {name: \"Alice\"})-[:KNOWS {since: 2020}]->(b:Person {name: \"Bob\"})";
    let result = parse_gram_notation(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);

    // Verify structure
    assert_eq!(patterns[0].elements.len(), 2);

    // First node (Alice)
    assert_eq!(patterns[0].elements[0].value.identity.0, "a");
    assert!(patterns[0].elements[0].value.labels.contains("Person"));
    assert!(patterns[0].elements[0]
        .value
        .properties
        .contains_key("name"));

    // Second node (Bob)
    assert_eq!(patterns[0].elements[1].value.identity.0, "b");
    assert!(patterns[0].elements[1].value.labels.contains("Person"));
    assert!(patterns[0].elements[1]
        .value
        .properties
        .contains_key("name"));
}
