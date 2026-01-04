# Research: Foldable Instance for Pattern

**Feature**: 009-foldable-instance  
**Date**: 2026-01-04  
**Status**: Complete

## Research Questions & Decisions

### 1. Should we implement a Foldable trait or provide direct methods?

**Decision**: Provide direct methods on `Pattern<V>` (no separate trait)

**Rationale**:
- Rust's standard library doesn't have a Foldable trait
- Rust uses the `Iterator` trait for fold operations, but `Pattern` is not an iterator (it's a recursive tree structure)
- Direct methods are more idiomatic and practical for Rust developers
- Follows the same approach as the Functor instance (direct `map` method, no trait)
- Simpler API: Users call `pattern.fold(...)` directly without trait imports

**Alternatives Considered**:
- **Custom Foldable trait**: Would add abstraction but no practical benefit (Pattern is the only type that needs it in this crate). Rejected - adds complexity without value.
- **Implement Iterator**: Would provide `fold()` for free, but Pattern is not conceptually an iterator - it's a tree structure. Implementing Iterator would require flattening, which loses structural information. Rejected - doesn't match the domain model.
- **Implement IntoIterator**: Similar to Iterator approach, but Pattern shouldn't be consumed as a flat sequence. Rejected for same reasons.

**References**:
- Rust std library: `Iterator::fold`, `Iterator::reduce`
- Haskell Foldable: `foldMap`, `foldr`, `foldl`
- Previous decision for Functor: Direct `map` method (008-functor-instance)

---

### 2. What methods should be provided for folding?

**Decision**: Provide three core methods:
1. `fold<B, F>(&self, init: B, f: F) -> B` - Right fold with accumulator
2. `values(&self) -> Vec<&V>` - Collect all values into a Vec (convenience)
3. Consider `fold_map` for monoid-based folding (advanced)

**Rationale**:
- `fold` is the most versatile and matches Rust's `Iterator::fold` convention
- Right fold processes root first, then elements (matches Haskell's foldMap behavior)
- `values()` is a common use case (convert pattern to collection)
- Left fold can be simulated by collecting to Vec first, then folding the Vec
- Monoid-based fold (`fold_map`) may be added if needed but is less common in Rust

**Alternatives Considered**:
- **Only provide `fold`**: Too minimal, doesn't provide convenience for common case of collecting values. Rejected - not user-friendly.
- **Provide both `foldr` and `foldl`**: Rust uses `fold` (not `foldr`). Left fold can be achieved by collecting first. Rejected - adds confusion about naming.
- **Implement full Foldable interface (toList, null, length, etc.)**: Most of these are already available as Pattern methods (length, size). Rejected - redundant with existing API.

**References**:
- Rust std: `Iterator::fold`
- Haskell: `Data.Foldable.foldMap`, `Data.Foldable.foldr`
- gram-hs implementation: `foldMap f (Pattern v es) = f v <> foldMap (foldMap f) es`

---

### 3. What should the function signature be for `fold`?

**Decision**: 
```rust
pub fn fold<B, F>(&self, init: B, f: F) -> B
where
    F: Fn(B, &V) -> B,
```

**Rationale**:
- Takes `&self` (borrows pattern) - allows reuse of pattern after folding
- Generic accumulator type `B` - flexible, doesn't require `B` = `V`
- Closure takes accumulator by value and value by reference
- Matches Rust's `Iterator::fold` signature pattern
- Order: accumulator first (`B`), then value (`&V`) - follows Rust convention

**Alternatives Considered**:
- **`self` (consume pattern)**: Pattern would be unusable after fold. Rejected - too restrictive, folding shouldn't consume the structure.
- **`&mut self`**: No mutation needed for fold. Rejected - unnecessary.
- **`F: Fn(&V, B) -> B` (value first)**: Doesn't match Rust convention. Rejected - confusing for Rust developers.
- **`F: FnMut(B, &V) -> B`**: FnMut allows mutation but not needed. `Fn` is more general. Rejected - overly restrictive.

**References**:
- Rust std: `pub fn fold<B, F>(self, init: B, f: F) -> B where F: FnMut(B, Self::Item) -> B`
- Haskell: `foldr :: (a -> b -> b) -> b -> t a -> b`

---

### 4. How should we handle traversal order?

**Decision**: Depth-first, root-first order (pre-order traversal)
- Process pattern's root value first
- Then recursively process each element's values (left to right)
- For each element, process its root first, then its children

**Rationale**:
- Matches Haskell's `foldMap` implementation: `f v <> foldMap (foldMap f) es`
- Root value comes first in the pattern structure
- Consistent with the Pattern semantics: "value provides information about the elements"
- Predictable and intuitive for users
- Enables efficient implementation with tail-call optimization

**Visual Example**:
```
Pattern { value: "A", elements: [
  Pattern { value: "B", elements: [] },
  Pattern { value: "C", elements: [
    Pattern { value: "D", elements: [] }
  ]}
]}

Fold order: "A" → "B" → "C" → "D"
```

**Alternatives Considered**:
- **Post-order (children first, then parent)**: Doesn't match Haskell semantics. Rejected - not equivalent to reference implementation.
- **Level-order (breadth-first)**: More complex, doesn't match Haskell. Rejected - unnecessary complexity.
- **Elements before root**: Violates Pattern semantics where value provides context for elements. Rejected - semantically incorrect.

**References**:
- Haskell Foldable instance: Processes root value first
- Tree traversal algorithms: Pre-order, in-order, post-order
- gram-hs tests: Verify order with non-commutative operations (string concatenation)

---

### 5. How should we implement `values()` efficiently?

**Decision**: Provide `values(&self) -> Vec<&V>` that uses `fold` internally

**Rationale**:
- Leverages existing fold implementation (code reuse)
- Returns references to avoid cloning values
- Simple implementation: `self.fold(Vec::new(), |mut acc, v| { acc.push(v); acc })`
- Efficient: Single allocation, linear time O(n)

**Alternatives Considered**:
- **Return owned values `Vec<V>`**: Requires cloning, expensive. Rejected - unnecessary copies.
- **Return iterator**: More complex, requires custom iterator type. Rejected - fold approach is simpler and sufficient.
- **Separate implementation (not using fold)**: Code duplication. Rejected - fold is designed for this use case.

**References**:
- Rust std: `collect()` on iterators
- Haskell: `Data.Foldable.toList`

---

### 6. Should we support consuming fold (takes ownership)?

**Decision**: No, only borrowing fold for now

**Rationale**:
- Most fold operations don't need to consume values (counting, summing, collecting references)
- Borrowing is more flexible - pattern can be reused after fold
- If consuming is needed, users can clone and then fold
- Simpler API with one method
- Can add consuming variant later if needed (won't break existing code)

**Alternatives Considered**:
- **Provide both borrowing and consuming versions**: More API surface, added complexity. Deferred - can add later if demand exists.
- **Only consuming fold**: Too restrictive. Rejected - forces users to clone when borrowing would suffice.

---

### 7. How to handle closure capture for recursive fold?

**Decision**: Use helper method pattern (same as Functor implementation)
- Public method: `fold<B, F>(&self, init: B, f: F) -> B` takes closure by value
- Internal helper: `fold_with<B, F>(&self, acc: B, f: &F) -> B` takes closure by reference
- Recursive calls use `fold_with`

**Rationale**:
- Same pattern as `map`/`map_with` from Functor instance (consistency)
- Avoids closure cloning (no `F: Clone` bound needed)
- Ergonomic public API (users pass closure naturally)
- Efficient recursion (closure passed by reference internally)

**Implementation**:
```rust
pub fn fold<B, F>(&self, init: B, f: F) -> B
where
    F: Fn(B, &V) -> B,
{
    self.fold_with(init, &f)
}

fn fold_with<B, F>(&self, acc: B, f: &F) -> B
where
    F: Fn(B, &V) -> B,
{
    let acc = f(acc, &self.value);  // Process root
    self.elements.iter().fold(acc, |acc, elem| elem.fold_with(acc, f))  // Process elements
}
```

**Alternatives Considered**:
- **Direct closure in recursion**: Type errors with nested borrows. Rejected - doesn't compile.
- **Clone closure**: Requires `F: Clone` bound, inefficient. Rejected - unnecessary constraint.
- **Single method without helper**: Doesn't work with recursion. Rejected - compiler errors.

**References**:
- Feature 008 Functor implementation: `map` and `map_with` pattern
- Rust closure capturing rules

---

## Implementation Strategy Summary

### Core Design

**Methods to implement**:
1. `pub fn fold<B, F>(&self, init: B, f: F) -> B where F: Fn(B, &V) -> B`
   - Right fold with accumulator
   - Depth-first, root-first traversal order
   - Borrows pattern (doesn't consume)

2. `pub fn values(&self) -> Vec<&V>`
   - Convenience method to collect all values
   - Uses `fold` internally
   - Returns references to values

### Traversal Order

```
Pattern { value: root, elements: [e1, e2, e3] }

Fold processes:
1. root value
2. All values in e1 (recursively)
3. All values in e2 (recursively)  
4. All values in e3 (recursively)
```

### Behavioral Equivalence

**Haskell**:
```haskell
foldMap f (Pattern v es) = f v <> foldMap (foldMap f) es
```

**Rust equivalent**:
```rust
let acc = f(init, &self.value);  // f v
self.elements.iter().fold(acc, |acc, e| e.fold_with(acc, f))  // <> foldMap (foldMap f) es
```

### Key Properties

- **Order guarantee**: Root first, then elements left-to-right, depth-first
- **Structure agnostic**: Works on any pattern structure (atomic, nested, deep)
- **Type flexibility**: Accumulator type `B` independent of value type `V`
- **Borrowing**: Pattern remains usable after fold
- **Performance**: O(n) time where n = number of nodes, O(d) stack space where d = depth

### Testing Focus

1. **Order verification**: Verify traversal order with non-commutative operations (string concatenation, list building)
2. **Correctness**: Sum, count, collect operations produce expected results
3. **Structure coverage**: Atomic, shallow, deep, wide patterns
4. **Type flexibility**: Fold to different types (int → string, string → count)
5. **Performance**: Large patterns (1000+ nodes), deep patterns (100+ levels)
6. **Integration**: Compose with map and other operations

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Stack overflow on deep nesting | Low | High | Test with 100+ levels early; Rust has good stack management |
| Performance issues on large patterns | Low | Medium | Benchmark with 1000+ nodes; fold is inherently O(n) |
| Traversal order confusion | Medium | Medium | Clear documentation with examples; comprehensive tests |
| User expects different semantics | Low | Low | Document behavior clearly, provide examples |

## Next Steps

1. Implement `fold` and `fold_with` methods
2. Implement `values` method  
3. Add comprehensive documentation with examples
4. Port tests from gram-hs
5. Add property-based tests
6. Benchmark performance
7. Verify WASM compatibility

