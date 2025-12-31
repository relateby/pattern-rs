# Data Model: Pattern Construction & Access

**Feature**: 005-basic-pattern-type  
**Date**: 2025-01-27

## Overview

This document defines the construction functions, accessor methods, and inspection utilities for the Pattern type. These functions operate on the existing `Pattern<V>` type defined in feature 004, adding convenient APIs for creating, accessing, and analyzing pattern instances. All functions match the gram-hs reference implementation in `../gram-hs/libs/pattern/src/Pattern/Core.hs`.

## Core Functions and Methods

### Pattern Construction Functions

Construction functions create new pattern instances. These are associated functions (no `self` parameter) that return new `Pattern<V>` instances.

#### `Pattern::point(value)`

Creates an atomic pattern (a pattern with no elements) from a value. This is the special case constructor for atomic patterns. Equivalent to gram-hs `point :: v -> Pattern v`.

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn point(value: V) -> Self
}
```

**Parameters**:
- `value: V` - The value component of the pattern

**Returns**: `Pattern<V>` - A new atomic pattern instance

**Characteristics**:
- Generic over value type `V`
- Creates pattern with specified value and empty elements
- Special case constructor for atomic patterns
- O(1) operation

**Usage**:
```rust
let atomic = Pattern::point("hello".to_string());
```

#### `Pattern::pattern(value, elements)`

Creates a pattern with a value and elements. This is the primary constructor for creating patterns. Takes a decoration value and a list of pattern elements. The elements form the pattern itself; the value provides decoration about that pattern. Equivalent to gram-hs `pattern :: v -> [Pattern v] -> Pattern v`.

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn pattern(value: V, elements: Vec<Pattern<V>>) -> Self
}
```

**Parameters**:
- `value: V` - The value component of the pattern
- `elements: Vec<Pattern<V>>` - The nested collection of patterns

**Returns**: `Pattern<V>` - A new pattern instance

**Characteristics**:
- Generic over value type `V`
- Creates pattern with specified value and elements
- Primary constructor for patterns
- No validation needed (Pattern structure is always valid)
- O(1) operation (just struct construction)

**Usage**:
```rust
let pattern = Pattern::pattern("parent".to_string(), vec![
    Pattern::point("child1".to_string()),
    Pattern::point("child2".to_string()),
]);
```

#### `Pattern::from_list(value, values)`

Creates a pattern from a list of values. The first argument is the decoration value, and the list of values are converted to atomic patterns and used as elements. Equivalent to gram-hs `fromList :: v -> [v] -> Pattern v`.

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn from_list(value: V, values: Vec<V>) -> Self
}
```

**Parameters**:
- `value: V` - The decoration value for the pattern
- `values: Vec<V>` - List of values to convert to atomic patterns as elements

**Returns**: `Pattern<V>` - A new pattern instance with value as decoration and values converted to atomic patterns as elements

**Characteristics**:
- Generic over value type `V`
- Converts each value in the list to an atomic pattern
- O(n) operation where n is the length of values list

**Usage**:
```rust
let pattern = Pattern::from_list("root".to_string(), vec![
    "a".to_string(),
    "b".to_string(),
    "c".to_string(),
]);
// Equivalent to:
// Pattern::pattern("root".to_string(), vec![
//     Pattern::point("a".to_string()),
//     Pattern::point("b".to_string()),
//     Pattern::point("c".to_string()),
// ])
```

### Pattern Accessor Methods

Accessor methods retrieve pattern components. These are methods (have `self` parameter) that return references to pattern fields. In gram-hs, these are record field accessors; in Rust, we provide methods for consistency.

#### `pattern.value()`

Returns a reference to the pattern's value component. Equivalent to gram-hs `value :: Pattern v -> v` (field accessor).

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn value(&self) -> &V
}
```

**Returns**: `&V` - Reference to the pattern's value

**Characteristics**:
- Returns immutable reference
- O(1) operation
- Preserves type information through generics

**Usage**:
```rust
let pattern = Pattern::point("hello".to_string());
let value = pattern.value(); // &String
```

#### `pattern.elements()`

Returns a slice of the pattern's elements. Equivalent to gram-hs `elements :: Pattern v -> [Pattern v]` (field accessor).

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn elements(&self) -> &[Pattern<V>]
}
```

**Returns**: `&[Pattern<V>]` - Slice of pattern elements

**Characteristics**:
- Returns immutable reference to element slice
- O(1) operation
- Allows iteration and indexing of elements

**Usage**:
```rust
let pattern = Pattern::pattern("parent".to_string(), vec![
    Pattern::point("child1".to_string()),
    Pattern::point("child2".to_string()),
]);
let elements = pattern.elements(); // &[Pattern<String>]
```

### Pattern Inspection Utilities

Inspection utilities analyze pattern structure and provide information about pattern characteristics. These are methods that perform structural analysis.

#### `pattern.length()`

Returns the number of direct elements in a pattern's sequence. Equivalent to gram-hs `length :: Pattern v -> Int`.

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn length(&self) -> usize
}
```

**Returns**: `usize` - Number of direct elements

**Characteristics**:
- Counts only direct elements (not nested)
- O(1) operation
- Equivalent to `pattern.elements().len()`

**Usage**:
```rust
let pattern = Pattern::pattern("parent".to_string(), vec![
    Pattern::point("child1".to_string()),
    Pattern::point("child2".to_string()),
]);
assert_eq!(pattern.length(), 2);
```

#### `pattern.size()`

Returns the total number of nodes in a pattern structure, including all nested patterns. Equivalent to gram-hs `size :: Pattern v -> Int`.

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn size(&self) -> usize
}
```

**Returns**: `usize` - Total number of nodes (root + all nested nodes)

**Characteristics**:
- Recursively counts all nodes in the pattern structure
- O(n) operation where n is total number of nodes
- Counts the root node plus all nodes in all nested subpatterns

**Usage**:
```rust
let atomic = Pattern::point("atom".to_string());
assert_eq!(atomic.size(), 1);

let pattern = Pattern::pattern("root".to_string(), vec![
    Pattern::point("child1".to_string()),
    Pattern::point("child2".to_string()),
]);
assert_eq!(pattern.size(), 3); // root + 2 children
```

#### `pattern.depth()`

Returns the maximum nesting depth of a pattern structure. Equivalent to gram-hs `depth :: Pattern v -> Int`.

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn depth(&self) -> usize
}
```

**Returns**: `usize` - Maximum nesting depth (0 for atomic patterns)

**Characteristics**:
- Recursively calculates maximum depth
- O(n) operation where n is total number of nodes
- Must handle deep nesting safely (at least 100 levels)
- **Important**: Atomic patterns have depth 0

**Usage**:
```rust
let atomic = Pattern::point("hello".to_string());
assert_eq!(atomic.depth(), 0); // Atomic patterns have depth 0

let nested = Pattern::pattern("parent".to_string(), vec![
    Pattern::pattern("child".to_string(), vec![
        Pattern::point("grandchild".to_string()),
    ]),
]);
assert_eq!(nested.depth(), 2); // parent (0) -> child (1) -> grandchild (2)
```

#### `pattern.is_atomic()`

Checks if a pattern is atomic (has no elements). This is a convenience helper not present in gram-hs but useful for pattern classification.

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn is_atomic(&self) -> bool
}
```

**Returns**: `bool` - `true` if pattern has no elements, `false` otherwise

**Characteristics**:
- Simple check of element count
- O(1) operation
- Equivalent to `pattern.length() == 0` or `pattern.elements().is_empty()`
- Useful for pattern classification

**Usage**:
```rust
let atomic = Pattern::point("hello".to_string());
assert!(atomic.is_atomic());

let nested = Pattern::pattern("parent".to_string(), vec![
    Pattern::point("child".to_string()),
]);
assert!(!nested.is_atomic());
```

## Relationships

### Construction → Access

Construction functions create patterns that can be accessed:
- `Pattern::point()`, `Pattern::pattern()`, and `Pattern::from_list()` create patterns
- `pattern.value()` and `pattern.elements()` access the created pattern's components

### Access → Inspection

Accessors provide data for inspection:
- `pattern.elements()` provides elements for depth and size calculation
- `pattern.elements().len()` is equivalent to `pattern.length()`
- `pattern.elements().is_empty()` is equivalent to `pattern.is_atomic()`

### Inspection Dependencies

Inspection utilities may depend on each other:
- `length()` is independent (just returns length)
- `is_atomic()` is independent (just checks element count)
- `size()` recursively calls itself on nested elements
- `depth()` recursively calls itself on nested elements

## Validation Rules

- **No validation needed**: Pattern structure is always valid (any value + any element collection is valid)
- **Type safety**: All functions preserve generic type parameter `V`
- **Performance**: Construction and access must be O(1), size and depth must handle 100+ levels safely
- **Immutability**: Accessors return immutable references (no mutation through accessors)
- **Behavioral equivalence**: All functions must match gram-hs behavior exactly

## State Transitions

N/A - These functions operate on immutable pattern instances. No state transitions involved.

## Constraints

- All functions must work generically with any value type `V` that Pattern supports
- Functions must maintain behavioral equivalence with gram-hs reference implementation
- Functions must compile for `wasm32-unknown-unknown` target
- Depth and size calculations must handle at least 100 nesting levels without stack overflow
- Functions must be efficient (O(1) for construction/access/length, O(n) for size/depth is acceptable)
