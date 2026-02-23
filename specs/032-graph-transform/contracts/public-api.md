# Public API Contract: GraphTransform (032-graph-transform)

**Date**: 2026-02-23  
**Module**: `pattern-core` crate, `graph` and `graph::transform` modules

This document specifies the public Rust API surface introduced by this feature. All items must be exported from `crates/pattern-core/src/lib.rs` (or the appropriate submodules) and be accessible as `pattern_core::*` or `pattern_core::graph::*` / `pattern_core::graph::transform::*`.

---

## Structs

### GraphView\<Extra, V\>

```rust
pub struct GraphView<Extra, V: GraphValue> {
    pub view_query:    GraphQuery<V>,
    pub view_elements: Vec<(GraphClass<Extra>, Pattern<V>)>,
}
```

**Trait implementations**: `Clone` (if view is cloneable; otherwise omit and document that view is consumed by transforms).

---

### CategoryMappers\<Extra, V\>

```rust
pub struct CategoryMappers<Extra, V: GraphValue> {
    pub nodes:         Box<dyn Fn(Pattern<V>) -> Pattern<V>>,
    pub relationships: Box<dyn Fn(Pattern<V>) -> Pattern<V>>,
    pub walks:         Box<dyn Fn(Pattern<V>) -> Pattern<V>>,
    pub annotations:   Box<dyn Fn(Pattern<V>) -> Pattern<V>>,
    pub other:         Box<dyn Fn(GraphClass<Extra>, Pattern<V>) -> Pattern<V>>,
}

impl<Extra, V: GraphValue> CategoryMappers<Extra, V> {
    /// Identity mapper for all categories.
    pub fn identity() -> Self;
}
```

---

## Enums

### Substitution\<V\>

```rust
pub enum Substitution<V: GraphValue> {
    NoSubstitution,
    ReplaceWith(Pattern<V>),
    RemoveContainer,
}
```

---

## Constructors (graph view)

### from_pattern_graph

```rust
pub fn from_pattern_graph<Extra, V: GraphValue>(
    classifier: &GraphClassifier<Extra, V>,
    graph: &PatternGraph<Extra, V>,
) -> GraphView<Extra, V>
where
    V: Clone + 'static,
    V::Id: Clone + Eq + Hash + 'static;
```

Builds a GraphView from an existing PatternGraph and classifier. The view’s query is built from the graph (e.g. via existing from_pattern_graph on GraphQuery); view_elements are the graph’s contents classified and listed.

### from_graph_lens (DEFERRED)

Not implemented. Placeholder with `unimplemented!()` or `todo!()` and a comment that GraphLens will be ported in a future feature.

---

## Operations

### materialize

```rust
pub fn materialize<Extra, V: GraphValue>(
    classifier: &GraphClassifier<Extra, V>,
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    view: GraphView<Extra, V>,
) -> PatternGraph<Extra, V>
where
    V: HasIdentity<V, Symbol> + Mergeable + Refinable + PartialEq + Clone,
    V::Id: Clone + Eq + Hash;
```

Consumes the view and produces a PatternGraph. Duplicate identities are resolved by the given policy.

---

### unfold (pattern module)

```rust
/// Anamorphism: expand a seed into a Pattern tree. Implemented iteratively to avoid stack overflow.
pub fn unfold<A, V: GraphValue>(
    expand: impl Fn(A) -> (V, Vec<A>),
    seed: A,
) -> Pattern<V>;
```

---

### unfold_graph

```rust
pub fn unfold_graph<A, Extra, V: GraphValue>(
    classifier: &GraphClassifier<Extra, V>,
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    expand: impl Fn(A) -> Vec<Pattern<V>>,
    seeds: Vec<A>,
) -> PatternGraph<Extra, V>
where
    V: HasIdentity<V, Symbol> + Mergeable + Refinable + PartialEq + Clone,
    V::Id: Clone + Eq + Hash;
```

---

### map_graph

```rust
#[inline]
pub fn map_graph<Extra, V: GraphValue>(
    classifier: &GraphClassifier<Extra, V>,
    mappers: CategoryMappers<Extra, V>,
    view: GraphView<Extra, V>,
) -> GraphView<Extra, V>;
```

---

### map_all_graph

```rust
#[inline]
pub fn map_all_graph<Extra, V: GraphValue>(
    f: impl Fn(Pattern<V>) -> Pattern<V>,
    view: GraphView<Extra, V>,
) -> GraphView<Extra, V>;
```

---

### filter_graph

```rust
#[inline]
pub fn filter_graph<Extra, V: GraphValue>(
    classifier: &GraphClassifier<Extra, V>,
    predicate: impl Fn(&GraphClass<Extra>, &Pattern<V>) -> bool,
    substitution: Substitution<V>,
    view: GraphView<Extra, V>,
) -> GraphView<Extra, V>;
```

---

### fold_graph

```rust
#[inline]
pub fn fold_graph<Extra, V: GraphValue, M>(
    f: impl Fn(M, &GraphClass<Extra>, &Pattern<V>) -> M,
    init: M,
    view: &GraphView<Extra, V>,
) -> M;
```

---

### map_with_context

```rust
#[inline]
pub fn map_with_context<Extra, V: GraphValue>(
    classifier: &GraphClassifier<Extra, V>,
    f: impl Fn(&GraphQuery<V>, Pattern<V>) -> Pattern<V>,
    view: GraphView<Extra, V>,
) -> GraphView<Extra, V>;
```

Snapshot: `f` receives a reference to the view’s query taken before any transformation. All elements see the same snapshot.

---

### para_graph

```rust
#[inline]
pub fn para_graph<Extra, V: GraphValue, R>(
    f: impl Fn(&GraphQuery<V>, &Pattern<V>, &[R]) -> R,
    view: &GraphView<Extra, V>,
) -> HashMap<V::Id, R>
where
    V::Id: Eq + Hash;
```

For DAGs, processing order should be defined (e.g. topological). For cyclic graphs, use `para_graph_fixed` for fixed-point semantics.

---

### para_graph_fixed

```rust
#[inline]
pub fn para_graph_fixed<Extra, V: GraphValue, R: Clone>(
    converged: impl Fn(&R, &R) -> bool,
    f: impl Fn(&GraphQuery<V>, &Pattern<V>, &[R]) -> R,
    init: R,
    view: &GraphView<Extra, V>,
) -> HashMap<V::Id, R>
where
    V::Id: Eq + Hash;
```

Iterates until `converged(old, new)` holds for all elements, then returns the final map.

---

## Re-exports

The following are re-exported from `pattern_core` (or `pattern_core::graph`) for convenience:

- `GraphView`
- `CategoryMappers`, `Substitution`
- `from_pattern_graph`, `materialize`
- `unfold` (from pattern module)
- `unfold_graph`, `map_graph`, `map_all_graph`, `filter_graph`, `fold_graph`, `map_with_context`, `para_graph`, `para_graph_fixed`

Existing types (`GraphQuery`, `GraphClassifier`, `GraphClass`, `Pattern`, `PatternGraph`, `ReconciliationPolicy`, etc.) remain as in 030/031.
