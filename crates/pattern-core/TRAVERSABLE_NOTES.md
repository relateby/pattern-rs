# Traversable Implementation Notes

**Feature**: 010-traversable-instance  
**Date**: 2026-01-04  
**Purpose**: Document key insights from gram-hs Traversable instance for Rust port

## Haskell Reference Implementation

**Location**: `../pattern-hs/libs/pattern/src/Pattern/Core.hs`

### Traversable Instance (Haskell)

```haskell
instance Traversable Pattern where
  traverse :: Applicative f => (a -> f b) -> Pattern a -> f (Pattern b)
```

**Key Insights**:
1. **Generic over any Applicative functor** - Works with Maybe, Either, IO, etc.
2. **Structure preservation** - Guaranteed by typeclass laws
3. **Effect sequencing** - Applicative constraint handles combining effects
4. **Traversal order** - Depth-first, root-first (same as Foldable)

### Rust Adaptation Strategy

**Design Decision**: Use concrete methods instead of generic trait

**Rationale**:
- Rust lacks Higher-Kinded Types (HKTs)
- Generic Traversable trait would require complex type-level gymnastics
- Concrete methods are more idiomatic and usable in Rust
- Better error messages and type inference

**Methods to Implement**:
1. `traverse_option<W, F>(&self, f: F) -> Option<Pattern<W>>`
2. `traverse_result<W, E, F>(&self, f: F) -> Result<Pattern<W>, E>`
3. `traverse_future<W, E, F>(&self, f: F) -> Future<Result<Pattern<W>, E>>` (feature-gated)
4. `validate<W, E, F>(&self, f: F) -> Result<Pattern<W>, Vec<E>>` (extension: collect all errors)
5. `sequence_option<W>(&self) -> Option<Pattern<W>>` (convenience)
6. `sequence_result<W, E>(&self) -> Result<Pattern<W>, E>` (convenience)

## Traversal Order Requirements

**Order**: Depth-first, root-first (pre-order traversal)

**Guarantee**: For pattern with root value V and elements [E1, E2, E3]:
1. Process V first (root value)
2. Process all values from E1 recursively
3. Process all values from E2 recursively
4. Process all values from E3 recursively

**Implementation Pattern** (from Haskell):
```haskell
traverse f (Pattern v es) = Pattern <$> f v <*> traverse (traverse f) es
```

**Rust Equivalent** (for Option):
```rust
let new_value = f(&self.value)?;  // Apply to root first
let new_elements: Option<Vec<Pattern<W>>> = self.elements
    .iter()
    .map(|elem| elem.traverse_option_with(f))  // Recursively to elements
    .collect();  // collect() handles Option sequencing
Some(Pattern { value: new_value, elements: new_elements? })
```

## Effect Sequencing

### Option Sequencing
- Uses `Option`'s short-circuit behavior via `?` operator
- `Iterator::collect::<Option<Vec<_>>>()` stops on first `None`
- Semantics: All Some → Some(result), any None → None

### Result Sequencing
- Uses `Result`'s short-circuit behavior via `?` operator
- `Iterator::collect::<Result<Vec<_>, E>>()` stops on first `Err`
- Semantics: All Ok → Ok(result), first Err → Err(e)

### Validate (No Short-circuit)
- Must process ALL values, collecting errors
- Cannot use `collect()` with short-circuit
- Manual accumulation in Vec<E>
- Semantics: All Ok → Ok(result), any Err → Err(vec![...all errors...])

## Implementation Pattern (Helper Methods)

**Borrowed from map and fold**:

Public method takes `F` by value (ergonomic):
```rust
pub fn traverse_option<W, F>(&self, f: F) -> Option<Pattern<W>>
where F: Fn(&V) -> Option<W>
{
    self.traverse_option_with(&f)
}
```

Internal helper takes `&F` by reference (efficient):
```rust
fn traverse_option_with<W, F>(&self, f: &F) -> Option<Pattern<W>>
where F: Fn(&V) -> Option<W>
{
    // Implementation with recursive calls using f: &F
}
```

**Why**: Avoids cloning closure on each recursive call, no `Clone` bound needed

## Traversable Laws

**From Haskell** (must be preserved in spirit, adapted for Rust):

### Identity Law
```haskell
traverse Identity = Identity
```

**Rust adaptation**:
```rust
pattern.traverse_option(|v| Some(*v)) == Some(pattern.clone())
```

### Composition Law
```haskell
traverse (Compose . fmap g . f) = Compose . fmap (traverse g) . traverse f
```

**Rust**: Hard to express generically without HKTs, test observable properties

### Naturality Law
```haskell
t . traverse f = traverse (t . f)  -- for natural transformation t
```

**Rust**: Test with concrete natural transformations (e.g., Option to Result)

### Structure Preservation (Property)
```rust
// If traversal succeeds, structure is unchanged
pattern.traverse_option(f).map(|p| p.size()) == Some(pattern.size())
pattern.traverse_option(f).map(|p| p.depth()) == Some(pattern.depth())
```

## Behavioral Equivalence

**Key requirements from gram-hs**:
1. ✅ Process values in depth-first, root-first order
2. ✅ Structure preservation (size, depth, element count)
3. ✅ Effect sequencing matches applicative semantics
4. ✅ All values processed exactly once (if no short-circuit)
5. ✅ Short-circuit on first error/None (for Result/Option)

## Performance Considerations

**From plan.md targets**:
- <50ms for patterns with 1000 nodes
- Support 100+ nesting levels without stack overflow
- <100MB memory for 10,000 elements

**Implementation notes**:
- Recursive calls use O(d) stack space where d = depth
- Each recursive level allocates new Pattern<W> (heap)
- Total allocations: O(n) where n = pattern size
- Effect overhead depends on effect type (Option/Result: minimal, Future: async runtime)

## Testing Strategy

**From gram-hs tests** (`../pattern-hs/libs/pattern/tests/Spec/Pattern/Properties.hs`):

1. **Property tests** (proptest, 100+ cases):
   - Identity law for each effect type
   - Structure preservation for each effect type
   - Naturality law where applicable

2. **Unit tests**:
   - Atomic patterns (no elements)
   - Nested patterns (multiple levels)
   - All-success cases
   - Failure cases (short-circuit verification)
   - Edge cases (empty, deep nesting, wide patterns)

3. **Integration tests**:
   - Composition with map (Functor)
   - Composition with fold (Foldable)
   - Complex pipelines

## References

- **Haskell Implementation**: `../pattern-hs/libs/pattern/src/Pattern/Core.hs`
- **Haskell Tests**: `../pattern-hs/libs/pattern/tests/Spec/Pattern/Properties.hs`
- **Research**: `specs/010-traversable-instance/research.md`
- **Data Model**: `specs/010-traversable-instance/data-model.md`
- **Type Signatures**: `specs/010-traversable-instance/contracts/type-signatures.md`
