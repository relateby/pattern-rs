# Type Signatures: Foldable Instance for Pattern

**Feature**: 009-foldable-instance  
**Date**: 2026-01-04  
**Status**: Draft

## Overview

This document defines the API contract for folding operations on `Pattern<V>`. These methods enable reducing a pattern structure to a single value by processing all values in depth-first, root-first order.

---

## Public API

### fold

Folds the pattern into a single value by applying a function to each value with an accumulator.

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn fold<B, F>(&self, init: B, f: F) -> B
    where
        F: Fn(B, &V) -> B;
}
```

**Type Parameters**:
- `V`: Value type stored in the pattern
- `B`: Accumulator type (independent of `V`)
- `F`: Function type that combines accumulator and value

**Parameters**:
- `self: &Pattern<V>` - Pattern to fold (borrowed, not consumed)
- `init: B` - Initial accumulator value
- `f: F` - Folding function with signature `Fn(B, &V) -> B`
  - First parameter: Accumulator (passed by value)
  - Second parameter: Value reference (borrowed from pattern)
  - Returns: New accumulator value

**Returns**: `B` - Final accumulated value

**Semantics**:
- Processes values in depth-first, root-first order (pre-order traversal)
- Root value processed first
- Elements processed left to right, recursively
- Each value processed exactly once
- Pattern structure preserved (not modified)
- Pattern can be reused after fold

**Time Complexity**: O(n) where n = total number of values  
**Space Complexity**: O(d) where d = maximum nesting depth (stack space)

**Examples**:
```rust
// Sum all integers
let sum: i32 = pattern.fold(0, |acc, v| acc + v);

// Count values
let count: usize = pattern.fold(0, |acc, _| acc + 1);

// Concatenate strings  
let concat: String = pattern.fold(String::new(), |acc, s| acc + s);

// Build vector
let values: Vec<i32> = pattern.fold(Vec::new(), |mut acc, v| {
    acc.push(*v);
    acc
});

// Check all values satisfy predicate
let all_positive: bool = pattern.fold(true, |acc, v| acc && *v > 0);

// Type transformation (string lengths to sum)
let total_len: usize = string_pattern.fold(0, |acc, s| acc + s.len());
```

**Behavioral Contract**:
1. Function `f` called exactly once per value in pattern
2. Call order: root first, then elements (left to right, depth-first)
3. Pattern unchanged after fold
4. Type safety ensured by Rust's type system

**Error Conditions**: None (pure operation, cannot fail)

---

### values

Collects all values from the pattern into a vector in traversal order.

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn values(&self) -> Vec<&V>;
}
```

**Type Parameters**:
- `V`: Value type stored in the pattern

**Parameters**:
- `self: &Pattern<V>` - Pattern to extract values from

**Returns**: `Vec<&V>` - Vector of references to all values in the pattern

**Semantics**:
- Returns references to values (no cloning)
- Order: depth-first, root-first (same as fold)
- Root value first in vector
- Elements follow in traversal order
- Implemented using `fold` internally

**Time Complexity**: O(n) where n = total number of values  
**Space Complexity**: O(n) for result vector

**Examples**:
```rust
// Get all values
let all_values: Vec<&i32> = pattern.values();
assert_eq!(all_values.len(), pattern.size());

// Verify order
let pattern = Pattern::pattern("A", vec![
    Pattern::point("B"),
    Pattern::point("C"),
]);
let values: Vec<&str> = pattern.values();
assert_eq!(values, vec![&"A", &"B", &"C"]);

// Use with iterator methods
let sum: i32 = pattern.values().iter().map(|&&v| v).sum();

// Check all values
let all_positive = pattern.values().iter().all(|&&v| v > 0);
```

**Behavioral Contract**:
1. Returns reference to every value in pattern exactly once
2. Order matches fold traversal order
3. Pattern unchanged after call
4. Can be called multiple times with same result

**Error Conditions**: None (pure operation, cannot fail)

---

## Internal API

### fold_with

Internal helper for recursive fold implementation.

**Signature**:
```rust
impl<V> Pattern<V> {
    fn fold_with<B, F>(&self, acc: B, f: &F) -> B
    where
        F: Fn(B, &V) -> B;
}
```

**Type Parameters**:
- `V`: Value type stored in the pattern
- `B`: Accumulator type
- `F`: Function type

**Parameters**:
- `self: &Pattern<V>` - Pattern to fold
- `acc: B` - Current accumulator value
- `f: &F` - Reference to folding function (not owned)

**Returns**: `B` - New accumulated value

**Purpose**: 
- Enables efficient recursion without cloning closure
- Public `fold` passes closure by value for ergonomics
- Internal `fold_with` passes closure by reference for efficiency
- Same pattern as `map`/`map_with` from Functor instance

**Visibility**: Private (`fn` not `pub fn`)

**Implementation Strategy**:
```rust
fn fold_with<B, F>(&self, acc: B, f: &F) -> B
where
    F: Fn(B, &V) -> B,
{
    // Process root value
    let acc = f(acc, &self.value);
    
    // Process elements recursively
    self.elements
        .iter()
        .fold(acc, |acc, elem| elem.fold_with(acc, f))
}
```

---

## Type Constraints

### Pattern Type

```rust
pub struct Pattern<V> {
    pub value: V,
    pub elements: Vec<Pattern<V>>,
}
```

**Constraints for Folding**:
- No trait bounds required on `V`
- Works with any value type
- Pattern can be empty (atomic), shallow, deep, or wide

### Function Type

**Constraint**: `F: Fn(B, &V) -> B`

**Requirements**:
- Must be callable multiple times (immutable borrow)
- Takes accumulator by value (ownership transfer)
- Takes value by reference (borrow from pattern)
- Returns new accumulator by value
- Should be pure (no side effects expected)

**Why `Fn` not `FnMut` or `FnOnce`**:
- `Fn` is most general (can be called multiple times immutably)
- Fold needs to call function once per value
- `FnMut` would require mutable borrow (unnecessary)
- `FnOnce` could only be called once (insufficient)

---

## Behavioral Laws

### Fold Properties

While fold doesn't have formal "laws" like Functor, it guarantees:

1. **Completeness**: All values processed
   ```rust
   let count = pattern.fold(0, |acc, _| acc + 1);
   assert_eq!(count, pattern.size());
   ```

2. **Order Preservation**: Deterministic traversal
   ```rust
   let result1 = pattern.fold(init, f);
   let result2 = pattern.fold(init, f);
   assert_eq!(result1, result2);  // Same result for same inputs
   ```

3. **Non-destructive**: Pattern unchanged
   ```rust
   let original = pattern.clone();
   let _ = pattern.fold(init, f);
   assert_eq!(pattern, original);
   ```

4. **Composition**: Can fold multiple times
   ```rust
   let sum = pattern.fold(0, |acc, v| acc + v);
   let product = pattern.fold(1, |acc, v| acc * v);
   // Both operations succeed, pattern unchanged
   ```

---

## Equivalence with Haskell

### Haskell Foldable Instance

```haskell
instance Foldable Pattern where
  foldMap f (Pattern v es) = f v <> foldMap (foldMap f) es
```

### Rust Equivalent

```rust
impl<V> Pattern<V> {
    fn fold_with<B, F>(&self, acc: B, f: &F) -> B
    where
        F: Fn(B, &V) -> B,
    {
        let acc = f(acc, &self.value);  // f v
        self.elements
            .iter()
            .fold(acc, |acc, e| e.fold_with(acc, f))  // <> foldMap (foldMap f) es
    }
}
```

**Key Differences**:
- Haskell uses `foldMap` with Monoid constraint
- Rust uses `fold` with explicit accumulator
- Both process root first, then elements
- Both maintain same traversal order

**Behavioral Equivalence**:
- Same values processed in same order
- Same structure preservation
- Same ability to reduce to single value

---

## Integration with Existing API

### Composability with Map (Functor)

```rust
// Map then fold
pattern.map(f).fold(init, g)

// Fold then use result
let result = pattern.fold(init, f);
process(result);
```

### Relationship to Existing Methods

```rust
// Size equals fold count
assert_eq!(pattern.size(), pattern.fold(0, |acc, _| acc + 1));

// Values method uses fold
pub fn values(&self) -> Vec<&V> {
    self.fold(Vec::new(), |mut acc, v| { acc.push(v); acc })
}

// Can check properties via fold
let is_atomic = pattern.fold(0, |acc, _| acc + 1) == 1;
assert_eq!(is_atomic, pattern.is_atomic());
```

---

## Performance Guarantees

### Time Complexity

| Operation | Complexity | Description |
|-----------|------------|-------------|
| `fold` | O(n) | n = total values in pattern |
| `values` | O(n) | Uses fold, same complexity |

### Space Complexity

| Operation | Stack Space | Heap Space |
|-----------|-------------|------------|
| `fold` | O(d) | O(1) |
| `values` | O(d) | O(n) |

Where:
- `n` = total number of values in pattern
- `d` = maximum nesting depth

### Scale Targets

- **Node count**: 1000 nodes in <10ms
- **Nesting depth**: 100 levels without stack overflow
- **Memory**: 10,000 elements without exceeding 100MB

---

## Testing Requirements

### Unit Tests

Required test cases:
1. Atomic pattern (single value)
2. Flat pattern (one level, multiple elements)
3. Nested pattern (multiple levels)
4. Different accumulator types (int, string, vec, bool)
5. Type transformation (V ≠ B)
6. Order verification (non-commutative operation)

### Property Tests

Required properties:
1. Count equals size: `fold(0, |acc, _| acc + 1) == size()`
2. Values length equals size: `values().len() == size()`
3. Fold is deterministic: Same inputs → same output
4. Pattern unchanged: Structure preserved after fold

### Performance Tests

Required benchmarks:
1. Large patterns (1000 nodes) < 10ms
2. Deep patterns (100 levels) no stack overflow
3. Wide patterns (1000 siblings) acceptable performance

---

## WASM Compatibility

All methods must compile for `wasm32-unknown-unknown` target:

```bash
cargo build --package pattern-core --target wasm32-unknown-unknown
```

**Constraints**:
- No platform-specific dependencies
- Stack usage must be reasonable (WASM has limited stack)
- No unsafe code required

---

## Migration Notes

### For Users of gram-hs

Haskell users familiar with `Foldable` should note:

| Haskell | Rust Equivalent | Notes |
|---------|----------------|-------|
| `foldMap` | `fold` | Rust uses explicit accumulator |
| `foldr` | `fold` | Right fold with accumulator |
| `foldl` | N/A | Use `values()` then Vec::fold |
| `toList` | `values()` | Returns Vec<&V> |
| `null` | `is_atomic()` | Existing method |
| `length` | `size()` | Existing method |

### Breaking Changes

None - this is a new feature. Adds methods to existing `Pattern<V>` type.

---

## References

- **Haskell Foldable**: `Data.Foldable` typeclass
- **gram-hs Implementation**: `../pattern-hs/libs/pattern/src/Pattern/Core.hs` (line 750-751)
- **Rust Iterator::fold**: Standard library pattern
- **Feature 008**: Functor instance (`map` method) for API consistency
