# Type Signatures: Pattern Construction & Access

**Feature**: 005-basic-pattern-type  
**Date**: 2025-01-27

## Overview

This document defines the public API type signatures for pattern construction functions, accessor methods, and inspection utilities. These serve as the contracts that define the interface users will interact with. All signatures match the gram-hs reference implementation in `../gram-hs/libs/pattern/src/Pattern/Core.hs`.

## Core Module: pattern_core

### Pattern Construction Functions

#### `Pattern::point`

Creates an atomic pattern (a pattern with no elements) from a value. This is the special case constructor for atomic patterns. Equivalent to gram-hs `point :: v -> Pattern v`.

```rust
impl<V> Pattern<V> {
    /// Creates an atomic pattern (a pattern with no elements) from a value.
    ///
    /// This is a convenience constructor for creating simple patterns.
    /// Equivalent to gram-hs `point :: v -> Pattern v`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value component of the pattern
    ///
    /// # Returns
    ///
    /// A new atomic `Pattern<V>` instance with the specified value and empty elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let atomic = Pattern::point("atom".to_string());
    /// assert_eq!(atomic.value(), "atom");
    /// assert!(atomic.is_atomic());
    /// ```
    pub fn point(value: V) -> Self;
}
```

#### `Pattern::pattern`

Creates a pattern with a value and elements. This is the primary constructor for creating patterns. Equivalent to gram-hs `pattern :: v -> [Pattern v] -> Pattern v`.

```rust
impl<V> Pattern<V> {
    /// Creates a pattern with a value and elements.
    ///
    /// This is the primary constructor for creating patterns. Takes a decoration value
    /// and a list of pattern elements. The elements form the pattern itself; the value
    /// provides decoration about that pattern.
    ///
    /// Equivalent to gram-hs `pattern :: v -> [Pattern v] -> Pattern v`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value component of the pattern
    /// * `elements` - The nested collection of patterns
    ///
    /// # Returns
    ///
    /// A new `Pattern<V>` instance with the specified value and elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("root".to_string(), vec![
    ///     Pattern::point("child".to_string()),
    /// ]);
    /// assert_eq!(pattern.value(), "root");
    /// assert_eq!(pattern.length(), 1);
    /// ```
    pub fn pattern(value: V, elements: Vec<Pattern<V>>) -> Self;
}
```

#### `Pattern::from_list`

Creates a pattern from a list of values. Equivalent to gram-hs `fromList :: v -> [v] -> Pattern v`.

```rust
impl<V> Pattern<V> {
    /// Creates a pattern from a list of values.
    ///
    /// Creates a pattern where the first argument is the decoration value,
    /// and the list of values are converted to atomic patterns and used as elements.
    /// Equivalent to gram-hs `fromList :: v -> [v] -> Pattern v`.
    ///
    /// # Arguments
    ///
    /// * `value` - The decoration value for the pattern
    /// * `values` - List of values to convert to atomic patterns as elements
    ///
    /// # Returns
    ///
    /// A new `Pattern<V>` instance with value as decoration and values converted to atomic patterns as elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::from_list("root".to_string(), vec![
    ///     "a".to_string(),
    ///     "b".to_string(),
    ///     "c".to_string(),
    /// ]);
    /// assert_eq!(pattern.value(), "root");
    /// assert_eq!(pattern.length(), 3);
    /// ```
    pub fn from_list(value: V, values: Vec<V>) -> Self;
}
```

### Pattern Accessor Methods

#### `Pattern::value`

Returns a reference to the pattern's value component. Equivalent to gram-hs `value :: Pattern v -> v` (field accessor).

```rust
impl<V> Pattern<V> {
    /// Returns a reference to the pattern's value component.
    ///
    /// Equivalent to gram-hs `value :: Pattern v -> v` (record field accessor).
    ///
    /// # Returns
    ///
    /// An immutable reference to the pattern's value.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::point("hello".to_string());
    /// let value = pattern.value(); // &String
    /// assert_eq!(value, "hello");
    /// ```
    pub fn value(&self) -> &V;
}
```

#### `Pattern::elements`

Returns a slice of the pattern's elements. Equivalent to gram-hs `elements :: Pattern v -> [Pattern v]` (field accessor).

```rust
impl<V> Pattern<V> {
    /// Returns a slice of the pattern's elements.
    ///
    /// Equivalent to gram-hs `elements :: Pattern v -> [Pattern v]` (record field accessor).
    ///
    /// # Returns
    ///
    /// An immutable slice of the pattern's nested elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("parent".to_string(), vec![
    ///     Pattern::point("child1".to_string()),
    ///     Pattern::point("child2".to_string()),
    /// ]);
    /// let elements = pattern.elements();
    /// assert_eq!(elements.len(), 2);
    /// ```
    pub fn elements(&self) -> &[Pattern<V>];
}
```

### Pattern Inspection Utilities

#### `Pattern::length`

Returns the number of direct elements in a pattern's sequence. Equivalent to gram-hs `length :: Pattern v -> Int`.

```rust
impl<V> Pattern<V> {
    /// Returns the number of direct elements in a pattern's sequence.
    ///
    /// This operation is O(1).
    /// Equivalent to gram-hs `length :: Pattern v -> Int`.
    ///
    /// # Returns
    ///
    /// The number of direct elements in the pattern.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let atomic = Pattern::point("atom".to_string());
    /// assert_eq!(atomic.length(), 0);
    ///
    /// let pattern = Pattern::pattern("pair".to_string(), vec![
    ///     Pattern::point(1),
    ///     Pattern::point(2),
    /// ]);
    /// assert_eq!(pattern.length(), 2);
    /// ```
    pub fn length(&self) -> usize;
}
```

#### `Pattern::size`

Returns the total number of nodes in a pattern structure. Equivalent to gram-hs `size :: Pattern v -> Int`.

```rust
impl<V> Pattern<V> {
    /// Returns the total number of nodes in a pattern structure.
    ///
    /// Counts the root node plus all nodes in all nested subpatterns.
    /// This operation is O(n) where n is the total number of nodes.
    /// Equivalent to gram-hs `size :: Pattern v -> Int`.
    ///
    /// # Returns
    ///
    /// The total number of nodes in the pattern structure.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let atomic = Pattern::point("atom".to_string());
    /// assert_eq!(atomic.size(), 1);
    ///
    /// let pattern = Pattern::pattern("root".to_string(), vec![
    ///     Pattern::point("child".to_string()),
    /// ]);
    /// assert_eq!(pattern.size(), 2);
    ///
    /// let nested = Pattern::pattern("root".to_string(), vec![
    ///     Pattern::point("a".to_string()),
    ///     Pattern::point("b".to_string()),
    /// ]);
    /// assert_eq!(nested.size(), 3);
    /// ```
    pub fn size(&self) -> usize;
}
```

#### `Pattern::depth`

Returns the maximum nesting depth of a pattern structure. Equivalent to gram-hs `depth :: Pattern v -> Int`.

```rust
impl<V> Pattern<V> {
    /// Returns the maximum nesting depth of a pattern structure.
    ///
    /// An atomic pattern has depth 0.
    /// A pattern with elements has depth 1 + max depth of elements.
    /// This operation is O(n) where n is the total number of nodes.
    /// Equivalent to gram-hs `depth :: Pattern v -> Int`.
    ///
    /// # Returns
    ///
    /// The maximum nesting depth of the pattern.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let atomic = Pattern::point("atom".to_string());
    /// assert_eq!(atomic.depth(), 0); // Atomic patterns have depth 0
    ///
    /// let pattern = Pattern::pattern("root".to_string(), vec![
    ///     Pattern::point("child".to_string()),
    /// ]);
    /// assert_eq!(pattern.depth(), 1);
    ///
    /// let nested = Pattern::pattern("root".to_string(), vec![
    ///     Pattern::pattern("middle".to_string(), vec![
    ///         Pattern::point("inner".to_string()),
    ///     ]),
    /// ]);
    /// assert_eq!(nested.depth(), 2);
    /// ```
    pub fn depth(&self) -> usize;
}
```

#### `Pattern::is_atomic`

Checks if a pattern is atomic (has no elements). This is a convenience helper not present in gram-hs but useful for pattern classification.

```rust
impl<V> Pattern<V> {
    /// Checks if the pattern is atomic (has no elements).
    ///
    /// This is a convenience helper not present in gram-hs but useful for pattern classification.
    /// Equivalent to `pattern.length() == 0` or `pattern.elements().is_empty()`.
    ///
    /// # Returns
    ///
    /// `true` if the pattern has no elements, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let atomic = Pattern::point("hello".to_string());
    /// assert!(atomic.is_atomic());
    ///
    /// let nested = Pattern::pattern("parent".to_string(), vec![
    ///     Pattern::point("child".to_string()),
    /// ]);
    /// assert!(!nested.is_atomic());
    /// ```
    pub fn is_atomic(&self) -> bool;
}
```

## Type Constraints

All functions are generic over value type `V` with no trait bounds (beyond what `Pattern<V>` itself requires). The functions work with any value type that can be used with `Pattern<V>`.

## Error Handling

No functions return `Result` types. All operations are infallible:
- Construction functions always succeed (Pattern structure is always valid)
- Accessor methods always succeed (fields always exist)
- Inspection utilities always succeed (structural analysis is always possible)

## Performance Characteristics

- **Construction**: 
  - `point()`: O(1)
  - `pattern()`: O(1)
  - `from_list()`: O(n) where n is length of values list
- **Accessors**: O(1) for both `value()` and `elements()`
- **Inspection**: 
  - `length()`: O(1)
  - `is_atomic()`: O(1)
  - `size()`: O(n) where n is total number of nodes (must handle 100+ levels safely)
  - `depth()`: O(n) where n is total number of nodes (must handle 100+ levels safely)

## WASM Compatibility

All functions are compatible with WebAssembly targets. They use only standard library types and operations that compile to WASM.

## Behavioral Equivalence

All functions must maintain behavioral equivalence with the gram-hs reference implementation:
- Construction functions must create patterns with identical structure
- Accessors must return identical values
- Inspection utilities must return identical results (note: `depth()` returns 0 for atomic patterns)

Reference implementation: `../gram-hs/libs/pattern/src/Pattern/Core.hs`
