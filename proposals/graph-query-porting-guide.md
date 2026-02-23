# Proposal: GraphQuery — Portable, Composable Graph Query Interface for pattern-rs

**Status**: 📝 Design Only  
**Date**: 2026-02-22  
**Depends on**: GraphClassifier port (pattern-rs)  
**Followed by**: GraphTransform port → GraphMutation port  
**Mirrors**: `proposals/graph-query.md` (pattern-hs)

---

## Summary

Port `GraphQuery<V>` to pattern-rs as a struct-of-function-pointers (or closures) that abstracts over graph traversal and lookup, decoupling graph algorithms from any particular graph representation. The same capability and behavior as the Haskell original is preserved: graph algorithms (shortest path, connected components, BFS/DFS, centrality) are expressed once as plain functions over `GraphQuery<V>` and work against any backing store — `PatternGraph`, a database, a Graph Frame, or any future representation — that can produce a `GraphQuery<V>` value.

Traversal direction and weighting are expressed as a `TraversalWeight` function supplied to algorithms at the call site, not embedded in `GraphQuery`. Directed, undirected, and weighted traversal are all constraints on the general case, not distinct representation types.

---

## Motivation

The motivation is identical to the Haskell proposal. Graph algorithms should not be bound to a single graph representation. Adding a new representation should not require re-implementing algorithms. Traversal direction should be a call-site decision, not a type-level decision.

The Rust port also inherits the composability argument: a `GraphQuery<V>` is a first-class value. Graph Frames, cached views, and database-backed graphs are all expressible as `GraphQuery<V>` values constructed by wrapping another `GraphQuery<V>` — no new types, no new trait impls.

---

## Design

### `TraversalDirection` and `TraversalWeight`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalDirection {
    Forward,
    Backward,
}

/// A function assigning a traversal cost to each (relationship, direction) pair.
///
/// Infinity (`f64::INFINITY`) encodes impassability — traversal is blocked in
/// that direction. Non-negative finite values encode traversal cost.
/// Negative weights are not supported by the standard Dijkstra-based algorithms.
pub type TraversalWeight<V> = Rc<dyn Fn(&Pattern<V>, TraversalDirection) -> f64>;

/// Uniform cost in both directions. Direction is ignored.
pub fn undirected<V>() -> TraversalWeight<V> {
    Rc::new(|_rel, _dir| 1.0)
}

/// Forward-only traversal. Backward traversal is impassable.
pub fn directed<V>() -> TraversalWeight<V> {
    Rc::new(|_rel, dir| match dir {
        TraversalDirection::Forward  => 1.0,
        TraversalDirection::Backward => f64::INFINITY,
    })
}

/// Backward-only traversal. Forward traversal is impassable.
pub fn directed_reverse<V>() -> TraversalWeight<V> {
    Rc::new(|_rel, dir| match dir {
        TraversalDirection::Forward  => f64::INFINITY,
        TraversalDirection::Backward => 1.0,
    })
}
```

`Arc<dyn Fn(...)>` is the idiomatic Rust equivalent of a first-class function value that can be cloned cheaply, shared across threads, and composed without lifetime issues. Callers who need a single-threaded weight function may use `Rc<dyn Fn(...)>` in a non-`Send` variant if performance demands it.

### `GraphQuery<V>` — the interface

```rust
/// A struct-of-closures representing a graph query interface.
///
/// Construct via `from_pattern_graph` or `from_graph_lens`. Compose with
/// `frame_query` and `memoize_incident_rels`.
///
/// # Invariants
///
/// - `query_source(r) = Some(s)` implies `s ∈ query_nodes()`
/// - `query_target(r) = Some(t)` implies `t ∈ query_nodes()`
/// - `r ∈ query_incident_rels(n)` implies `query_source(r) = Some(n) || query_target(r) = Some(n)`
/// - `query_degree(n) == query_incident_rels(n).len()` (default; implementations may be faster)
/// - `query_node_by_id(n.id()) = Some(n)` for all `n ∈ query_nodes()`
/// - `query_relationship_by_id(r.id()) = Some(r)` for all `r ∈ query_relationships()`
/// - `query_containers` returns only direct containers — not transitive containment
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

`query_neighbors` is intentionally absent — identical reasoning to the Haskell original. Because traversal direction is expressed through `TraversalWeight`, "neighbors" is not a fixed concept. Algorithms derive reachable neighbors from `query_incident_rels`, `query_source`, `query_target`, and the supplied weight.

`query_degree` is derivable from `query_incident_rels` but included explicitly to allow O(1) implementations (e.g. a graph backed by a degree index).

`query_containers` supports upward traversal: "what higher-order structures directly contain this element?" It is required by the GraphMutation port for coherent deletion and is independently useful for impact analysis and pattern matching.

### Rust-specific design notes

**Why `Rc<dyn Fn(...)>` instead of a trait?**  
The Haskell design deliberately chose records-of-functions over typeclasses for portability. In Rust, a trait over the backing store would be the typeclass-equivalent choice. We reject it for the same reasons: it couples algorithms to the trait definition, prevents runtime-variable query construction (e.g. database-backed graphs), and makes composition via wrapping awkward. A struct-of-closures is the idiomatic Rust analog of a Haskell record-of-functions. `Rc<dyn Fn(...)>` gives cheap clone and composition via closure capture with no unnecessary overhead.

**`Rc` by default; `Arc` via feature flag**  
pattern-rs targets Rust, Python (via PyO3), and WASM — all primarily single-threaded contexts. WASM in particular has an immature threading story (SharedArrayBuffer + Atomics), and most WASM targets are single-threaded today. `Rc` is therefore the correct default: it avoids the atomic reference-count overhead of `Arc` on every clone, and it does not force `Send + Sync` bounds onto closure captures or backing stores.

Multi-threaded support is available via a `thread-safe` Cargo feature flag. When enabled, a type alias swaps `Rc` for `Arc` and adds `Send + Sync` bounds throughout:

```rust
// Default (no feature flag): single-threaded
pub type Shared<T> = Rc<T>;
pub type SharedFn<T> = Rc<dyn Fn() -> T>;  // (illustrative; field types follow this pattern)

// With `thread-safe` feature: multi-threaded
pub type Shared<T> = Arc<T>;
pub type SharedFn<T> = Arc<dyn Fn() -> T + Send + Sync>;
```

All public API — `GraphQuery<V>`, `TraversalWeight<V>`, constructors, combinators, algorithms — is written against these aliases. Enabling the feature recompiles with `Arc` throughout; no API changes are required. Migrating from `Rc` to `Arc` at a later date is a mechanical, non-breaking upgrade — the direction of travel is easy. The reverse (demoting `Arc` to `Rc`) is not, which is why we start with `Rc`.

**`Clone` for `GraphQuery<V>`**  
Because all fields are `Rc<...>` (or `Arc<...>` under the feature flag), `GraphQuery<V>` cannot derive `Clone` — `dyn Fn` is not `Clone` — but cloning each `Rc` manually is cheap. Cloning shares the underlying closures; it does not copy backing data. See the Clone section under Portability decisions.

**Hot-path fields**  
`query_incident_rels`, `query_source`, `query_target`, and `query_degree` are hot-path functions. Their `Rc<dyn Fn(...)>` dispatch cost is one vtable lookup per call — acceptable for correctness-first work, and avoidable via monomorphization if profiling shows it matters. The `#[inline]` attribute on algorithm functions that receive `GraphQuery<V>` encourages the compiler to inline dispatch where the concrete type is statically known.

### Constructors

#### From `PatternGraph`

```rust
pub fn from_pattern_graph<V: GraphValue + Clone>(
    graph: Rc<PatternGraph<V>>,
) -> GraphQuery<V>
where
    V::Id: Clone + Eq + Hash,
{
    let g1 = Rc::clone(&graph);
    let g2 = Rc::clone(&graph);
    // ... (one clone per field, all share the same Rc)
    GraphQuery {
        query_nodes: Rc::new(move || g1.nodes().values().cloned().collect()),
        query_relationships: Arc::new(move || g2.relationships().values().cloned().collect()),
        // remaining fields follow the same pattern
        ..
    }
}
```

Each closure captures a cheap `Arc` clone of the backing `PatternGraph`. No data is copied; the graph is shared.

#### From `GraphLens`

```rust
pub fn from_graph_lens<V: GraphValue + Clone + Eq>(
    lens: Rc<GraphLens<V>>,
) -> GraphQuery<V>
{
    // mirrors fromGraphLens in pattern-hs; GraphLens fields map directly
}
```

### Composability

#### Frame query

```rust
/// Restrict a `GraphQuery` to elements satisfying `include`.
///
/// A frame is itself a `GraphQuery` — no new types required.
pub fn frame_query<V>(
    include: Rc<dyn Fn(&Pattern<V>) -> bool>,
    base: GraphQuery<V>,
) -> GraphQuery<V>
where
    V: GraphValue + Clone,
{
    let inc1 = Rc::clone(&include);
    let inc2 = Rc::clone(&include);
    let base_nodes = Rc::clone(&base.query_nodes);
    let base_rels  = Rc::clone(&base.query_relationships);
    let base_incident = Rc::clone(&base.query_incident_rels);
    let base_source   = Rc::clone(&base.query_source);
    let base_target   = Rc::clone(&base.query_target);
    // ...
    GraphQuery {
        query_nodes: Rc::new(move || {
            base_nodes().into_iter().filter(|n| inc1(n)).collect()
        }),
        query_relationships: Rc::new(move || {
            base_rels().into_iter().filter(|r| {
                base_source(r).map_or(false, |s| inc2(&s))
                    && base_target(r).map_or(false, |t| inc2(&t))
            }).collect()
        }),
        query_incident_rels: Rc::new(move |n| {
            base_incident(n).into_iter().filter(|r| {
                base_source(r).map_or(false, |s| include(&s))
                    && base_target(r).map_or(false, |t| include(&t))
            }).collect()
        }),
        // remaining fields delegate to base with filter applied
    }
}
```

#### Memoized incident rels

```rust
/// Wrap `query_incident_rels` with a `HashMap`-backed cache.
///
/// Useful for algorithms that call `query_incident_rels` on the same node
/// many times (e.g. iterative message-passing).
pub fn memoize_incident_rels<V>(base: GraphQuery<V>) -> GraphQuery<V>
where
    V: GraphValue + Clone,
    V::Id: Clone + Eq + Hash,
{
    let cache: Rc<RefCell<HashMap<V::Id, Vec<Pattern<V>>>>> =
        Rc::new(RefCell::new(HashMap::new()));
    let base_incident = Rc::clone(&base.query_incident_rels);
    GraphQuery {
        query_incident_rels: Rc::new(move |n| {
            let key = /* extract id from n */;
            let mut cache = cache.borrow_mut();
            cache.entry(key.clone())
                .or_insert_with(|| base_incident(n))
                .clone()
        }),
        ..base
    }
}
```

### Context query helpers

These are derived, not primitive — they do not add fields to `GraphQuery<V>`.

```rust
/// All annotations directly attached to `element`.
pub fn query_annotations_of<V>(q: &GraphQuery<V>, element: &Pattern<V>) -> Vec<Pattern<V>>
where V: GraphValue + Clone,
{
    (q.query_containers)(element)
        .into_iter()
        .filter(|p| p.is_annotation())
        .collect()
}

/// All walks that directly contain `relationship`.
pub fn query_walks_containing<V>(q: &GraphQuery<V>, relationship: &Pattern<V>) -> Vec<Pattern<V>>
where V: GraphValue + Clone,
{
    (q.query_containers)(relationship)
        .into_iter()
        .filter(|p| p.is_walk())
        .collect()
}

/// All elements that share a direct container with `element`.
pub fn query_co_members<V>(q: &GraphQuery<V>, element: &Pattern<V>) -> Vec<Pattern<V>>
where V: GraphValue + Clone,
{
    (q.query_containers)(element)
        .into_iter()
        .flat_map(|container| container.elements().to_vec())
        .filter(|e| e != element)
        .collect()
}
```

### Algorithms module

All algorithms are free functions that take `&GraphQuery<V>` and a `TraversalWeight<V>`. None depend on the backing representation.

```rust
pub mod algorithms {
    use super::*;
    use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

    /// Breadth-first traversal from `start`. Returns nodes in visit order.
    pub fn bfs<V>(q: &GraphQuery<V>, weight: &TraversalWeight<V>, start: &Pattern<V>)
        -> Vec<Pattern<V>>
    where V: GraphValue + Clone + Eq + Hash { /* ... */ }

    /// Depth-first traversal from `start`. Returns nodes in visit order.
    pub fn dfs<V>(q: &GraphQuery<V>, weight: &TraversalWeight<V>, start: &Pattern<V>)
        -> Vec<Pattern<V>>
    where V: GraphValue + Clone + Eq + Hash { /* ... */ }

    /// Shortest path between `from` and `to` using Dijkstra's algorithm.
    /// Returns `None` if no path exists under the supplied weight.
    pub fn shortest_path<V>(
        q: &GraphQuery<V>,
        weight: &TraversalWeight<V>,
        from: &Pattern<V>,
        to: &Pattern<V>,
    ) -> Option<Vec<Pattern<V>>>
    where V: GraphValue + Clone + Eq + Hash + Ord { /* ... */ }

    /// Returns `true` if a path from `from` to `to` exists under the supplied weight.
    pub fn has_path<V>(
        q: &GraphQuery<V>,
        weight: &TraversalWeight<V>,
        from: &Pattern<V>,
        to: &Pattern<V>,
    ) -> bool
    where V: GraphValue + Clone + Eq + Hash { /* ... */ }

    /// Connected components. Each component is a `Vec<Pattern<V>>`.
    pub fn connected_components<V>(q: &GraphQuery<V>, weight: &TraversalWeight<V>)
        -> Vec<Vec<Pattern<V>>>
    where V: GraphValue + Clone + Eq + Hash { /* ... */ }

    /// Degree centrality for all nodes. Returns a map from node identity to score.
    pub fn degree_centrality<V>(q: &GraphQuery<V>, weight: &TraversalWeight<V>)
        -> HashMap<V::Id, f64>
    where V: GraphValue + Clone + Eq + Hash { /* ... */ }
}
```

---

## What changes in pattern-rs

| Component | Action | Notes |
|---|---|---|
| `pattern_rs::graph::graph_query` | **New** — introduced by this proposal | Core new module |
| `TraversalDirection` | **New** — in `graph_query` | Two-variant enum |
| `TraversalWeight<V>` | **New** — in `graph_query` | Type alias for `Rc<dyn Fn(...)>`; `Arc` under `thread-safe` feature |
| `undirected`, `directed`, `directed_reverse` | **New** — canonical weight values | Provided by library |
| `GraphQuery<V>` | **New** — in `graph_query` | Struct-of-closures; `Rc`-based by default |
| `from_pattern_graph` | **New** — constructor | `Rc<PatternGraph<V>>` → `GraphQuery<V>` |
| `from_graph_lens` | **New** — constructor | `Rc<GraphLens<V>>` → `GraphQuery<V>` |
| `frame_query` | **New** — combinator | `GraphQuery<V>` → `GraphQuery<V>` |
| `memoize_incident_rels` | **New** — combinator | Caches `query_incident_rels` via `RefCell<HashMap>` |
| Context helpers | **New** — free functions | `query_annotations_of`, `query_walks_containing`, `query_co_members` |
| `pattern_rs::graph::algorithms` | **New** — algorithms module | `bfs`, `dfs`, `shortest_path`, `has_path`, `connected_components`, `degree_centrality` |
| Existing `GraphLens` algorithms | **Wrap** — backward-compat shims | Delegate to `algorithms::*` with `undirected()` default |
| `thread-safe` Cargo feature | **New** — opt-in | Swaps `Rc` → `Arc`, adds `Send + Sync` bounds throughout |

---

## Portability decisions

### Haskell → Rust mapping

| Haskell | Rust | Notes |
|---|---|---|
| `GraphQuery v` (record of functions) | `GraphQuery<V>` (struct of `Rc<dyn Fn(...)>`) | Equivalent expressiveness; `Rc` for cheap clone in single-threaded contexts |
| `TraversalWeight v` (type alias for function) | `TraversalWeight<V>` (type alias for `Rc<dyn Fn(...)>`) | Same semantics |
| `1/0 :: Double` (infinity) | `f64::INFINITY` | Direct equivalent |
| `Maybe` | `Option` | Direct equivalent |
| `[Pattern v]` | `Vec<Pattern<V>>` | Direct equivalent |
| `Map (Id v) (Pattern v)` | `HashMap<V::Id, Pattern<V>>` | Direct equivalent |
| `{-# INLINE #-}` on hot paths | `#[inline]` on algorithm functions | Same intent |
| Lazy evaluation (list fields) | Eager `Vec` (call site) | Fields are closures; callers pay only for what they invoke |

### Key divergence: lazy vs. eager field evaluation

In Haskell, `queryNodes :: [Pattern v]` is a lazy list — it is only evaluated when demanded. In Rust, `query_nodes: Arc<dyn Fn() -> Vec<Pattern<V>>>` is a closure that returns a `Vec` when called — evaluation is explicit and eager at the call site. This is the correct Rust idiom: callers pay only when they invoke the function. Algorithms that call `query_nodes()` multiple times should bind the result to a local variable.

### `Clone` for `GraphQuery<V>`

Deriving `Clone` is not possible with `Rc<dyn Fn(...)>` fields (the inner `dyn Fn` is not `Clone`). Instead, `GraphQuery<V>` implements `Clone` manually by cloning each `Rc` — which is a cheap pointer increment:

```rust
impl<V: GraphValue> Clone for GraphQuery<V> {
    fn clone(&self) -> Self {
        GraphQuery {
            query_nodes:              Rc::clone(&self.query_nodes),
            query_relationships:      Rc::clone(&self.query_relationships),
            query_incident_rels:      Rc::clone(&self.query_incident_rels),
            query_source:             Rc::clone(&self.query_source),
            query_target:             Rc::clone(&self.query_target),
            query_degree:             Rc::clone(&self.query_degree),
            query_node_by_id:         Rc::clone(&self.query_node_by_id),
            query_relationship_by_id: Rc::clone(&self.query_relationship_by_id),
            query_containers:         Rc::clone(&self.query_containers),
        }
    }
}
```

Under the `thread-safe` feature, all `Rc::clone` calls become `Arc::clone` — a mechanical substitution with no API impact.

### Bulk adjacency (deferred)

Identical to the Haskell decision: algorithms like Louvain and large-scale betweenness centrality benefit from a pre-materialized adjacency structure rather than repeated `query_incident_rels` calls. A `query_adjacency: Rc<dyn Fn() -> HashMap<V::Id, Vec<Pattern<V>>>>` field can be added later without breaking existing usage. Deferred until performance requirements are concrete.

---

## What `GraphQuery` intentionally omits (Rust edition)

All omissions from the Haskell proposal carry over unchanged:

- **`query_neighbors`** — direction-dependent; derived from `query_incident_rels` + weight at algorithm call site.
- **Bulk adjacency** — deferred; extension point noted.
- **Pattern matching** — a query language layer above `GraphQuery`; not part of the interface.
- **Walk enumeration** — a `GraphClassifier` concern; derivable from `query_relationships` and the chaining predicate.

---

## Design decisions

**One struct, no trait.** `GraphQuery<V>` is a concrete struct, not a trait. Algorithms are functions over `&GraphQuery<V>`. Users compose and adapt via wrapping, not via trait impls. This preserves the portability argument and avoids trait coherence issues.

**`Rc<dyn Fn(...)>` by default; `Arc` opt-in.** pattern-rs targets Rust, Python (via PyO3), and WASM — all primarily single-threaded. `Rc` avoids atomic reference-count overhead and imposes no `Send + Sync` constraints on closure captures or backing stores. Multi-threaded support is available via the `thread-safe` feature flag, which substitutes `Arc` for `Rc` throughout. Starting with `Rc` and upgrading to `Arc` later is a mechanical, non-breaking change; the reverse is not. See the Rust-specific design notes section.

**`Rc` over `Box`.** `Box` would make `GraphQuery<V>` non-`Clone`. `Rc` enables cheap cloning and sharing — essential for frame composition, where multiple `GraphQuery<V>` values share the same underlying closures. The cost is one (non-atomic) reference count per clone.

**`RefCell` in `memoize_incident_rels`.** The memoization cache requires interior mutability. `RefCell<HashMap<...>>` is the correct choice for the single-threaded default — it has no locking overhead and panics (rather than deadlocking) on misuse. Under the `thread-safe` feature, this becomes `Mutex<HashMap<...>>`.

**`GraphValue` trait bound.** `V: GraphValue` is required on `GraphQuery<V>` because `V::Id` appears in `query_node_by_id` and `query_relationship_by_id`. This matches the Haskell dependency on the `GraphValue` typeclass.

**Backward compatibility.** Existing `GraphLens`-specific algorithm functions are retained as shims delegating to `algorithms::*` with `undirected()`. No existing callers break.

**Implementation order.** GraphClassifier → GraphQuery → GraphTransform → GraphMutation. GraphQuery is a prerequisite for GraphTransform (`map_with_context`, `filter_graph`, context helpers) and for GraphMutation (`query_containers` in deletion).

---

## Open questions

1. **`query_nodes()` call cost.** The closure returns a freshly allocated `Vec` each time. For algorithms that call it multiple times, this is wasteful. An alternative is a `query_nodes_ref: Rc<dyn Fn() -> &[Pattern<V>]>` returning a reference into backing storage — but this requires a lifetime, making the type signature significantly more complex. Deferred; callers should cache the result locally.

2. **`Debug` and `Display` for `GraphQuery<V>`.** `Rc<dyn Fn(...)>` fields do not implement `Debug` by default. A hand-written `Debug` impl that names each field with a placeholder (e.g. `<fn>`) is sufficient for diagnostics. Worth adding in the initial implementation.
