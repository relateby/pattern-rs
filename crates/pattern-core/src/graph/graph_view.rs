//! GraphView: a universal view over a classified graph.
//!
//! Ported from `Pattern.Graph.GraphView` in the Haskell reference implementation.
//!
//! A `GraphView` pairs a `GraphQuery` (read-only query interface) with a flat
//! list of classified elements. All graph transformations operate over `GraphView`
//! and produce a new `GraphView`; only `materialize` converts back to `PatternGraph`.

use std::hash::Hash;

use crate::graph::graph_classifier::{GraphClass, GraphClassifier, GraphValue};
use crate::graph::graph_query::GraphQuery;
use crate::pattern::Pattern;
use crate::pattern_graph::PatternGraph;
use crate::reconcile::{HasIdentity, Mergeable, ReconciliationPolicy, Refinable};
use crate::subject::Symbol;

// ============================================================================
// GraphView<Extra, V>
// ============================================================================

/// A universal graph-like interface: a query over the graph plus a list of
/// classified elements.
///
/// `view_query` provides read-only access to the underlying graph structure.
/// `view_elements` is the flat list of all elements in the view, each tagged
/// with its `GraphClass`.
///
/// Transformations consume the view and produce a new one. Clone is available
/// because `GraphQuery` is cheap to clone (Rc/Arc pointer increments only).
pub struct GraphView<Extra, V: GraphValue> {
    pub view_query: GraphQuery<V>,
    pub view_elements: Vec<(GraphClass<Extra>, Pattern<V>)>,
}

impl<Extra: Clone, V: GraphValue + Clone> Clone for GraphView<Extra, V> {
    fn clone(&self) -> Self {
        GraphView {
            view_query: self.view_query.clone(),
            view_elements: self.view_elements.clone(),
        }
    }
}

// ============================================================================
// from_pattern_graph
// ============================================================================

/// Builds the flat list of classified elements from a graph (shared by Rc/Arc variants).
fn view_elements_from_graph<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    graph: &PatternGraph<Extra, V>,
) -> Vec<(GraphClass<Extra>, Pattern<V>)>
where
    Extra: Clone,
    V: GraphValue + Clone,
{
    let mut view_elements: Vec<(GraphClass<Extra>, Pattern<V>)> = Vec::new();
    for p in graph.pg_nodes.values() {
        view_elements.push(((classifier.classify)(p), p.clone()));
    }
    for p in graph.pg_relationships.values() {
        view_elements.push(((classifier.classify)(p), p.clone()));
    }
    for p in graph.pg_walks.values() {
        view_elements.push(((classifier.classify)(p), p.clone()));
    }
    for p in graph.pg_annotations.values() {
        view_elements.push(((classifier.classify)(p), p.clone()));
    }
    for (_, p) in graph.pg_other.values() {
        view_elements.push(((classifier.classify)(p), p.clone()));
    }
    view_elements
}

/// Builds a `GraphView` from an existing `PatternGraph` and classifier.
///
/// The view's query is built from the graph (shared ownership via Rc/Arc).
/// `view_elements` contains every element in the graph classified by the
/// given classifier, in an unspecified but stable order.
#[cfg(not(feature = "thread-safe"))]
pub fn from_pattern_graph<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    graph: &PatternGraph<Extra, V>,
) -> GraphView<Extra, V>
where
    Extra: Clone + 'static,
    V: GraphValue + Clone + 'static,
    V::Id: Clone + Eq + Hash + 'static,
{
    use std::rc::Rc;

    let rc_graph = Rc::new(PatternGraph {
        pg_nodes: graph.pg_nodes.clone(),
        pg_relationships: graph.pg_relationships.clone(),
        pg_walks: graph.pg_walks.clone(),
        pg_annotations: graph.pg_annotations.clone(),
        pg_other: graph.pg_other.clone(),
        pg_conflicts: graph.pg_conflicts.clone(),
    });
    let view_query = crate::pattern_graph::from_pattern_graph(rc_graph);
    let view_elements = view_elements_from_graph(classifier, graph);
    GraphView {
        view_query,
        view_elements,
    }
}

#[cfg(feature = "thread-safe")]
pub fn from_pattern_graph<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    graph: &PatternGraph<Extra, V>,
) -> GraphView<Extra, V>
where
    Extra: Clone + Send + Sync + 'static,
    V: GraphValue + Clone + Send + Sync + 'static,
    V::Id: Clone + Eq + Hash + Send + Sync + 'static,
{
    use std::sync::Arc;

    let arc_graph = Arc::new(PatternGraph {
        pg_nodes: graph.pg_nodes.clone(),
        pg_relationships: graph.pg_relationships.clone(),
        pg_walks: graph.pg_walks.clone(),
        pg_annotations: graph.pg_annotations.clone(),
        pg_other: graph.pg_other.clone(),
        pg_conflicts: graph.pg_conflicts.clone(),
    });
    let view_query = crate::pattern_graph::from_pattern_graph(arc_graph);
    let view_elements = view_elements_from_graph(classifier, graph);
    GraphView {
        view_query,
        view_elements,
    }
}

// ============================================================================
// from_graph_lens (DEFERRED)
// ============================================================================

/// Builds a `GraphView` from a `GraphLens`.
///
/// **Deferred**: `GraphLens` has not yet been ported to pattern-rs.
/// This constructor will be implemented when `GraphLens` is available.
/// The signature accepts a classifier and a lens placeholder so that callers
/// can use the same API shape; it panics at runtime until the type is ported.
#[allow(dead_code)]
pub fn from_graph_lens<Extra, V, L>(
    _classifier: &GraphClassifier<Extra, V>,
    _lens: L,
) -> GraphView<Extra, V>
where
    V: GraphValue,
{
    todo!("from_graph_lens: deferred until GraphLens is ported to pattern-rs")
}

// ============================================================================
// materialize
// ============================================================================

/// Consumes a `GraphView` and produces a `PatternGraph`.
///
/// Each element in `view_elements` is inserted into a new graph using the
/// given classifier and reconciliation policy. Duplicate identities are resolved
/// by the policy.
pub fn materialize<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    view: GraphView<Extra, V>,
) -> PatternGraph<Extra, V>
where
    V: GraphValue<Id = Symbol>
        + HasIdentity<V, Symbol>
        + Mergeable
        + Refinable
        + PartialEq
        + Clone
        + 'static,
    Extra: 'static,
{
    crate::pattern_graph::from_patterns_with_policy(
        classifier,
        policy,
        view.view_elements.into_iter().map(|(_, p)| p),
    )
}
