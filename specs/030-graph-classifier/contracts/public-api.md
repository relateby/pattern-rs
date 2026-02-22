# Public API Contract: Graph Classifier Port (030)

**Type**: Rust library public interface
**Crate**: `pattern-core`
**Date**: 2026-02-22

---

## Overview

This feature adds three new modules to `pattern-core` and modifies one existing type. All new public items are exported from `pattern_core` (the crate root) or from their module paths. No existing public APIs are changed.

---

## New Module: `pattern_core::graph::graph_classifier`

### Type: `GraphClass<Extra>`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphClass<Extra> {
    GNode,
    GRelationship,
    GAnnotation,
    GWalk,
    GOther(Extra),
}

impl<Extra> GraphClass<Extra> {
    pub fn map_other<F, B>(self, f: F) -> GraphClass<B>
    where F: FnOnce(Extra) -> B;
}
```

**Invariants**:
- `GNode` if and only if `pattern.elements.is_empty()`
- `GAnnotation` if and only if `pattern.elements.len() == 1`
- `GRelationship` if and only if `pattern.elements.len() == 2` and both elements are `GNode`-shaped
- `GWalk` only if all elements are `GRelationship`-shaped and form a valid end-to-end chain
- `GOther(_)` for all other shapes

---

### Trait: `GraphValue`

```rust
pub trait GraphValue {
    type Id: Ord + Clone + std::hash::Hash;
    fn identify(&self) -> &Self::Id;
}
```

**Implementations provided**:
- `GraphValue for Subject` — `type Id = Symbol`, returns `&self.identity`

**Constraints on implementors**:
- `Id` must satisfy `Ord + Clone + Hash`: `Ord` is a superclass constraint on `GraphValue` itself (matching `class Ord (Id v) => GraphValue v` in the Haskell reference) and is required by all downstream graph algorithms (`bfs`, `dfs`, `topologicalSort`, etc.) that use `Set<Id>` for visited tracking; `Hash` is required for `HashMap` use in `PatternGraph`
- `Id` must be `Clone` (for insertion into maps)
- `identify` must return a reference that is stable for the lifetime of `&self`

---

### Type: `GraphClassifier<Extra, V>`

```rust
pub struct GraphClassifier<Extra, V> {
    pub classify: Box<dyn Fn(&Pattern<V>) -> GraphClass<Extra> + 'static>,
}

impl<Extra, V> GraphClassifier<Extra, V> {
    pub fn new<F>(f: F) -> Self
    where F: Fn(&Pattern<V>) -> GraphClass<Extra> + 'static;
}
```

**Contract**:
- The `classify` function must be pure (no side effects) within a single `PatternGraph` construction
- The same classifier instance can be used for multiple `merge` / `from_patterns` calls
- `classify` takes `&Pattern<V>` (shared borrow); it must not consume the pattern

---

### Functions

```rust
/// Classifies a pattern by its structural shape.
pub fn classify_by_shape<V: GraphValue>(pattern: &Pattern<V>) -> GraphClass<()>;

/// Returns the standard shape-based classifier.
pub fn canonical_classifier<V: GraphValue + 'static>() -> GraphClassifier<(), V>;

/// Wraps a node predicate into a two-category classifier (GNode vs GOther(())).
pub fn from_test_node<V, F>(test_node: F) -> GraphClassifier<(), V>
where F: Fn(&Pattern<V>) -> bool + 'static;
```

**`classify_by_shape` is the canonical reference**: All tests measure behavioral equivalence against `classify_by_shape`. The `canonical_classifier` is a wrapper around it and must produce identical results.

---

## New Module: `pattern_core::reconcile`

### Traits

```rust
pub trait HasIdentity<V, I: Ord> {
    fn identity(v: &V) -> &I;
}

pub trait Mergeable {
    type MergeStrategy;
    fn merge(strategy: &Self::MergeStrategy, a: Self, b: Self) -> Self;
}

pub trait Refinable {
    fn is_refinement_of(sup: &Self, sub: &Self) -> bool;
}
```

**Implementations provided**:
- `HasIdentity<Subject, Symbol>` — delegates to `subject.identity`
- `Mergeable for Subject` — `MergeStrategy = SubjectMergeStrategy`
- `Refinable for Subject` — checks identity match and label/property subset

---

### Type: `ReconciliationPolicy<S>`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReconciliationPolicy<S> {
    LastWriteWins,
    FirstWriteWins,
    Merge(ElementMergeStrategy, S),
    Strict,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementMergeStrategy {
    ReplaceElements,
    AppendElements,
    UnionElements,
}
```

---

### Function

```rust
pub fn reconcile<V>(
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    pattern: &Pattern<V>,
) -> Result<Pattern<V>, ReconcileError>
where
    V: HasIdentity<V, <V as GraphValue>::Id> + Mergeable + Refinable + Eq + Clone;
```

**Contract**:
- `LastWriteWins` and `FirstWriteWins` never return `Err`
- `Strict` returns `Err` if any duplicate identity has different content
- `Merge(...)` never returns `Err`

---

## New Module: `pattern_core::pattern_graph`

### Type: `PatternGraph<Extra, V: GraphValue>`

```rust
pub struct PatternGraph<Extra, V: GraphValue> {
    pub pg_nodes:         HashMap<V::Id, Pattern<V>>,
    pub pg_relationships: HashMap<V::Id, Pattern<V>>,
    pub pg_walks:         HashMap<V::Id, Pattern<V>>,
    pub pg_annotations:   HashMap<V::Id, Pattern<V>>,
    pub pg_other:         HashMap<V::Id, (Extra, Pattern<V>)>,
    pub pg_conflicts:     HashMap<V::Id, Vec<Pattern<V>>>,
}
```

**Invariants**:
- Every inserted pattern appears in exactly one of the six fields
- No pattern is ever silently dropped
- `pg_other` values always store `(extra_tag, original_pattern)` — the tag from the classifier
- Identity collisions that cannot be reconciled go to `pg_conflicts`

---

### Functions

```rust
impl<Extra, V: GraphValue> PatternGraph<Extra, V> {
    pub fn empty() -> Self;
}

// Inserts one pattern using LastWriteWins policy.
pub fn merge<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    pattern: Pattern<V>,
    graph: PatternGraph<Extra, V>,
) -> PatternGraph<Extra, V>
where V: GraphValue + Eq + Clone + Mergeable + Refinable, ...;

// Inserts one pattern using the explicit policy.
pub fn merge_with_policy<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    pattern: Pattern<V>,
    graph: PatternGraph<Extra, V>,
) -> PatternGraph<Extra, V>
where V: GraphValue + Eq + Clone + Mergeable + Refinable, ...;

// Builds a graph from an iterable, using LastWriteWins.
pub fn from_patterns<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    patterns: impl IntoIterator<Item = Pattern<V>>,
) -> PatternGraph<Extra, V>
where V: GraphValue + Eq + Clone + Mergeable + Refinable, ...;

// Builds a graph from an iterable with an explicit policy.
pub fn from_patterns_with_policy<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    patterns: impl IntoIterator<Item = Pattern<V>>,
) -> PatternGraph<Extra, V>
where V: GraphValue + Eq + Clone + Mergeable + Refinable, ...;
```

---

## Modified Existing Type

### `subject::Symbol` — Add `Ord` and `PartialOrd`

```rust
// BEFORE (existing):
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Symbol(pub String);

// AFTER:
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Symbol(pub String);
```

**Change impact**: Additive only. No existing code is broken. `Symbol` gains lexicographic ordering via `String`'s natural ordering.

---

## `lib.rs` Exports

The following new items should be re-exported from the crate root (`lib.rs`):

```rust
pub mod graph;
pub mod reconcile;
pub mod pattern_graph;

pub use graph::graph_classifier::{GraphClass, GraphClassifier, GraphValue,
    classify_by_shape, canonical_classifier, from_test_node};
pub use reconcile::{ReconciliationPolicy, ElementMergeStrategy,
    HasIdentity, Mergeable, Refinable};
pub use pattern_graph::{PatternGraph, merge, merge_with_policy,
    from_patterns, from_patterns_with_policy};
```

---

## Backward Compatibility

No existing public APIs are changed. The `Combinable` trait and its implementations (`Subject`, `FirstSubject`, `LastSubject`, `EmptySubject`) are unaffected. The new `Ord` derive on `Symbol` is additive and non-breaking.
