# Data Model: Pattern Paramorphism

**Feature**: 025-pattern-paramorphism  
**Date**: 2026-01-30

## Overview

This document describes the data model for paramorphism on `Pattern<V>`. Paramorphism does not introduce new persistent entities; it adds a method that folds over the existing pattern structure with access to both the current pattern and recursively computed results from its elements.

## Core Types

### Pattern<V> (Existing)

The pattern data structure that paramorphism folds over.

```rust
pub struct Pattern<V> {
    pub value: V,
    pub elements: Vec<Pattern<V>>,
}
```

**Properties**:
- **value**: The value at this pattern node (type `V`)
- **elements**: The pattern's elements (zero or more `Pattern<V>`); order is significant (left-to-right)

**Role in Paramorphism**:
- Each node is visited exactly once in bottom-up order (elements processed before the current node)
- The folding function receives a reference to the current pattern, so it can read `value`, `elements`, `depth()`, `length()`, etc.
- Element order is preserved when building the slice of results passed to the folding function

**Validation rules** (from existing Pattern): No new validation; para operates on any valid Pattern. Atomic patterns (empty elements) are the base case.

---

### Result Type R (Generic)

The type of the aggregated result produced by the folding function.

**Type Parameter**: `R` (generic, chosen by the caller)

**Characteristics**:
- Independent of value type `V` (can be the same or different)
- Produced at each node by the folding function from the current pattern and the slice of element results
- Final result at the root is the para result

**Examples**:
- `i32` – depth-weighted sum, or sum of values (simulating fold)
- `Vec<V>` – pre-order value list (simulating toList)
- `(i64, usize, usize)` – (sum, count, max_depth) for nesting statistics
- `Pattern<V>` – structure-preserving transformation (e.g. depth-weighted values)

---

### Folding Function

The function that combines the current pattern with results from its elements to produce a result.

**Type**: `F where F: Fn(&Pattern<V>, &[R]) -> R`

**Parameters**:
- First: `&Pattern<V>` – reference to the current pattern subtree (read-only)
- Second: `&[R]` – slice of results from the pattern's elements, in element order (left-to-right)

**Returns**: `R` – the result for this node, which may be combined by the parent's folding function.

**Invariants**:
- Called exactly once per node in the pattern
- For atomic patterns (no elements), the second argument is an empty slice
- Element results are in the same order as `pattern.elements()`

---

## State and Lifecycle

Paramorphism is stateless: it does not mutate the pattern or maintain internal state. Each call to `para` performs a full traversal and returns a single result. The pattern can be used again after the call.

---

## Relationships

- **Pattern<V>** – has many **elements** (also `Pattern<V>`); para recurses over elements then combines at the current node
- **Folding function** – consumes `&Pattern<V>` and `&[R]`, produces `R`; no other entities
- **Result type R** – produced by the folding function; no persistence

No new entities or tables; para is a pure function over the existing pattern structure.
