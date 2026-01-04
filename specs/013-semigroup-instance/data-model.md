# Data Model: Pattern Combination Operations

**Feature**: 013-semigroup-instance  
**Date**: 2026-01-04

## Overview

This document describes the data model for pattern combination operations. The combination operation is a binary associative operation that merges two Pattern<V> instances into a single Pattern<V>, combining their values and concatenating their elements.

## Core Concepts

### Pattern Structure

A `Pattern<V>` consists of:
- **value**: A single value of type `V` (the "decoration" or "information about elements")
- **elements**: A vector of nested `Pattern<V>` (recursive structure)

```
Pattern<V>
├── value: V
└── elements: Vec<Pattern<V>>
    ├── Pattern<V>
    │   ├── value: V
    │   └── elements: Vec<Pattern<V>>
    └── ...
```

### Combination Model

The combination operation merges two patterns into one:

```
   Pattern<V> (p1)           Pattern<V> (p2)
   ┌─────────────┐          ┌─────────────┐
   │ value: v1   │    +     │ value: v2   │
   ├─────────────┤          ├─────────────┤
   │ elements:   │          │ elements:   │
   │  ├─ e1      │          │  ├─ e3      │
   │  └─ e2      │          │  └─ e4      │
   └─────────────┘          └─────────────┘
           │                        │
           └────── combine() ───────┘
                      │
                      ↓
               Pattern<V> (result)
               ┌─────────────────┐
               │ value: v1⊕v2    │  ← values combined
               ├─────────────────┤
               │ elements:       │
               │  ├─ e1          │  ← left pattern elements
               │  ├─ e2          │     (order preserved)
               │  ├─ e3          │  ← right pattern elements
               │  └─ e4          │     (order preserved)
               └─────────────────┘
```

**Key Properties**:
1. **Value Combination**: Values are combined using V's Combinable trait
2. **Element Concatenation**: Elements are concatenated (left first, then right)
3. **Order Preservation**: Element order is preserved from both input patterns
4. **Associativity**: (a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)

## Entities

### Pattern<V>

The recursive pattern structure being combined.

**Fields**:
- `value: V` - The value component
- `elements: Vec<Pattern<V>>` - The nested pattern elements

**Constraints**:
- V must implement `Combinable` trait

**Operations**:
- `combine(self, other: Self) -> Self` - Combines two patterns

### Combinable Trait

Trait for types that support associative combination.

**Purpose**: Expresses that a type can be combined with another instance of itself in an associative way.

**Requirements**:
- Method: `fn combine(self, other: Self) -> Self`
- Law: Associativity must hold: `(a.combine(b)).combine(c) == a.combine(b.combine(c))`

**Standard Implementations**:
- `String`: Concatenation (`s1 + &s2`)
- `Vec<T>`: Concatenation (`v1.extend(v2)`)
- `()`: Trivial (returns `()`)

## Combination Algorithm

### Step-by-Step Process

```
combine(p1: Pattern<V>, p2: Pattern<V>) -> Pattern<V>:
  
  1. Combine Values
     combined_value = p1.value.combine(p2.value)
     
  2. Concatenate Elements
     combined_elements = p1.elements ++ p2.elements
     
  3. Create Result Pattern
     return Pattern {
       value: combined_value,
       elements: combined_elements
     }
```

### Pseudocode

```rust
pub fn combine(self, other: Self) -> Self {
    // Step 1: Combine values using V's Combinable implementation
    let combined_value = self.value.combine(other.value);
    
    // Step 2: Concatenate elements (left first, then right)
    let mut combined_elements = self.elements;
    combined_elements.extend(other.elements);
    
    // Step 3: Return new pattern
    Pattern {
        value: combined_value,
        elements: combined_elements,
    }
}
```

### Complexity

- **Time Complexity**: O(|elements1| + |elements2| + value_combine_cost)
  - Element concatenation: O(|elements1| + |elements2|)
  - Value combination: Depends on V's combine implementation
- **Space Complexity**: O(|elements1| + |elements2|)
  - New pattern with combined elements

## Examples

### Example 1: Atomic Patterns (No Elements)

```rust
let p1 = Pattern::point("hello");  // value: "hello", elements: []
let p2 = Pattern::point(" world"); // value: " world", elements: []

let result = p1.combine(p2);
// result: Pattern { value: "hello world", elements: [] }
```

### Example 2: Patterns with Elements

```rust
let p1 = Pattern::pattern("a", vec![
    Pattern::point("b"),
    Pattern::point("c"),
]);
// p1: Pattern { value: "a", elements: [b, c] }

let p2 = Pattern::pattern("d", vec![
    Pattern::point("e"),
]);
// p2: Pattern { value: "d", elements: [e] }

let result = p1.combine(p2);
// result: Pattern { value: "ad", elements: [b, c, e] }
```

### Example 3: Associativity Demonstration

```rust
let a = Pattern::point(1);
let b = Pattern::point(2);
let c = Pattern::point(3);

// Left association: (a ⊕ b) ⊕ c
let left = a.clone().combine(b.clone()).combine(c.clone());
// Step 1: (1 ⊕ 2) = 3
// Step 2: 3 ⊕ 3 = 6

// Right association: a ⊕ (b ⊕ c)
let right = a.combine(b.combine(c));
// Step 1: (2 ⊕ 3) = 5
// Step 2: 1 ⊕ 5 = 6

assert_eq!(left, right); // 6 == 6 ✓
```

### Example 4: Multiple Pattern Combination (Fold)

```rust
let patterns = vec![
    Pattern::point("a"),
    Pattern::point("b"),
    Pattern::point("c"),
];

let result = patterns.into_iter()
    .reduce(|acc, p| acc.combine(p))
    .unwrap();

// result: Pattern { value: "abc", elements: [] }
```

### Example 5: Nested Pattern Combination

```rust
let p1 = Pattern::pattern("root1", vec![
    Pattern::pattern("child1", vec![
        Pattern::point("leaf1"),
    ]),
]);

let p2 = Pattern::pattern("root2", vec![
    Pattern::pattern("child2", vec![
        Pattern::point("leaf2"),
    ]),
]);

let result = p1.combine(p2);
// result: Pattern {
//   value: "root1root2",
//   elements: [
//     Pattern { value: "child1", elements: [leaf1] },
//     Pattern { value: "child2", elements: [leaf2] }
//   ]
// }
```

## Associativity Property

### Mathematical Definition

For all patterns `a`, `b`, `c`:
```
(a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)
```

### Why It Holds

1. **Value Combination Associativity**:
   - If V's combine operation is associative
   - Then value combination is associative

2. **Element Concatenation Associativity**:
   - List concatenation is inherently associative
   - `(list1 ++ list2) ++ list3 = list1 ++ (list2 ++ list3)`

3. **Combined Associativity**:
   - Since both value combination and element concatenation are associative
   - The overall pattern combination is associative

### Proof Sketch

```
Let: a = Pattern(v1, [e1...]), b = Pattern(v2, [e2...]), c = Pattern(v3, [e3...])

Left Association: (a ⊕ b) ⊕ c
  = Pattern(v1 ⊕ v2, [e1...] ++ [e2...]) ⊕ c
  = Pattern((v1 ⊕ v2) ⊕ v3, ([e1...] ++ [e2...]) ++ [e3...])
  = Pattern(v1 ⊕ (v2 ⊕ v3), [e1...] ++ ([e2...] ++ [e3...])) [by associativity]

Right Association: a ⊕ (b ⊕ c)
  = a ⊕ Pattern(v2 ⊕ v3, [e2...] ++ [e3...])
  = Pattern(v1 ⊕ (v2 ⊕ v3), [e1...] ++ ([e2...] ++ [e3...]))

Therefore: (a ⊕ b) ⊕ c = a ⊕ (b ⊕ c) ✓
```

## Edge Cases

### Empty Elements

```rust
let p1 = Pattern::point("a");  // elements: []
let p2 = Pattern::point("b");  // elements: []

let result = p1.combine(p2);
// result: Pattern { value: "ab", elements: [] }
// Empty lists concatenate to empty list
```

### One Pattern Empty, One With Elements

```rust
let p1 = Pattern::point("a");  // elements: []
let p2 = Pattern::pattern("b", vec![Pattern::point("c")]);  // elements: [c]

let result = p1.combine(p2);
// result: Pattern { value: "ab", elements: [c] }
// [] ++ [c] = [c]
```

### Deep Nesting

```rust
// Deep patterns combine without special handling
// Combination happens at the top level only
// Nested structures remain intact
```

### Large Element Counts

```rust
// Efficient element concatenation using Vec::extend
// O(n) where n is total element count
// No special handling needed
```

## Relationships

### Pattern<V> Dependencies

```
Pattern<V>
    ↓ requires
Combinable on V
    ↓ provides
combine() operation
    ↓ guarantees
Associativity property
```

### Integration with Existing Operations

```
Pattern Operations:
├── Construction: point(), pattern()
├── Transformation: map()
├── Reduction: fold()
├── Traversal: traverse()
└── Combination: combine() ← NEW
```

## Type Constraints

### For Pattern Combination

```rust
impl<V: Combinable> Pattern<V> {
    pub fn combine(self, other: Self) -> Self
}
```

**Constraint**: V must implement Combinable

**Rationale**: Values need to be combined, so V must support combination

### For Combinable Trait

```rust
pub trait Combinable {
    fn combine(self, other: Self) -> Self;
}
```

**No constraints**: Base trait with no super traits

**Invariant**: Implementations must be associative (documented, tested, not enforced by type system)

## Performance Characteristics

### Best Case
- Atomic patterns (no elements)
- Simple value combination (O(1))
- **Total**: O(1)

### Average Case
- Patterns with moderate element counts (10-100)
- String concatenation value combination (O(n))
- **Total**: O(element_count + string_length)

### Worst Case
- Large element vectors (1000+ elements)
- Expensive value combination
- **Total**: O(elements1 + elements2 + value_combine_cost)

### Memory
- Allocates new Pattern
- Extends element vector (may reallocate)
- Value combination allocation (depends on V)
- **Total**: O(elements1 + elements2 + value_size)

## Summary

The pattern combination operation provides:
1. **Simple semantics**: Value combination + element concatenation
2. **Mathematical property**: Associativity guaranteed
3. **Type safety**: Only available when V implements Combinable
4. **Efficiency**: O(n) where n is total element count
5. **Idiomatic Rust**: Concrete method, moves ownership, follows existing patterns

