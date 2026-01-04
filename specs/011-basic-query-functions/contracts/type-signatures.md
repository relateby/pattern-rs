# API Contracts: Pattern Query Operations

**Date**: 2025-01-04  
**Phase**: 1 - Design & Contracts  
**Target**: `crates/pattern-core/src/pattern.rs`

## Overview

This document specifies the public API signatures for the three new query operations and documents the existing operations that will receive enhanced test coverage.

## New Public APIs

### 1. any_value

**Purpose**: Check if at least one value in a pattern satisfies a given predicate

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn any_value<F>(&self, predicate: F) -> bool
    where
        F: Fn(&V) -> bool,
    {
        // Implementation leverages existing fold()
    }
}
```

**Type Parameters**:
- `V`: The value type of the pattern (inherited from Pattern<V>)
- `F`: The predicate function type

**Parameters**:
- `&self`: Immutable reference to the pattern
- `predicate`: A function that takes a reference to a value and returns bool

**Return Value**:
- `bool`: `true` if at least one value satisfies the predicate, `false` otherwise

**Haskell Equivalent**: `anyValue :: (v -> Bool) -> Pattern v -> Bool`

**Complexity**:
- Time: O(n) worst case, O(1) to O(n) average (short-circuits on first match)
- Space: O(1) heap, O(d) stack where d = max depth

**Behavioral Contract**:
- MUST traverse values in pre-order (root, then elements in order)
- MUST short-circuit and return `true` on first matching value
- MUST return `false` for empty patterns (atomic patterns with no further nesting)
- MUST NOT mutate the pattern or its values
- MUST be safe to call on patterns of any depth or size

**Example Usage**:
```rust
let pattern = Pattern::pattern(5, vec![
    Pattern::point(10),
    Pattern::point(3),
]);

assert!(pattern.any_value(|v| *v > 8));  // true (10 > 8)
assert!(!pattern.any_value(|v| *v > 20)); // false (no value > 20)
```

---

### 2. all_values

**Purpose**: Check if all values in a pattern satisfy a given predicate

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn all_values<F>(&self, predicate: F) -> bool
    where
        F: Fn(&V) -> bool,
    {
        // Implementation leverages existing fold()
    }
}
```

**Type Parameters**:
- `V`: The value type of the pattern (inherited from Pattern<V>)
- `F`: The predicate function type

**Parameters**:
- `&self`: Immutable reference to the pattern
- `predicate`: A function that takes a reference to a value and returns bool

**Return Value**:
- `bool`: `true` if all values satisfy the predicate, `false` otherwise

**Haskell Equivalent**: `allValues :: (v -> Bool) -> Pattern v -> Bool`

**Complexity**:
- Time: O(n) worst case, O(1) to O(n) average (short-circuits on first non-match)
- Space: O(1) heap, O(d) stack where d = max depth

**Behavioral Contract**:
- MUST traverse values in pre-order (root, then elements in order)
- MUST short-circuit and return `false` on first non-matching value
- MUST return `true` for empty patterns (vacuous truth)
- MUST NOT mutate the pattern or its values
- MUST be safe to call on patterns of any depth or size

**Example Usage**:
```rust
let pattern = Pattern::pattern(5, vec![
    Pattern::point(10),
    Pattern::point(8),
]);

assert!(pattern.all_values(|v| *v > 0));  // true (all positive)
assert!(!pattern.all_values(|v| *v > 8)); // false (5 is not > 8)

// Empty pattern (atomic) - vacuous truth
let empty = Pattern::point(5);
assert!(empty.elements.is_empty());
// Note: empty.elements is empty, but pattern still has a value (5)
```

---

### 3. filter

**Purpose**: Extract all subpatterns (including root) that satisfy a given pattern predicate

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn filter<F>(&self, predicate: F) -> Vec<&Pattern<V>>
    where
        F: Fn(&Pattern<V>) -> bool,
    {
        // Custom recursive implementation
    }
}
}
```

**Type Parameters**:
- `V`: The value type of the pattern (inherited from Pattern<V>)
- `F`: The pattern predicate function type

**Parameters**:
- `&self`: Immutable reference to the pattern
- `predicate`: A function that takes a reference to a pattern and returns bool

**Return Value**:
- `Vec<&Pattern<V>>`: Collection of immutable references to matching patterns

**Haskell Equivalent**: `filterPatterns :: (Pattern v -> Bool) -> Pattern v -> [Pattern v]`

**Complexity**:
- Time: O(n) where n = total number of nodes
- Space: O(m) heap where m = number of matches, O(d) stack where d = max depth

**Behavioral Contract**:
- MUST check predicate on current pattern before recursing to elements
- MUST recursively check all element patterns at all nesting levels
- MUST return matches in pre-order traversal order (root first, then elements in order)
- MUST return empty vec if no patterns match
- MUST include root pattern in results if it matches predicate
- MUST NOT mutate the pattern or its elements
- Returned references are valid as long as the source pattern is valid

**Example Usage**:
```rust
let pattern = Pattern::pattern(
    "root",
    vec![
        Pattern::point("a"),
        Pattern::pattern("b", vec![Pattern::point("c")]),
    ],
);

// Find all atomic patterns
let atomic = pattern.filter(|p| p.elements.is_empty());
assert_eq!(atomic.len(), 2); // "a" and "c"

// Find patterns with specific value
let matched = pattern.filter(|p| p.value == "root");
assert_eq!(matched.len(), 1);
assert_eq!(matched[0].value, "root");

// Find all patterns (const true predicate)
let all = pattern.filter(|_| true);
assert_eq!(all.len(), 4); // root + 3 subpatterns
```

## Existing APIs (Enhanced Testing)

### 4. length

**Purpose**: Returns the number of direct elements in a pattern's sequence

**Signature** (existing):
```rust
impl<V> Pattern<V> {
    pub fn length(&self) -> usize {
        self.elements.len()
    }
}
```

**Haskell Equivalent**: `length :: Pattern v -> Int`

**Enhanced Test Requirements**:
- Test with atomic patterns (should return 0)
- Test with patterns having 1, 2, many direct elements
- Test that it only counts direct elements (not nested descendants)
- Cross-implementation equivalence testing with gram-hs

---

### 5. size

**Purpose**: Returns the total number of nodes in a pattern structure (including all nested patterns)

**Signature** (existing):
```rust
impl<V> Pattern<V> {
    pub fn size(&self) -> usize {
        1 + self.elements.iter().map(|e| e.size()).sum::<usize>()
    }
}
```

**Haskell Equivalent**: `size :: Pattern v -> Int`

**Enhanced Test Requirements**:
- Test with atomic patterns (should return 1)
- Test with flat patterns (1 + direct element count)
- Test with deeply nested patterns (correct total count)
- Test with patterns having varying branch depths
- Performance test with large patterns (10,000+ nodes)
- Cross-implementation equivalence testing with gram-hs

---

### 6. depth

**Purpose**: Returns the maximum nesting depth of a pattern structure

**Signature** (existing):
```rust
impl<V> Pattern<V> {
    pub fn depth(&self) -> usize {
        if self.elements.is_empty() {
            0
        } else {
            1 + self.elements.iter().map(|e| e.depth()).max().unwrap_or(0)
        }
    }
}
```

**Haskell Equivalent**: `depth :: Pattern v -> Int`

**Enhanced Test Requirements**:
- Test with atomic patterns (should return 0)
- Test with one level of nesting (should return 1)
- Test with deeply nested patterns (correct max depth)
- Test with patterns having branches of different depths (returns maximum)
- Performance test with very deep patterns (100+ levels)
- Cross-implementation equivalence testing with gram-hs

---

### 7. values

**Purpose**: Extracts all values from a pattern structure as a flat list

**Signature** (existing):
```rust
impl<V> Pattern<V> {
    pub fn values(&self) -> Vec<&V> {
        let mut result = Vec::new();
        self.fold((), |_, v| {
            result.push(v);
        });
        result
    }
}
```

**Haskell Equivalent**: `values :: Pattern v -> [v]`

**Enhanced Test Requirements**:
- Test with atomic patterns (should return single-element list)
- Test with nested patterns (should return all values in pre-order)
- Test order consistency (parent first, then elements in order, recursively)
- Test with duplicate values (should return all, including duplicates)
- Performance test with large patterns (10,000+ nodes)
- Cross-implementation equivalence testing with gram-hs

## API Consistency Rules

### Naming Conventions

All query operations follow these conventions:
- Use `snake_case` for method names (Rust convention)
- Use descriptive names that indicate purpose
- Prefix with operation type when appropriate (`any_`, `all_`, `filter`)

### Predicate Conventions

- Value predicates: `F: Fn(&V) -> bool`
- Pattern predicates: `F: Fn(&Pattern<V>) -> bool`
- Always take references (no unnecessary copies)
- Never require Clone, Copy, or other bounds on V

### Return Type Conventions

- Boolean results: Return `bool` directly
- Collection results: Return `Vec<&T>` for references, `Vec<T>` only if necessary
- Never return `Option` unless operation can fail (these operations always succeed)

## Cross-Cutting Concerns

### Thread Safety

All operations are read-only and safe to call from multiple threads if:
- `Pattern<V>` is `Sync` (which it is when `V: Sync`)
- No interior mutability is used

### WASM Compatibility

All operations are pure computation with no platform-specific code:
- No I/O operations
- No file system access
- No platform-specific APIs
- Compatible with `wasm32-unknown-unknown` target

### Performance Guarantees

- `any_value` and `all_values`: O(n) worst case, short-circuit on early match/non-match
- `filter`: O(n) always (must visit all nodes)
- All operations: O(d) stack usage where d = max depth
- Safe for deep nesting (100+ levels tested)

## Integration Points

### With Existing Pattern Methods

New operations integrate with:
- `fold()` - Used by `any_value` and `all_values`
- `length()` - Can be used in predicates passed to `filter`
- `size()` - Can be used in predicates passed to `filter`
- `depth()` - Can be used in predicates passed to `filter`
- `values()` - Complementary value extraction

### With Test Infrastructure

- Uses existing `proptest` generators from `test_utils/generators.rs`
- Uses existing equivalence testing from `tests/equivalence/`
- Uses existing benchmark infrastructure from `benches/`

## Migration/Compatibility

### Breaking Changes

None. This is a purely additive change.

### Deprecations

None.

### Version Requirements

- Minimum Rust version: 1.75+ (matches existing codebase)
- No new external dependencies

## Documentation Requirements

Each method MUST include:
1. Summary description
2. Detailed semantics
3. Complexity analysis (time and space)
4. Edge case behavior
5. Example usage (at least 2 examples)
6. Reference to Haskell equivalent
7. Note about short-circuit behavior (for `any_value`, `all_values`)

## Validation

### Compile-Time Validation

- Type safety enforced by Rust's type system
- Lifetime correctness verified by borrow checker
- Generic constraints enforced at monomorphization

### Runtime Validation

- Property tests verify correctness properties
- Equivalence tests verify behavioral equivalence with Haskell
- Performance tests verify complexity guarantees
- Edge case tests verify corner case handling

## Summary

This feature adds 3 new methods to `Pattern<V>` and enhances testing for 4 existing methods:

**New Methods**:
1. `any_value` - Check if any value satisfies predicate
2. `all_values` - Check if all values satisfy predicate
3. `filter` - Extract matching subpatterns

**Enhanced Testing**:
4. `length` - Count direct elements
5. `size` - Count total nodes
6. `depth` - Maximum nesting depth
7. `values` - Extract all values

All methods maintain behavioral equivalence with the Haskell reference implementation while following Rust idioms and conventions.

