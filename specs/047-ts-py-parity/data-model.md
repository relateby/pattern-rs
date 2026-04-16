# Data Model: TypeScript and Python Pattern API Parity

**Phase**: 1 — Design
**Feature**: 047-ts-py-parity
**Date**: 2026-04-16

---

## Overview

This feature adds new operations to existing data structures. No new entities or storage is introduced. The entities described here are the existing `Pattern<V>` and supporting types whose interfaces are being extended.

---

## Pattern\<V\>

The core recursive tree structure. Each node carries a value of type `V` and an ordered list of child `Pattern<V>` nodes.

**Invariants:**
- A `point` (leaf) has an empty elements list.
- A non-atomic pattern has one or more elements.
- All operations preserve the value/elements structure.

### New constructors

| Constructor | Inputs | Output | Invariant |
|---|---|---|---|
| `Pattern.pattern(v, elements)` | value `V`, children `Pattern<V>[]` | `Pattern<V>` | `result.value == v`, `result.elements == children` |
| `Pattern.fromList(v, values)` | value `V`, values `V[]` | `Pattern<V>` | Each `values[i]` becomes `Pattern.point(values[i])` as child `i` |

### New predicate operations

| Operation | Inputs | Output | Semantics |
|---|---|---|---|
| `anyValue(pred)` | predicate `V -> bool` | `bool` | Short-circuit `true` on first value satisfying `pred`, pre-order |
| `allValues(pred)` | predicate `V -> bool` | `bool` | Short-circuit `false` on first value failing `pred`, pre-order |
| `matches(a, b)` | two `Pattern<V>` | `bool` | Structural equality: same depth, same element count, equal values at all positions |
| `contains(haystack, needle)` | two `Pattern<V>` | `bool` | `true` if `needle` matches any subpattern of `haystack` (including root) |

### New transformation operations

| Operation | Inputs | Output | Semantics |
|---|---|---|---|
| `para(f)` | fold fn `(Pattern<V>, R[]) -> R` | `R` | Bottom-up fold; `f` receives current sub-pattern + pre-computed child results |
| `unfold(expand, seed)` | expand fn `A -> (V, A[])`, seed `A` | `Pattern<V>` | Expands seed recursively; terminates when expand returns empty children |
| `combine(combineV)(a, b)` | combine fn `(V,V)->V`, two patterns | `Pattern<V>` | `value = combineV(a.value, b.value)`, `elements = a.elements ++ b.elements` |

### New comonad helpers

| Operation | Inputs | Output | Semantics |
|---|---|---|---|
| `depthAt(p)` | `Pattern<V>` | `Pattern<number>` | Each node's value = depth of its subtree (leaf = 0) |
| `sizeAt(p)` | `Pattern<V>` | `Pattern<number>` | Each node's value = total count of nodes in its subtree |
| `indicesAt(p)` | `Pattern<V>` | `Pattern<number[]>` | Each node's value = list of 0-based indices forming path from root to that node |

---

## Para Result R

The paramorphism fold type `R` is fully polymorphic — determined by the caller's fold function. No constraints on `R` other than it being a consistent type within a single `para` call.

**Key property**: Unlike `fold` (which only passes values), `para` passes the full `Pattern<V>` at each position. This means `f` can inspect `p.length`, `p.depth`, `p.size`, `p.value`, and `p.elements` when computing the result.

---

## Combinable Value Contract

`combine` requires the caller to supply a value-combination function of type `(a: V, b: V) => V`. This function SHOULD satisfy:

- **Associativity**: `f(f(a, b), c) == f(a, f(b, c))`

The library does not enforce associativity at runtime, but callers are expected to provide associative combination functions to match the Semigroup semantics of the Haskell reference.

**Subject combination** (most common use case): combine labels via union, combine properties via left-bias merge, keep first non-empty identity.

---

## GraphView (Python — new)

The Python graph transform functions operate on a conceptual `GraphView` — a snapshot of classified graph elements. In practice, this is represented by the Python `StandardGraph`'s internal data. The transforms produce a new `StandardGraph` or classified element list.

**Classified element pair**: `(GraphClass, Pattern[Subject])` where `GraphClass` ∈ `{GNode, GRelationship, GAnnotation, GWalk, GOther}`.

### New Python graph transform operations

| Operation | Inputs | Output | Semantics |
|---|---|---|---|
| `map_graph(graph, mappers)` | `StandardGraph`, per-class mappers | `StandardGraph` | Transform each element according to its class-specific mapper |
| `map_all_graph(graph, f)` | `StandardGraph`, `Pattern[Subject] -> Pattern[Subject]` | `StandardGraph` | Apply same transform to all elements regardless of class |
| `filter_graph(graph, pred, subst)` | `StandardGraph`, predicate, substitution | `StandardGraph` | Remove non-matching elements; handle affected containers per substitution |
| `fold_graph(graph, f, empty, combine)` | `StandardGraph`, fold fn, identity, combine fn | `R` | Reduce all elements to single value using monoid-like API |
| `map_with_context(graph, f)` | `StandardGraph`, `(GraphQuery, Pattern[Subject]) -> Pattern[Subject]` | `StandardGraph` | Transform with snapshot graph query; snapshot is frozen at transform start |
| `para_graph(graph, f)` | `StandardGraph`, `(GraphQuery, Pattern[Subject], {id: R}) -> R` | `dict[str, R]` | Bottom-up fold in topological order; passes pre-computed child results as dict |

### Substitution strategies (for `filter_graph`)

| Strategy | Semantics |
|---|---|
| `DeleteContainer` | Remove entire container when any of its elements is removed |
| `SpliceGap` | Remove the element, close the gap (container keeps remaining elements) |
| `ReplaceWithSurrogate(pattern)` | Replace removed element with surrogate pattern |

These match the TypeScript `Substitution` ADT exactly.

---

## Validation Rules

- `unfold` has no depth limit; callers must provide terminating expand functions.
- `combine` has no value-type constraint enforced at runtime; callers supply the combiner.
- `para` has no result-type constraint; `R` is inferred from the fold function.
- `indicesAt` paths are 0-based and ordered by position in the elements list.
