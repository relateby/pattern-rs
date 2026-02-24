//! Tests for GraphQuery construction, structural invariants, weight functions,
//! frame_query combinator, memoize_incident_rels, and query_containers.
//!
//! Corresponds to tasks T021, T034, T040. Haskell test IDs: HS-T015–HS-T017,
//! HS-T047–HS-T051, HS-T056.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use pattern_core::{
    canonical_classifier, connected_components, directed, directed_reverse, frame_query,
    from_patterns, graph_query_from_pattern_graph, is_connected, memoize_incident_rels, undirected,
    GraphClass, GraphQuery, GraphValue, Pattern, PatternGraph, Subject, Symbol, TraversalDirection,
};

// ============================================================================
// Test helpers
// ============================================================================

fn subj(id: &str) -> Subject {
    Subject {
        identity: Symbol(id.to_string()),
        labels: HashSet::new(),
        properties: HashMap::new(),
    }
}

fn node(id: &str) -> Pattern<Subject> {
    Pattern {
        value: subj(id),
        elements: vec![],
    }
}

fn rel(id: &str, src: Pattern<Subject>, tgt: Pattern<Subject>) -> Pattern<Subject> {
    Pattern {
        value: subj(id),
        elements: vec![src, tgt],
    }
}

fn annotation(id: &str, inner: Pattern<Subject>) -> Pattern<Subject> {
    Pattern {
        value: subj(id),
        elements: vec![inner],
    }
}

fn walk(id: &str, rels: Vec<Pattern<Subject>>) -> Pattern<Subject> {
    Pattern {
        value: subj(id),
        elements: rels,
    }
}

/// Build a simple triangle graph: A→B, B→C, A→C and return the wrapped GraphQuery.
fn triangle_query() -> (GraphQuery<Subject>, Rc<PatternGraph<(), Subject>>) {
    let na = node("A");
    let nb = node("B");
    let nc = node("C");
    let rab = rel("AB", na.clone(), nb.clone());
    let rbc = rel("BC", nb.clone(), nc.clone());
    let rac = rel("AC", na.clone(), nc.clone());

    let classifier = canonical_classifier::<Subject>();
    let pg = Rc::new(from_patterns(&classifier, vec![rab, rbc, rac]));
    let gq = graph_query_from_pattern_graph(Rc::clone(&pg));
    (gq, pg)
}

// ============================================================================
// HS-T015: All 9 GraphQuery fields return correct results from from_pattern_graph
// ============================================================================

#[test]
fn hs_t015_query_nodes_returns_all_nodes() {
    let (gq, pg) = triangle_query();
    let nodes = (gq.query_nodes)();
    assert_eq!(nodes.len(), pg.pg_nodes.len());
    assert_eq!(nodes.len(), 3, "triangle has 3 nodes");
}

#[test]
fn hs_t015_query_relationships_returns_all_rels() {
    let (gq, pg) = triangle_query();
    let rels = (gq.query_relationships)();
    assert_eq!(rels.len(), pg.pg_relationships.len());
    assert_eq!(rels.len(), 3, "triangle has 3 relationships");
}

#[test]
fn hs_t015_query_incident_rels_for_node_a() {
    let (gq, _) = triangle_query();
    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A must exist");
    let incident = (gq.query_incident_rels)(&a);
    // A is source of AB and AC
    assert_eq!(incident.len(), 2, "A has 2 incident rels (AB, AC)");
}

#[test]
fn hs_t015_query_source_target() {
    let (gq, _) = triangle_query();
    let ab = (gq.query_relationship_by_id)(&Symbol("AB".to_string())).expect("AB must exist");
    let src = (gq.query_source)(&ab).expect("AB must have source");
    let tgt = (gq.query_target)(&ab).expect("AB must have target");
    assert_eq!(src.value.identify(), &Symbol("A".to_string()));
    assert_eq!(tgt.value.identify(), &Symbol("B".to_string()));
}

#[test]
fn hs_t015_query_degree() {
    let (gq, _) = triangle_query();
    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A must exist");
    let degree = (gq.query_degree)(&a);
    assert_eq!(degree, 2, "A has degree 2");
}

#[test]
fn hs_t015_query_node_by_id() {
    let (gq, _) = triangle_query();
    let b = (gq.query_node_by_id)(&Symbol("B".to_string()));
    assert!(b.is_some(), "B must be found by id");
    let not_found = (gq.query_node_by_id)(&Symbol("Z".to_string()));
    assert!(not_found.is_none());
}

#[test]
fn hs_t015_query_relationship_by_id() {
    let (gq, _) = triangle_query();
    let bc = (gq.query_relationship_by_id)(&Symbol("BC".to_string()));
    assert!(bc.is_some(), "BC must be found by id");
}

// ============================================================================
// HS-T016: Structural invariants hold for a valid PatternGraph-backed GraphQuery
// ============================================================================

#[test]
fn hs_t016_inv1_source_in_query_nodes() {
    let (gq, _) = triangle_query();
    let node_ids: HashSet<Symbol> = (gq.query_nodes)()
        .iter()
        .map(|n| n.value.identity.clone())
        .collect();
    for rel_pat in (gq.query_relationships)() {
        if let Some(src) = (gq.query_source)(&rel_pat) {
            assert!(
                node_ids.contains(src.value.identify()),
                "source of rel must be in query_nodes"
            );
        }
    }
}

#[test]
fn hs_t016_inv2_target_in_query_nodes() {
    let (gq, _) = triangle_query();
    let node_ids: HashSet<Symbol> = (gq.query_nodes)()
        .iter()
        .map(|n| n.value.identity.clone())
        .collect();
    for rel_pat in (gq.query_relationships)() {
        if let Some(tgt) = (gq.query_target)(&rel_pat) {
            assert!(
                node_ids.contains(tgt.value.identify()),
                "target of rel must be in query_nodes"
            );
        }
    }
}

#[test]
fn hs_t016_inv3_incident_rel_endpoint_is_node() {
    let (gq, _) = triangle_query();
    for node_pat in (gq.query_nodes)() {
        let node_id = node_pat.value.identify().clone();
        for inc_rel in (gq.query_incident_rels)(&node_pat) {
            let src_id = (gq.query_source)(&inc_rel).map(|s| s.value.identify().clone());
            let tgt_id = (gq.query_target)(&inc_rel).map(|t| t.value.identify().clone());
            let is_endpoint =
                src_id.as_ref() == Some(&node_id) || tgt_id.as_ref() == Some(&node_id);
            assert!(is_endpoint, "node must be endpoint of its incident rels");
        }
    }
}

#[test]
fn hs_t016_inv4_degree_equals_incident_len() {
    let (gq, _) = triangle_query();
    for node_pat in (gq.query_nodes)() {
        let degree = (gq.query_degree)(&node_pat);
        let inc_len = (gq.query_incident_rels)(&node_pat).len();
        assert_eq!(degree, inc_len, "degree must equal incident rels count");
    }
}

#[test]
fn hs_t016_inv5_node_by_id_consistent() {
    let (gq, _) = triangle_query();
    for node_pat in (gq.query_nodes)() {
        let found = (gq.query_node_by_id)(node_pat.value.identify());
        assert!(
            found.is_some(),
            "query_node_by_id must find all nodes from query_nodes"
        );
    }
}

#[test]
fn hs_t016_inv6_rel_by_id_consistent() {
    let (gq, _) = triangle_query();
    for rel_pat in (gq.query_relationships)() {
        let found = (gq.query_relationship_by_id)(rel_pat.value.identify());
        assert!(
            found.is_some(),
            "query_relationship_by_id must find all rels from query_relationships"
        );
    }
}

// ============================================================================
// HS-T017: Canonical weight functions return correct costs
// ============================================================================

#[test]
fn hs_t017_undirected_always_1() {
    let (gq, _) = triangle_query();
    let w = undirected();
    for rel_pat in (gq.query_relationships)() {
        assert_eq!(w(&rel_pat, TraversalDirection::Forward), 1.0);
        assert_eq!(w(&rel_pat, TraversalDirection::Backward), 1.0);
    }
}

#[test]
fn hs_t017_directed_forward_1_backward_infinity() {
    let (gq, _) = triangle_query();
    let w = directed();
    for rel_pat in (gq.query_relationships)() {
        assert_eq!(w(&rel_pat, TraversalDirection::Forward), 1.0);
        assert_eq!(w(&rel_pat, TraversalDirection::Backward), f64::INFINITY);
    }
}

#[test]
fn hs_t017_directed_reverse_forward_infinity_backward_1() {
    let (gq, _) = triangle_query();
    let w = directed_reverse();
    for rel_pat in (gq.query_relationships)() {
        assert_eq!(w(&rel_pat, TraversalDirection::Forward), f64::INFINITY);
        assert_eq!(w(&rel_pat, TraversalDirection::Backward), 1.0);
    }
}

// ============================================================================
// HS-T047–HS-T051: frame_query combinator tests
// ============================================================================

/// Build a mixed graph: A→B (both "Person"), C→D (both "Thing")
fn mixed_graph_query() -> GraphQuery<Subject> {
    fn labeled_node(id: &str, label: &str) -> Pattern<Subject> {
        let mut labels = HashSet::new();
        labels.insert(label.to_string());
        Pattern {
            value: Subject {
                identity: Symbol(id.to_string()),
                labels,
                properties: HashMap::new(),
            },
            elements: vec![],
        }
    }

    let a = labeled_node("A", "Person");
    let b = labeled_node("B", "Person");
    let c = labeled_node("C", "Thing");
    let d = labeled_node("D", "Thing");

    let rab = rel("AB", a.clone(), b.clone());
    let rcd = rel("CD", c.clone(), d.clone());

    let classifier = canonical_classifier::<Subject>();
    let pg = Rc::new(from_patterns(&classifier, vec![rab, rcd]));
    graph_query_from_pattern_graph(pg)
}

#[test]
fn hs_t047_frame_excludes_nodes_outside_predicate() {
    let gq = mixed_graph_query();
    let include: Rc<dyn Fn(&Pattern<Subject>) -> bool> =
        Rc::new(|p: &Pattern<Subject>| p.value.labels.contains("Person"));

    let framed = frame_query(Rc::clone(&include), gq);
    let nodes = (framed.query_nodes)();
    assert_eq!(nodes.len(), 2, "only Person nodes should appear");
    for n in &nodes {
        assert!(
            n.value.labels.contains("Person"),
            "all framed nodes must be Person"
        );
    }
}

#[test]
fn hs_t048_frame_incident_rels_excludes_cross_frame_rels() {
    let gq = mixed_graph_query();
    let include: Rc<dyn Fn(&Pattern<Subject>) -> bool> =
        Rc::new(|p: &Pattern<Subject>| p.value.labels.contains("Person"));

    let framed = frame_query(Rc::clone(&include), gq);
    let nodes = (framed.query_nodes)();
    for n in &nodes {
        let inc = (framed.query_incident_rels)(n);
        for rel_pat in &inc {
            // Both endpoints must satisfy predicate
            let src_ok = (framed.query_source)(rel_pat)
                .as_ref()
                .map(|s| include(s))
                .unwrap_or(false);
            let tgt_ok = (framed.query_target)(rel_pat)
                .as_ref()
                .map(|t| include(t))
                .unwrap_or(false);
            assert!(src_ok && tgt_ok, "both endpoints must be in frame");
        }
    }
}

#[test]
fn hs_t049_memoize_returns_same_results() {
    let (gq, _) = triangle_query();
    let gq2 = gq.clone();
    let memo = memoize_incident_rels(gq2);

    for node_pat in (gq.query_nodes)() {
        let base_rels = (gq.query_incident_rels)(&node_pat);
        let memo_rels = (memo.query_incident_rels)(&node_pat);
        let mut base_ids: Vec<Symbol> =
            base_rels.iter().map(|r| r.value.identity.clone()).collect();
        let mut memo_ids: Vec<Symbol> =
            memo_rels.iter().map(|r| r.value.identity.clone()).collect();
        base_ids.sort();
        memo_ids.sort();
        assert_eq!(
            base_ids, memo_ids,
            "memoized rels must match base for each node"
        );
    }
}

#[test]
fn hs_t050_memoize_degree_equals_incident_len() {
    let (gq, _) = triangle_query();
    let memo = memoize_incident_rels(gq);

    for node_pat in (memo.query_nodes)() {
        let degree = (memo.query_degree)(&node_pat);
        let inc_len = (memo.query_incident_rels)(&node_pat).len();
        assert_eq!(
            degree, inc_len,
            "memoized degree must equal memoized incident len"
        );
    }
}

#[test]
fn hs_t051_frame_query_structural_invariants() {
    let gq = mixed_graph_query();
    let include: Rc<dyn Fn(&Pattern<Subject>) -> bool> =
        Rc::new(|p: &Pattern<Subject>| p.value.labels.contains("Person"));
    let framed = frame_query(include, gq);

    // Invariant 4: degree == incident_len
    for node_pat in (framed.query_nodes)() {
        let degree = (framed.query_degree)(&node_pat);
        let inc_len = (framed.query_incident_rels)(&node_pat).len();
        assert_eq!(degree, inc_len);
    }

    // Invariant 5: node_by_id consistent
    for node_pat in (framed.query_nodes)() {
        let found = (framed.query_node_by_id)(node_pat.value.identify());
        assert!(found.is_some());
    }

    // Invariant 6: rel_by_id consistent
    for rel_pat in (framed.query_relationships)() {
        let found = (framed.query_relationship_by_id)(rel_pat.value.identify());
        assert!(found.is_some());
    }
}

// ============================================================================
// HS-T056: query_containers returns correct containers
// ============================================================================

#[test]
fn hs_t056_query_containers_finds_relationship_for_node() {
    let na = node("A");
    let nb = node("B");
    let rab = rel("AB", na.clone(), nb.clone());

    let classifier = canonical_classifier::<Subject>();
    let pg = Rc::new(from_patterns(&classifier, vec![rab]));
    let gq = graph_query_from_pattern_graph(pg);

    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A must exist");
    let containers = (gq.query_containers)(&a);
    assert_eq!(containers.len(), 1, "A should have 1 container (rel AB)");
    assert_eq!(containers[0].value.identify(), &Symbol("AB".to_string()));
}

#[test]
fn hs_t056_query_containers_finds_walk_for_relationship() {
    let na = node("A");
    let nb = node("B");
    let nc = node("C");

    // Build rel patterns for walk
    let rab_for_walk = rel("AB", na.clone(), nb.clone());
    let rbc_for_walk = rel("BC", nb.clone(), nc.clone());
    let w = walk("W1", vec![rab_for_walk.clone(), rbc_for_walk.clone()]);

    let classifier = canonical_classifier::<Subject>();
    let pg = Rc::new(from_patterns(
        &classifier,
        vec![rab_for_walk, rbc_for_walk, w],
    ));
    let gq = graph_query_from_pattern_graph(pg);

    // Find AB relationship and check its containers include W1
    let ab = (gq.query_relationship_by_id)(&Symbol("AB".to_string())).expect("AB must exist");
    let containers = (gq.query_containers)(&ab);
    let walk_containers: Vec<_> = containers
        .iter()
        .filter(|c| c.value.identity == Symbol("W1".to_string()))
        .collect();
    assert_eq!(
        walk_containers.len(),
        1,
        "AB should be contained in walk W1"
    );
}

#[test]
fn query_containers_finds_annotation() {
    let na = node("A");
    let ann = annotation("Ann1", na.clone());

    let classifier = canonical_classifier::<Subject>();
    let pg = Rc::new(from_patterns(&classifier, vec![ann]));
    let gq = graph_query_from_pattern_graph(pg);

    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A must exist");
    let containers = (gq.query_containers)(&a);
    assert_eq!(containers.len(), 1, "A should have annotation as container");
    assert_eq!(containers[0].value.identity, Symbol("Ann1".to_string()));
}

#[test]
fn hs_t056_query_annotations_of() {
    let na = node("A");
    let ann = annotation("Ann1", na.clone());

    let classifier = canonical_classifier::<Subject>();
    let pg = Rc::new(from_patterns(&classifier, vec![ann]));
    let gq = graph_query_from_pattern_graph(pg);

    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A must exist");
    let annotations = pattern_core::query_annotations_of(&classifier, &gq, &a);
    assert_eq!(annotations.len(), 1, "should find 1 annotation");
    assert_eq!(annotations[0].value.identity, Symbol("Ann1".to_string()));
}

#[test]
fn hs_t056_query_walks_containing() {
    let na = node("A");
    let nb = node("B");
    let nc = node("C");

    let rab = rel("AB", na.clone(), nb.clone());
    let rbc = rel("BC", nb.clone(), nc.clone());
    let w = walk("W1", vec![rab.clone(), rbc.clone()]);

    let classifier = canonical_classifier::<Subject>();
    let pg = Rc::new(from_patterns(&classifier, vec![rab, rbc, w]));
    let gq = graph_query_from_pattern_graph(pg);

    let ab = (gq.query_relationship_by_id)(&Symbol("AB".to_string())).expect("AB must exist");
    let walks = pattern_core::query_walks_containing(&classifier, &gq, &ab);
    assert_eq!(walks.len(), 1, "AB should be in 1 walk");
    assert_eq!(walks[0].value.identity, Symbol("W1".to_string()));
}

#[test]
fn hs_t056_query_co_members() {
    let na = node("A");
    let nb = node("B");
    let nc = node("C");

    let rab = rel("AB", na.clone(), nb.clone());
    let rbc = rel("BC", nb.clone(), nc.clone());
    let w = walk("W1", vec![rab.clone(), rbc.clone()]);

    let classifier = canonical_classifier::<Subject>();
    let pg = Rc::new(from_patterns(&classifier, vec![rab, rbc, w]));
    let gq = graph_query_from_pattern_graph(pg);

    let ab = (gq.query_relationship_by_id)(&Symbol("AB".to_string())).expect("AB must exist");
    // W1 is a walk (pg_walks), not a node or relationship; reach it via query_containers.
    let containers = (gq.query_containers)(&ab);
    let w_pat = containers
        .into_iter()
        .find(|c| c.value.identity == Symbol("W1".to_string()))
        .expect("walk W1 must be a container of AB");

    let co_members = pattern_core::query_co_members(&gq, &ab, &w_pat);
    assert_eq!(co_members.len(), 1, "AB should have 1 co-member in W1 (BC)");
    assert_eq!(co_members[0].value.identity, Symbol("BC".to_string()));
}

// ============================================================================
// GraphQuery manual construction (representation-independence smoke test)
// ============================================================================

// ============================================================================
// Edge cases from spec
// ============================================================================

#[test]
fn frame_all_excluded_algorithms_return_empty() {
    // Predicate that excludes everything
    let gq = mixed_graph_query();
    let include: Rc<dyn Fn(&Pattern<Subject>) -> bool> = Rc::new(|_| false);
    let framed = frame_query(include, gq);

    assert_eq!(
        (framed.query_nodes)().len(),
        0,
        "all-excluded frame has no nodes"
    );
    assert_eq!(
        (framed.query_relationships)().len(),
        0,
        "all-excluded frame has no relationships"
    );

    // is_connected on empty frame is vacuously true
    assert!(
        pattern_core::is_connected(&framed, &undirected()),
        "empty frame is vacuously connected"
    );

    // connected_components on empty frame is empty
    let components = pattern_core::connected_components(&framed, &undirected());
    assert_eq!(components.len(), 0, "empty frame has no components");
}

#[test]
fn frame_nested_composition_filters_correctly() {
    // Build a graph with 3 nodes: A (Person+Female), B (Person+Male), C (Thing)
    fn labeled2(id: &str, l1: &str, l2: &str) -> Pattern<Subject> {
        let mut labels = HashSet::new();
        labels.insert(l1.to_string());
        labels.insert(l2.to_string());
        Pattern {
            value: Subject {
                identity: Symbol(id.to_string()),
                labels,
                properties: HashMap::new(),
            },
            elements: vec![],
        }
    }
    fn labeled1(id: &str, l1: &str) -> Pattern<Subject> {
        let mut labels = HashSet::new();
        labels.insert(l1.to_string());
        Pattern {
            value: Subject {
                identity: Symbol(id.to_string()),
                labels,
                properties: HashMap::new(),
            },
            elements: vec![],
        }
    }

    let a = labeled2("A", "Person", "Female");
    let b = labeled2("B", "Person", "Male");
    let c = labeled1("C", "Thing");

    let classifier = canonical_classifier::<Subject>();
    let pg = Rc::new(from_patterns(
        &classifier,
        vec![
            rel("AB", a.clone(), b.clone()),
            rel("BC", b.clone(), c.clone()),
        ],
    ));
    let gq = graph_query_from_pattern_graph(pg);

    // First frame: only Person nodes
    let person_pred: Rc<dyn Fn(&Pattern<Subject>) -> bool> =
        Rc::new(|p: &Pattern<Subject>| p.value.labels.contains("Person"));
    let person_frame = frame_query(Rc::clone(&person_pred), gq);

    // Second frame on top: only Female nodes
    let female_pred: Rc<dyn Fn(&Pattern<Subject>) -> bool> =
        Rc::new(|p: &Pattern<Subject>| p.value.labels.contains("Female"));
    let female_frame = frame_query(female_pred, person_frame);

    let nodes = (female_frame.query_nodes)();
    assert_eq!(
        nodes.len(),
        1,
        "nested frame should contain only A (Person+Female)"
    );
    assert_eq!(nodes[0].value.identity, Symbol("A".to_string()));
}

// ============================================================================
// GraphQuery manual construction (representation-independence smoke test)
// ============================================================================

#[test]
fn clone_is_cheap_pointer_clone() {
    let (gq, _) = triangle_query();
    let gq2 = gq.clone();
    let nodes1 = (gq.query_nodes)();
    let nodes2 = (gq2.query_nodes)();
    assert_eq!(nodes1.len(), nodes2.len(), "clones share same data");
}
