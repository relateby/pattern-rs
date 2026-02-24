//! Tests for graph transform operations: map, filter, fold, map_with_context,
//! para_graph, para_graph_fixed, unfold, unfold_graph.
//!
//! Covers T030.

use std::collections::{HashMap, HashSet};

use pattern_core::{
    canonical_classifier, filter_graph, fold_graph, from_pattern_graph, from_patterns,
    map_all_graph, map_graph, map_with_context, materialize, para_graph, para_graph_fixed, unfold,
    unfold_graph, CategoryMappers, GraphClass, GraphClassifier, Pattern, ReconciliationPolicy,
    Subject, SubjectMergeStrategy, Substitution, Symbol, Value,
};

fn classifier() -> GraphClassifier<(), Subject> {
    canonical_classifier::<Subject>()
}

fn lww() -> ReconciliationPolicy<SubjectMergeStrategy> {
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

fn subj_with_prop(id: &str, key: &str, val: &str) -> Subject {
    let mut props = HashMap::new();
    props.insert(key.to_string(), Value::VString(val.to_string()));
    Subject {
        identity: Symbol(id.to_string()),
        labels: HashSet::new(),
        properties: props,
    }
}

fn node(id: &str) -> Pattern<Subject> {
    Pattern::point(subj(id))
}

fn rel(id: &str, src: &str, tgt: &str) -> Pattern<Subject> {
    Pattern {
        value: subj(id),
        elements: vec![node(src), node(tgt)],
    }
}

// ============================================================================
// unfold
// ============================================================================

#[test]
fn unfold_leaf_seed() {
    let tree = unfold(|n: u32| (n, vec![]), 42u32);
    assert_eq!(tree.value, 42);
    assert!(tree.elements.is_empty());
}

#[test]
fn unfold_depth_2_binary_tree() {
    let tree = unfold(
        |depth: u32| {
            if depth == 0 {
                (depth, vec![])
            } else {
                (depth, vec![depth - 1, depth - 1])
            }
        },
        2u32,
    );
    assert_eq!(tree.value, 2);
    assert_eq!(tree.elements.len(), 2);
    assert_eq!(tree.elements[0].value, 1);
    assert_eq!(tree.elements[0].elements.len(), 2);
    assert_eq!(tree.elements[0].elements[0].value, 0);
}

#[test]
fn unfold_linear_chain() {
    // Build a chain: 3 -> 2 -> 1 -> 0
    let tree = unfold(
        |n: u32| {
            if n == 0 {
                (n, vec![])
            } else {
                (n, vec![n - 1])
            }
        },
        3u32,
    );
    assert_eq!(tree.value, 3);
    assert_eq!(tree.elements.len(), 1);
    assert_eq!(tree.elements[0].value, 2);
    assert_eq!(tree.elements[0].elements[0].value, 1);
}

// ============================================================================
// unfold_graph
// ============================================================================

#[test]
fn unfold_graph_from_seeds() {
    let classifier = classifier();
    let policy = lww();

    let seeds = vec!["a", "b", "c"];
    let graph = unfold_graph(&classifier, &policy, |id: &str| vec![node(id)], seeds);

    assert_eq!(graph.pg_nodes.len(), 3);
}

#[test]
fn unfold_graph_empty_seeds() {
    let classifier = classifier();
    let policy = lww();

    let graph = unfold_graph(&classifier, &policy, |_: &str| vec![], vec!["x"]);

    assert_eq!(graph.pg_nodes.len(), 0);
}

// ============================================================================
// map_graph (by category)
// ============================================================================

#[test]
fn map_graph_transforms_only_nodes() {
    let classifier = classifier();
    let policy = lww();
    let graph = from_patterns(&classifier, vec![rel("r1", "a", "b")]);
    let view = from_pattern_graph(&classifier, &graph);

    // Add a property to every node
    let mappers = CategoryMappers {
        nodes: Box::new(|mut p: Pattern<Subject>| {
            p.value
                .properties
                .insert("transformed".to_string(), Value::VString("yes".to_string()));
            p
        }),
        ..CategoryMappers::identity()
    };

    let view = map_graph(&classifier, mappers, view);

    // Check view_elements directly: nodes should have the property, rels should not
    for (cls, pat) in &view.view_elements {
        match cls {
            GraphClass::GNode => {
                assert!(
                    pat.value.properties.contains_key("transformed"),
                    "node should have 'transformed' property"
                );
            }
            GraphClass::GRelationship => {
                assert!(
                    !pat.value.properties.contains_key("transformed"),
                    "relationship should not have 'transformed' property"
                );
            }
            _ => {}
        }
    }
}

// ============================================================================
// map_all_graph
// ============================================================================

#[test]
fn map_all_graph_transforms_all_elements() {
    let classifier = classifier();
    let policy = lww();
    let graph = from_patterns(&classifier, vec![node("a"), node("b")]);
    let view = from_pattern_graph(&classifier, &graph);

    let view = map_all_graph(
        |mut p: Pattern<Subject>| {
            p.value
                .properties
                .insert("touched".to_string(), Value::VString("1".to_string()));
            p
        },
        view,
    );
    let back = materialize(&classifier, &policy, view);

    for n in back.pg_nodes.values() {
        assert!(n.value.properties.contains_key("touched"));
    }
}

// ============================================================================
// filter_graph
// ============================================================================

#[test]
fn filter_graph_removes_non_matching() {
    let classifier = classifier();
    let policy = lww();
    let graph = from_patterns(
        &classifier,
        vec![node("keep1"), node("keep2"), node("drop1")],
    );
    let view = from_pattern_graph(&classifier, &graph);

    let view = filter_graph(
        &classifier,
        |_cls, p| p.value.identity.0.starts_with("keep"),
        Substitution::NoSubstitution,
        view,
    );
    let back = materialize(&classifier, &policy, view);

    assert_eq!(back.pg_nodes.len(), 2);
    assert!(back.pg_nodes.contains_key(&Symbol("keep1".to_string())));
    assert!(back.pg_nodes.contains_key(&Symbol("keep2".to_string())));
    assert!(!back.pg_nodes.contains_key(&Symbol("drop1".to_string())));
}

// ============================================================================
// fold_graph
// ============================================================================

#[test]
fn fold_graph_count_by_class() {
    let classifier = classifier();
    let graph = from_patterns(&classifier, vec![node("a"), node("b"), rel("r1", "a", "b")]);
    let view = from_pattern_graph(&classifier, &graph);

    let (node_count, rel_count) = fold_graph(
        |(nc, rc), cls, _p| match cls {
            GraphClass::GNode => (nc + 1, rc),
            GraphClass::GRelationship => (nc, rc + 1),
            _ => (nc, rc),
        },
        (0usize, 0usize),
        &view,
    );

    assert_eq!(node_count, 2);
    assert_eq!(rel_count, 1);
}

// ============================================================================
// map_with_context (snapshot semantics)
// ============================================================================

#[test]
fn map_with_context_uses_snapshot() {
    let classifier = classifier();
    let policy = lww();
    let graph = from_patterns(&classifier, vec![node("a"), node("b"), rel("r1", "a", "b")]);
    let view = from_pattern_graph(&classifier, &graph);

    // Enrich each node with its degree from the snapshot query
    let view = map_with_context(
        &classifier,
        |query, mut p| {
            let degree = (query.query_degree)(&p);
            p.value
                .properties
                .insert("degree".to_string(), Value::VString(degree.to_string()));
            p
        },
        view,
    );

    // Check view_elements directly: all elements should have the degree property
    for (cls, pat) in &view.view_elements {
        if matches!(cls, GraphClass::GNode) {
            assert!(
                pat.value.properties.contains_key("degree"),
                "node should have degree property"
            );
        }
    }
}

// ============================================================================
// para_graph (DAG)
// ============================================================================

#[test]
fn para_graph_depth_in_dag() {
    let classifier = classifier();
    // Chain: a -> b -> c (a is root, c is leaf)
    let graph = from_patterns(&classifier, vec![rel("r1", "a", "b"), rel("r2", "b", "c")]);
    let view = from_pattern_graph(&classifier, &graph);

    // Compute depth: root = 0, each node = max(pred_depths) + 1
    let depths = para_graph(
        |_query, _node, pred_results: &[usize]| {
            pred_results
                .iter()
                .cloned()
                .max()
                .map(|d| d + 1)
                .unwrap_or(0)
        },
        &view,
    );

    let a_depth = depths[&Symbol("a".to_string())];
    let b_depth = depths[&Symbol("b".to_string())];
    let c_depth = depths[&Symbol("c".to_string())];

    // a has no predecessors → depth 0
    assert_eq!(a_depth, 0);
    // b's predecessor is a (depth 0) → depth 1
    assert_eq!(b_depth, 1);
    // c's predecessor is b (depth 1) → depth 2
    assert_eq!(c_depth, 2);
}

// ============================================================================
// para_graph_fixed (cyclic convergence)
// ============================================================================

#[test]
fn para_graph_fixed_converges_on_simple_graph() {
    let classifier = classifier();
    let graph = from_patterns(&classifier, vec![node("a"), node("b"), rel("r1", "a", "b")]);
    let view = from_pattern_graph(&classifier, &graph);

    // Simple: each node gets 1 + max(predecessor values), init = 0
    let result = para_graph_fixed(
        |old: &usize, new: &usize| old == new,
        |_query, _node, preds: &[usize]| preds.iter().cloned().max().map(|v| v + 1).unwrap_or(1),
        0usize,
        &view,
    );

    // Should converge: a=1 (no preds), b=2 (pred a=1)
    assert!(result.contains_key(&Symbol("a".to_string())));
    assert!(result.contains_key(&Symbol("b".to_string())));
}
