# Research: GraphQuery Port (031-graph-query)

**Date**: 2026-02-22
**Branch**: `031-graph-query`
**Reference**: `../pattern-hs/libs/pattern/src/Pattern/Graph/GraphQuery.hs` (207 lines)
**Reference**: `../pattern-hs/libs/pattern/src/Pattern/Graph/Algorithms.hs` (470 lines)
**Reference**: `../pattern-hs/specs/035-graph-query/`

---

## Decision 1: Struct-of-closures vs. trait

**Decision**: `GraphQuery<V>` is a concrete struct with `Rc<dyn Fn(...)>` fields.

**Rationale**: Mirrors the Haskell record-of-functions design deliberately chosen over typeclasses. Trait-based design would couple algorithms to the trait definition, block runtime-variable construction (database-backed graphs), and make wrapping combinators awkward. `Rc<dyn Fn(...)>` is the idiomatic Rust equivalent of a Haskell record-of-functions — one vtable lookup per call, acceptable for correctness-first work.

**Alternatives considered**:
- Trait with associated functions — rejected: couples algorithms to trait, no runtime variability
- `Box<dyn Fn(...)>` — rejected: makes `GraphQuery<V>` non-Clone; Rc enables cheap sharing for frame composition

---

## Decision 2: Rc by default; Arc under `thread-safe` feature

**Decision**: All `Rc<dyn Fn(...)>` fields use `Rc` by default. A `thread-safe` Cargo feature flag swaps `Rc` → `Arc` throughout with `Send + Sync` bounds.

**Rationale**: pattern-rs targets Rust, Python (via PyO3), and WASM — all primarily single-threaded contexts. Rc avoids atomic reference-count overhead and does not impose `Send + Sync` constraints on closure captures or backing stores. Upgrading Rc → Arc later is mechanical and non-breaking; downgrading is not. Start with the weaker, simpler choice.

**Alternatives considered**:
- Arc by default — rejected: unnecessary atomic overhead in single-threaded dominant use cases, forces Send+Sync on all backing stores
- No threading support — rejected: multi-threaded server usage is a valid future target

---

## Decision 3: memoizeIncidentRels is eager

**Decision**: `memoize_incident_rels` builds a complete `HashMap<V::Id, Vec<Pattern<V>>>` from all nodes upfront when called, then serves subsequent `query_incident_rels` calls from the pre-built map.

**Rationale**: The Haskell reference uses eager construction (`Map.fromList [...] | n <- queryNodes base`). Eager construction amortizes the cost across all subsequent algorithm calls and is safe for immutable graph queries. A lazy `RefCell` cache would add borrow complexity with no benefit for the primary use case (algorithms that visit all nodes multiple times).

**Alternatives considered**:
- Lazy `RefCell<HashMap>` cache — rejected: adds interior mutability complexity, no benefit over eager for the intended use case (betweenness centrality visits all nodes)

---

## Decision 4: 15 algorithms in scope

**Decision**: Port all 15 algorithms from `Pattern.Graph.Algorithms`: `bfs`, `dfs`, `shortest_path`, `has_path`, `all_paths`, `is_neighbor`, `is_connected`, `connected_components`, `topological_sort`, `has_cycle`, `minimum_spanning_tree`, `degree_centrality`, `betweenness_centrality`, `query_annotations_of`, `query_walks_containing`.

The proposal document listed only 6 algorithms. The Haskell reference has 15. All should be ported for full behavioral equivalence.

**Rationale**: Constitution Principle I requires faithful replication of the reference implementation.

**Algorithms with non-obvious Rust complexity**:
- `topological_sort` — DFS post-order with cycle detection; returns `Option<Vec<...>>` (None = cycle)
- `minimum_spanning_tree` — Kruskal's with union-find; returns nodes in MST (not edges)
- `betweenness_centrality` — Brandes algorithm, O(n·(n+r)·log n); recommend wrapping with `memoize_incident_rels`

---

## Decision 5: degreeCentrality does not take TraversalWeight

**Decision**: `degree_centrality(q: &GraphQuery<V>) -> HashMap<V::Id, f64>` — no `TraversalWeight` parameter.

**Rationale**: Haskell signature is `degreeCentrality :: GraphQuery v -> Map (Id v) Double`. It uses `queryDegree` which counts all incident relationships regardless of direction. Degree centrality is a structural property (count of connections), not a traversal-direction property. The proposal document incorrectly included a weight parameter.

**Alternatives considered**:
- Weight parameter — rejected: deviates from reference; degree centrality is direction-agnostic

---

## Decision 6: Context helpers accept GraphClassifier

**Decision**: `query_annotations_of` and `query_walks_containing` accept `&GraphClassifier<Extra, V>` as a parameter.

**Rationale**: Haskell signatures:
```haskell
queryAnnotationsOf   :: GraphClassifier extra v -> GraphQuery v -> Pattern v -> [Pattern v]
queryWalksContaining :: GraphClassifier extra v -> GraphQuery v -> Pattern v -> [Pattern v]
```
These helpers filter `queryContainers` results by `GraphClass`. Classification requires a `GraphClassifier`. The proposal simplified this by calling `p.is_annotation()` / `p.is_walk()` — but `Pattern<V>` has no such methods and classification is the classifier's responsibility.

Rust signatures:
```rust
pub fn query_annotations_of<Extra, V>(classifier: &GraphClassifier<Extra, V>, q: &GraphQuery<V>, element: &Pattern<V>) -> Vec<Pattern<V>>
pub fn query_walks_containing<Extra, V>(classifier: &GraphClassifier<Extra, V>, q: &GraphQuery<V>, element: &Pattern<V>) -> Vec<Pattern<V>>
```

**Alternatives considered**:
- Storing classifier in GraphQuery — rejected: GraphQuery is representation-agnostic; classifier is a separate concern

---

## Decision 7: queryCoMembers takes explicit container (3-arg)

**Decision**: `query_co_members(q, element, container)` — three arguments, container is explicit.

**Rationale**: Haskell signature is `queryCoMembers :: GraphQuery v -> Pattern v -> Pattern v -> [Pattern v]` where the third arg is the container. This finds all elements that are co-members within a specific container. The 2-arg version in the proposal iterates all containers of the element which is a different (higher-level) operation. Matching Haskell for behavioral equivalence.

**Implementation note**: The Rust implementation can use `container.elements` accessor for O(k) element lookup (vs. Haskell's O(n·r) scan). This is a valid Rust-idiomatic optimization over the reference.

---

## Decision 8: from_pattern_graph lives in pattern_graph.rs module

**Decision**: `from_pattern_graph` constructor is defined in `pattern_graph.rs`, not in `graph_query.rs`, mirroring Haskell's import structure.

**Rationale**: In Haskell, `fromPatternGraph` is defined in `Pattern.PatternGraph` (not in `Pattern.Graph.GraphQuery`) to avoid a circular import. The same consideration applies in Rust: `graph_query.rs` should not import from `pattern_graph.rs` to keep the dependency graph acyclic.

---

## Decision 9: from_graph_lens deferred

**Decision**: `from_graph_lens` constructor (FR-005) is deferred. No `GraphLens` type exists in pattern-rs yet.

**Rationale**: `GraphLens` is a future feature. A `from_graph_lens` shim cannot be written without the type. A placeholder note will be added to the module with a TODO comment.

**Impact**: FR-005 is out of scope for this feature. No existing callers are affected (GraphLens doesn't exist yet, so there's nothing to break).

---

## Decision 10: Algorithms module placement

**Decision**: Algorithms live in `crates/pattern-core/src/graph/algorithms.rs`, not a separate crate.

**Rationale**: All existing code is in `pattern-core`. Adding a sub-crate for algorithms would add workspace complexity with no benefit. Algorithms are logically part of the graph module.

---

## Findings: Behavioral notes from Haskell reference

### BFS/DFS return start node
Both `bfs` and `dfs` include the start node in the result. BFS returns nodes in visit order (start first); DFS uses a stack (start may appear first or later depending on implementation).

### shortestPath: same-node case
`shortestPath start end` where `start == end` (by identity) returns `Just [start]` immediately — no traversal needed.

### topologicalSort ignores TraversalWeight
`topologicalSort` operates on the directed structure implied by relationship endpoint order (source → target), ignoring any `TraversalWeight`. It uses only `querySource`/`queryTarget` from the GraphQuery.

### minimumSpanningTree: edge cost = min(forward, backward)
Kruskal's algorithm uses `min(fwd, bwd)` as the edge cost. Edges with `INFINITY` cost in both directions are excluded.

### isConnected: empty graph is connected
An empty graph (no nodes) is vacuously connected — returns `true`.

### memoizeIncidentRels: eager, per-query cache
The cache is built eagerly from `queryNodes` at construction time. It is per-`GraphQuery` value, not global.

### betweennessCentrality: unnormalized scores
`betweenness_centrality` returns unnormalized betweenness scores. Callers who need normalized values divide by `(n-1)*(n-2)/2` for undirected or `(n-1)*(n-2)` for directed graphs.
