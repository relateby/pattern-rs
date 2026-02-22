# Data Model: Predicate-Based Pattern Matching

**Date**: 2025-01-05  
**Phase**: 1 - Design & Contracts

## Overview

This document defines the types, function signatures, and data structures used in the predicate-based pattern matching feature for pattern-rs. The feature extends the existing `Pattern<V>` type with three new query methods that enable finding patterns by predicate, checking structural equality, and testing subpattern containment.

## Existing Types (No Changes)

### Pattern<V>

```rust
pub struct Pattern<V> {
    pub value: V,
    pub elements: Vec<Pattern<V>>,
}
```

**Status**: Already defined in `crates/pattern-core/src/pattern.rs`  
**Changes**: None (adding methods only)

**Existing Predicate Methods**:
- `any_value<F>(&self, F) -> bool where F: Fn(&V) -> bool` - checks if any value matches
- `all_values<F>(&self, F) -> bool where F: Fn(&V) -> bool` - checks if all values match
- `filter<F>(&self, F) -> Vec<&Pattern<V>> where F: Fn(&Pattern<V>) -> bool` - collects all matching patterns

## New Methods

### find_first

**Purpose**: Find the first subpattern that matches a predicate.

**Signature**:
```rust
pub fn find_first<F>(&self, predicate: F) -> Option<&Pattern<V>>
where
    F: Fn(&Pattern<V>) -> bool,
```

**Type Parameters**:
- `F` - Pattern predicate function type (must be reusable)

**Parameters**:
- `predicate` - Function that takes pattern reference and returns boolean

**Returns**:
- `Some(&Pattern<V>)` - First matching subpattern (including root if it matches)
- `None` - No matching subpattern found

**Traversal Order**: Depth-first pre-order (root first, then elements left-to-right)

**Characteristics**:
- Borrows from self (lifetime-bound to self)
- Short-circuits on first match
- Includes root pattern in search scope
- Predicate can be called multiple times (Fn, not FnOnce)
- Time complexity: O(k) where k is position of first match (best O(1), worst O(n))
- Space complexity: O(d) for recursion stack where d is depth

**Example**:
```rust
let pattern = Pattern::pattern(
    "root",
    vec![
        Pattern::point("leaf1"),
        Pattern::pattern("branch", vec![Pattern::point("leaf2")]),
    ],
);

// Find first atomic pattern
let first_leaf = pattern.find_first(|p| p.is_atomic());
assert_eq!(first_leaf, Some(&Pattern::point("leaf1")));

// Find pattern with specific value
let branch = pattern.find_first(|p| p.value == "branch");
assert!(branch.is_some());

// No match returns None
let no_match = pattern.find_first(|p| p.value == "nonexistent");
assert_eq!(no_match, None);
```

### matches

**Purpose**: Check if two patterns have identical structure (same values and element arrangement recursively).

**Signature**:
```rust
pub fn matches(&self, other: &Pattern<V>) -> bool
where
    V: PartialEq,
```

**Type Constraints**:
- `V: PartialEq` - Values must be comparable for equality

**Parameters**:
- `other` - Pattern to compare against

**Returns**:
- `true` - Patterns have identical structure (same value, same number of elements, elements match recursively)
- `false` - Patterns differ in structure

**Characteristics**:
- Structural comparison (not just value equality)
- Short-circuits on first mismatch
- Reflexive: `p.matches(&p)` is always true
- Symmetric: `p.matches(&q) == q.matches(&p)`
- Distinguishes patterns with same flattened values but different structures
- Time complexity: O(min(n, m)) where n, m are pattern sizes
- Space complexity: O(min(d1, d2)) for recursion stack

**Relationship to Eq**:
- `matches` is distinct from `Eq` trait
- For now: `p1.matches(&p2) == (p1 == p2)` when both defined
- Future: matches may support wildcards, partial matching (different semantics than Eq)

**Example**:
```rust
let p1 = Pattern::pattern("root", vec![Pattern::point("a"), Pattern::point("b")]);
let p2 = Pattern::pattern("root", vec![Pattern::point("a"), Pattern::point("b")]);
let p3 = Pattern::pattern("root", vec![Pattern::point("a")]);

// Identical structure matches
assert!(p1.matches(&p2));

// Self-matching
assert!(p1.matches(&p1));

// Different structure doesn't match
assert!(!p1.matches(&p3));

// Same values, different structure
let p4 = Pattern::pattern("root", vec![Pattern::point("a")]);
let p5 = Pattern::pattern("root", vec![Pattern::pattern("a", vec![])]);
assert!(!p4.matches(&p5)); // "a" point vs "a" pattern with empty elements
```

### contains

**Purpose**: Check if a pattern contains another pattern as a subpattern anywhere in its structure.

**Signature**:
```rust
pub fn contains(&self, subpattern: &Pattern<V>) -> bool
where
    V: PartialEq,
```

**Type Constraints**:
- `V: PartialEq` - Values must be comparable (uses matches internally)

**Parameters**:
- `subpattern` - Pattern to search for

**Returns**:
- `true` - Subpattern found (either as self or in elements recursively)
- `false` - Subpattern not found

**Characteristics**:
- Uses `matches` for structural comparison
- Short-circuits on first match
- Reflexive: `p.contains(&p)` is always true
- Transitive: if `a.contains(&b)` and `b.contains(&c)` then `a.contains(&c)`
- Time complexity: O(n*m) worst case where n = container size, m = subpattern size
- Space complexity: O(d) for recursion stack

**Relationship to matches**:
- `p.matches(&q)` implies `p.contains(&q)`
- `p.contains(&q)` does not imply `p.matches(&q)` (containment is weaker than equality)

**Example**:
```rust
let pattern = Pattern::pattern(
    "root",
    vec![
        Pattern::point("a"),
        Pattern::pattern("b", vec![Pattern::point("c")]),
    ],
);

let subpat1 = Pattern::point("a");
let subpat2 = Pattern::pattern("b", vec![Pattern::point("c")]);
let subpat3 = Pattern::point("x");

// Contains atomic subpattern
assert!(pattern.contains(&subpat1));

// Contains nested subpattern
assert!(pattern.contains(&subpat2));

// Doesn't contain non-existent subpattern
assert!(!pattern.contains(&subpat3));

// Self-containment
assert!(pattern.contains(&pattern));
```

## Predicate Function Types

### Pattern Predicate

**Concept**: A reusable function that takes a pattern reference and returns a boolean.

**Rust Representation**:
```rust
F: Fn(&Pattern<V>) -> bool
```

**Characteristics**:
- Generic type parameter (compile-time polymorphism)
- Takes immutable reference to pattern (no copying, access to full structure)
- Callable multiple times (Fn, not FnMut or FnOnce)
- Pure function semantics (should not mutate state)
- Used by: `find_first`, `filter`

**Examples**:
```rust
// Check if pattern is atomic
|p: &Pattern<String>| p.is_atomic()

// Check value property
|p: &Pattern<i32>| p.value > 10

// Check structural property
|p: &Pattern<String>| p.length() > 2

// Combined predicate
|p: &Pattern<i32>| p.value > 0 && p.depth() < 3

// Using matches
|p: &Pattern<String>| p.matches(&target_pattern)

// Using contains
|p: &Pattern<String>| p.contains(&subpattern)
```

### Value Predicate

**Note**: Already defined (used by any_value, all_values). Included for completeness.

**Rust Representation**:
```rust
F: Fn(&V) -> bool
```

**Characteristics**:
- Takes immutable reference to value only (not full pattern)
- Used by: `any_value`, `all_values`

## Return Types

### Boolean Result

**Type**: `bool`  
**Used by**: `any_value`, `all_values`, `matches`, `contains`  
**Semantics**:
- `true` = condition satisfied (at least one match, all match, structures match, contains subpattern)
- `false` = condition not satisfied

### Optional Pattern Reference

**Type**: `Option<&Pattern<V>>`  
**Used by**: `find_first`  
**Semantics**:
- `Some(&Pattern<V>)` = matching pattern found (borrowed reference)
- `None` = no matching pattern found (absence, not error)

**Lifetime**: Borrows from source pattern, same lifetime as self

### Vector of Pattern References

**Type**: `Vec<&Pattern<V>>`  
**Used by**: `filter` (existing)  
**Semantics**:
- Non-empty vec = matching patterns found
- Empty vec = no matching patterns found

## Traversal Semantics

All predicate functions use depth-first pre-order traversal:

**Order**:
1. Check/process root pattern first
2. Then process elements left-to-right
3. For each element, recursively apply same order

**Example Traversal Order**:
```rust
Pattern::pattern("A", vec![
    Pattern::point("B"),
    Pattern::pattern("C", vec![
        Pattern::point("D"),
    ]),
    Pattern::point("E"),
])

// Traversal order: A, B, C, D, E
```

**Consistency**: All operations use same traversal order:
- `any_value`, `all_values`, `filter`, `find_first` (new)
- `fold`, `map`, `values` (existing)

## Error Handling

**No Error Types**: All functions use `bool` or `Option` return types. No match is not an error, it's an expected outcome.

**Panics**: None. All functions handle edge cases gracefully:
- Atomic patterns (no elements)
- Empty elements (empty Vec)
- Deep nesting (up to 100+ levels)
- No matches (returns false or None)

## Performance Characteristics

### Time Complexity Summary

| Function | Best Case | Average Case | Worst Case |
|----------|-----------|--------------|------------|
| find_first | O(1) | O(k) where k=match position | O(n) |
| matches | O(1) | O(min(n,m)/2) | O(min(n,m)) |
| contains | O(1) | O(n*m/2) | O(n*m) |

Where:
- n = number of nodes in container pattern
- m = number of nodes in subpattern or comparison pattern
- k = position of first match (1 ≤ k ≤ n)

### Space Complexity Summary

All functions use O(d) stack space where d = maximum nesting depth (recursion overhead).

For target workloads (depth ≤ 100), stack usage is ~10KB, well within limits.

## Type Safety Guarantees

**Lifetime Safety**: All returned references borrow from source pattern. Compiler prevents use-after-free:

```rust
let pattern = Pattern::point("a");
let result = pattern.find_first(|_| true);
// result borrows from pattern
// cannot move or drop pattern while result exists
```

**Type Safety**: Generic bounds ensure operations are only available when values support required operations:

```rust
// matches/contains require PartialEq
impl<V> Pattern<V> {
    pub fn matches(&self, other: &Pattern<V>) -> bool
    where
        V: PartialEq, // Only available when V comparable
    { ... }
}
```

## Integration Points

New functions integrate with existing Pattern operations:

```rust
// Combine find_first with structural queries
pattern.find_first(|p| p.length() > 2 && p.depth() < 5)

// Use matches in filter
pattern.filter(|p| p.matches(&target))

// Chain with map
pattern.map(transform).find_first(predicate)

// Use with fold
let has_match = pattern.fold(false, |acc, v| acc || predicate(v));
```

## Testing Data Structures

Test patterns used for verification:

**Atomic Pattern**: `Pattern::point(value)` - no elements
**Small Nested**: 2-3 levels, 5-10 nodes
**Deep Nested**: 10+ levels, verify stack handling
**Wide Pattern**: Single level, 10+ elements
**Balanced Tree**: Depth = log₂(nodes)

## Equivalence Verification

Behavioral equivalence with gram-hs verified through:
1. Unit tests with identical inputs/outputs
2. Property-based tests for mathematical properties
3. Edge case coverage matching gram-hs test suite

Reference implementation: `../pattern-hs/libs/pattern/src/Pattern/Core.hs`
