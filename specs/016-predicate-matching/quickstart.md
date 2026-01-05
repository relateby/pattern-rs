# Quick Reference: Predicate-Based Pattern Matching

**Feature**: 016-predicate-matching  
**Target Audience**: Developers using pattern-core library

## TL;DR

Three new pattern matching operations added to Pattern<V>:

```rust
// Find first matching pattern (returns Option)
pattern.find_first(|p| p.length() > 0)  // → Option<&Pattern<V>>

// Check structural equality
pattern1.matches(&pattern2)  // → bool

// Check subpattern containment
pattern.contains(&subpattern)  // → bool
```

## Quick Start Examples

### find_first - Find First Matching Pattern

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("root", vec![
    Pattern::point("leaf1"),
    Pattern::pattern("branch", vec![
        Pattern::point("leaf2"),
    ]),
]);

// Find first atomic (leaf) pattern
let first_leaf = pattern.find_first(|p| p.is_atomic());
assert_eq!(first_leaf.unwrap().value, "leaf1");

// Find first pattern with specific value
let branch = pattern.find_first(|p| p.value == "branch");
assert!(branch.is_some());

// No match returns None
let no_match = pattern.find_first(|p| p.value == "nonexistent");
assert_eq!(no_match, None);
```

**When to use**: Search for specific pattern, need just one result, want Option semantics

---

### matches - Check Structural Equality

```rust
use pattern_core::Pattern;

let p1 = Pattern::pattern("root", vec![
    Pattern::point("a"),
    Pattern::point("b"),
]);

let p2 = Pattern::pattern("root", vec![
    Pattern::point("a"),
    Pattern::point("b"),
]);

// Identical patterns match
assert!(p1.matches(&p2));

// Self-matching
assert!(p1.matches(&p1));

// Different structure doesn't match
let p3 = Pattern::pattern("root", vec![Pattern::point("a")]);
assert!(!p1.matches(&p3));
```

**When to use**: Compare pattern structures, check equality beyond ==, verify structural correspondence

---

### contains - Check Subpattern Containment

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("root", vec![
    Pattern::point("a"),
    Pattern::pattern("b", vec![
        Pattern::point("c"),
    ]),
]);

// Check for atomic subpattern
let subpat1 = Pattern::point("a");
assert!(pattern.contains(&subpat1));

// Check for nested subpattern
let subpat2 = Pattern::pattern("b", vec![Pattern::point("c")]);
assert!(pattern.contains(&subpat2));

// Self-containment
assert!(pattern.contains(&pattern));

// Non-existent subpattern
let subpat3 = Pattern::point("x");
assert!(!pattern.contains(&subpat3));
```

**When to use**: Check if pattern contains another, verify subpattern presence, structural queries

---

## Common Use Cases

### Use Case 1: Find Specific Patterns

```rust
// Find first pattern with depth > 2
let deep = pattern.find_first(|p| p.depth() > 2);

// Find first non-empty pattern
let non_empty = pattern.find_first(|p| p.length() > 0);

// Find first pattern with specific structural property
let special = pattern.find_first(|p| {
    p.length() > 2 && p.all_values(|v| *v > 0)
});
```

### Use Case 2: Structural Validation

```rust
// Check if two patterns have same structure
if pattern1.matches(&pattern2) {
    println!("Patterns have identical structure");
}

// Verify pattern contains expected subpatterns
let required_parts = vec![subpat1, subpat2, subpat3];
let all_present = required_parts.iter().all(|sp| pattern.contains(sp));
```

### Use Case 3: Pattern Search and Filter

```rust
// Find first, then process
if let Some(found) = pattern.find_first(|p| p.value == "target") {
    println!("Found: {}", found.value);
}

// Combine with existing operations
let matching_patterns: Vec<_> = pattern
    .filter(|p| p.length() > 0)
    .into_iter()
    .filter(|p| p.contains(&subpattern))
    .collect();
```

## Comparison with Existing Methods

| Method | Scope | Return | Short-Circuit |
|--------|-------|--------|---------------|
| any_value | Values only | bool | Yes (first true) |
| all_values | Values only | bool | Yes (first false) |
| filter | Patterns | Vec<&Pattern> | No |
| **find_first** | **Patterns** | **Option<&Pattern>** | **Yes (first Some)** |
| **matches** | **Two patterns** | **bool** | **Yes (first false)** |
| **contains** | **Two patterns** | **bool** | **Yes (first true)** |

## Method Selection Guide

**Use find_first when**:
- You need the first matching pattern
- You want Option semantics (Some/None)
- You want short-circuit evaluation
- You need a borrowed reference to the pattern

**Use matches when**:
- You need to compare two patterns structurally
- You want to verify identical structure
- You need reflexive/symmetric comparison
- Values implement PartialEq

**Use contains when**:
- You need to check subpattern presence
- You want structural containment testing
- You need reflexive/transitive checking
- Values implement PartialEq

**Use filter when**:
- You need all matching patterns
- You want a Vec of results
- You don't need early termination

## Advanced Examples

### Combining Methods

```rust
// Find pattern that matches target and contains subpattern
let result = pattern.find_first(|p| {
    p.matches(&target) && p.contains(&subpattern)
});

// Filter patterns that contain specific subpattern
let containers = pattern.filter(|p| p.contains(&subpattern));

// Check if any element matches a pattern
let has_match = pattern.any_value(|_| true) && 
    pattern.filter(|p| p.is_atomic()).iter().any(|p| p.matches(&target));
```

### With Value Predicates

```rust
// Find first pattern where all values are positive
let all_positive = pattern.find_first(|p| {
    p.all_values(|v| *v > 0)
});

// Check if pattern contains subpattern with specific value properties
let has_valid = pattern.filter(|p| {
    p.any_value(|v| *v > 100) && p.contains(&subpattern)
}).len() > 0;
```

### Structural Queries

```rust
// Find first balanced tree (depth == log₂(size))
let balanced = pattern.find_first(|p| {
    let expected_depth = (p.size() as f64).log2().ceil() as usize;
    p.depth() == expected_depth
});

// Find patterns with palindromic element structure
let palindrome = pattern.find_first(|p| {
    let elements = p.elements();
    let reversed: Vec<_> = elements.iter().rev().collect();
    elements.iter().zip(reversed.iter()).all(|(a, b)| a.matches(b))
});
```

## Edge Cases

### Atomic Patterns

```rust
// find_first on atomic pattern
let atomic = Pattern::point("a");
let found = atomic.find_first(|p| p.value == "a");
assert_eq!(found, Some(&atomic));  // Finds itself

// matches with atomic patterns
let a1 = Pattern::point("a");
let a2 = Pattern::point("a");
assert!(a1.matches(&a2));

// contains with atomic patterns
assert!(atomic.contains(&atomic));  // Self-containment
```

### Empty Elements

```rust
// Pattern with empty elements
let empty_elems = Pattern::pattern("root", vec![]);

let found = empty_elems.find_first(|p| p.is_atomic());
assert_eq!(found, None);  // No elements to match

assert!(empty_elems.matches(&Pattern::pattern("root", vec![])));
assert!(!empty_elems.matches(&Pattern::point("root")));  // Different structure
```

### No Matches

```rust
// find_first with no matches
let none = pattern.find_first(|p| p.value == "nonexistent");
assert_eq!(none, None);

// matches with different structures
let p1 = Pattern::point("a");
let p2 = Pattern::pattern("a", vec![]);
assert!(!p1.matches(&p2));  // Same value, different structure

// contains with non-existent subpattern
assert!(!pattern.contains(&Pattern::point("nonexistent")));
```

## Performance Tips

### Early Termination

```rust
// find_first stops on first match (efficient)
let first = pattern.find_first(|p| p.length() > 5);

// matches stops on first mismatch (efficient)
let equal = pattern1.matches(&pattern2);

// contains stops on first match (efficient)
let has_it = pattern.contains(&subpattern);
```

### Avoiding Redundant Work

```rust
// Good: Use find_first when you only need one
let first = pattern.find_first(predicate);

// Less efficient: Filter then take first
let first_alt = pattern.filter(predicate).into_iter().next();

// Good: Use matches for structural comparison
if pattern1.matches(&pattern2) { ... }

// Less efficient: Compare flattened values
if pattern1.values() == pattern2.values() { ... }  // Misses structure!
```

## Integration with Iterator

```rust
// Convert Option to Iterator
let results: Vec<_> = pattern.find_first(predicate).into_iter().collect();

// Chain with other iterator operations
let transformed = pattern.find_first(predicate)
    .map(|p| p.value)
    .unwrap_or_default();

// Use with filter
let both: Vec<_> = pattern.filter(|p| p.contains(&sub1) && p.contains(&sub2));
```

## Troubleshooting

**Q: find_first returns None but I know a match exists**
- Check your predicate logic
- Verify traversal order (depth-first pre-order)
- Use filter to see all matches

**Q: matches returns false for "identical" patterns**
- Check if structures are truly identical (not just values)
- Verify element counts match
- Use filter or find_first to examine structure

**Q: contains returns false unexpectedly**
- Verify subpattern structure matches exactly
- Check if values implement PartialEq correctly
- Use matches to debug structural differences

**Q: Methods not available**
- Check if V implements required bounds (PartialEq for matches/contains)
- Verify pattern-core version includes this feature

## Further Reading

- [Data Model](data-model.md) - Detailed type definitions and semantics
- [Type Signatures](contracts/type-signatures.md) - API contracts and guarantees
- [Research](research.md) - Design decisions and alternatives considered

