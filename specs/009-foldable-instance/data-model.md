# Data Model: Foldable Instance for Pattern

**Feature**: 009-foldable-instance  
**Date**: 2026-01-04

## Overview

This document describes the data model for folding operations on `Pattern<V>`. Folding reduces a pattern structure to a single accumulated value by processing all values in the pattern in a predictable order.

## Core Types

### Pattern<V> (Existing)

The pattern data structure that will be folded.

```rust
pub struct Pattern<V> {
    pub value: V,
    pub elements: Vec<Pattern<V>>,
}
```

**Properties**:
- **value**: The root value at this pattern node
- **elements**: Child patterns (may be empty for atomic patterns)

**Role in Folding**:
- The pattern structure defines the traversal order
- Each `value` is processed exactly once
- Traversal is depth-first, root-first (pre-order)

---

### Accumulator Type (Generic)

The intermediate and final result type for fold operations.

**Type Parameter**: `B` (generic)

**Characteristics**:
- Independent of value type `V` (can be different)
- Threaded through the fold operation
- Updated at each value in the pattern
- Final accumulator value is the fold result

**Examples**:
- `i32` - for summing or counting
- `String` - for concatenating values
- `Vec<T>` - for collecting values
- `HashMap<K, V>` - for building indices
- `bool` - for validation checks

---

### Folding Function

The function that combines an accumulator with a value to produce a new accumulator.

**Type**: `F where F: Fn(B, &V) -> B`

**Signature**: `Fn(acc: B, value: &V) -> B`

**Parameters**:
- `acc: B` - Current accumulator value (owned, passed by value)
- `value: &V` - Reference to pattern value (borrowed, not consumed)

**Returns**: `B` - New accumulator value

**Properties**:
- Pure function (no side effects expected)
- Takes accumulator by value for efficiency (move semantics)
- Takes value by reference to avoid cloning
- Called once per value in the pattern

**Examples**:
```rust
// Sum integers
|acc, v| acc + v

// Concatenate strings
|acc, s| acc + s

// Count values
|count, _| count + 1

// Collect into vector
|mut vec, v| { vec.push(v.clone()); vec }

// Build index
|mut map, item| { map.insert(item.id(), item.clone()); map }
```

---

## Data Flow

### Fold Operation Flow

```
Input:
  - Pattern<V>: The pattern structure to fold
  - B: Initial accumulator value
  - F: Folding function Fn(B, &V) -> B

Process:
  1. Start with initial accumulator
  2. Process root value: acc = f(acc, &pattern.value)
  3. For each element (left to right):
       acc = element.fold_with(acc, f)  // Recursive
  4. Return final accumulator

Output:
  - B: Final accumulated value
```

### Traversal Order Example

Given pattern:
```rust
Pattern {
    value: "A",
    elements: [
        Pattern { value: "B", elements: [] },
        Pattern {
            value: "C",
            elements: [
                Pattern { value: "D", elements: [] }
            ]
        }
    ]
}
```

**Processing Order**:
1. Process "A" (root)
2. Process "B" (first element, atomic)
3. Process "C" (second element, root)
4. Process "D" (second element's child)

**Fold Execution**:
```rust
let init = String::new();
let f = |acc: String, v: &str| acc + v;

// Step by step:
acc = f(String::new(), "A")  // acc = "A"
acc = f(acc, "B")             // acc = "AB"  
acc = f(acc, "C")             // acc = "ABC"
acc = f(acc, "D")             // acc = "ABCD"

// Result: "ABCD"
```

---

## State Transformations

### Accumulator State Evolution

The accumulator evolves through the fold operation:

```
Initial State (B)
    ↓
Process Root Value
    ↓
Accumulator after root (B)
    ↓
Process Element 1 recursively
    ↓
Accumulator after element 1 (B)
    ↓
Process Element 2 recursively
    ↓
Accumulator after element 2 (B)
    ↓
...
    ↓
Final State (B)
```

### State Properties

- **Immutability**: Each step produces a new accumulator (functional style)
- **Threading**: Accumulator flows through all processing steps
- **Single-pass**: Each value processed exactly once
- **Order-dependent**: Final state depends on processing order

---

## Collection Conversion

### Values Collection

Converting a pattern to a vector of references:

**Method**: `values(&self) -> Vec<&V>`

**Data Flow**:
```
Pattern<V>
    ↓ (fold)
Vec<&V>
```

**Implementation**:
```rust
pub fn values(&self) -> Vec<&V> {
    self.fold(Vec::new(), |mut acc, v| {
        acc.push(v);
        acc
    })
}
```

**Properties**:
- Returns references (no cloning)
- Preserves traversal order
- O(n) time complexity
- O(n) space for result vector

---

## Type Relationships

### Generic Type Parameters

```rust
impl<V> Pattern<V> {
    pub fn fold<B, F>(&self, init: B, f: F) -> B
    where
        F: Fn(B, &V) -> B
    { ... }
}
```

**Type Variables**:
- `V`: Value type in the pattern (fixed for a given pattern instance)
- `B`: Accumulator type (chosen by caller, independent of V)
- `F`: Function type (inferred from closure)

**Type Constraints**:
- `F: Fn(B, &V) -> B` - Function must take B and &V, return B
- No constraints on `V` or `B` (fully generic)

### Type Flexibility Examples

```rust
// V = i32, B = i32 (same type)
let sum: i32 = pattern.fold(0, |acc, v| acc + v);

// V = String, B = usize (different types)
let total_length: usize = pattern.fold(0, |acc, s| acc + s.len());

// V = Person, B = Vec<String> (complex types)
let names: Vec<String> = pattern.fold(Vec::new(), |mut acc, person| {
    acc.push(person.name.clone());
    acc
});

// V = i32, B = bool (checking all values)
let all_positive: bool = pattern.fold(true, |acc, v| acc && *v > 0);
```

---

## Behavioral Contracts

### Fold Guarantees

1. **Completeness**: Every value in the pattern is processed exactly once
2. **Order**: Values processed in depth-first, root-first order
3. **Purity**: Pattern structure is not modified (borrows only)
4. **Type Safety**: Rust's type system ensures correct accumulator threading

### Invariants

**Pattern Invariants** (preserved):
- Pattern structure unchanged after fold
- All values remain accessible after fold
- Pattern can be folded multiple times

**Accumulator Invariants**:
- Type `B` maintained throughout fold
- Each step produces valid `B` value
- Function `f` called with correct types

---

## Performance Characteristics

### Time Complexity

- **fold**: O(n) where n = total number of values in pattern
- **values**: O(n) (uses fold internally)

### Space Complexity

- **fold**: O(d) stack space where d = maximum nesting depth
- **values**: O(n) for result vector + O(d) stack

### Scalability

**Tested Scales**:
- Patterns with 1000 nodes: < 10ms
- Patterns with 100 nesting levels: No stack overflow
- Patterns with 10,000 elements: < 100MB memory

---

## Integration with Pattern API

### Existing Pattern Methods (Used with Fold)

```rust
// Can be used to transform before folding
pattern.map(f).fold(init, g)

// Structure information
let size = pattern.size();  // Total values = fold count
let depth = pattern.depth();  // Max nesting

// Accessors
pattern.value()  // Root value (folded first)
pattern.elements()  // Child patterns (folded after root)
```

### Composability

Fold integrates with other Pattern operations:

```rust
// Map then fold
let result = pattern
    .map(|s| s.len())  // Transform values
    .fold(0, |acc, len| acc + len);  // Sum lengths

// Fold multiple times
let sum = pattern.fold(0, |acc, v| acc + v);
let product = pattern.fold(1, |acc, v| acc * v);
```

---

## Validation Rules

### Pattern Validation (Pre-existing)

Fold works on any valid pattern structure:
- Atomic patterns (no elements)
- Shallow patterns (one level)
- Deep patterns (many levels)
- Wide patterns (many siblings)

No additional validation needed for fold operations.

### Function Validation

The folding function must:
- Be callable with `(B, &V)` arguments
- Return a value of type `B`
- Ideally be pure (no side effects)

Rust's type system enforces these requirements at compile time.

---

## Testing Considerations

### Test Data Structures

**Test patterns**:
1. Atomic: `Pattern { value: v, elements: [] }`
2. Flat: `Pattern { value: root, elements: [p1, p2, p3] }` (all children atomic)
3. Nested: Multiple levels of nesting
4. Asymmetric: Some branches deep, others shallow
5. Wide: Many siblings at same level

### Test Accumulators

**Test accumulator types**:
1. Numeric: `i32`, `f64` (sum, product)
2. String: Concatenation
3. Collection: `Vec<V>`, `HashSet<V>`
4. Boolean: All/any predicates
5. Complex: Custom types

### Verification Points

1. **Order**: Use non-commutative operation (string concatenation)
2. **Completeness**: Count matches pattern.size()
3. **Correctness**: Results match expected values
4. **Type flexibility**: Fold to different types
5. **Performance**: Large patterns complete in time
6. **Memory**: No excessive allocations

---

## References

- **Haskell Foldable**: `Data.Foldable` typeclass
- **Rust Iterator**: `Iterator::fold` method pattern
- **Pattern Type**: `crates/pattern-core/src/pattern.rs`
- **Feature 008**: Functor instance (map operation) for integration examples

