# Data Model: GraphTransform (032-graph-transform)

**Date**: 2026-02-23  
**Branch**: `032-graph-transform`  
**Reference**: `proposals/graph-transform-porting-guide.md`, spec Key Entities

---

## Core Types

### GraphView\<Extra, V\>

A universal graph-like interface: a query over the graph plus a list of classified elements.

| Field | Type | Meaning |
|-------|------|---------|
| `view_query` | `GraphQuery<V>` | Read-only query interface over the same graph (owned, e.g. via Rc) |
| `view_elements` | `Vec<(GraphClass<Extra>, Pattern<V>)>` | All elements in the view, each tagged with its classification |

**Invariants**:
- Every element in `view_elements` is classified by the same classifier that was used to build the view.
- `view_query` is consistent with the graph that produced `view_elements` (e.g. built from the same PatternGraph or lens).

**Clone**: If supported, cloning the view clones the query handle and the element list (or shares via Rc where applicable). Per research, transformations consume the view; clone only when caller needs to materialize twice or reuse.

---

### Substitution\<V\>

Policy for container elements (e.g. walks) when a contained element is removed by `filter_graph`.

| Variant | Meaning |
|---------|---------|
| `NoSubstitution` | Removed element leaves a gap; container is kept as-is |
| `ReplaceWith(Pattern<V>)` | Removed element is replaced by the given filler pattern |
| `RemoveContainer` | Containers that contained a removed element are themselves removed |

**Used by**: `filter_graph(classifier, predicate, substitution, view)`.

---

### CategoryMappers\<Extra, V\>

Per-category transformation functions for `map_graph`. Categories not overridden behave as identity.

| Field | Type | Meaning |
|-------|------|---------|
| `nodes` | `Box<dyn Fn(Pattern<V>) -> Pattern<V>>` (or equivalent) | Mapper for node-class elements |
| `relationships` | ditto | Mapper for relationship-class elements |
| `walks` | ditto | Mapper for walk-class elements |
| `annotations` | ditto | Mapper for annotation-class elements |
| `other` | `Box<dyn Fn(GraphClass<Extra>, Pattern<V>) -> Pattern<V>>` | Mapper for other-class elements (receives the extra tag) |

**Default**: `CategoryMappers::identity()` returns a struct where every category is the identity function. Callers use struct-update to replace only the categories they need.

---

### ReconciliationPolicy (existing)

From `reconcile` module; used by `materialize` and `unfold_graph`. No new variant introduced by this feature. Types: `LastWriteWins`, `FirstWriteWins`, `Strict`, `Merge(...)`.

---

### GraphClass\<Extra\> (existing)

From 030-graph-classifier. Must derive `Clone, Debug, PartialEq, Eq, Hash` for use as map keys in `fold_graph` and elsewhere. Variants: `Node`, `Relationship`, `Annotation`, `Walk`, `Other(Extra)`.

---

## Operations (summary)

| Operation | Input | Output | Consumes view? |
|-----------|--------|--------|----------------|
| `from_pattern_graph` | classifier, PatternGraph | GraphView | N/A |
| `from_graph_lens` | classifier, GraphLens | GraphView | N/A (deferred) |
| `materialize` | classifier, policy, view | PatternGraph | Yes |
| `unfold` | expand fn, seed | Pattern<V> | N/A |
| `unfold_graph` | classifier, policy, expand fn, seeds | PatternGraph | N/A |
| `map_graph` | classifier, CategoryMappers, view | GraphView | Yes |
| `map_all_graph` | f, view | GraphView | Yes |
| `filter_graph` | classifier, predicate, substitution, view | GraphView | Yes |
| `fold_graph` | f, init, view | M (accumulator) | No (&view) |
| `map_with_context` | classifier, f, view | GraphView | Yes |
| `para_graph` | f, view | HashMap<V::Id, R> | No (&view) |
| `para_graph_fixed` | converged, f, init, view | HashMap<V::Id, R> | No (&view) |

---

## Module Layout

| Concept | Module path |
|---------|-------------|
| GraphView, from_pattern_graph, from_graph_lens, materialize | `graph::graph_view` |
| Substitution, CategoryMappers | `graph::transform` (or `graph::transform::types`) |
| map_graph, map_all_graph, filter_graph, fold_graph | `graph::transform` |
| map_with_context | `graph::transform` |
| para_graph, para_graph_fixed | `graph::transform` |
| unfold (single Pattern tree) | `pattern::unfold` |
| unfold_graph | `graph::transform` |

---

## Trait Bounds (recurring)

- `V: GraphValue` (from 030) for classification and identity.
- `V::Id: Eq + Hash` where results are keyed by identity (e.g. para_graph result map).
- For `materialize` and `unfold_graph`: `V` (and MergeStrategy) must satisfy the same bounds as `PatternGraph` construction (HasIdentity, Mergeable, Refinable, etc.) as provided by the existing `reconcile` and `pattern_graph` modules.
