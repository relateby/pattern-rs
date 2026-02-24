//! Shared types for graph transformations: Substitution and CategoryMappers.

use crate::graph::graph_classifier::{GraphClass, GraphValue};
use crate::pattern::Pattern;

// ============================================================================
// Substitution<V>
// ============================================================================

/// Policy for container elements when a contained element is removed by `filter_graph`.
pub enum Substitution<V: GraphValue> {
    /// Removed element leaves a gap; container is kept as-is with the element absent.
    NoSubstitution,
    /// Removed element is replaced by the given filler pattern in any container.
    ReplaceWith(Pattern<V>),
    /// Containers that contained a removed element are themselves removed from the view.
    RemoveContainer,
}

// ============================================================================
// CategoryMappers<Extra, V>
// ============================================================================

/// Per-category transformation functions for `map_graph`.
///
/// Categories not overridden use the identity function. Build via
/// `CategoryMappers::identity()` and override only the categories you need
/// using struct-update syntax: `CategoryMappers { nodes: Box::new(f), ..CategoryMappers::identity() }`.
pub struct CategoryMappers<Extra, V: GraphValue> {
    pub nodes: Box<dyn Fn(Pattern<V>) -> Pattern<V>>,
    pub relationships: Box<dyn Fn(Pattern<V>) -> Pattern<V>>,
    pub walks: Box<dyn Fn(Pattern<V>) -> Pattern<V>>,
    pub annotations: Box<dyn Fn(Pattern<V>) -> Pattern<V>>,
    #[allow(clippy::type_complexity)]
    pub other: Box<dyn Fn(GraphClass<Extra>, Pattern<V>) -> Pattern<V>>,
}

impl<Extra, V: GraphValue> CategoryMappers<Extra, V> {
    /// Returns a `CategoryMappers` where every category is the identity function.
    pub fn identity() -> Self {
        CategoryMappers {
            nodes: Box::new(|p| p),
            relationships: Box::new(|p| p),
            walks: Box::new(|p| p),
            annotations: Box::new(|p| p),
            other: Box::new(|_cls, p| p),
        }
    }
}
