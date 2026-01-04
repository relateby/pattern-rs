# Quick Reference: Pattern Query Operations

**Feature**: 011-basic-query-functions  
**Target Audience**: Developers using pattern-core library

## TL;DR

Three new predicate-based query operations for patterns:

```rust
// Check if ANY value matches
pattern.any_value(|v| *v > 5)  // → bool

// Check if ALL values match
pattern.all_values(|v| *v > 0)  // → bool

// Extract matching subpatterns
pattern.filter(|p| p.length() > 0)  // → Vec<&Pattern<V>>
```

## Quick Examples

### any_value - Find if at least one matches

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern(5, vec![
    Pattern::point(10),
    Pattern::point(3),
    Pattern::point(7),
]);

// Check if any value is greater than 8
assert!(pattern.any_value(|v| *v > 8));  // true (10 > 8)

// Check if any value is negative
assert!(!pattern.any_value(|v| *v < 0));  // false (all positive)
```

**When to use**: Validation, conditional logic, search operations

---

### all_values - Verify all match

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern(5, vec![
    Pattern::point(10),
    Pattern::point(3),
]);

// Check if all values are positive
assert!(pattern.all_values(|v| *v > 0));  // true

// Check if all values are greater than 8
assert!(!pattern.all_values(|v| *v > 8));  // false (5 and 3 fail)
```

**When to use**: Validation, invariant checking, constraint verification

---

### filter - Extract matching patterns

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern(
    "root",
    vec![
        Pattern::point("leaf1"),
        Pattern::pattern("branch", vec![
            Pattern::point("leaf2"),
        ]),
    ],
);

// Find all atomic (leaf) patterns
let leaves = pattern.filter(|p| p.is_atomic());
assert_eq!(leaves.len(), 2);  // leaf1, leaf2

// Find patterns with specific value
let roots = pattern.filter(|p| p.value() == "root");
assert_eq!(roots.len(), 1);

// Find patterns with specific structure
let branches = pattern.filter(|p| p.length() > 0);
assert_eq!(branches.len(), 2);  // root, branch
```

**When to use**: Pattern extraction, structural queries, graph component selection

## Common Patterns

### Validation

```rust
// Ensure all values meet requirement
fn validate_all_positive<V>(pattern: &Pattern<i32>) -> bool {
    pattern.all_values(|v| *v > 0)
}

// Check if any value violates constraint
fn has_invalid_value(pattern: &Pattern<String>) -> bool {
    pattern.any_value(|v| v.is_empty())
}
```

### Searching

```rust
// Find if pattern contains specific value
fn contains_value<V: PartialEq>(pattern: &Pattern<V>, target: &V) -> bool {
    pattern.any_value(|v| v == target)
}

// Extract all patterns matching criteria
fn find_deep_patterns<V>(pattern: &Pattern<V>, min_depth: usize) -> Vec<&Pattern<V>> {
    pattern.filter(|p| p.depth() >= min_depth)
}
```

### Combining Predicates

```rust
// Complex value predicate
pattern.any_value(|v| *v > 10 && *v < 20)

// Complex pattern predicate
pattern.filter(|p| {
    p.length() > 2 && p.depth() < 5 && p.size() < 100
})

// Chaining operations
let large_patterns = pattern.filter(|p| p.size() > 10);
let has_special_value = large_patterns
    .iter()
    .any(|p| p.any_value(|v| *v == 42));
```

## Performance Tips

### Short-Circuit Evaluation

```rust
// ✅ Good: Short-circuits on first match
pattern.any_value(|v| expensive_check(v))

// ✅ Good: Short-circuits on first failure
pattern.all_values(|v| expensive_check(v))

// ⚠️ Note: filter must visit all nodes
pattern.filter(|p| expensive_check(p))  // No short-circuit
```

### Predicate Efficiency

```rust
// ✅ Good: Simple predicate
pattern.any_value(|v| *v > 0)

// ⚠️ Avoid: Expensive operations in predicate
pattern.any_value(|v| {
    let result = very_expensive_computation(*v);
    result > threshold
})

// ✅ Better: Pre-compute if possible
let threshold = compute_threshold();
pattern.any_value(|v| *v > threshold)
```

### Reference vs Clone

```rust
// ✅ Good: Uses references (no cloning)
let matches = pattern.filter(|p| p.length() > 0);

// If you need owned patterns:
let owned: Vec<Pattern<V>> = matches.iter()
    .map(|p| (*p).clone())
    .collect();
```

## Edge Cases

### Empty Patterns

```rust
let atomic = Pattern::point(42);
assert_eq!(atomic.length(), 0);  // No elements

// any_value checks the value (42)
assert!(atomic.any_value(|v| *v == 42));

// all_values checks the value (42)
assert!(atomic.all_values(|v| *v > 0));
```

### Vacuous Truth

```rust
// For patterns with no values to check, all_values returns true
// (This is standard in logic: "all elements of empty set satisfy property")
let pattern_with_no_values = /* ... */;
assert!(pattern_with_no_values.all_values(|_| false));  // true!
```

### Filter Including Root

```rust
let pattern = Pattern::point("test");

// filter checks root pattern too
let matches = pattern.filter(|p| p.value() == "test");
assert_eq!(matches.len(), 1);  // Includes root
assert_eq!(matches[0].value(), "test");
```

## Common Mistakes

### ❌ Dereferencing Issues

```rust
// ❌ Wrong: Predicate takes reference
pattern.any_value(|v| v > 5)  // Type error!

// ✅ Correct: Dereference the reference
pattern.any_value(|v| *v > 5)
```

### ❌ Mutating in Predicates

```rust
// ❌ Wrong: Predicates should be pure
let mut counter = 0;
pattern.any_value(|v| {
    counter += 1;  // Side effect!
    *v > 0
});

// ✅ Correct: Use separate count if needed
let count = pattern.values().len();
let has_positive = pattern.any_value(|v| *v > 0);
```

### ❌ Expecting Owned Patterns from filter

```rust
// ❌ Won't compile: filter returns references
let owned: Vec<Pattern<i32>> = pattern.filter(|p| p.length() > 0);

// ✅ Correct: Use references or clone explicitly
let refs: Vec<&Pattern<i32>> = pattern.filter(|p| p.length() > 0);
let owned: Vec<Pattern<i32>> = refs.iter().map(|p| (*p).clone()).collect();
```

## Comparison with Existing Operations

| Operation | Purpose | Return Type | Short-Circuit |
|-----------|---------|-------------|---------------|
| `length()` | Count direct elements | `usize` | N/A (O(1)) |
| `size()` | Count total nodes | `usize` | No |
| `depth()` | Max nesting depth | `usize` | No |
| `values()` | Extract all values | `Vec<&V>` | No |
| **`any_value()`** | Check if any value matches | `bool` | **Yes** |
| **`all_values()`** | Check if all values match | `bool` | **Yes** |
| **`filter()`** | Extract matching patterns | `Vec<&Pattern<V>>` | No |

## Testing Your Usage

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use pattern_core::Pattern;

    #[test]
    fn test_validation() {
        let pattern = Pattern::pattern(5, vec![
            Pattern::point(10),
            Pattern::point(3),
        ]);
        
        // All values should be positive
        assert!(pattern.all_values(|v| *v > 0));
    }

    #[test]
    fn test_search() {
        let pattern = Pattern::pattern(
            "root",
            vec![Pattern::point("target")],
        );
        
        // Should find target value
        assert!(pattern.any_value(|v| v == "target"));
    }

    #[test]
    fn test_filtering() {
        let pattern = build_test_pattern();
        
        let atomic = pattern.filter(|p| p.is_atomic());
        assert!(!atomic.is_empty());
    }
}
```

## Further Reading

- **API Documentation**: `cargo doc --open`
- **Full Specification**: `specs/011-basic-query-functions/spec.md`
- **Type Signatures**: `specs/011-basic-query-functions/contracts/type-signatures.md`
- **Haskell Reference**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` (lines 945-1028)

## Cheat Sheet

```rust
// Value-based queries (check values)
pattern.any_value(|v| predicate(v))   // → bool
pattern.all_values(|v| predicate(v))  // → bool

// Pattern-based queries (check structure)
pattern.filter(|p| predicate(p))      // → Vec<&Pattern<V>>

// Existing structural queries
pattern.length()    // → usize (direct elements)
pattern.size()      // → usize (total nodes)
pattern.depth()     // → usize (max nesting)
pattern.values()    // → Vec<&V> (all values)

// Combining operations
pattern
    .filter(|p| p.size() > 10)
    .iter()
    .any(|p| p.any_value(|v| special_check(v)))
```

## Getting Help

- Check test files for examples: `crates/pattern-core/tests/query_*.rs`
- Review property tests: `crates/pattern-core/tests/property/query_operations.rs`
- Compare with Haskell examples: `../gram-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs`

