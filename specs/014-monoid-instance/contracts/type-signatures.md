# Type Signatures: Pattern Identity Element via Default Trait

**Feature**: 014-monoid-instance  
**Created**: 2026-01-05

## Overview

This document defines the type signatures and trait implementations for adding identity element support to patterns via the `Default` trait. These signatures are implementation-agnostic but show the expected API.

---

## Default Trait Implementation

### Pattern Default

```rust
impl<V> Default for Pattern<V>
where
    V: Default
{
    fn default() -> Self
}
```

**Description**: Creates a default (identity) pattern with the default value and empty elements.

**Type Constraints**:
- `V: Default` - The value type must have a default value

**Returns**: A pattern with `V::default()` as the value and an empty elements vector

**Semantics**:
- Result: `Pattern { value: V::default(), elements: vec![] }`
- The returned pattern acts as an identity element for combination operations

**Examples**:
```rust
// String pattern
let empty: Pattern<String> = Pattern::default();
// Pattern { value: "", elements: vec![] }

// Vec pattern  
let empty: Pattern<Vec<i32>> = Pattern::default();
// Pattern { value: vec![], elements: vec![] }

// Unit pattern
let empty: Pattern<()> = Pattern::default();
// Pattern { value: (), elements: vec![] }
```

---

## Monoid Laws

The `Default` implementation must satisfy these mathematical properties when combined with the `Combinable` trait:

### Left Identity Law

```rust
∀ p: Pattern<V>, Pattern::default().combine(p) == p
```

**Description**: Combining the identity pattern on the left with any pattern yields that pattern unchanged.

**Type Constraints**: `V: Default + Combinable`

**Property Test Signature**:
```rust
fn left_identity_law<V>(p: Pattern<V>) -> bool
where
    V: Default + Combinable + Eq + Clone
{
    let empty = Pattern::<V>::default();
    empty.combine(p.clone()) == p
}
```

### Right Identity Law

```rust
∀ p: Pattern<V>, p.combine(Pattern::default()) == p
```

**Description**: Combining any pattern with the identity pattern on the right yields that pattern unchanged.

**Type Constraints**: `V: Default + Combinable`

**Property Test Signature**:
```rust
fn right_identity_law<V>(p: Pattern<V>) -> bool
where
    V: Default + Combinable + Eq + Clone
{
    let empty = Pattern::<V>::default();
    p.clone().combine(empty) == p
}
```

---

## Complete Monoid Trait Bounds

For a pattern to form a complete monoid (associative operation + identity):

```rust
Pattern<V>
where
    V: Default + Combinable + Eq + Clone
```

**Trait Requirements**:
- `Default`: Provides the identity value
- `Combinable`: Provides associative combination (from feature 013)
- `Eq`: Required for testing equality in monoid laws
- `Clone`: Required for non-consuming comparison operations

---

## Integration with Existing APIs

### With Iterator Methods

```rust
// fold with default as initial value
fn fold_patterns<V>(patterns: Vec<Pattern<V>>) -> Pattern<V>
where
    V: Default + Combinable
{
    patterns.into_iter().fold(Pattern::default(), |acc, p| acc.combine(p))
}

// reduce with default fallback
fn reduce_or_default<V>(patterns: Vec<Pattern<V>>) -> Pattern<V>
where
    V: Default + Combinable
{
    patterns.into_iter()
        .reduce(|acc, p| acc.combine(p))
        .unwrap_or_else(Pattern::default)
}
```

### With Standard Library Functions

```rust
// mem::take uses Default
use std::mem;

fn take_pattern<V>(p: &mut Pattern<V>) -> Pattern<V>
where
    V: Default
{
    mem::take(p)  // Replaces p with Pattern::default(), returns old value
}

// Option::unwrap_or_default
fn get_or_default<V>(opt: Option<Pattern<V>>) -> Pattern<V>
where
    V: Default
{
    opt.unwrap_or_default()  // Uses Pattern::default() if None
}
```

---

## Value Type Implementations

### Common Value Types

The following common types already implement `Default` and can be used with patterns:

```rust
// String: Default is empty string
impl Default for Pattern<String>
// Pattern::default() == Pattern { value: "", elements: vec![] }

// Vec<T>: Default is empty vector
impl<T> Default for Pattern<Vec<T>>
// Pattern::default() == Pattern { value: vec![], elements: vec![] }

// (): Default is unit
impl Default for Pattern<()>
// Pattern::default() == Pattern { value: (), elements: vec![] }

// Numeric types: Default is zero
impl Default for Pattern<i32>  // Also i8, i16, i64, i128, isize
// Pattern::default() == Pattern { value: 0, elements: vec![] }

impl Default for Pattern<u32>  // Also u8, u16, u64, u128, usize
// Pattern::default() == Pattern { value: 0, elements: vec![] }

impl Default for Pattern<f32>  // Also f64
// Pattern::default() == Pattern { value: 0.0, elements: vec![] }

// Bool: Default is false
impl Default for Pattern<bool>
// Pattern::default() == Pattern { value: false, elements: vec![] }
```

### Custom Value Types

For custom types to work with `Pattern::default()`:

```rust
#[derive(Default, Clone, PartialEq, Eq)]
struct CustomValue {
    data: String,
    count: i32,
}

impl Combinable for CustomValue {
    fn combine(mut self, other: Self) -> Self {
        self.data.push_str(&other.data);
        self.count += other.count;
        self
    }
}

// Now Pattern<CustomValue> can implement Default
impl Default for Pattern<CustomValue>  // Automatically available
```

---

## Testing Signatures

### Property-Based Tests

```rust
use proptest::prelude::*;

// Left identity law
proptest! {
    #[test]
    fn prop_left_identity(p: Pattern<String>) {
        let empty = Pattern::default();
        prop_assert_eq!(empty.combine(p.clone()), p);
    }
}

// Right identity law
proptest! {
    #[test]
    fn prop_right_identity(p: Pattern<String>) {
        let empty = Pattern::default();
        prop_assert_eq!(p.clone().combine(empty), p);
    }
}
```

### Unit Tests

```rust
#[test]
fn test_default_string_pattern() {
    let empty: Pattern<String> = Pattern::default();
    assert_eq!(empty.value(), "");
    assert_eq!(empty.length(), 0);
}

#[test]
fn test_identity_with_atomic_pattern() {
    let p = Pattern::point("test".to_string());
    let empty = Pattern::default();
    
    assert_eq!(empty.clone().combine(p.clone()), p);
    assert_eq!(p.clone().combine(empty), p);
}

#[test]
fn test_identity_with_compound_pattern() {
    let p = Pattern::pattern("root".to_string(), vec![
        Pattern::point("child".to_string())
    ]);
    let empty = Pattern::default();
    
    assert_eq!(empty.clone().combine(p.clone()), p);
    assert_eq!(p.clone().combine(empty), p);
}
```

---

## Equivalence with Haskell

### Haskell Monoid Instance

```haskell
instance Monoid v => Monoid (Pattern v) where
  mempty = Pattern mempty []
  
-- Usage:
mempty <> p == p
p <> mempty == p
```

### Rust Default Implementation

```rust
impl<V: Default> Default for Pattern<V> {
    fn default() -> Self {
        Pattern::point(V::default())
    }
}

// Usage:
Pattern::default().combine(p) == p
p.combine(Pattern::default()) == p
```

**Equivalence**:
- Haskell's `mempty` ≡ Rust's `Pattern::default()`
- Haskell's `<>` ≡ Rust's `.combine()`
- Both create pattern with default value and empty elements
- Both satisfy monoid identity laws

---

## Summary

The `Default` trait implementation provides:
1. **Identity element** for pattern combination
2. **Monoid completion** (with Combinable from feature 013)
3. **Idiomatic Rust** API using standard library traits
4. **Iterator integration** for fold and accumulation patterns
5. **Behavioral equivalence** with gram-hs Haskell implementation

The implementation is purely additive and doesn't affect existing pattern operations.

