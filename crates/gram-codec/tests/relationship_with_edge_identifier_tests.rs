/// Tests for relationship serialization with edge identifiers
///
/// These tests verify that patterns with identifiers and 2 atomic elements
/// correctly serialize as relationship notation, not subject pattern notation.
///
/// Bug fix: Previously, is_relationship_pattern() required empty identifier,
/// but the spec says relationship notation should be used whenever both
/// elements are atomic, regardless of identifier/labels/properties.
use gram_codec::{serialize_pattern, Pattern, Subject};
use pattern_core::subject::Symbol;
use std::collections::{HashMap, HashSet};

#[test]
fn test_relationship_with_edge_labels() {
    // Build a pattern with edge labels (empty identifier)
    let mut labels = HashSet::new();
    labels.insert("KNOWS".to_string());

    let alice = Pattern {
        value: Subject {
            identity: Symbol("alice".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        },
        elements: vec![],
    };

    let bob = Pattern {
        value: Subject {
            identity: Symbol("bob".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        },
        elements: vec![],
    };

    let relationship = Pattern {
        value: Subject {
            identity: Symbol("".to_string()),
            labels: labels.clone(),
            properties: HashMap::new(),
        },
        elements: vec![alice.clone(), bob.clone()],
    };

    let serialized = serialize_pattern(&relationship).unwrap();
    assert!(serialized.contains("->"), "Should be relationship notation");
    assert!(serialized.contains("[:KNOWS]"), "Should have edge label");
    assert_eq!(serialized, "(alice)-[:KNOWS]->(bob)");
}

#[test]
fn test_relationship_with_edge_identifier() {
    // Build a pattern with edge identifier (no labels)
    // This is the bug fix case: should serialize as relationship despite having identifier
    let alice = Pattern {
        value: Subject {
            identity: Symbol("alice".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        },
        elements: vec![],
    };

    let bob = Pattern {
        value: Subject {
            identity: Symbol("bob".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        },
        elements: vec![],
    };

    let relationship = Pattern {
        value: Subject {
            identity: Symbol("KNOWS".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        },
        elements: vec![alice.clone(), bob.clone()],
    };

    let serialized = serialize_pattern(&relationship).unwrap();
    assert!(serialized.contains("->"), "Should be relationship notation");
    assert!(
        serialized.contains("[KNOWS]"),
        "Should have edge identifier"
    );
    assert_eq!(serialized, "(alice)-[KNOWS]->(bob)");
}

#[test]
fn test_relationship_with_edge_identifier_and_labels() {
    // Build a pattern with both edge identifier and labels
    let mut labels = HashSet::new();
    labels.insert("KNOWS".to_string());

    let alice = Pattern {
        value: Subject {
            identity: Symbol("alice".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        },
        elements: vec![],
    };

    let bob = Pattern {
        value: Subject {
            identity: Symbol("bob".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        },
        elements: vec![],
    };

    let relationship = Pattern {
        value: Subject {
            identity: Symbol("rel1".to_string()),
            labels,
            properties: HashMap::new(),
        },
        elements: vec![alice, bob],
    };

    let serialized = serialize_pattern(&relationship).unwrap();
    assert!(serialized.contains("->"), "Should be relationship notation");
    assert!(
        serialized.contains("rel1") && serialized.contains("KNOWS"),
        "Should have both identifier and label, got: {}",
        serialized
    );
    assert_eq!(serialized, "(alice)-[rel1:KNOWS]->(bob)");
}

#[test]
fn test_simple_relationship_without_edge_data() {
    // Pattern with no edge identifier, labels, or properties
    let alice = Pattern {
        value: Subject {
            identity: Symbol("alice".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        },
        elements: vec![],
    };

    let bob = Pattern {
        value: Subject {
            identity: Symbol("bob".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        },
        elements: vec![],
    };

    let relationship = Pattern {
        value: Subject {
            identity: Symbol("".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        },
        elements: vec![alice, bob],
    };

    let serialized = serialize_pattern(&relationship).unwrap();
    assert_eq!(
        serialized, "(alice)-->(bob)",
        "Should be simple relationship"
    );
}
