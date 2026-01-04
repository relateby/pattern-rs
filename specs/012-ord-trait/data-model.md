# Data Model: Pattern Ordering and Comparison

**Feature**: 012-ord-trait  
**Date**: 2025-01-04  
**Research**: [research.md](./research.md)

## Overview

This document defines the ordering relationships and comparison semantics for Pattern<V> types. The ordering enables patterns to be sorted, compared, and used in ordered data structures while maintaining behavioral equivalence with the gram-hs reference implementation.

## Core Entity

### Pattern<V>

**Type**: Generic recursive structure  
**Definition** (existing in pattern-core):
```rust
pub struct Pattern<V> {
    pub value: V,
    pub elements: Vec<Pattern<V>>,
}
```

**Ordering Constraint**: For Pattern<V> to be ordered, the value type V must implement:
- `PartialOrd` for partial ordering (Pattern implements PartialOrd)
- `Ord` for total ordering (Pattern implements Ord)

**Existing Traits** (prerequisites):
- ✅ `Clone` - Patterns can be cloned
- ✅ `PartialEq` - Patterns can be compared for equality
- ✅ `Eq` - Patterns have reflexive equality

**New Traits** (this feature):
- 🆕 `PartialOrd` - Patterns can be partially ordered (when V: PartialOrd)
- 🆕 `Ord` - Patterns have total ordering (when V: Ord)

## Ordering Semantics

### Comparison Algorithm

**Lexicographic Comparison** (value-first):

```
compare(Pattern<V>(v1, es1), Pattern<V>(v2, es2)):
    1. value_order = compare(v1, v2)
    2. if value_order == Equal:
        return compare(es1, es2)  // lexicographic element comparison
    3. else:
        return value_order
```

**Properties**:
- **Value precedence**: Values are compared before structure
- **Recursive**: Element vectors compared recursively
- **Lexicographic**: Elements compared left-to-right, stops at first difference
- **Length-sensitive**: If all compared elements equal, shorter list < longer list

### Ordering Result

**std::cmp::Ordering** (standard Rust enum):
```rust
pub enum Ordering {
    Less,    // First pattern is less than second
    Equal,   // Patterns are equal
    Greater  // First pattern is greater than second
}
```

**Interpretation**:
- `p1.cmp(&p2) == Ordering::Less` means `p1 < p2`
- `p1.cmp(&p2) == Ordering::Equal` means `p1 == p2`
- `p1.cmp(&p2) == Ordering::Greater` means `p1 > p2`

## Ordering Examples

### Example 1: Atomic Patterns (Different Values)

```rust
let p1 = Pattern::point(1);  // Pattern { value: 1, elements: [] }
let p2 = Pattern::point(2);  // Pattern { value: 2, elements: [] }
```

**Comparison**:
- Compare values: `1 < 2` → `Less`
- Elements not compared (values differ)
- **Result**: `p1 < p2` ✓

### Example 2: Nested Patterns (Same Value, Different Elements)

```rust
let p1 = Pattern::pattern(5, vec![Pattern::point(1)]);
// Pattern { value: 5, elements: [Pattern { value: 1, elements: [] }] }

let p2 = Pattern::pattern(5, vec![Pattern::point(2)]);
// Pattern { value: 5, elements: [Pattern { value: 2, elements: [] }] }
```

**Comparison**:
- Compare values: `5 == 5` → `Equal`
- Compare elements: `[point(1)] vs [point(2)]`
  - Compare first elements: `point(1) < point(2)` → `Less`
- **Result**: `p1 < p2` ✓

### Example 3: Different Lengths (Prefix Match)

```rust
let p1 = Pattern::pattern(5, vec![Pattern::point(1)]);
// Pattern { value: 5, elements: [point(1)] }

let p2 = Pattern::pattern(5, vec![Pattern::point(1), Pattern::point(2)]);
// Pattern { value: 5, elements: [point(1), point(2)] }
```

**Comparison**:
- Compare values: `5 == 5` → `Equal`
- Compare elements: `[point(1)] vs [point(1), point(2)]`
  - First elements equal: `point(1) == point(1)`
  - Length comparison: `1 < 2` (shorter < longer when prefix matches)
- **Result**: `p1 < p2` ✓

### Example 4: Deep Nesting

```rust
let p1 = Pattern::pattern(
    "root",
    vec![Pattern::pattern(
        "middle",
        vec![Pattern::point("a")]
    )]
);

let p2 = Pattern::pattern(
    "root",
    vec![Pattern::pattern(
        "middle",
        vec![Pattern::point("b")]
    )]
);
```

**Comparison**:
- Compare values: `"root" == "root"` → `Equal`
- Compare elements: `[pattern("middle", [point("a")])] vs [pattern("middle", [point("b")])]`
  - Compare first element patterns:
    - Values: `"middle" == "middle"` → `Equal`
    - Elements: `[point("a")] vs [point("b")]`
      - Compare: `point("a") < point("b")` → `Less`
- **Result**: `p1 < p2` ✓

## Ordering Properties (Ord Laws)

### 1. Reflexivity

**Property**: `x.cmp(&x) == Equal` for all x

**Example**:
```rust
let p = Pattern::pattern(1, vec![Pattern::point(2)]);
assert_eq!(p.cmp(&p), Ordering::Equal);
```

### 2. Antisymmetry

**Property**: If `x < y` then `!(y < x)`

**Example**:
```rust
let p1 = Pattern::point(1);
let p2 = Pattern::point(2);
assert!(p1 < p2);
assert!(!(p2 < p1));
assert!(p2 > p1);
```

### 3. Transitivity

**Property**: If `x < y` and `y < z` then `x < z`

**Example**:
```rust
let p1 = Pattern::point(1);
let p2 = Pattern::point(2);
let p3 = Pattern::point(3);
assert!(p1 < p2);
assert!(p2 < p3);
assert!(p1 < p3);  // transitivity
```

### 4. Totality

**Property**: For all x, y, exactly one holds: `x < y`, `x == y`, or `x > y`

**Example**:
```rust
let p1 = Pattern::point(1);
let p2 = Pattern::point(2);
// Exactly one is true:
assert!(p1 < p2);       // true
assert!(!(p1 == p2));   // false
assert!(!(p1 > p2));    // false
```

### 5. Consistency with Eq

**Property**: If `x == y` then `x.cmp(&y) == Equal`

**Example**:
```rust
let p1 = Pattern::pattern(1, vec![Pattern::point(2)]);
let p2 = Pattern::pattern(1, vec![Pattern::point(2)]);
assert!(p1 == p2);
assert_eq!(p1.cmp(&p2), Ordering::Equal);
```

## Relationship to gram-hs

### Haskell Implementation

```haskell
instance Ord v => Ord (Pattern v) where
  compare (Pattern v1 es1) (Pattern v2 es2) =
    case compare v1 v2 of
      EQ -> compare es1 es2
      other -> other
```

### Rust Implementation

```rust
impl<V: Ord> Ord for Pattern<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value.cmp(&other.value) {
            Ordering::Equal => self.elements.cmp(&other.elements),
            non_equal => non_equal
        }
    }
}
```

### Semantic Equivalence

| Aspect | Haskell | Rust | Equivalent? |
|--------|---------|------|-------------|
| **Algorithm** | Value first, then elements | Value first, then elements | ✅ Yes |
| **Element comparison** | List Ord (lexicographic) | Vec Ord (lexicographic) | ✅ Yes |
| **Recursion** | Automatic via list Ord | Automatic via Vec Ord | ✅ Yes |
| **Short-circuit** | Yes (case statement) | Yes (match expression) | ✅ Yes |
| **Type constraint** | `Ord v =>` | `V: Ord` | ✅ Yes |

## Use Cases

### 1. Sorting Pattern Collections

```rust
let mut patterns = vec![
    Pattern::point(3),
    Pattern::point(1),
    Pattern::point(2),
];
patterns.sort();  // Uses Ord implementation
// Result: [point(1), point(2), point(3)]
```

### 2. Finding Min/Max

```rust
let patterns = vec![
    Pattern::point(3),
    Pattern::point(1),
    Pattern::point(2),
];
let min_pattern = patterns.iter().min().unwrap();  // point(1)
let max_pattern = patterns.iter().max().unwrap();  // point(3)
```

### 3. Ordered Data Structures

```rust
use std::collections::{BTreeMap, BTreeSet};

// BTreeSet automatically maintains ordering
let mut set = BTreeSet::new();
set.insert(Pattern::point(3));
set.insert(Pattern::point(1));
set.insert(Pattern::point(2));
// Iteration yields: point(1), point(2), point(3)

// BTreeMap with pattern keys
let mut map = BTreeMap::new();
map.insert(Pattern::point(1), "first");
map.insert(Pattern::point(2), "second");
// Keys are ordered: point(1) before point(2)
```

### 4. Binary Search

```rust
let mut patterns = vec![/* ... */];
patterns.sort();
let target = Pattern::point(42);
let index = patterns.binary_search(&target);  // Uses Ord for efficient search
```

## Performance Characteristics

### Time Complexity

| Operation | Best Case | Average Case | Worst Case | Notes |
|-----------|-----------|--------------|------------|-------|
| **compare()** | O(1) | O(log n) | O(n) | n = total nodes |
| **Value differs** | O(1) | O(1) | O(1) | Short-circuit |
| **Equal values** | O(min(k1, k2)) | O(min(k1, k2)) | O(min(k1, k2)) | k = element count |
| **Deep patterns** | O(1) | O(d) | O(d) | d = depth |

### Space Complexity

- **Stack space**: O(d) where d is maximum pattern depth
- **Heap allocation**: O(0) - comparison uses references only
- **No temporary allocations** during comparison

### Performance Targets

- ✅ Sort 10,000 patterns: <200ms (O(N log N) × comparison cost)
- ✅ Compare deep (200+ levels): <100ms, no stack overflow
- ✅ Compare wide (5,000+ elements): <500ms

## Edge Cases

### Empty Elements

```rust
let p1 = Pattern::point(1);  // elements: []
let p2 = Pattern::point(1);  // elements: []
assert_eq!(p1.cmp(&p2), Ordering::Equal);  // Values equal, empty vectors equal
```

### Deeply Nested (200+ levels)

```rust
// Create deep pattern recursively
fn deep_pattern(depth: usize) -> Pattern<i32> {
    if depth == 0 {
        Pattern::point(0)
    } else {
        Pattern::pattern(0, vec![deep_pattern(depth - 1)])
    }
}

let p1 = deep_pattern(250);
let p2 = deep_pattern(250);
assert_eq!(p1.cmp(&p2), Ordering::Equal);  // Should not stack overflow
```

### Wide Patterns (5,000+ elements)

```rust
let elements: Vec<Pattern<i32>> = (0..5000).map(Pattern::point).collect();
let p1 = Pattern::pattern(0, elements.clone());
let p2 = Pattern::pattern(0, elements);
assert_eq!(p1.cmp(&p2), Ordering::Equal);  // Should complete in <500ms
```

## Validation Rules

### Compile-Time Validation

Rust's type system enforces:
1. **V: PartialOrd**: Required for Pattern<V> to implement PartialOrd
2. **V: Ord**: Required for Pattern<V> to implement Ord
3. **Trait hierarchy**: Ord requires Eq + PartialOrd (already satisfied)

### Runtime Validation

No runtime validation needed:
- Comparison is pure (no side effects)
- No allocation or resource usage
- No failure modes (comparison always succeeds for Ord types)

## Summary

The ordering implementation provides:
- ✅ Simple value-first lexicographic comparison
- ✅ Behavioral equivalence with gram-hs Ord instance
- ✅ Automatic recursive comparison via Rust's Vec Ord
- ✅ Efficient short-circuit evaluation
- ✅ Satisfies all Ord laws (reflexivity, antisymmetry, transitivity, totality, consistency)
- ✅ Enables ordered collections (BTreeMap, BTreeSet, sorting, min/max)
- ✅ No allocations during comparison
- ✅ Handles deep nesting and wide patterns efficiently

