# Quick Start: Pattern Identity Element via Default Trait

**Feature**: 014-monoid-instance  
**Status**: Specification Phase

## Overview

This feature adds identity element support to Pattern types by implementing the standard `Default` trait. The identity pattern acts as a neutral element for combination operations, completing the monoid algebraic structure (associative operation + identity).

## Quick Example

```rust
use pattern_core::{Pattern, Combinable};

// Create the identity pattern
let empty = Pattern::<String>::default();
// Pattern { value: "", elements: vec![] }

// Identity laws in action
let p = Pattern::point("hello".to_string());

// Left identity: empty.combine(p) == p
assert_eq!(empty.clone().combine(p.clone()), p);

// Right identity: p.combine(empty) == p
assert_eq!(p.clone().combine(empty.clone()), p);
```

## Use Cases

### 1. Iterator Fold Operations

Use the default pattern as the initial value when folding a collection:

```rust
let patterns = vec![
    Pattern::point("hello".to_string()),
    Pattern::point(" ".to_string()),
    Pattern::point("world".to_string()),
];

let result = patterns.into_iter()
    .fold(Pattern::default(), |acc, p| acc.combine(p));

assert_eq!(result.value(), "hello world");
```

### 2. Handling Empty Collections

The default pattern provides a natural result for empty collections:

```rust
let empty_collection: Vec<Pattern<String>> = vec![];

let result = empty_collection.into_iter()
    .fold(Pattern::default(), |acc, p| acc.combine(p));

// Result is the identity pattern
assert_eq!(result, Pattern::default());
```

### 3. Building Patterns Incrementally

Start from the identity and add patterns incrementally:

```rust
let mut accumulator = Pattern::<String>::default();

for item in items {
    let p = Pattern::point(item);
    accumulator = accumulator.combine(p);
}
```

## Monoid Laws

The default pattern satisfies the monoid identity laws:

### Left Identity
```rust
Pattern::default().combine(x) == x
```

For any pattern `x`, combining the identity on the left yields `x`:

```rust
let p = Pattern::pattern("value".to_string(), vec![
    Pattern::point("child".to_string())
]);
let empty = Pattern::default();

assert_eq!(empty.combine(p.clone()), p);
```

### Right Identity
```rust
x.combine(Pattern::default()) == x
```

For any pattern `x`, combining the identity on the right yields `x`:

```rust
let p = Pattern::pattern("value".to_string(), vec![
    Pattern::point("child".to_string())
]);
let empty = Pattern::default();

assert_eq!(p.clone().combine(empty), p);
```

## Value Type Requirements

The `Pattern<V>` can implement `Default` when the value type `V` implements `Default`:

```rust
// Works because String implements Default
let s = Pattern::<String>::default();

// Works because Vec<T> implements Default
let v = Pattern::<Vec<i32>>::default();

// Works because () implements Default
let u = Pattern::<()>::default();

// Works because integers implement Default
let i = Pattern::<i32>::default();
```

For full monoid behavior (combination + identity), the value type must implement both `Default` and `Combinable`:

```rust
// String: implements Default (empty string) and Combinable (concatenation)
let empty = Pattern::<String>::default();
let p1 = Pattern::point("hello".to_string());
let p2 = Pattern::point(" world".to_string());

let result = empty.combine(p1).combine(p2);
assert_eq!(result.value(), "hello world");
```

## Testing Strategy

### Unit Tests

Basic tests verify the identity laws for specific cases:

```rust
#[test]
fn test_default_is_identity() {
    let empty = Pattern::<String>::default();
    let p = Pattern::point("test".to_string());
    
    // Left identity
    assert_eq!(empty.clone().combine(p.clone()), p);
    
    // Right identity
    assert_eq!(p.clone().combine(empty), p);
}
```

### Property-Based Tests

Use `proptest` to verify laws for randomly generated patterns:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn left_identity(p: Pattern<String>) {
        let empty = Pattern::default();
        prop_assert_eq!(empty.combine(p.clone()), p);
    }
    
    #[test]
    fn right_identity(p: Pattern<String>) {
        let empty = Pattern::default();
        prop_assert_eq!(p.clone().combine(empty), p);
    }
}
```

## Common Patterns

### Pattern Accumulation

```rust
// Accumulate patterns from an iterator
let result: Pattern<String> = source_data
    .into_iter()
    .map(|item| create_pattern(item))
    .fold(Pattern::default(), |acc, p| acc.combine(p));
```

### Optional Pattern Result

```rust
// Use reduce for optional result, or default for guaranteed result
let opt_result = patterns.into_iter()
    .reduce(|acc, p| acc.combine(p));  // Returns Option<Pattern<V>>

let guaranteed_result = patterns.into_iter()
    .fold(Pattern::default(), |acc, p| acc.combine(p));  // Returns Pattern<V>
```

### Default as Placeholder

```rust
// Use default as a placeholder that will be replaced
let mut current = Pattern::default();

if let Some(first) = patterns.pop() {
    current = first;
    for p in patterns {
        current = current.combine(p);
    }
}
```

## Integration with Existing Features

This feature builds on and integrates with:

- **Feature 013 (Semigroup)**: Provides the `combine()` operation
- **Feature 009 (Foldable)**: Works with `fold()` method on patterns
- **Feature 008 (Functor)**: `map()` over default pattern preserves identity
- **Feature 011 (Query)**: Default pattern has single value (the default value)

## Next Steps

After specification is approved:
1. Create technical plan (`/speckit.plan`)
2. Implement `Default` trait for `Pattern<V>`
3. Add property-based tests for identity laws
4. Update documentation with monoid law explanations
5. Verify behavioral equivalence with gram-hs

## References

- **Specification**: [spec.md](spec.md)
- **Data Model**: [data-model.md](data-model.md)
- **Research**: [research.md](research.md)
- **Feature 013**: Semigroup/Combinable trait
- **Haskell Reference**: `../gram-hs/libs/pattern/` (Monoid instance)

