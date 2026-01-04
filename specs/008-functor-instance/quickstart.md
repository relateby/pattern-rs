# Quickstart: Functor Instance for Pattern

**Feature**: 008-functor-instance  
**Date**: 2026-01-04

## Overview

This guide shows how to use the `map` method to transform values in `Pattern<V>` structures while preserving their shape.

**Key Concept**: The `map` method applies a function to every value in a pattern, creating a new pattern with the same structure but transformed values.

## Basic Usage

### Transform Values (Same Type)

Transform string values to uppercase:

```rust
use pattern_core::Pattern;

fn main() {
    // Create a pattern
    let pattern = Pattern::point("hello");
    
    // Transform all values to uppercase
    let upper = pattern.map(|s| s.to_uppercase());
    
    assert_eq!(upper.value, "HELLO");
}
```

### Convert Types

Convert numbers to strings:

```rust
use pattern_core::Pattern;

fn main() {
    // Pattern with numeric values
    let numbers = Pattern::point(42);
    
    // Convert to strings
    let strings = numbers.map(|n| n.to_string());
    
    assert_eq!(strings.value, "42");
}
```

### Nested Patterns

Transform all values in nested structure:

```rust
use pattern_core::Pattern;

fn main() {
    // Create nested pattern
    let pattern = Pattern::pattern("root", vec![
        Pattern::point("child1"),
        Pattern::point("child2"),
    ]);
    
    // Transform all values (root and children)
    let upper = pattern.map(|s| s.to_uppercase());
    
    assert_eq!(upper.value, "ROOT");
    assert_eq!(upper.elements[0].value, "CHILD1");
    assert_eq!(upper.elements[1].value, "CHILD2");
}
```

## Common Patterns

### Chain Multiple Transformations

```rust
use pattern_core::Pattern;

fn main() {
    let result = Pattern::point(5)
        .map(|n| n * 2)           // Multiply by 2
        .map(|n| n + 1)           // Add 1
        .map(|n| format!("Result: {}", n)); // Convert to string
    
    assert_eq!(result.value, "Result: 11");
}
```

### Extract Fields from Structs

```rust
use pattern_core::Pattern;

#[derive(Clone)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    
    let pattern: Pattern<Person> = Pattern::point(person);
    
    // Extract just the names
    let names = pattern.map(|p| p.name.clone());
    
    assert_eq!(names.value, "Alice");
}
```

### Arithmetic Operations

```rust
use pattern_core::Pattern;

fn main() {
    let pattern = Pattern::pattern(10, vec![
        Pattern::point(20),
        Pattern::point(30),
    ]);
    
    // Double all values
    let doubled = pattern.map(|n| n * 2);
    
    assert_eq!(doubled.value, 20);
    assert_eq!(doubled.elements[0].value, 40);
    assert_eq!(doubled.elements[1].value, 60);
}
```

## Advanced Usage

### Complex Nested Structures

```rust
use pattern_core::Pattern;

fn main() {
    // Deeply nested pattern
    let deep = Pattern::pattern("level1", vec![
        Pattern::pattern("level2", vec![
            Pattern::point("level3"),
        ]),
    ]);
    
    // Transform reaches all levels
    let transformed = deep.map(|s| s.len());
    
    assert_eq!(transformed.value, 6);  // "level1".len()
    assert_eq!(transformed.elements[0].value, 6);  // "level2".len()
    assert_eq!(transformed.elements[0].elements[0].value, 6);  // "level3".len()
}
```

### Conditional Transformations

```rust
use pattern_core::Pattern;

fn main() {
    let pattern = Pattern::pattern(1, vec![
        Pattern::point(2),
        Pattern::point(3),
        Pattern::point(4),
    ]);
    
    // Double even numbers, triple odd numbers
    let transformed = pattern.map(|n| {
        if n % 2 == 0 {
            n * 2
        } else {
            n * 3
        }
    });
    
    assert_eq!(transformed.value, 3);   // 1 * 3
    assert_eq!(transformed.elements[0].value, 4);   // 2 * 2
    assert_eq!(transformed.elements[1].value, 9);   // 3 * 3
    assert_eq!(transformed.elements[2].value, 8);   // 4 * 2
}
```

### Using External Functions

```rust
use pattern_core::Pattern;

// Helper function
fn double(n: &i32) -> i32 {
    n * 2
}

fn main() {
    let pattern = Pattern::point(21);
    let doubled = pattern.map(double);
    assert_eq!(doubled.value, 42);
}
```

### Capturing Context in Closures

```rust
use pattern_core::Pattern;

fn main() {
    let multiplier = 10;
    
    let pattern = Pattern::pattern(1, vec![
        Pattern::point(2),
        Pattern::point(3),
    ]);
    
    // Closure captures `multiplier` from environment
    let scaled = pattern.map(|n| n * multiplier);
    
    assert_eq!(scaled.value, 10);
    assert_eq!(scaled.elements[0].value, 20);
    assert_eq!(scaled.elements[1].value, 30);
}
```

## Performance Considerations

### Efficient Transformations

```rust
use pattern_core::Pattern;

fn main() {
    // ✅ Efficient: Consumes original pattern
    let pattern = Pattern::point("hello");
    let upper = pattern.map(|s| s.to_uppercase());
    
    // ✅ Efficient: Chain transformations without intermediate clones
    let result = Pattern::point(5)
        .map(|n| n * 2)
        .map(|n| n + 1);
}
```

### When to Clone

```rust
use pattern_core::Pattern;

fn main() {
    let pattern = Pattern::point("hello");
    
    // If you need to keep the original:
    let original = pattern.clone();  // Clone before mapping
    let transformed = pattern.map(|s| s.to_uppercase());
    
    // Now both exist:
    assert_eq!(original.value, "hello");
    assert_eq!(transformed.value, "HELLO");
}
```

## Common Pitfalls

### ❌ Trying to Mutate Values

```rust
// This won't compile - function takes &V, not &mut V
let pattern = Pattern::point(42);
// let bad = pattern.map(|n| { *n += 1; *n });  // ❌ Compile error
```

**Solution**: Return a new value:
```rust
let pattern = Pattern::point(42);
let good = pattern.map(|n| n + 1);  // ✅ Returns new value
```

### ❌ Forgetting to Handle References

```rust
#[derive(Clone)]
struct Data { value: i32 }

let pattern: Pattern<Data> = Pattern::point(Data { value: 42 });

// Function receives &Data, not Data
let result = pattern.map(|d| d.value);  // ✅ Access field via reference
// let bad = pattern.map(|d| d + 1);     // ❌ Can't add to struct
```

### ❌ Using FnOnce

```rust
// This won't compile - FnOnce can't be called multiple times
let pattern = Pattern::pattern(1, vec![Pattern::point(2)]);
// let moved_value = String::from("hello");
// let bad = pattern.map(|n| moved_value);  // ❌ Moves `moved_value` first time
```

**Solution**: Clone or use references:
```rust
let pattern = Pattern::pattern(1, vec![Pattern::point(2)]);
let value = String::from("hello");
let good = pattern.map(|_n| value.clone());  // ✅ Clones each time
```

## Testing Your Transformations

### Verify Structure Preservation

```rust
use pattern_core::Pattern;

#[test]
fn test_structure_preserved() {
    let original = Pattern::pattern("root", vec![
        Pattern::point("child1"),
        Pattern::point("child2"),
    ]);
    
    let original_size = original.size();
    let original_depth = original.depth();
    let original_length = original.length();
    
    let transformed = original.map(|s| s.len());
    
    assert_eq!(transformed.size(), original_size);
    assert_eq!(transformed.depth(), original_depth);
    assert_eq!(transformed.length(), original_length);
}
```

### Verify Functor Laws

```rust
use pattern_core::Pattern;

#[test]
fn test_identity_law() {
    let pattern = Pattern::point(42);
    let identity = pattern.clone().map(|x| *x);
    assert_eq!(pattern, identity);
}

#[test]
fn test_composition_law() {
    let pattern = Pattern::point(5);
    let f = |x: &i32| x * 2;
    let g = |x: &i32| x + 1;
    
    let composed = pattern.clone().map(|x| g(&f(x)));
    let sequential = pattern.map(f).map(g);
    
    assert_eq!(composed, sequential);
}
```

## Integration with Existing Features

### Works with Pattern Construction

```rust
use pattern_core::Pattern;

fn main() {
    // Using Pattern::from_list
    let pattern = Pattern::from_list("root", vec!["a", "b", "c"]);
    let upper = pattern.map(|s| s.to_uppercase());
    
    assert_eq!(upper.value, "ROOT");
    assert_eq!(upper.elements.len(), 3);
}
```

### Works with Pattern Accessors

```rust
use pattern_core::Pattern;

fn main() {
    let pattern = Pattern::pattern("root", vec![
        Pattern::point("child"),
    ]);
    
    let transformed = pattern.map(|s| s.len());
    
    // Use accessor methods on result
    assert_eq!(transformed.value(), &4);  // "root".len()
    assert_eq!(transformed.length(), 1);
    assert_eq!(transformed.size(), 2);
}
```

## Next Steps

- See [data-model.md](./data-model.md) for detailed transformation semantics
- See [contracts/type-signatures.md](./contracts/type-signatures.md) for type information
- See [plan.md](./plan.md) for implementation details
- Try the examples in your project!

## Summary

The `map` method provides a simple, powerful way to transform pattern values:

✅ **Preserves structure**: Element count, depth, and order unchanged  
✅ **Type-safe**: Compile-time checking prevents errors  
✅ **Composable**: Chain multiple transformations  
✅ **Efficient**: Zero-copy transformations when possible  
✅ **Flexible**: Works with any value types

**Remember**: `map` takes a reference to each value (`&V`) and returns a new value (`W`), creating a new pattern while preserving the original structure.

