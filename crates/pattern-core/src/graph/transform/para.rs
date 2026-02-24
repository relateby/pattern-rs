//! para_graph and para_graph_fixed: shape-aware structural fold over a graph view.
//!
//! Ported from `Pattern.Graph.Transform.paraGraph` and `paraGraphFixed`
//! in the Haskell reference implementation.
//!
//! ## Processing order: `topo_shape_sort`
//!
//! Elements are sorted in two passes before the fold begins:
//!
//! **Pass 1 — Inter-bucket ordering** (fixed shape class priority):
//! ```text
//! GNode < GRelationship < GWalk < GAnnotation < GOther
//! ```
//! This ensures cross-class containment dependencies are satisfied: nodes (atomic)
//! before relationships (contain nodes), relationships before walks, walks before
//! annotations (can reference any shape below them), and annotations before other
//! (`GOther` is unconstrained).
//!
//! **Pass 2 — Within-bucket ordering** (Kahn's algorithm):
//! Applied to the `GAnnotation` and `GOther` buckets only. Direct sub-elements
//! (`Pattern::elements`) that belong to the same bucket are treated as dependencies
//! — they must appear before the element that contains them.
//!
//! `GNode`, `GRelationship`, and `GWalk` require no within-bucket sort: by the
//! definition of `classify_by_shape`, their sub-elements always belong to a
//! lower-priority bucket.
//!
//! **Cycle handling**: If a dependency cycle is detected within a bucket (e.g.,
//! annotation A references annotation B which references A), the cycle members
//! are appended after all non-cycle elements in their encountered order. No error
//! is raised. Cycle members receive `sub_results = &[]` for the other cycle
//! members (soft-miss). Callers should treat `sub_results` as best-effort.

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

use crate::graph::graph_classifier::{GraphClass, GraphValue};
use crate::graph::graph_query::GraphQuery;
use crate::graph::graph_view::GraphView;
use crate::pattern::Pattern;

// ============================================================================
// topo_shape_sort
// ============================================================================

/// Sort element indices into bottom-up containment order for `para_graph`.
///
/// Returns indices into `elems` in two-pass order:
/// 1. Partitioned by shape class: GNode, GRelationship, GWalk, GAnnotation, GOther.
/// 2. Within GAnnotation and GOther buckets: topologically sorted by in-bucket
///    dependencies via Kahn's algorithm; cycle members appended last.
///
/// Mirrors `topoShapeSort` in the Haskell reference implementation.
fn topo_shape_sort<Extra, V: GraphValue>(elems: &[(GraphClass<Extra>, Pattern<V>)]) -> Vec<usize>
where
    V::Id: Eq + Hash + Clone,
{
    let mut buckets: [Vec<usize>; 5] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    for (i, (cls, _)) in elems.iter().enumerate() {
        let rank = match cls {
            GraphClass::GNode => 0,
            GraphClass::GRelationship => 1,
            GraphClass::GWalk => 2,
            GraphClass::GAnnotation => 3,
            GraphClass::GOther(_) => 4,
        };
        buckets[rank].push(i);
    }

    let mut result = Vec::with_capacity(elems.len());

    for (rank, bucket) in buckets.iter().enumerate() {
        if rank >= 3 {
            // GAnnotation (3) and GOther (4): within-bucket topological sort
            result.extend(within_bucket_topo_sort(elems, bucket));
        } else {
            // GNode, GRelationship, GWalk: no within-bucket deps possible
            result.extend_from_slice(bucket);
        }
    }

    result
}

// ============================================================================
// within_bucket_topo_sort
// ============================================================================

/// Kahn's topological sort within a single bucket.
///
/// `bucket` is a slice of indices into `elems`. For each element `p` in the
/// bucket, any sub-element (`p.elements`) whose identity also belongs to the
/// bucket is treated as a dependency: it must be processed before `p`.
///
/// Returns bucket indices sorted so dependencies come first. Cycle members
/// are appended after all non-cycle elements in their encountered order.
fn within_bucket_topo_sort<Extra, V: GraphValue>(
    elems: &[(GraphClass<Extra>, Pattern<V>)],
    bucket: &[usize],
) -> Vec<usize>
where
    V::Id: Eq + Hash + Clone,
{
    if bucket.is_empty() {
        return Vec::new();
    }

    let n = bucket.len();

    // Map element identity → position within bucket (0..n)
    let id_to_bucket_pos: HashMap<V::Id, usize> = bucket
        .iter()
        .enumerate()
        .map(|(pos, &idx)| (elems[idx].1.value.identify().clone(), pos))
        .collect();

    // in_degree[pos] = number of in-bucket deps for element at bucket[pos]
    // dependents[pos] = bucket positions whose element depends on element at pos
    let mut in_degree = vec![0usize; n];
    let mut dependents: Vec<Vec<usize>> = vec![Vec::new(); n];

    for pos in 0..n {
        let (_, p) = &elems[bucket[pos]];
        for e in &p.elements {
            let eid = e.value.identify();
            if let Some(&dep_pos) = id_to_bucket_pos.get(eid) {
                // p (at pos) depends on e (at dep_pos)
                in_degree[pos] += 1;
                dependents[dep_pos].push(pos);
            }
        }
    }

    // Kahn's: initialize queue with zero-in-degree positions (encountered order)
    let mut queue: VecDeque<usize> = (0..n).filter(|&pos| in_degree[pos] == 0).collect();
    let mut sorted_positions: Vec<usize> = Vec::with_capacity(n);

    while let Some(pos) = queue.pop_front() {
        sorted_positions.push(pos);
        for &succ_pos in &dependents[pos] {
            in_degree[succ_pos] -= 1;
            if in_degree[succ_pos] == 0 {
                queue.push_back(succ_pos);
            }
        }
    }

    // Append cycle members in encountered order
    let mut in_sorted = vec![false; n];
    for &pos in &sorted_positions {
        in_sorted[pos] = true;
    }
    for (pos, &included) in in_sorted.iter().enumerate() {
        if !included {
            sorted_positions.push(pos);
        }
    }

    // Convert bucket positions back to elems indices
    sorted_positions.iter().map(|&pos| bucket[pos]).collect()
}

// ============================================================================
// para_graph_with_seed (private helper)
// ============================================================================

/// Run one paramorphism round over `view`, seeding sub-element results from `seed`.
///
/// Processes all `view_elements` in `topo_shape_sort` order. For each element `p`,
/// looks up results for its direct syntactic children (`p.elements`) in the
/// accumulator (which starts as `seed` and grows within the round — Gauss-Seidel
/// style).
///
/// Mirrors `paraGraphWithSeed` in the Haskell reference.
fn para_graph_with_seed<Extra, V: GraphValue, R: Clone>(
    f: &impl Fn(&GraphQuery<V>, &Pattern<V>, &[R]) -> R,
    view: &GraphView<Extra, V>,
    seed: HashMap<V::Id, R>,
) -> HashMap<V::Id, R>
where
    V::Id: Eq + Hash + Clone,
{
    let query = &view.view_query;
    let order = topo_shape_sort(&view.view_elements);
    let mut acc = seed;

    for i in order {
        let (_, p) = &view.view_elements[i];
        let sub_results: Vec<R> = p
            .elements
            .iter()
            .filter_map(|e| acc.get(e.value.identify()).cloned())
            .collect();
        let r = f(query, p, &sub_results);
        acc.insert(p.value.identify().clone(), r);
    }

    acc
}

// ============================================================================
// para_graph
// ============================================================================

/// Single-pass shape-aware structural fold over a `GraphView`.
///
/// Processes ALL elements (nodes, relationships, walks, annotations, other) in
/// `topo_shape_sort` order so that each element receives already-computed results
/// for its direct syntactic children (`Pattern::elements`).
///
/// The `sub_results` slice passed to `f` is best-effort: it contains one result
/// per direct sub-element that has already been processed. For cycle-free graphs
/// this is always complete. For cycle members within the `GAnnotation` or `GOther`
/// buckets, other members of the cycle will be absent from `sub_results`. Handle
/// `sub_results = &[]` as a valid, non-error input.
///
/// Returns a map from element identity to computed result.
///
/// Mirrors `paraGraph` in the Haskell reference implementation.
#[inline]
pub fn para_graph<Extra, V: GraphValue, R: Clone>(
    f: impl Fn(&GraphQuery<V>, &Pattern<V>, &[R]) -> R,
    view: &GraphView<Extra, V>,
) -> HashMap<V::Id, R>
where
    V::Id: Eq + Hash + Clone,
{
    para_graph_with_seed(&f, view, HashMap::new())
}

// ============================================================================
// para_graph_fixed
// ============================================================================

/// Fixed-point shape-aware fold for iterative convergence.
///
/// Iterates rounds of `para_graph` until `converged(old, new)` returns `true`
/// for all elements, then returns the final map. Each element starts with
/// `init.clone()`.
///
/// Each round uses the same `topo_shape_sort` ordering (the `GraphView` is
/// immutable). Within each round, already-processed elements' results are
/// available to later elements (Gauss-Seidel style).
///
/// Mirrors `paraGraphFixed` in the Haskell reference implementation.
#[inline]
pub fn para_graph_fixed<Extra, V: GraphValue, R: Clone>(
    converged: impl Fn(&R, &R) -> bool,
    f: impl Fn(&GraphQuery<V>, &Pattern<V>, &[R]) -> R,
    init: R,
    view: &GraphView<Extra, V>,
) -> HashMap<V::Id, R>
where
    V::Id: Eq + Hash + Clone,
{
    // Initialize all elements with init
    let mut current: HashMap<V::Id, R> = view
        .view_elements
        .iter()
        .map(|(_, p)| (p.value.identify().clone(), init.clone()))
        .collect();

    loop {
        let next = para_graph_with_seed(&f, view, current.clone());

        // Converged when conv(prev, new) holds for every key in next
        let all_converged = next
            .iter()
            .all(|(k, new_r)| current.get(k).is_some_and(|old_r| converged(old_r, new_r)));

        current = next;

        if all_converged {
            break;
        }
    }

    current
}
