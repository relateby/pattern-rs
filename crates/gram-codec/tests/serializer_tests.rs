//! Serializer integration tests

use gram_codec::{parse_gram_notation, serialize_pattern, serialize_patterns};
use pattern_core::{Pattern, Subject, Symbol};
use std::collections::{HashMap, HashSet};

// Helper function to create a Subject with identifier
fn subject_with_id(id: &str) -> Subject {
    Subject {
        identity: Symbol(id.to_string()),
        labels: HashSet::new(),
        properties: HashMap::new(),
    }
}

// Helper function to create an empty Subject
fn empty_subject() -> Subject {
    Subject {
        identity: Symbol(String::new()),
        labels: HashSet::new(),
        properties: HashMap::new(),
    }
}

#[test]
fn test_serialize_empty_node() {
    let pattern = Pattern::point(empty_subject());
    let result = serialize_pattern(&pattern);
    assert!(result.is_ok(), "Failed to serialize: {:?}", result.err());
    assert_eq!(result.unwrap(), "()");
}

#[test]
fn test_serialize_node_with_identifier() {
    let pattern = Pattern::point(subject_with_id("hello"));
    let result = serialize_pattern(&pattern);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "(hello)");
}

#[test]
fn test_serialize_node_with_label() {
    let mut subject = subject_with_id("a");
    subject.labels.insert("Person".to_string());

    let pattern = Pattern::point(subject);
    let result = serialize_pattern(&pattern);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "(a:Person)");
}

#[test]
fn test_serialize_node_with_multiple_labels() {
    let mut subject = subject_with_id("a");
    subject.labels.insert("Person".to_string());
    subject.labels.insert("Employee".to_string());

    let pattern = Pattern::point(subject);
    let result = serialize_pattern(&pattern);
    assert!(result.is_ok());
    let output = result.unwrap();
    // Labels should be sorted alphabetically
    assert!(output.contains(":Employee"));
    assert!(output.contains(":Person"));
}

#[test]
fn test_serialize_node_with_properties() {
    let mut subject = subject_with_id("a");
    subject.properties.insert(
        "name".to_string(),
        pattern_core::Value::VString("Alice".to_string()),
    );
    subject
        .properties
        .insert("age".to_string(), pattern_core::Value::VInteger(30));

    let pattern = Pattern::point(subject);
    let result = serialize_pattern(&pattern);
    assert!(result.is_ok());
    let output = result.unwrap();
    eprintln!("Serialized output: {}", output);
    assert!(output.contains("name: \"Alice\""));
    assert!(output.contains("age: 30"));
}

#[test]
fn test_serialize_simple_relationship() {
    let left = Pattern::point(subject_with_id("a"));
    let right = Pattern::point(subject_with_id("b"));

    let pattern = Pattern::pattern(empty_subject(), vec![left, right]);
    let result = serialize_pattern(&pattern);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "(a)-->(b)");
}

#[test]
fn test_serialize_relationship_with_label() {
    let left = Pattern::point(subject_with_id("a"));
    let right = Pattern::point(subject_with_id("b"));

    let mut edge_subject = empty_subject();
    edge_subject.labels.insert("KNOWS".to_string());

    let pattern = Pattern::pattern(edge_subject, vec![left, right]);
    let result = serialize_pattern(&pattern);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "(a)-[:KNOWS]->(b)");
}

#[test]
fn test_serialize_subject_pattern_with_elements() {
    let elem1 = Pattern::point(subject_with_id("alice"));
    let elem2 = Pattern::point(subject_with_id("bob"));

    let pattern = Pattern::pattern(subject_with_id("team"), vec![elem1, elem2]);
    let result = serialize_pattern(&pattern);
    assert!(result.is_ok());
    // Per spec: 2-element patterns with atomic elements serialize as relationships
    // regardless of identifier, so this becomes a relationship with edge identifier
    assert_eq!(result.unwrap(), "(alice)-[team]->(bob)");
}

#[test]
fn test_serialize_nested_subject_pattern() {
    let leaf = Pattern::point(subject_with_id("leaf"));
    let inner = Pattern::pattern(subject_with_id("inner"), vec![leaf]);
    let outer = Pattern::pattern(subject_with_id("outer"), vec![inner]);

    let result = serialize_pattern(&outer);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "[outer | [inner | (leaf)]]");
}

#[test]
fn test_serialize_multiple_patterns() {
    let patterns = vec![
        Pattern::point(subject_with_id("a")),
        Pattern::point(subject_with_id("b")),
        Pattern::point(subject_with_id("c")),
    ];

    let result = serialize_patterns(&patterns);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "(a)\n(b)\n(c)");
}

#[test]
fn test_serialize_identifier_with_special_chars_needs_quoting() {
    let pattern = Pattern::point(subject_with_id("hello world"));
    let result = serialize_pattern(&pattern);
    assert!(result.is_ok());
    let output = result.unwrap();
    // Should be quoted because it contains a space
    assert!(output.contains("\"hello world\""));
}

#[test]
fn test_serialize_identifier_starting_with_digit_needs_quoting() {
    let pattern = Pattern::point(subject_with_id("42node"));
    let result = serialize_pattern(&pattern);
    assert!(result.is_ok());
    let output = result.unwrap();
    // Should be quoted because it starts with a digit
    assert!(output.contains("\"42node\""));
}

#[test]
fn test_round_trip_simple_node() {
    let original = "(hello)";
    let parsed = parse_gram_notation(original).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();
    let reparsed = parse_gram_notation(&serialized).unwrap();

    // Check structural equivalence
    assert_eq!(parsed[0].elements.len(), reparsed[0].elements.len());
    assert_eq!(parsed[0].value.identity.0, reparsed[0].value.identity.0);
}

#[test]
fn test_round_trip_relationship() {
    let original = "(a)-->(b)";
    let parsed = parse_gram_notation(original).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();
    let reparsed = parse_gram_notation(&serialized).unwrap();

    // Check structural equivalence
    assert_eq!(parsed[0].elements.len(), 2);
    assert_eq!(reparsed[0].elements.len(), 2);
}

#[test]
fn test_round_trip_subject_pattern() {
    let original = "[team | (alice), (bob)]";
    let parsed = parse_gram_notation(original).unwrap();
    let serialized = serialize_pattern(&parsed[0]).unwrap();
    let reparsed = parse_gram_notation(&serialized).unwrap();

    // Check structural equivalence
    assert_eq!(parsed[0].elements.len(), reparsed[0].elements.len());
    assert_eq!(parsed[0].value.identity.0, reparsed[0].value.identity.0);
}
