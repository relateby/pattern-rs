//! PatternGraph: typed container for nodes, relationships, walks, and annotations.
//!
//! Ported from `Pattern.PatternGraph` in the Haskell reference implementation.
//! Patterns are routed into six typed collections by a `GraphClassifier`.
//! Duplicate identities are resolved via `ReconciliationPolicy`.

use std::collections::HashMap;

use crate::graph::graph_classifier::{GraphClass, GraphClassifier, GraphValue};
use crate::graph::graph_query::GraphQuery;
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
                    g.pg_other.insert(i, (existing_extra, merged));
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

// ============================================================================
// GraphQuery constructor
// ============================================================================

/// Wraps a `PatternGraph` in a `GraphQuery<V>`.
///
/// All nine `GraphQuery` fields are implemented against the `PatternGraph` maps.
///
/// # Complexity
///
/// - `query_nodes` / `query_relationships`: O(n) / O(r) to collect values
/// - `query_incident_rels`: O(r) scan of all relationships
/// - `query_source` / `query_target`: O(1) element access
/// - `query_degree`: O(r) scan
/// - `query_node_by_id` / `query_relationship_by_id`: O(log n) / O(log r) HashMap lookup
/// - `query_containers`: O(r + w + a) scan of relationships, walks, annotations
///
/// # Deferred
///
/// TODO: `from_graph_lens` — deferred until `GraphLens` type is available in pattern-rs.
#[cfg(not(feature = "thread-safe"))]
pub fn from_pattern_graph<Extra, V>(graph: std::rc::Rc<PatternGraph<Extra, V>>) -> GraphQuery<V>
where
    Extra: 'static,
    V: GraphValue + Clone + 'static,
    V::Id: Clone + Eq + std::hash::Hash + 'static,
{
    use std::rc::Rc;

    let g1 = Rc::clone(&graph);
    let query_nodes = Rc::new(move || g1.pg_nodes.values().cloned().collect());

    let g2 = Rc::clone(&graph);
    let query_relationships = Rc::new(move || g2.pg_relationships.values().cloned().collect());

    let g3 = Rc::clone(&graph);
    let query_incident_rels = Rc::new(move |node: &Pattern<V>| {
        let node_id = node.value.identify();
        g3.pg_relationships
            .values()
            .filter(|rel| {
                rel.elements.len() == 2
                    && (rel.elements[0].value.identify() == node_id
                        || rel.elements[1].value.identify() == node_id)
            })
            .cloned()
            .collect()
    });

    // query_source and query_target read directly from relationship elements (O(1))
    let query_source = Rc::new(|rel: &Pattern<V>| rel.elements.first().cloned());
    let query_target = Rc::new(|rel: &Pattern<V>| rel.elements.get(1).cloned());

    let g4 = Rc::clone(&graph);
    let query_degree = Rc::new(move |node: &Pattern<V>| {
        let node_id = node.value.identify();
        g4.pg_relationships
            .values()
            .filter(|rel| {
                rel.elements.len() == 2
                    && (rel.elements[0].value.identify() == node_id
                        || rel.elements[1].value.identify() == node_id)
            })
            .count()
    });

    let g5 = Rc::clone(&graph);
    let query_node_by_id = Rc::new(move |id: &V::Id| g5.pg_nodes.get(id).cloned());

    let g6 = Rc::clone(&graph);
    let query_relationship_by_id = Rc::new(move |id: &V::Id| g6.pg_relationships.get(id).cloned());

    let g7 = Rc::clone(&graph);
    let query_containers = Rc::new(move |element: &Pattern<V>| {
        let elem_id = element.value.identify();
        let mut containers = Vec::new();

        // Relationships: element is an endpoint (source or target)
        for rel in g7.pg_relationships.values() {
            if rel.elements.len() == 2
                && (rel.elements[0].value.identify() == elem_id
                    || rel.elements[1].value.identify() == elem_id)
            {
                containers.push(rel.clone());
            }
        }

        // Walks: element is one of the walk's direct sub-elements
        for walk in g7.pg_walks.values() {
            if walk.elements.iter().any(|e| e.value.identify() == elem_id) {
                containers.push(walk.clone());
            }
        }

        // Annotations: element is the single inner element
        for ann in g7.pg_annotations.values() {
            if ann.elements.len() == 1 && ann.elements[0].value.identify() == elem_id {
                containers.push(ann.clone());
            }
        }

        containers
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
pub fn from_pattern_graph<Extra, V>(graph: std::sync::Arc<PatternGraph<Extra, V>>) -> GraphQuery<V>
where
    Extra: Send + Sync + 'static,
    V: GraphValue + Clone + Send + Sync + 'static,
    V::Id: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
{
    use std::sync::Arc;

    let g1 = Arc::clone(&graph);
    let query_nodes = Arc::new(move || g1.pg_nodes.values().cloned().collect());

    let g2 = Arc::clone(&graph);
    let query_relationships = Arc::new(move || g2.pg_relationships.values().cloned().collect());

    let g3 = Arc::clone(&graph);
    let query_incident_rels = Arc::new(move |node: &Pattern<V>| {
        let node_id = node.value.identify();
        g3.pg_relationships
            .values()
            .filter(|rel| {
                rel.elements.len() == 2
                    && (rel.elements[0].value.identify() == node_id
                        || rel.elements[1].value.identify() == node_id)
            })
            .cloned()
            .collect()
    });

    let query_source = Arc::new(|rel: &Pattern<V>| rel.elements.first().cloned());
    let query_target = Arc::new(|rel: &Pattern<V>| rel.elements.get(1).cloned());

    let g4 = Arc::clone(&graph);
    let query_degree = Arc::new(move |node: &Pattern<V>| {
        let node_id = node.value.identify();
        g4.pg_relationships
            .values()
            .filter(|rel| {
                rel.elements.len() == 2
                    && (rel.elements[0].value.identify() == node_id
                        || rel.elements[1].value.identify() == node_id)
            })
            .count()
    });

    let g5 = Arc::clone(&graph);
    let query_node_by_id = Arc::new(move |id: &V::Id| g5.pg_nodes.get(id).cloned());

    let g6 = Arc::clone(&graph);
    let query_relationship_by_id = Arc::new(move |id: &V::Id| g6.pg_relationships.get(id).cloned());

    let g7 = Arc::clone(&graph);
    let query_containers = Arc::new(move |element: &Pattern<V>| {
        let elem_id = element.value.identify();
        let mut containers = Vec::new();

        for rel in g7.pg_relationships.values() {
            if rel.elements.len() == 2
                && (rel.elements[0].value.identify() == elem_id
                    || rel.elements[1].value.identify() == elem_id)
            {
                containers.push(rel.clone());
            }
        }

        for walk in g7.pg_walks.values() {
            if walk.elements.iter().any(|e| e.value.identify() == elem_id) {
                containers.push(walk.clone());
            }
        }

        for ann in g7.pg_annotations.values() {
            if ann.elements.len() == 1 && ann.elements[0].value.identify() == elem_id {
                containers.push(ann.clone());
            }
        }

        containers
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
