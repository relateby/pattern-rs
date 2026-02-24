//! Tests for GraphView construction and materialize round-trip.
//!
//! Covers T029: view construction, materialize round-trip (US1).

use std::collections::{HashMap, HashSet};

use pattern_core::{
    canonical_classifier, filter_graph, fold_graph, from_pattern_graph, from_patterns,
    map_all_graph, materialize, GraphClass, GraphClassifier, Pattern, ReconciliationPolicy,
    Subject, Substitution, Symbol,
};

fn classifier() -> GraphClassifier<(), Subject> {
    canonical_classifier::<Subject>()
}

fn lww() -> ReconciliationPolicy<pattern_core::SubjectMergeStrategy> {
    ReconciliationPolicy::LastWriteWins
}

// ============================================================================
// Helpers
// ============================================================================

fn subj(id: &str) -> Subject {
    Subject {
        identity: Symbol(id.to_string()),
        labels: HashSet::new(),
        properties: HashMap::new(),
    }
}

fn subj_labeled(id: &str, label: &str) -> Subject {
    let mut labels = HashSet::new();
    labels.insert(label.to_string());
    Subject {
        identity: Symbol(id.to_string()),
        labels,
        properties: HashMap::new(),
    }
}

fn node(id: &str) -> Pattern<Subject> {
    Pattern::point(subj(id))
}

fn node_labeled(id: &str, label: &str) -> Pattern<Subject> {
    Pattern::point(subj_labeled(id, label))
}

fn rel(id: &str, src: &str, tgt: &str) -> Pattern<Subject> {
    Pattern {
        value: subj(id),
        elements: vec![node(src), node(tgt)],
    }
}

// ============================================================================
// T029: View construction
// ============================================================================

#[test]
fn view_from_single_node_graph() {
    let classifier = classifier();
    let graph = from_patterns(&classifier, vec![node("a")]);

    let view = from_pattern_graph(&classifier, &graph);

    assert_eq!(view.view_elements.len(), 1);
    assert!(matches!(view.view_elements[0].0, GraphClass::GNode));
}

#[test]
fn view_from_graph_with_node_and_rel() {
    let classifier = classifier();
    let graph = from_patterns(&classifier, vec![rel("r1", "a", "b")]);

    let view = from_pattern_graph(&classifier, &graph);

    // rel("r1","a","b") inserts nodes a, b and relationship r1
    let node_count = view
        .view_elements
        .iter()
        .filter(|(cls, _)| matches!(cls, GraphClass::GNode))
        .count();
    let rel_count = view
        .view_elements
        .iter()
        .filter(|(cls, _)| matches!(cls, GraphClass::GRelationship))
        .count();

    assert_eq!(node_count, 2, "expected 2 nodes");
    assert_eq!(rel_count, 1, "expected 1 relationship");
}

// ============================================================================
// T029: materialize round-trip
// ============================================================================

#[test]
fn materialize_round_trip_nodes_only() {
    let classifier = classifier();
    let policy = lww();
    let graph = from_patterns(&classifier, vec![node("a"), node("b"), node("c")]);

    let view = from_pattern_graph(&classifier, &graph);
    let back = materialize(&classifier, &policy, view);

    assert_eq!(back.pg_nodes.len(), 3);
    assert!(back.pg_nodes.contains_key(&Symbol("a".to_string())));
    assert!(back.pg_nodes.contains_key(&Symbol("b".to_string())));
    assert!(back.pg_nodes.contains_key(&Symbol("c".to_string())));
}

#[test]
fn materialize_round_trip_with_relationship() {
    let classifier = classifier();
    let policy = lww();
    let graph = from_patterns(&classifier, vec![rel("r1", "a", "b")]);

    let view = from_pattern_graph(&classifier, &graph);
    let back = materialize(&classifier, &policy, view);

    assert_eq!(back.pg_relationships.len(), 1);
    assert!(back
        .pg_relationships
        .contains_key(&Symbol("r1".to_string())));
    assert_eq!(back.pg_nodes.len(), 2);
}

// ============================================================================
// map_all_graph identity round-trip
// ============================================================================

#[test]
fn map_all_identity_is_round_trip() {
    let classifier = classifier();
    let policy = lww();
    let graph = from_patterns(&classifier, vec![node("a"), node("b"), rel("r1", "a", "b")]);

    let view = from_pattern_graph(&classifier, &graph);
    let view = map_all_graph(|p| p, view);
    let back = materialize(&classifier, &policy, view);

    assert_eq!(back.pg_nodes.len(), 2);
    assert_eq!(back.pg_relationships.len(), 1);
}

// ============================================================================
// filter_graph
// ============================================================================

#[test]
fn filter_graph_keeps_only_nodes() {
    let classifier = classifier();
    let policy = lww();
    let graph = from_patterns(&classifier, vec![rel("r1", "a", "b")]);

    let view = from_pattern_graph(&classifier, &graph);
    let view = filter_graph(
        &classifier,
        |cls, _p| matches!(cls, GraphClass::GNode),
        Substitution::NoSubstitution,
        view,
    );
    let back = materialize(&classifier, &policy, view);

    assert_eq!(back.pg_nodes.len(), 2);
    assert_eq!(back.pg_relationships.len(), 0);
}

// ============================================================================
// fold_graph
// ============================================================================

#[test]
fn fold_graph_counts_elements() {
    use std::collections::HashMap;

    let classifier = classifier();
    let graph = from_patterns(&classifier, vec![node("a"), node("b"), rel("r1", "a", "b")]);

    let view = from_pattern_graph(&classifier, &graph);

    let counts: HashMap<String, usize> = fold_graph(
        |mut acc, cls, _p| {
            let key = format!("{:?}", cls);
            *acc.entry(key).or_insert(0) += 1;
            acc
        },
        HashMap::new(),
        &view,
    );

    let node_count: usize = counts
        .iter()
        .filter(|(k, _)| k.contains("GNode"))
        .map(|(_, v)| v)
        .sum();
    let rel_count: usize = counts
        .iter()
        .filter(|(k, _)| k.contains("GRelationship"))
        .map(|(_, v)| v)
        .sum();

    assert_eq!(node_count, 2);
    assert_eq!(rel_count, 1);
}

// ============================================================================
// filter_graph with ReplaceWith substitution
// ============================================================================

#[test]
fn filter_graph_replace_with_filler() {
    let classifier = classifier();
    let policy = lww();
    let filler = node("_filler");

    let graph = from_patterns(&classifier, vec![node("a"), node("b"), node("c")]);
    let view = from_pattern_graph(&classifier, &graph);

    // Keep only "a", replace others with filler
    let view = filter_graph(
        &classifier,
        |_cls, p| p.value.identity.0 == "a",
        Substitution::ReplaceWith(filler),
        view,
    );

    // Should have 1 original + 2 fillers = 3 elements
    assert_eq!(view.view_elements.len(), 3);
}
