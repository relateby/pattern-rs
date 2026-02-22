//! Graph classifier — structural classification of `Pattern<V>` values.
//!
//! Ported from `Pattern.Graph.GraphClassifier` in the Haskell reference implementation.

use std::hash::Hash;

use crate::pattern::Pattern;
use crate::subject::{Subject, Symbol};

// -----------------------------------------------------------------------------
// GraphValue trait
// -----------------------------------------------------------------------------

/// Value types that can be used with `GraphClassifier` and `PatternGraph`.
pub trait GraphValue {
    type Id: Ord + Clone + Hash;
    fn identify(&self) -> &Self::Id;
}

impl GraphValue for Subject {
    type Id = Symbol;

    fn identify(&self) -> &Symbol {
        &self.identity
    }
}

// -----------------------------------------------------------------------------
// GraphClass enum
// -----------------------------------------------------------------------------

/// The five structural categories a pattern can belong to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphClass<Extra> {
    GNode,
    GRelationship,
    GAnnotation,
    GWalk,
    GOther(Extra),
}

impl<Extra> GraphClass<Extra> {
    /// Maps a function over the `Extra` payload of `GOther`, leaving other variants unchanged.
    pub fn map_other<F, B>(self, f: F) -> GraphClass<B>
    where
        F: FnOnce(Extra) -> B,
    {
        match self {
            GraphClass::GNode => GraphClass::GNode,
            GraphClass::GRelationship => GraphClass::GRelationship,
            GraphClass::GAnnotation => GraphClass::GAnnotation,
            GraphClass::GWalk => GraphClass::GWalk,
            GraphClass::GOther(e) => GraphClass::GOther(f(e)),
        }
    }
}

// -----------------------------------------------------------------------------
// GraphClassifier struct
// -----------------------------------------------------------------------------

/// Injectable classification strategy wrapping a boxed closure.
pub struct GraphClassifier<Extra, V> {
    #[allow(clippy::type_complexity)]
    pub classify: Box<dyn Fn(&Pattern<V>) -> GraphClass<Extra> + 'static>,
}

impl<Extra, V> GraphClassifier<Extra, V> {
    /// Creates a new `GraphClassifier` from a classification function.
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&Pattern<V>) -> GraphClass<Extra> + 'static,
    {
        GraphClassifier {
            classify: Box::new(f),
        }
    }
}

// -----------------------------------------------------------------------------
// Private shape helpers
// -----------------------------------------------------------------------------

fn is_node_like<V>(p: &Pattern<V>) -> bool {
    p.elements.is_empty()
}

fn is_relationship_like<V>(p: &Pattern<V>) -> bool {
    p.elements.len() == 2 && p.elements.iter().all(is_node_like)
}

/// Frontier-based walk validation. Direction-agnostic: either endpoint can match.
fn is_valid_walk<V: GraphValue>(rels: &[Pattern<V>]) -> bool {
    if rels.is_empty() {
        return false;
    }

    // Seed the frontier with both endpoints of the first relationship.
    let first = &rels[0];
    if first.elements.len() != 2 {
        return false;
    }
    let mut frontier: Vec<&Pattern<V>> = vec![&first.elements[0], &first.elements[1]];

    for rel in &rels[1..] {
        if rel.elements.len() != 2 {
            return false;
        }
        let a = &rel.elements[0];
        let b = &rel.elements[1];

        let a_id = a.value.identify();
        let b_id = b.value.identify();

        let a_matches = frontier.iter().any(|x| x.value.identify() == a_id);
        let b_matches = frontier.iter().any(|x| x.value.identify() == b_id);

        frontier = match (a_matches, b_matches) {
            (true, false) => vec![b],
            (false, true) => vec![a],
            (true, true) => vec![a, b],
            (false, false) => return false,
        };
    }

    !frontier.is_empty()
}

// -----------------------------------------------------------------------------
// Public classification functions
// -----------------------------------------------------------------------------

/// Classifies a pattern by its structural shape.
pub fn classify_by_shape<V: GraphValue>(pattern: &Pattern<V>) -> GraphClass<()> {
    let els = &pattern.elements;

    if els.is_empty() {
        GraphClass::GNode
    } else if els.len() == 1 {
        GraphClass::GAnnotation
    } else if els.len() == 2 && els.iter().all(is_node_like) {
        GraphClass::GRelationship
    } else if els.iter().all(is_relationship_like) && is_valid_walk(els) {
        GraphClass::GWalk
    } else {
        GraphClass::GOther(())
    }
}

/// Returns the standard shape-based classifier.
pub fn canonical_classifier<V: GraphValue + 'static>() -> GraphClassifier<(), V> {
    GraphClassifier::new(|p| classify_by_shape(p))
}

/// Wraps a node predicate into a two-category classifier (`GNode` vs `GOther(())`).
pub fn from_test_node<V, F>(test_node: F) -> GraphClassifier<(), V>
where
    F: Fn(&Pattern<V>) -> bool + 'static,
{
    GraphClassifier::new(move |p| {
        if test_node(p) {
            GraphClass::GNode
        } else {
            GraphClass::GOther(())
        }
    })
}
