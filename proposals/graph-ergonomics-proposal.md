# Graph Ergonomics Proposal

**Status**: Draft  
**Date**: 2026-03-02  
**Target**: pattern-core crate graph module

## Summary

Improve developer experience for graph construction, querying, and transformation by introducing builder patterns, ergonomic constructors, and clearer naming. This proposal reorganizes the graph module for better coherence while maintaining the existing abstraction architecture.

## Goals

1. **Easier graph construction** - Builder API reduces boilerplate for common cases
2. **Ergonomic Subject building** - Fluent API for Subject construction
3. **Intuitive PatternGraph API** - Methods over free functions, with sensible defaults
4. **Clearer naming** - Reflect actual semantics (GraphSnapshot vs GraphView)
5. **Coherent module structure** - Group related functionality

## Non-Goals

- Change the underlying abstraction architecture (Pattern<V>, GraphQuery, PatternGraph relationships)
- Add new graph algorithms
- Modify graph transformation semantics
- Support GraphLens (deferred to future feature)

## Current Architecture (Preserved)

```
Pattern<V> ──► PatternGraph (materialized storage)
                  │
                  ▼ (as_query)
              GraphQuery (read-only query interface)
                  │
              ┌───┴───┐
              ▼       ▼
        Algorithms  GraphSnapshot
                        │
                        ▼
                  Transformations ──► PatternGraph (materialize)
```

## Detailed Design

### 1. Module Reorganization (Option B)

**New structure:**

```
crates/pattern-core/src/graph/
├── mod.rs              # Core exports, type aliases
├── prelude.rs          # "use graph::prelude::*" convenience
├── builder.rs          # GraphBuilder, SubjectBuilder
├── construct.rs        # Convenience constructors for Pattern<Subject>
├── classifier.rs       # GraphClassifier, GraphClass (renamed from graph_classifier.rs)
├── query.rs            # GraphQuery and methods (renamed from graph_query.rs)
├── snapshot.rs         # GraphSnapshot (renamed from graph_view.rs)
├── storage.rs          # PatternGraph (moved from pattern_graph.rs)
├── algorithms.rs       # Traversal, path, centrality (keep existing)
└── transform/
    ├── mod.rs
    ├── types.rs
    ├── map_filter_fold.rs
    ├── para.rs
    ├── unfold_graph.rs
    └── context.rs
```

**Export structure in `graph/mod.rs`:**

```rust
// Type aliases for common case (Subject, no extra data)
pub type SubjectGraph = PatternGraph<(), Subject>;
pub type SubjectSnapshot = GraphSnapshot<(), Subject>;
pub type SubjectQuery = GraphQuery<Subject>;
pub type SubjectClassifier = GraphClassifier<(), Subject>;

// Re-exports from submodules
pub use builder::{GraphBuilder, SubjectBuilder};
pub use construct::{node, relationship, annotation, walk};
pub use classifier::{GraphClass, GraphClassifier, GraphValue, classify_by_shape, canonical_classifier};
pub use query::{GraphQuery, TraversalDirection, TraversalWeight, undirected, directed, directed_reverse};
pub use snapshot::{GraphSnapshot, materialize};
pub use storage::{PatternGraph};

// Algorithms remain as free functions
pub use algorithms::{bfs, dfs, shortest_path, /* ... */};

// Transform functions
pub use transform::{map_graph, filter_graph, fold_graph, /* ... */};

// Prelude module
pub mod prelude {
    pub use super::{GraphBuilder, SubjectBuilder, SubjectGraph, SubjectSnapshot, SubjectQuery};
    pub use super::{node, relationship, annotation, walk};
    pub use super::{canonical_classifier, undirected, directed};
}
```

### 2. Builder-Style API for Graph Construction

**`GraphBuilder`** provides fluent construction with deferred materialization:

```rust
pub struct GraphBuilder<Extra, V: GraphValue> {
    classifier: GraphClassifier<Extra, V>,
    policy: ReconciliationPolicy<V::MergeStrategy>,
    nodes: Vec<Pattern<V>>,
    relationships: Vec<(V::Id, V::Id, Pattern<V>)>, // (source_id, target_id, rel_pattern)
    walks: Vec<(Vec<V::Id>, Pattern<V>)>, // (rel_ids, walk_pattern)
}

impl GraphBuilder<(), Subject> {
    /// Create builder with canonical classifier and LastWriteWins policy
    pub fn new() -> Self;
}

impl<Extra, V: GraphValue> GraphBuilder<Extra, V> {
    /// Create with custom classifier and policy
    pub fn with_config(
        classifier: GraphClassifier<Extra, V>,
        policy: ReconciliationPolicy<V::MergeStrategy>
    ) -> Self;
    
    /// Add a node using SubjectBuilder closure
    pub fn node(
        mut self,
        id: impl Into<Symbol>,
        f: impl FnOnce(SubjectBuilder) -> Subject
    ) -> Self;
    
    /// Add a node with pre-built Subject
    pub fn node_subject(mut self, subject: Subject) -> Self;
    
    /// Add relationship using SubjectBuilder closure
    /// Panics if source/target not yet added (checked at build time)
    pub fn relationship(
        mut self,
        id: impl Into<Symbol>,
        source_id: impl Into<Symbol>,
        target_id: impl Into<Symbol>,
        f: impl FnOnce(SubjectBuilder) -> Subject
    ) -> Self;
    
    /// Add walk (sequence of relationship IDs)
    pub fn walk(
        mut self,
        id: impl Into<Symbol>,
        relationship_ids: &[impl AsRef<str>],
        f: impl FnOnce(SubjectBuilder) -> Subject
    ) -> Self;
    
    /// Add annotation (wraps a single element)
    pub fn annotation(
        mut self,
        id: impl Into<Symbol>,
        element_id: impl Into<Symbol>,
        f: impl FnOnce(SubjectBuilder) -> Subject
    ) -> Self;
    
    /// Build into PatternGraph
    /// Resolves all references, applies reconciliation policy for duplicates
    pub fn build(self) -> PatternGraph<Extra, V>;
    
    /// Build and create snapshot in one step
    pub fn build_snapshot(self) -> GraphSnapshot<Extra, V>;
}
```

**Usage example:**

```rust
use pattern_core::graph::{GraphBuilder, SubjectBuilder};
use pattern_core::graph::prelude::*;

let graph = GraphBuilder::new()
    .node("alice", |s| s.label("Person").property("name", "Alice"))
    .node("bob", |s| s.label("Person").property("name", "Bob"))
    .relationship("r1", "alice", "bob", |s| s.label("KNOWS").property("since", 2020))
    .walk("path1", &["r1"], |s| s.label("PATH"))
    .build();
```

### 3. Subject Constructor Helpers

**`SubjectBuilder`** for fluent Subject construction:

```rust
pub struct SubjectBuilder {
    identity: Symbol,
    labels: HashSet<String>,
    properties: HashMap<String, Value>,
}

impl SubjectBuilder {
    pub fn new(id: impl Into<Symbol>) -> Self;
    
    pub fn label(mut self, label: impl Into<String>) -> Self;
    pub fn labels(mut self, labels: &[impl AsRef<str>]) -> Self;
    
    pub fn property(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self;
    
    /// Chain multiple properties
    pub fn properties(mut self, props: impl IntoIterator<Item = (String, Value)>) -> Self;
    
    pub fn build(self) -> Subject;
    
    /// Auto-convert to Subject when builder is complete
    pub fn into_subject(self) -> Subject { self.build() }
}

/// Convenience trait for value conversions
pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for i64 { ... }
impl IntoValue for f64 { ... }
impl IntoValue for bool { ... }
impl IntoValue for String { ... }
impl IntoValue for &str { ... }
impl<T: IntoValue> IntoValue for Vec<T> { ... } // VArray
impl<T: IntoValue> IntoValue for HashMap<String, T> { ... } // VMap
```

**Usage:**

```rust
let subject = Subject::new("alice")
    .label("Person")
    .label("Employee")
    .property("name", "Alice Smith")
    .property("age", 30)
    .property("skills", vec!["Rust", "Graphs"])
    .build();
```

### 4. PatternGraph with Merge Semantics

PatternGraph stores optional default classifier and policy, allowing merge methods to use defaults or accept overrides:

```rust
pub struct PatternGraph<Extra, V: GraphValue> {
    // Existing storage fields
    pub pg_nodes: HashMap<V::Id, Pattern<V>>,
    pub pg_relationships: HashMap<V::Id, Pattern<V>>,
    pub pg_walks: HashMap<V::Id, Pattern<V>>,
    pub pg_annotations: HashMap<V::Id, Pattern<V>>,
    pub pg_other: HashMap<V::Id, (Extra, Pattern<V>)>,
    pub pg_conflicts: HashMap<V::Id, Vec<Pattern<V>>>,
    
    // Optional defaults for merge operations
    default_classifier: Option<GraphClassifier<Extra, V>>,
    default_policy: Option<ReconciliationPolicy<V::MergeStrategy>>,
}

impl<Extra, V: GraphValue> PatternGraph<Extra, V> {
    /// Empty graph with no defaults
    pub fn empty() -> Self;
    
    /// Create with default classifier and policy
    pub fn with_defaults(
        classifier: GraphClassifier<Extra, V>,
        policy: ReconciliationPolicy<V::MergeStrategy>
    ) -> Self;
    
    /// Set defaults after construction
    pub fn set_defaults(
        &mut self,
        classifier: GraphClassifier<Extra, V>,
        policy: ReconciliationPolicy<V::MergeStrategy>
    ) -> &mut Self;
    
    // === Merge operations with stored defaults ===
    
    /// Merge node using default classifier/policy
    /// Panics if defaults not set
    pub fn merge_node(mut self, node: Pattern<V>) -> Self;
    
    /// Merge relationship using default classifier/policy
    /// Panics if defaults not set
    pub fn merge_relationship(mut self, rel: Pattern<V>) -> Self;
    
    /// Merge with classification (uses default policy)
    /// Panics if default policy not set
    pub fn merge_with_class(mut self, classifier: &GraphClassifier<Extra, V>, pattern: Pattern<V>) -> Self;
    
    // === Merge operations with explicit parameters ===
    
    pub fn merge_node_with_policy(
        mut self,
        classifier: &GraphClassifier<Extra, V>,
        policy: &ReconciliationPolicy<V::MergeStrategy>,
        node: Pattern<V>
    ) -> Self;
    
    pub fn merge_relationship_with_policy(
        mut self,
        classifier: &GraphClassifier<Extra, V>,
        policy: &ReconciliationPolicy<V::MergeStrategy>,
        rel: Pattern<V>
    ) -> Self;
    
    /// Generic merge with full control
    pub fn merge_with_policy(
        self,
        classifier: &GraphClassifier<Extra, V>,
        policy: &ReconciliationPolicy<V::MergeStrategy>,
        pattern: Pattern<V>
    ) -> Self;
    
    // === Queries ===
    
    pub fn is_empty(&self) -> bool;
    pub fn len(&self) -> usize;  // Total across all collections
    pub fn node_count(&self) -> usize;
    pub fn relationship_count(&self) -> usize;
    pub fn walk_count(&self) -> usize;
    pub fn annotation_count(&self) -> usize;
    
    pub fn get_node(&self, id: &V::Id) -> Option<&Pattern<V>>;
    pub fn get_relationship(&self, id: &V::Id) -> Option<&Pattern<V>>;
    pub fn get_walk(&self, id: &V::Id) -> Option<&Pattern<V>>;
    pub fn get_annotation(&self, id: &V::Id) -> Option<&Pattern<V>>;
    
    // === Conversions ===
    
    /// Create read-only query interface
    pub fn as_query(&self) -> GraphQuery<V>;
    
    /// Create transformation snapshot
    pub fn as_snapshot(&self, classifier: &GraphClassifier<Extra, V>) -> GraphSnapshot<Extra, V>;
}

// FromIterator support for ergonomic construction
impl<Extra, V: GraphValue> FromIterator<Pattern<V>> for PatternGraph<Extra, V> {
    fn from_iter<I: IntoIterator<Item = Pattern<V>>>(iter: I) -> Self;
}
```

**Usage patterns:**

```rust
// With defaults
let graph = PatternGraph::with_defaults(classifier, policy)
    .merge_node(node_a)
    .merge_node(node_b)
    .merge_relationship(rel_ab);

// Without defaults (explicit every time)
let graph = PatternGraph::empty()
    .merge_node_with_policy(&classifier, &policy, node_a)
    .merge_relationship_with_policy(&classifier, &policy, rel_ab);

// From iterator
let patterns: Vec<Pattern<Subject>> = vec![node_a, node_b, rel_ab];
let graph: SubjectGraph = patterns.into_iter().collect();
```

### 5. GraphSnapshot (formerly GraphView)

Renamed to emphasize its purpose as a snapshot for transformation:

```rust
/// A snapshot of a classified graph for transformation operations.
/// 
/// Combines a read-only query interface with a flat list of classified elements.
/// All graph transformations consume a snapshot and produce a new one.
/// Use `materialize` to convert back to PatternGraph.
pub struct GraphSnapshot<Extra, V: GraphValue> {
    pub snapshot_query: GraphQuery<V>,
    pub snapshot_elements: Vec<(GraphClass<Extra>, Pattern<V>)>,
}

impl<Extra: Clone, V: GraphValue + Clone> Clone for GraphSnapshot<Extra, V> { ... }

/// Build snapshot from PatternGraph
pub fn from_pattern_graph<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    graph: &PatternGraph<Extra, V>,
) -> GraphSnapshot<Extra, V>;

/// Build snapshot from GraphLens (deferred)
pub fn from_graph_lens<Extra, V, L>(
    _classifier: &GraphClassifier<Extra, V>,
    _lens: L,
) -> GraphSnapshot<Extra, V> {
    unimplemented!("GraphLens not yet ported")
}

/// Consume snapshot and produce PatternGraph
pub fn materialize<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    snapshot: GraphSnapshot<Extra, V>,
) -> PatternGraph<Extra, V>;
```

### 6. Convenience Constructors for Pattern<Subject>

Graph-oriented constructors for the common `Pattern<Subject>` case:

```rust
// In graph/construct.rs

/// Create a node pattern (atomic, no elements)
pub fn node(subject: Subject) -> Pattern<Subject> {
    Pattern::point(subject)
}

/// Create a relationship pattern connecting source and target
pub fn relationship(
    subject: Subject,
    source: &Pattern<Subject>,
    target: &Pattern<Subject>,
) -> Pattern<Subject> {
    Pattern {
        value: subject,
        elements: vec![source.clone(), target.clone()],
    }
}

/// Create an annotation pattern wrapping a single inner element
pub fn annotation(
    subject: Subject,
    inner: &Pattern<Subject>,
) -> Pattern<Subject> {
    Pattern {
        value: subject,
        elements: vec![inner.clone()],
    }
}

/// Create a walk pattern from component relationships
pub fn walk(
    subject: Subject,
    relationships: &[Pattern<Subject>],
) -> Pattern<Subject> {
    Pattern {
        value: subject,
        elements: relationships.to_vec(),
    }
}

// Alternative: methods on Pattern<Subject>
impl Pattern<Subject> {
    pub fn as_node(subject: Subject) -> Self { Self::point(subject) }
    pub fn as_relationship(subject: Subject, source: &Self, target: &Self) -> Self { ... }
    pub fn as_annotation(subject: Subject, inner: &Self) -> Self { ... }
    pub fn as_walk(subject: Subject, relationships: &[Self]) -> Self { ... }
}
```

**Recommendation:** Use free functions `node()`, `relationship()`, etc. - they read naturally in graph construction contexts and signal the graph-oriented use case clearly.

### 7. Type Aliases for Common Cases

In `graph/mod.rs`:

```rust
/// PatternGraph with Subject values and no extra metadata
pub type SubjectGraph = PatternGraph<(), Subject>;

/// GraphSnapshot for Subject-based graphs
pub type SubjectSnapshot = GraphSnapshot<(), Subject>;

/// GraphQuery for Subject values
pub type SubjectQuery = GraphQuery<Subject>;

/// GraphClassifier with no extra payload for Subject
pub type SubjectClassifier = GraphClassifier<(), Subject>;

/// TraversalWeight for Subject values
pub type SubjectTraversalWeight = TraversalWeight<Subject>;
```

### 8. GraphQuery Ergonomics

**Current approach:** GraphQuery is a struct-of-closures. **Open question:** Should this become a trait?

**Option A: Keep struct-of-closures, add ergonomic methods**

```rust
impl<V: GraphValue> GraphQuery<V> {
    /// Returns all nodes
    pub fn nodes(&self) -> Vec<Pattern<V>> { (self.query_nodes)() }
    
    /// Returns all relationships
    pub fn relationships(&self) -> Vec<Pattern<V>> { (self.query_relationships)() }
    
    /// Returns relationships incident to node
    pub fn incident_rels(&self, node: &Pattern<V>) -> Vec<Pattern<V>> { (self.query_incident_rels)(node) }
    
    /// Returns source node of relationship
    pub fn source(&self, rel: &Pattern<V>) -> Option<Pattern<V>> { (self.query_source)(rel) }
    
    /// Returns target node of relationship
    pub fn target(&self, rel: &Pattern<V>) -> Option<Pattern<V>> { (self.query_target)(rel) }
    
    /// Returns node degree
    pub fn degree(&self, node: &Pattern<V>) -> usize { (self.query_degree)(node) }
    
    /// Returns node by ID
    pub fn node_by_id(&self, id: &V::Id) -> Option<Pattern<V>> { (self.query_node_by_id)(id) }
    
    /// Returns relationship by ID
    pub fn relationship_by_id(&self, id: &V::Id) -> Option<Pattern<V>> { (self.query_relationship_by_id)(id) }
    
    /// Returns direct containers of element
    pub fn containers(&self, element: &Pattern<V>) -> Vec<Pattern<V>> { (self.query_containers)(element) }
    
    /// Create filtered view of this query
    pub fn filter(self, predicate: impl Fn(&Pattern<V>) -> bool + 'static) -> Self 
    where Self: Sized {
        frame_query(Rc::new(predicate), self)
    }
    
    /// Create memoized version of this query
    pub fn memoized(self) -> Self where Self: Sized {
        memoize_incident_rels(self)
    }
}
```

**Option B: Trait-based interface (more idiomatic?)**

```rust
pub trait GraphRead<V: GraphValue> {
    fn nodes(&self) -> Vec<Pattern<V>>;
    fn relationships(&self) -> Vec<Pattern<V>>;
    fn incident_rels(&self, node: &Pattern<V>) -> Vec<Pattern<V>>;
    fn source(&self, rel: &Pattern<V>) -> Option<Pattern<V>>;
    fn target(&self, rel: &Pattern<V>) -> Option<Pattern<V>>;
    fn degree(&self, node: &Pattern<V>) -> usize;
    fn node_by_id(&self, id: &V::Id) -> Option<Pattern<V>>;
    fn relationship_by_id(&self, id: &V::Id) -> Option<Pattern<V>>;
    fn containers(&self, element: &Pattern<V>) -> Vec<Pattern<V>>;
}

// GraphQuery implements GraphRead
impl<V: GraphValue> GraphRead<V> for GraphQuery<V> { ... }

// Future: GraphLens can implement GraphRead
// impl<V: GraphValue> GraphRead<V> for GraphLens<V> { ... }

// Algorithms become generic over GraphRead
pub fn bfs<V: GraphValue>(
    graph: &impl GraphRead<V>,
    weight: &TraversalWeight<V>,
    start: &Pattern<V>,
) -> Vec<Pattern<V>>;
```

**Considerations:**
- **Option A** maintains backward compatibility and the ability to clone GraphQuery cheaply
- **Option B** is more idiomatic Rust but requires trait objects or generics throughout algorithms
- **Hybrid:** Keep GraphQuery struct, implement trait for it, algorithms accept impl Trait

**Recommendation:** Start with Option A (methods on GraphQuery). Consider Option B (GraphRead trait) as future enhancement when GraphLens is ported.

## Implementation Plan

### Phase 1: Core Structure (no breaking changes)
1. Create new files: `builder.rs`, `construct.rs`, `storage.rs`
2. Add `SubjectBuilder` implementation
3. Add `GraphBuilder` implementation
4. Add convenience constructors (`node()`, `relationship()`, etc.)
5. Add type aliases to `graph/mod.rs`

### Phase 2: PatternGraph API Enhancement
1. Move `PatternGraph` to `storage.rs` with new methods
2. Add `as_query()` method (keep `from_pattern_graph` as deprecated alias)
3. Implement `FromIterator` for PatternGraph
4. Update tests to use new API

### Phase 3: Renaming (breaking changes)
1. Rename `graph_view.rs` → `snapshot.rs`, `GraphView` → `GraphSnapshot`
2. Rename `graph_query.rs` → `query.rs` (file only, type name unchanged)
3. Rename `graph_classifier.rs` → `classifier.rs`
4. Update all internal references

### Phase 4: GraphQuery Ergonomics
1. Add ergonomic accessor methods to `GraphQuery`
2. Move `frame_query` and `memoize_incident_rels` to methods
3. Update algorithm internal usage

### Phase 5: Cleanup
1. Remove deprecated `from_pattern_graph` free function
2. Remove old file aliases
3. Update documentation and examples
4. Update Python/TypeScript bindings

## Backward Compatibility

### Breaking Changes (Phase 3)
- `GraphView` → `GraphSnapshot` (type rename)
- `graph_view.rs` module path changes
- `from_pattern_graph` free function removed (use `PatternGraph::as_query`)

### Migration Guide
```rust
// Before
use pattern_core::{GraphView, from_pattern_graph, from_patterns};
let view = from_pattern_graph(&classifier, &graph);

// After
use pattern_core::graph::{GraphSnapshot, from_pattern_graph as build_snapshot};
// Or with prelude:
use pattern_core::graph::prelude::*;
let snapshot = graph.as_snapshot(&classifier);
```

## Open Questions

1. **GraphQuery as trait?** Should we introduce a `GraphRead` trait for algorithms, or keep struct-of-closures with methods?

2. **Error handling in GraphBuilder?** Should invalid references (unknown node IDs in relationships) return `Result` or panic? Builder pattern typically panics on build, but Result is more Rust-idiomatic.

3. **GraphMutation interface?** Future work for mutable graph operations. Keep separate from read-only GraphQuery.

4. **GraphLens timeline?** When ported, should it implement same interface as PatternGraph or require separate handling?

## Alternatives Considered

### Keep GraphView name
- **Pro:** Less churn
- **Con:** "View" suggests live/updatable view, but it's actually a snapshot for transformation

### Trait-based GraphQuery now
- **Pro:** More idiomatic, enables generic algorithms
- **Con:** Major refactor of all algorithm signatures, trait objects have overhead

### Don't reorganize files
- **Pro:** No breaking changes
- **Con:** Module structure remains inconsistent (`graph_classifier.rs` but `algorithms.rs`)

## References

- Original porting specs: `specs/030-graph-classifier/`, `specs/031-graph-query/`, `specs/032-graph-transform/`
- gram-hs reference: `../pattern-hs/libs/pattern/src/Pattern/Graph/`
- Current implementation: `crates/pattern-core/src/graph/`
