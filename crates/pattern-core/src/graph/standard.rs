//! StandardGraph: ergonomic wrapper around PatternGraph<(), Subject>.
//!
//! Provides zero-configuration graph construction and querying for the common
//! case of building graph structures from atomic patterns (nodes, relationships,
//! walks, annotations).

use std::collections::HashMap;

use crate::graph::graph_classifier::{canonical_classifier, GraphValue};
use crate::graph::graph_query::GraphQuery;
use crate::graph::graph_view::GraphView;
use crate::pattern::Pattern;
use crate::pattern_graph::PatternGraph;
use crate::subject::{Subject, Symbol};

/// A concrete, ergonomic graph type wrapping `PatternGraph<(), Subject>`.
///
/// StandardGraph eliminates the type parameters and classifier/policy boilerplate
/// needed when working with `PatternGraph` directly. It provides fluent construction
/// methods and graph-native queries.
///
/// # Examples
///
/// ```rust
/// use pattern_core::graph::StandardGraph;
/// use pattern_core::subject::Subject;
///
/// let mut g = StandardGraph::new();
/// g.add_node(Subject::build("alice").label("Person").done());
/// g.add_node(Subject::build("bob").label("Person").done());
/// g.add_relationship(
///     Subject::build("r1").label("KNOWS").done(),
///     &"alice".into(),
///     &"bob".into(),
/// );
/// assert_eq!(g.node_count(), 2);
/// assert_eq!(g.relationship_count(), 1);
/// ```
pub struct StandardGraph {
    inner: PatternGraph<(), Subject>,
}

impl StandardGraph {
    /// Creates an empty StandardGraph.
    pub fn new() -> Self {
        StandardGraph {
            inner: PatternGraph::empty(),
        }
    }

    // ========================================================================
    // Atomic element addition (Phase 3: US1)
    // ========================================================================

    /// Adds a node to the graph.
    ///
    /// The subject becomes an atomic pattern (no elements). If a node with the
    /// same identity already exists, it is replaced (last-write-wins).
    pub fn add_node(&mut self, subject: Subject) -> &mut Self {
        let id = subject.identity.clone();
        let pattern = Pattern::point(subject);
        self.inner.pg_nodes.insert(id, pattern);
        self
    }

    /// Adds a relationship to the graph.
    ///
    /// Creates a 2-element pattern with the source and target nodes as elements.
    /// If the source or target nodes don't exist yet, minimal placeholder nodes
    /// are created automatically.
    pub fn add_relationship(
        &mut self,
        subject: Subject,
        source: &Symbol,
        target: &Symbol,
    ) -> &mut Self {
        let source_pattern = self.get_or_create_placeholder_node(source);
        let target_pattern = self.get_or_create_placeholder_node(target);

        let id = subject.identity.clone();
        let pattern = Pattern::pattern(subject, vec![source_pattern, target_pattern]);
        self.inner.pg_relationships.insert(id, pattern);
        self
    }

    /// Adds a walk to the graph.
    ///
    /// Creates an N-element pattern where each element is a relationship pattern.
    /// If referenced relationships don't exist, minimal placeholders are created.
    pub fn add_walk(&mut self, subject: Subject, relationships: &[Symbol]) -> &mut Self {
        let rel_patterns: Vec<Pattern<Subject>> = relationships
            .iter()
            .map(|rel_id| self.get_or_create_placeholder_relationship(rel_id))
            .collect();

        let id = subject.identity.clone();
        let pattern = Pattern::pattern(subject, rel_patterns);
        self.inner.pg_walks.insert(id, pattern);
        self
    }

    /// Adds an annotation to the graph.
    ///
    /// Creates a 1-element pattern wrapping the referenced element.
    /// If the referenced element doesn't exist, a minimal placeholder node is created.
    pub fn add_annotation(&mut self, subject: Subject, element: &Symbol) -> &mut Self {
        let element_pattern = self
            .find_element(element)
            .unwrap_or_else(|| Self::make_placeholder_node(element));

        let id = subject.identity.clone();
        let pattern = Pattern::pattern(subject, vec![element_pattern]);
        self.inner.pg_annotations.insert(id, pattern);
        self
    }

    // ========================================================================
    // Pattern ingestion (Phase 4: US3)
    // ========================================================================

    /// Adds a single pattern, classifying it by shape and inserting into the
    /// appropriate bucket.
    pub fn add_pattern(&mut self, pattern: Pattern<Subject>) -> &mut Self {
        let classifier = canonical_classifier();
        self.inner = crate::pattern_graph::merge(
            &classifier,
            pattern,
            std::mem::replace(&mut self.inner, PatternGraph::empty()),
        );
        self
    }

    /// Adds multiple patterns, classifying each by shape.
    pub fn add_patterns(
        &mut self,
        patterns: impl IntoIterator<Item = Pattern<Subject>>,
    ) -> &mut Self {
        let classifier = canonical_classifier();
        let mut graph = std::mem::replace(&mut self.inner, PatternGraph::empty());
        for pattern in patterns {
            graph = crate::pattern_graph::merge(&classifier, pattern, graph);
        }
        self.inner = graph;
        self
    }

    /// Creates a StandardGraph from an iterator of patterns.
    pub fn from_patterns(patterns: impl IntoIterator<Item = Pattern<Subject>>) -> Self {
        let classifier = canonical_classifier();
        let inner = crate::pattern_graph::from_patterns(&classifier, patterns);
        StandardGraph { inner }
    }

    /// Creates a StandardGraph by wrapping an existing PatternGraph directly.
    pub fn from_pattern_graph(graph: PatternGraph<(), Subject>) -> Self {
        StandardGraph { inner: graph }
    }

    // ========================================================================
    // Element access (Phase 3: US1)
    // ========================================================================

    /// Returns the node with the given identity.
    pub fn node(&self, id: &Symbol) -> Option<&Pattern<Subject>> {
        self.inner.pg_nodes.get(id)
    }

    /// Returns the relationship with the given identity.
    pub fn relationship(&self, id: &Symbol) -> Option<&Pattern<Subject>> {
        self.inner.pg_relationships.get(id)
    }

    /// Returns the walk with the given identity.
    pub fn walk(&self, id: &Symbol) -> Option<&Pattern<Subject>> {
        self.inner.pg_walks.get(id)
    }

    /// Returns the annotation with the given identity.
    pub fn annotation(&self, id: &Symbol) -> Option<&Pattern<Subject>> {
        self.inner.pg_annotations.get(id)
    }

    // ========================================================================
    // Counts and health (Phase 3: US1)
    // ========================================================================

    /// Returns the number of nodes.
    pub fn node_count(&self) -> usize {
        self.inner.pg_nodes.len()
    }

    /// Returns the number of relationships.
    pub fn relationship_count(&self) -> usize {
        self.inner.pg_relationships.len()
    }

    /// Returns the number of walks.
    pub fn walk_count(&self) -> usize {
        self.inner.pg_walks.len()
    }

    /// Returns the number of annotations.
    pub fn annotation_count(&self) -> usize {
        self.inner.pg_annotations.len()
    }

    /// Returns true if the graph has no elements in any bucket.
    pub fn is_empty(&self) -> bool {
        self.inner.pg_nodes.is_empty()
            && self.inner.pg_relationships.is_empty()
            && self.inner.pg_walks.is_empty()
            && self.inner.pg_annotations.is_empty()
            && self.inner.pg_other.is_empty()
    }

    /// Returns true if any reconciliation conflicts have been recorded.
    pub fn has_conflicts(&self) -> bool {
        !self.inner.pg_conflicts.is_empty()
    }

    /// Returns the conflict map (identity → conflicting patterns).
    pub fn conflicts(&self) -> &HashMap<Symbol, Vec<Pattern<Subject>>> {
        &self.inner.pg_conflicts
    }

    /// Returns the "other" bucket (unclassifiable patterns).
    pub fn other(&self) -> &HashMap<Symbol, ((), Pattern<Subject>)> {
        &self.inner.pg_other
    }

    // ========================================================================
    // Iterators (Phase 5: US4)
    // ========================================================================

    /// Iterates over all nodes.
    pub fn nodes(&self) -> impl Iterator<Item = (&Symbol, &Pattern<Subject>)> {
        self.inner.pg_nodes.iter()
    }

    /// Iterates over all relationships.
    pub fn relationships(&self) -> impl Iterator<Item = (&Symbol, &Pattern<Subject>)> {
        self.inner.pg_relationships.iter()
    }

    /// Iterates over all walks.
    pub fn walks(&self) -> impl Iterator<Item = (&Symbol, &Pattern<Subject>)> {
        self.inner.pg_walks.iter()
    }

    /// Iterates over all annotations.
    pub fn annotations(&self) -> impl Iterator<Item = (&Symbol, &Pattern<Subject>)> {
        self.inner.pg_annotations.iter()
    }

    // ========================================================================
    // Graph-native queries (Phase 5: US4)
    // ========================================================================

    /// Returns the source node of a relationship.
    pub fn source(&self, rel_id: &Symbol) -> Option<&Pattern<Subject>> {
        self.inner
            .pg_relationships
            .get(rel_id)
            .and_then(|rel| rel.elements.first())
    }

    /// Returns the target node of a relationship.
    pub fn target(&self, rel_id: &Symbol) -> Option<&Pattern<Subject>> {
        self.inner
            .pg_relationships
            .get(rel_id)
            .and_then(|rel| rel.elements.get(1))
    }

    /// Returns all neighbor nodes of the given node (both directions).
    pub fn neighbors(&self, node_id: &Symbol) -> Vec<&Pattern<Subject>> {
        let mut result = Vec::new();
        for rel in self.inner.pg_relationships.values() {
            if rel.elements.len() == 2 {
                let src_id = rel.elements[0].value.identify();
                let tgt_id = rel.elements[1].value.identify();
                if src_id == node_id {
                    result.push(&rel.elements[1]);
                } else if tgt_id == node_id {
                    result.push(&rel.elements[0]);
                }
            }
        }
        result
    }

    /// Returns the degree of a node (number of incident relationships, both directions).
    pub fn degree(&self, node_id: &Symbol) -> usize {
        self.inner
            .pg_relationships
            .values()
            .filter(|rel| {
                rel.elements.len() == 2
                    && (rel.elements[0].value.identify() == node_id
                        || rel.elements[1].value.identify() == node_id)
            })
            .count()
    }

    // ========================================================================
    // Escape hatches (Phase 6: US5)
    // ========================================================================

    /// Returns a reference to the inner PatternGraph.
    pub fn as_pattern_graph(&self) -> &PatternGraph<(), Subject> {
        &self.inner
    }

    /// Consumes the StandardGraph and returns the inner PatternGraph.
    pub fn into_pattern_graph(self) -> PatternGraph<(), Subject> {
        self.inner
    }

    /// Creates a GraphQuery from this graph.
    #[cfg(not(feature = "thread-safe"))]
    pub fn as_query(&self) -> GraphQuery<Subject> {
        use std::rc::Rc;
        let graph = Rc::new(PatternGraph {
            pg_nodes: self.inner.pg_nodes.clone(),
            pg_relationships: self.inner.pg_relationships.clone(),
            pg_walks: self.inner.pg_walks.clone(),
            pg_annotations: self.inner.pg_annotations.clone(),
            pg_other: self.inner.pg_other.clone(),
            pg_conflicts: self.inner.pg_conflicts.clone(),
        });
        crate::pattern_graph::from_pattern_graph(graph)
    }

    /// Creates a GraphQuery from this graph.
    #[cfg(feature = "thread-safe")]
    pub fn as_query(&self) -> GraphQuery<Subject> {
        use std::sync::Arc;
        let graph = Arc::new(PatternGraph {
            pg_nodes: self.inner.pg_nodes.clone(),
            pg_relationships: self.inner.pg_relationships.clone(),
            pg_walks: self.inner.pg_walks.clone(),
            pg_annotations: self.inner.pg_annotations.clone(),
            pg_other: self.inner.pg_other.clone(),
            pg_conflicts: self.inner.pg_conflicts.clone(),
        });
        crate::pattern_graph::from_pattern_graph(graph)
    }

    /// Creates a GraphView snapshot from this graph.
    pub fn as_snapshot(&self) -> GraphView<(), Subject> {
        let classifier = canonical_classifier();
        crate::graph::graph_view::from_pattern_graph(&classifier, &self.inner)
    }

    // ========================================================================
    // Private helpers
    // ========================================================================

    fn make_placeholder_node(id: &Symbol) -> Pattern<Subject> {
        Pattern::point(Subject {
            identity: id.clone(),
            labels: std::collections::HashSet::new(),
            properties: HashMap::new(),
        })
    }

    fn get_or_create_placeholder_node(&mut self, id: &Symbol) -> Pattern<Subject> {
        if let Some(node) = self.inner.pg_nodes.get(id) {
            node.clone()
        } else {
            let placeholder = Self::make_placeholder_node(id);
            self.inner.pg_nodes.insert(id.clone(), placeholder.clone());
            placeholder
        }
    }

    fn get_or_create_placeholder_relationship(&self, id: &Symbol) -> Pattern<Subject> {
        if let Some(rel) = self.inner.pg_relationships.get(id) {
            rel.clone()
        } else {
            // Create a minimal placeholder relationship (just a point with the id)
            Pattern::point(Subject {
                identity: id.clone(),
                labels: std::collections::HashSet::new(),
                properties: HashMap::new(),
            })
        }
    }

    fn find_element(&self, id: &Symbol) -> Option<Pattern<Subject>> {
        self.inner
            .pg_nodes
            .get(id)
            .or_else(|| self.inner.pg_relationships.get(id))
            .or_else(|| self.inner.pg_walks.get(id))
            .or_else(|| self.inner.pg_annotations.get(id))
            .cloned()
    }
}

impl Default for StandardGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&str> for Symbol {
    fn from(s: &str) -> Self {
        Symbol(s.to_string())
    }
}

impl From<String> for Symbol {
    fn from(s: String) -> Self {
        Symbol(s)
    }
}
