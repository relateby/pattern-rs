# Data Model: Pattern Query Operations

**Date**: 2025-01-04  
**Phase**: 1 - Design & Contracts

## Overview

This document defines the data structures and types used in the pattern query operations feature. Since this feature adds methods to an existing type (`Pattern<V>`), the focus is on predicate types, return types, and data flow.

## Existing Types (No Changes)

### Pattern<V>

```rust
pub struct Pattern<V> {
    pub value: V,
    pub elements: Vec<Pattern<V>>,
}
```

**Status**: Already defined in `crates/pattern-core/src/pattern.rs`  
**Changes**: None (adding methods only)

## New Type Concepts

### Value Predicate

**Concept**: A function that takes a value reference and returns a boolean

**Rust Representation**:
```rust
F: Fn(&V) -> bool
```

**Characteristics**:
- Generic type parameter (compile-time polymorphism)
- Takes immutable reference to value (no copying)
- Pure function (should not mutate state)
- Used by: `any_value`, `all_values`

**Examples**:
```rust
// Closure
|v: &i32| *v > 5

// Function pointer
fn is_positive(v: &i32) -> bool { *v > 0 }

// Method reference
String::is_empty
```

### Pattern Predicate

**Concept**: A function that takes a pattern reference and returns a boolean

**Rust Representation**:
```rust
F: Fn(&Pattern<V>) -> bool
```

**Characteristics**:
- Generic type parameter (compile-time polymorphism)
- Takes immutable reference to entire pattern (access to value + elements)
- Pure function (should not mutate state)
- Used by: `filter`

**Examples**:
```rust
// Check if pattern is atomic
|p: &Pattern<String>| p.elements.is_empty()

// Check value property
|p: &Pattern<i32>| p.value > 10

// Check structural property
|p: &Pattern<String>| p.length() > 2

// Combined predicate
|p: &Pattern<i32>| p.value > 0 && p.depth() < 3
```

## Return Types

### Boolean Result

**Type**: `bool`  
**Used by**: `any_value`, `all_values`  
**Semantics**: 
- `true` = at least one value satisfies predicate (`any_value`)
- `true` = all values satisfy predicate (`all_values`)
- `false` = negation of above

**Special Cases**:
- Empty pattern + `any_value` → `false` (no values to test)
- Empty pattern + `all_values` → `true` (vacuous truth)

### Pattern Reference Collection

**Type**: `Vec<&Pattern<V>>`  
**Used by**: `filter`  
**Semantics**: 
- Collection of immutable references to matching patterns
- References point to patterns within the original structure
- Order: pre-order traversal (root first, then elements in order)
- Empty vec if no matches

**Lifetime**: References are valid as long as the source pattern exists

**Rationale for References**:
- Avoid cloning potentially large pattern structures
- Idiomatic Rust (borrowing rather than owning)
- Efficient: O(m) space where m = number of matches
- Users can clone explicitly if needed: `filter(...).into_iter().cloned().collect()`

## Data Flow

### any_value Data Flow

```text
Pattern<V> ----> fold() ----> bool
    |              |
    |              +-- Initial: false
    |              +-- Combine: acc || predicate(v)
    |
    +-- Values traversed in pre-order
    +-- Short-circuits on first true
```

**Steps**:
1. Start with accumulator = `false`
2. For each value in pre-order:
   - Evaluate `predicate(value)`
   - Combine with accumulator using OR: `acc || predicate(v)`
   - If result is `true`, short-circuit (stop evaluation)
3. Return final boolean

### all_values Data Flow

```text
Pattern<V> ----> fold() ----> bool
    |              |
    |              +-- Initial: true
    |              +-- Combine: acc && predicate(v)
    |
    +-- Values traversed in pre-order
    +-- Short-circuits on first false
```

**Steps**:
1. Start with accumulator = `true`
2. For each value in pre-order:
   - Evaluate `predicate(value)`
   - Combine with accumulator using AND: `acc && predicate(v)`
   - If result is `false`, short-circuit (stop evaluation)
3. Return final boolean

### filter Data Flow

```text
Pattern<V> ----> recursive traversal ----> Vec<&Pattern<V>>
    |                    |
    |                    +-- Check predicate on current
    |                    +-- Recursively check elements
    |
    +-- Pre-order traversal
    +-- No short-circuiting (must visit all)
```

**Steps**:
1. Create empty result vector
2. If `predicate(current_pattern)` is true:
   - Add reference to current pattern to results
3. For each element pattern:
   - Recursively call `filter` on element
   - Extend results with element's matches
4. Return result vector

## Validation Rules

### Value Predicate Requirements

- **MUST** be pure (no side effects)
- **SHOULD NOT** panic (undefined behavior if it does)
- **MAY** capture variables from enclosing scope
- **MUST** be safe to call multiple times with same value

### Pattern Predicate Requirements

- **MUST** be pure (no side effects)
- **SHOULD NOT** panic (undefined behavior if it does)
- **MAY** capture variables from enclosing scope
- **MUST** be safe to call multiple times with same pattern
- **MAY** call other pattern methods (length, size, depth, etc.)

## Memory Characteristics

### Stack Usage

- `any_value`: O(d) where d = maximum depth (recursive fold)
- `all_values`: O(d) where d = maximum depth (recursive fold)
- `filter`: O(d) where d = maximum depth (recursive traversal)

**Note**: All operations safe for patterns with 100+ nesting levels (tested in feature 009)

### Heap Usage

- `any_value`: O(1) - no allocation
- `all_values`: O(1) - no allocation
- `filter`: O(m) where m = number of matches (allocates vector)

### Performance Characteristics

| Operation | Time Complexity | Space Complexity | Short-Circuit |
|-----------|----------------|------------------|---------------|
| `any_value` | O(n) worst, O(1)-O(n) average | O(1) heap, O(d) stack | Yes (on first true) |
| `all_values` | O(n) worst, O(1)-O(n) average | O(1) heap, O(d) stack | Yes (on first false) |
| `filter` | O(n) | O(m) heap, O(d) stack | No (must visit all) |

Where:
- n = total number of nodes in pattern
- m = number of matching patterns
- d = maximum nesting depth

## Relationship to Existing Patterns

### Fold-Based Implementation

`any_value` and `all_values` leverage the existing `fold` method from feature 009:

```rust
// Existing fold signature (feature 009)
pub fn fold<B, F>(&self, init: B, f: F) -> B
where
    F: Fn(B, &V) -> B,
{
    // ... pre-order traversal with accumulator
}

// any_value implementation
pub fn any_value<F>(&self, predicate: F) -> bool
where
    F: Fn(&V) -> bool,
{
    self.fold(false, |acc, v| acc || predicate(v))
}

// all_values implementation
pub fn all_values<F>(&self, predicate: F) -> bool
where
    F: Fn(&V) -> bool,
{
    self.fold(true, |acc, v| acc && predicate(v))
}
```

**Benefits**:
- Reuses well-tested traversal logic
- Maintains consistent pre-order semantics
- Inherits performance characteristics

### Independent Implementation

`filter` requires custom implementation because:
- Needs access to entire `Pattern<V>`, not just values
- Returns collection of pattern references
- Cannot be expressed as a fold operation

## Type System Guarantees

### Compile-Time Guarantees

- **Type safety**: Predicates must match pattern's value type
- **Reference validity**: Returned references have correct lifetime bounds
- **Generic flexibility**: Works with any value type `V`
- **Zero-cost abstractions**: Monomorphization eliminates runtime overhead

### Lifetime Guarantees

```rust
// filter lifetime relationship
impl<V> Pattern<V> {
    pub fn filter<F>(&self, predicate: F) -> Vec<&Pattern<V>>
    where
        F: Fn(&Pattern<V>) -> bool,
    {
        // Returned references borrow from self
        // Valid as long as self is valid
    }
}

// Usage example
let pattern = Pattern::point(42);
let matches = pattern.filter(|p| p.value > 40);
// matches[0] borrows from pattern
// Cannot move/drop pattern while matches exist
```

## Behavioral Equivalence with Haskell

### Type Mappings

| Haskell | Rust | Notes |
|---------|------|-------|
| `v -> Bool` | `Fn(&V) -> bool` | Value predicate |
| `Pattern v -> Bool` | `Fn(&Pattern<V>) -> bool` | Pattern predicate |
| `Bool` | `bool` | Boolean result |
| `[Pattern v]` | `Vec<&Pattern<V>>` | Reference collection |

### Semantic Equivalence

- **Traversal order**: Both use pre-order (root, then elements)
- **Short-circuit behavior**: Rust's `||` and `&&` match Haskell's lazy evaluation
- **Empty pattern behavior**: Both follow same conventions (false/true for any/all)
- **Predicate application**: Same evaluation semantics

## Summary

This feature introduces no new data structures - it adds three methods to the existing `Pattern<V>` type. The key design elements are:

1. **Predicate abstraction**: Using `Fn` trait bounds for flexibility
2. **Reference returns**: Efficient access to matching patterns
3. **Fold reuse**: Leveraging existing infrastructure where possible
4. **Type safety**: Compile-time guarantees for correctness

All designs maintain behavioral equivalence with the Haskell reference implementation while following Rust idioms.

