# Porting Guide: GraphTransform — Rust

**Source**: `graph-transform.md` (design proposal)  
**Depends on**: GraphClassifier porting guide, GraphQuery porting guide  
**Language target**: Rust  
**Status**: Porting guide — design only

---

## Overview

This guide covers the Rust translation of the five components introduced in the
GraphTransform proposal:

1. `GraphView` — the universal graph-like interface
2. `materialize` — explicit `GraphView → PatternGraph` step
3. `unfold` / `unfoldGraph` — seed-based construction
4. Categorized transformations — `map_graph`, `filter_graph`, `fold_graph`, `map_all_graph`
5. `map_with_context` — context-aware mapping with snapshot semantics
6. `para_graph` / `para_graph_fixed` — topology-aware folding; Pregel foundation

The section on [Rust-specific considerations](#rust-specific-considerations) at the end
covers the cross-cutting decisions that affect everything above.

---

## 1. `GraphView`

### Haskell

```haskell
data GraphView extra v = GraphView
  { viewQuery    :: GraphQuery v
  , viewElements :: [(GraphClass extra, Pattern v)]
  }
```

### Rust

```rust
pub struct GraphView<'a, Extra, V: GraphValue> {
    pub view_query:    GraphQuery<'a, V>,
    pub view_elements: Vec<(GraphClass<Extra>, Pattern<V>)>,
}
```

**Lifetime `'a`**: `GraphQuery` holds function pointers (or closures) that may borrow
graph storage. The lifetime ties `GraphView` to the storage it views. This matches the
Haskell semantics where `GraphQuery v` is assembled from the backing graph at construction
time.

**Alternatively — owned `GraphQuery`**: if `GraphQuery` is defined as a struct of
`Arc`-wrapped closures (as discussed in the GraphQuery porting guide), `GraphView` can
own it without a lifetime parameter:

```rust
pub struct GraphView<Extra, V: GraphValue> {
    pub view_query:    GraphQuery<V>,          // Arc-wrapped closures, no borrow
    pub view_elements: Vec<(GraphClass<Extra>, Pattern<V>)>,
}
```

Which representation to use follows from the choice already made when porting
`GraphQuery`. This guide assumes the `Arc`-wrapped owned form throughout, since it
simplifies pipeline composition (no lifetime threading across transformation steps).

### Construction functions

```rust
// In the graph module, alongside PatternGraph and GraphLens

pub fn from_pattern_graph<Extra, V: GraphValue>(
    classifier: &GraphClassifier<Extra, V>,
    graph: &PatternGraph<V>,
) -> GraphView<Extra, V> { ... }

pub fn from_graph_lens<Extra, V: GraphValue>(
    classifier: &GraphClassifier<Extra, V>,
    lens: &GraphLens<V>,
) -> GraphView<Extra, V> { ... }
```

Adapter constructors (`from_csv`, `from_json`) live in separate crates and are not part
of this module — they target the `GraphView` interface but are not defined here.

---

## 2. `materialize`

### Haskell

```haskell
materialize :: GraphClassifier extra v
            -> ReconciliationPolicy (MergeStrategy v)
            -> GraphView extra v
            -> PatternGraph v
```

### Rust

```rust
pub fn materialize<Extra, V: GraphValue>(
    classifier: &GraphClassifier<Extra, V>,
    policy: ReconciliationPolicy<MergeStrategy<V>>,
    view: GraphView<Extra, V>,
) -> PatternGraph<V> { ... }
```

`materialize` consumes the `GraphView` (moves it). This is the right default: once you
have materialized a view into a `PatternGraph`, you typically don't need the view anymore.
If callers need to keep the view alive after materialization (e.g., to materialize again
with a different policy), they clone before calling.

**The canonical pipeline shape in Rust**:

```rust
fn pipeline(csv: Csv) -> PatternGraph<Subject> {
    let view = from_csv(&canonical_classifier(), csv);
    let view = filter_graph(&canonical_classifier(), is_relevant, Substitution::NoSubstitution, view);
    let view = map_with_context(&canonical_classifier(), enrich, view);
    materialize(&canonical_classifier(), ReconciliationPolicy::LastWriteWins, view)
}
```

This is less point-free than the Haskell form but the data flow is identical: CSV becomes
a `GraphView` immediately; all operations work over `GraphView`; `materialize` is the
explicit boundary producing owned `PatternGraph` storage.

---

## 3. `unfold` and `unfoldGraph`

### Haskell

```haskell
unfold :: (a -> (v, [a])) -> a -> Pattern v

unfoldGraph :: GraphClassifier extra v
            -> ReconciliationPolicy (MergeStrategy v)
            -> (a -> [Pattern v])
            -> [a]
            -> PatternGraph v
```

### Rust

```rust
/// Anamorphism: recursively expand a seed into a Pattern tree.
pub fn unfold<A, V: GraphValue>(
    expand: impl Fn(A) -> (V, Vec<A>),
    seed: A,
) -> Pattern<V> { ... }

/// Expand a collection of seeds into a PatternGraph.
pub fn unfold_graph<A, Extra, V: GraphValue>(
    classifier: &GraphClassifier<Extra, V>,
    policy: ReconciliationPolicy<MergeStrategy<V>>,
    expand: impl Fn(A) -> Vec<Pattern<V>>,
    seeds: Vec<A>,
) -> PatternGraph<V> { ... }
```

**`unfold` placement**: in Haskell, `unfold` lives in `Pattern.Core` alongside `para`.
In Rust, it belongs in the `pattern` or `pattern_core` module next to the `para`
function. `unfold_graph` is the graph-level wrapper and lives in the `graph_transform`
module.

**Recursion and stack overflow**: `unfold` is recursive. For deep hierarchies (document
trees, deep org charts), a naive recursive implementation will overflow the stack. Two
options:

- Use a trampolined / iterative implementation with an explicit work stack (`Vec`)
- Annotate with `#[allow(clippy::only_used_in_recursion)]` and document the depth limit

For production use, the explicit work stack is preferred. The interface is unchanged; the
implementation iterates rather than recurses.

**ETL example**:

```rust
fn row_to_patterns(row: &Row) -> Vec<Pattern<Subject>> {
    vec![person_node(row), department_node(row), works_in_rel(row)]
}

let graph = unfold_graph(
    &canonical_classifier(),
    ReconciliationPolicy::LastWriteWins,
    |row| row_to_patterns(&row),
    rows,
);
```

---

## 4. Categorized transformations

### `map_graph`

#### Haskell

```haskell
mapGraph :: GraphClassifier extra v
         -> (Pattern v -> Pattern v)  -- nodes
         -> (Pattern v -> Pattern v)  -- relationships
         -> (Pattern v -> Pattern v)  -- walks
         -> (Pattern v -> Pattern v)  -- annotations
         -> (Pattern v -> Pattern v)  -- other
         -> GraphView extra v -> GraphView extra v
```

#### Rust — named struct approach

Five positional function parameters becomes awkward in Rust as the list grows and callers
must supply an identity function for every category they don't care about. A named struct
avoids this:

```rust
pub struct CategoryMappers<Extra, V: GraphValue> {
    pub nodes:         Box<dyn Fn(Pattern<V>) -> Pattern<V>>,
    pub relationships: Box<dyn Fn(Pattern<V>) -> Pattern<V>>,
    pub walks:         Box<dyn Fn(Pattern<V>) -> Pattern<V>>,
    pub annotations:   Box<dyn Fn(Pattern<V>) -> Pattern<V>>,
    pub other:         Box<dyn Fn(GraphClass<Extra>, Pattern<V>) -> Pattern<V>>,
}

impl<Extra, V: GraphValue> CategoryMappers<Extra, V> {
    /// Identity mapper for all categories — useful as a starting point.
    pub fn identity() -> Self { ... }
}

pub fn map_graph<Extra, V: GraphValue>(
    classifier: &GraphClassifier<Extra, V>,
    mappers: CategoryMappers<Extra, V>,
    view: GraphView<Extra, V>,
) -> GraphView<Extra, V> { ... }
```

Callers start from `CategoryMappers::identity()` and replace only the categories they
need:

```rust
let mappers = CategoryMappers {
    nodes: Box::new(normalize_node),
    ..CategoryMappers::identity()
};
let view = map_graph(&classifier, mappers, view);
```

This is the idiomatic Rust equivalent of the named-argument / record-update pattern.

Alternatively, use the builder pattern if the project already uses it consistently:

```rust
let view = map_graph(&classifier, view, |b| b.nodes(normalize_node));
```

Choose whichever matches the existing ergonomics in the codebase. The struct-update form
is simpler to implement.

### `map_all_graph`

```rust
pub fn map_all_graph<Extra, V: GraphValue>(
    f: impl Fn(Pattern<V>) -> Pattern<V>,
    view: GraphView<Extra, V>,
) -> GraphView<Extra, V> { ... }
```

Note no `GraphClassifier` parameter — this applies `f` uniformly across all elements
without category distinction.

### `filter_graph`

#### Haskell

```haskell
filterGraph :: GraphClassifier extra v
            -> (GraphClass extra -> Pattern v -> Bool)
            -> Substitution v
            -> GraphView extra v -> GraphView extra v
```

#### Rust

```rust
pub fn filter_graph<Extra, V: GraphValue>(
    classifier: &GraphClassifier<Extra, V>,
    predicate: impl Fn(&GraphClass<Extra>, &Pattern<V>) -> bool,
    substitution: Substitution<V>,
    view: GraphView<Extra, V>,
) -> GraphView<Extra, V> { ... }
```

`Substitution<V>` is defined in the shared types module (`pattern_graph::types`) ahead of
the full `GraphMutation` implementation. It determines what happens to container elements
(walks) when a contained relationship is filtered out:

```rust
pub enum Substitution<V: GraphValue> {
    /// Removed elements leave a gap; the container is kept with the gap as-is.
    NoSubstitution,
    /// Removed elements are replaced by a specified filler element.
    ReplaceWith(Pattern<V>),
    /// Containers of removed elements are themselves removed.
    RemoveContainer,
}
```

The predicate takes shared references (`&GraphClass`, `&Pattern`) — filtering reads but
does not consume elements. The input `view` is consumed (moved); the returned `GraphView`
is freshly allocated from the filtered subset.

### `fold_graph`

#### Haskell

```haskell
foldGraph :: Monoid m
          => (GraphClass extra -> Pattern v -> m)
          -> GraphView extra v -> m
```

#### Rust

Rust has no `Monoid` typeclass. The idiom is to pass an initial accumulator value and a
fold function, or use the `Iterator` combinator chain. Two options:

**Option A — explicit accumulator** (more general, works for non-commutative folds):

```rust
pub fn fold_graph<Extra, V: GraphValue, M>(
    f: impl Fn(M, &GraphClass<Extra>, &Pattern<V>) -> M,
    init: M,
    view: &GraphView<Extra, V>,
) -> M { ... }
```

**Option B — require `Default + std::ops::Add`** (mirrors the `Monoid` constraint):

```rust
pub fn fold_graph<Extra, V: GraphValue, M: Default + std::ops::Add<Output = M>>(
    f: impl Fn(&GraphClass<Extra>, &Pattern<V>) -> M,
    view: &GraphView<Extra, V>,
) -> M { ... }
```

Option A is recommended: it handles `HashMap`-based folds (the count-by-class example)
naturally without requiring `Add` on `HashMap`. `HashMap` accumulation is a very common
use case and Option B excludes it.

**Count-by-class example in Rust**:

```rust
let counts: HashMap<GraphClass<Void>, usize> =
    fold_graph(
        |mut acc, cls, _pat| {
            *acc.entry(cls.clone()).or_insert(0) += 1;
            acc
        },
        HashMap::new(),
        &view,
    );
```

`fold_graph` takes `&GraphView` (shared reference) — it reads but does not consume.

---

## 5. `map_with_context`

### Haskell

```haskell
mapWithContext :: GraphClassifier extra v
               -> (GraphQuery v -> Pattern v -> Pattern v)
               -> GraphView extra v -> GraphView extra v
```

### Rust

```rust
pub fn map_with_context<Extra, V: GraphValue>(
    classifier: &GraphClassifier<Extra, V>,
    f: impl Fn(&GraphQuery<V>, Pattern<V>) -> Pattern<V>,
    view: GraphView<Extra, V>,
) -> GraphView<Extra, V> { ... }
```

**Snapshot semantics** — the mapping function `f` receives a reference to the
`GraphQuery` derived from the *original* view, not from an incrementally updated state.
All elements are transformed against the same source. The Rust implementation must take
care to derive the snapshot query from `view.view_query` before the transformation loop
begins, and pass a reference to that snapshot into each call to `f`.

```rust
// Inside the implementation:
let snapshot_query = &view.view_query;  // snapshot taken before any transformation
let new_elements = view.view_elements
    .into_iter()
    .map(|(cls, pat)| {
        let new_pat = f(snapshot_query, pat);
        (cls, new_pat)
    })
    .collect();
```

**`f` takes `Pattern<V>` by value** (moves the element in). This is the correct default:
the mapper produces a new element, consuming the old one. If callers need the original
alongside the transformed result, they clone before passing.

**Annotation-count enrichment example**:

```rust
fn enrich_with_annotation_count(
    query: &GraphQuery<Subject>,
    node: Pattern<Subject>,
) -> Pattern<Subject> {
    let count = query_annotations_of(&canonical_classifier(), query, &node).len();
    set_annotation_count(count, node)
}

let view = map_with_context(&canonical_classifier(), enrich_with_annotation_count, view);
```

---

## 6. `para_graph` and `para_graph_fixed`

### Haskell

```haskell
paraGraph :: (GraphQuery v -> Pattern v -> [r] -> r)
          -> GraphView extra v
          -> Map (Id v) r

paraGraphFixed :: (r -> r -> Bool)
               -> (GraphQuery v -> Pattern v -> [r] -> r)
               -> r
               -> GraphView extra v
               -> Map (Id v) r
```

### Rust

```rust
pub fn para_graph<Extra, V: GraphValue, R>(
    f: impl Fn(&GraphQuery<V>, &Pattern<V>, &[R]) -> R,
    view: &GraphView<Extra, V>,
) -> HashMap<V::Id, R> { ... }

pub fn para_graph_fixed<Extra, V: GraphValue, R: Clone>(
    converged: impl Fn(&R, &R) -> bool,
    f: impl Fn(&GraphQuery<V>, &Pattern<V>, &[R]) -> R,
    init: R,
    view: &GraphView<Extra, V>,
) -> HashMap<V::Id, R> { ... }
```

Both functions take `&GraphView` (shared reference) — they read the graph structure but
do not consume it.

**Processing order for `para_graph`**: for DAGs, a topological sort of `view.view_elements`
before applying `f` gives each element access to previously computed neighbor results.
For general graphs (with cycles), `para_graph_fixed` is the correct choice. Document this
clearly: callers using `para_graph` on a cyclic graph will get results that depend on
iteration order, which is insertion-history-dependent.

**`para_graph_fixed` — convergence loop**:

```rust
// Sketch of the fixpoint loop
let mut current: HashMap<V::Id, R> = /* initialize all elements to `init` */;
loop {
    let next = /* one pass of `f` over all elements using `current` as neighbor results */;
    let stable = current.iter().all(|(id, old)| {
        next.get(id).map_or(false, |new| converged(old, new))
    });
    current = next;
    if stable { break; }
}
current
```

**`R: Clone` on `para_graph_fixed`**: required because the convergence loop reads the
previous round's values while computing the next. The current map must be cloned (or
at minimum, individual `R` values must be cloned) to compare old and new.

**Float convergence example**:

```rust
let pagerank = para_graph_fixed(
    |old, new| (old - new).abs() < 1e-6,
    compute_pagerank_step,
    1.0_f64 / node_count as f64,
    &view,
);
```

**`HashMap` key requirements**: `V::Id` must implement `Eq + Hash`. Add this bound where
needed:

```rust
pub fn para_graph<Extra, V: GraphValue, R>(
    f: impl Fn(&GraphQuery<V>, &Pattern<V>, &[R]) -> R,
    view: &GraphView<Extra, V>,
) -> HashMap<V::Id, R>
where
    V::Id: Eq + Hash,
{ ... }
```

---

## Rust-specific considerations

### Ownership and `GraphView` consumption

The Haskell design is lazy: `GraphView → GraphView` transformations compose without
copying. In Rust, laziness requires either:

- **Eager allocation per step** (simplest): each transformation produces a new `Vec` of
  classified elements. This is the recommended starting point — correct, simple, and easy
  to profile.
- **Iterator-based laziness** (zero-copy): represent `view_elements` as a boxed iterator
  rather than a `Vec`. Transformation functions chain iterators. Materialization collects.

The iterator approach mirrors the Haskell semantics more closely but is significantly
harder to express in Rust because chained closures produce different opaque types at each
step. `Box<dyn Iterator<...>>` or `impl Iterator` return positions are workable but add
complexity. 

**Recommendation**: start with `Vec`-based eager allocation. Profile before switching.
The overhead is typically dominated by the work done inside the mapping functions, not by
the allocation of the element list.

### `#[inline]` placement

The Haskell proposal notes that `INLINE` pragmas are needed because GHC cannot inline
through record field function pointers. In Rust, the analogous concern is that `dyn Fn`
trait objects (in `CategoryMappers`) prevent monomorphization and inlining.

Where performance matters, prefer `impl Fn` (generic parameter) over `Box<dyn Fn>` for
the per-element callbacks passed to `map_graph`, `filter_graph`, etc.:

```rust
// Preferred — monomorphizable, inlinable
pub fn map_all_graph<Extra, V: GraphValue>(
    f: impl Fn(Pattern<V>) -> Pattern<V>,  // ← generic, not boxed
    view: GraphView<Extra, V>,
) -> GraphView<Extra, V> { ... }

// Only use Box<dyn Fn> when the function type must be stored (e.g., in CategoryMappers)
```

Annotate the transformation functions themselves with `#[inline]` so the compiler can
inline them at call sites:

```rust
#[inline]
pub fn map_all_graph<Extra, V: GraphValue>( ... ) -> GraphView<Extra, V> { ... }
```

### `Clone` and `Copy` on `GraphClass`

`GraphClass<Extra>` is used as a map key in `fold_graph` examples. Derive `Clone`, `Hash`,
`Eq`, and `PartialEq` on `GraphClass`:

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GraphClass<Extra> {
    Node,
    Relationship,
    Annotation,
    Walk,
    Other(Extra),
}
```

`Copy` is derivable only if `Extra: Copy`. For the canonical case (`Extra = !`), the
compiler will derive `Copy` automatically. For user-defined `Extra` types, derive `Copy`
if it makes sense for that type.

### Module layout

Following the Haskell Option C recommendation:

| Haskell | Rust crate/module |
|---|---|
| `Pattern.Core.unfold` | `pattern_core::unfold` |
| `Pattern.Graph.GraphView` | `pattern_graph::GraphView` |
| `Pattern.Graph.materialize` | `pattern_graph::materialize` |
| `Pattern.Graph.fromPatternGraph` | `pattern_graph::from_pattern_graph` |
| `Pattern.Graph.fromGraphLens` | `pattern_graph::from_graph_lens` |
| `Pattern.Graph.Transform.*` | `pattern_graph::transform::*` |

All transformation functions (`map_graph`, `map_all_graph`, `filter_graph`, `fold_graph`,
`map_with_context`, `para_graph`, `para_graph_fixed`) live in `pattern_graph::transform`.
`unfold_graph` lives there too, despite being construction-adjacent — it wraps `unfold`
for graph use and belongs with the other graph-level operations.

---

## Summary of Rust translation decisions

| Decision | Choice | Rationale |
|---|---|---|
| `GraphView` lifetime | `Arc`-wrapped owned closures in `GraphQuery`; no `'a` on `GraphView` | Simplifies pipeline composition |
| `materialize` ownership | Consumes `GraphView` | Normal case; clone if reuse needed |
| `unfold` recursion | Iterative with explicit work stack | Avoids stack overflow on deep hierarchies |
| `map_graph` parameters | `CategoryMappers` struct with struct-update default | Replaces 5 positional function params |
| `fold_graph` accumulator | Explicit `init` + fold function (Option A) | Works for `HashMap` and non-`Add` types |
| `filter_graph` predicate | Takes `&GraphClass`, `&Pattern` | Reads only; no ownership needed |
| `map_with_context` | `f` takes `Pattern<V>` by value | Mapper produces new element |
| Snapshot in `map_with_context` | Snapshot reference derived before loop | Enforces snapshot semantics |
| `para_graph_fixed` bound | `R: Clone` | Required for convergence comparison |
| `HashMap` keys | `V::Id: Eq + Hash` bound | Required for result map |
| Laziness | Eager `Vec` per step | Start simple; profile before optimizing |
| Inlining | `impl Fn` for per-element callbacks; `#[inline]` on transform fns | Enables monomorphization |
| `GraphClass` derives | `Clone, Debug, PartialEq, Eq, Hash` | Required for map keys and comparisons |
