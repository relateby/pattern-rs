//! Integration tests for StandardGraph and SubjectBuilder.

use pattern_core::graph::StandardGraph;
use pattern_core::subject::{Subject, Symbol, Value};
use pattern_core::Pattern;

// ============================================================================
// SubjectBuilder tests (US2)
// ============================================================================

#[test]
fn subject_builder_basic() {
    let subject = Subject::build("alice")
        .label("Person")
        .property("name", "Alice")
        .done();

    assert_eq!(subject.identity, Symbol("alice".to_string()));
    assert!(subject.labels.contains("Person"));
    assert_eq!(
        subject.properties.get("name"),
        Some(&Value::VString("Alice".to_string()))
    );
}

#[test]
fn subject_builder_multiple_labels_and_properties() {
    let subject = Subject::build("alice")
        .label("Person")
        .label("Employee")
        .property("name", "Alice Smith")
        .property("age", 30i64)
        .property("active", true)
        .done();

    assert_eq!(subject.identity.0, "alice");
    assert_eq!(subject.labels.len(), 2);
    assert!(subject.labels.contains("Person"));
    assert!(subject.labels.contains("Employee"));
    assert_eq!(subject.properties.len(), 3);
    assert_eq!(subject.properties.get("age"), Some(&Value::VInteger(30)));
    assert_eq!(
        subject.properties.get("active"),
        Some(&Value::VBoolean(true))
    );
}

#[test]
fn subject_builder_into_subject() {
    let subject: Subject = Subject::build("bob").label("Person").into();
    assert_eq!(subject.identity.0, "bob");
    assert!(subject.labels.contains("Person"));
}

#[test]
fn subject_builder_empty() {
    let subject = Subject::build("x").done();
    assert_eq!(subject.identity.0, "x");
    assert!(subject.labels.is_empty());
    assert!(subject.properties.is_empty());
}

// ============================================================================
// StandardGraph construction tests (US1)
// ============================================================================

#[test]
fn new_graph_is_empty() {
    let g = StandardGraph::new();
    assert!(g.is_empty());
    assert_eq!(g.node_count(), 0);
    assert_eq!(g.relationship_count(), 0);
    assert_eq!(g.walk_count(), 0);
    assert_eq!(g.annotation_count(), 0);
    assert!(!g.has_conflicts());
}

#[test]
fn add_nodes() {
    let mut g = StandardGraph::new();
    g.add_node(
        Subject::build("alice")
            .label("Person")
            .property("name", "Alice")
            .done(),
    );
    g.add_node(
        Subject::build("bob")
            .label("Person")
            .property("name", "Bob")
            .done(),
    );

    assert_eq!(g.node_count(), 2);
    assert!(!g.is_empty());

    let alice = g.node(&Symbol("alice".to_string())).unwrap();
    assert_eq!(alice.value.identity.0, "alice");
    assert!(alice.value.labels.contains("Person"));
}

#[test]
fn add_relationship_creates_placeholder_nodes() {
    let mut g = StandardGraph::new();
    g.add_relationship(
        Subject::build("r1").label("KNOWS").done(),
        &Subject::from_id("alice"),
        &Subject::from_id("bob"),
    );

    assert_eq!(g.relationship_count(), 1);
    // Placeholder nodes should be auto-created
    assert_eq!(g.node_count(), 2);
    assert!(g.node(&"alice".into()).is_some());
    assert!(g.node(&"bob".into()).is_some());
}

#[test]
fn add_relationship_uses_existing_nodes() {
    let mut g = StandardGraph::new();
    let alice = Subject::build("alice")
        .label("Person")
        .property("name", "Alice")
        .done();
    let bob = Subject::build("bob").label("Person").done();
    g.add_node(alice.clone());
    g.add_node(bob.clone());
    g.add_relationship(Subject::build("r1").label("KNOWS").done(), &alice, &bob);

    assert_eq!(g.node_count(), 2);
    assert_eq!(g.relationship_count(), 1);

    // The relationship's source should have the full alice node data
    let rel = g.relationship(&"r1".into()).unwrap();
    assert!(rel.elements[0].value.labels.contains("Person"));
    assert_eq!(
        rel.elements[0].value.properties.get("name"),
        Some(&Value::VString("Alice".to_string()))
    );
}

#[test]
fn add_node_last_write_wins() {
    let mut g = StandardGraph::new();
    g.add_node(Subject::build("alice").label("Person").done());
    g.add_node(Subject::build("alice").label("Employee").done());

    assert_eq!(g.node_count(), 1);
    let alice = g.node(&"alice".into()).unwrap();
    // Last write wins — should have Employee label, not Person
    assert!(alice.value.labels.contains("Employee"));
    assert!(!alice.value.labels.contains("Person"));
}

#[test]
fn add_walk() {
    let mut g = StandardGraph::new();
    g.add_node(Subject::build("a").done());
    g.add_node(Subject::build("b").done());
    g.add_node(Subject::build("c").done());
    g.add_relationship(
        Subject::build("r1").done(),
        &Subject::from_id("a"),
        &Subject::from_id("b"),
    );
    g.add_relationship(
        Subject::build("r2").done(),
        &Subject::from_id("b"),
        &Subject::from_id("c"),
    );
    g.add_walk(
        Subject::build("w1").label("Path").done(),
        &[Subject::from_id("r1"), Subject::from_id("r2")],
    );

    assert_eq!(g.walk_count(), 1);
    let walk = g.walk(&"w1".into()).unwrap();
    assert!(walk.value.labels.contains("Path"));
}

#[test]
fn add_annotation() {
    let mut g = StandardGraph::new();
    g.add_node(Subject::build("alice").label("Person").done());
    g.add_annotation(
        Subject::build("a1").label("Note").done(),
        &Subject::from_id("alice"),
    );

    assert_eq!(g.annotation_count(), 1);
    let ann = g.annotation(&"a1".into()).unwrap();
    assert!(ann.value.labels.contains("Note"));
    assert_eq!(ann.elements.len(), 1);
}

#[test]
fn element_access_returns_none_for_missing() {
    let g = StandardGraph::new();
    assert!(g.node(&"missing".into()).is_none());
    assert!(g.relationship(&"missing".into()).is_none());
    assert!(g.walk(&"missing".into()).is_none());
    assert!(g.annotation(&"missing".into()).is_none());
}

// ============================================================================
// Pattern ingestion tests (US3)
// ============================================================================

#[test]
fn add_pattern_classifies_node() {
    let mut g = StandardGraph::new();
    let node_pattern = Pattern::point(Subject::build("alice").label("Person").done());
    g.add_pattern(node_pattern);

    assert_eq!(g.node_count(), 1);
    assert!(g.node(&"alice".into()).is_some());
}

#[test]
fn add_pattern_classifies_relationship() {
    let mut g = StandardGraph::new();
    let rel = Pattern::pattern(
        Subject::build("r1").label("KNOWS").done(),
        vec![
            Pattern::point(Subject::build("alice").done()),
            Pattern::point(Subject::build("bob").done()),
        ],
    );
    g.add_pattern(rel);

    assert_eq!(g.relationship_count(), 1);
    // Endpoint nodes should be merged into nodes bucket
    assert!(g.node_count() >= 2);
}

#[test]
fn from_patterns_constructor() {
    let patterns = vec![
        Pattern::point(Subject::build("alice").label("Person").done()),
        Pattern::point(Subject::build("bob").label("Person").done()),
    ];
    let g = StandardGraph::from_patterns(patterns);

    assert_eq!(g.node_count(), 2);
}

#[test]
fn from_pattern_graph_wraps_directly() {
    let mut g1 = StandardGraph::new();
    g1.add_node(Subject::build("alice").done());
    g1.add_node(Subject::build("bob").done());

    let pg = g1.into_pattern_graph();
    let g2 = StandardGraph::from_pattern_graph(pg);

    assert_eq!(g2.node_count(), 2);
}

// ============================================================================
// Iterator tests (US4)
// ============================================================================

#[test]
fn iterators_visit_all_elements() {
    let mut g = StandardGraph::new();
    g.add_node(Subject::build("a").done());
    g.add_node(Subject::build("b").done());
    g.add_node(Subject::build("c").done());
    g.add_relationship(
        Subject::build("r1").done(),
        &Subject::from_id("a"),
        &Subject::from_id("b"),
    );
    g.add_relationship(
        Subject::build("r2").done(),
        &Subject::from_id("b"),
        &Subject::from_id("c"),
    );

    assert_eq!(g.nodes().count(), 3);
    assert_eq!(g.relationships().count(), 2);
}

// ============================================================================
// Graph-native query tests (US4)
// ============================================================================

#[test]
fn source_and_target() {
    let mut g = StandardGraph::new();
    g.add_node(Subject::build("alice").done());
    g.add_node(Subject::build("bob").done());
    g.add_relationship(
        Subject::build("r1").done(),
        &Subject::from_id("alice"),
        &Subject::from_id("bob"),
    );

    let src = g.source(&"r1".into()).unwrap();
    assert_eq!(src.value.identity.0, "alice");

    let tgt = g.target(&"r1".into()).unwrap();
    assert_eq!(tgt.value.identity.0, "bob");
}

#[test]
fn source_target_returns_none_for_missing() {
    let g = StandardGraph::new();
    assert!(g.source(&"missing".into()).is_none());
    assert!(g.target(&"missing".into()).is_none());
}

#[test]
fn neighbors_bidirectional() {
    let mut g = StandardGraph::new();
    g.add_node(Subject::build("a").done());
    g.add_node(Subject::build("b").done());
    g.add_node(Subject::build("c").done());
    g.add_relationship(
        Subject::build("r1").done(),
        &Subject::from_id("a"),
        &Subject::from_id("b"),
    );
    g.add_relationship(
        Subject::build("r2").done(),
        &Subject::from_id("c"),
        &Subject::from_id("b"),
    );

    // b has neighbors: a (via r1 where b is target) and c (via r2 where b is target)
    let neighbors = g.neighbors(&"b".into());
    assert_eq!(neighbors.len(), 2);

    let neighbor_ids: std::collections::HashSet<&str> = neighbors
        .iter()
        .map(|p| p.value.identity.0.as_str())
        .collect();
    assert!(neighbor_ids.contains("a"));
    assert!(neighbor_ids.contains("c"));
}

#[test]
fn degree() {
    let mut g = StandardGraph::new();
    g.add_node(Subject::build("a").done());
    g.add_node(Subject::build("b").done());
    g.add_node(Subject::build("c").done());
    g.add_relationship(
        Subject::build("r1").done(),
        &Subject::from_id("a"),
        &Subject::from_id("b"),
    );
    g.add_relationship(
        Subject::build("r2").done(),
        &Subject::from_id("b"),
        &Subject::from_id("c"),
    );

    assert_eq!(g.degree(&"a".into()), 1);
    assert_eq!(g.degree(&"b".into()), 2);
    assert_eq!(g.degree(&"c".into()), 1);
    assert_eq!(g.degree(&"missing".into()), 0);
}

// ============================================================================
// Escape hatch tests (US5)
// ============================================================================

#[test]
fn as_pattern_graph() {
    let mut g = StandardGraph::new();
    g.add_node(Subject::build("alice").done());

    let pg = g.as_pattern_graph();
    assert_eq!(pg.pg_nodes.len(), 1);
}

#[test]
fn into_pattern_graph() {
    let mut g = StandardGraph::new();
    g.add_node(Subject::build("alice").done());
    g.add_node(Subject::build("bob").done());

    let pg = g.into_pattern_graph();
    assert_eq!(pg.pg_nodes.len(), 2);
}

#[test]
fn as_query() {
    let mut g = StandardGraph::new();
    g.add_node(Subject::build("alice").done());
    g.add_node(Subject::build("bob").done());
    g.add_relationship(
        Subject::build("r1").done(),
        &Subject::from_id("alice"),
        &Subject::from_id("bob"),
    );

    let query = g.as_query();
    let nodes = (query.query_nodes)();
    assert_eq!(nodes.len(), 2);
    let rels = (query.query_relationships)();
    assert_eq!(rels.len(), 1);
}

#[test]
fn as_snapshot() {
    let mut g = StandardGraph::new();
    g.add_node(Subject::build("alice").done());
    g.add_node(Subject::build("bob").done());
    g.add_relationship(
        Subject::build("r1").done(),
        &Subject::from_id("alice"),
        &Subject::from_id("bob"),
    );

    let snapshot = g.as_snapshot();
    // 2 nodes + 1 relationship = 3 elements
    assert_eq!(snapshot.view_elements.len(), 3);
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn default_creates_empty_graph() {
    let g = StandardGraph::default();
    assert!(g.is_empty());
}

#[test]
fn chaining_add_methods() {
    let mut g = StandardGraph::new();
    g.add_node(Subject::build("a").done())
        .add_node(Subject::build("b").done())
        .add_relationship(
            Subject::build("r1").done(),
            &Subject::from_id("a"),
            &Subject::from_id("b"),
        );

    assert_eq!(g.node_count(), 2);
    assert_eq!(g.relationship_count(), 1);
}

// ============================================================================
// Scale validation (T018b)
// ============================================================================

#[test]
fn scale_1000_nodes_5000_relationships() {
    let mut g = StandardGraph::new();

    // Add 1000 nodes
    for i in 0..1000 {
        g.add_node(Subject::build(format!("n{}", i)).label("Node").done());
    }
    assert_eq!(g.node_count(), 1000);

    // Add 5000 relationships (cycling through nodes)
    for i in 0..5000 {
        let src = Subject::from_id(format!("n{}", i % 1000));
        let tgt = Subject::from_id(format!("n{}", (i + 1) % 1000));
        g.add_relationship(
            Subject::build(format!("r{}", i)).label("REL").done(),
            &src,
            &tgt,
        );
    }
    assert_eq!(g.relationship_count(), 5000);
    assert_eq!(g.node_count(), 1000);

    // Verify node access
    assert!(g.node(&"n0".into()).is_some());
    assert!(g.node(&"n999".into()).is_some());
    assert!(g.node(&"n1000".into()).is_none());

    // Verify iterator counts
    assert_eq!(g.nodes().count(), 1000);
    assert_eq!(g.relationships().count(), 5000);

    // Verify source/target
    let src = g.source(&"r0".into()).unwrap();
    assert_eq!(src.value.identity.0, "n0");
    let tgt = g.target(&"r0".into()).unwrap();
    assert_eq!(tgt.value.identity.0, "n1");

    // Verify degree (n0 has: r0 as source, r4999 as target, r999 as source, r1999 as source, etc.)
    let deg = g.degree(&"n0".into());
    assert!(deg > 0);

    // Verify neighbors
    let neighbors = g.neighbors(&"n0".into());
    assert!(!neighbors.is_empty());
}

// ============================================================================
// Back-reference label preservation tests (issue: overwrite bug)
// ============================================================================

/// A relationship pattern whose endpoint nodes are back-references (no labels)
/// must not overwrite labels established by an earlier pattern.
#[test]
fn from_patterns_back_reference_preserves_labels() {
    // Pattern 1: (red:Red)-[:GO]->(blue:Blue)
    let red_labeled = Subject::build("red").label("Red").done();
    let blue_labeled = Subject::build("blue").label("Blue").done();
    let p1 = Pattern::pattern(
        Subject::build("go1").label("GO").done(),
        vec![
            Pattern::point(red_labeled),
            Pattern::point(blue_labeled),
        ],
    );
    // Pattern 2: (blue)-[:GO]->(red)  — back-references, no labels
    let p2 = Pattern::pattern(
        Subject::build("go2").label("GO").done(),
        vec![
            Pattern::point(Subject::from_id("blue")),
            Pattern::point(Subject::from_id("red")),
        ],
    );

    let g = StandardGraph::from_patterns(vec![p1, p2]);

    let red = g.node(&"red".into()).unwrap();
    let blue = g.node(&"blue".into()).unwrap();

    assert!(red.value.labels.contains("Red"), "red should keep label Red");
    assert!(blue.value.labels.contains("Blue"), "blue should keep label Blue");
    assert_eq!(g.relationship_count(), 2);
}

/// add_pattern should apply the same union semantics as from_patterns.
#[test]
fn add_pattern_back_reference_preserves_labels() {
    let mut g = StandardGraph::new();

    let p1 = Pattern::pattern(
        Subject::build("r1").label("LINK").done(),
        vec![
            Pattern::point(Subject::build("a").label("A").done()),
            Pattern::point(Subject::build("b").label("B").done()),
        ],
    );
    let p2 = Pattern::pattern(
        Subject::build("r2").label("LINK").done(),
        vec![
            Pattern::point(Subject::from_id("b")),
            Pattern::point(Subject::from_id("a")),
        ],
    );

    g.add_pattern(p1);
    g.add_pattern(p2);

    let a = g.node(&"a".into()).unwrap();
    let b = g.node(&"b".into()).unwrap();

    assert!(a.value.labels.contains("A"), "a should keep label A");
    assert!(b.value.labels.contains("B"), "b should keep label B");
}

/// Three-node cycle: every node has its label only in the first pattern.
#[test]
fn from_patterns_three_node_cycle_preserves_all_labels() {
    // (green:Green)-[:GO]->(red:Red)
    let p1 = Pattern::pattern(
        Subject::build("go1").label("GO").done(),
        vec![
            Pattern::point(Subject::build("green").label("Green").done()),
            Pattern::point(Subject::build("red").label("Red").done()),
        ],
    );
    // (red)-[:GO]->(blue:Blue)
    let p2 = Pattern::pattern(
        Subject::build("go2").label("GO").done(),
        vec![
            Pattern::point(Subject::from_id("red")),
            Pattern::point(Subject::build("blue").label("Blue").done()),
        ],
    );
    // (blue)-[:GO]->(green)
    let p3 = Pattern::pattern(
        Subject::build("go3").label("GO").done(),
        vec![
            Pattern::point(Subject::from_id("blue")),
            Pattern::point(Subject::from_id("green")),
        ],
    );

    let g = StandardGraph::from_patterns(vec![p1, p2, p3]);

    assert!(g.node(&"green".into()).unwrap().value.labels.contains("Green"));
    assert!(g.node(&"red".into()).unwrap().value.labels.contains("Red"));
    assert!(g.node(&"blue".into()).unwrap().value.labels.contains("Blue"));
    assert_eq!(g.relationship_count(), 3);
}
