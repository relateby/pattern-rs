# Data Model: GraphQuery (031-graph-query)

**Date**: 2026-02-22
**Branch**: `031-graph-query`
**Reference**: `../pattern-hs/libs/pattern/src/Pattern/Graph/GraphQuery.hs`

---

## Core Types

### TraversalDirection

A two-variant enum expressing which direction along a directed relationship is being traversed.

| Variant  | Meaning                                  |
|----------|------------------------------------------|
| Forward  | Source → Target (follow relationship)    |
| Backward | Target → Source (reverse relationship)   |

**Invariants**: No additional invariants — this is a pure discriminant.

---

### TraversalWeight\<V\>

A function type alias. Given a relationship pattern and a traversal direction, returns the traversal cost as a non-negative `f64`.

| Value       | Cost semantics                                              |
|-------------|-------------------------------------------------------------|
| Finite ≥ 0  | Traversal allowed at that cost                              |
| `INFINITY`  | Traversal blocked in that direction (impassable)            |
| Negative    | Not supported; behavior of algorithms is undefined          |

**Canonical instances**:
- `undirected` — returns 1.0 for all inputs
- `directed` — 1.0 Forward, INFINITY Backward
- `directed_reverse` — INFINITY Forward, 1.0 Backward

---

### GraphQuery\<V\>

A struct-of-closures representing the complete query interface over a graph. All nine fields are independent callable operations.

| Field                  | Input                | Output                     | Complexity (default) |
|------------------------|----------------------|----------------------------|-----------------------|
| `query_nodes`          | (none)               | All node patterns          | O(n)                  |
| `query_relationships`  | (none)               | All relationship patterns  | O(r)                  |
| `query_incident_rels`  | node pattern         | Incident relationships     | O(r)                  |
| `query_source`         | relationship pattern | Source node or None        | O(1)                  |
| `query_target`         | relationship pattern | Target node or None        | O(1)                  |
| `query_degree`         | node pattern         | Count of incident rels     | O(r) or O(1) indexed  |
| `query_node_by_id`     | node identity        | Node pattern or None       | O(log n) from PatternGraph; O(n) from GraphLens |
| `query_relationship_by_id` | rel identity     | Relationship pattern or None | O(log r) from PatternGraph; O(r) from GraphLens |
| `query_containers`     | element pattern      | Direct container patterns  | O(r + w + a)          |

**Structural invariants**:
1. `query_source(r) = Some(s)` implies `s ∈ query_nodes()`
2. `query_target(r) = Some(t)` implies `t ∈ query_nodes()`
3. `r ∈ query_incident_rels(n)` implies `query_source(r) = Some(n) || query_target(r) = Some(n)`
4. `query_degree(n) == query_incident_rels(n).len()` (default; implementations may be faster)
5. `query_node_by_id(n.value.identify()) = Some(n)` for all `n ∈ query_nodes()`
6. `query_relationship_by_id(r.value.identify()) = Some(r)` for all `r ∈ query_relationships()`
7. `query_containers` returns only **direct** containers — not transitive containment

**Clone semantics**: Cloning a `GraphQuery<V>` increments reference counts (pointer clone). No backing data is copied.

---

## Constructors

### from_pattern_graph

Builds a `GraphQuery<V>` from an `Rc<PatternGraph<V>>`.

- Source: defined in `pattern_graph.rs` (not `graph_query.rs`) to avoid circular imports
- `query_node_by_id` — O(log n) HashMap lookup
- `query_relationship_by_id` — O(log r) HashMap lookup
- `query_containers` — scans all four typed maps: relationships, walks, annotations, (and others for the `other` map)

### from_graph_lens

**DEFERRED** — `GraphLens` type does not yet exist in pattern-rs. A TODO placeholder is added.

---

## Combinators

### frame_query

Restricts a `GraphQuery<V>` to elements satisfying a predicate. The resulting value is itself a full `GraphQuery<V>`.

| Field in framed query          | Behavior                                                                  |
|--------------------------------|---------------------------------------------------------------------------|
| `query_nodes`                  | Filtered by predicate                                                     |
| `query_relationships`          | Filtered by predicate                                                     |
| `query_incident_rels(n)`       | Base incident rels where both source AND target satisfy predicate         |
| `query_source`, `query_target` | Delegated unchanged to base                                               |
| `query_degree(n)`              | Count of filtered incident rels (source AND target satisfy predicate)     |
| `query_node_by_id(i)`          | Base lookup; returns None if result doesn't satisfy predicate             |
| `query_relationship_by_id(i)`  | Base lookup; returns None if result doesn't satisfy predicate             |
| `query_containers(p)`          | Base containers filtered by predicate                                     |

**Invariant preservation**: All 7 GraphQuery invariants hold for the framed query if they hold for the base.

### memoize_incident_rels

Wraps `query_incident_rels` and `query_degree` with an eager `HashMap<V::Id, Vec<Pattern<V>>>` cache. All other fields pass through unchanged.

- Cache is built eagerly at construction time by calling `query_nodes()` and `query_incident_rels(n)` for each node.
- Per-`GraphQuery` cache — not global.
- No `RefCell` needed (eager, immutable after construction).
- Recommended for algorithms that call `query_incident_rels` repeatedly (e.g., betweenness centrality).

---

## Algorithms

All algorithms are free functions in the `algorithms` module. None depend on the graph backing representation.

| Algorithm               | Signature sketch                                      | Notes                              |
|-------------------------|-------------------------------------------------------|------------------------------------|
| `bfs`                   | `(q, weight, start) → Vec<Pattern<V>>`                | Includes start node                |
| `dfs`                   | `(q, weight, start) → Vec<Pattern<V>>`                | Includes start node                |
| `shortest_path`         | `(q, weight, from, to) → Option<Vec<Pattern<V>>>`     | Dijkstra; same-node → Some([node]) |
| `has_path`              | `(q, weight, from, to) → bool`                        | Delegates to shortest_path         |
| `all_paths`             | `(q, weight, from, to) → Vec<Vec<Pattern<V>>>`        | DFS; exponential worst case        |
| `is_neighbor`           | `(q, weight, a, b) → bool`                            | Direct finite-cost connection      |
| `is_connected`          | `(q, weight) → bool`                                  | Empty graph = true (vacuous)       |
| `connected_components`  | `(q, weight) → Vec<Vec<Pattern<V>>>`                  | BFS-based component partition      |
| `topological_sort`      | `(q) → Option<Vec<Pattern<V>>>`                       | None = cycle; ignores weight       |
| `has_cycle`             | `(q) → bool`                                          | Delegates to topological_sort      |
| `minimum_spanning_tree` | `(q, weight) → Vec<Pattern<V>>`                       | Kruskal; cost = min(fwd, bwd)      |
| `degree_centrality`     | `(q) → HashMap<V::Id, f64>`                           | No weight param; structural        |
| `betweenness_centrality`| `(q, weight) → HashMap<V::Id, f64>`                   | Brandes; unnormalized              |

---

## Context Query Helpers

These are derived functions, not fields on `GraphQuery<V>`. They filter `query_containers` results using a `GraphClassifier`.

| Helper                  | Parameters                                        | Returns                                    |
|-------------------------|---------------------------------------------------|--------------------------------------------|
| `query_annotations_of`  | `classifier, q, element`                          | All annotation containers of element       |
| `query_walks_containing`| `classifier, q, element`                          | All walk containers of element             |
| `query_co_members`      | `q, element, container`                           | All elements sharing `container` with element (excluding element itself) |

---

## Feature Flag: thread-safe

When the `thread-safe` Cargo feature is enabled:

| Default                  | Thread-safe variant           |
|--------------------------|-------------------------------|
| `Rc<dyn Fn(...)>`        | `Arc<dyn Fn(...) + Send + Sync>` |
| `Rc<HashMap<...>>`       | `Arc<HashMap<...>>`            |
| No `Send`/`Sync` bounds  | `Send + Sync` bounds on V, V::Id, closures |

This is a compile-time substitution via type aliases. No runtime behavior changes.

---

## Module Layout

```
crates/pattern-core/src/
├── lib.rs                         update: re-export GraphQuery public API
├── pattern_graph.rs               update: add from_pattern_graph constructor
├── graph/
│   ├── mod.rs                     update: re-export from graph_query + algorithms
│   ├── graph_classifier.rs        existing (030)
│   ├── graph_query.rs             NEW: TraversalDirection, TraversalWeight, GraphQuery,
│   │                                   canonical weights, Clone impl, frame_query,
│   │                                   memoize_incident_rels
│   └── algorithms.rs              NEW: all 13 algorithm functions + 3 context helpers

crates/pattern-core/tests/
├── graph_query.rs                 NEW: T015–T017, T047–T051, T056 (matching Haskell test IDs)
└── algorithms.rs                  NEW: algorithm correctness tests

crates/pattern-core/Cargo.toml    update: add thread-safe feature flag
```

---

## Dependencies (unchanged)

No new external crates are needed. All data structures required are available in `std`:
- `std::collections::HashMap` — memoization cache, centrality maps
- `std::collections::BinaryHeap` / `std::collections::BTreeMap` — Dijkstra priority queue
- `std::collections::HashSet` — BFS/DFS visited tracking
- `std::collections::VecDeque` — BFS queue
- `std::rc::Rc` (default) / `std::sync::Arc` (thread-safe feature)
- `std::cell::RefCell` — if needed for lazy patterns (research says eager; not needed)
