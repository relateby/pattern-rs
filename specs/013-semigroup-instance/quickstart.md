# Quickstart: Pattern Combination Operations

**Feature**: 013-semigroup-instance  
**Date**: 2026-01-04

## Overview

This guide shows how to use pattern combination operations to build complex patterns from simpler components. The `combine()` method provides an associative way to merge two patterns by combining their values and concatenating their elements.

## Basic Usage

### Combining Atomic Patterns

The simplest case: combining patterns with no elements.

```rust
use pattern_core::Pattern;

fn main() {
    // Create two atomic patterns
    let p1 = Pattern::point("hello");
    let p2 = Pattern::point(" world");
    
    // Combine them
    let result = p1.combine(p2);
    
    // result: Pattern { value: "hello world", elements: [] }
    assert_eq!(result.value(), "hello world");
    assert_eq!(result.length(), 0);
}
```

### Combining Patterns with Elements

Combining patterns preserves all elements from both patterns.

```rust
use pattern_core::Pattern;

fn main() {
    // Create pattern with elements
    let p1 = Pattern::pattern("parent1", vec![
        Pattern::point("child1"),
        Pattern::point("child2"),
    ]);
    
    let p2 = Pattern::pattern("parent2", vec![
        Pattern::point("child3"),
    ]);
    
    // Combine them
    let result = p1.combine(p2);
    
    // result.value: "parent1parent2"
    // result.elements: [child1, child2, child3]
    assert_eq!(result.value(), "parent1parent2");
    assert_eq!(result.length(), 3);
}
```

## Associativity

The combination operation is associative, meaning grouping doesn't matter.

```rust
use pattern_core::Pattern;

fn main() {
    let a = Pattern::point("a");
    let b = Pattern::point("b");
    let c = Pattern::point("c");
    
    // Left association: (a ⊕ b) ⊕ c
    let left = a.clone().combine(b.clone()).combine(c.clone());
    
    // Right association: a ⊕ (b ⊕ c)
    let right = a.combine(b.combine(c));
    
    // Results are equal
    assert_eq!(left, right);
    assert_eq!(left.value(), "abc");
}
```

## Common Patterns

### Building Patterns Incrementally

Start with a base pattern and add to it.

```rust
use pattern_core::Pattern;

fn main() {
    // Start with an empty structure
    let mut pattern = Pattern::point("");
    
    // Add components incrementally
    pattern = pattern.combine(Pattern::point("first "));
    pattern = pattern.combine(Pattern::point("second "));
    pattern = pattern.combine(Pattern::point("third"));
    
    assert_eq!(pattern.value(), "first second third");
}
```

### Combining Multiple Patterns

Use iterator methods to combine many patterns.

```rust
use pattern_core::Pattern;

fn main() {
    let patterns = vec![
        Pattern::point("a"),
        Pattern::point("b"),
        Pattern::point("c"),
        Pattern::point("d"),
    ];
    
    // Combine all using reduce
    let result = patterns.into_iter()
        .reduce(|acc, p| acc.combine(p))
        .unwrap();
    
    assert_eq!(result.value(), "abcd");
}
```

### Conditional Combination

Combine patterns based on conditions.

```rust
use pattern_core::Pattern;

fn main() {
    let base = Pattern::point("base");
    
    let extensions = vec![
        (true, Pattern::point("-ext1")),
        (false, Pattern::point("-ext2")),  // Won't be included
        (true, Pattern::point("-ext3")),
    ];
    
    let result = extensions.into_iter()
        .filter_map(|(include, pattern)| {
            if include { Some(pattern) } else { None }
        })
        .fold(base, |acc, p| acc.combine(p));
    
    assert_eq!(result.value(), "base-ext1-ext3");
}
```

## Working with Different Value Types

### String Values (Concatenation)

```rust
use pattern_core::Pattern;

fn main() {
    let greeting = Pattern::pattern("Hello, ".to_string(), vec![]);
    let name = Pattern::pattern("World!".to_string(), vec![]);
    
    let message = greeting.combine(name);
    
    assert_eq!(message.value(), "Hello, World!");
}
```

### Vector Values (Concatenation)

```rust
use pattern_core::Pattern;

fn main() {
    let p1 = Pattern::point(vec![1, 2, 3]);
    let p2 = Pattern::point(vec![4, 5]);
    
    let combined = p1.combine(p2);
    
    assert_eq!(combined.value(), &vec![1, 2, 3, 4, 5]);
}
```

### Unit Type (Trivial Combination)

```rust
use pattern_core::Pattern;

fn main() {
    let p1 = Pattern::pattern((), vec![Pattern::point(())]);
    let p2 = Pattern::pattern((), vec![Pattern::point(())]);
    
    let combined = p1.combine(p2);
    
    // Elements are combined, value remains ()
    assert_eq!(combined.value(), &());
    assert_eq!(combined.length(), 2);
}
```

## Advanced Usage

### Combining with map and fold

Chain combination with other pattern operations.

```rust
use pattern_core::Pattern;

fn main() {
    // Create patterns with numeric values
    let patterns = vec![
        Pattern::point(1),
        Pattern::point(2),
        Pattern::point(3),
    ];
    
    // Combine all, then transform
    let combined = patterns.into_iter()
        .reduce(|acc, p| acc.combine(p))
        .unwrap();
    
    let doubled = combined.map(|n| n * 2);
    
    assert_eq!(doubled.value(), &6);  // (1+2+3)*2 = 12? No: last value * 2 = 6
    // Note: For numeric types, you'd need to implement Combinable with addition semantics
}
```

### Pattern Trees

Build tree structures by combining nested patterns.

```rust
use pattern_core::Pattern;

fn main() {
    // Create leaf nodes
    let leaf1 = Pattern::point("leaf1");
    let leaf2 = Pattern::point("leaf2");
    let leaf3 = Pattern::point("leaf3");
    
    // Create branch nodes
    let branch1 = Pattern::pattern("branch1", vec![leaf1, leaf2]);
    let branch2 = Pattern::pattern("branch2", vec![leaf3]);
    
    // Combine branches into root
    let root = Pattern::pattern("root", vec![])
        .combine(branch1)
        .combine(branch2);
    
    // root now has elements from both branches
    assert_eq!(root.value(), "rootbranch1branch2");
    // Note: This flattens the structure - elements from branches become root's elements
}
```

### Implementing Combinable for Custom Types

```rust
use pattern_core::{Pattern, Combinable};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Counter {
    count: i32,
}

impl Combinable for Counter {
    fn combine(self, other: Self) -> Self {
        Counter {
            count: self.count + other.count,
        }
    }
}

fn main() {
    let p1 = Pattern::point(Counter { count: 5 });
    let p2 = Pattern::point(Counter { count: 3 });
    
    let combined = p1.combine(p2);
    
    assert_eq!(combined.value().count, 8);
}
```

## Performance Considerations

### Efficient Combination

The combine operation uses `Vec::extend` for efficient element concatenation.

```rust
use pattern_core::Pattern;

fn main() {
    // Create a pattern with many elements
    let elements1: Vec<_> = (0..1000)
        .map(|i| Pattern::point(i))
        .collect();
    
    let elements2: Vec<_> = (1000..2000)
        .map(|i| Pattern::point(i))
        .collect();
    
    let p1 = Pattern::pattern(0, elements1);
    let p2 = Pattern::pattern(0, elements2);
    
    // This is efficient: O(n) where n = total element count
    let combined = p1.combine(p2);
    
    assert_eq!(combined.length(), 2000);
}
```

### Avoiding Unnecessary Clones

The combine method moves its arguments, so clone only when necessary.

```rust
use pattern_core::Pattern;

fn main() {
    let p1 = Pattern::point("data");
    let p2 = Pattern::point("more");
    
    // ❌ Unnecessary clone
    let result1 = p1.clone().combine(p2.clone());
    
    // ✅ Move if you don't need the originals
    let result2 = p1.combine(p2);
    
    // If you do need to keep originals, clone is necessary
    let p3 = Pattern::point("keep");
    let p4 = Pattern::point("this");
    let keep_p3 = p3.clone();
    let result3 = p3.combine(p4);
    // Can still use keep_p3 here
}
```

## Common Pitfalls

### Non-Commutativity

Pattern combination is **not commutative** - order matters!

```rust
use pattern_core::Pattern;

fn main() {
    let p1 = Pattern::pattern("a", vec![Pattern::point("1")]);
    let p2 = Pattern::pattern("b", vec![Pattern::point("2")]);
    
    let result1 = p1.clone().combine(p2.clone());
    let result2 = p2.combine(p1);
    
    // Values are different: "ab" vs "ba"
    assert_ne!(result1.value(), result2.value());
    
    // Element order is different: [1, 2] vs [2, 1]
    assert_ne!(result1.elements(), result2.elements());
}
```

### Empty Collections

Be careful when combining from empty collections.

```rust
use pattern_core::Pattern;

fn main() {
    let patterns: Vec<Pattern<String>> = vec![];
    
    // This returns None because the iterator is empty
    let result = patterns.into_iter()
        .reduce(|acc, p| acc.combine(p));
    
    assert!(result.is_none());
    
    // Provide a default if you need a value
    let patterns: Vec<Pattern<String>> = vec![];
    let result = patterns.into_iter()
        .fold(Pattern::point(String::new()), |acc, p| acc.combine(p));
    
    assert_eq!(result.value(), "");  // Default empty string
}
```

## Testing Your Combinations

### Verify Associativity

When implementing Combinable for custom types, test associativity.

```rust
use pattern_core::{Pattern, Combinable};

#[derive(Debug, Clone, PartialEq, Eq)]
struct MyType { /* fields */ }

impl Combinable for MyType {
    fn combine(self, other: Self) -> Self {
        // Your implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_associativity(
            a in any::<MyType>(),
            b in any::<MyType>(),
            c in any::<MyType>(),
        ) {
            let left = a.clone().combine(b.clone()).combine(c.clone());
            let right = a.combine(b.combine(c));
            assert_eq!(left, right);
        }
    }
}
```

## Summary

### Key Points

- ✅ `combine()` merges two patterns by combining values and concatenating elements
- ✅ Operation is **associative**: (a⊕b)⊕c = a⊕(b⊕c)
- ✅ Operation is **not commutative**: a⊕b ≠ b⊕a (in general)
- ✅ Works with any type V that implements `Combinable`
- ✅ Efficient: O(n) where n = total element count

### Common Use Cases

- Building patterns incrementally
- Combining pattern collections
- Merging pattern fragments
- Compositional pattern construction

### Next Steps

- Explore the [data model](./data-model.md) for implementation details
- Review [type signatures](./contracts/type-signatures.md) for API contracts
- Check out [plan.md](./plan.md) for development approach

