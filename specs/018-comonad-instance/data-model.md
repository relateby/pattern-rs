# Data Model: Comonad Operations on Pattern

**Feature**: 018-comonad-instance  
**Date**: 2026-01-05

## Overview

This document describes the Pattern data structure and how Comonad operations interact with it. Pattern's "decorated sequence" semantics (value decorates elements) make it a natural Comonad, where operations work with decoration as contextual information.

## Core Data Structure

### Pattern<V>

```rust
pub struct Pattern<V> {
    /// The decorative value - provides information ABOUT the elements
    pub value: V,
    
    /// The elements - these ARE the pattern content
    pub elements: Vec<Pattern<V>>,
}
```

**Semantics**:
- **Elements**: The actual pattern content (the "sequence")
- **Value**: Decorative information about those elements (the "decoration")
- This "decorated sequence" model is what makes Pattern a Comonad

**Example**:
```rust
Pattern {
    value: "sonata",           // Information ABOUT the pattern
    elements: [                // The pattern itself
        Pattern { value: "A", elements: [] },
        Pattern { value: "B", elements: [] },
        Pattern { value: "A", elements: [] },
    ]
}
```

The elements `["A", "B", "A"]` form the pattern. The value `"sonata"` decorates it with information.

## Comonad Operations

### extract

**Purpose**: Access the decorative value at the current position.

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn extract(&self) -> &V
}
```

**Semantics**:
- Returns a reference to the `value` field
- O(1) operation
- No structural changes
- Pure accessor (no side effects)

**Mental Model**: "What decoration is at this position?"

**Example**:
```rust
let p = Pattern { value: 42, elements: vec![] };
assert_eq!(p.extract(), &42);
```

### extend

**Purpose**: Compute new decorative information at every position based on the subpattern context.

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn extend<W, F>(&self, f: &F) -> Pattern<W>
    where
        F: Fn(&Pattern<V>) -> W
}
```

**Semantics**:
- Applies function `f` to every subpattern in the tree
- Function `f` receives the full subpattern (not just the value)
- Returns new pattern with same structure, decorated with computed values
- O(n) operation where n = number of nodes
- Pure function (no side effects)

**Mental Model**: "At each position, compute new decoration based on what's there."

**Example**:
```rust
// Decorate each position with its depth
let p = Pattern {
    value: "root",
    elements: vec![
        Pattern { value: "a", elements: vec![
            Pattern { value: "x", elements: vec![] }
        ]},
        Pattern { value: "b", elements: vec![] }
    ]
};

let depths = p.extend(&|subpattern| subpattern.depth());
// Result:
// Pattern {
//     value: 2,                  // root has depth 2
//     elements: [
//         Pattern { value: 1, elements: [
//             Pattern { value: 0, elements: [] }
//         ]},
//         Pattern { value: 0, elements: [] }
//     ]
// }
```

**Key Property**: Function sees full subpattern, enabling context-aware computation.

## Helper Functions

These demonstrate practical applications of Comonad operations.

### depth_at

**Purpose**: Decorate each position with the depth of its subpattern.

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn depth_at(&self) -> Pattern<usize>
}
```

**Implementation Strategy**: Uses `extend` with `depth` function
```rust
pub fn depth_at(&self) -> Pattern<usize> {
    self.extend(&|p| p.depth())
}
```

**Semantics**:
- Depth = maximum nesting level below this position
- Atomic patterns (no elements) have depth 0
- Pattern with elements has depth = 1 + max(child depths)

**Example**:
```rust
let p = point("x");  // atomic
assert_eq!(p.depth_at().extract(), &0);

let p = Pattern {
    value: "root",
    elements: vec![point("a"), point("b")]
};
assert_eq!(p.depth_at().extract(), &0);  // children are atomic

let p = Pattern {
    value: "root",
    elements: vec![
        Pattern { value: "a", elements: vec![point("x")] },
        point("b")
    ]
};
assert_eq!(p.depth_at().extract(), &1);  // "a" has child, so root depth is 1
```

### size_at

**Purpose**: Decorate each position with the total number of nodes in its subtree.

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn size_at(&self) -> Pattern<usize>
}
```

**Implementation Strategy**: Uses `extend` with `size` function
```rust
pub fn size_at(&self) -> Pattern<usize> {
    self.extend(&|p| p.size())
}
```

**Semantics**:
- Size = 1 (self) + sum of child sizes
- Atomic patterns have size 1
- Useful for identifying "heavy" subtrees

**Example**:
```rust
let p = point("x");  // atomic
assert_eq!(p.size_at().extract(), &1);

let p = Pattern {
    value: "root",
    elements: vec![point("a"), point("b")]
};
assert_eq!(p.size_at().extract(), &3);  // 1 + 1 + 1

let p = Pattern {
    value: "root",
    elements: vec![
        Pattern { value: "a", elements: vec![point("x")] },
        point("b")
    ]
};
assert_eq!(p.size_at().extract(), &4);  // 1 + 2 + 1
```

### indices_at

**Purpose**: Decorate each position with its path from root (sequence of element indices).

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn indices_at(&self) -> Pattern<Vec<usize>>
}
```

**Implementation Strategy**: Direct recursion with path accumulation (cannot use `extend`)

**Reason**: Requires tracking path during traversal. `extend` only sees local subpattern, not path from root.

**Semantics**:
- Root has empty path `[]`
- Child at index `i` has path `parent_path + [i]`
- Useful for addressing specific positions

**Example**:
```rust
let p = point("x");  // atomic
assert_eq!(p.indices_at().extract(), &vec![]);

let p = Pattern {
    value: "root",
    elements: vec![point("a"), point("b")]
};
let result = p.indices_at();
assert_eq!(result.extract(), &vec![]);
assert_eq!(result.elements[0].extract(), &vec![0]);
assert_eq!(result.elements[1].extract(), &vec![1]);

let p = Pattern {
    value: "root",
    elements: vec![
        Pattern { value: "a", elements: vec![point("x")] },
        point("b")
    ]
};
let result = p.indices_at();
assert_eq!(result.extract(), &vec![]);
assert_eq!(result.elements[0].extract(), &vec![0]);
assert_eq!(result.elements[0].elements[0].extract(), &vec![0, 0]);
assert_eq!(result.elements[1].extract(), &vec![1]);
```

## Comonad Laws

These are mathematical properties that `extract` and `extend` must satisfy.

### Law 1: Left Identity

**Specification**: `extract(extend(f, p)) == f(p)`

**English**: Extracting after extending with `f` gives the same result as calling `f` directly.

**Why**: The root value of `extend(f, p)` is `f(p)` by definition of `extend`.

**Verification**: Property-based test

### Law 2: Right Identity

**Specification**: `extend(extract, p) == p`

**English**: Extending with `extract` returns the pattern unchanged.

**Why**: At each position, we're computing the decoration using `extract`, which just returns the existing decoration.

**Verification**: Property-based test

### Law 3: Associativity

**Specification**: `extend(f, extend(g, p)) == extend(f ∘ extend(g), p)`

**English**: Extending twice is the same as extending once with the composed function.

**Why**: The order of applying context-aware transformations shouldn't matter.

**Verification**: Property-based test

## State Transitions

**Note**: Comonad operations are pure - they don't mutate state. They produce new patterns.

### Pattern Structure Preservation

All Comonad operations preserve the structure:
- Same number of nodes
- Same tree shape
- Only values change

**Example**:
```rust
let p = Pattern {
    value: "a",
    elements: vec![
        Pattern { value: "b", elements: vec![point("c")] },
        point("d")
    ]
};

let p2 = p.extend(&|p| p.size());
// Structure preserved:
// - Still has 2 elements
// - First element still has 1 element
// - Only values changed (strings → numbers)
```

## Validation Rules

### extract

**Preconditions**: None (always valid)
**Postconditions**: Returns reference to value field

### extend

**Preconditions**: 
- Function `f` must be pure (no side effects)
- Function `f` must terminate for all patterns

**Postconditions**:
- Output pattern has same structure as input
- Output node count equals input node count
- Comonad laws hold

### Helper Functions

**Preconditions**: None (all patterns valid)
**Postconditions**: 
- Structure preserved
- Specific decoration computed correctly:
  - `depth_at`: depths are accurate
  - `size_at`: sizes are accurate
  - `indices_at`: paths are accurate

## Relationships

### To Existing Pattern Operations

| Operation | Type | Interaction with Comonad |
|-----------|------|--------------------------|
| `map` | Functor | Can map over decorated pattern |
| `fold` | Foldable | Can fold decorated pattern |
| `traverse` | Traversable | Can traverse decorated pattern |
| `filter` | Query | Can filter decorated pattern |
| `combine` | Semigroup | Can combine decorated patterns |

**Composability Example**:
```rust
// Compute depths, then sum them
let total_depth: usize = pattern
    .depth_at()           // Pattern<usize>
    .fold(0, |acc, d| acc + d);  // usize

// Compute sizes, then filter large subtrees
let large_subtrees = pattern
    .size_at()            // Pattern<usize>
    .filter(|p| *p.extract() > 10);  // Pattern<usize>
```

## Performance Characteristics

| Operation | Time | Space | Notes |
|-----------|------|-------|-------|
| `extract` | O(1) | O(1) | Direct field access |
| `extend` | O(n) | O(n) | Single traversal, new pattern created |
| `depth_at` | O(n) | O(n) | Uses extend |
| `size_at` | O(n) | O(n) | Uses extend |
| `indices_at` | O(n) | O(n) | Direct recursion with path accumulation |

Where n = number of nodes in the pattern.

**Memory**: All operations create new patterns (no mutation). Input pattern unchanged.

## Examples

### Use Case 1: Pattern Visualization

```rust
// Annotate pattern with metadata for visualization
struct VisualMeta {
    depth: usize,
    size: usize,
    path: Vec<usize>,
}

fn annotate(p: &Pattern<String>) -> Pattern<VisualMeta> {
    let depths = p.depth_at();
    let sizes = p.size_at();
    let paths = p.indices_at();
    
    // Combine using extend
    p.extend(&|subp| {
        let d = depths./* navigate to matching position */;
        let s = sizes./* navigate to matching position */;
        let path = paths./* navigate to matching position */;
        VisualMeta { depth: d, size: s, path }
    })
}
```

### Use Case 2: Structural Analysis

```rust
// Find deepest and largest subtrees
fn analyze(p: &Pattern<String>) {
    let depths = p.depth_at();
    let sizes = p.size_at();
    
    let max_depth = depths.fold(0, |max, &d| max.max(d));
    let max_size = sizes.fold(0, |max, &s| max.max(s));
    
    println!("Max depth: {}, Max size: {}", max_depth, max_size);
}
```

### Use Case 3: Custom Context-Aware Decoration

```rust
// Compute custom metric at each position
fn balance_factor(p: &Pattern<String>) -> Pattern<f64> {
    p.extend(&|subp| {
        if subp.elements.is_empty() {
            1.0
        } else {
            let sizes: Vec<usize> = subp.elements.iter()
                .map(|e| e.size())
                .collect();
            let avg = sizes.iter().sum::<usize>() as f64 / sizes.len() as f64;
            let variance = sizes.iter()
                .map(|&s| (s as f64 - avg).powi(2))
                .sum::<f64>() / sizes.len() as f64;
            1.0 / (1.0 + variance)  // Higher balance = lower variance
        }
    })
}
```

## Migration Notes

**From existing code**: No migration needed. All new operations are additive.

**New capabilities**:
- Access decorative value: `p.extract()` (clearer than `p.value()`)
- Context-aware transformations: `p.extend(&f)`
- Position metadata: `p.depth_at()`, `p.size_at()`, `p.indices_at()`

## References

- **Pattern definition**: `crates/pattern-core/src/pattern/mod.rs`
- **Haskell reference**: `../pattern-hs/libs/pattern/src/Pattern/Core.hs` (lines 720-728)
- **Feature spec**: [spec.md](./spec.md)
- **API contracts**: [contracts/comonad.md](./contracts/comonad.md)
