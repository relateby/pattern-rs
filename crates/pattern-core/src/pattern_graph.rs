//! PatternGraph: typed container for nodes, relationships, walks, and annotations.
//!
//! Ported from `Pattern.PatternGraph` in the Haskell reference implementation.
//! Patterns are routed into six typed collections by a `GraphClassifier`.
//! Duplicate identities are resolved via `ReconciliationPolicy`.

use std::collections::HashMap;

use crate::graph::graph_classifier::{GraphClass, GraphClassifier, GraphValue};
use crate::pattern::Pattern;
use crate::reconcile::{HasIdentity, Mergeable, ReconciliationPolicy, Refinable};
use crate::subject::Symbol;

// -----------------------------------------------------------------------------
// PatternGraph struct
// -----------------------------------------------------------------------------

/// Materialized graph container with six typed collections, each keyed by identity.
pub struct PatternGraph<Extra, V: GraphValue> {
    pub pg_nodes: HashMap<V::Id, Pattern<V>>,
    pub pg_relationships: HashMap<V::Id, Pattern<V>>,
    pub pg_walks: HashMap<V::Id, Pattern<V>>,
    pub pg_annotations: HashMap<V::Id, Pattern<V>>,
    pub pg_other: HashMap<V::Id, (Extra, Pattern<V>)>,
    pub pg_conflicts: HashMap<V::Id, Vec<Pattern<V>>>,
}

impl<Extra, V: GraphValue> PatternGraph<Extra, V> {
    /// Returns an empty graph with all six maps empty.
    pub fn empty() -> Self {
        PatternGraph {
            pg_nodes: HashMap::new(),
            pg_relationships: HashMap::new(),
            pg_walks: HashMap::new(),
            pg_annotations: HashMap::new(),
            pg_other: HashMap::new(),
            pg_conflicts: HashMap::new(),
        }
    }
}

// -----------------------------------------------------------------------------
// Trait bounds alias (used throughout)
// -----------------------------------------------------------------------------

// V must be GraphValue + HasIdentity<V, Symbol> + Mergeable + Refinable + PartialEq + Clone
// We spell this out on each function rather than using a trait alias (stable Rust).

// -----------------------------------------------------------------------------
// twoOccurrences helper
// -----------------------------------------------------------------------------

/// Constructs a synthetic pattern with `existing` as the root and `incoming` as
/// its single child. This gives `reconcile` exactly two occurrences to resolve.
fn two_occurrences<V: Clone>(existing: &Pattern<V>, incoming: Pattern<V>) -> Pattern<V> {
    Pattern {
        value: existing.value.clone(),
        elements: vec![incoming],
    }
}

// -----------------------------------------------------------------------------
// Private insert functions
// -----------------------------------------------------------------------------

fn insert_node<Extra, V>(
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    p: Pattern<V>,
    mut g: PatternGraph<Extra, V>,
) -> PatternGraph<Extra, V>
where
    V: GraphValue<Id = Symbol> + HasIdentity<V, Symbol> + Mergeable + Refinable + PartialEq + Clone,
{
    let i = V::identity(&p.value).clone();
    match g.pg_nodes.remove(&i) {
        None => {
            g.pg_nodes.insert(i, p);
        }
        Some(existing) => {
            let synthetic = two_occurrences(&existing, p.clone());
            match crate::reconcile::reconcile(policy, &synthetic) {
                Err(_) => {
                    g.pg_nodes.insert(i.clone(), existing);
                    g.pg_conflicts.entry(i).or_default().push(p);
                }
                Ok(merged) => {
                    g.pg_nodes.insert(i, merged);
                }
            }
        }
    }
    g
}

fn insert_relationship<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    p: Pattern<V>,
    g: PatternGraph<Extra, V>,
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
    // Merge endpoint nodes first.
    let g1 = if p.elements.len() == 2 {
        let n1 = p.elements[0].clone();
        let n2 = p.elements[1].clone();
        let g1 = merge_with_policy(classifier, policy, n1, g);
        merge_with_policy(classifier, policy, n2, g1)
    } else {
        g
    };

    let i = V::identity(&p.value).clone();
    let mut g2 = g1;
    match g2.pg_relationships.remove(&i) {
        None => {
            g2.pg_relationships.insert(i, p);
        }
        Some(existing) => {
            let synthetic = two_occurrences(&existing, p.clone());
            match crate::reconcile::reconcile(policy, &synthetic) {
                Err(_) => {
                    g2.pg_relationships.insert(i.clone(), existing);
                    g2.pg_conflicts.entry(i).or_default().push(p);
                }
                Ok(merged) => {
                    g2.pg_relationships.insert(i, merged);
                }
            }
        }
    }
    g2
}

fn insert_walk<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    p: Pattern<V>,
    g: PatternGraph<Extra, V>,
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
    // Merge each component relationship (which recursively merges their nodes).
    let elements: Vec<Pattern<V>> = p.elements.clone();
    let g1 = elements.into_iter().fold(g, |acc, elem| {
        merge_with_policy(classifier, policy, elem, acc)
    });

    let i = V::identity(&p.value).clone();
    let mut g2 = g1;
    match g2.pg_walks.remove(&i) {
        None => {
            g2.pg_walks.insert(i, p);
        }
        Some(existing) => {
            let synthetic = two_occurrences(&existing, p.clone());
            match crate::reconcile::reconcile(policy, &synthetic) {
                Err(_) => {
                    g2.pg_walks.insert(i.clone(), existing);
                    g2.pg_conflicts.entry(i).or_default().push(p);
                }
                Ok(merged) => {
                    g2.pg_walks.insert(i, merged);
                }
            }
        }
    }
    g2
}

fn insert_annotation<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    p: Pattern<V>,
    g: PatternGraph<Extra, V>,
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
    // Merge the single inner element first.
    let g1 = if p.elements.len() == 1 {
        let inner = p.elements[0].clone();
        merge_with_policy(classifier, policy, inner, g)
    } else {
        g
    };

    let i = V::identity(&p.value).clone();
    let mut g2 = g1;
    match g2.pg_annotations.remove(&i) {
        None => {
            g2.pg_annotations.insert(i, p);
        }
        Some(existing) => {
            let synthetic = two_occurrences(&existing, p.clone());
            match crate::reconcile::reconcile(policy, &synthetic) {
                Err(_) => {
                    g2.pg_annotations.insert(i.clone(), existing);
                    g2.pg_conflicts.entry(i).or_default().push(p);
                }
                Ok(merged) => {
                    g2.pg_annotations.insert(i, merged);
                }
            }
        }
    }
    g2
}

fn insert_other<Extra, V>(
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    extra: Extra,
    p: Pattern<V>,
    mut g: PatternGraph<Extra, V>,
) -> PatternGraph<Extra, V>
where
    V: GraphValue<Id = Symbol> + HasIdentity<V, Symbol> + Mergeable + Refinable + PartialEq + Clone,
{
    let i = V::identity(&p.value).clone();
    match g.pg_other.remove(&i) {
        None => {
            g.pg_other.insert(i, (extra, p));
        }
        Some((existing_extra, existing)) => {
            let synthetic = two_occurrences(&existing, p.clone());
            match crate::reconcile::reconcile(policy, &synthetic) {
                Err(_) => {
                    g.pg_other.insert(i.clone(), (existing_extra, existing));
                    g.pg_conflicts.entry(i).or_default().push(p);
                }
                Ok(merged) => {
                    g.pg_other.insert(i, (extra, merged));
                }
            }
        }
    }
    g
}

// -----------------------------------------------------------------------------
// Public API
// -----------------------------------------------------------------------------

/// Inserts one pattern using the given reconciliation policy.
///
/// Dispatches to the appropriate typed collection based on `classifier`.
/// Sub-elements are recursively merged before the top-level pattern is inserted.
pub fn merge_with_policy<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    p: Pattern<V>,
    g: PatternGraph<Extra, V>,
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
    match (classifier.classify)(&p) {
        GraphClass::GNode => insert_node(policy, p, g),
        GraphClass::GRelationship => insert_relationship(classifier, policy, p, g),
        GraphClass::GWalk => insert_walk(classifier, policy, p, g),
        GraphClass::GAnnotation => insert_annotation(classifier, policy, p, g),
        GraphClass::GOther(extra) => insert_other(policy, extra, p, g),
    }
}

/// Inserts one pattern using `LastWriteWins` policy.
pub fn merge<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    p: Pattern<V>,
    g: PatternGraph<Extra, V>,
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
    merge_with_policy(classifier, &ReconciliationPolicy::LastWriteWins, p, g)
}

/// Builds a graph from an iterable of patterns using `LastWriteWins`.
pub fn from_patterns<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    patterns: impl IntoIterator<Item = Pattern<V>>,
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
    from_patterns_with_policy(classifier, &ReconciliationPolicy::LastWriteWins, patterns)
}

/// Builds a graph from an iterable of patterns using the given policy.
pub fn from_patterns_with_policy<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    patterns: impl IntoIterator<Item = Pattern<V>>,
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
    patterns.into_iter().fold(PatternGraph::empty(), |g, p| {
        merge_with_policy(classifier, policy, p, g)
    })
}
