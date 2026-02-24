//! para_graph and para_graph_fixed: topology-aware folding over a graph view.
//!
//! Ported from `Pattern.Graph.Transform.paraGraph` and `paraGraphFixed`
//! in the Haskell reference implementation.

use std::collections::HashMap;
use std::hash::Hash;

use crate::graph::graph_classifier::GraphValue;
use crate::graph::graph_query::GraphQuery;
use crate::graph::graph_view::GraphView;
use crate::pattern::Pattern;

// ============================================================================
// para_graph
// ============================================================================

/// Topology-aware fold for DAGs.
///
/// Processes nodes in topological order (or arbitrary order if the graph is cyclic).
/// `f` receives the query, the current node, and a slice of results for predecessor
/// nodes (nodes with a directed relationship pointing to this node).
///
/// Returns a map from node identity to computed result.
#[inline]
pub fn para_graph<Extra, V: GraphValue + Clone, R: Clone>(
    f: impl Fn(&GraphQuery<V>, &Pattern<V>, &[R]) -> R,
    view: &GraphView<Extra, V>,
) -> HashMap<V::Id, R>
where
    V::Id: Eq + Hash + Clone + Ord,
{
    let query = &view.view_query;

    let ordered =
        crate::graph::algorithms::topological_sort(query).unwrap_or_else(|| (query.query_nodes)());

    let mut results: HashMap<V::Id, R> = HashMap::new();

    for node in &ordered {
        let node_id = node.value.identify();
        let rels = (query.query_incident_rels)(node);

        // Collect results of predecessor nodes (source of rels pointing to this node)
        let pred_results: Vec<R> = rels
            .iter()
            .filter_map(|rel| {
                let tgt = (query.query_target)(rel)?;
                if tgt.value.identify() == node_id {
                    let src = (query.query_source)(rel)?;
                    results.get(src.value.identify()).cloned()
                } else {
                    None
                }
            })
            .collect();

        let r = f(query, node, &pred_results);
        results.insert(node_id.clone(), r);
    }

    results
}

// ============================================================================
// para_graph_fixed
// ============================================================================

/// Fixed-point topology-aware fold for cyclic graphs.
///
/// Iterates rounds until `converged(old, new)` returns `true` for all elements,
/// then returns the final map. Each element starts with `init.clone()`.
///
/// `f` receives the query, the current element, and a slice of the current
/// round's results for predecessor nodes.
#[inline]
pub fn para_graph_fixed<Extra, V: GraphValue + Clone, R: Clone>(
    converged: impl Fn(&R, &R) -> bool,
    f: impl Fn(&GraphQuery<V>, &Pattern<V>, &[R]) -> R,
    init: R,
    view: &GraphView<Extra, V>,
) -> HashMap<V::Id, R>
where
    V::Id: Eq + Hash + Clone,
{
    let query = &view.view_query;
    let nodes = (query.query_nodes)();

    // Initialize all nodes with init
    let mut current: HashMap<V::Id, R> = nodes
        .iter()
        .map(|n| (n.value.identify().clone(), init.clone()))
        .collect();

    loop {
        let mut next: HashMap<V::Id, R> = HashMap::new();
        let mut all_converged = true;

        for node in &nodes {
            let node_id = node.value.identify();
            let rels = (query.query_incident_rels)(node);

            let pred_results: Vec<R> = rels
                .iter()
                .filter_map(|rel| {
                    let tgt = (query.query_target)(rel)?;
                    if tgt.value.identify() == node_id {
                        let src = (query.query_source)(rel)?;
                        current.get(src.value.identify()).cloned()
                    } else {
                        None
                    }
                })
                .collect();

            let new_r = f(query, node, &pred_results);

            if let Some(old_r) = current.get(node_id) {
                if !converged(old_r, &new_r) {
                    all_converged = false;
                }
            }

            next.insert(node_id.clone(), new_r);
        }

        current = next;

        if all_converged {
            break;
        }
    }

    current
}
