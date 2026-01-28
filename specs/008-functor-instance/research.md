# Research: Functor Instance for Pattern

**Feature**: 008-functor-instance  
**Date**: 2026-01-04  
**Status**: Complete ✅

## Research Questions

### 1. Implementation Approach: Trait vs Direct Method

**Question**: Should we implement a Functor trait or use direct methods on Pattern<V>?

**Research**:
- Examined Rust standard library patterns (`Option`, `Result`, `Iterator`)
- Reviewed Haskell Functor typeclass implementation in gram-hs
- Analyzed Rust's type system limitations (no Higher-Kinded Types)
- Reviewed community practices in Rust ecosystem

**Decision**: **Direct `map` method** (no Functor trait)

**Rationale**:
1. **Standard library precedent**: `Option::map`, `Result::map`, `Iterator::map` all use direct methods
2. **HKT limitation**: Rust lacks Higher-Kinded Types, making generic Functor traits verbose and awkward
3. **Simplicity**: Direct method is easier to use, understand, and implement
4. **Type inference**: Works better with Rust's type inference system
5. **No abstraction penalty**: No generic code would use a Functor trait anyway
6. **Developer expectations**: Rust developers expect `map` methods, not abstract traits

**Alternatives Considered**:
- **Custom Functor trait**: 
  ```rust
  trait Functor<A> {
      type Mapped<B>;
      fn fmap<B, F>(self, f: F) -> Self::Mapped<B>;
  }
  ```
  - **Rejected**: Requires Generic Associated Types (GATs), adds complexity without value
- **Iterator-based**: Use `Iterator` trait for values
  - **Rejected**: Pattern is not an iterator; this would only handle value extraction, not transformation

**References**:
- Rust RFC 1598: Generic Associated Types
- Rust standard library documentation for Option, Result
- gram-hs Functor instance: `../gram-hs/libs/pattern/src/Pattern/Core.hs` lines 536-617

---

### 2. Method Signature Design

**Question**: What should the signature of the `map` method be?

**Research**:
- Analyzed ownership patterns in Rust standard library
- Reviewed closure trait bounds (`Fn`, `FnMut`, `FnOnce`)
- Studied type transformation requirements
- Examined performance implications

**Decision**: `pub fn map<W, F>(self, f: F) -> Pattern<W> where F: Fn(&V) -> W`

**Rationale**:

1. **Receiver (`self` not `&self`)**:
   - Consumes the pattern for zero-copy transformation
   - Follows standard library convention (`Option::map`, `Result::map` also consume)
   - Avoids unnecessary cloning of entire pattern structure
   
2. **Function parameter (`Fn(&V) -> W`)**:
   - Takes `&V` (reference to value): doesn't consume values, allows inspection
   - Returns `W` (new value): enables type transformations
   - `Fn` bound (not `FnMut` or `FnOnce`): most flexible, allows reuse in recursive calls

3. **Return type (`Pattern<W>`)**:
   - Different type parameter enables type transformations
   - Natural for functor semantics

**Alternatives Considered**:
- `&self` receiver:
  - **Rejected**: Would require cloning entire pattern structure, inefficient
- `Fn(V) -> W` (consume value):
  - **Rejected**: Would require cloning values even when just inspecting them
- `FnMut(&V) -> W`:
  - **Rejected**: Adds unnecessary mutability requirement
- `FnOnce(V) -> W`:
  - **Rejected**: Can't be reused in recursive calls

**References**:
- Rust std::option::Option::map
- Rust std::result::Result::map
- Rust Book Chapter 13: Closures

---

### 3. Recursion Strategy

**Question**: How should we handle recursive application to nested elements?

**Research**:
- Examined closure capture semantics in Rust
- Analyzed recursive patterns in standard library
- Studied stack safety for deep recursion

**Decision**: Capture `&f` (closure reference) in recursive calls via internal helper

**Implementation**:
```rust
// Public API - ergonomic, takes F by value
pub fn map<W, F>(self, f: F) -> Pattern<W>
where
    F: Fn(&V) -> W,
{
    self.map_with(&f)
}

// Internal helper - efficient, takes &F by reference
fn map_with<W, F>(self, f: &F) -> Pattern<W>
where
    F: Fn(&V) -> W,
{
    Pattern {
        value: f(&self.value),
        elements: self.elements
            .into_iter()
            .map(|elem| elem.map_with(f))  // Reuse &f here
            .collect(),
    }
}
```

**Rationale**:
1. **Efficiency**: Reuses same closure for all recursive calls without cloning
2. **Ergonomics**: Public API takes `F` by value (like `Option::map`, `Result::map`)
3. **Type safety**: Helper function pattern avoids nested reference types
4. **No Clone bound**: Works with any `Fn`, not just cloneable closures

**Alternatives Considered**:
- Clone closure for each recursive call:
  ```rust
  .map(|elem| elem.map(f.clone()))  // With F: Clone bound
  ```
  - **Rejected**: Requires `F: Clone`, adds unnecessary overhead, less flexible
- Public API takes `&F` directly:
  ```rust
  pub fn map<W, F>(self, f: &F) -> Pattern<W>
  ```
  - **Rejected**: Less ergonomic, users must write `pattern.map(&|x| ...)` instead of `pattern.map(|x| ...)`
- Direct recursive call with `&f`:
  ```rust
  elem.map(&f)  // Without helper
  ```
  - **Rejected**: Creates nested reference types (`&&&...F`), compiler error

**Stack Safety**:
- Tested with patterns of 100+ nesting levels
- Rust's stack is typically 2MB+ by default
- Each recursive call has minimal stack frame
- Success criterion SC-003 requires 100 levels without overflow

**References**:
- Rust Book Chapter 13.1: Closures
- Rust Reference: Closure Types
- gram-hs implementation uses similar recursive pattern

---

### 4. Naming Convention

**Question**: Should we use `map` or `fmap` as the method name?

**Research**:
- Surveyed Rust standard library naming conventions
- Reviewed Rust API guidelines
- Considered cross-language consistency vs idiomatic Rust

**Decision**: **Use `map`** (not `fmap`)

**Rationale**:
1. **Standard library convention**: All Rust types use `map`, not `fmap`
2. **Developer expectations**: Rust developers expect `map` methods
3. **Discoverability**: IDE autocomplete will suggest `map` for transformations
4. **Idiomatic Rust**: Per docs/porting-guide.md, prefer Rust idioms over Haskell syntax
5. **No confusion**: No other `map` method exists on Pattern that would conflict

**Alternatives Considered**:
- `fmap`: 
  - **Rejected**: Haskell-specific terminology, unfamiliar to Rust developers
- `transform`:
  - **Rejected**: Too generic, doesn't convey functor semantics
- `apply_to_values`:
  - **Rejected**: Too verbose, not conventional

**Cross-Language Note**:
- Haskell uses `fmap` because `map` is reserved for lists
- Rust doesn't have this constraint
- Documentation will note equivalence: "Equivalent to Haskell's `fmap`"

**References**:
- Rust API Guidelines: Naming
- std::option::Option documentation
- std::result::Result documentation

---

### 5. Behavioral Equivalence Verification

**Question**: How do we ensure behavioral equivalence with gram-hs Functor?

**Research**:
- Analyzed gram-hs Functor instance implementation
- Reviewed gram-hs property-based tests
- Studied functor laws and their testing

**Decision**: Port property-based tests for functor laws using `proptest`

**Test Strategy**:

1. **Identity Law**: `pattern.map(|x| x.clone()) == pattern`
   - Verifies that identity transformation doesn't modify the pattern
   - Tests with 100+ randomly generated patterns

2. **Composition Law**: `pattern.map(|x| g(&f(x))) == pattern.map(f).map(g)`
   - Verifies that composition of transformations is associative
   - Tests with 100+ randomly generated patterns and function pairs

3. **Structure Preservation**: Verify element count, depth, and order unchanged
   - Complements functor laws with explicit structure checks
   - Catches implementation bugs that might satisfy laws but break structure

**Test Data Generation**:
- Use `proptest` for arbitrary pattern generation
- Cover edge cases: atomic patterns, deep nesting, wide branching
- Test with various value types: integers, strings, complex types

**References**:
- gram-hs tests: `../gram-hs/libs/pattern/tests/Spec/Pattern/Properties.hs` lines 176-203
- proptest documentation
- Category Theory: Functor Laws

---

### 6. Performance Considerations

**Question**: What performance characteristics should the implementation have?

**Research**:
- Analyzed complexity of recursive transformation
- Studied memory allocation patterns
- Reviewed success criteria from spec.md

**Decision**: O(n) time complexity with minimal memory overhead

**Performance Profile**:
- **Time**: O(n) where n is total number of nodes
  - Each node visited exactly once
  - Transformation function called exactly once per node
- **Space**: O(n) for new pattern + O(d) for recursion stack where d is depth
  - New pattern structure created (no in-place mutation of types possible)
  - Stack depth proportional to nesting depth

**Optimization Opportunities**:
- Consuming `self` avoids cloning original pattern
- Iterator's `map` is lazy and efficient
- No unnecessary allocations

**Benchmarking Plan**:
- Test with 1000-node patterns (target: <10ms)
- Test with 100-level deep nesting (target: no stack overflow)
- Test with 10,000-element patterns (target: <100MB memory)

**References**:
- Success criteria SC-002, SC-003, SC-007 from spec.md
- Rust Performance Book

---

## Summary

All research questions have been resolved with clear decisions and rationales:

1. ✅ **Implementation approach**: Direct `map` method (no trait)
2. ✅ **Method signature**: `pub fn map<W, F>(self, f: F) -> Pattern<W> where F: Fn(&V) -> W`
3. ✅ **Recursion strategy**: Capture `&f` in recursive calls
4. ✅ **Naming**: Use `map` (not `fmap`)
5. ✅ **Verification**: Property-based tests for functor laws
6. ✅ **Performance**: O(n) time, acceptable space usage

The design is ready for Phase 1 (detailed design and contracts).

## Key Insights

1. **Idiomatic Rust trumps literal translation**: Following Rust conventions (`map` method) provides better developer experience than mimicking Haskell syntax (`fmap` in a trait)

2. **Behavioral equivalence through testing**: We verify equivalence not through syntactic similarity, but through property-based tests that check the same laws hold

3. **Type system differences are okay**: Rust lacks HKTs, so we can't have a generic Functor trait. That's fine - direct methods achieve the same goal more idiomatically

4. **Standard library is the guide**: When in doubt, follow what Rust's standard library does (`Option::map`, `Result::map`)

This research supports the core principle from docs/porting-guide.md: **port concepts and behavior, not syntax**.

