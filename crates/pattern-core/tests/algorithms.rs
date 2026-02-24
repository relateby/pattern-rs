//! Tests for graph algorithms: BFS, DFS, shortest_path, traversal direction,
//! connectivity, topological sort, MST, centrality, and representation independence.
//!
//! Corresponds to tasks T022, T022b, T027, T028, T035.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use pattern_core::{
    all_paths, betweenness_centrality, bfs, canonical_classifier, connected_components,
    degree_centrality, dfs, directed, directed_reverse, from_patterns,
    graph_query_from_pattern_graph, has_cycle, has_path, is_connected, is_neighbor,
    minimum_spanning_tree, shortest_path, topological_sort, undirected, GraphQuery, GraphValue,
    Pattern, PatternGraph, Subject, Symbol, TraversalDirection, TraversalWeight,
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

/// Build a PatternGraph from nodes and relationships and wrap in GraphQuery.
fn make_gq(patterns: Vec<Pattern<Subject>>) -> GraphQuery<Subject> {
    let classifier = canonical_classifier::<Subject>();
    let pg = Rc::new(from_patterns(&classifier, patterns));
    graph_query_from_pattern_graph(pg)
}

/// Build a linear chain: A→B→C
fn chain_abc() -> GraphQuery<Subject> {
    make_gq(vec![
        rel("AB", node("A"), node("B")),
        rel("BC", node("B"), node("C")),
    ])
}

/// Build a directed chain: A→B→C (same as chain but explicitly directed)
fn directed_chain_abc() -> GraphQuery<Subject> {
    chain_abc()
}

// ============================================================================
// T022: BFS correctness
// ============================================================================

#[test]
fn bfs_includes_start_node() {
    let gq = chain_abc();
    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A");
    let visited = bfs(&gq, &undirected(), &a);
    let ids: Vec<Symbol> = visited.iter().map(|n| n.value.identity.clone()).collect();
    assert!(
        ids.contains(&Symbol("A".to_string())),
        "BFS must include start"
    );
}

#[test]
fn bfs_visits_all_reachable_nodes() {
    let gq = chain_abc();
    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A");
    let visited = bfs(&gq, &undirected(), &a);
    assert_eq!(
        visited.len(),
        3,
        "BFS from A visits all 3 nodes (undirected)"
    );
}

#[test]
fn bfs_start_is_first() {
    let gq = chain_abc();
    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A");
    let visited = bfs(&gq, &undirected(), &a);
    assert_eq!(
        visited[0].value.identity,
        Symbol("A".to_string()),
        "BFS start node is first"
    );
}

// ============================================================================
// T022: DFS correctness
// ============================================================================

#[test]
fn dfs_includes_start_node() {
    let gq = chain_abc();
    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A");
    let visited = dfs(&gq, &undirected(), &a);
    let ids: Vec<Symbol> = visited.iter().map(|n| n.value.identity.clone()).collect();
    assert!(ids.contains(&Symbol("A".to_string())));
}

#[test]
fn dfs_visits_all_reachable_nodes() {
    let gq = chain_abc();
    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A");
    let visited = dfs(&gq, &undirected(), &a);
    assert_eq!(visited.len(), 3, "DFS from A visits all 3 nodes");
}

// ============================================================================
// T022: shortest_path correctness
// ============================================================================

#[test]
fn shortest_path_same_node_returns_singleton() {
    let gq = chain_abc();
    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A");
    let path = shortest_path(&gq, &undirected(), &a, &a);
    assert_eq!(path, Some(vec![a.clone()]), "same node → Some([node])");
}

#[test]
fn shortest_path_finds_direct_connection() {
    let gq = chain_abc();
    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A");
    let b = (gq.query_node_by_id)(&Symbol("B".to_string())).expect("B");
    let path = shortest_path(&gq, &undirected(), &a, &b);
    assert!(path.is_some(), "path A→B must exist");
    assert_eq!(path.unwrap().len(), 2, "path A→B has 2 nodes");
}

#[test]
fn shortest_path_finds_indirect_path() {
    let gq = chain_abc();
    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A");
    let c = (gq.query_node_by_id)(&Symbol("C".to_string())).expect("C");
    let path = shortest_path(&gq, &undirected(), &a, &c);
    assert!(path.is_some(), "path A→C exists through B");
    assert_eq!(path.unwrap().len(), 3, "path A→B→C has 3 nodes");
}

#[test]
fn shortest_path_disconnected_returns_none() {
    // Two isolated nodes
    let gq = make_gq(vec![node("X"), node("Y")]);
    let x = (gq.query_node_by_id)(&Symbol("X".to_string())).expect("X");
    let y = (gq.query_node_by_id)(&Symbol("Y".to_string())).expect("Y");
    let path = shortest_path(&gq, &undirected(), &x, &y);
    assert!(path.is_none(), "no path between isolated nodes");
}

// ============================================================================
// T022: has_path
// ============================================================================

#[test]
fn has_path_connected() {
    let gq = chain_abc();
    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A");
    let c = (gq.query_node_by_id)(&Symbol("C".to_string())).expect("C");
    assert!(has_path(&gq, &undirected(), &a, &c));
}

#[test]
fn has_path_same_node() {
    let gq = chain_abc();
    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A");
    assert!(has_path(&gq, &undirected(), &a, &a));
}

// ============================================================================
// T022: connected_components
// ============================================================================

#[test]
fn connected_components_single_component() {
    let gq = chain_abc();
    let components = connected_components(&gq, &undirected());
    assert_eq!(
        components.len(),
        1,
        "chain A-B-C is one connected component"
    );
    assert_eq!(components[0].len(), 3);
}

#[test]
fn connected_components_two_components() {
    let gq = make_gq(vec![
        rel("AB", node("A"), node("B")),
        rel("CD", node("C"), node("D")),
    ]);
    let components = connected_components(&gq, &undirected());
    assert_eq!(components.len(), 2, "two disconnected pairs → 2 components");
}

// ============================================================================
// T022: degree_centrality
// ============================================================================

#[test]
fn degree_centrality_star_graph() {
    // Star: center → L1, L2, L3 (center is source of 3 rels)
    // 4 nodes total, center degree=3 → centrality = 3/3 = 1.0
    // leaf degree=1 → centrality = 1/3
    let gq = make_gq(vec![
        rel("CL1", node("C"), node("L1")),
        rel("CL2", node("C"), node("L2")),
        rel("CL3", node("C"), node("L3")),
    ]);

    let centrality = degree_centrality(&gq);
    let center_score = centrality[&Symbol("C".to_string())];
    let leaf_score = centrality[&Symbol("L1".to_string())];

    assert!(
        (center_score - 1.0).abs() < 1e-10,
        "center degree centrality = 1.0, got {center_score}"
    );
    assert!(
        (leaf_score - 1.0 / 3.0).abs() < 1e-10,
        "leaf degree centrality = 1/3, got {leaf_score}"
    );
}

#[test]
fn degree_centrality_single_node() {
    let gq = make_gq(vec![node("A")]);
    let centrality = degree_centrality(&gq);
    assert_eq!(
        centrality[&Symbol("A".to_string())],
        0.0,
        "single node has centrality 0"
    );
}

// ============================================================================
// T022b: Representation independence — GraphQuery built from literal closures
// ============================================================================

/// Build a GraphQuery<Subject> by hand using literal closures, no PatternGraph backing.
/// This verifies that algorithms depend only on the GraphQuery interface (SC-002).
fn hand_built_triangle() -> GraphQuery<Subject> {
    let a = node("A");
    let b = node("B");
    let c = node("C");
    let rab = rel("AB", a.clone(), b.clone());
    let rbc = rel("BC", b.clone(), c.clone());
    let rac = rel("AC", a.clone(), c.clone());

    let all_nodes = Rc::new(vec![a.clone(), b.clone(), c.clone()]);
    let all_rels = Rc::new(vec![rab.clone(), rbc.clone(), rac.clone()]);

    let nodes_clone = Rc::clone(&all_nodes);
    let query_nodes = Rc::new(move || nodes_clone.as_ref().clone());

    let rels_clone = Rc::clone(&all_rels);
    let query_relationships = Rc::new(move || rels_clone.as_ref().clone());

    let rels_inc = Rc::clone(&all_rels);
    let query_incident_rels = Rc::new(move |n: &Pattern<Subject>| {
        let nid = n.value.identify();
        rels_inc
            .iter()
            .filter(|r| {
                r.elements.len() == 2
                    && (r.elements[0].value.identify() == nid
                        || r.elements[1].value.identify() == nid)
            })
            .cloned()
            .collect()
    });

    let query_source = Rc::new(|r: &Pattern<Subject>| r.elements.first().cloned());
    let query_target = Rc::new(|r: &Pattern<Subject>| r.elements.get(1).cloned());

    let rels_deg = Rc::clone(&all_rels);
    let query_degree = Rc::new(move |n: &Pattern<Subject>| {
        let nid = n.value.identify();
        rels_deg
            .iter()
            .filter(|r| {
                r.elements.len() == 2
                    && (r.elements[0].value.identify() == nid
                        || r.elements[1].value.identify() == nid)
            })
            .count()
    });

    let nodes_nbi = Rc::clone(&all_nodes);
    let query_node_by_id =
        Rc::new(move |id: &Symbol| nodes_nbi.iter().find(|n| n.value.identify() == id).cloned());

    let rels_rbi = Rc::clone(&all_rels);
    let query_relationship_by_id =
        Rc::new(move |id: &Symbol| rels_rbi.iter().find(|r| r.value.identify() == id).cloned());

    let query_containers = Rc::new(|_: &Pattern<Subject>| vec![]);

    GraphQuery {
        query_nodes,
        query_relationships,
        query_incident_rels,
        query_source,
        query_target,
        query_degree,
        query_node_by_id,
        query_relationship_by_id,
        query_containers,
    }
}

#[test]
fn hand_built_bfs_visits_all_nodes() {
    let gq = hand_built_triangle();
    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A");
    let visited = bfs(&gq, &undirected(), &a);
    assert_eq!(visited.len(), 3, "hand-built triangle: BFS visits 3 nodes");
}

#[test]
fn hand_built_connected_components_one_component() {
    let gq = hand_built_triangle();
    let components = connected_components(&gq, &undirected());
    assert_eq!(components.len(), 1, "hand-built triangle is one component");
}

// ============================================================================
// T027: Traversal direction tests (US2)
// ============================================================================

#[test]
fn directed_bfs_from_a_reaches_b_and_c() {
    let gq = directed_chain_abc();
    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A");
    let visited = bfs(&gq, &directed(), &a);
    let ids: Vec<Symbol> = visited.iter().map(|n| n.value.identity.clone()).collect();
    assert!(ids.contains(&Symbol("A".to_string())));
    assert!(ids.contains(&Symbol("B".to_string())));
    assert!(ids.contains(&Symbol("C".to_string())));
}

#[test]
fn directed_bfs_from_c_reaches_only_c() {
    let gq = directed_chain_abc();
    let c = (gq.query_node_by_id)(&Symbol("C".to_string())).expect("C");
    let visited = bfs(&gq, &directed(), &c);
    // With directed() from C: C has no outgoing edges, so only C is visited
    assert_eq!(visited.len(), 1, "directed from C: only C reachable");
    assert_eq!(visited[0].value.identity, Symbol("C".to_string()));
}

#[test]
fn directed_reverse_from_c_reaches_all() {
    let gq = directed_chain_abc();
    let c = (gq.query_node_by_id)(&Symbol("C".to_string())).expect("C");
    let visited = bfs(&gq, &directed_reverse(), &c);
    // Backward from C: C←B←A
    assert_eq!(visited.len(), 3, "directed_reverse from C: all 3 reachable");
}

#[test]
fn undirected_from_c_reaches_all() {
    let gq = directed_chain_abc();
    let c = (gq.query_node_by_id)(&Symbol("C".to_string())).expect("C");
    let visited = bfs(&gq, &undirected(), &c);
    assert_eq!(visited.len(), 3, "undirected from C: all 3 reachable");
}

#[test]
fn custom_weighted_shortest_path() {
    // Build a graph with two paths from A to C:
    // A→B→C (each edge cost 1) vs A→C direct (cost 5)
    let gq = make_gq(vec![
        rel("AB", node("A"), node("B")),
        rel("BC", node("B"), node("C")),
        rel("AC", node("A"), node("C")),
    ]);

    // Custom weight: AC costs 5.0, AB costs 1.0, BC costs 1.0
    let weight: TraversalWeight<Subject> = Rc::new(|rel: &Pattern<Subject>, dir| match dir {
        TraversalDirection::Forward => {
            if rel.value.identity == Symbol("AC".to_string()) {
                5.0
            } else {
                1.0
            }
        }
        TraversalDirection::Backward => f64::INFINITY,
    });

    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A");
    let c = (gq.query_node_by_id)(&Symbol("C".to_string())).expect("C");
    let path = shortest_path(&gq, &weight, &a, &c);

    assert!(path.is_some());
    let path = path.unwrap();
    // Should take A→B→C (cost 2) not A→C (cost 5)
    assert_eq!(path.len(), 3, "shortest path goes through B");
    assert_eq!(path[1].value.identity, Symbol("B".to_string()));
}

// ============================================================================
// T028: Topological sort and cycle detection (US2)
// ============================================================================

#[test]
fn topological_sort_on_dag() {
    // DAG: A→B, A→C, B→D, C→D
    let gq = make_gq(vec![
        rel("AB", node("A"), node("B")),
        rel("AC", node("A"), node("C")),
        rel("BD", node("B"), node("D")),
        rel("CD", node("C"), node("D")),
    ]);

    let order = topological_sort(&gq);
    assert!(order.is_some(), "DAG must have topological order");
    let order = order.unwrap();

    // Verify: A comes before B and C; B and C come before D
    let pos: HashMap<Symbol, usize> = order
        .iter()
        .enumerate()
        .map(|(i, n)| (n.value.identity.clone(), i))
        .collect();

    assert!(pos[&Symbol("A".to_string())] < pos[&Symbol("B".to_string())]);
    assert!(pos[&Symbol("A".to_string())] < pos[&Symbol("C".to_string())]);
    assert!(pos[&Symbol("B".to_string())] < pos[&Symbol("D".to_string())]);
    assert!(pos[&Symbol("C".to_string())] < pos[&Symbol("D".to_string())]);
}

#[test]
fn topological_sort_cyclic_returns_none() {
    // Cycle: A→B→C→A
    let gq = make_gq(vec![
        rel("AB", node("A"), node("B")),
        rel("BC", node("B"), node("C")),
        rel("CA", node("C"), node("A")),
    ]);

    let order = topological_sort(&gq);
    assert!(order.is_none(), "cyclic graph has no topological order");
}

#[test]
fn has_cycle_detects_cycle() {
    let gq = make_gq(vec![
        rel("AB", node("A"), node("B")),
        rel("BA", node("B"), node("A")),
    ]);
    assert!(has_cycle(&gq));
}

#[test]
fn has_cycle_false_on_dag() {
    let gq = chain_abc();
    assert!(!has_cycle(&gq), "linear chain A→B→C has no cycle");
}

#[test]
fn all_paths_finds_all_simple_paths() {
    // Two paths from A to C: A→B→C and A→C (direct)
    let gq = make_gq(vec![
        rel("AB", node("A"), node("B")),
        rel("BC", node("B"), node("C")),
        rel("AC", node("A"), node("C")),
    ]);

    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A");
    let c = (gq.query_node_by_id)(&Symbol("C".to_string())).expect("C");
    let paths = all_paths(&gq, &directed(), &a, &c);

    assert_eq!(paths.len(), 2, "should find 2 simple paths from A to C");
}

// ============================================================================
// T035: frame_query + BFS consistency (US3)
// ============================================================================

#[test]
fn frame_query_bfs_consistent_with_predicate() {
    use pattern_core::frame_query;

    // Build graph with Person and Thing nodes
    fn labeled(id: &str, lbl: &str) -> Pattern<Subject> {
        let mut labels = HashSet::new();
        labels.insert(lbl.to_string());
        Pattern {
            value: Subject {
                identity: Symbol(id.to_string()),
                labels,
                properties: HashMap::new(),
            },
            elements: vec![],
        }
    }

    let pa = labeled("A", "Person");
    let pb = labeled("B", "Person");
    let tc = labeled("C", "Thing");

    let gq = make_gq(vec![
        rel("AB", pa.clone(), pb.clone()),
        rel("BC", pb.clone(), tc.clone()),
    ]);

    let include: Rc<dyn Fn(&Pattern<Subject>) -> bool> =
        Rc::new(|p: &Pattern<Subject>| p.value.labels.contains("Person"));
    let framed = frame_query(include, gq);

    let fa = (framed.query_node_by_id)(&Symbol("A".to_string())).expect("A in frame");
    let visited = bfs(&framed, &undirected(), &fa);

    // Only Person nodes (A, B) should be visited; C is excluded
    for n in &visited {
        assert!(
            n.value.labels.contains("Person"),
            "BFS on framed graph should only visit Person nodes"
        );
    }
    assert_eq!(visited.len(), 2, "framed BFS visits only 2 Person nodes");
}

// ============================================================================
// T035: minimum_spanning_tree (US3)
// ============================================================================

#[test]
fn minimum_spanning_tree_triangle() {
    // Triangle A-B-C undirected (all costs 1)
    let gq = make_gq(vec![
        rel("AB", node("A"), node("B")),
        rel("BC", node("B"), node("C")),
        rel("AC", node("A"), node("C")),
    ]);

    let mst = minimum_spanning_tree(&gq, &undirected());
    // MST of triangle has all 3 nodes but only 2 edges (n-1)
    assert_eq!(mst.len(), 3, "MST of 3-node graph includes all 3 nodes");
}

#[test]
fn minimum_spanning_tree_empty() {
    let gq = make_gq(vec![node("A"), node("B")]);
    // No edges → empty MST
    let mst = minimum_spanning_tree(&gq, &undirected());
    assert_eq!(mst.len(), 0, "no edges → empty MST");
}

// ============================================================================
// T035: betweenness_centrality (US3)
// ============================================================================

#[test]
fn betweenness_centrality_path_graph() {
    // Path: A-B-C (undirected)
    // B is the only "bridge" node, so B should have higher betweenness than A and C
    let gq = make_gq(vec![
        rel("AB", node("A"), node("B")),
        rel("BC", node("B"), node("C")),
    ]);

    let centrality = betweenness_centrality(&gq, &undirected());
    let b_score = centrality[&Symbol("B".to_string())];
    let a_score = centrality[&Symbol("A".to_string())];
    let c_score = centrality[&Symbol("C".to_string())];

    assert!(
        b_score > a_score,
        "middle node B should have higher betweenness than A: b={b_score} a={a_score}"
    );
    assert!(
        b_score > c_score,
        "middle node B should have higher betweenness than C: b={b_score} c={c_score}"
    );
}

// ============================================================================
// Edge cases from spec
// ============================================================================

#[test]
fn bfs_start_node_not_in_graph_returns_singleton() {
    // A node constructed outside the graph — query_incident_rels returns []
    let gq = chain_abc();
    let ghost = node("Z"); // not inserted into the graph
    let visited = bfs(&gq, &undirected(), &ghost);
    // BFS always includes the start; no incident rels → only start returned
    assert_eq!(
        visited.len(),
        1,
        "BFS from absent node returns just that node"
    );
    assert_eq!(visited[0].value.identity, Symbol("Z".to_string()));
}

#[test]
fn dfs_start_node_not_in_graph_returns_singleton() {
    let gq = chain_abc();
    let ghost = node("Z");
    let visited = dfs(&gq, &undirected(), &ghost);
    assert_eq!(
        visited.len(),
        1,
        "DFS from absent node returns just that node"
    );
    assert_eq!(visited[0].value.identity, Symbol("Z".to_string()));
}

#[test]
fn all_infinity_weight_blocks_all_traversal() {
    let gq = chain_abc();
    let a = (gq.query_node_by_id)(&Symbol("A".to_string())).expect("A");
    let b = (gq.query_node_by_id)(&Symbol("B".to_string())).expect("B");

    // Weight that returns INFINITY in every direction for every edge
    let blocked: TraversalWeight<Subject> =
        Rc::new(|_rel: &Pattern<Subject>, _dir: TraversalDirection| f64::INFINITY);

    let visited = bfs(&gq, &blocked, &a);
    assert_eq!(
        visited.len(),
        1,
        "all-infinity weight: BFS visits only start node"
    );

    assert!(
        !has_path(&gq, &blocked, &a, &b),
        "all-infinity weight: no path exists between any two nodes"
    );

    let components = connected_components(&gq, &blocked);
    assert_eq!(
        components.len(),
        3,
        "all-infinity weight: every node is its own component"
    );
}
