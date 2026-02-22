use pattern_core::{
    canonical_classifier, classify_by_shape, from_patterns, from_patterns_with_policy, GraphClass,
    GraphClassifier, Pattern, PatternGraph, ReconciliationPolicy, Subject, Symbol,
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

fn rel(r: &str, a: &str, b: &str) -> Pattern<Subject> {
    Pattern {
        value: Subject {
            identity: Symbol(r.to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        },
        elements: vec![node(a), node(b)],
    }
}

// Test 1: empty graph has all six maps with size 0
#[test]
fn empty_graph_has_zero_counts() {
    let g: PatternGraph<(), Subject> = PatternGraph::empty();
    assert_eq!(g.pg_nodes.len(), 0);
    assert_eq!(g.pg_relationships.len(), 0);
    assert_eq!(g.pg_walks.len(), 0);
    assert_eq!(g.pg_annotations.len(), 0);
    assert_eq!(g.pg_other.len(), 0);
    assert_eq!(g.pg_conflicts.len(), 0);
}

// Test 2: merge a node → pg_nodes size 1
#[test]
fn merge_node_appears_in_pg_nodes() {
    let classifier = canonical_classifier::<Subject>();
    let g = pattern_core::pg_merge(&classifier, node("a"), PatternGraph::empty());
    assert_eq!(g.pg_nodes.len(), 1);
    assert_eq!(g.pg_relationships.len(), 0);
}

// Test 3: merge relationship → pg_nodes size 2, pg_relationships size 1
#[test]
fn merge_relationship_inserts_endpoints_and_rel() {
    let classifier = canonical_classifier::<Subject>();
    let g0 = PatternGraph::empty();
    let g1 = pattern_core::pg_merge(&classifier, node("a"), g0);
    let g2 = pattern_core::pg_merge(&classifier, node("b"), g1);
    let g3 = pattern_core::pg_merge(&classifier, rel("r", "a", "b"), g2);
    assert_eq!(g3.pg_nodes.len(), 2);
    assert_eq!(g3.pg_relationships.len(), 1);
}

// Test 4: from_patterns with mixed list → correct counts
#[test]
fn from_patterns_builds_correct_counts() {
    let classifier = canonical_classifier::<Subject>();
    let patterns = vec![node("a"), node("b"), rel("r1", "a", "b")];
    let g = from_patterns(&classifier, patterns);
    assert_eq!(g.pg_nodes.len(), 2);
    assert_eq!(g.pg_relationships.len(), 1);
}

// Test 5: unrecognized pattern (3 node elements) → pg_other
#[test]
fn unrecognized_pattern_goes_to_pg_other() {
    let classifier = canonical_classifier::<Subject>();
    let weird = Pattern {
        value: Subject {
            identity: Symbol("w".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        },
        elements: vec![node("a"), node("b"), node("c")],
    };
    let g = from_patterns(&classifier, vec![node("a"), weird]);
    assert_eq!(g.pg_nodes.len(), 1);
    assert_eq!(g.pg_other.len(), 1);
}

// Test 6: duplicate identity with LastWriteWins → single entry
#[test]
fn duplicate_identity_last_write_wins() {
    let classifier = canonical_classifier::<Subject>();
    let g = from_patterns_with_policy(
        &classifier,
        &ReconciliationPolicy::LastWriteWins,
        vec![node("a"), node("a"), node("a")],
    );
    assert_eq!(g.pg_nodes.len(), 1);
}

// Test 7: from_patterns_with_policy FirstWriteWins → first value kept
#[test]
fn from_patterns_first_write_wins() {
    let classifier = canonical_classifier::<Subject>();
    let g = from_patterns_with_policy(
        &classifier,
        &ReconciliationPolicy::FirstWriteWins,
        vec![node("a"), node("a")],
    );
    assert_eq!(g.pg_nodes.len(), 1);
}

// Test 8: walk decomposition → pgWalks=1, pgRelationships=2, pgNodes=3
#[test]
fn walk_decomposition_stores_walk_rels_and_nodes() {
    let classifier = canonical_classifier::<Subject>();
    let r1 = rel("r1", "a", "b");
    let r2 = rel("r2", "b", "c");
    let walk_pat = Pattern {
        value: Subject {
            identity: Symbol("path".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        },
        elements: vec![r1, r2],
    };
    let g = pattern_core::pg_merge(&classifier, walk_pat, PatternGraph::empty());

    assert_eq!(g.pg_walks.len(), 1);
    assert_eq!(g.pg_relationships.len(), 2);
    assert_eq!(g.pg_nodes.len(), 3);

    assert!(g.pg_walks.contains_key(&Symbol("path".to_string())));
    assert!(g.pg_relationships.contains_key(&Symbol("r1".to_string())));
    assert!(g.pg_relationships.contains_key(&Symbol("r2".to_string())));
    assert!(g.pg_nodes.contains_key(&Symbol("a".to_string())));
    assert!(g.pg_nodes.contains_key(&Symbol("b".to_string())));
    assert!(g.pg_nodes.contains_key(&Symbol("c".to_string())));
}

// Test 9 (US3): custom classifier with typed tag → pg_other stores (DomainHyperedge, pattern)
#[derive(Debug, PartialEq)]
enum MyDomain {
    DomainHyperedge,
    DomainOther,
}

#[test]
fn custom_classifier_routes_to_pg_other_with_typed_tag() {
    let my_classifier = GraphClassifier::new(|p: &Pattern<Subject>| {
        if p.elements.len() > 2 && p.elements.iter().all(|e| e.elements.is_empty()) {
            GraphClass::GOther(MyDomain::DomainHyperedge)
        } else {
            match classify_by_shape(p) {
                GraphClass::GNode => GraphClass::GNode,
                GraphClass::GRelationship => GraphClass::GRelationship,
                GraphClass::GWalk => GraphClass::GWalk,
                GraphClass::GAnnotation => GraphClass::GAnnotation,
                GraphClass::GOther(()) => GraphClass::GOther(MyDomain::DomainOther),
            }
        }
    });

    let n1 = node("n1");
    let n2 = node("n2");
    let n3 = node("n3");
    let hyperedge = Pattern {
        value: Subject {
            identity: Symbol("hyper".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        },
        elements: vec![node("n1"), node("n2"), node("n3")],
    };

    let g = from_patterns(&my_classifier, vec![n1, n2, n3, hyperedge]);

    assert_eq!(g.pg_nodes.len(), 3);
    assert_eq!(g.pg_other.len(), 1);

    let (tag, _pat) = g
        .pg_other
        .get(&Symbol("hyper".to_string()))
        .expect("hyperedge should be in pg_other");
    assert_eq!(*tag, MyDomain::DomainHyperedge);
}
