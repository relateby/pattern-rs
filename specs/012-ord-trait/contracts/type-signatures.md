# Type Signatures & API Contracts: Pattern Ordering

**Feature**: 012-ord-trait  
**Date**: 2025-01-04  
**Research**: [../research.md](../research.md) | **Data Model**: [../data-model.md](../data-model.md)

## Overview

This document defines the precise type signatures and API contracts for ordering operations on Pattern<V>. All signatures follow Rust's standard library conventions and maintain behavioral equivalence with the gram-hs reference implementation.

## Trait Implementations

### PartialOrd Trait

**Source**: `std::cmp::PartialOrd`  
**Purpose**: Partial ordering for patterns where value type implements PartialOrd

```rust
impl<V> PartialOrd for Pattern<V>
where
    V: PartialOrd,
{
    /// Compares two patterns, returning `Some(ordering)` if comparable, `None` otherwise.
    ///
    /// Uses value-first lexicographic comparison:
    /// 1. Compare pattern values
    /// 2. If equal, compare element vectors lexicographically
    ///
    /// # Returns
    /// - `Some(Ordering::Less)` if `self < other`
    /// - `Some(Ordering::Equal)` if `self == other`
    /// - `Some(Ordering::Greater)` if `self > other`
    /// - `None` if values cannot be compared (e.g., NaN in floats)
    ///
    /// # Examples
    /// ```rust
    /// use pattern_core::Pattern;
    /// use std::cmp::Ordering;
    ///
    /// let p1 = Pattern::point(1);
    /// let p2 = Pattern::point(2);
    /// assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Less));
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
}
```

**Contract**:
- **Input**: `&self`, `&other: &Self` - Immutable references to patterns
- **Output**: `Option<Ordering>` - Comparison result or None
- **Time Complexity**: O(1) best case (values differ), O(n) worst case (compare all nodes)
- **Space Complexity**: O(1) - no allocation, uses stack for recursion
- **Side Effects**: None (pure function)
- **Panics**: Never
- **Undefined Behavior**: None

**Properties**:
- If `partial_cmp` returns `Some`, the ordering is consistent with `PartialEq`
- May return `None` for incomparable values (e.g., NaN vs NaN in floating point)
- Comparison is **non-commutative**: `a.partial_cmp(&b) != b.partial_cmp(&a)` in general

### Ord Trait

**Source**: `std::cmp::Ord`  
**Purpose**: Total ordering for patterns where value type implements Ord

```rust
impl<V> Ord for Pattern<V>
where
    V: Ord,
{
    /// Compares two patterns, returning their ordering.
    ///
    /// Uses value-first lexicographic comparison:
    /// 1. Compare pattern values
    /// 2. If equal, compare element vectors lexicographically
    ///
    /// This method always returns a definitive ordering (never None).
    ///
    /// # Returns
    /// - `Ordering::Less` if `self < other`
    /// - `Ordering::Equal` if `self == other`
    /// - `Ordering::Greater` if `self > other`
    ///
    /// # Examples
    /// ```rust
    /// use pattern_core::Pattern;
    /// use std::cmp::Ordering;
    ///
    /// let p1 = Pattern::point(1);
    /// let p2 = Pattern::point(2);
    /// assert_eq!(p1.cmp(&p2), Ordering::Less);
    ///
    /// // Can use comparison operators
    /// assert!(p1 < p2);
    /// assert!(p1 <= p2);
    /// assert!(p2 > p1);
    /// assert!(p2 >= p1);
    /// ```
    fn cmp(&self, other: &Self) -> Ordering;
}
```

**Contract**:
- **Input**: `&self`, `&other: &Self` - Immutable references to patterns
- **Output**: `Ordering` - Always returns a definitive ordering
- **Time Complexity**: O(1) best case (values differ), O(n) worst case (compare all nodes)
- **Space Complexity**: O(1) - no allocation, uses stack for recursion
- **Side Effects**: None (pure function)
- **Panics**: Never
- **Undefined Behavior**: None

**Properties** (Ord Laws):
1. **Reflexivity**: `a.cmp(&a) == Ordering::Equal` for all a
2. **Antisymmetry**: `if a.cmp(&b) == Less then b.cmp(&a) == Greater`
3. **Transitivity**: `if a < b and b < c then a < c`
4. **Totality**: For all a, b, exactly one of `a < b`, `a == b`, `a > b` holds
5. **Consistency with Eq**: `a == b ⟺ a.cmp(&b) == Equal`

### Trait Dependencies

```rust
// Required trait hierarchy for Ord:
Pattern<V>: Ord
    ⇒ Pattern<V>: PartialOrd    // Provided by this feature
    ⇒ Pattern<V>: Eq            // Already implemented (feature 004)
    ⇒ Pattern<V>: PartialEq     // Already implemented (feature 004)

// Type constraints:
Pattern<V>: PartialOrd  ⟺  V: PartialOrd
Pattern<V>: Ord         ⟺  V: Ord
```

## Comparison Operators

### Standard Comparison Operators

Once `Ord` is implemented, patterns automatically support standard comparison operators:

```rust
// Less than
pub fn lt<V: Ord>(self: &Pattern<V>, other: &Pattern<V>) -> bool {
    self.cmp(other) == Ordering::Less
}

// Less than or equal
pub fn le<V: Ord>(self: &Pattern<V>, other: &Pattern<V>) -> bool {
    matches!(self.cmp(other), Ordering::Less | Ordering::Equal)
}

// Greater than
pub fn gt<V: Ord>(self: &Pattern<V>, other: &Pattern<V>) -> bool {
    self.cmp(other) == Ordering::Greater
}

// Greater than or equal
pub fn ge<V: Ord>(self: &Pattern<V>, other: &Pattern<V>) -> bool {
    matches!(self.cmp(other), Ordering::Greater | Ordering::Equal)
}
```

**Note**: These are automatically provided by the standard library when `Ord` is implemented.

### Min/Max Operations

```rust
// Minimum of two patterns
pub fn min<V: Ord>(self: Pattern<V>, other: Pattern<V>) -> Pattern<V> {
    if self <= other { self } else { other }
}

// Maximum of two patterns
pub fn max<V: Ord>(self: Pattern<V>, other: Pattern<V>) -> Pattern<V> {
    if self >= other { self } else { other }
}

// Clamp pattern to range [min, max]
pub fn clamp<V: Ord>(self: Pattern<V>, min: Pattern<V>, max: Pattern<V>) -> Pattern<V> {
    assert!(min <= max);
    if self < min { min } else if self > max { max } else { self }
}
```

**Note**: These are automatically provided by the standard library.

## Integration with Standard Library

### Iterator Methods

Once `Ord` is implemented, patterns can use ordering-based iterator methods:

```rust
// Find minimum pattern in collection
fn min<I>(iter: I) -> Option<Pattern<V>>
where
    I: Iterator<Item = Pattern<V>>,
    V: Ord,
{
    iter.min()  // Uses Ord::cmp
}

// Find maximum pattern in collection
fn max<I>(iter: I) -> Option<Pattern<V>>
where
    I: Iterator<Item = Pattern<V>>,
    V: Ord,
{
    iter.max()  // Uses Ord::cmp
}

// Find min and max simultaneously
fn min_max<I>(iter: I) -> Option<(Pattern<V>, Pattern<V>)>
where
    I: Iterator<Item = Pattern<V>>,
    V: Ord,
{
    iter.minmax()  // Uses Ord::cmp
}
```

### Sorting Methods

```rust
// Sort mutable vector of patterns in place
fn sort<V: Ord>(patterns: &mut Vec<Pattern<V>>) {
    patterns.sort();  // Uses Ord::cmp
}

// Sort with custom key function
fn sort_by_key<V, K, F>(patterns: &mut Vec<Pattern<V>>, f: F)
where
    K: Ord,
    F: FnMut(&Pattern<V>) -> K,
{
    patterns.sort_by_key(f);
}

// Binary search in sorted vector
fn binary_search<V: Ord>(patterns: &[Pattern<V>], target: &Pattern<V>) -> Result<usize, usize> {
    patterns.binary_search(target)  // Uses Ord::cmp
}
```

### Ordered Collections

```rust
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

// BTreeSet - ordered set of patterns (no duplicates)
let mut set: BTreeSet<Pattern<i32>> = BTreeSet::new();
set.insert(Pattern::point(3));
set.insert(Pattern::point(1));
set.insert(Pattern::point(2));
// Iteration in order: point(1), point(2), point(3)

// BTreeMap - ordered map with pattern keys
let mut map: BTreeMap<Pattern<i32>, String> = BTreeMap::new();
map.insert(Pattern::point(1), "first".to_string());
map.insert(Pattern::point(2), "second".to_string());
// Keys in order: point(1) before point(2)

// BinaryHeap - max-heap of patterns
let mut heap: BinaryHeap<Pattern<i32>> = BinaryHeap::new();
heap.push(Pattern::point(1));
heap.push(Pattern::point(3));
heap.push(Pattern::point(2));
assert_eq!(heap.pop(), Some(Pattern::point(3)));  // Largest first
```

## Comparison Algorithm Specification

### Detailed Algorithm

```rust
// Conceptual implementation (actual implementation may vary)
fn cmp<V: Ord>(p1: &Pattern<V>, p2: &Pattern<V>) -> Ordering {
    // Step 1: Compare values
    let value_order = p1.value.cmp(&p2.value);
    
    // Step 2: If values equal, compare elements
    match value_order {
        Ordering::Equal => {
            // Lexicographic element comparison (handled by Vec::cmp)
            p1.elements.cmp(&p2.elements)
        }
        other_order => other_order
    }
}

// Vec<T>::cmp implements lexicographic comparison:
// - Compare element-by-element from left to right
// - Stop at first differing element
// - If all compared elements equal, shorter < longer
```

### Lexicographic Element Comparison

**For `vec1.cmp(&vec2)` where both are `Vec<Pattern<V>>`:**

1. Compare lengths: `let (len1, len2) = (vec1.len(), vec2.len())`
2. For i in 0..min(len1, len2):
   - Compare `vec1[i].cmp(&vec2[i])`
   - If not Equal, return result
3. If all compared elements equal:
   - If `len1 < len2`: return `Ordering::Less`
   - If `len1 == len2`: return `Ordering::Equal`
   - If `len1 > len2`: return `Ordering::Greater`

**Properties**:
- **Short-circuit**: Stops at first differing element
- **Prefix comparison**: `[a, b]` < `[a, b, c]` (shorter < longer when prefix matches)
- **Recursive**: Element comparison uses same algorithm (value-first)

## Type Bounds and Constraints

### Compile-Time Constraints

```rust
// PartialOrd implementation requires:
impl<V: PartialOrd> PartialOrd for Pattern<V> { ... }
//      ^^^^^^^^^^^ Value type must be partially ordered

// Ord implementation requires:
impl<V: Ord> Ord for Pattern<V> { ... }
//      ^^^ Value type must be totally ordered

// Ord also requires (automatically satisfied):
impl<V: Eq + PartialOrd> Ord for Pattern<V> { ... }
//      ^^^^^^^^^^^^^^^ Already implemented in feature 004
```

**Compiler Errors** (expected):

```rust
// ❌ Cannot compare patterns with non-comparable values
let p1: Pattern<SomeUnorderableType> = ...;
let p2: Pattern<SomeUnorderableType> = ...;
p1.cmp(&p2);  // ERROR: SomeUnorderableType doesn't implement Ord
```

### Runtime Behavior

**No runtime checks needed**:
- Comparison is pure (no side effects)
- No panics or errors possible
- Type system ensures all operations are valid

## Performance Guarantees

### Time Complexity Guarantees

| Scenario | Complexity | Description |
|----------|------------|-------------|
| **Values differ** | O(1) | Immediate return after value comparison |
| **Values equal, early difference** | O(k) | k = index of first differing element |
| **Values equal, prefix match** | O(min(n1, n2)) | Compare all elements of shorter vector |
| **Worst case** | O(min(n1, n2) × d) | n = nodes, d = depth |

### Space Complexity Guarantees

- **Stack space**: O(d) where d = maximum pattern depth
- **Heap allocation**: O(0) - no allocations during comparison
- **Temporary storage**: None

### Performance Targets

- ✅ Sort 10,000 patterns: <200ms (depends on pattern size and hardware)
- ✅ Compare deep patterns (200+ levels): no stack overflow, <100ms
- ✅ Compare wide patterns (5,000+ elements): <500ms

## Error Handling

### No Errors

Comparison operations are **infallible**:
- Never panic
- Never return errors
- Always produce valid ordering

### Type Safety

The Rust type system prevents:
- ❌ Comparing patterns with non-comparable values (compile error)
- ❌ Mixing patterns of different value types (compile error)
- ❌ Using Ord on PartialOrd-only types (compile error)

## Behavioral Equivalence with gram-hs

### Haskell Type Signature

```haskell
compare :: Ord v => Pattern v -> Pattern v -> Ordering
```

### Rust Type Signature

```rust
fn cmp<V: Ord>(&self, other: &Self) -> Ordering
```

### Equivalence Table

| Aspect | Haskell | Rust | Equivalent? |
|--------|---------|------|-------------|
| **Type constraint** | `Ord v =>` | `V: Ord` | ✅ Yes |
| **Algorithm** | Value first, then list | Value first, then Vec | ✅ Yes |
| **Lexicographic** | Automatic for lists | Automatic for Vec | ✅ Yes |
| **Recursion** | Via list Ord | Via Vec Ord | ✅ Yes |
| **Short-circuit** | Yes | Yes | ✅ Yes |
| **Return type** | `Ordering` | `Ordering` | ✅ Yes |

## Summary

This API contract provides:
- ✅ Standard Rust trait implementations (PartialOrd, Ord)
- ✅ Behavioral equivalence with gram-hs reference implementation
- ✅ Integration with standard library (sorting, min/max, collections)
- ✅ Type-safe comparison (compile-time checks)
- ✅ Efficient implementation (short-circuit evaluation)
- ✅ No runtime errors or panics
- ✅ Clear complexity guarantees
- ✅ Support for ordered data structures (BTreeMap, BTreeSet, BinaryHeap)

