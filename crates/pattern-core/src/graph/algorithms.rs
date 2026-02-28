//! Graph algorithms operating against the `GraphQuery<V>` interface.
//!
//! Ported from `Pattern.Graph.Algorithms` in the Haskell reference implementation.
//!
//! All functions are representation-independent: they operate on [`GraphQuery<V>`]
//! closures only. The same code works against `PatternGraph`, in-memory closures,
//! or any future backing store.
//!
//! # Traversal weight semantics
//!
//! Pass a [`TraversalWeight<V>`] to control which edges are traversable and at
//! what cost. Use the canonical functions [`undirected`](crate::graph::graph_query::undirected), [`directed`](crate::graph::graph_query::directed), or
//! [`directed_reverse`](crate::graph::graph_query::directed_reverse), or supply a custom `Rc<dyn Fn(...)>`.
//!
//! An edge with `INFINITY` cost in a given direction is impassable in that direction.

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use crate::graph::graph_classifier::{GraphClass, GraphClassifier, GraphValue};
use crate::graph::graph_query::{GraphQuery, TraversalDirection, TraversalWeight};
use crate::pattern::Pattern;

// ============================================================================
// Private helper: reachable_neighbors
// ============================================================================

/// Returns all immediately reachable neighbors of `node` under `weight`,
/// together with the traversal cost to reach each neighbor.
///
/// A neighbor is reachable via a forward traversal if the node is the source
/// of an incident relationship and the forward cost is finite. A neighbor is
/// reachable via a backward traversal if the node is the target and the
/// backward cost is finite.
#[inline]
fn reachable_neighbors<V>(
    q: &GraphQuery<V>,
    weight: &TraversalWeight<V>,
    node: &Pattern<V>,
) -> Vec<(Pattern<V>, f64)>
where
    V: GraphValue + Clone,
{
    let node_id = node.value.identify();
    let rels = (q.query_incident_rels)(node);
    let mut neighbors = Vec::new();

    for rel in rels {
        let src = (q.query_source)(&rel);
        let tgt = (q.query_target)(&rel);

        // Forward: node is the source → neighbor is the target
        if let Some(ref s) = src {
            if s.value.identify() == node_id {
                let fwd = weight(&rel, TraversalDirection::Forward);
                if fwd.is_finite() {
                    if let Some(t) = tgt.clone() {
                        neighbors.push((t, fwd));
                    }
                }
            }
        }

        // Backward: node is the target → neighbor is the source
        if let Some(ref t) = tgt {
            if t.value.identify() == node_id {
                let bwd = weight(&rel, TraversalDirection::Backward);
                if bwd.is_finite() {
                    if let Some(s) = src.clone() {
                        neighbors.push((s, bwd));
                    }
                }
            }
        }
    }

    neighbors
}

// ============================================================================
// Traversal algorithms
// ============================================================================

/// Breadth-first traversal from `start`.
///
/// Returns nodes in BFS visit order. The start node is always included.
pub fn bfs<V>(q: &GraphQuery<V>, weight: &TraversalWeight<V>, start: &Pattern<V>) -> Vec<Pattern<V>>
where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + std::hash::Hash + Ord,
{
    let mut visited: HashSet<V::Id> = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    let start_id = start.value.identify().clone();
    visited.insert(start_id);
    queue.push_back(start.clone());

    while let Some(current) = queue.pop_front() {
        result.push(current.clone());
        for (neighbor, _cost) in reachable_neighbors(q, weight, &current) {
            let nid = neighbor.value.identify().clone();
            if visited.insert(nid) {
                queue.push_back(neighbor);
            }
        }
    }

    result
}

/// Depth-first traversal from `start`.
///
/// Returns nodes in DFS visit order. The start node is always included.
pub fn dfs<V>(q: &GraphQuery<V>, weight: &TraversalWeight<V>, start: &Pattern<V>) -> Vec<Pattern<V>>
where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + std::hash::Hash + Ord,
{
    let mut visited: HashSet<V::Id> = HashSet::new();
    let mut stack = vec![start.clone()];
    let mut result = Vec::new();

    while let Some(current) = stack.pop() {
        let cid = current.value.identify().clone();
        if visited.insert(cid) {
            result.push(current.clone());
            for (neighbor, _cost) in reachable_neighbors(q, weight, &current) {
                if !visited.contains(neighbor.value.identify()) {
                    stack.push(neighbor);
                }
            }
        }
    }

    result
}

// ============================================================================
// Path algorithms
// ============================================================================

/// Find the minimum-cost path from `from` to `to` using Dijkstra's algorithm.
///
/// - Same node: returns `Some(vec![node])` immediately.
/// - No path: returns `None`.
/// - Uses `f64::INFINITY` cost to mark impassable edges.
pub fn shortest_path<V>(
    q: &GraphQuery<V>,
    weight: &TraversalWeight<V>,
    from: &Pattern<V>,
    to: &Pattern<V>,
) -> Option<Vec<Pattern<V>>>
where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + std::hash::Hash + Ord,
{
    // Same-node case
    if from.value.identify() == to.value.identify() {
        return Some(vec![from.clone()]);
    }

    // dist[id] = best known cost from `from` to node with that id
    let mut dist: HashMap<V::Id, f64> = HashMap::new();
    // prev[id] = predecessor node on the best-known path
    let mut prev: HashMap<V::Id, Pattern<V>> = HashMap::new();

    let from_id = from.value.identify().clone();
    dist.insert(from_id.clone(), 0.0);

    // Priority queue: (cost_bits_for_ordering, node_id) → node
    // For non-negative finite f64, the IEEE 754 bit pattern preserves ordering.
    let mut pq: BTreeMap<(u64, V::Id), Pattern<V>> = BTreeMap::new();
    pq.insert((0u64, from_id.clone()), from.clone());

    while let Some(((cost_bits, uid), node)) = pq.pop_first() {
        let cost = f64::from_bits(cost_bits);

        // Skip stale entries
        if let Some(&best) = dist.get(&uid) {
            if cost > best {
                continue;
            }
        }

        // Reached destination
        if uid == *to.value.identify() {
            let mut path = vec![node.clone()];
            let mut cur_id = uid.clone();
            while let Some(p) = prev.get(&cur_id) {
                path.push(p.clone());
                cur_id = p.value.identify().clone();
            }
            path.reverse();
            return Some(path);
        }

        for (neighbor, edge_cost) in reachable_neighbors(q, weight, &node) {
            if !edge_cost.is_finite() {
                continue;
            }
            let new_cost = cost + edge_cost;
            let nid = neighbor.value.identify().clone();

            let should_update = dist.get(&nid).map(|&d| new_cost < d).unwrap_or(true);
            if should_update {
                dist.insert(nid.clone(), new_cost);
                prev.insert(nid.clone(), node.clone());
                pq.insert((new_cost.to_bits(), nid), neighbor);
            }
        }
    }

    None
}

/// Returns `true` if a path exists from `from` to `to`.
///
/// Delegates to [`shortest_path`].
pub fn has_path<V>(
    q: &GraphQuery<V>,
    weight: &TraversalWeight<V>,
    from: &Pattern<V>,
    to: &Pattern<V>,
) -> bool
where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + std::hash::Hash + Ord,
{
    shortest_path(q, weight, from, to).is_some()
}

/// Enumerate all simple paths from `from` to `to` (no repeated nodes).
///
/// Returns a `Vec` of paths. Exponential worst case — use only on small graphs
/// or bounded subgraphs.
pub fn all_paths<V>(
    q: &GraphQuery<V>,
    weight: &TraversalWeight<V>,
    from: &Pattern<V>,
    to: &Pattern<V>,
) -> Vec<Vec<Pattern<V>>>
where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + std::hash::Hash + Ord,
{
    let mut all = Vec::new();
    let mut current_path = vec![from.clone()];
    let mut visited: HashSet<V::Id> = HashSet::new();
    visited.insert(from.value.identify().clone());

    all_paths_dfs(
        q,
        weight,
        from,
        to,
        &mut visited,
        &mut current_path,
        &mut all,
    );
    all
}

fn all_paths_dfs<V>(
    q: &GraphQuery<V>,
    weight: &TraversalWeight<V>,
    current: &Pattern<V>,
    to: &Pattern<V>,
    visited: &mut HashSet<V::Id>,
    current_path: &mut Vec<Pattern<V>>,
    all: &mut Vec<Vec<Pattern<V>>>,
) where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + std::hash::Hash + Ord,
{
    if current.value.identify() == to.value.identify() {
        all.push(current_path.clone());
        return;
    }

    for (neighbor, _cost) in reachable_neighbors(q, weight, current) {
        let nid = neighbor.value.identify().clone();
        if !visited.contains(&nid) {
            visited.insert(nid.clone());
            current_path.push(neighbor.clone());
            all_paths_dfs(q, weight, &neighbor, to, visited, current_path, all);
            current_path.pop();
            visited.remove(&nid);
        }
    }
}

// ============================================================================
// Boolean queries
// ============================================================================

/// Returns `true` if `b` is directly reachable from `a` in one hop under `weight`.
pub fn is_neighbor<V>(
    q: &GraphQuery<V>,
    weight: &TraversalWeight<V>,
    a: &Pattern<V>,
    b: &Pattern<V>,
) -> bool
where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + std::hash::Hash,
{
    let b_id = b.value.identify();
    reachable_neighbors(q, weight, a)
        .iter()
        .any(|(n, _)| n.value.identify() == b_id)
}

/// Returns `true` if the entire graph is connected under `weight`.
///
/// An empty graph is vacuously connected (returns `true`).
pub fn is_connected<V>(q: &GraphQuery<V>, weight: &TraversalWeight<V>) -> bool
where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + std::hash::Hash + Ord,
{
    let nodes = (q.query_nodes)();
    if nodes.is_empty() {
        return true;
    }
    let visited = bfs(q, weight, &nodes[0]);
    visited.len() == nodes.len()
}

// ============================================================================
// Structural algorithms
// ============================================================================

/// Partition the graph into connected components.
///
/// Returns a `Vec` of `Vec`s; each inner `Vec` is one component.
/// Uses BFS internally.
pub fn connected_components<V>(
    q: &GraphQuery<V>,
    weight: &TraversalWeight<V>,
) -> Vec<Vec<Pattern<V>>>
where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + std::hash::Hash + Ord,
{
    let all_nodes = (q.query_nodes)();
    let mut visited: HashSet<V::Id> = HashSet::new();
    let mut components = Vec::new();

    for node in &all_nodes {
        let nid = node.value.identify().clone();
        if !visited.contains(&nid) {
            let component = bfs(q, weight, node);
            for n in &component {
                visited.insert(n.value.identify().clone());
            }
            components.push(component);
        }
    }

    components
}

/// Topological sort using iterative DFS post-order with cycle detection.
///
/// - Returns `Some(order)` if the graph is a DAG.
/// - Returns `None` if a directed cycle is detected.
/// - Ignores `TraversalWeight` — uses relationship endpoint order only.
pub fn topological_sort<V>(q: &GraphQuery<V>) -> Option<Vec<Pattern<V>>>
where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + std::hash::Hash + Ord,
{
    let nodes = (q.query_nodes)();

    let mut in_stack: HashSet<V::Id> = HashSet::new();
    let mut done: HashSet<V::Id> = HashSet::new();
    let mut result: Vec<Pattern<V>> = Vec::new();

    // Returns forward neighbors (rels where node is the source)
    let forward_neighbors = |node: &Pattern<V>| -> Vec<Pattern<V>> {
        let rels = (q.query_incident_rels)(node);
        let node_id = node.value.identify();
        rels.into_iter()
            .filter_map(|rel| {
                let src = (q.query_source)(&rel)?;
                if src.value.identify() == node_id {
                    (q.query_target)(&rel)
                } else {
                    None
                }
            })
            .collect()
    };

    for start in &nodes {
        if done.contains(start.value.identify()) {
            continue;
        }

        let start_id = start.value.identify().clone();
        in_stack.insert(start_id);
        let neighbors = forward_neighbors(start);
        // Stack: (node, its_forward_neighbors, current_neighbor_index)
        let mut stack: Vec<(Pattern<V>, Vec<Pattern<V>>, usize)> =
            vec![(start.clone(), neighbors, 0)];

        while !stack.is_empty() {
            let cur_idx = stack.last().unwrap().2;
            let neighbors_len = stack.last().unwrap().1.len();

            if cur_idx < neighbors_len {
                let neighbor = stack.last().unwrap().1[cur_idx].clone();
                stack.last_mut().unwrap().2 += 1;

                let nid = neighbor.value.identify().clone();
                if in_stack.contains(&nid) {
                    return None; // Back edge — cycle detected
                }
                if !done.contains(&nid) {
                    in_stack.insert(nid);
                    let next_neighbors = forward_neighbors(&neighbor);
                    stack.push((neighbor, next_neighbors, 0));
                }
            } else {
                let (node, _, _) = stack.pop().unwrap();
                let nid = node.value.identify().clone();
                in_stack.remove(&nid);
                done.insert(nid);
                result.push(node);
            }
        }
    }

    result.reverse();
    Some(result)
}

/// Returns `true` if the graph contains a directed cycle.
///
/// Delegates to [`topological_sort`].
pub fn has_cycle<V>(q: &GraphQuery<V>) -> bool
where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + std::hash::Hash + Ord,
{
    topological_sort(q).is_none()
}

// ============================================================================
// Spanning
// ============================================================================

/// Minimum spanning tree using Kruskal's algorithm with path-compression union-find.
///
/// - Edge cost is `min(forward_cost, backward_cost)`.
/// - Edges with `INFINITY` cost in both directions are excluded.
/// - Returns the subset of nodes that are included in the MST.
pub fn minimum_spanning_tree<V>(q: &GraphQuery<V>, weight: &TraversalWeight<V>) -> Vec<Pattern<V>>
where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + std::hash::Hash + Ord,
{
    let nodes = (q.query_nodes)();
    if nodes.is_empty() {
        return Vec::new();
    }

    // Collect all edges with their MST cost
    let mut edges: Vec<(f64, Pattern<V>)> = (q.query_relationships)()
        .into_iter()
        .filter_map(|rel| {
            let fwd = weight(&rel, TraversalDirection::Forward);
            let bwd = weight(&rel, TraversalDirection::Backward);
            let cost = fwd.min(bwd);
            if cost.is_finite() {
                Some((cost, rel))
            } else {
                None
            }
        })
        .collect();

    // Sort edges by cost (ascending)
    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    // Initialize union-find: each node is its own component
    let mut parent: HashMap<V::Id, V::Id> = nodes
        .iter()
        .map(|n| (n.value.identify().clone(), n.value.identify().clone()))
        .collect();

    let mut mst_node_ids: HashSet<V::Id> = HashSet::new();

    for (_, rel) in edges {
        let src = match (q.query_source)(&rel) {
            Some(s) => s,
            None => continue,
        };
        let tgt = match (q.query_target)(&rel) {
            Some(t) => t,
            None => continue,
        };

        let src_id = src.value.identify().clone();
        let tgt_id = tgt.value.identify().clone();

        let root_src = uf_find(&mut parent, src_id.clone());
        let root_tgt = uf_find(&mut parent, tgt_id.clone());

        if root_src != root_tgt {
            // Union: merge the two components
            parent.insert(root_src, root_tgt);
            mst_node_ids.insert(src_id);
            mst_node_ids.insert(tgt_id);
        }
    }

    nodes
        .into_iter()
        .filter(|n| mst_node_ids.contains(n.value.identify()))
        .collect()
}

/// Path-compression union-find: returns the root of the component containing `x`.
fn uf_find<Id>(parent: &mut HashMap<Id, Id>, x: Id) -> Id
where
    Id: Clone + Eq + std::hash::Hash,
{
    let p = parent[&x].clone();
    if p == x {
        return x;
    }
    let root = uf_find(parent, p);
    parent.insert(x, root.clone());
    root
}

// ============================================================================
// Centrality
// ============================================================================

/// Degree centrality for all nodes.
///
/// For a graph with `n` nodes, the degree centrality of node `v` is
/// `degree(v) / (n - 1)`. Returns 0.0 for all nodes in a single-node graph.
///
/// Does **not** take a `TraversalWeight` parameter — degree centrality is a
/// structural property (count of incident relationships, direction-agnostic).
pub fn degree_centrality<V>(q: &GraphQuery<V>) -> HashMap<V::Id, f64>
where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + std::hash::Hash,
{
    let nodes = (q.query_nodes)();
    let n = nodes.len();
    let mut result = HashMap::new();

    for node in &nodes {
        let degree = (q.query_degree)(node) as f64;
        let centrality = if n > 1 { degree / (n - 1) as f64 } else { 0.0 };
        result.insert(node.value.identify().clone(), centrality);
    }

    result
}

/// Betweenness centrality using the Brandes BFS algorithm (unnormalized).
///
/// Returns the unnormalized betweenness score for each node. To normalize for
/// an undirected graph with `n` nodes, divide by `(n-1)(n-2)/2`.
///
/// Uses the `weight` function to determine which edges are traversable (finite
/// cost = reachable). All reachable edges are treated as unit-weight for the
/// shortest-path counting phase.
pub fn betweenness_centrality<V>(
    q: &GraphQuery<V>,
    weight: &TraversalWeight<V>,
) -> HashMap<V::Id, f64>
where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + std::hash::Hash + Ord,
{
    let nodes = (q.query_nodes)();
    let mut betweenness: HashMap<V::Id, f64> = nodes
        .iter()
        .map(|n| (n.value.identify().clone(), 0.0))
        .collect();

    for s in &nodes {
        let s_id = s.value.identify().clone();

        // BFS phase
        let mut stack: Vec<Pattern<V>> = Vec::new();
        let mut pred: HashMap<V::Id, Vec<Pattern<V>>> = nodes
            .iter()
            .map(|n| (n.value.identify().clone(), Vec::new()))
            .collect();
        let mut sigma: HashMap<V::Id, f64> = nodes
            .iter()
            .map(|n| (n.value.identify().clone(), 0.0))
            .collect();
        sigma.insert(s_id.clone(), 1.0);
        let mut dist: HashMap<V::Id, i64> = nodes
            .iter()
            .map(|n| (n.value.identify().clone(), -1))
            .collect();
        dist.insert(s_id.clone(), 0);

        let mut queue = VecDeque::new();
        queue.push_back(s.clone());

        while let Some(v) = queue.pop_front() {
            stack.push(v.clone());
            let v_id = v.value.identify().clone();
            let v_dist = dist[&v_id];
            let v_sigma = sigma[&v_id];

            for (w, _cost) in reachable_neighbors(q, weight, &v) {
                let w_id = w.value.identify().clone();
                // First time visiting w?
                if dist[&w_id] < 0 {
                    queue.push_back(w.clone());
                    *dist.get_mut(&w_id).unwrap() = v_dist + 1;
                }
                // On a shortest path through v?
                if dist[&w_id] == v_dist + 1 {
                    *sigma.get_mut(&w_id).unwrap() += v_sigma;
                    pred.get_mut(&w_id).unwrap().push(v.clone());
                }
            }
        }

        // Back-propagation
        let mut delta: HashMap<V::Id, f64> = nodes
            .iter()
            .map(|n| (n.value.identify().clone(), 0.0))
            .collect();

        while let Some(w) = stack.pop() {
            let w_id = w.value.identify().clone();
            for v in &pred[&w_id] {
                let v_id = v.value.identify().clone();
                let sigma_w = sigma[&w_id];
                if sigma_w != 0.0 {
                    let coeff = sigma[&v_id] / sigma_w * (1.0 + delta[&w_id]);
                    *delta.get_mut(&v_id).unwrap() += coeff;
                }
            }
            if w_id != s_id {
                *betweenness.get_mut(&w_id).unwrap() += delta[&w_id];
            }
        }
    }

    betweenness
}

// ============================================================================
// Context query helpers
// ============================================================================

/// Returns all containers of `element` that are classified as annotations.
pub fn query_annotations_of<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    q: &GraphQuery<V>,
    element: &Pattern<V>,
) -> Vec<Pattern<V>>
where
    V: GraphValue + Clone,
{
    (q.query_containers)(element)
        .into_iter()
        .filter(|c| matches!((classifier.classify)(c), GraphClass::GAnnotation))
        .collect()
}

/// Returns all containers of `element` that are classified as walks.
pub fn query_walks_containing<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    q: &GraphQuery<V>,
    element: &Pattern<V>,
) -> Vec<Pattern<V>>
where
    V: GraphValue + Clone,
{
    (q.query_containers)(element)
        .into_iter()
        .filter(|c| matches!((classifier.classify)(c), GraphClass::GWalk))
        .collect()
}

/// Returns all elements that share `container` with `element`, excluding `element` itself.
///
/// Co-membership is checked by identity (`V::Id`). The container's `elements` field
/// is traversed directly — O(k) where k = number of elements in the container.
pub fn query_co_members<V>(
    _q: &GraphQuery<V>,
    element: &Pattern<V>,
    container: &Pattern<V>,
) -> Vec<Pattern<V>>
where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + std::hash::Hash,
{
    let elem_id = element.value.identify();
    container
        .elements
        .iter()
        .filter(|e| e.value.identify() != elem_id)
        .cloned()
        .collect()
}
