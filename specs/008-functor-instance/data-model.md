# Data Model: Functor Instance for Pattern

**Feature**: 008-functor-instance  
**Date**: 2026-01-04

## Overview

This document describes the data transformation model for the Functor instance on `Pattern<V>`. The functor enables structure-preserving transformations that map values of type `V` to values of type `W` while maintaining the pattern's structural properties.

## Core Concepts

### Pattern Structure

A `Pattern<V>` consists of:
- **value**: A single value of type `V` (the "decoration")
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

### Transformation Model

The functor transformation applies a function `f: V → W` to create a new `Pattern<W>`:

```
      Pattern<V>                    Pattern<W>
      ┌─────────┐                   ┌─────────┐
      │ value:V │  ──f(&v)→W──→    │ value:W │
      ├─────────┤                   ├─────────┤
      │elements │                   │elements │
      │  ├─ P1  │  ──recurse──→    │  ├─ P1' │
      │  ├─ P2  │  ──recurse──→    │  ├─ P2' │
      │  └─ P3  │  ──recurse──→    │  └─ P3' │
      └─────────┘                   └─────────┘
```

**Key Properties**:
1. **Structure preservation**: Element count, nesting depth, and order unchanged
2. **Recursive application**: Function applied to all values at all levels
3. **Type transformation**: Pattern type changes from `Pattern<V>` to `Pattern<W>`

## Transformation Rules

### Rule 1: Value Transformation

The value at each node is transformed by applying the function:

```
transform_value(pattern, f) = f(&pattern.value)
```

- Function receives a **reference** to the value (`&V`)
- Function returns a **new value** (`W`)
- Original value is consumed (not mutated in place)

### Rule 2: Elements Transformation

The elements vector is transformed recursively:

```
transform_elements(pattern, f) = pattern.elements
    .into_iter()
    .map(|elem| elem.map(&f))
    .collect()
```

- Each element is transformed independently
- Same function `f` applied to all elements
- Recursion continues until atomic patterns (empty elements) reached

### Rule 3: Structure Invariants

The following structural properties MUST be preserved:

| Property | Definition | Preservation |
|----------|------------|--------------|
| **Element Count** | `pattern.length()` | MUST remain unchanged |
| **Nesting Depth** | `pattern.depth()` | MUST remain unchanged |
| **Total Size** | `pattern.size()` | MUST remain unchanged |
| **Element Order** | Order of elements in vector | MUST remain unchanged |
| **Atomicity** | Whether `elements.is_empty()` | MUST remain unchanged |

## Functor Laws

The transformation MUST satisfy two mathematical laws:

### Law 1: Identity

Transforming with the identity function produces an identical pattern:

```
pattern.map(|x| x.clone()) == pattern
```

**Formal**: `fmap id = id`

**Verification**:
```rust
let original = Pattern::point(42);
let transformed = original.clone().map(|x| x.clone());
assert_eq!(original, transformed);
```

### Law 2: Composition

Composition of transformations equals sequential application:

```
pattern.map(|x| g(&f(x))) == pattern.map(f).map(g)
```

**Formal**: `fmap (g ∘ f) = fmap g ∘ fmap f`

**Verification**:
```rust
let f = |x: &i32| x * 2;
let g = |x: &i32| x + 1;

let composed = pattern.clone().map(|x| g(&f(x)));
let sequential = pattern.map(f).map(g);

assert_eq!(composed, sequential);
```

## State Transitions

### Before Transformation

```
Pattern<String> {
    value: "root",
    elements: vec![
        Pattern {
            value: "child1",
            elements: vec![]
        },
        Pattern {
            value: "child2",
            elements: vec![]
        }
    ]
}
```

**Structural Properties**:
- Length: 2
- Depth: 1
- Size: 3
- Type: `Pattern<String>`

### During Transformation

Apply function `f: &String → usize` (string length):

```
1. Transform root value:    "root" → 4
2. Transform child1 value:   "child1" → 6  
3. Transform child2 value:   "child2" → 6
4. Reconstruct structure with new values
```

### After Transformation

```
Pattern<usize> {
    value: 4,
    elements: vec![
        Pattern {
            value: 6,
            elements: vec![]
        },
        Pattern {
            value: 6,
            elements: vec![]
        }
    ]
}
```

**Structural Properties** (unchanged):
- Length: 2
- Depth: 1
- Size: 3
- Type: `Pattern<usize>` (type changed, structure preserved)

## Edge Cases

### Atomic Pattern (No Elements)

```
Input:  Pattern { value: 42, elements: vec![] }
Func:   |x| x * 2
Output: Pattern { value: 84, elements: vec![] }
```

**Behavior**: Transformation applies only to root value. Atomicity preserved.

### Deep Nesting

```
Input:  Pattern { value: 1, elements: [
          Pattern { value: 2, elements: [
            Pattern { value: 3, elements: [] }
          ]}
        ]}
Func:   |x| x * 10
Output: Pattern { value: 10, elements: [
          Pattern { value: 20, elements: [
            Pattern { value: 30, elements: [] }
          ]}
        ]}
```

**Behavior**: Recursion reaches all depths. Structure preserved.

### Wide Branching

```
Input:  Pattern { value: 0, elements: [P1, P2, P3, ..., P1000] }
Func:   |x| x + 1
Output: Pattern { value: 1, elements: [P1', P2', P3', ..., P1000'] }
```

**Behavior**: All siblings transformed. Order preserved.

### Empty Pattern (Theoretical)

Note: `Pattern<V>` always has a value, so there's no "empty" pattern. The closest is an atomic pattern with zero elements.

## Performance Characteristics

### Time Complexity

- **Best case**: O(1) - Atomic pattern with no elements
- **Average case**: O(n) - Where n is total number of nodes
- **Worst case**: O(n) - Must visit every node

### Space Complexity

- **Pattern structure**: O(n) - New pattern created
- **Stack depth**: O(d) - Where d is maximum nesting depth
- **Total**: O(n + d)

### Memory Allocation

```
Before:  Pattern<V> with n nodes → memory M_v
After:   Pattern<W> with n nodes → memory M_w
Peak:    Both patterns exist during transformation
```

The original pattern is consumed (`self`), so peak memory is approximately `M_v + M_w` during transformation, then drops to `M_w` after completion.

## Relationship to gram-hs

### Haskell Implementation

```haskell
instance Functor Pattern where
  fmap f (Pattern v es) = Pattern (f v) (map (fmap f) es)
```

### Rust Implementation

```rust
pub fn map<W, F>(self, f: F) -> Pattern<W>
where
    F: Fn(&V) -> W,
{
    Pattern {
        value: f(&self.value),
        elements: self.elements.into_iter().map(|e| e.map(&f)).collect(),
    }
}
```

### Semantic Equivalence

| Aspect | Haskell | Rust | Notes |
|--------|---------|------|-------|
| **Structure preservation** | Yes | Yes | Both preserve structure |
| **Recursive application** | `map (fmap f) es` | `.map(\|e\| e.map(&f))` | Equivalent recursion |
| **Type transformation** | `Pattern v → Pattern w` | `Pattern<V> → Pattern<W>` | Same capability |
| **Identity law** | Satisfied | Satisfied | Tested via property tests |
| **Composition law** | Satisfied | Satisfied | Tested via property tests |

## Validation Rules

### Compile-Time Validation

The Rust type system enforces:
1. Function type matches: `F: Fn(&V) -> W`
2. Input pattern type: `Pattern<V>`
3. Output pattern type: `Pattern<W>`
4. Closure can be captured: `&F` is valid

### Runtime Validation

Property tests verify:
1. Identity law holds for all patterns
2. Composition law holds for all patterns
3. Structure invariants preserved
4. Performance requirements met

No explicit runtime validation needed in implementation (type system guarantees correctness).

## Summary

The functor transformation model provides:
- ✅ Structure-preserving value transformation
- ✅ Recursive application to nested patterns
- ✅ Type-safe transformations (V → W)
- ✅ Mathematical correctness (functor laws)
- ✅ Behavioral equivalence with gram-hs
- ✅ Idiomatic Rust implementation

This model enables developers to transform pattern values without manually traversing the tree structure, while maintaining all structural properties and satisfying functor laws.

