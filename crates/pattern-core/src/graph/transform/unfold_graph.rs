//! unfold_graph: build a PatternGraph from seeds using an expander function.
//!
//! Ported from `Pattern.Graph.Transform.unfoldGraph` in the Haskell reference.

use crate::graph::graph_classifier::{GraphClassifier, GraphValue};
use crate::pattern::Pattern;
use crate::pattern_graph::{from_patterns_with_policy, PatternGraph};
use crate::reconcile::{HasIdentity, Mergeable, ReconciliationPolicy, Refinable};
use crate::subject::Symbol;

/// Build a `PatternGraph` from a list of seeds.
///
/// For each seed, `expand` returns a list of `Pattern<V>` to insert into the graph.
/// All patterns from all seeds are collected and inserted using the given classifier
/// and reconciliation policy.
///
/// # Examples
///
/// ```rust
/// use pattern_core::{unfold_graph, canonical_classifier, Pattern, Subject, Symbol, Value};
/// use pattern_core::reconcile::ReconciliationPolicy;
/// use std::collections::{HashSet, HashMap};
///
/// let classifier = canonical_classifier();
/// let policy = ReconciliationPolicy::LastWriteWins;
///
/// struct Row { id: &'static str }
///
/// let rows = vec![Row { id: "a" }, Row { id: "b" }];
/// let graph = unfold_graph(
///     &classifier,
///     &policy,
///     |row: Row| vec![Pattern::point(Subject {
///         identity: Symbol(row.id.to_string()),
///         labels: HashSet::new(),
///         properties: HashMap::new(),
///     })],
///     rows,
/// );
///
/// assert_eq!(graph.pg_nodes.len(), 2);
/// ```
pub fn unfold_graph<A, Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    expand: impl Fn(A) -> Vec<Pattern<V>>,
    seeds: Vec<A>,
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
    let patterns: Vec<Pattern<V>> = seeds.into_iter().flat_map(expand).collect();
    from_patterns_with_policy(classifier, policy, patterns)
}
