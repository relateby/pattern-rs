# Public API Contract: GraphQuery (031-graph-query)

**Date**: 2026-02-22
**Module**: `pattern-core` crate, `graph` module

This document specifies the public Rust API surface introduced by this feature. All items listed here must be exported from `crates/pattern-core/src/lib.rs` and accessible as `pattern_core::*`.

---

## Enums

### `TraversalDirection`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalDirection {
    Forward,
    Backward,
}
```

---

## Type Aliases

### `TraversalWeight<V>`

```rust
// Default (single-threaded)
pub type TraversalWeight<V> = Rc<dyn Fn(&Pattern<V>, TraversalDirection) -> f64>;

// With `thread-safe` feature
// pub type TraversalWeight<V> = Arc<dyn Fn(&Pattern<V>, TraversalDirection) -> f64 + Send + Sync>;
```

---

## Structs

### `GraphQuery<V>`

```rust
pub struct GraphQuery<V: GraphValue> {
    pub query_nodes:              Rc<dyn Fn() -> Vec<Pattern<V>>>,
    pub query_relationships:      Rc<dyn Fn() -> Vec<Pattern<V>>>,
    pub query_incident_rels:      Rc<dyn Fn(&Pattern<V>) -> Vec<Pattern<V>>>,
    pub query_source:             Rc<dyn Fn(&Pattern<V>) -> Option<Pattern<V>>>,
    pub query_target:             Rc<dyn Fn(&Pattern<V>) -> Option<Pattern<V>>>,
    pub query_degree:             Rc<dyn Fn(&Pattern<V>) -> usize>,
    pub query_node_by_id:         Rc<dyn Fn(&V::Id) -> Option<Pattern<V>>>,
    pub query_relationship_by_id: Rc<dyn Fn(&V::Id) -> Option<Pattern<V>>>,
    pub query_containers:         Rc<dyn Fn(&Pattern<V>) -> Vec<Pattern<V>>>,
}
```

**Trait implementations**: `Clone` (manual; pointer increments only)

---

## Canonical Weight Functions

```rust
/// Uniform cost in both directions.
pub fn undirected<V>() -> TraversalWeight<V>;

/// Forward-only traversal. Backward is impassable (INFINITY).
pub fn directed<V>() -> TraversalWeight<V>;

/// Backward-only traversal. Forward is impassable (INFINITY).
pub fn directed_reverse<V>() -> TraversalWeight<V>;
```

---

## Constructors

### from_pattern_graph (defined in pattern_graph module)

```rust
/// Construct a GraphQuery from a PatternGraph.
/// O(log n) node/relationship lookups via HashMap.
pub fn from_pattern_graph<V>(graph: Rc<PatternGraph<V>>) -> GraphQuery<V>
where
    V: GraphValue + Clone + Eq,
    V::Id: Clone + Eq + Hash,
```

### from_graph_lens (DEFERRED)

Not available in this feature. `GraphLens` type does not yet exist in pattern-rs. A TODO comment is placed in the module for a future feature.

---

## Combinators

```rust
/// Restrict a GraphQuery to elements satisfying `include`.
/// Returns a new GraphQuery — no data is copied.
pub fn frame_query<V>(
    include: Rc<dyn Fn(&Pattern<V>) -> bool>,
    base: GraphQuery<V>,
) -> GraphQuery<V>
where
    V: GraphValue + Clone;

/// Wrap query_incident_rels and query_degree with an eager HashMap cache.
/// Cache is built upfront from all nodes; all other fields pass through unchanged.
pub fn memoize_incident_rels<V>(base: GraphQuery<V>) -> GraphQuery<V>
where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + Hash;
```

---

## Algorithms Module

All functions are in the `algorithms` sub-module, also re-exported at crate level.

### Traversal

```rust
pub fn bfs<V>(q: &GraphQuery<V>, weight: &TraversalWeight<V>, start: &Pattern<V>)
    -> Vec<Pattern<V>>
where V: GraphValue + Clone, V::Id: Clone + Eq + Hash + Ord;

pub fn dfs<V>(q: &GraphQuery<V>, weight: &TraversalWeight<V>, start: &Pattern<V>)
    -> Vec<Pattern<V>>
where V: GraphValue + Clone, V::Id: Clone + Eq + Hash + Ord;
```

### Paths

```rust
pub fn shortest_path<V>(
    q: &GraphQuery<V>, weight: &TraversalWeight<V>,
    from: &Pattern<V>, to: &Pattern<V>,
) -> Option<Vec<Pattern<V>>>
where V: GraphValue + Clone, V::Id: Clone + Eq + Hash + Ord;

pub fn has_path<V>(
    q: &GraphQuery<V>, weight: &TraversalWeight<V>,
    from: &Pattern<V>, to: &Pattern<V>,
) -> bool
where V: GraphValue + Clone, V::Id: Clone + Eq + Hash + Ord;

pub fn all_paths<V>(
    q: &GraphQuery<V>, weight: &TraversalWeight<V>,
    from: &Pattern<V>, to: &Pattern<V>,
) -> Vec<Vec<Pattern<V>>>
where V: GraphValue + Clone, V::Id: Clone + Eq + Hash + Ord;
```

### Boolean Queries

```rust
pub fn is_neighbor<V>(
    q: &GraphQuery<V>, weight: &TraversalWeight<V>,
    a: &Pattern<V>, b: &Pattern<V>,
) -> bool
where V: GraphValue + Clone, V::Id: Clone + Eq + Hash;

pub fn is_connected<V>(q: &GraphQuery<V>, weight: &TraversalWeight<V>) -> bool
where V: GraphValue + Clone, V::Id: Clone + Eq + Hash + Ord;
```

### Structural

```rust
pub fn connected_components<V>(q: &GraphQuery<V>, weight: &TraversalWeight<V>)
    -> Vec<Vec<Pattern<V>>>
where V: GraphValue + Clone, V::Id: Clone + Eq + Hash + Ord;

pub fn topological_sort<V>(q: &GraphQuery<V>) -> Option<Vec<Pattern<V>>>
where V: GraphValue + Clone, V::Id: Clone + Eq + Hash + Ord;

pub fn has_cycle<V>(q: &GraphQuery<V>) -> bool
where V: GraphValue + Clone, V::Id: Clone + Eq + Hash + Ord;
```

### Spanning

```rust
pub fn minimum_spanning_tree<V>(q: &GraphQuery<V>, weight: &TraversalWeight<V>)
    -> Vec<Pattern<V>>
where V: GraphValue + Clone, V::Id: Clone + Eq + Hash + Ord;
```

### Centrality

```rust
pub fn degree_centrality<V>(q: &GraphQuery<V>) -> HashMap<V::Id, f64>
where V: GraphValue + Clone, V::Id: Clone + Eq + Hash;

pub fn betweenness_centrality<V>(q: &GraphQuery<V>, weight: &TraversalWeight<V>)
    -> HashMap<V::Id, f64>
where V: GraphValue + Clone, V::Id: Clone + Eq + Hash + Ord;
```

### Context Query Helpers

```rust
pub fn query_annotations_of<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    q: &GraphQuery<V>,
    element: &Pattern<V>,
) -> Vec<Pattern<V>>
where V: GraphValue + Clone;

pub fn query_walks_containing<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    q: &GraphQuery<V>,
    element: &Pattern<V>,
) -> Vec<Pattern<V>>
where V: GraphValue + Clone;

pub fn query_co_members<V>(
    q: &GraphQuery<V>,
    element: &Pattern<V>,
    container: &Pattern<V>,
) -> Vec<Pattern<V>>
where V: GraphValue + Clone, V::Id: Clone + Eq + Hash;
```

---

## Cargo Feature: thread-safe

```toml
[features]
thread-safe = []
```

When enabled: all `Rc` → `Arc`, `dyn Fn` → `dyn Fn + Send + Sync`, additional bounds on V and V::Id.

---

## Behavioral Contracts (invariants implementers must uphold)

1. `from_pattern_graph(pg)` — all 7 `GraphQuery` structural invariants hold
2. `frame_query(pred, base)` — all 7 invariants hold for the returned query
3. `memoize_incident_rels(base)` — results of `query_incident_rels` are identical to base; `query_degree` equals `len(query_incident_rels)`
4. `shortest_path(q, w, n, n)` — returns `Some(vec![n])` (same node)
5. `is_connected(q, w)` on empty graph — returns `true`
6. `topological_sort(q)` — returns `None` iff graph contains a directed cycle
7. `degree_centrality` — all values are in [0.0, 1.0] for graphs with ≥ 2 nodes
