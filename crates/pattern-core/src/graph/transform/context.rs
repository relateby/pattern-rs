//! map_with_context: snapshot-based context-aware element mapping.
//!
//! Ported from `Pattern.Graph.Transform.mapWithContext` in the Haskell reference.

use crate::graph::graph_classifier::{GraphClassifier, GraphValue};
use crate::graph::graph_query::GraphQuery;
use crate::graph::graph_view::GraphView;
use crate::pattern::Pattern;

// ============================================================================
// map_with_context
// ============================================================================

/// Map elements in the view using the view's query as context.
///
/// The mapping function `f` receives a reference to the view's `GraphQuery`
/// (a snapshot taken before any transformation) and the element pattern by value.
/// All elements see the same snapshot — the query is not updated as elements
/// are transformed.
///
/// The view is consumed; the returned view has the same query (cloned from the
/// snapshot) and transformed elements.
#[inline]
pub fn map_with_context<Extra, V: GraphValue>(
    _classifier: &GraphClassifier<Extra, V>,
    f: impl Fn(&GraphQuery<V>, Pattern<V>) -> Pattern<V>,
    view: GraphView<Extra, V>,
) -> GraphView<Extra, V> {
    // Take a snapshot of the query before any transformation.
    let snapshot = view.view_query.clone();

    let view_elements = view
        .view_elements
        .into_iter()
        .map(|(cls, p)| (cls, f(&snapshot, p)))
        .collect();

    GraphView {
        view_query: snapshot,
        view_elements,
    }
}
