# API Contracts: Comonad Operations

**Feature**: 018-comonad-instance  
**Date**: 2026-01-05

## Overview

This document specifies the API contracts for Comonad operations on Pattern. All signatures, laws, and behavioral requirements are documented here.

## Type Signatures

### Core Operations

#### extract

```rust
impl<V> Pattern<V> {
    /// Extracts the decorative value at the current position.
    ///
    /// # Returns
    /// Reference to the value field (the decoration)
    ///
    /// # Complexity
    /// Time: O(1), Space: O(1)
    ///
    /// # Examples
    /// ```
    /// let p = Pattern { value: 42, elements: vec![] };
    /// assert_eq!(p.extract(), &42);
    /// ```
    pub fn extract(&self) -> &V;
}
```

**Contract**:
- **Input**: `&self` (immutable reference to Pattern)
- **Output**: `&V` (reference to value, lifetime tied to self)
- **Preconditions**: None (always valid)
- **Postconditions**: Returns `&self.value`
- **Side Effects**: None (pure function)
- **Panics**: Never
- **Error Conditions**: None

#### extend

```rust
impl<V> Pattern<V> {
    /// Computes new decorative information at each position.
    ///
    /// Takes a function that receives the full subpattern at each position
    /// and computes a new decoration based on that context.
    ///
    /// # Type Parameters
    /// - `W`: The type of new decorative values
    /// - `F`: The function type (must be `Fn(&Pattern<V>) -> W`)
    ///
    /// # Arguments
    /// - `f`: Context-aware function that computes new decoration
    ///
    /// # Returns
    /// New pattern with same structure, decorated with computed values
    ///
    /// # Complexity
    /// Time: O(n) where n = node count, Space: O(n)
    ///
    /// # Examples
    /// ```
    /// let p = Pattern { value: "root", elements: vec![point("a"), point("b")] };
    /// let depths = p.extend(&|subp| subp.depth());
    /// assert_eq!(depths.extract(), &0);  // root has depth 0
    /// ```
    pub fn extend<W, F>(&self, f: &F) -> Pattern<W>
    where
        F: Fn(&Pattern<V>) -> W;
}
```

**Contract**:
- **Input**: 
  - `&self` (immutable reference to Pattern)
  - `f: &F` (reference to function that takes Pattern reference and returns W)
- **Output**: `Pattern<W>` (new pattern with transformed values)
- **Preconditions**: 
  - Function `f` must be pure (no side effects observable through API)
  - Function `f` must terminate for all patterns
- **Postconditions**:
  - Output structure matches input structure (same node count, same tree shape)
  - Output root value equals `f(self)`
  - Output element `i` equals `self.elements[i].extend(f)`
  - Comonad laws hold (see Laws section)
- **Side Effects**: None (pure function, creates new pattern)
- **Panics**: Only if function `f` panics
- **Error Conditions**: None (beyond `f` panicking)

### Helper Functions

#### depth_at

```rust
impl<V> Pattern<V> {
    /// Decorates each position with its depth (maximum nesting level).
    ///
    /// Depth is defined as:
    /// - Atomic pattern (no elements): depth 0
    /// - Pattern with elements: depth = 1 + max(child depths)
    ///
    /// # Returns
    /// Pattern where each position's value is its depth
    ///
    /// # Complexity
    /// Time: O(n), Space: O(n)
    ///
    /// # Examples
    /// ```
    /// let p = point("x");
    /// assert_eq!(p.depth_at().extract(), &0);
    ///
    /// let p = Pattern { value: "root", elements: vec![
    ///     Pattern { value: "a", elements: vec![point("x")] }
    /// ]};
    /// assert_eq!(p.depth_at().extract(), &1);
    /// ```
    pub fn depth_at(&self) -> Pattern<usize>;
}
```

**Contract**:
- **Input**: `&self`
- **Output**: `Pattern<usize>` with depths
- **Preconditions**: None
- **Postconditions**:
  - Structure preserved
  - Each position's value equals the depth of that subpattern
  - Atomic patterns decorated with 0
  - Patterns with elements decorated with 1 + max(child depths)
- **Implementation**: Uses `self.extend(&|p| p.depth())`
- **Side Effects**: None
- **Panics**: Never
- **Error Conditions**: None

#### size_at

```rust
impl<V> Pattern<V> {
    /// Decorates each position with the total node count of its subtree.
    ///
    /// Size is defined as:
    /// - 1 (self) + sum of child sizes
    ///
    /// # Returns
    /// Pattern where each position's value is its subtree size
    ///
    /// # Complexity
    /// Time: O(n), Space: O(n)
    ///
    /// # Examples
    /// ```
    /// let p = point("x");
    /// assert_eq!(p.size_at().extract(), &1);
    ///
    /// let p = Pattern { value: "root", elements: vec![point("a"), point("b")] };
    /// assert_eq!(p.size_at().extract(), &3);
    /// ```
    pub fn size_at(&self) -> Pattern<usize>;
}
```

**Contract**:
- **Input**: `&self`
- **Output**: `Pattern<usize>` with sizes
- **Preconditions**: None
- **Postconditions**:
  - Structure preserved
  - Each position's value equals the size of that subpattern
  - Size = 1 + sum(child sizes)
- **Implementation**: Uses `self.extend(&|p| p.size())`
- **Side Effects**: None
- **Panics**: Never
- **Error Conditions**: None

#### indices_at

```rust
impl<V> Pattern<V> {
    /// Decorates each position with its path from root.
    ///
    /// Path is a vector of element indices from root to this position.
    /// - Root: empty vector []
    /// - Child at index i: parent_path + [i]
    ///
    /// # Returns
    /// Pattern where each position's value is its path from root
    ///
    /// # Complexity
    /// Time: O(n), Space: O(n * depth)
    ///
    /// # Examples
    /// ```
    /// let p = point("x");
    /// assert_eq!(p.indices_at().extract(), &vec![]);
    ///
    /// let p = Pattern { value: "root", elements: vec![point("a"), point("b")] };
    /// let result = p.indices_at();
    /// assert_eq!(result.extract(), &vec![]);
    /// assert_eq!(result.elements[0].extract(), &vec![0]);
    /// assert_eq!(result.elements[1].extract(), &vec![1]);
    /// ```
    pub fn indices_at(&self) -> Pattern<Vec<usize>>;
}
```

**Contract**:
- **Input**: `&self`
- **Output**: `Pattern<Vec<usize>>` with paths
- **Preconditions**: None
- **Postconditions**:
  - Structure preserved
  - Root decorated with empty vector `[]`
  - Element at index `i` decorated with `parent_path + [i]`
  - Paths uniquely identify each position
- **Implementation**: Direct recursion with path accumulation (not using extend)
- **Side Effects**: None
- **Panics**: Never
- **Error Conditions**: None

## Comonad Laws

These are mathematical properties that MUST hold for all patterns and functions.

### Law 1: Extract-Extend (Left Identity)

**Specification**:
```rust
for all p: Pattern<V>, f: Fn(&Pattern<V>) -> W:
    p.extend(f).extract() == f(&p)
```

**English**: Extracting the root value after extending gives the same result as applying the function to the original pattern.

**Why**: By definition of `extend`, the root value of the result is `f(p)`.

**Test Strategy**: Property-based test with arbitrary patterns and functions.

**Example**:
```rust
let p = Pattern { value: "x", elements: vec![] };
let f = |p: &Pattern<&str>| p.depth();

assert_eq!(p.extend(&f).extract(), &f(&p));
// Both sides equal 0
```

### Law 2: Extend-Extract (Right Identity)

**Specification**:
```rust
for all p: Pattern<V>:
    p.extend(&|p| p.extract().clone()) == p
```

**English**: Extending with extract returns the pattern unchanged.

**Why**: At each position, we're computing the decoration using `extract`, which returns the existing decoration.

**Note**: Requires `V: Clone` for the test (or use `PartialEq` to compare)

**Test Strategy**: Property-based test with arbitrary patterns.

**Example**:
```rust
let p = Pattern { value: 42, elements: vec![point(10), point(20)] };
let result = p.extend(&|p| p.extract().clone());

assert_eq!(result, p);
```

### Law 3: Extend-Extend (Associativity)

**Specification**:
```rust
for all p: Pattern<V>, f: Fn(&Pattern<V>) -> W, g: Fn(&Pattern<V>) -> X:
    p.extend(g).extend(f) == p.extend(&|p| f(&p.extend(g)))
```

**English**: Extending twice in sequence is the same as extending once with the composed function.

**Why**: The order of applying context-aware transformations shouldn't change the result structure.

**Test Strategy**: Property-based test with arbitrary patterns and two functions.

**Example**:
```rust
let p = Pattern { value: "x", elements: vec![point("a"), point("b")] };
let f = |p: &Pattern<&str>| p.depth();
let g = |p: &Pattern<&str>| p.size();

let left = p.extend(&g).extend(&f);
let right = p.extend(&|p: &Pattern<&str>| f(&p.extend(&g)));

assert_eq!(left, right);
```

## Behavioral Requirements

### Structure Preservation

**Requirement**: All Comonad operations preserve pattern structure.

**Formally**:
```rust
for all p: Pattern<V>, f: Fn(&Pattern<V>) -> W:
    let result = p.extend(f);
    assert_eq!(result.node_count(), p.node_count());
    assert_eq!(result.depth(), p.depth());
    assert_eq!(result.elements.len(), p.elements.len());
```

**Verification**: Property-based test

### Purity

**Requirement**: All operations are pure (no observable side effects).

**Formally**: Given same input, always produce same output. No mutation of input.

**Verification**: 
- Multiple calls with same input produce identical results
- Original pattern unchanged after operation

### Termination

**Requirement**: All operations terminate for finite patterns.

**Assumption**: Pattern is finite (no infinite recursion in structure).

**Verification**: Operations complete in bounded time proportional to pattern size.

## Performance Contracts

### Time Complexity

| Operation | Worst Case | Average Case | Best Case |
|-----------|------------|--------------|-----------|
| `extract` | O(1) | O(1) | O(1) |
| `extend` | O(n) | O(n) | O(n) |
| `depth_at` | O(n) | O(n) | O(n) |
| `size_at` | O(n) | O(n) | O(n) |
| `indices_at` | O(n) | O(n) | O(n) |

Where n = total number of nodes in the pattern.

### Space Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| `extract` | O(1) | Returns reference, no allocation |
| `extend` | O(n) | Creates new pattern with n nodes |
| `depth_at` | O(n) | New pattern with n nodes |
| `size_at` | O(n) | New pattern with n nodes |
| `indices_at` | O(n * d) | New pattern + path vectors (d = depth) |

### Performance Targets

From spec.md Success Criteria:
- **SC-006**: Process 1000+ elements in <100ms
- **SC-007**: Operations complete in O(n) time

**Verification**: Benchmark suite with patterns of varying sizes.

## Error Handling

### No Errors

Comonad operations do not return `Result` or `Option`. They always succeed for valid patterns.

**Rationale**: Operations are always well-defined for any pattern structure.

### Panics

**Conditions**:
- Function `f` passed to `extend` panics → `extend` propagates panic
- Out of memory (pattern too large) → allocation panic

**Not panics**:
- Empty patterns → Valid (atomic patterns work fine)
- Large patterns → Should complete (barring OOM)
- Deep nesting → Should handle (barring stack overflow, which is rare for reasonable depths)

## Compatibility

### WASM Compatibility

**Status**: ✅ Fully compatible

**Rationale**: All operations are pure, no I/O, no blocking, no platform-specific code.

### Thread Safety

**Status**: ✅ Safe (with appropriate bounds)

**Details**:
- Pattern operations are pure → no shared mutable state
- If `V: Send`, then `Pattern<V>: Send`
- If `V: Sync`, then `Pattern<V>: Sync`

### FFI Compatibility

**Status**: ⚠️ Requires wrappers

**Challenges**:
- `extend` takes function parameter → cannot expose directly via C FFI
- Generic types require monomorphization

**Solution**: Provide specific monomorphized versions or callback-based API.

## Versioning

**Initial Version**: 1.0.0 (when feature is released)

**Breaking Changes**: 
- Changing function signature (e.g., adding required trait bounds)
- Changing semantics (e.g., violating laws)
- Removing operations

**Non-Breaking Changes**:
- Adding helper functions
- Performance improvements
- Documentation improvements

## Testing Requirements

### Property-Based Tests

**Required**:
- ✅ Law 1 (extract-extend) verified for arbitrary patterns
- ✅ Law 2 (extend-extract) verified for arbitrary patterns  
- ✅ Law 3 (associativity) verified for arbitrary patterns
- ✅ Structure preservation verified

**Tool**: `proptest` crate

### Unit Tests

**Required**:
- ✅ `depth_at` correctness on known patterns
- ✅ `size_at` correctness on known patterns
- ✅ `indices_at` correctness on known patterns
- ✅ Edge cases: atomic patterns, deeply nested, large patterns

### Behavioral Equivalence Tests

**Required**:
- ✅ Compare `depth_at` outputs with gram-hs
- ✅ Compare `size_at` outputs with gram-hs
- ✅ Compare `indices_at` outputs with gram-hs

**Method**: Test same pattern structures, verify identical outputs.

## Documentation Requirements

### Rustdoc

**Required**:
- ✅ Module-level docs explaining "decorated sequence" semantics
- ✅ Each function has doc comment with description
- ✅ Examples in doc comments
- ✅ Complexity documented
- ✅ Links to related functions

### Examples

**Required**:
- ✅ Basic usage of `extract` and `extend`
- ✅ Using `depth_at`, `size_at`, `indices_at`
- ✅ Composing comonad operations with existing Pattern methods
- ✅ Practical use case (e.g., visualization metadata)

## References

- **Haskell Comonad laws**: https://hackage.haskell.org/package/comonad
- **gram-hs implementation**: `../pattern-hs/libs/pattern/src/Pattern/Core.hs`
- **Feature spec**: [spec.md](../spec.md)
- **Data model**: [data-model.md](../data-model.md)
