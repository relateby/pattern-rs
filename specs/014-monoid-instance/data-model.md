# Data Model: Pattern Identity Element

**Feature**: 014-monoid-instance  
**Created**: 2026-01-05

## Overview

This feature adds identity element support to patterns through the `Default` trait, completing the monoid algebraic structure for Pattern types. The data model focuses on the relationship between patterns, default values, and combination operations.

## Key Concepts

### Monoid Structure

A **Monoid** is an algebraic structure consisting of:
1. A set of values (here: `Pattern<V>` for some value type `V`)
2. An associative binary operation (here: `combine()` from feature 013)
3. An identity element (here: `Pattern::default()`)

The identity element must satisfy:
- **Left Identity**: `identity ⊕ x = x`
- **Right Identity**: `x ⊕ identity = x`

### Pattern Structure

```
Pattern<V>
├─ value: V              // The value component
└─ elements: Vec<Pattern<V>>  // The nested elements (possibly empty)
```

### Identity Pattern

The **identity pattern** (or "default pattern") is defined as:
- Value: `V::default()` (the default value for type V)
- Elements: `[]` (empty vector)

Example for `Pattern<String>`:
```
Pattern {
    value: "",           // String::default()
    elements: vec![]     // Empty
}
```

Example for `Pattern<Vec<i32>>`:
```
Pattern {
    value: vec![],       // Vec::default()
    elements: vec![]     // Empty
}
```

## Entities & Relationships

### Pattern<V> + Default

```
┌─────────────────────────────────┐
│      Pattern<V>                 │
│  where V: Default + Combinable  │
├─────────────────────────────────┤
│ + default() -> Pattern<V>       │
│ + combine(Pattern<V>) -> Self   │
└─────────────────────────────────┘
```

**Relationship**: When `V` implements both `Default` and `Combinable`, `Pattern<V>` can implement `Default` and satisfy monoid laws.

### Monoid Laws

```
Left Identity Law:
  Pattern::default().combine(p) == p
  
Right Identity Law:
  p.combine(Pattern::default()) == p
  
Combined with Associativity (from feature 013):
  (a.combine(b)).combine(c) == a.combine(b.combine(c))
```

### Value Type Requirements

For `Pattern<V>` to form a complete monoid:

1. **V must implement Default**
   - Provides the value component for the identity pattern
   - Must be an identity for V's combination: `V::default().combine(v) == v`

2. **V must implement Combinable** (from feature 013)
   - Provides associative combination of values
   - Example: `String` concatenation, `Vec<T>` concatenation

### Standard Library Integration

```
┌──────────────────────┐
│  std::default        │
│     ::Default        │
└──────────────────────┘
          △
          │ implements
          │
┌──────────────────────┐      ┌──────────────────────┐
│    Pattern<V>        │──────│   Combinable         │
│  where V: Default    │ uses │   (feature 013)      │
└──────────────────────┘      └──────────────────────┘
```

## Examples

### String Pattern Monoid

```rust
// Identity element
let empty = Pattern::<String>::default();
// Pattern { value: "", elements: vec![] }

// Left identity: empty.combine(p) == p
let p = Pattern::point("hello".to_string());
assert_eq!(empty.clone().combine(p.clone()), p);

// Right identity: p.combine(empty) == p
assert_eq!(p.clone().combine(empty.clone()), p);
```

### Vec Pattern Monoid

```rust
// Identity element
let empty = Pattern::<Vec<i32>>::default();
// Pattern { value: vec![], elements: vec![] }

// Left identity
let p = Pattern::point(vec![1, 2, 3]);
assert_eq!(empty.clone().combine(p.clone()), p);

// Right identity
assert_eq!(p.clone().combine(empty.clone()), p);
```

### Unit Pattern Monoid

```rust
// Identity element
let empty = Pattern::<()>::default();
// Pattern { value: (), elements: vec![] }

// Trivial monoid (all values are identity)
let p = Pattern::point(());
assert_eq!(empty.clone().combine(p.clone()), p);
assert_eq!(p.clone().combine(empty.clone()), p);
```

## Usage with Iterators

The identity element enables clean iterator patterns:

```rust
// fold with explicit identity
let patterns = vec![p1, p2, p3];
let result = patterns.into_iter()
    .fold(Pattern::default(), |acc, p| acc.combine(p));

// Handles empty collections naturally
let empty_vec: Vec<Pattern<String>> = vec![];
let result = empty_vec.into_iter()
    .fold(Pattern::default(), |acc, p| acc.combine(p));
// result == Pattern::default()
```

## Invariants

1. **Default is well-formed**: `Pattern::default()` is always a valid pattern
2. **Identity laws hold**: For all patterns `p`:
   - `Pattern::default().combine(p) == p`
   - `p.combine(Pattern::default()) == p`
3. **Value-element relationship**: Default pattern has default value and empty elements
4. **Consistency**: Combining any pattern with default doesn't change the pattern's structure or values

## Type Constraints

```rust
impl<V> Default for Pattern<V>
where
    V: Default
```

The `Pattern<V>` can implement `Default` only when the value type `V` implements `Default`. This ensures the value component can be constructed with a default value.

For full monoid behavior:
```rust
where
    V: Default + Combinable
```

Both traits are required for patterns to form a complete monoid with combination and identity.

