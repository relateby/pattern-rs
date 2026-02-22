# Quick Start: Foldable Instance for Pattern

**Feature**: 009-foldable-instance  
**Date**: 2026-01-04

## Overview

This guide demonstrates how to use the fold operations on `Pattern<V>` to reduce a pattern structure to a single value. Folding processes all values in the pattern in a predictable order (depth-first, root-first).

---

## Basic Usage

### Summing Values

```rust
use pattern_core::Pattern;

// Create a pattern with numeric values
let pattern = Pattern::pattern(10, vec![
    Pattern::point(20),
    Pattern::point(30),
]);

// Sum all values
let sum = pattern.fold(0, |acc, v| acc + v);
assert_eq!(sum, 60);  // 10 + 20 + 30
```

### Counting Values

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("root", vec![
    Pattern::point("child1"),
    Pattern::point("child2"),
    Pattern::point("child3"),
]);

// Count all values in the pattern
let count = pattern.fold(0, |acc, _| acc + 1);
assert_eq!(count, 4);  // root + 3 children
assert_eq!(count, pattern.size());  // Same as size()
```

### Concatenating Strings

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("Hello", vec![
    Pattern::point(" "),
    Pattern::point("World"),
]);

// Concatenate all strings
let result = pattern.fold(String::new(), |acc, s| acc + s);
assert_eq!(result, "Hello World");
```

---

## Collecting Values

### Get All Values as Vector

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern(1, vec![
    Pattern::point(2),
    Pattern::point(3),
]);

// Collect all values into a vector
let values: Vec<&i32> = pattern.values();
assert_eq!(values, vec![&1, &2, &3]);

// Or use fold directly
let values: Vec<i32> = pattern.fold(Vec::new(), |mut acc, v| {
    acc.push(*v);
    acc
});
assert_eq!(values, vec![1, 2, 3]);
```

### Verify Traversal Order

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("A", vec![
    Pattern::point("B"),
    Pattern::pattern("C", vec![
        Pattern::point("D"),
    ]),
]);

// Values are processed in depth-first, root-first order
let values: Vec<&str> = pattern.values();
assert_eq!(values, vec![&"A", &"B", &"C", &"D"]);

// Concatenation preserves order
let concat = pattern.fold(String::new(), |acc, s| acc + s);
assert_eq!(concat, "ABCD");
```

---

## Type Transformations

### Different Accumulator Type

```rust
use pattern_core::Pattern;

// Pattern with strings, fold to number
let pattern = Pattern::pattern("hello", vec![
    Pattern::point("world"),
    Pattern::point("!"),
]);

// Calculate total string length
let total_length: usize = pattern.fold(0, |acc, s| acc + s.len());
assert_eq!(total_length, 11);  // "hello" (5) + "world" (5) + "!" (1)
```

### Building Data Structures

```rust
use pattern_core::Pattern;
use std::collections::HashMap;

#[derive(Clone)]
struct Item {
    id: String,
    value: i32,
}

let pattern = Pattern::pattern(
    Item { id: "a".to_string(), value: 10 },
    vec![
        Pattern::point(Item { id: "b".to_string(), value: 20 }),
        Pattern::point(Item { id: "c".to_string(), value: 30 }),
    ],
);

// Build a map from the pattern
let map: HashMap<String, i32> = pattern.fold(
    HashMap::new(),
    |mut acc, item| {
        acc.insert(item.id.clone(), item.value);
        acc
    },
);

assert_eq!(map.get("a"), Some(&10));
assert_eq!(map.get("b"), Some(&20));
assert_eq!(map.get("c"), Some(&30));
```

---

## Advanced Examples

### Validation with Fold

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern(5, vec![
    Pattern::point(10),
    Pattern::point(15),
]);

// Check if all values are positive
let all_positive = pattern.fold(true, |acc, v| acc && *v > 0);
assert!(all_positive);

// Check if any value is greater than 10
let any_large = pattern.fold(false, |acc, v| acc || *v > 10);
assert!(any_large);

// Find maximum value
let max = pattern.fold(i32::MIN, |acc, v| acc.max(*v));
assert_eq!(max, 15);
```

### Composing with Map

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("hello", vec![
    Pattern::point("world"),
]);

// Transform values, then fold
let total_length = pattern
    .map(|s| s.len())
    .fold(0, |acc, len| acc + len);
assert_eq!(total_length, 10);  // "hello" (5) + "world" (5)

// Equivalent to folding with transformation inline
let total_length2 = pattern
    .fold(0, |acc, s| acc + s.len());
assert_eq!(total_length, total_length2);
```

### Multiple Folds on Same Pattern

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern(2, vec![
    Pattern::point(3),
    Pattern::point(4),
]);

// Fold doesn't consume the pattern, so we can fold multiple times
let sum = pattern.fold(0, |acc, v| acc + v);
let product = pattern.fold(1, |acc, v| acc * v);
let count = pattern.fold(0, |acc, _| acc + 1);

assert_eq!(sum, 9);      // 2 + 3 + 4
assert_eq!(product, 24); // 2 * 3 * 4
assert_eq!(count, 3);    // 3 values
```

---

## Nested Patterns

### Deep Nesting

```rust
use pattern_core::Pattern;

// Create a deeply nested pattern
let pattern = Pattern::pattern(1, vec![
    Pattern::pattern(2, vec![
        Pattern::pattern(3, vec![
            Pattern::point(4),
        ]),
    ]),
]);

// Fold processes all levels
let sum = pattern.fold(0, |acc, v| acc + v);
assert_eq!(sum, 10);  // 1 + 2 + 3 + 4

// Order is depth-first, root-first
let values: Vec<&i32> = pattern.values();
assert_eq!(values, vec![&1, &2, &3, &4]);
```

### Wide Patterns

```rust
use pattern_core::Pattern;

// Pattern with many siblings
let pattern = Pattern::pattern(0, vec![
    Pattern::point(1),
    Pattern::point(2),
    Pattern::point(3),
    Pattern::point(4),
    Pattern::point(5),
]);

// All siblings processed in order
let sum = pattern.fold(0, |acc, v| acc + v);
assert_eq!(sum, 15);  // 0 + 1 + 2 + 3 + 4 + 5
```

---

## Real-World Scenarios

### Collecting Node Labels

```rust
use pattern_core::{Pattern, Subject, Symbol};
use std::collections::{HashSet, HashMap};

// Create a pattern of subjects (graph nodes)
let node1 = Subject {
    identity: Symbol("n1".to_string()),
    labels: ["Person"].iter().map(|s| s.to_string()).collect(),
    properties: HashMap::new(),
};

let node2 = Subject {
    identity: Symbol("n2".to_string()),
    labels: ["Person", "Employee"].iter().map(|s| s.to_string()).collect(),
    properties: HashMap::new(),
};

let pattern = Pattern::pattern(node1, vec![Pattern::point(node2)]);

// Collect all labels from all subjects
let all_labels: HashSet<String> = pattern.fold(
    HashSet::new(),
    |mut acc, subject| {
        acc.extend(subject.labels.iter().cloned());
        acc
    },
);

assert!(all_labels.contains("Person"));
assert!(all_labels.contains("Employee"));
```

### Computing Statistics

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern(10, vec![
    Pattern::point(20),
    Pattern::point(30),
    Pattern::point(40),
]);

// Compute mean
let sum = pattern.fold(0, |acc, v| acc + v);
let count = pattern.fold(0, |acc, _| acc + 1);
let mean = sum as f64 / count as f64;
assert_eq!(mean, 25.0);  // (10 + 20 + 30 + 40) / 4

// Find min and max
let min = pattern.fold(i32::MAX, |acc, v| acc.min(*v));
let max = pattern.fold(i32::MIN, |acc, v| acc.max(*v));
assert_eq!(min, 10);
assert_eq!(max, 40);
```

---

## Common Patterns

### Early Termination Alternative

Rust's fold doesn't support early termination, but you can use `try_fold` on the values:

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern(1, vec![
    Pattern::point(2),
    Pattern::point(3),
    Pattern::point(4),
]);

// Find first value greater than 2
let values = pattern.values();
let first_large = values
    .iter()
    .find(|&&v| *v > 2);

assert_eq!(first_large, Some(&&3));
```

### Conditional Accumulation

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern(1, vec![
    Pattern::point(2),
    Pattern::point(3),
    Pattern::point(4),
]);

// Sum only even values
let even_sum = pattern.fold(0, |acc, v| {
    if v % 2 == 0 {
        acc + v
    } else {
        acc
    }
});
assert_eq!(even_sum, 6);  // 2 + 4
```

### Building Strings

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern(1, vec![
    Pattern::point(2),
    Pattern::point(3),
]);

// Build formatted string
let formatted = pattern.fold(
    String::from("["),
    |mut acc, v| {
        if acc.len() > 1 {
            acc.push_str(", ");
        }
        acc.push_str(&v.to_string());
        acc
    },
);
let formatted = formatted + "]";

assert_eq!(formatted, "[1, 2, 3]");
```

---

## Performance Considerations

### Efficient Accumulator Updates

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("a", vec![
    Pattern::point("b"),
    Pattern::point("c"),
]);

// GOOD: Use mutable accumulator for efficiency
let result = pattern.fold(Vec::new(), |mut acc, v| {
    acc.push(v.clone());
    acc
});

// AVOID: Creating new vectors each time (inefficient)
let result = pattern.fold(Vec::new(), |acc, v| {
    let mut new_vec = acc.clone();
    new_vec.push(v.clone());
    new_vec
});
```

### Large Patterns

```rust
use pattern_core::Pattern;

// For large patterns, fold is O(n) - efficient
let large_pattern = /* pattern with 1000s of nodes */;

// Fold is efficient even on large patterns
let sum = large_pattern.fold(0, |acc, v| acc + v);
```

---

## Comparison with Haskell

For developers familiar with Haskell's `Foldable`:

```haskell
-- Haskell
import Data.Foldable (fold, toList)

-- Sum values
sum pattern

-- Convert to list
toList pattern

-- Fold with custom function
foldr (+) 0 pattern
```

```rust
// Rust equivalent
use pattern_core::Pattern;

// Sum values
let sum = pattern.fold(0, |acc, v| acc + v);

// Convert to vector
let values = pattern.values();

// Fold with custom function
let result = pattern.fold(0, |acc, v| acc + v);
```

**Key Differences**:
- Haskell uses `foldMap` with Monoid; Rust uses explicit accumulator
- Haskell has both `foldr` and `foldl`; Rust provides right fold (can simulate left via Vec)
- Rust requires explicit initial value; Haskell can infer for Monoids

---

## Next Steps

- Read [data-model.md](./data-model.md) for detailed type information
- Read [contracts/type-signatures.md](./contracts/type-signatures.md) for API details
- See tests in `crates/pattern-core/tests/` for more examples
- Check [research.md](./research.md) for design decisions and alternatives

---

## References

- **Feature Spec**: [spec.md](./spec.md) - Requirements and user stories
- **Type Signatures**: [contracts/type-signatures.md](./contracts/type-signatures.md) - API contracts
- **Functor Instance**: [../008-functor-instance/](../008-functor-instance/) - Related feature
- **gram-hs Implementation**: `../../../pattern-hs/libs/pattern/src/Pattern/Core.hs`
