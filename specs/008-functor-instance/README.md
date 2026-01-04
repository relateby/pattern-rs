# Feature 008: Functor Instance for Pattern

**Branch**: `008-functor-instance`  
**Status**: Planning Complete ✅  
**Type**: Idiomatic Rust Port

## Overview

This feature implements structure-preserving value transformation for `Pattern<V>` using an idiomatic Rust `map` method, maintaining behavioral equivalence with Haskell's Functor typeclass while following Rust standard library conventions.

## Key Documents

- **[spec.md](./spec.md)** - Feature specification (user stories, requirements, success criteria)
- **[plan.md](./plan.md)** - Technical implementation plan with phases
- **[IMPLEMENTATION_NOTES.md](./IMPLEMENTATION_NOTES.md)** - Design decision rationale
- **[checklists/requirements.md](./checklists/requirements.md)** - Specification quality validation

## Design Philosophy

This feature demonstrates the project's core principle: **port concepts and behavior, not syntax**.

### What We're Porting

✅ **Concept**: Structure-preserving value transformation (Functor)  
✅ **Behavior**: Maps function over all values recursively  
✅ **Laws**: Identity (`map id == id`) and composition (`map (f∘g) == map f ∘ map g`)  
✅ **Tests**: Property-based tests verifying functor laws

### What We're NOT Porting

❌ Haskell typeclass syntax  
❌ Function name `fmap` (using idiomatic `map` instead)  
❌ Typeclass abstractions (direct method is more Rust-idiomatic)

## API Design

### Idiomatic Rust Approach

```rust
impl<V> Pattern<V> {
    pub fn map<W, F>(self, f: F) -> Pattern<W>
    where
        F: Fn(&V) -> W,
    {
        // Implementation
    }
}
```

**Why this is idiomatic**:
- Matches standard library (`Option::map`, `Result::map`)
- Name Rust developers expect
- Simple to use and compose
- No trait complexity

### Usage Examples

```rust
// String transformation
let pattern = Pattern::point("hello");
let upper = pattern.map(|s| s.to_uppercase());

// Type conversion
let numbers = Pattern::point(42);
let strings = numbers.map(|n| n.to_string());

// Composition
let result = Pattern::point(5)
    .map(|n| n * 2)
    .map(|n| n + 1);
```

## Implementation Status

- [x] Specification complete
- [x] Plan complete
- [x] Design rationale documented
- [x] PORTING_GUIDE.md updated with idiomatic principles
- [ ] Phase 0: Research (to be done)
- [ ] Phase 1: Design details
- [ ] Phase 2: Core implementation
- [ ] Phase 3: Functor law testing
- [ ] Phase 4: Structure preservation verification
- [ ] Phase 5: Integration & documentation

## Success Criteria

All from [spec.md](./spec.md):

1. ✅ Functor laws verified (100+ property tests each)
2. ✅ Performance: <10ms for 1000-node patterns
3. ✅ Stack safety: 100+ nesting levels supported
4. ✅ Type transformations work correctly
5. ✅ All gram-hs tests ported and passing
6. ✅ WASM compilation succeeds
7. ✅ Memory efficiency: 10K elements under 100MB overhead

## References

- **Haskell Implementation**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` (lines 536-617)
- **Haskell Tests**: `../gram-hs/libs/pattern/tests/Spec/Pattern/Properties.hs` (lines 176-203)
- **Porting Guide**: `../../../PORTING_GUIDE.md` - Updated with idiomatic principles
- **Rust Patterns**: Standard library `map` methods for conventions

## Next Steps

Ready for implementation! Start with:
```bash
# Continue on feature branch
git checkout 008-functor-instance

# Begin implementation
# See plan.md for phase breakdown
```

