# API Contracts: Pattern Combination Operations

**Feature**: 013-semigroup-instance  
**Date**: 2026-01-04

## Overview

This document defines the type signatures and API contracts for pattern combination operations. All types and methods must be implemented exactly as specified to ensure correct behavior and maintain compatibility with the gram-hs reference implementation semantics.

## Core Trait: Combinable

### Definition

```rust
/// Types that support associative combination.
///
/// Implementors must ensure that combination is associative:
/// `(a.combine(b)).combine(c)` must equal `a.combine(b.combine(c))` for all values.
///
/// This trait is used to enable pattern combination for `Pattern<V>` where `V: Combinable`.
///
/// # Laws
///
/// **Associativity**: For all values a, b, c of type Self:
/// ```text
/// (a.combine(b)).combine(c) == a.combine(b.combine(c))
/// ```
///
/// # Examples
///
/// ```rust
/// use pattern_core::Combinable;
///
/// impl Combinable for String {
///     fn combine(mut self, other: Self) -> Self {
///         self.push_str(&other);
///         self
///     }
/// }
///
/// let s1 = String::from("hello");
/// let s2 = String::from(" world");
/// let result = s1.combine(s2);
/// assert_eq!(result, "hello world");
/// ```
pub trait Combinable {
    /// Combines two values associatively.
    ///
    /// # Parameters
    ///
    /// * `self` - The first value (consumed)
    /// * `other` - The second value to combine with (consumed)
    ///
    /// # Returns
    ///
    /// A new value representing the combination of `self` and `other`.
    ///
    /// # Laws
    ///
    /// Must be associative: `(a.combine(b)).combine(c) == a.combine(b.combine(c))`
    fn combine(self, other: Self) -> Self;
}
```

### Type Parameters

- `Self`: The type implementing Combinable (no additional constraints)

### Method: combine

**Signature**: `fn combine(self, other: Self) -> Self`

**Ownership**: Takes ownership of both `self` and `other` (moves, not borrows)

**Semantics**: Creates a new value by combining the two input values

**Requirements**:
- Must be associative
- Must consume both inputs (no borrowing)
- Must return a new value (no mutation of external state)

**Complexity**: Depends on implementing type (document in impl)

## Pattern Combination Method

### Definition

```rust
impl<V: Combinable> Pattern<V> {
    /// Combines two patterns associatively.
    ///
    /// Creates a new pattern by:
    /// 1. Combining the values using `V::combine`
    /// 2. Concatenating the element vectors (left first, then right)
    ///
    /// The operation is associative: `(a.combine(b)).combine(c)` equals `a.combine(b.combine(c))`.
    ///
    /// # Parameters
    ///
    /// * `self` - The first pattern (consumed)
    /// * `other` - The second pattern to combine with (consumed)
    ///
    /// # Returns
    ///
    /// A new `Pattern<V>` with:
    /// * `value`: Result of `self.value.combine(other.value)`
    /// * `elements`: Concatenation of `self.elements` and `other.elements`
    ///
    /// # Examples
    ///
    /// ## Atomic Patterns
    ///
    /// ```rust
    /// use pattern_core::Pattern;
    ///
    /// let p1 = Pattern::point("hello");
    /// let p2 = Pattern::point(" world");
    /// let result = p1.combine(p2);
    ///
    /// assert_eq!(result.value(), "hello world");
    /// assert_eq!(result.length(), 0);  // No elements
    /// ```
    ///
    /// ## Patterns with Elements
    ///
    /// ```rust
    /// use pattern_core::Pattern;
    ///
    /// let p1 = Pattern::pattern("a", vec![
    ///     Pattern::point("b"),
    ///     Pattern::point("c"),
    /// ]);
    ///
    /// let p2 = Pattern::pattern("d", vec![
    ///     Pattern::point("e"),
    /// ]);
    ///
    /// let result = p1.combine(p2);
    ///
    /// assert_eq!(result.value(), "ad");
    /// assert_eq!(result.length(), 3);  // [b, c, e]
    /// ```
    ///
    /// ## Associativity
    ///
    /// ```rust
    /// use pattern_core::Pattern;
    ///
    /// let a = Pattern::point(1);
    /// let b = Pattern::point(2);
    /// let c = Pattern::point(3);
    ///
    /// let left = a.clone().combine(b.clone()).combine(c.clone());
    /// let right = a.combine(b.combine(c));
    ///
    /// assert_eq!(left, right);  // Associativity holds
    /// ```
    ///
    /// # Performance
    ///
    /// * Time: O(|elements1| + |elements2| + value_combine_cost)
    /// * Space: O(|elements1| + |elements2|)
    ///
    /// Element concatenation uses `Vec::extend` for efficiency.
    pub fn combine(self, other: Self) -> Self {
        let combined_value = self.value.combine(other.value);
        
        let mut combined_elements = self.elements;
        combined_elements.extend(other.elements);
        
        Pattern {
            value: combined_value,
            elements: combined_elements,
        }
    }
}
```

### Type Parameters

- `V: Combinable`: The value type must implement Combinable trait

### Method: combine

**Signature**: `pub fn combine(self, other: Self) -> Self`

**Constraints**: `V: Combinable`

**Ownership**: Takes ownership of both patterns (moves)

**Semantics**:
1. Combines values: `v1.combine(v2)`
2. Concatenates elements: `elements1 ++ elements2`
3. Returns new pattern

**Requirements**:
- Must be associative (inherited from V's associativity and list concatenation)
- Must preserve element order (left elements first, then right elements)
- Must create a well-formed Pattern

**Complexity**:
- Time: O(|elements1| + |elements2| + cost(V::combine))
- Space: O(|elements1| + |elements2|)

## Standard Implementations

### String

```rust
impl Combinable for String {
    /// Combines two strings by concatenation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pattern_core::Combinable;
    ///
    /// let s1 = String::from("hello");
    /// let s2 = String::from(" world");
    /// let result = s1.combine(s2);
    /// assert_eq!(result, "hello world");
    /// ```
    fn combine(mut self, other: Self) -> Self {
        self.push_str(&other);
        self
    }
}
```

**Associativity**: String concatenation is associative ✓

**Complexity**: O(|other|)

### Vec<T>

```rust
impl<T> Combinable for Vec<T> {
    /// Combines two vectors by concatenation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pattern_core::Combinable;
    ///
    /// let v1 = vec![1, 2, 3];
    /// let v2 = vec![4, 5];
    /// let result = v1.combine(v2);
    /// assert_eq!(result, vec![1, 2, 3, 4, 5]);
    /// ```
    fn combine(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}
```

**Associativity**: Vector concatenation is associative ✓

**Complexity**: O(|other|)

### Unit Type

```rust
impl Combinable for () {
    /// Combines two unit values (trivial).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pattern_core::Combinable;
    ///
    /// let u1 = ();
    /// let u2 = ();
    /// let result = u1.combine(u2);
    /// assert_eq!(result, ());
    /// ```
    fn combine(self, _other: Self) -> Self {
        ()
    }
}
```

**Associativity**: Trivial combination is associative ✓

**Complexity**: O(1)

## Iterator Extension (Optional Enhancement)

### Method: combine_all

```rust
/// Extension trait for iterators of combinable items
pub trait CombineIterator: Iterator {
    /// Combines all items in the iterator using the Combinable trait.
    ///
    /// Returns `None` if the iterator is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pattern_core::{Pattern, CombineIterator};
    ///
    /// let patterns = vec![
    ///     Pattern::point("a"),
    ///     Pattern::point("b"),
    ///     Pattern::point("c"),
    /// ];
    ///
    /// let result = patterns.into_iter().combine_all();
    /// assert_eq!(result.unwrap().value(), "abc");
    /// ```
    fn combine_all(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Combinable,
    {
        self.reduce(|acc, item| acc.combine(item))
    }
}

impl<I: Iterator> CombineIterator for I {}
```

**Note**: This is a convenience extension. The core functionality is the `combine()` method on Pattern.

## Properties and Invariants

### Associativity Law

**Property**: For all patterns `a`, `b`, `c`:
```rust
(a.combine(b)).combine(c) == a.combine(b.combine(c))
```

**Verification**: Property-based testing with proptest

### Element Order Preservation

**Property**: For all patterns `p1`, `p2`:
```rust
let result = p1.clone().combine(p2.clone());
assert_eq!(result.elements[0..p1.length()], p1.elements);
assert_eq!(result.elements[p1.length()..], p2.elements);
```

### Value Combination Delegation

**Property**: For all patterns `p1`, `p2`:
```rust
let result = p1.clone().combine(p2.clone());
assert_eq!(result.value, p1.value.combine(p2.value));
```

### Structural Validity

**Property**: For all patterns `p1`, `p2`:
```rust
let result = p1.combine(p2);
// result is always a well-formed Pattern
assert!(result.validate(&ValidationRules::default()).is_ok());
```

## Error Handling

**No Error Cases**: The combine operation cannot fail. It always produces a valid Pattern.

**Panics**: None. The operation is total (defined for all valid inputs).

**Type Safety**: Combination is only available when `V: Combinable`, enforced at compile time.

## Testing Contracts

### Unit Tests

Tests must cover:
1. Atomic pattern combination (no elements)
2. Patterns with elements
3. Mixed structures (one atomic, one with elements)
4. Deep nesting (100+ levels)
5. Wide patterns (1000+ elements)

### Property Tests

Tests must verify:
1. **Associativity**: `(a⊕b)⊕c == a⊕(b⊕c)` for all a, b, c
2. **Element preservation**: Combined pattern has all elements from both inputs
3. **Element order**: Left pattern elements come before right pattern elements
4. **Value combination**: Result value equals `v1.combine(v2)`

### Equivalence Tests

Tests must verify:
- Behavior matches gram-hs Semigroup instance
- Test cases ported from gram-hs pass identically

### Performance Tests

Benchmarks must verify:
- Combine 1000-element patterns in <1ms
- Combine deep patterns (100+ levels) without stack overflow
- Fold 100 patterns in <100ms

## Compatibility

### With Existing Pattern Operations

- ✅ **map**: Can combine then map, or map then combine
- ✅ **fold**: Can use combine as the fold operation
- ✅ **traverse**: Can combine patterns after traversal
- ✅ **filter**: Can filter then combine remaining patterns

### With Standard Library

- ✅ **Iterator::reduce**: Works with combine method
- ✅ **Clone**: Both inputs are moved, but can clone before combining
- ✅ **PartialEq/Eq**: Can test equality of combined patterns

## Version Compatibility

- **Minimum Rust Version**: 1.70.0 (workspace MSRV)
- **Edition**: 2021
- **Breaking Changes**: None (new functionality, no existing API changes)

## Summary

### Core API

- `Combinable` trait: Binary associative combination for types
- `Pattern::combine()`: Combines two patterns (value + element concatenation)
- Standard implementations: String, Vec<T>, ()

### Properties

- Associativity guaranteed
- Element order preserved
- Type-safe (compile-time enforcement)
- No panics, no errors

### Complexity

- Time: O(n) where n = total element count
- Space: O(n) for result pattern

