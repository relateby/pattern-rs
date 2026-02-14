# Research & Technical Decisions: Comonad Operations

**Feature**: 018-comonad-instance  
**Date**: 2026-01-05  
**Status**: Complete

## Overview

This document captures research findings and technical decisions made during planning for Comonad operations on Pattern. All "NEEDS CLARIFICATION" items from Technical Context have been resolved.

## Research Questions & Resolutions

### 1. Implementation Strategy: Direct Methods vs. Trait Abstraction

**Question**: Should we implement Comonad as a trait or as direct methods on Pattern?

**Decision**: **Direct methods on `impl<V> Pattern<V>`**

**Rationale**:
- Pattern is the only type that will implement Comonad in this codebase
- Rust doesn't have Higher-Kinded Types (HKTs), making generic Comonad trait awkward
- Direct methods are more idiomatic Rust for single-type use cases
- Users can call `pattern.extract()` and `pattern.extend(f)` directly
- No need for trait imports or generic abstractions
- Simpler type signatures and error messages

**Alternatives Considered**:
1. **Trait-based Comonad**:
   ```rust
   trait Comonad {
       type Value;
       fn extract(&self) -> &Self::Value;
       fn extend<W, F>(&self, f: F) -> /* ??? */;
   }
   ```
   - Rejected: No other types need Comonad, adds unnecessary abstraction
   - Rust lacks HKTs, making `extend` return type specification awkward

2. **Separate Comonad module with free functions**:
   ```rust
   mod comonad {
       pub fn extract<V>(p: &Pattern<V>) -> &V { ... }
       pub fn extend<V, W, F>(p: &Pattern<V>, f: F) -> Pattern<W> { ... }
   }
   ```
   - Rejected: Less discoverable than methods, breaks IDE autocompletion flow

**References**:
- Rust API Guidelines: Prefer methods over free functions for discoverability
- pattern-rs precedent: Functor (map), Foldable (fold), etc. implemented as direct methods

### 2. Function Passing Strategy: By Reference vs. By Value

**Question**: Should `extend` take function parameter by value `F` or by reference `&F`?

**Decision**: **By reference `&F`** for primary implementation

**Rationale**:
- No Clone bound required on function type
- More flexible for users (can pass `&my_fn`)
- Consistent with Rust best practices for function parameters
- Enables easier testing (pass reference to test function)

**Type Signature**:
```rust
pub fn extend<W, F>(&self, f: &F) -> Pattern<W>
where
    F: Fn(&Pattern<V>) -> W
```

**Alternatives Considered**:
1. **By value with Clone bound**:
   ```rust
   pub fn extend<W, F>(&self, f: F) -> Pattern<W>
   where
       F: Fn(&Pattern<V>) -> W + Clone
   ```
   - Rejected: Requires Clone bound, less flexible
   - Requires cloning function on each recursive call

2. **By value without Clone** (owned function):
   ```rust
   pub fn extend<W, F>(self, f: F) -> Pattern<W>
   where
       F: Fn(&Pattern<V>) -> W
   ```
   - Rejected: Can only call extend once (consumes function)
   - Not useful for practical use cases

**References**:
- Rust Book: Pass functions by reference when possible
- Iterator adapters precedent: `map`, `filter`, etc. take closure by value but Clone when needed

### 3. Helper Implementation: Using `extend` vs. Direct Recursion

**Question**: Should `depth_at` and `size_at` use `extend` or direct recursive implementation?

**Decision**: **Use `extend` for conceptual consistency**

**Rationale**:
- Makes "decorative computation" pattern explicit
- Demonstrates Comonad's practical utility
- Maintains conceptual clarity even if performance is identical
- gram-hs uses `extend` for `depthAt` (we'll do same for both)

**Implementation**:
```rust
pub fn depth_at(&self) -> Pattern<usize> {
    self.extend(&|p| p.depth())
}

pub fn size_at(&self) -> Pattern<usize> {
    self.extend(&|p| p.size())
}
```

**Alternatives Considered**:
1. **Direct recursive implementation** (like gram-hs `sizeAt`):
   ```rust
   pub fn size_at(&self) -> Pattern<usize> {
       Pattern {
           value: self.size(),
           elements: self.elements.iter().map(|e| e.size_at()).collect(),
       }
   }
   ```
   - Rejected: Obscures the "context-aware decoration" concept
   - Same performance but less clear intent

**Exception**: `indices_at` requires path tracking, cannot use `extend`:
```rust
pub fn indices_at(&self) -> Pattern<Vec<usize>> {
    fn go<V>(path: Vec<usize>, p: &Pattern<V>) -> Pattern<Vec<usize>> {
        Pattern {
            value: path.clone(),
            elements: p.elements.iter().enumerate()
                .map(|(i, e)| {
                    let mut new_path = path.clone();
                    new_path.push(i);
                    go(new_path, e)
                })
                .collect(),
        }
    }
    go(vec![], self)
}
```

**References**:
- gram-hs: `depthAt = extend depth`
- RECOMMENDATION.md: "Use extend for conceptual consistency"

### 4. Testing Approach: Property-Based Laws + Behavioral Equivalence

**Question**: How to verify correctness of Comonad implementation?

**Decision**: **Three-tier testing strategy**

**Tier 1: Property-Based Law Tests**
- Use `proptest` to verify Comonad laws hold for arbitrary patterns
- Laws:
  1. Left identity: `extract(extend(f, p)) == f(p)`
  2. Right identity: `extend(extract, p) == p`
  3. Associativity: `extend(f, extend(g, p)) == extend(f ∘ extend(g), p)`

**Tier 2: Unit Tests for Helpers**
- Test `depth_at`, `size_at`, `indices_at` on known pattern structures
- Verify outputs match expected values
- Cover edge cases: atomic patterns, deeply nested, large patterns

**Tier 3: Behavioral Equivalence with gram-hs**
- Compare outputs of helpers with gram-hs reference implementation
- Test cases:
  - Atomic patterns
  - Patterns with elements
  - Deeply nested patterns
  - Various pattern structures

**Implementation**:
```rust
// tests/comonad_laws.rs
proptest! {
    #[test]
    fn comonad_left_identity(p in arbitrary_pattern()) {
        let f = |p: &Pattern<i32>| p.depth();
        assert_eq!(p.extend(&f).extract(), &f(&p));
    }
    
    #[test]
    fn comonad_right_identity(p in arbitrary_pattern()) {
        let result = p.extend(&|p: &Pattern<i32>| p.extract().clone());
        assert_eq!(result, p);
    }
    
    #[test]
    fn comonad_associativity(p in arbitrary_pattern()) {
        let f = |p: &Pattern<i32>| p.depth();
        let g = |p: &Pattern<i32>| p.size();
        
        let left = p.extend(&g).extend(&f);
        let right = p.extend(&|p: &Pattern<i32>| f(&p.extend(&g)));
        
        assert_eq!(left, right);
    }
}
```

**Alternatives Considered**:
1. **Only unit tests**: Rejected - doesn't verify laws hold in general
2. **Only property tests**: Rejected - doesn't verify behavioral equivalence with gram-hs
3. **Manual law verification**: Rejected - property-based testing is more thorough

**References**:
- gram-hs: `tests/Spec/Pattern/Properties.hs` (lines 1287-1332) for law tests
- gram-hs: `tests/Spec/Pattern/CoreSpec.hs` (lines 4242-4400) for helper tests
- proptest documentation: https://docs.rs/proptest/latest/proptest/

### 5. Performance Considerations

**Question**: What performance characteristics should we target?

**Decision**: **O(n) single-pass operations, <100ms for 1000+ elements**

**Analysis**:
- `extract`: O(1) - direct field access
- `extend`: O(n) - single traversal, applies function at each node
- `depth_at`: O(n) - uses extend, which is O(n)
- `size_at`: O(n) - uses extend, which is O(n)
- `indices_at`: O(n) - single traversal with path accumulation

**Verification**:
- Benchmark suite to measure actual performance
- Test with patterns of varying sizes (10, 100, 1000, 10000 nodes)
- Verify linear scaling

**Optimizations** (if needed):
- None expected - operations are already optimal
- If performance issues arise, profile before optimizing

**References**:
- Existing Pattern operations (map, fold) are O(n)
- gram-hs performs identically (Haskell laziness doesn't help here)

## Technology Stack Confirmation

**Language**: Rust 1.75+ (stable)
**Testing Framework**: 
- `cargo test` (built-in)
- `proptest` for property-based testing
**Documentation**: 
- rustdoc with examples
- Module-level conceptual explanation
**Benchmarking** (optional): 
- criterion.rs if performance verification needed
**CI/CD**: 
- Existing pattern-rs CI pipeline
- Add Comonad tests to test suite

## Best Practices Applied

### Rust Idioms
- Direct methods over traits (when single type)
- Pass functions by reference (no Clone bound)
- Return references from `extract` (no ownership transfer)
- Standard naming conventions (`snake_case`)

### Functional Programming
- Pure functions (no side effects)
- Immutable data structures
- Law verification (mathematical properties)

### Testing
- Property-based testing for laws
- Unit testing for specific functionality
- Behavioral equivalence with reference implementation

### Documentation
- Conceptual explanation of "decorated sequence" semantics
- Examples showing practical use cases
- Clear API documentation with type signatures

## Integration Patterns

### With Existing Pattern Operations
- **Composability**: `extend` output can be `map`ped, `fold`ed, `traverse`d
- **Chaining**: `pattern.extend(&f).map(g).fold(init, h)`
- **No conflicts**: New methods are purely additive

### With External Libraries
- **WASM**: All operations are pure, no special considerations
- **Serialization**: Pattern already serializable, decorated patterns same
- **Visualization**: `depth_at`, `size_at`, `indices_at` useful for visual tools

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Performance regression | Medium | Benchmark suite, compare with gram-hs |
| Law violations in edge cases | High | Property-based testing with arbitrary patterns |
| Conceptual confusion for users | Medium | Clear documentation with examples |
| Breaking changes to Pattern | High | Only additive changes, no modifications |
| WASM incompatibility | Medium | All operations pure, tested on WASM target |

## Open Questions

**Status**: All questions resolved

1. ✅ Implementation strategy: Direct methods
2. ✅ Function passing: By reference
3. ✅ Helper implementation: Use extend
4. ✅ Testing approach: Three-tier strategy
5. ✅ Performance targets: O(n), <100ms for 1000+ elements

## Next Steps

1. ✅ Research complete - proceed to Phase 1 (design)
2. Create data-model.md (Pattern structure with Comonad operations)
3. Create contracts/comonad.md (API contracts and laws)
4. Create quickstart.md (usage examples)
5. Update agent context with technology decisions
6. Generate implementation tasks with `/speckit.tasks`

## References

- **gram-hs Haskell implementation**: `../gram-hs/libs/pattern/src/Pattern/Core.hs`
- **gram-hs tests**: `../gram-hs/libs/pattern/tests/Spec/Pattern/`
- **Feature spec**: [spec.md](./spec.md)
- **Analysis**: [ANALYSIS.md](./ANALYSIS.md)
- **Recommendation**: [RECOMMENDATION.md](./RECOMMENDATION.md)
- **Constitution**: `../../.specify/memory/constitution.md`
- **Rust API Guidelines**: https://rust-lang.github.io/api-guidelines/
- **proptest docs**: https://docs.rs/proptest/latest/proptest/
