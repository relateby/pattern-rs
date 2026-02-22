use pattern_core::{
    canonical_classifier, classify_by_shape, from_test_node, GraphClass, Pattern, Subject, Symbol,
};
use std::collections::{HashMap, HashSet};

fn node(s: &str) -> Pattern<Subject> {
    Pattern {
        value: Subject {
            identity: Symbol(s.to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        },
        elements: vec![],
    }
}

fn pat(s: &str, els: Vec<Pattern<Subject>>) -> Pattern<Subject> {
    Pattern {
        value: Subject {
            identity: Symbol(s.to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        },
        elements: els,
    }
}

// Test 1: atomic pattern → GNode
#[test]
fn atomic_pattern_is_gnode() {
    let n = node("a");
    assert_eq!(classify_by_shape(&n), GraphClass::GNode);
}

// Test 2: 1-element pattern → GAnnotation
#[test]
fn one_element_pattern_is_gannotation() {
    let n1 = node("b");
    let anno = pat("a", vec![n1]);
    assert_eq!(classify_by_shape(&anno), GraphClass::GAnnotation);
}

// Test 3: 2 node elements → GRelationship
#[test]
fn two_node_elements_is_grelationship() {
    let n1 = node("a");
    let n2 = node("b");
    let rel = pat("r", vec![n1, n2]);
    assert_eq!(classify_by_shape(&rel), GraphClass::GRelationship);
}

// Test 4: direction-agnostic chaining walk → GWalk
// r1=[A,B], r2=[B,C], r3=[D,C] — r3 connects via C (reverse direction)
#[test]
fn chaining_relationships_form_gwalk() {
    let na = node("A");
    let nb = node("B");
    let nc = node("C");
    let nd = node("D");

    let r1 = pat("r1", vec![na, nb]);
    let r2 = pat("r2", vec![node("B"), nc]);
    let r3 = pat("r3", vec![nd, node("C")]);

    let w = pat("w", vec![r1, r2, r3]);
    assert_eq!(classify_by_shape(&w), GraphClass::GWalk);
}

// Test 5: star pattern (shared center, not end-to-end) → GOther
#[test]
fn star_pattern_is_gother() {
    let r1 = pat("r1", vec![node("A"), node("B")]);
    let r2 = pat("r2", vec![node("A"), node("C")]);
    let r3 = pat("r3", vec![node("A"), node("D")]);

    let star = pat("star", vec![r1, r2, r3]);
    assert_eq!(classify_by_shape(&star), GraphClass::GOther(()));
}

// Test 6: relationship containing non-node element → GOther
#[test]
fn relationship_with_non_node_element_is_gother() {
    let n1 = node("a");
    let not_node = pat("b", vec![node("c")]);
    let bad_rel = pat("r", vec![n1, not_node]);
    assert_eq!(classify_by_shape(&bad_rel), GraphClass::GOther(()));
}

// Test 7: walk containing a non-relationship (a bare node) → GOther
#[test]
fn walk_with_non_relationship_is_gother() {
    let n1 = node("a");
    let n2 = node("b");
    let rel = pat("r", vec![n1.clone(), n2]);
    let bad_walk = pat("w", vec![rel, n1]);
    assert_eq!(classify_by_shape(&bad_walk), GraphClass::GOther(()));
}

// Test 8: canonical_classifier produces same result as classify_by_shape
#[test]
fn canonical_classifier_matches_classify_by_shape() {
    let n = node("a");
    let classifier = canonical_classifier::<Subject>();
    assert_eq!((classifier.classify)(&n), GraphClass::GNode);
    assert_eq!((classifier.classify)(&n), classify_by_shape(&n));
}

// from_test_node: predicate wrapping
#[test]
fn from_test_node_classifies_matching_as_gnode() {
    let classifier = from_test_node::<Subject, _>(|p| p.elements.is_empty());
    let n = node("a");
    assert_eq!((classifier.classify)(&n), GraphClass::GNode);
}

#[test]
fn from_test_node_classifies_non_matching_as_gother() {
    let classifier = from_test_node::<Subject, _>(|p| p.elements.is_empty());
    let with_elements = pat("x", vec![node("y")]);
    assert_eq!(
        (classifier.classify)(&with_elements),
        GraphClass::GOther(())
    );
}
