# Porting Guide: GraphClassifier — pattern-hs → pattern-rs

**Source**: `pattern-hs` / `libs/pattern/src/Pattern/Graph/GraphClassifier.hs` and `libs/pattern/src/Pattern/PatternGraph.hs`
**Target**: `pattern-rs`
**Feature**: GraphClassifier (Feature 034)
**Fidelity goal**: Behavioral equivalence. The Rust port must produce identical classification results and identical container contents for the same input data. API surface should follow Rust idioms, not Haskell syntax.

---

## Overview

GraphClassifier introduces a unified, injectable classification abstraction over `Pattern<V>` values. It defines five named structural categories (node, relationship, annotation, walk, other), separates classification logic from value identity, and replaces the previous hardcoded arity check with a structural shape validator that correctly handles walks.

The Haskell implementation consists of two modules:

- `Pattern.Graph.GraphClassifier` — the core vocabulary: `GraphClass<Extra>`, `GraphClassifier<Extra, V>`, `classify_by_shape`, `canonical_classifier`, and the simplified `GraphValue` trait.
- `Pattern.PatternGraph` — the eager materialized container: `PatternGraph<Extra, V>` and its construction/merge functions.

Both modules must be ported. The order is: GraphClassifier types and logic first, then PatternGraph.

---

## Conventions for This Document

Haskell names are given first, then the Rust equivalent. Type signatures are shown in both languages. Where a decision was left open in the original proposal but resolved in the Haskell implementation, the implemented behavior is what the Rust port must follow.

Rust field and function names should use `snake_case`. Type names should use `PascalCase`. The Rust `Pattern<V>` type already exists in pattern-rs; its fields are `value: V` and `elements: Vec<Pattern<V>>`.

---

## Part 1: `GraphClass<Extra>` — Classification Vocabulary

### Haskell

```haskell
data GraphClass extra
  = GNode
  | GRelationship
  | GAnnotation
  | GWalk
  | GOther extra
  deriving (Eq, Show, Functor, Traversable, Foldable)
```

### Rust

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphClass<Extra> {
    GNode,
    GRelationship,
    GAnnotation,
    GWalk,
    GOther(Extra),
}
```

### Notes

Keep the variant names identical to the Haskell source. These names are part of the shared vocabulary across pattern-hs, pattern-rs, and future ports. Renaming them (e.g. to `Node`, `Relationship`) would break cross-project alignment.

The canonical form is `GraphClass<()>` — `GOther(())` is the catch-all bucket in default usage. Custom classifiers parameterize with a domain-specific enum in place of `()`.

The Haskell `Functor`/`Foldable`/`Traversable` instances exist to map over the `Extra` payload. In Rust, implement a `map_other` method instead, since Rust does not have higher-kinded types:

```rust
impl<Extra> GraphClass<Extra> {
    pub fn map_other<F, B>(self, f: F) -> GraphClass<B>
    where
        F: FnOnce(Extra) -> B,
    {
        match self {
            GraphClass::GNode => GraphClass::GNode,
            GraphClass::GRelationship => GraphClass::GRelationship,
            GraphClass::GAnnotation => GraphClass::GAnnotation,
            GraphClass::GWalk => GraphClass::GWalk,
            GraphClass::GOther(e) => GraphClass::GOther(f(e)),
        }
    }
}
```

---

## Part 2: `GraphValue` Trait — Simplified Identity Contract

### Haskell

```haskell
class Ord (Id v) => GraphValue v where
  type Id v
  identify :: v -> Id v
```

The `classify` method that existed in the previous version of this typeclass has been removed. Classification is no longer a responsibility of the value type.

The `GraphValue` trait lives in `Pattern.Graph.GraphClassifier`, not in `Pattern.PatternGraph`.

### Rust

```rust
pub trait GraphValue {
    type Id: Ord + Clone + std::hash::Hash;
    fn identify(&self) -> &Self::Id;
}
```

The `Hash` bound is required because `Id` is used as a `HashMap` key in `PatternGraph`. The `Clone` bound is needed to insert keys into maps.

### Subject instance

The `Subject` instance in Haskell uses `Symbol` as `Id`. In the Rust canonical types, `Subject.identity` is a `String`. The `GraphValue` impl for `Subject` should use `String` as `Id`:

```rust
impl GraphValue for Subject {
    type Id = String;
    fn identify(&self) -> &String {
        &self.identity
    }
}
```

---

## Part 3: `GraphClassifier<Extra, V>` — Injectable Classification Logic

### Haskell

```haskell
data GraphClassifier extra v = GraphClassifier
  { classify :: Pattern v -> GraphClass extra
  }
```

### Rust

The Haskell record-of-functions maps directly to a Rust struct holding a boxed closure:

```rust
pub struct GraphClassifier<Extra, V> {
    pub classify: Box<dyn Fn(&Pattern<V>) -> GraphClass<Extra>>,
}
```

**Ownership decision**: The closure takes `&Pattern<V>` (a shared borrow). Classification is a read-only operation; it must not consume the pattern. The `PatternGraph` construction path borrows the classifier while iterating patterns, so the classifier itself should be borrowed at construction time rather than moved.

**`'static` bound**: If classifiers need to be stored in structs that are `Send` or that outlive a local scope, the closure will need a `'static` bound. For the initial port, use `'static` as the default and document that lifetimed classifiers are a possible future refinement.

```rust
pub struct GraphClassifier<Extra, V> {
    pub classify: Box<dyn Fn(&Pattern<V>) -> GraphClass<Extra> + 'static>,
}
```

**Constructor helper**: Provide a `new` function to avoid callers writing `Box::new(...)` directly:

```rust
impl<Extra, V> GraphClassifier<Extra, V> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&Pattern<V>) -> GraphClass<Extra> + 'static,
    {
        GraphClassifier { classify: Box::new(f) }
    }
}
```

---

## Part 4: `classify_by_shape` — Walk-Aware Classification Logic

This is the most behaviorally significant function in the port. Get it right before moving to `PatternGraph`.

### Haskell

```haskell
classifyByShape :: GraphValue v => Pattern v -> GraphClass ()
classifyByShape (Pattern _ els)
  | null els                                              = GNode
  | length els == 1                                       = GAnnotation
  | length els == 2 && all isNodeLike els                 = GRelationship
  | length els >= 1 && all isRelationshipLike els
      && isValidWalk els                                  = GWalk
  | otherwise                                             = GOther ()
  where
    isNodeLike (Pattern _ inner) = null inner
    isRelationshipLike (Pattern _ inner) =
        length inner == 2 && all isNodeLike inner
```

The walk case requires that all elements are structurally relationship-shaped AND that they form a valid chain. Arity alone is insufficient — a star pattern (multiple relationships sharing a hub node but not chaining end-to-end) has the right arity but must fall to `GOther`.

### Walk validation: `isValidWalk`

```haskell
isValidWalk :: GraphValue v => [Pattern v] -> Bool
isValidWalk [] = False
isValidWalk rels = not (null (foldl step [] rels))
  where
    step [] (Pattern _ [a, b]) = [a, b]
    step active (Pattern _ [a, b]) =
      let fromA = if any (\x -> identify (value a) == identify (value x)) active
                  then [b] else []
          fromB = if any (\x -> identify (value b) == identify (value x)) active
                  then [a] else []
      in fromA ++ fromB
    step _ _ = []
```

**What this algorithm does**: It maintains a frontier of "live endpoints" — nodes that are reachable at the current end of the chain. Starting from the first relationship, both endpoint nodes are live. For each subsequent relationship, a node is added to the new frontier only if the node at the other end of that relationship matches something already in the frontier (direction-agnostic). If the frontier ever becomes empty, the walk is invalid (a disconnected hop was found). An empty final frontier means the last relationship did not connect.

**Important**: The `value` function in the Haskell code above refers to accessing the `v` field of `Pattern v`, i.e. `Pattern.value` in Rust terms. The comparison uses `identify` from `GraphValue`, not structural equality.

### Rust

```rust
pub fn classify_by_shape<V: GraphValue>(pattern: &Pattern<V>) -> GraphClass<()> {
    let els = &pattern.elements;
    if els.is_empty() {
        GraphClass::GNode
    } else if els.len() == 1 {
        GraphClass::GAnnotation
    } else if els.len() == 2 && els.iter().all(is_node_like) {
        GraphClass::GRelationship
    } else if els.iter().all(is_relationship_like) && is_valid_walk::<V>(els) {
        GraphClass::GWalk
    } else {
        GraphClass::GOther(())
    }
}

fn is_node_like<V>(p: &Pattern<V>) -> bool {
    p.elements.is_empty()
}

fn is_relationship_like<V>(p: &Pattern<V>) -> bool {
    p.elements.len() == 2 && p.elements.iter().all(is_node_like)
}

fn is_valid_walk<V: GraphValue>(rels: &[Pattern<V>]) -> bool {
    if rels.is_empty() {
        return false;
    }
    // frontier holds the ids of currently reachable chain endpoints
    let mut frontier: Vec<&V::Id> = Vec::new();
    for rel in rels {
        if rel.elements.len() != 2 {
            return false;
        }
        let a = rel.elements[0].value.identify();
        let b = rel.elements[1].value.identify();
        if frontier.is_empty() {
            // first relationship: both endpoints are live
            frontier = vec![a, b];
        } else {
            let a_matches = frontier.iter().any(|x| *x == a);
            let b_matches = frontier.iter().any(|x| *x == b);
            let mut next: Vec<&V::Id> = Vec::new();
            if a_matches { next.push(b); }
            if b_matches { next.push(a); }
            frontier = next;
        }
    }
    !frontier.is_empty()
}
```

**The frontier grows**: Note that both `a` and `b` can match the frontier independently. This means a single relationship that connects two currently-live endpoints keeps both directions alive. The Haskell implementation uses list concatenation (`fromA ++ fromB`), which the Rust version mirrors with conditional pushes to `next`.

---

## Part 5: `canonical_classifier` — The Standard Classifier

### Haskell

```haskell
canonicalClassifier :: GraphValue v => GraphClassifier () v
canonicalClassifier = GraphClassifier { classify = classifyByShape }
```

Note: The implementation made `canonicalClassifier` polymorphic over any `GraphValue v`, not pinned to `Subject`. The Rust port should do the same.

### Rust

```rust
pub fn canonical_classifier<V: GraphValue + 'static>() -> GraphClassifier<(), V> {
    GraphClassifier::new(|p| classify_by_shape(p))
}
```

---

## Part 6: `from_test_node` — `GraphLens` Compatibility Bridge

### Haskell

```haskell
fromTestNode :: (Pattern v -> Bool) -> GraphClassifier () v
fromTestNode testNode = GraphClassifier
  { classify = \p -> if testNode p then GNode else GOther () }
```

This function constructs a two-category classifier — the `GraphLens` specialization — from a predicate. It is the bridge that allows existing `GraphLens` code, which was built around a single `isNode` predicate, to delegate to `GraphClassifier` internally without changing its public API.

### Rust

```rust
pub fn from_test_node<V, F>(test_node: F) -> GraphClassifier<(), V>
where
    F: Fn(&Pattern<V>) -> bool + 'static,
{
    GraphClassifier::new(move |p| {
        if test_node(p) {
            GraphClass::GNode
        } else {
            GraphClass::GOther(())
        }
    })
}
```

---

## Part 7: `PatternGraph<Extra, V>` — Eager Materialized Container

### Type definition

The `PatternGraph` type has an `extra` type parameter that was resolved from an open question in the proposal — the implementation chose to store the `extra` tag alongside the pattern in `pg_other`.

### Haskell

```haskell
data PatternGraph extra v = PatternGraph
  { pgNodes         :: Map (Id v) (Pattern v)
  , pgRelationships :: Map (Id v) (Pattern v)
  , pgWalks         :: Map (Id v) (Pattern v)
  , pgAnnotations   :: Map (Id v) (Pattern v)
  , pgOther         :: Map (Id v) (extra, Pattern v)
  , pgConflicts     :: Map (Id v) [Pattern v]
  }
```

### Rust

```rust
use std::collections::HashMap;

pub struct PatternGraph<Extra, V: GraphValue> {
    pub pg_nodes:         HashMap<V::Id, Pattern<V>>,
    pub pg_relationships: HashMap<V::Id, Pattern<V>>,
    pub pg_walks:         HashMap<V::Id, Pattern<V>>,
    pub pg_annotations:   HashMap<V::Id, Pattern<V>>,
    pub pg_other:         HashMap<V::Id, (Extra, Pattern<V>)>,
    pub pg_conflicts:     HashMap<V::Id, Vec<Pattern<V>>>,
}
```

**`pg_other` storage shape**: Stores `(Extra, Pattern<V>)` tuples, preserving the classifier's `extra` tag alongside the pattern. Do not simplify to `HashMap<V::Id, Pattern<V>>` — the tag is part of the contract.

**`pg_conflicts`**: Patterns that fail reconciliation (key collision where the reconciliation policy does not produce a winner) are stored here rather than dropped. This is not in the original proposal but is part of the implemented behavior and must be ported.

---

## Part 8: `empty` and Construction Functions

### Haskell

```haskell
empty :: PatternGraph extra v

fromPatterns
  :: (GraphValue v, ...)
  => GraphClassifier extra v
  -> [Pattern v]
  -> PatternGraph extra v

fromPatternsWithPolicy
  :: (GraphValue v, ...)
  => GraphClassifier extra v
  -> ReconciliationPolicy (MergeStrategy v)
  -> [Pattern v]
  -> PatternGraph extra v

merge
  :: (GraphValue v, ...)
  => GraphClassifier extra v
  -> Pattern v
  -> PatternGraph extra v
  -> PatternGraph extra v

mergeWithPolicy
  :: (GraphValue v, ...)
  => GraphClassifier extra v
  -> ReconciliationPolicy (MergeStrategy v)
  -> Pattern v
  -> PatternGraph extra v
  -> PatternGraph extra v
```

`fromPatterns` defaults to `LastWriteWins` reconciliation. `fromPatternsWithPolicy` is the explicit-policy variant. The same split applies to `merge` / `mergeWithPolicy`.

### Rust

```rust
impl<Extra, V> PatternGraph<Extra, V>
where
    V: GraphValue,
    // ... reconciliation bounds
{
    pub fn empty() -> Self { ... }

    pub fn merge(
        classifier: &GraphClassifier<Extra, V>,
        pattern: Pattern<V>,
        graph: Self,
    ) -> Self { ... }

    pub fn merge_with_policy(
        classifier: &GraphClassifier<Extra, V>,
        policy: &ReconciliationPolicy,
        pattern: Pattern<V>,
        graph: Self,
    ) -> Self { ... }
}

pub fn from_patterns<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    patterns: impl IntoIterator<Item = Pattern<V>>,
) -> PatternGraph<Extra, V>
where
    V: GraphValue,
    // ... reconciliation bounds
{ ... }

pub fn from_patterns_with_policy<Extra, V>(
    classifier: &GraphClassifier<Extra, V>,
    policy: &ReconciliationPolicy,
    patterns: impl IntoIterator<Item = Pattern<V>>,
) -> PatternGraph<Extra, V>
where
    V: GraphValue,
    // ... reconciliation bounds
{ ... }
```

**Classifier is borrowed**: The classifier is borrowed during construction, not consumed. It is only needed at insertion time to determine which map a pattern belongs in.

**Construction logic**: For each pattern, call `(classifier.classify)(&pattern)` to get the `GraphClass`. Then dispatch:

- `GNode` → insert into `pg_nodes` (with reconciliation on key collision)
- `GRelationship` → insert into `pg_relationships`
- `GAnnotation` → insert into `pg_annotations`
- `GWalk` → insert into `pg_walks`
- `GOther(extra)` → insert `(extra, pattern)` into `pg_other`

Reconciliation failures go into `pg_conflicts`. The Haskell implementation uses a `ReconciliationPolicy` type from `Pattern.Reconcile`. Port this type or reference whatever equivalent already exists in pattern-rs.

---

## Part 9: Module Layout

Mirror the Haskell module structure. Suggested crate layout:

```
src/
├── graph/
│   ├── mod.rs
│   ├── graph_classifier.rs   ← GraphClass, GraphClassifier, GraphValue,
│   │                            classify_by_shape, canonical_classifier,
│   │                            from_test_node
│   └── graph_lens.rs         ← existing GraphLens, re-derived using from_test_node
└── pattern_graph.rs          ← PatternGraph, empty, merge, from_patterns, etc.
```

`pattern_graph.rs` imports from `graph::graph_classifier`. The `GraphValue` trait lives in `graph_classifier`, not in `pattern_graph`. This matches the Haskell module split.

---

## Part 10: Test Coverage Requirements

All test cases from the Haskell test suite must have Rust equivalents. The behavioral contract must be identical.

### `classify_by_shape` tests (from `GraphClassifierSpec.hs`)

These test cases must pass:

1. An atomic pattern (0 elements) classifies as `GNode`.
2. A pattern with 1 element classifies as `GAnnotation`.
3. A pattern with 2 node elements classifies as `GRelationship`.
4. A sequence of properly chaining relationships classifies as `GWalk`. The test uses a chain of three relationships `r1=[A,B]`, `r2=[B,C]`, `r3=[D,C]` (note `r3` connects via `C`, testing direction-agnostic chaining).
5. A star pattern — multiple relationships sharing a center node but not chaining end-to-end (e.g. `r1=[A,B]`, `r2=[A,C]`) — classifies as `GOther(())`.
6. A pattern with 3 node elements (not all relationship-shaped) classifies as `GOther(())`.

### `PatternGraph` tests (from `PatternGraphSpec.hs`)

1. `empty` graph has all six maps empty.
2. Merging a node adds it to `pg_nodes`.
3. Merging a relationship adds it to `pg_relationships`.
4. `from_patterns` builds correct node and relationship counts from a mixed list.
5. An unrecognized pattern (3 node elements) appears in `pg_other`, not in any other map.
6. A pattern classified as `GOther(extra)` stores `(extra, pattern)` in `pg_other`.
7. `pg_conflicts` receives a pattern when a reconciliation collision occurs.

### Custom classifier test

Verify that a user-defined `GraphClassifier` with a custom `Extra` type routes patterns to `pg_other` with the correct `extra` tag preserved.

---

## Part 11: Key Behavioral Invariants

These are the properties the port must preserve. They can serve as property-based test generators.

1. **Total classification**: Every pattern is classified into exactly one bucket. `from_patterns` never panics or silently drops a pattern. Every input pattern appears in exactly one of `pg_nodes`, `pg_relationships`, `pg_annotations`, `pg_walks`, `pg_other`, or `pg_conflicts`.

2. **Deterministic shape**: For a given classifier and input list, `from_patterns` always produces the same `PatternGraph`.

3. **`GNode` ↔ empty elements**: A pattern classifies as `GNode` if and only if `pattern.elements.is_empty()`.

4. **`GAnnotation` ↔ one element**: A pattern classifies as `GAnnotation` if and only if `pattern.elements.len() == 1`.

5. **Walk requires structural validity**: A pattern with N ≥ 3 elements where all elements are relationship-shaped but do not form a contiguous chain must classify as `GOther`, not `GWalk`.

6. **`canonical_classifier` consistency**: `canonical_classifier` applied to a pattern produces the same result as calling `classify_by_shape` directly.

---

## Part 12: What Not to Port (Out of Scope)

The following are present in the Haskell implementation but are forward-looking stubs for the next feature (GraphTransform) and should not be ported as part of this feature:

- `to_graph_view` / `materialize` — these bridge to `GraphView`, which belongs to GraphTransform.
- `from_pattern_graph` returning a `GraphQuery` — belongs to GraphQuery.

Port only the types and functions described in Parts 1–9 above.

---

## Summary: Decision Log

| Decision | Choice | Rationale |
|---|---|---|
| `classifyByArity` renamed | `classify_by_shape` | Arity alone cannot distinguish walks from star patterns |
| `canonicalClassifier` signature | Polymorphic over `GraphValue v` | More general than `Subject`-specific; matches implementation |
| `pg_other` storage | `HashMap<Id, (Extra, Pattern<V>)>` | Preserves classifier tag; resolved open question |
| `PatternGraph` type parameter | `PatternGraph<Extra, V>` | `extra` propagates from classifier to container |
| `GraphValue` trait location | `graph_classifier` module | Matches Haskell: trait lives with classifier, not container |
| Classifier in construction | Borrowed (`&GraphClassifier`) | Needed only at insertion time; container owns its patterns |
| `pg_conflicts` | Include in port | Part of implemented behavior; conflicts must not be silently dropped |
| `to_graph_view`, `from_pattern_graph` | Do not port | Belong to subsequent features (GraphTransform, GraphQuery) |
