//! GraphQuery: portable, composable graph query interface.
//!
//! Ported from `Pattern.Graph.GraphQuery` in the Haskell reference implementation.
//!
//! # Overview
//!
//! `GraphQuery<V>` is a struct-of-closures representing the complete query interface
//! over a graph. Algorithms operate against this interface, not against any specific
//! backing representation. This enables the same algorithm code to run against
//! `PatternGraph`, database-backed stores, or any other structure that can produce
//! the nine required closures.
//!
//! # Structural Invariants
//!
//! Implementations of `GraphQuery<V>` must uphold these invariants:
//! 1. `query_source(r) = Some(s)` implies `s ∈ query_nodes()`
//! 2. `query_target(r) = Some(t)` implies `t ∈ query_nodes()`
//! 3. `r ∈ query_incident_rels(n)` implies `query_source(r) = Some(n) || query_target(r) = Some(n)`
//! 4. `query_degree(n) == query_incident_rels(n).len()` (default; may be faster indexed)
//! 5. `query_node_by_id(n.value.identify()) = Some(n)` for all `n ∈ query_nodes()`
//! 6. `query_relationship_by_id(r.value.identify()) = Some(r)` for all `r ∈ query_relationships()`
//! 7. `query_containers` returns only **direct** containers — not transitive containment

use std::collections::HashMap;

use crate::graph::graph_classifier::GraphValue;
use crate::pattern::Pattern;

// ============================================================================
// TraversalDirection
// ============================================================================

/// Which direction along a directed relationship is being traversed.
///
/// Used by [`TraversalWeight`] functions to return per-direction costs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalDirection {
    /// Source → Target: follow the relationship in its declared direction.
    Forward,
    /// Target → Source: traverse the relationship in reverse.
    Backward,
}

// ============================================================================
// TraversalWeight type alias  (Rc default; Arc under thread-safe feature)
// ============================================================================

/// A cost function for traversing a relationship in a given direction.
///
/// Returns a non-negative `f64`:
/// - Finite ≥ 0 — traversal allowed at that cost
/// - `f64::INFINITY` — traversal is blocked (impassable)
/// - Negative values are not supported; algorithm behavior is undefined
///
/// # Thread Safety
///
/// By default uses `Rc` (single-threaded). Enable the `thread-safe` feature
/// to use `Arc` with `Send + Sync` bounds.
#[cfg(not(feature = "thread-safe"))]
pub type TraversalWeight<V> = std::rc::Rc<dyn Fn(&Pattern<V>, TraversalDirection) -> f64>;

#[cfg(feature = "thread-safe")]
pub type TraversalWeight<V> =
    std::sync::Arc<dyn Fn(&Pattern<V>, TraversalDirection) -> f64 + Send + Sync>;

// ============================================================================
// Canonical weight functions
// ============================================================================

/// Uniform cost 1.0 in both directions — treat all edges as bidirectional.
#[cfg(not(feature = "thread-safe"))]
pub fn undirected<V>() -> TraversalWeight<V> {
    std::rc::Rc::new(|_rel: &Pattern<V>, _dir: TraversalDirection| 1.0)
}

#[cfg(feature = "thread-safe")]
pub fn undirected<V: Send + Sync + 'static>() -> TraversalWeight<V> {
    std::sync::Arc::new(|_rel: &Pattern<V>, _dir: TraversalDirection| 1.0)
}

/// Forward cost 1.0, Backward cost INFINITY — follow edge direction only.
#[cfg(not(feature = "thread-safe"))]
pub fn directed<V>() -> TraversalWeight<V> {
    std::rc::Rc::new(|_rel: &Pattern<V>, dir: TraversalDirection| match dir {
        TraversalDirection::Forward => 1.0,
        TraversalDirection::Backward => f64::INFINITY,
    })
}

#[cfg(feature = "thread-safe")]
pub fn directed<V: Send + Sync + 'static>() -> TraversalWeight<V> {
    std::sync::Arc::new(|_rel: &Pattern<V>, dir: TraversalDirection| match dir {
        TraversalDirection::Forward => 1.0,
        TraversalDirection::Backward => f64::INFINITY,
    })
}

/// Forward cost INFINITY, Backward cost 1.0 — follow edges in reverse only.
#[cfg(not(feature = "thread-safe"))]
pub fn directed_reverse<V>() -> TraversalWeight<V> {
    std::rc::Rc::new(|_rel: &Pattern<V>, dir: TraversalDirection| match dir {
        TraversalDirection::Forward => f64::INFINITY,
        TraversalDirection::Backward => 1.0,
    })
}

#[cfg(feature = "thread-safe")]
pub fn directed_reverse<V: Send + Sync + 'static>() -> TraversalWeight<V> {
    std::sync::Arc::new(|_rel: &Pattern<V>, dir: TraversalDirection| match dir {
        TraversalDirection::Forward => f64::INFINITY,
        TraversalDirection::Backward => 1.0,
    })
}

// ============================================================================
// GraphQuery struct (Rc default; Arc under thread-safe feature)
// ============================================================================

/// Portable graph query interface: a struct of nine closures.
///
/// All graph algorithms operate against `GraphQuery<V>`, not against any specific
/// backing representation. Cloning is cheap — it increments reference counts only.
///
/// # Construction
///
/// Use [`crate::from_pattern_graph`] to wrap a [`crate::PatternGraph`], or build
/// manually by providing all nine closure fields.
///
/// # Thread Safety
///
/// By default uses `Rc` (single-threaded). Enable the `thread-safe` Cargo feature
/// to use `Arc` with `Send + Sync` bounds throughout.
#[cfg(not(feature = "thread-safe"))]
#[allow(clippy::type_complexity)]
pub struct GraphQuery<V: GraphValue> {
    /// Returns all node patterns in the graph.
    pub query_nodes: std::rc::Rc<dyn Fn() -> Vec<Pattern<V>>>,
    /// Returns all relationship patterns in the graph.
    pub query_relationships: std::rc::Rc<dyn Fn() -> Vec<Pattern<V>>>,
    /// Returns all relationships incident to the given node (as source or target).
    pub query_incident_rels: std::rc::Rc<dyn Fn(&Pattern<V>) -> Vec<Pattern<V>>>,
    /// Returns the source node of a relationship, or `None` if not available.
    pub query_source: std::rc::Rc<dyn Fn(&Pattern<V>) -> Option<Pattern<V>>>,
    /// Returns the target node of a relationship, or `None` if not available.
    pub query_target: std::rc::Rc<dyn Fn(&Pattern<V>) -> Option<Pattern<V>>>,
    /// Returns the count of incident relationships for a node.
    pub query_degree: std::rc::Rc<dyn Fn(&Pattern<V>) -> usize>,
    /// Returns the node with the given identity, or `None`.
    pub query_node_by_id: std::rc::Rc<dyn Fn(&V::Id) -> Option<Pattern<V>>>,
    /// Returns the relationship with the given identity, or `None`.
    pub query_relationship_by_id: std::rc::Rc<dyn Fn(&V::Id) -> Option<Pattern<V>>>,
    /// Returns all direct containers of the given element (relationships, walks, annotations).
    pub query_containers: std::rc::Rc<dyn Fn(&Pattern<V>) -> Vec<Pattern<V>>>,
}

#[cfg(feature = "thread-safe")]
#[allow(clippy::type_complexity)]
pub struct GraphQuery<V: GraphValue> {
    /// Returns all node patterns in the graph.
    pub query_nodes: std::sync::Arc<dyn Fn() -> Vec<Pattern<V>> + Send + Sync>,
    /// Returns all relationship patterns in the graph.
    pub query_relationships: std::sync::Arc<dyn Fn() -> Vec<Pattern<V>> + Send + Sync>,
    /// Returns all relationships incident to the given node (as source or target).
    pub query_incident_rels: std::sync::Arc<dyn Fn(&Pattern<V>) -> Vec<Pattern<V>> + Send + Sync>,
    /// Returns the source node of a relationship, or `None` if not available.
    pub query_source: std::sync::Arc<dyn Fn(&Pattern<V>) -> Option<Pattern<V>> + Send + Sync>,
    /// Returns the target node of a relationship, or `None` if not available.
    pub query_target: std::sync::Arc<dyn Fn(&Pattern<V>) -> Option<Pattern<V>> + Send + Sync>,
    /// Returns the count of incident relationships for a node.
    pub query_degree: std::sync::Arc<dyn Fn(&Pattern<V>) -> usize + Send + Sync>,
    /// Returns the node with the given identity, or `None`.
    pub query_node_by_id: std::sync::Arc<dyn Fn(&V::Id) -> Option<Pattern<V>> + Send + Sync>,
    /// Returns the relationship with the given identity, or `None`.
    pub query_relationship_by_id:
        std::sync::Arc<dyn Fn(&V::Id) -> Option<Pattern<V>> + Send + Sync>,
    /// Returns all direct containers of the given element (relationships, walks, annotations).
    pub query_containers: std::sync::Arc<dyn Fn(&Pattern<V>) -> Vec<Pattern<V>> + Send + Sync>,
}

// ============================================================================
// Manual Clone for GraphQuery (pointer clone only — no data copy)
// ============================================================================

#[cfg(not(feature = "thread-safe"))]
impl<V: GraphValue> Clone for GraphQuery<V> {
    fn clone(&self) -> Self {
        GraphQuery {
            query_nodes: std::rc::Rc::clone(&self.query_nodes),
            query_relationships: std::rc::Rc::clone(&self.query_relationships),
            query_incident_rels: std::rc::Rc::clone(&self.query_incident_rels),
            query_source: std::rc::Rc::clone(&self.query_source),
            query_target: std::rc::Rc::clone(&self.query_target),
            query_degree: std::rc::Rc::clone(&self.query_degree),
            query_node_by_id: std::rc::Rc::clone(&self.query_node_by_id),
            query_relationship_by_id: std::rc::Rc::clone(&self.query_relationship_by_id),
            query_containers: std::rc::Rc::clone(&self.query_containers),
        }
    }
}

#[cfg(feature = "thread-safe")]
impl<V: GraphValue> Clone for GraphQuery<V> {
    fn clone(&self) -> Self {
        GraphQuery {
            query_nodes: std::sync::Arc::clone(&self.query_nodes),
            query_relationships: std::sync::Arc::clone(&self.query_relationships),
            query_incident_rels: std::sync::Arc::clone(&self.query_incident_rels),
            query_source: std::sync::Arc::clone(&self.query_source),
            query_target: std::sync::Arc::clone(&self.query_target),
            query_degree: std::sync::Arc::clone(&self.query_degree),
            query_node_by_id: std::sync::Arc::clone(&self.query_node_by_id),
            query_relationship_by_id: std::sync::Arc::clone(&self.query_relationship_by_id),
            query_containers: std::sync::Arc::clone(&self.query_containers),
        }
    }
}

// ============================================================================
// frame_query combinator
// ============================================================================

/// Restrict a `GraphQuery<V>` to elements satisfying `include`.
///
/// The returned `GraphQuery<V>` is itself a full query interface. All seven
/// structural invariants are preserved if they hold for `base`.
///
/// - `query_nodes` / `query_relationships` — filtered by predicate
/// - `query_incident_rels(n)` — base incident rels where both source AND target satisfy predicate
/// - `query_source` / `query_target` — delegated unchanged to base
/// - `query_degree(n)` — count of filtered incident rels
/// - `query_node_by_id(i)` — base lookup; returns `None` if result doesn't satisfy predicate
/// - `query_relationship_by_id(i)` — base lookup; returns `None` if result doesn't satisfy predicate
/// - `query_containers(p)` — base containers filtered by predicate
///
/// Rc and Arc variants are intentionally separate (no macro): only one is compiled per build,
/// and Rust does not abstract over Rc/Arc here without macros or runtime indirection.
#[cfg(not(feature = "thread-safe"))]
#[allow(clippy::type_complexity)]
pub fn frame_query<V>(
    include: std::rc::Rc<dyn Fn(&Pattern<V>) -> bool>,
    base: GraphQuery<V>,
) -> GraphQuery<V>
where
    V: GraphValue + Clone + 'static,
{
    use std::rc::Rc;

    let inc1 = Rc::clone(&include);
    let query_nodes = Rc::new(move || {
        (base.query_nodes)()
            .into_iter()
            .filter(|n| inc1(n))
            .collect()
    });

    let inc2 = Rc::clone(&include);
    let base_rels = Rc::clone(&base.query_relationships);
    let query_relationships =
        Rc::new(move || base_rels().into_iter().filter(|r| inc2(r)).collect());

    let inc3 = Rc::clone(&include);
    let base_inc = Rc::clone(&base.query_incident_rels);
    let base_src = Rc::clone(&base.query_source);
    let base_tgt = Rc::clone(&base.query_target);
    let query_incident_rels = Rc::new(move |node: &Pattern<V>| {
        base_inc(node)
            .into_iter()
            .filter(|rel| {
                let src_ok = base_src(rel).as_ref().map(|s| inc3(s)).unwrap_or(false);
                let tgt_ok = base_tgt(rel).as_ref().map(|t| inc3(t)).unwrap_or(false);
                src_ok && tgt_ok
            })
            .collect()
    });

    let query_source = Rc::clone(&base.query_source);
    let query_target = Rc::clone(&base.query_target);

    let inc4 = Rc::clone(&include);
    let base_inc2 = Rc::clone(&base.query_incident_rels);
    let base_src2 = Rc::clone(&base.query_source);
    let base_tgt2 = Rc::clone(&base.query_target);
    let query_degree = Rc::new(move |node: &Pattern<V>| {
        base_inc2(node)
            .into_iter()
            .filter(|rel| {
                let src_ok = base_src2(rel).as_ref().map(|s| inc4(s)).unwrap_or(false);
                let tgt_ok = base_tgt2(rel).as_ref().map(|t| inc4(t)).unwrap_or(false);
                src_ok && tgt_ok
            })
            .count()
    });

    let inc5 = Rc::clone(&include);
    let base_nbi = Rc::clone(&base.query_node_by_id);
    let query_node_by_id = Rc::new(move |id: &V::Id| base_nbi(id).filter(|n| inc5(n)));

    let inc6 = Rc::clone(&include);
    let base_rbi = Rc::clone(&base.query_relationship_by_id);
    let query_relationship_by_id = Rc::new(move |id: &V::Id| base_rbi(id).filter(|r| inc6(r)));

    let inc7 = Rc::clone(&include);
    let base_cont = Rc::clone(&base.query_containers);
    let query_containers = Rc::new(move |element: &Pattern<V>| {
        base_cont(element).into_iter().filter(|c| inc7(c)).collect()
    });

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

#[cfg(feature = "thread-safe")]
#[allow(clippy::type_complexity)]
pub fn frame_query<V>(
    include: std::sync::Arc<dyn Fn(&Pattern<V>) -> bool + Send + Sync>,
    base: GraphQuery<V>,
) -> GraphQuery<V>
where
    V: GraphValue + Clone + Send + Sync + 'static,
    V::Id: Clone + Send + Sync + 'static,
{
    use std::sync::Arc;

    let inc1 = Arc::clone(&include);
    let query_nodes = Arc::new(move || {
        (base.query_nodes)()
            .into_iter()
            .filter(|n| inc1(n))
            .collect()
    });

    let inc2 = Arc::clone(&include);
    let base_rels = Arc::clone(&base.query_relationships);
    let query_relationships =
        Arc::new(move || base_rels().into_iter().filter(|r| inc2(r)).collect());

    let inc3 = Arc::clone(&include);
    let base_inc = Arc::clone(&base.query_incident_rels);
    let base_src = Arc::clone(&base.query_source);
    let base_tgt = Arc::clone(&base.query_target);
    let query_incident_rels = Arc::new(move |node: &Pattern<V>| {
        base_inc(node)
            .into_iter()
            .filter(|rel| {
                let src_ok = base_src(rel).as_ref().map(|s| inc3(s)).unwrap_or(false);
                let tgt_ok = base_tgt(rel).as_ref().map(|t| inc3(t)).unwrap_or(false);
                src_ok && tgt_ok
            })
            .collect()
    });

    let query_source = Arc::clone(&base.query_source);
    let query_target = Arc::clone(&base.query_target);

    let inc4 = Arc::clone(&include);
    let base_inc2 = Arc::clone(&base.query_incident_rels);
    let base_src2 = Arc::clone(&base.query_source);
    let base_tgt2 = Arc::clone(&base.query_target);
    let query_degree = Arc::new(move |node: &Pattern<V>| {
        base_inc2(node)
            .into_iter()
            .filter(|rel| {
                let src_ok = base_src2(rel).as_ref().map(|s| inc4(s)).unwrap_or(false);
                let tgt_ok = base_tgt2(rel).as_ref().map(|t| inc4(t)).unwrap_or(false);
                src_ok && tgt_ok
            })
            .count()
    });

    let inc5 = Arc::clone(&include);
    let base_nbi = Arc::clone(&base.query_node_by_id);
    let query_node_by_id = Arc::new(move |id: &V::Id| base_nbi(id).filter(|n| inc5(n)));

    let inc6 = Arc::clone(&include);
    let base_rbi = Arc::clone(&base.query_relationship_by_id);
    let query_relationship_by_id = Arc::new(move |id: &V::Id| base_rbi(id).filter(|r| inc6(r)));

    let inc7 = Arc::clone(&include);
    let base_cont = Arc::clone(&base.query_containers);
    let query_containers = Arc::new(move |element: &Pattern<V>| {
        base_cont(element).into_iter().filter(|c| inc7(c)).collect()
    });

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

// ============================================================================
// memoize_incident_rels combinator
// ============================================================================

/// Wrap `query_incident_rels` and `query_degree` with an eager HashMap cache.
///
/// The cache is built upfront at construction time by calling `query_nodes()`
/// and `query_incident_rels(n)` for each node. All other fields pass through
/// unchanged.
///
/// Recommended for algorithms that call `query_incident_rels` repeatedly
/// (e.g., betweenness centrality).
///
/// # Cache Semantics
///
/// - Eager: the full cache is built when `memoize_incident_rels` is called.
/// - Per-`GraphQuery` cache — not global.
/// - No `RefCell` needed (immutable after construction).
#[cfg(not(feature = "thread-safe"))]
pub fn memoize_incident_rels<V>(base: GraphQuery<V>) -> GraphQuery<V>
where
    V: GraphValue + Clone + 'static,
    V::Id: Clone + Eq + std::hash::Hash + 'static,
{
    use std::rc::Rc;

    // Build the cache eagerly from all nodes.
    let nodes = (base.query_nodes)();
    let mut cache: HashMap<V::Id, Vec<Pattern<V>>> = HashMap::new();
    for node in &nodes {
        let id = node.value.identify().clone();
        let rels = (base.query_incident_rels)(node);
        cache.insert(id, rels);
    }
    let cache = Rc::new(cache);

    let cache1 = Rc::clone(&cache);
    let query_incident_rels = Rc::new(move |node: &Pattern<V>| {
        cache1
            .get(node.value.identify())
            .cloned()
            .unwrap_or_default()
    });

    let cache2 = Rc::clone(&cache);
    let query_degree = Rc::new(move |node: &Pattern<V>| {
        cache2
            .get(node.value.identify())
            .map(|v| v.len())
            .unwrap_or(0)
    });

    GraphQuery {
        query_nodes: base.query_nodes,
        query_relationships: base.query_relationships,
        query_incident_rels,
        query_source: base.query_source,
        query_target: base.query_target,
        query_degree,
        query_node_by_id: base.query_node_by_id,
        query_relationship_by_id: base.query_relationship_by_id,
        query_containers: base.query_containers,
    }
}

#[cfg(feature = "thread-safe")]
pub fn memoize_incident_rels<V>(base: GraphQuery<V>) -> GraphQuery<V>
where
    V: GraphValue + Clone + Send + Sync + 'static,
    V::Id: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
{
    use std::sync::Arc;

    let nodes = (base.query_nodes)();
    let mut cache: HashMap<V::Id, Vec<Pattern<V>>> = HashMap::new();
    for node in &nodes {
        let id = node.value.identify().clone();
        let rels = (base.query_incident_rels)(node);
        cache.insert(id, rels);
    }
    let cache = Arc::new(cache);

    let cache1 = Arc::clone(&cache);
    let query_incident_rels = Arc::new(move |node: &Pattern<V>| {
        cache1
            .get(node.value.identify())
            .cloned()
            .unwrap_or_default()
    });

    let cache2 = Arc::clone(&cache);
    let query_degree = Arc::new(move |node: &Pattern<V>| {
        cache2
            .get(node.value.identify())
            .map(|v| v.len())
            .unwrap_or(0)
    });

    GraphQuery {
        query_nodes: base.query_nodes,
        query_relationships: base.query_relationships,
        query_incident_rels,
        query_source: base.query_source,
        query_target: base.query_target,
        query_degree,
        query_node_by_id: base.query_node_by_id,
        query_relationship_by_id: base.query_relationship_by_id,
        query_containers: base.query_containers,
    }
}
