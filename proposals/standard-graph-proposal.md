# StandardGraph Proposal

**Status**: Draft
**Date**: 2026-03-15
**Target**: pattern-core crate, `graph/standard.rs`
**Supersedes**: Builder portions of `graph-ergonomics-proposal.md`

## Motivation

`PatternGraph<Extra, V>` is the correct generalization, but its type parameters, external classifier, and reconciliation policy create unnecessary friction for the common case: working with `Pattern<Subject>` graphs using the canonical classifier.

Graph-people coming from Neo4j, NetworkX, or similar systems think in terms of nodes and relationships. They shouldn't need to understand `GraphClassifier`, `ReconciliationPolicy`, or `GraphValue` trait bounds to build and query a graph.

`StandardGraph` is the **standard interpretation** of patterns as graph elements. It is concrete (no type parameters), opinionated (canonical classifier, last-write-wins), and complete (accepts everything gram notation can describe).

## Design Principles

1. **Concrete over parameterized** — No type parameters. Fixed to `Subject` values and canonical classification.
2. **Composition over reimplementation** — Wraps `PatternGraph<(), Subject>` internally. Reuses existing reconciliation, merge, and classification logic.
3. **Two construction modes** — Atomic construction (you know what you're adding) and pattern ingestion (gram notation or arbitrary patterns, classified automatically).
4. **Segregate, don't reject** — Unrecognizable patterns go to an "other" bucket. Conflicts accumulate. The graph is always constructable.
5. **Escape hatches** — Convert to/from `PatternGraph`, `GraphQuery`, `GraphSnapshot` when you need the abstract layer.

## Architecture

```
StandardGraph (concrete, ergonomic)
  │  wraps
  ▼
PatternGraph<(), Subject> (abstract, flexible)
  │
  │  .as_query()
  ▼
GraphQuery<Subject> / GraphSnapshot (read-only, transformations)
```

`StandardGraph` is the recommended entry point for most users. `PatternGraph` remains available for custom value types, custom classifiers, or non-standard reconciliation needs.

## Type Definition

```rust
pub struct StandardGraph {
    inner: PatternGraph<(), Subject>,
}
```

The canonical classifier (`classify_by_shape`) and `LastWriteWins` policy are used internally. No configuration is exposed in this initial implementation.

## Construction

### Atomic Construction

Classification is implicit in the method name. No classifier is invoked.

```rust
let mut g = StandardGraph::new();

// Nodes
g.add_node(subject);

// Relationships (references existing nodes by identity)
g.add_relationship(rel_subject, &source_id, &target_id);

// Walks (references existing relationships by identity)
g.add_walk(walk_subject, &[rel_id_1, rel_id_2]);

// Annotations (wraps a single existing element)
g.add_annotation(ann_subject, &element_id);
```

Atomic construction builds the correct `Pattern<Subject>` structure internally:
- `add_node` → `Pattern::point(subject)` stored in nodes
- `add_relationship` → `Pattern { value: subject, elements: [source, target] }` stored in relationships
- `add_walk` → `Pattern { value: subject, elements: [rel1, rel2, ...] }` stored in walks
- `add_annotation` → `Pattern { value: subject, elements: [inner] }` stored in annotations

When a referenced identity (e.g., source node for a relationship) doesn't exist in the graph yet, the method creates a minimal placeholder node with that identity. This matches how gram notation works — a relationship like `(a)-[:KNOWS]->(b)` implicitly introduces nodes `a` and `b`.

### Pattern Ingestion

For gram notation and arbitrary patterns. The canonical classifier routes each pattern to the correct bucket.

```rust
// From gram notation
let g = StandardGraph::from_gram("(alice:Person)-[:KNOWS]->(bob:Person)")?;

// Single pattern (classified automatically)
g.add_pattern(pattern);

// Bulk ingestion
g.add_patterns(vec![p1, p2, p3]);
```

Unrecognizable patterns (those classified as `GOther`) are stored in the `other` bucket, not rejected. Identity collisions that can't be reconciled are stored in `conflicts`.

### From Existing Structures

```rust
// From PatternGraph (re-wraps, no reclassification needed)
let g = StandardGraph::from_pattern_graph(pattern_graph);

// From patterns (classifies each via canonical classifier)
let g = StandardGraph::from_patterns(vec![p1, p2, p3]);
```

## Querying

### Element Access

```rust
g.node(&id) -> Option<&Pattern<Subject>>
g.relationship(&id) -> Option<&Pattern<Subject>>
g.walk(&id) -> Option<&Pattern<Subject>>
g.annotation(&id) -> Option<&Pattern<Subject>>

g.nodes() -> impl Iterator<Item = (&Symbol, &Pattern<Subject>)>
g.relationships() -> impl Iterator<Item = (&Symbol, &Pattern<Subject>)>
g.walks() -> impl Iterator<Item = (&Symbol, &Pattern<Subject>)>
g.annotations() -> impl Iterator<Item = (&Symbol, &Pattern<Subject>)>
```

### Graph-Native Queries

```rust
g.source(&rel_id) -> Option<&Pattern<Subject>>
g.target(&rel_id) -> Option<&Pattern<Subject>>
g.neighbors(&node_id) -> Vec<&Pattern<Subject>>
g.degree(&node_id) -> usize
```

Exact signatures and return types to be determined during implementation. These may delegate to `GraphQuery` internally or be implemented directly on the stored data.

### Counts and Health

```rust
g.node_count() -> usize
g.relationship_count() -> usize
g.walk_count() -> usize
g.annotation_count() -> usize
g.is_empty() -> bool

g.has_conflicts() -> bool
g.conflicts() -> &HashMap<Symbol, Vec<Pattern<Subject>>>
g.other() -> &HashMap<Symbol, ((), Pattern<Subject>)>
```

## Escape Hatches

```rust
// To PatternGraph
g.as_pattern_graph() -> &PatternGraph<(), Subject>
g.into_pattern_graph() -> PatternGraph<(), Subject>

// To query/snapshot layer
g.as_query() -> GraphQuery<Subject>
g.as_snapshot() -> GraphSnapshot<(), Subject>
```

These enable interop with existing algorithms, transformations, and any code that operates on the abstract types.

## SubjectBuilder

Fluent construction for `Subject` values, used standalone or with `StandardGraph`:

```rust
let subject = Subject::build("alice")
    .label("Person")
    .label("Employee")
    .property("name", "Alice Smith")
    .property("age", 30)
    .done();
```

`SubjectBuilder` is an independent utility, not specific to `StandardGraph`. It eliminates the `Symbol(s.to_string())` / `HashSet::new()` / `HashMap::new()` boilerplate that appears throughout the current test code.

Exact API (method names, trait bounds for value conversion, whether `build()` or `done()` or implicit `Into<Subject>`) to be determined during implementation.

## Relationship to Existing Code

### What Changes

- New file: `crates/pattern-core/src/graph/standard.rs`
- New public type: `StandardGraph`
- New public type: `SubjectBuilder` (location TBD — may live in `subject.rs` or `graph/builder.rs`)
- New re-exports from `graph/mod.rs`

### What Doesn't Change

- `PatternGraph<Extra, V>` — unchanged, still the abstract layer
- `GraphClassifier`, `GraphQuery`, `GraphSnapshot` — unchanged
- `canonical_classifier` / `classify_by_shape` — used internally by `StandardGraph`
- Reconciliation machinery — used internally, not exposed
- All existing tests — continue to pass unmodified

### Relationship to graph-ergonomics-proposal.md

That proposal's `GraphBuilder` is superseded by `StandardGraph`'s atomic construction methods. The remaining items from that proposal (module reorganization, `GraphSnapshot` rename, `GraphQuery` ergonomic methods, type aliases, convenience constructors) are orthogonal and can proceed independently.

## What This Proposal Does NOT Cover

- **Module reorganization** — File moves and renames are a separate concern
- **GraphQuery trait (`GraphRead`)** — Deferred until GraphLens is ported
- **Python/WASM bindings** for StandardGraph — Future work
- **Neo4j interop** — Future work, but StandardGraph is designed to be the natural target
- **Mutable graph operations** — `StandardGraph` is append-oriented (add elements). In-place mutation (remove node, update properties) is future work
- **Policy configuration** — Internal `LastWriteWins` for now. Can expose later if needed
- **Performance optimization** — Correct first, optimize later

## Implementation Order

1. **SubjectBuilder** — Fluent Subject construction. Immediately useful in tests.
2. **StandardGraph struct + atomic construction** — `new`, `add_node`, `add_relationship`, `add_walk`, `add_annotation`
3. **Pattern ingestion** — `add_pattern`, `add_patterns`, `from_gram`, `from_patterns`
4. **Element access and counts** — `node()`, `nodes()`, `node_count()`, etc.
5. **Graph-native queries** — `source`, `target`, `neighbors`, `degree`
6. **Escape hatches** — `as_pattern_graph`, `as_query`, `as_snapshot`
7. **Tests** — Port relevant `pattern_graph` tests to use `StandardGraph`, add new ergonomic tests

## Usage Example

```rust
use pattern_core::graph::StandardGraph;
use pattern_core::subject::Subject;

// Atomic construction
let mut g = StandardGraph::new();
g.add_node(Subject::build("alice").label("Person").property("name", "Alice").done());
g.add_node(Subject::build("bob").label("Person").property("name", "Bob").done());
g.add_relationship(
    Subject::build("r1").label("KNOWS").property("since", 2020).done(),
    &"alice".into(),
    &"bob".into(),
);

assert_eq!(g.node_count(), 2);
assert_eq!(g.relationship_count(), 1);
assert!(!g.has_conflicts());

// Gram ingestion
let g2 = StandardGraph::from_gram(
    "(alice:Person {name:'Alice'})-[:KNOWS {since:2020}]->(bob:Person {name:'Bob'})"
).unwrap();

// Query
let alice = g.node(&"alice".into()).unwrap();
let knows = g.relationship(&"r1".into()).unwrap();
let bob = g.target(&"r1".into()).unwrap();

// Escape to abstract layer
let query = g.as_query();
let snapshot = g.as_snapshot();
```

## References

- Canonical classifier: `crates/pattern-core/src/graph/graph_classifier.rs` (`classify_by_shape`)
- PatternGraph: `crates/pattern-core/src/pattern_graph.rs`
- Reconciliation: `crates/pattern-core/src/reconcile.rs`
- Gram parser: `crates/gram-codec/src/lib.rs` (`parse_gram`)
- Subject/Symbol: `crates/pattern-core/src/subject.rs`
- Prior proposal: `proposals/graph-ergonomics-proposal.md`
