//! map_graph, map_all_graph, filter_graph, fold_graph — categorized graph transformations.
//!
//! Ported from `Pattern.Graph.Transform` in the Haskell reference implementation.

use crate::graph::graph_classifier::{GraphClass, GraphClassifier, GraphValue};
use crate::graph::graph_view::GraphView;
use crate::pattern::Pattern;

use super::types::{CategoryMappers, Substitution};

// ============================================================================
// map_all_graph
// ============================================================================

/// Apply `f` uniformly to every element in the view, regardless of category.
///
/// The view's query is unchanged; only `view_elements` is transformed.
#[inline]
pub fn map_all_graph<Extra, V: GraphValue>(
    f: impl Fn(Pattern<V>) -> Pattern<V>,
    view: GraphView<Extra, V>,
) -> GraphView<Extra, V> {
    let view_query = view.view_query;
    let view_elements = view
        .view_elements
        .into_iter()
        .map(|(cls, p)| (cls, f(p)))
        .collect();
    GraphView {
        view_query,
        view_elements,
    }
}

// ============================================================================
// map_graph
// ============================================================================

/// Apply per-category mappers to elements in the view.
///
/// Each element is dispatched to the appropriate mapper based on its `GraphClass`.
/// Categories not overridden in `mappers` use the identity function (via
/// `CategoryMappers::identity()`).
#[inline]
pub fn map_graph<Extra: Clone, V: GraphValue>(
    _classifier: &GraphClassifier<Extra, V>,
    mappers: CategoryMappers<Extra, V>,
    view: GraphView<Extra, V>,
) -> GraphView<Extra, V> {
    let view_query = view.view_query;
    let view_elements = view
        .view_elements
        .into_iter()
        .map(|(cls, p)| {
            let mapped = match &cls {
                GraphClass::GNode => (mappers.nodes)(p),
                GraphClass::GRelationship => (mappers.relationships)(p),
                GraphClass::GWalk => (mappers.walks)(p),
                GraphClass::GAnnotation => (mappers.annotations)(p),
                GraphClass::GOther(_) => (mappers.other)(cls.clone(), p),
            };
            (cls, mapped)
        })
        .collect();
    GraphView {
        view_query,
        view_elements,
    }
}

// ============================================================================
// filter_graph
// ============================================================================

/// Filter elements in the view by a predicate, applying the substitution policy
/// for containers whose contained elements are removed.
///
/// - `NoSubstitution`: removed elements simply disappear; containers are kept as-is.
/// - `ReplaceWith(filler)`: removed elements are replaced by `filler` in the output list.
/// - `RemoveContainer`: **limitation** — in the flat `GraphView` model, container
///   relationships are not tracked; this variant behaves the same as `NoSubstitution`.
///   Full semantics (removing container elements when contained elements are filtered out)
///   would require container tracking or a post-materialize pass; not implemented here.
///
/// Note: in the flat `GraphView` model, container relationships are not tracked structurally
/// in `view_elements` — each element is independent. The `Substitution` policy affects
/// how removed elements are handled in the output list.
#[inline]
pub fn filter_graph<Extra: Clone, V: GraphValue + Clone>(
    _classifier: &GraphClassifier<Extra, V>,
    predicate: impl Fn(&GraphClass<Extra>, &Pattern<V>) -> bool,
    substitution: Substitution<V>,
    view: GraphView<Extra, V>,
) -> GraphView<Extra, V> {
    let view_query = view.view_query;
    let mut view_elements: Vec<(GraphClass<Extra>, Pattern<V>)> = Vec::new();

    for (cls, p) in view.view_elements {
        if predicate(&cls, &p) {
            view_elements.push((cls, p));
        } else {
            match &substitution {
                Substitution::NoSubstitution => {
                    // Element removed; leave gap (nothing added)
                }
                Substitution::ReplaceWith(filler) => {
                    view_elements.push((cls, filler.clone()));
                }
                Substitution::RemoveContainer => {
                    // In the flat view model, treat same as NoSubstitution
                }
            }
        }
    }

    GraphView {
        view_query,
        view_elements,
    }
}

// ============================================================================
// fold_graph
// ============================================================================

/// Fold over all elements in the view, accumulating a result.
///
/// `f` receives the current accumulator, the element's `GraphClass`, and a
/// reference to the element pattern. The view is not consumed.
#[inline]
pub fn fold_graph<Extra, V: GraphValue, M>(
    f: impl Fn(M, &GraphClass<Extra>, &Pattern<V>) -> M,
    init: M,
    view: &GraphView<Extra, V>,
) -> M {
    view.view_elements
        .iter()
        .fold(init, |acc, (cls, p)| f(acc, cls, p))
}
