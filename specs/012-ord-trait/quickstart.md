# Quickstart: Pattern Ordering and Comparison

**Feature**: 012-ord-trait  
**Date**: 2025-01-04  
**For**: Developers using pattern-core library

## Overview

This guide shows how to use ordering and comparison operations with patterns. After implementing this feature, patterns can be sorted, compared, and used in ordered data structures.

## Basic Usage

### Comparing Patterns

```rust
use pattern_core::Pattern;
use std::cmp::Ordering;

// Compare atomic patterns
let p1 = Pattern::point(1);
let p2 = Pattern::point(2);

// Using cmp method
assert_eq!(p1.cmp(&p2), Ordering::Less);
assert_eq!(p2.cmp(&p1), Ordering::Greater);
assert_eq!(p1.cmp(&p1), Ordering::Equal);

// Using comparison operators (more idiomatic)
assert!(p1 < p2);
assert!(p1 <= p2);
assert!(p2 > p1);
assert!(p2 >= p1);
assert!(p1 == p1);
```

### Sorting Patterns

```rust
use pattern_core::Pattern;

let mut patterns = vec![
    Pattern::point(3),
    Pattern::point(1),
    Pattern::point(2),
];

// Sort in place
patterns.sort();

assert_eq!(patterns, vec![
    Pattern::point(1),
    Pattern::point(2),
    Pattern::point(3),
]);
```

### Finding Min/Max

```rust
use pattern_core::Pattern;

let patterns = vec![
    Pattern::point(3),
    Pattern::point(1),
    Pattern::point(2),
];

// Find minimum
let min_pattern = patterns.iter().min().unwrap();
assert_eq!(min_pattern, &Pattern::point(1));

// Find maximum
let max_pattern = patterns.iter().max().unwrap();
assert_eq!(max_pattern, &Pattern::point(3));

// Min/max of two patterns
let p1 = Pattern::point(5);
let p2 = Pattern::point(10);
assert_eq!(p1.clone().min(p2.clone()), p1);
assert_eq!(p1.max(p2.clone()), p2);
```

## Nested Patterns

### Comparing Nested Structures

```rust
use pattern_core::Pattern;

// Same value, different elements
let p1 = Pattern::pattern(5, vec![Pattern::point(1)]);
let p2 = Pattern::pattern(5, vec![Pattern::point(2)]);

assert!(p1 < p2);  // Values equal, first element 1 < 2

// Different values (value takes precedence)
let p3 = Pattern::pattern(3, vec![Pattern::point(100)]);
let p4 = Pattern::pattern(4, vec![Pattern::point(1)]);

assert!(p3 < p4);  // 3 < 4, elements not compared
```

### Sorting Nested Patterns

```rust
use pattern_core::Pattern;

let mut patterns = vec![
    Pattern::pattern("b", vec![Pattern::point("y")]),
    Pattern::pattern("a", vec![Pattern::point("z")]),
    Pattern::pattern("a", vec![Pattern::point("x")]),
];

patterns.sort();

// Sorted by value first, then by elements
assert_eq!(patterns, vec![
    Pattern::pattern("a", vec![Pattern::point("x")]),
    Pattern::pattern("a", vec![Pattern::point("z")]),
    Pattern::pattern("b", vec![Pattern::point("y")]),
]);
```

## Ordered Collections

### BTreeSet - Ordered Set

```rust
use pattern_core::Pattern;
use std::collections::BTreeSet;

let mut set = BTreeSet::new();

// Insert patterns (automatically maintains order)
set.insert(Pattern::point(3));
set.insert(Pattern::point(1));
set.insert(Pattern::point(2));
set.insert(Pattern::point(1));  // Duplicate, won't be added

// Iteration in order
let ordered: Vec<_> = set.iter().cloned().collect();
assert_eq!(ordered, vec![
    Pattern::point(1),
    Pattern::point(2),
    Pattern::point(3),
]);

// Check membership
assert!(set.contains(&Pattern::point(2)));
assert!(!set.contains(&Pattern::point(4)));
```

### BTreeMap - Ordered Map with Pattern Keys

```rust
use pattern_core::Pattern;
use std::collections::BTreeMap;

let mut map = BTreeMap::new();

// Use patterns as keys
map.insert(Pattern::point(3), "third");
map.insert(Pattern::point(1), "first");
map.insert(Pattern::point(2), "second");

// Iteration in key order
for (pattern, value) in &map {
    println!("{:?}: {}", pattern, value);
}
// Output:
// Pattern { value: 1, elements: [] }: first
// Pattern { value: 2, elements: [] }: second
// Pattern { value: 3, elements: [] }: third

// Retrieve by key
assert_eq!(map.get(&Pattern::point(2)), Some(&"second"));
```

### BTreeMap - Range Queries

```rust
use pattern_core::Pattern;
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
for i in 0..10 {
    map.insert(Pattern::point(i), format!("value-{}", i));
}

// Range query: all patterns from 3 to 7
let range: Vec<_> = map
    .range(Pattern::point(3)..=Pattern::point(7))
    .collect();

assert_eq!(range.len(), 5);  // 3, 4, 5, 6, 7
```

### BinaryHeap - Priority Queue

```rust
use pattern_core::Pattern;
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();

// Insert patterns (max-heap)
heap.push(Pattern::point(1));
heap.push(Pattern::point(5));
heap.push(Pattern::point(3));
heap.push(Pattern::point(2));

// Pop in descending order (largest first)
assert_eq!(heap.pop(), Some(Pattern::point(5)));
assert_eq!(heap.pop(), Some(Pattern::point(3)));
assert_eq!(heap.pop(), Some(Pattern::point(2)));
assert_eq!(heap.pop(), Some(Pattern::point(1)));
assert_eq!(heap.pop(), None);
```

## Advanced Usage

### Binary Search

```rust
use pattern_core::Pattern;

let patterns = vec![
    Pattern::point(1),
    Pattern::point(3),
    Pattern::point(5),
    Pattern::point(7),
    Pattern::point(9),
];

// Binary search (requires sorted vector)
match patterns.binary_search(&Pattern::point(5)) {
    Ok(index) => println!("Found at index {}", index),  // index = 2
    Err(index) => println!("Not found, would insert at {}", index),
}

// Search for non-existent element
match patterns.binary_search(&Pattern::point(6)) {
    Ok(_) => unreachable!(),
    Err(index) => assert_eq!(index, 3),  // Would insert between 5 and 7
}
```

### Sorting with Custom Key

```rust
use pattern_core::Pattern;

let mut patterns = vec![
    Pattern::pattern("short", vec![]),
    Pattern::pattern("medium", vec![Pattern::point(1)]),
    Pattern::pattern("long", vec![Pattern::point(1), Pattern::point(2)]),
];

// Sort by element count
patterns.sort_by_key(|p| p.length());

assert_eq!(patterns[0].value, "short");   // 0 elements
assert_eq!(patterns[1].value, "medium");  // 1 element
assert_eq!(patterns[2].value, "long");    // 2 elements
```

### Deduplication

```rust
use pattern_core::Pattern;

let mut patterns = vec![
    Pattern::point(1),
    Pattern::point(2),
    Pattern::point(1),  // Duplicate
    Pattern::point(3),
    Pattern::point(2),  // Duplicate
];

// Sort and deduplicate
patterns.sort();
patterns.dedup();

assert_eq!(patterns, vec![
    Pattern::point(1),
    Pattern::point(2),
    Pattern::point(3),
]);
```

### Partial Ordering (with Option<Ordering>)

```rust
use pattern_core::Pattern;
use std::cmp::Ordering;

// For types that implement PartialOrd but not Ord (e.g., floats with NaN)
let p1: Pattern<f64> = Pattern::point(1.0);
let p2: Pattern<f64> = Pattern::point(2.0);
let p_nan: Pattern<f64> = Pattern::point(f64::NAN);

// partial_cmp returns Option<Ordering>
assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Less));
assert_eq!(p_nan.partial_cmp(&p1), None);  // NaN can't be compared
```

## Common Patterns

### Finding Minimum in a Stream

```rust
use pattern_core::Pattern;

let patterns = vec![
    Pattern::point(3),
    Pattern::point(1),
    Pattern::point(4),
    Pattern::point(1),
    Pattern::point(5),
];

// Fold to find minimum
let min = patterns.iter()
    .fold(None, |min_so_far, p| {
        match min_so_far {
            None => Some(p),
            Some(min) => Some(if p < min { p } else { min }),
        }
    });

assert_eq!(min, Some(&Pattern::point(1)));
```

### Top K Patterns

```rust
use pattern_core::Pattern;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// Find top 3 smallest patterns (using min-heap)
let patterns = vec![
    Pattern::point(5),
    Pattern::point(2),
    Pattern::point(8),
    Pattern::point(1),
    Pattern::point(9),
    Pattern::point(3),
];

// Use Reverse to make it a min-heap
let mut heap: BinaryHeap<Reverse<Pattern<i32>>> = BinaryHeap::new();

for pattern in patterns {
    heap.push(Reverse(pattern));
    if heap.len() > 3 {
        heap.pop();
    }
}

// Extract top 3
let top3: Vec<_> = heap.into_sorted_vec()
    .into_iter()
    .map(|Reverse(p)| p)
    .collect();

assert_eq!(top3, vec![
    Pattern::point(1),
    Pattern::point(2),
    Pattern::point(3),
]);
```

### Merging Sorted Pattern Lists

```rust
use pattern_core::Pattern;

let list1 = vec![
    Pattern::point(1),
    Pattern::point(3),
    Pattern::point(5),
];

let list2 = vec![
    Pattern::point(2),
    Pattern::point(4),
    Pattern::point(6),
];

// Merge two sorted lists
let merged: Vec<_> = list1.into_iter()
    .chain(list2.into_iter())
    .collect();

let mut merged = merged;
merged.sort();

assert_eq!(merged, vec![
    Pattern::point(1),
    Pattern::point(2),
    Pattern::point(3),
    Pattern::point(4),
    Pattern::point(5),
    Pattern::point(6),
]);
```

## Performance Considerations

### Efficient Comparison

```rust
use pattern_core::Pattern;

// ✅ Efficient: Values differ, stops immediately
let p1 = Pattern::pattern(1, vec![/* huge element list */]);
let p2 = Pattern::pattern(2, vec![/* huge element list */]);
assert!(p1 < p2);  // O(1), only compares values

// ⚠️ Less efficient: Values equal, must compare all elements
let p3 = Pattern::pattern(1, vec![/* many elements */]);
let p4 = Pattern::pattern(1, vec![/* many elements */]);
p3.cmp(&p4);  // O(n), compares all elements
```

### Sorting Large Collections

```rust
use pattern_core::Pattern;

// For very large collections, consider:
let mut patterns: Vec<Pattern<i32>> = /* ... */;

// 1. Use parallel sorting (if available)
// patterns.par_sort(); // rayon crate

// 2. Or use unstable sort (faster, doesn't preserve order of equal elements)
patterns.sort_unstable();

// 3. Or sort by key if extracting key is cheaper than comparison
patterns.sort_by_key(|p| p.value);
```

## Error Handling

### Type Constraints

```rust
use pattern_core::Pattern;

// ✅ Works: i32 implements Ord
let p1: Pattern<i32> = Pattern::point(1);
let p2: Pattern<i32> = Pattern::point(2);
assert!(p1 < p2);

// ✅ Works: String implements Ord
let p3: Pattern<String> = Pattern::point("a".to_string());
let p4: Pattern<String> = Pattern::point("b".to_string());
assert!(p3 < p4);

// ❌ Won't compile: MyType doesn't implement Ord
// struct MyType { /* ... */ }
// let p5: Pattern<MyType> = Pattern::point(MyType { /* ... */ });
// let p6: Pattern<MyType> = Pattern::point(MyType { /* ... */ });
// p5.cmp(&p6);  // COMPILE ERROR: MyType doesn't implement Ord
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use pattern_core::Pattern;
    use std::cmp::Ordering;

    #[test]
    fn test_atomic_pattern_ordering() {
        let p1 = Pattern::point(1);
        let p2 = Pattern::point(2);
        
        assert_eq!(p1.cmp(&p2), Ordering::Less);
        assert_eq!(p2.cmp(&p1), Ordering::Greater);
        assert_eq!(p1.cmp(&p1), Ordering::Equal);
    }

    #[test]
    fn test_nested_pattern_ordering() {
        let p1 = Pattern::pattern(5, vec![Pattern::point(1)]);
        let p2 = Pattern::pattern(5, vec![Pattern::point(2)]);
        
        assert!(p1 < p2);
    }

    #[test]
    fn test_sorting() {
        let mut patterns = vec![
            Pattern::point(3),
            Pattern::point(1),
            Pattern::point(2),
        ];
        
        patterns.sort();
        
        assert_eq!(patterns, vec![
            Pattern::point(1),
            Pattern::point(2),
            Pattern::point(3),
        ]);
    }
}
```

## Next Steps

- **Read**: [data-model.md](./data-model.md) - Detailed comparison semantics
- **Read**: [contracts/type-signatures.md](./contracts/type-signatures.md) - Complete API reference
- **Explore**: Use patterns in your ordered data structures
- **Test**: Write property-based tests for your pattern comparisons

## Summary

Pattern ordering provides:
- ✅ Simple comparison using `<`, `<=`, `>`, `>=` operators
- ✅ Sorting with `sort()` and `sort_by_key()`
- ✅ Min/max operations with `min()`, `max()`
- ✅ Ordered collections: BTreeSet, BTreeMap, BinaryHeap
- ✅ Binary search with `binary_search()`
- ✅ Value-first comparison (values take precedence over structure)
- ✅ Lexicographic element comparison (recursive, short-circuits early)
- ✅ Type-safe (compile-time checks ensure comparable values)

