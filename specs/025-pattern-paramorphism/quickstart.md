# Quick Start: Pattern Paramorphism

**Feature**: 025-pattern-paramorphism  
**Date**: 2026-01-30

## Overview

This guide shows how to use the `para` method on `Pattern<V>` for structure-aware folding. Unlike `fold`, which only sees values, `para` gives your function access to the current pattern and the recursively computed results from its elements—so you can analyze patterns of elements (e.g. detect A, B, A sequences), depth-weighted sums, and element-count-aware aggregations.

---

## Basic Usage

### Sum All Values (same as fold)

```rust
use pattern_core::Pattern;

let p = Pattern::pattern(10, vec![
    Pattern::point(5),
    Pattern::point(3),
]);

let sum: i32 = p.para(|pat, rs| *pat.value() + rs.iter().sum::<i32>());
assert_eq!(sum, 18);  // 10 + 5 + 3
```

### Depth-Weighted Sum

```rust
use pattern_core::Pattern;

let p = Pattern::pattern(10, vec![
    Pattern::point(5),
    Pattern::point(3),
]);

// Value at depth d is weighted by d; atomics are depth 0
let depth_weighted: i32 = p.para(|pat, rs| {
    *pat.value() * pat.depth() as i32 + rs.iter().sum::<i32>()
});
// Root (depth 1): 10*1 + (0+0) = 10
// Elements (depth 0): 5*0+0=0, 3*0+0=0
assert_eq!(depth_weighted, 10);
```

### Atomic Pattern (No Elements)

```rust
use pattern_core::Pattern;

let atomic = Pattern::point(42);
let result: i32 = atomic.para(|pat, rs| {
    assert!(rs.is_empty());  // No elements => empty slice
    *pat.value()
});
assert_eq!(result, 42);
```

---

## Pattern-of-Elements Analysis

### Collect Values in Order (simulate toList)

```rust
use pattern_core::Pattern;

let p = Pattern::pattern(1, vec![
    Pattern::point(2),
    Pattern::point(3),
]);

let values: Vec<i32> = p.para(|pat, rs| {
    let mut v = vec![*pat.value()];
    for r in rs {
        v.extend(r.clone());
    }
    v
});
assert_eq!(values, vec![1, 2, 3]);  // Pre-order, element order preserved
```

### Element-Count-Aware Aggregation

```rust
use pattern_core::Pattern;

let p = Pattern::pattern(10, vec![
    Pattern::point(5),
    Pattern::point(3),
]);

// Value * number of elements at this node + sum of element results
let result: i32 = p.para(|pat, rs| {
    *pat.value() * pat.elements().len() as i32 + rs.iter().sum::<i32>()
});
// Root: 10*2 + (5+3) = 28; elements: 5*0+0=5, 3*0+0=3
assert_eq!(result, 28);
```

---

## When to Use para vs fold vs extend

| Need | Use | Example |
|------|-----|--------|
| Reduce to one value, no structure | `fold` | Sum, count, concatenate |
| Structure-aware aggregation | `para` | Depth-weighted sum, pattern-of-elements, nesting stats |
| Structure-aware transformation (new Pattern) | `extend` (Comonad) | Replace each value with a function of its context |

---

## Next Steps

- See [contracts/type-signatures.md](contracts/type-signatures.md) for the full API contract.
- See [data-model.md](data-model.md) for the data model.
- Reference: `../pattern-hs/docs/reference/features/paramorphism.md`.
