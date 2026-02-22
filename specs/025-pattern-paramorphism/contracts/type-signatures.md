# Type Signatures: Paramorphism for Pattern

**Feature**: 025-pattern-paramorphism  
**Date**: 2026-01-30  
**Status**: Draft

## Overview

This document defines the API contract for the paramorphism operation on `Pattern<V>`. Paramorphism enables structure-aware folding: the folding function receives both the current pattern and recursively computed results from its elements.

---

## Public API

### para

Folds the pattern into a single value using a structure-aware folding function. The function receives the current pattern and a slice of results from its elements (in element order).

**Signature**:

```rust
impl<V> Pattern<V> {
    pub fn para<R, F>(&self, f: F) -> R
    where
        F: Fn(&Pattern<V>, &[R]) -> R;
}
```

**Type Parameters**:
- `V`: Value type stored in the pattern
- `R`: Result type produced by the folding function (can differ from `V`)
- `F`: Folding function type `Fn(&Pattern<V>, &[R]) -> R`

**Parameters**:
- `self: &Pattern<V>` – Pattern to fold (borrowed, not consumed)
- `f: F` – Folding function with signature `Fn(&Pattern<V>, &[R]) -> R`
  - First parameter: Reference to the current pattern subtree
  - Second parameter: Slice of results from the pattern's elements (left-to-right order)
  - Returns: Result for this node (type `R`)

**Returns**: `R` – The result produced by applying the folding function at the root after all elements have been processed recursively.

**Semantics**:
- **Evaluation order**: Bottom-up. For each node, para is first applied to all elements (left to right); then the folding function is called with the current pattern and the slice of those results.
- **Atomic patterns**: If the pattern has no elements, the folding function receives an empty slice `&[]` for the second argument.
- **Element order**: The slice of results is in the same order as `self.elements()`.
- **Non-destructive**: The pattern is not modified. The pattern can be reused after the call.
- **Single traversal**: Each node is visited exactly once.

**Time Complexity**: O(n) where n = total number of nodes in the pattern  
**Space Complexity**: O(n) for collecting element results (plus O(d) stack where d = max depth)

**Examples**:

```rust
use pattern_core::Pattern;

// Sum all values (equivalent to fold with +)
let p = Pattern::pattern(10, vec![Pattern::point(5), Pattern::point(3)]);
let sum: i32 = p.para(|pat, rs| *pat.value() + rs.iter().sum::<i32>());
assert_eq!(sum, 18);  // 10 + 5 + 3

// Depth-weighted sum
let depth_weighted: i32 = p.para(|pat, rs| *pat.value() * pat.depth() as i32 + rs.iter().sum::<i32>());
// Root: 10*1 + (0+0) = 10; elements are atomic (depth 0) so 5*0+0=0, 3*0+0=0

// Nesting statistics (sum, count, max_depth)
let (s, c, d): (i32, usize, usize) = p.para(|pat, rs| {
    let (child_s, child_c, child_d) = rs.iter()
        .fold((0_i32, 0_usize, 0_usize), |(s, c, d), (s2, c2, d2)| (s + s2, c + c2, d.max(*d2)));
    (*pat.value() + child_s, 1 + child_c, pat.depth().max(child_d))
});
```

**Behavioral Contract**:
1. The folding function is called exactly once per node.
2. Call order: for any node, all its elements are processed (recursively) before the node itself.
3. For atomic patterns, the second argument is an empty slice.
4. The slice length equals the number of elements at that node.
5. Result type `R` is determined by the caller; common choices include numeric types, `Vec<V>`, or tuples.

---

## Relationship to Other Operations

| Operation | Type | Access | Use when |
|----------|------|--------|----------|
| **fold** | `Fn(B, &V) -> B` | Values only | Reducing to a single value without structure (e.g. sum, count) |
| **para** | `Fn(&Pattern<V>, &[R]) -> R` | Pattern + element results | Structure-aware aggregation (e.g. depth-weighted sum, pattern-of-elements analysis) |
| **extend** (Comonad) | `Fn(&Pattern<V>) -> V` | Pattern for transformation | Structure-aware transformation that returns a new Pattern |

**Equivalence property** (for testing): For any pattern and numeric value type,  
`pattern.para(|p, rs| p.value() + rs.iter().sum())` must equal  
`pattern.fold(0, \|acc, v\| acc + v)`.

---

## Reference

- gram-hs: `../pattern-hs/libs/pattern/src/Pattern/Core.hs` (lines 1188–1190)
- gram-hs docs: `../pattern-hs/docs/reference/features/paramorphism.md`
- Porting guide: `../pattern-hs/docs/reference/PORTING-GUIDE.md` (Paramorphism Implementation)
