# Data Model: Graph Classifier Port (030)

**Phase**: 1 — Design & Contracts
**Date**: 2026-02-22

---

## Overview

This feature introduces three new modules to `pattern-core` and one modification to an existing type. The modules are organized to mirror the Haskell reference:

```
Pattern.Graph.GraphClassifier  →  src/graph/graph_classifier.rs
Pattern.Reconcile              →  src/reconcile.rs
Pattern.PatternGraph           →  src/pattern_graph.rs
```

---

## Module 1: `src/graph/graph_classifier.rs`

### `GraphClass<Extra>` — Classification Vocabulary

An enum representing the five structural categories a pattern can belong to.

| Variant | Meaning | Shape Condition |
|---------|---------|-----------------|
| `GNode` | Atomic graph element | `elements.is_empty()` |
| `GRelationship` | Binary connection between two nodes | `elements.len() == 2` and both are `GNode`-shaped |
| `GAnnotation` | Decoration on a single element | `elements.len() == 1` |
| `GWalk` | Directed or undirected chain of relationships | All elements are `GRelationship`-shaped and they chain end-to-end |
| `GOther(Extra)` | Unrecognized or domain-specific | None of the above |

**Type parameter**: `Extra` — the payload type for the `GOther` variant. In the canonical classifier, `Extra = ()`. Custom classifiers parameterize with a domain-specific enum.

**Operations**:
- `map_other(f)` — maps a function over the `Extra` payload, producing `GraphClass<B>`. Matches Haskell's `Functor` instance without requiring HKTs.

---

### `GraphValue` — Identity Contract

A trait that value types must implement to be used with `GraphClassifier` and `PatternGraph`.

| Associated type / method | Description |
|--------------------------|-------------|
| `type Id: Ord + Clone + Hash` | The identity type, usable as a map key |
| `fn identify(&self) -> &Self::Id` | Extracts the stable identity from the value |

**Instance for `Subject`**:
- `type Id = Symbol`
- `fn identify(&self) -> &Symbol { &self.identity }`

**Note**: `Symbol` currently derives `Clone, PartialEq, Eq, Hash`. Adding `Ord + PartialOrd` is required by the trait bound and is a non-breaking addition to `subject.rs`.

---

### `GraphClassifier<Extra, V>` — Injectable Classification Strategy

A struct wrapping a boxed classification closure.

| Field | Type | Description |
|-------|------|-------------|
| `classify` | `Box<dyn Fn(&Pattern<V>) -> GraphClass<Extra> + 'static>` | The classification function |

**Constructor**: `GraphClassifier::new(f)` — avoids callers writing `Box::new(...)`.

**Lifetime**: `'static` bound enables classifiers to be stored in structs and passed across thread boundaries if needed.

---

### Functions in `graph_classifier.rs`

| Function | Signature | Description |
|----------|-----------|-------------|
| `classify_by_shape` | `fn classify_by_shape<V: GraphValue>(p: &Pattern<V>) -> GraphClass<()>` | Standard shape-based classifier |
| `canonical_classifier` | `fn canonical_classifier<V: GraphValue + 'static>() -> GraphClassifier<(), V>` | Returns a `GraphClassifier` backed by `classify_by_shape` |
| `from_test_node` | `fn from_test_node<V, F>(test_node: F) -> GraphClassifier<(), V>` | Bridge: wraps a node-predicate as a two-category classifier |

**`classify_by_shape` decision logic** (priority order):
1. No elements → `GNode`
2. 1 element → `GAnnotation`
3. 2 elements, both node-shaped → `GRelationship`
4. All elements are relationship-shaped AND `is_valid_walk(elements)` → `GWalk`
5. Otherwise → `GOther(())`

**`is_valid_walk` algorithm**: Frontier-based. Starts with both endpoints of the first relationship as live. For each subsequent relationship, advances the frontier to the opposite endpoint if one end matches the current frontier. If the frontier empties (disconnected hop), returns `false`. An empty final frontier also means invalid. Direction-agnostic: either endpoint can match.

---

## Module 2: `src/reconcile.rs`

### Traits

| Trait | Purpose |
|-------|---------|
| `HasIdentity<V, I>` | Value has a stable, comparable identity of type `I` |
| `Mergeable` | Value can be merged with another using a `MergeStrategy` |
| `Refinable` | Value can be checked for whether it is a "partial" reference of another |

**`HasIdentity<V, I>`**:
- `fn identity(v: &V) -> &I`
- Implemented for `Subject` with `I = Symbol`

**`Mergeable`**:
- `type MergeStrategy`
- `fn merge(strategy: &Self::MergeStrategy, a: Self, b: Self) -> Self`
- Implemented for `Subject` with `MergeStrategy = SubjectMergeStrategy`

**`Refinable`**:
- `fn is_refinement_of(sup: &Self, sub: &Self) -> bool` — true if `sub` is a partial reference to `sup`
- Implemented for `Subject`: same identity, and `sub`'s labels/properties are subsets of `sup`'s

---

### `ReconciliationPolicy<S>` — Policy Enum

| Variant | Behavior |
|---------|----------|
| `LastWriteWins` | The incoming pattern replaces the existing one |
| `FirstWriteWins` | The existing pattern is kept; incoming is ignored |
| `Merge(ElementMergeStrategy, S)` | Combine both patterns using the given strategies |
| `Strict` | Fail if duplicates have different content |

**`ElementMergeStrategy`**:
- `ReplaceElements` — later element list replaces earlier
- `AppendElements` — concatenate all element lists
- `UnionElements` — deduplicate by identity

**`SubjectMergeStrategy`** (for `Merge` variant when `V = Subject`):
- `label_merge: LabelMerge` — `UnionLabels | IntersectLabels | ReplaceLabels`
- `property_merge: PropertyMerge` — `ReplaceProperties | ShallowMerge | DeepMerge`

---

### `reconcile` Function

```
reconcile(policy, pattern) -> Result<Pattern<V>, ReconcileError<I, V>>
```

Normalizes a pattern by resolving duplicate identities according to the policy. Used internally by `PatternGraph`'s insert functions to decide which pattern wins when two share the same identity key.

**Note**: `PatternGraph` does not call the full recursive `reconcile` but uses a simplified two-occurrence reconciliation: it constructs a synthetic `Pattern(existing.value, [incoming])` and calls `reconcile` on that. This is the `twoOccurrences` pattern from the Haskell source.

---

## Module 3: `src/pattern_graph.rs`

### `PatternGraph<Extra, V>` — Materialized Graph Container

| Field | Type | Contents |
|-------|------|----------|
| `pg_nodes` | `HashMap<V::Id, Pattern<V>>` | Patterns classified as `GNode` |
| `pg_relationships` | `HashMap<V::Id, Pattern<V>>` | Patterns classified as `GRelationship` |
| `pg_walks` | `HashMap<V::Id, Pattern<V>>` | Patterns classified as `GWalk` |
| `pg_annotations` | `HashMap<V::Id, Pattern<V>>` | Patterns classified as `GAnnotation` |
| `pg_other` | `HashMap<V::Id, (Extra, Pattern<V>)>` | Patterns classified as `GOther`, with tag |
| `pg_conflicts` | `HashMap<V::Id, Vec<Pattern<V>>>` | Patterns that failed reconciliation |

**Key type**: `V::Id` — the value type's associated identity, used as the map key across all six collections.

**`pg_other` storage shape**: Stores `(Extra, Pattern<V>)` tuples, preserving the classifier's typed tag alongside the pattern. This is the resolved design decision from the porting guide (Part 7).

---

### Construction Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `PatternGraph::empty()` | `-> PatternGraph<Extra, V>` | All six maps empty |
| `PatternGraph::merge(classifier, pattern, graph)` | `-> PatternGraph<Extra, V>` | Insert one pattern, LastWriteWins |
| `PatternGraph::merge_with_policy(classifier, policy, pattern, graph)` | `-> PatternGraph<Extra, V>` | Insert one pattern, explicit policy |
| `from_patterns(classifier, patterns)` | `-> PatternGraph<Extra, V>` | Build from iterable, LastWriteWins |
| `from_patterns_with_policy(classifier, policy, patterns)` | `-> PatternGraph<Extra, V>` | Build from iterable, explicit policy |

---

### Insert Logic (per classification)

| Classification | Action |
|---------------|--------|
| `GNode` | Check identity collision in `pg_nodes`. Win/lose per policy or push to `pg_conflicts`. |
| `GRelationship` | First recursively merge each endpoint node (`elements[0]`, `elements[1]`) via `merge_with_policy`. Then insert the relationship into `pg_relationships`. |
| `GWalk` | First recursively merge each element (each component relationship) via `merge_with_policy`. Then insert the walk into `pg_walks`. |
| `GAnnotation` | First recursively merge the inner element (`elements[0]`) via `merge_with_policy`. Then insert the annotation into `pg_annotations`. |
| `GOther(extra)` | Insert `(extra, pattern)` into `pg_other`. Collision handled per policy on the pattern. |

**Conflict rule**: When reconciliation fails (policy returns `Left`/`Err`), push the incoming pattern to `pg_conflicts` under the identity key. Do not drop it.

---

## Modifications to Existing Types

### `subject.rs`: Add `Ord` and `PartialOrd` to `Symbol`

`Symbol` currently derives `Clone, PartialEq, Eq, Hash`. The `GraphValue::Id` bound requires `Ord + Clone + Hash`. Add:

```rust
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Symbol(pub String);
```

String's natural lexicographic ordering serves as the `Symbol` ordering. This is a non-breaking addition (no existing code uses `Symbol` in `Ord` contexts, so no behavior changes).

---

## Entity Relationships

```
GraphClassifier<Extra, V>
  │
  │ produces
  ▼
GraphClass<Extra>  ──── contains ────► Extra (user tag)
  │
  │ routes to
  ▼
PatternGraph<Extra, V>
  ├── pg_nodes:         HashMap<V::Id, Pattern<V>>
  ├── pg_relationships: HashMap<V::Id, Pattern<V>>
  ├── pg_walks:         HashMap<V::Id, Pattern<V>>
  ├── pg_annotations:   HashMap<V::Id, Pattern<V>>
  ├── pg_other:         HashMap<V::Id, (Extra, Pattern<V>)>
  └── pg_conflicts:     HashMap<V::Id, Vec<Pattern<V>>>

V: GraphValue  ─── provides Id ───► HashMap key
ReconciliationPolicy  ─── governs ───► collision resolution
```
