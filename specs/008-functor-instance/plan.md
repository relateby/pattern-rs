# Implementation Plan: Functor Instance for Pattern

**Branch**: `008-functor-instance` | **Date**: 2026-01-04 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/008-functor-instance/spec.md`

## Summary

Implement idiomatic Rust mapping functionality for the `Pattern<V>` type that provides the same behavioral guarantees as Haskell's Functor typeclass. This enables structure-preserving value transformations using a direct `map` method following Rust standard library conventions (similar to `Option::map`, `Result::map`, `Iterator::map`).

**Key Principle**: We are porting the **concept** of a Functor (structure-preserving value transformation) that was tested in Haskell, not the Haskell syntax itself. The implementation will be idiomatic Rust that maintains behavioral equivalence with the Haskell implementation.

## Technical Context

**Language/Version**: Rust 1.70.0+ (workspace MSRV), Edition 2021  
**Primary Dependencies**: 
- Standard library only (no new dependencies required)
- Existing Pattern<V> type from feature 004

**Storage**: N/A (in-memory transformations only)  
**Testing**: 
- `cargo test` - Standard Rust test framework
- `proptest` (workspace) - Property-based testing for functor laws
- Test utilities in `crates/pattern-core/src/test_utils/` for equivalence checking

**Target Platform**: 
- Native Rust targets (x86_64, ARM, etc.)
- WebAssembly (`wasm32-unknown-unknown`)

**Project Type**: Library crate (part of multi-crate workspace)  
**Performance Goals**: 
- Transformations should be O(n) where n is total number of nodes
- <10ms for patterns with 1000 nodes
- Support 100+ nesting levels without stack overflow
- Minimal memory overhead (transformation in-place where possible)

**Constraints**: 
- MUST maintain behavioral equivalence with gram-hs Functor instance
- MUST compile for `wasm32-unknown-unknown` target
- MUST use idiomatic Rust patterns (standard library conventions)
- MUST satisfy functor laws (identity and composition)
- SHOULD follow Rust naming conventions (`map` instead of `fmap`)
- SHOULD consume ownership for zero-copy transformations

**Scale/Scope**: 
- Implementation in `pattern-core` crate
- Property-based tests for functor laws
- Integration with existing pattern construction functions
- Handles patterns with 10,000+ nodes efficiently

**Verified from gram-hs Implementation**:
- ✅ Functor instance exists in `Pattern/Core.hs` (lines 536-617)
- ✅ Implementation: `fmap f (Pattern v es) = Pattern (f v) (map (fmap f) es)`
- ✅ Functor laws tested in `Spec/Pattern/Properties.hs` (lines 176-203)
- ✅ Structure preservation is guaranteed by implementation
- ✅ Type transformation supported (`Pattern v` → `Pattern w`)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity ✅
- **Status**: PASS
- **Verification**: Feature spec references the actual Haskell Functor instance in `../pattern-hs/libs/pattern/src/Pattern/Core.hs` as the authoritative source of behavioral requirements
- **Plan**: Implement Rust functionality that maintains the same behavior (structure-preserving value transformation) while using idiomatic Rust syntax
- **Reference Path**: `../pattern-hs/libs/pattern/src/Pattern/Core.hs` (lines 536-617) and test file `../pattern-hs/libs/pattern/tests/Spec/Pattern/Properties.hs` (lines 176-203)
- **Porting Approach**: Per docs/porting-guide.md, we port **concepts and behavior**, not syntax. The Haskell implementation defines what the code should do; the Rust implementation achieves this idiomatically.

### II. Correctness & Compatibility (NON-NEGOTIABLE) ✅
- **Status**: PASS
- **Verification**: Spec requires functor laws (identity, composition) to pass property tests
- **Plan**: Port functor law tests from gram-hs and verify behavior matches
- **Behavioral Equivalence**: Same structure preservation, same recursive application, same type transformations

### III. Rust Native Idioms ✅
- **Status**: PASS  
- **Verification**: Plan uses standard Rust method naming (`map` instead of `fmap`)
- **Plan**: Follow standard library conventions (similar to `Option::map`, `Result::map`)
- **Rationale**: Rust developers expect `map` methods, not `fmap` from Haskell. Direct method is more idiomatic than a Functor trait (which would require HKTs that Rust doesn't have)

### IV. Multi-Target Library Design ✅
- **Status**: PASS
- **Verification**: Spec requires WASM compilation
- **Plan**: Pure transformation logic with no platform dependencies

### V. External Language Bindings & Examples ✅
- **Status**: DEFERRED
- **Verification**: WASM bindings are out of scope for this feature
- **Plan**: Methods must compile for WASM but bindings deferred to later features

**Note**: When porting features from gram-hs, **always use the Haskell implementation in `../pattern-hs/libs/` as the behavioral specification**. Per docs/porting-guide.md, we port concepts and behavior (what the code does), not syntax (how it looks in Haskell). The Rust implementation should be idiomatic while maintaining behavioral equivalence. See [docs/porting-guide.md](../../../docs/porting-guide.md) section on "Idiomatic Rust vs Literal Translation" for detailed guidance.

## Project Structure

### Documentation (this feature)

```text
specs/008-functor-instance/
├── plan.md                  # This file
├── spec.md                  # Feature specification
├── IMPLEMENTATION_NOTES.md  # Design rationale (already created)
├── README.md                # Feature overview (already created)
├── research.md              # Phase 0 output (to be generated)
├── data-model.md            # Phase 1 output (to be generated)
├── quickstart.md            # Phase 1 output (to be generated)
├── contracts/               # Phase 1 output (to be generated)
│   └── type-signatures.md   # Method signatures
├── checklists/
│   └── requirements.md      # Specification quality checklist (complete)
└── tasks.md                 # Phase 2 output (/speckit.tasks command)
```

### Implementation

```text
crates/pattern-core/
├── src/
│   ├── pattern.rs       # Add map method to impl<V> Pattern<V> block
│   └── lib.rs           # Export map functionality
└── tests/
    └── functor_laws.rs  # New: Property-based tests for functor laws
```

## Phase Breakdown

### Phase 0: Research & Validation ✅

**Status**: Complete

**Research Questions Resolved**:

1. **Q: Should we implement a Functor trait or direct methods?**
   - **Decision**: Direct `map` method (no trait)
   - **Rationale**: Rust lacks Higher-Kinded Types (HKTs) making generic Functor traits awkward. Standard library uses direct methods (`Option::map`, `Result::map`). More idiomatic and easier to use.
   - **Alternatives considered**: Custom Functor trait (rejected - adds complexity without value), Iterator-based approach (rejected - Pattern is not an iterator)

2. **Q: What should the method signature be?**
   - **Decision**: `pub fn map<W, F>(self, f: F) -> Pattern<W> where F: Fn(&V) -> W`
   - **Rationale**: Follows standard library conventions. Consumes `self` for zero-copy. Function takes `&V` (doesn't consume values) and returns `W` (supports type transformation).
   - **Alternatives considered**: `&self` receiver (rejected - requires cloning), `FnMut` or `FnOnce` bounds (rejected - `Fn` is most flexible)

3. **Q: How to handle recursion (closure capture)?**
   - **Decision**: Use internal `map_with(&F)` helper for recursive calls
   - **Rationale**: Public API takes `F` by value for ergonomics, internal helper takes `&F` by reference for efficiency. Avoids `Clone` bound and nested reference types.
   - **Alternatives considered**: Clone closure (rejected - requires `F: Clone` bound), direct `&f` capture (rejected - nested reference type errors)

4. **Q: What naming convention to use?**
   - **Decision**: `map` (not `fmap`)
   - **Rationale**: Rust standard library convention. Rust developers expect `map`.
   - **Alternatives considered**: `fmap` (rejected - Haskell-specific terminology unfamiliar to Rust developers)

**See**: [research.md](./research.md) for detailed analysis

### Phase 1: Design

**Tasks**:
1. Define method signature for `Pattern::map`
2. Document recursive transformation strategy
3. Plan closure capture mechanism for recursive calls
4. Design property-based test structure
5. Create API contracts (type signatures)
6. Generate quickstart guide with examples

**Outputs**:
- [data-model.md](./data-model.md) - Pattern transformation model
- [contracts/type-signatures.md](./contracts/type-signatures.md) - Method signatures
- [quickstart.md](./quickstart.md) - Usage examples

**Acceptance Criteria**:
- Method signature documented and approved
- Recursion strategy validated
- Test structure planned
- All Phase 1 artifacts generated

### Phase 2: Core Implementation

**Tasks**:
1. Implement `Pattern::map` method in `crates/pattern-core/src/pattern.rs`
2. Add comprehensive documentation with examples
3. Ensure WASM compatibility
4. Add unit tests for basic cases

**Acceptance Criteria**:
- Method compiles without errors
- Basic transformations work (string → string, int → int)
- Type transformations work (string → int, int → string)
- Documentation includes clear examples
- WASM target compiles successfully

### Phase 3: Functor Laws Testing

**Tasks**:
1. Port identity law tests from gram-hs
2. Port composition law tests from gram-hs
3. Implement property-based tests using `proptest`
4. Test with various pattern structures (atomic, nested, deep)
5. Verify tests pass with 100+ random cases

**Acceptance Criteria**:
- Identity law: 100+ property test cases pass
- Composition law: 100+ property test cases pass
- Tests cover edge cases (empty patterns, deep nesting)
- All ported gram-hs tests pass

### Phase 4: Structure Preservation Verification

**Tasks**:
1. Verify element count preservation
2. Verify nesting depth preservation
3. Verify element order preservation
4. Performance testing with large patterns (1000+ nodes)
5. Stack safety testing (100+ nesting levels)

**Acceptance Criteria**:
- Structure metrics unchanged after transformation
- Performance targets met (<10ms for 1000 nodes)
- Stack overflow doesn't occur (100+ levels)
- Memory overhead acceptable (10K elements under 100MB)

### Phase 5: Integration & Documentation

**Tasks**:
1. Update crate documentation
2. Add examples to README
3. Verify WASM compilation
4. Update TODO.md feature checklist
5. Create migration notes if needed

**Acceptance Criteria**:
- Feature marked complete in TODO.md
- Documentation examples compile and run
- WASM target compiles successfully
- All success criteria from spec.md verified

## Implementation Approach: Idiomatic Rust

### Core Implementation Strategy

**Implement a `map` method directly on `Pattern<V>`** following Rust standard library conventions:

```rust
impl<V> Pattern<V> {
    /// Maps a function over all values in the pattern, preserving structure.
    ///
    /// This is equivalent to Haskell's `fmap` for the Functor typeclass,
    /// but follows Rust naming conventions.
    ///
    /// # Functor Laws
    ///
    /// This implementation satisfies the functor laws:
    /// - Identity: `pattern.map(|x| x) == pattern`
    /// - Composition: `pattern.map(|x| g(f(x))) == pattern.map(f).map(g)`
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// // String transformation
    /// let pattern = Pattern::point("hello");
    /// let upper = pattern.map(|s| s.to_uppercase());
    /// assert_eq!(upper.value, "HELLO");
    ///
    /// // Type conversion
    /// let numbers = Pattern::point(42);
    /// let strings = numbers.map(|n| n.to_string());
    /// assert_eq!(strings.value, "42");
    ///
    /// // Composition
    /// let result = Pattern::point(5)
    ///     .map(|n| n * 2)
    ///     .map(|n| n + 1);
    /// assert_eq!(result.value, 11);
    /// ```
    pub fn map<W, F>(self, f: F) -> Pattern<W>
    where
        F: Fn(&V) -> W,
    {
        Pattern {
            value: f(&self.value),
            elements: self.elements
                .into_iter()
                .map(|elem| elem.map(&f))
                .collect(),
        }
    }
}
```

**Key Design Decisions**:

1. **Method name**: `map` (not `fmap`) - follows Rust conventions
2. **Ownership**: Consumes `self` for zero-copy transformation
3. **Function parameter**: `Fn(&V) -> W` - takes reference to value, returns new value
4. **Recursive**: Uses closure capture (`&f`) for recursive calls
5. **Type transformation**: Enables `Pattern<V>` → `Pattern<W>`

### Why This Is Idiomatic Rust

- ✅ **Standard library pattern**: Matches `Option::map`, `Result::map`, `Iterator::map`
- ✅ **Clear naming**: Rust developers immediately understand `map`
- ✅ **Ownership model**: Consumes pattern for efficient transformation
- ✅ **Type inference**: Works seamlessly with Rust's type system
- ✅ **No traits needed**: Simple method, no complex trait hierarchies
- ✅ **Composable**: Can chain calls: `pattern.map(f).map(g)`

### Behavioral Equivalence with Haskell

The Rust implementation maintains the same behavior as Haskell's `fmap`:

**Haskell**:
```haskell
fmap f (Pattern v es) = Pattern (f v) (map (fmap f) es)
```

**Rust equivalent**:
```rust
Pattern {
    value: f(&self.value),
    elements: self.elements.into_iter().map(|e| e.map(&f)).collect(),
}
```

**Functor Laws** (must be satisfied):
1. **Identity**: `pattern.map(|x| x.clone()) == pattern`
2. **Composition**: `pattern.map(|x| g(&f(x))) == pattern.map(f).map(g)`

These laws will be tested via property-based tests ported from gram-hs.

## Testing Strategy

### Property-Based Tests (Primary)

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn identity_law(pattern in arbitrary_pattern()) {
        let original = pattern.clone();
        let mapped = pattern.map(|x| x.clone());
        prop_assert_eq!(original, mapped);
    }

    #[test]
    fn composition_law(pattern in arbitrary_pattern()) {
        let f = |x: &i32| x * 2;
        let g = |x: &i32| x + 1;
        
        let composed = pattern.clone().map(|x| g(&f(x)));
        let sequential = pattern.map(f).map(g);
        
        prop_assert_eq!(composed, sequential);
    }
    
    #[test]
    fn structure_preservation(pattern in arbitrary_pattern()) {
        let original_len = pattern.length();
        let original_depth = pattern.depth();
        let original_size = pattern.size();
        
        let mapped = pattern.map(|x: &i32| x * 2);
        
        prop_assert_eq!(mapped.length(), original_len);
        prop_assert_eq!(mapped.depth(), original_depth);
        prop_assert_eq!(mapped.size(), original_size);
    }
}
```

### Unit Tests (Supplementary)

```rust
#[test]
fn map_atomic_pattern() {
    let p = Pattern::point("hello");
    let upper = p.map(|s| s.to_uppercase());
    assert_eq!(upper.value, "HELLO");
    assert_eq!(upper.elements.len(), 0);
}

#[test]
fn map_nested_pattern() {
    let p = Pattern::pattern("root", vec![
        Pattern::point("child1"),
        Pattern::point("child2"),
    ]);
    let upper = p.map(|s| s.to_uppercase());
    assert_eq!(upper.value, "ROOT");
    assert_eq!(upper.elements[0].value, "CHILD1");
    assert_eq!(upper.elements[1].value, "CHILD2");
}

#[test]
fn map_type_conversion() {
    let p = Pattern::point(42);
    let stringified = p.map(|n| n.to_string());
    assert_eq!(stringified.value, "42");
}

#[test]
fn map_preserves_structure() {
    let p = Pattern::pattern("root", vec![
        Pattern::point("a"),
        Pattern::pattern("b", vec![Pattern::point("c")]),
    ]);
    
    let original_size = p.size();
    let original_depth = p.depth();
    let original_length = p.length();
    
    let mapped = p.map(|s| s.to_uppercase());
    
    assert_eq!(mapped.size(), original_size);
    assert_eq!(mapped.depth(), original_depth);
    assert_eq!(mapped.length(), original_length);
}
```

## Success Criteria

From spec.md (all must pass):

- ✅ **SC-001**: All functor law property tests pass with at least 100 randomly generated test cases each
- ✅ **SC-002**: Transformations complete on patterns with 1000 nodes in under 10 milliseconds
- ✅ **SC-003**: Transformations complete on patterns with 100 nesting levels without stack overflow
- ✅ **SC-004**: Code can transform patterns between different value types without type errors
- ✅ **SC-005**: 100% of existing gram-hs functor tests are ported and pass in Rust implementation
- ✅ **SC-006**: Functor implementation compiles for WASM target without errors
- ✅ **SC-007**: Pattern structures with 10,000 elements can be transformed without exceeding 100MB memory overhead

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Stack overflow on deep nesting | Low | High | Test with 100+ levels early; use iterative approach if needed |
| Performance issues on large patterns | Low | Medium | Benchmark early with 1000+ nodes; optimize if needed |
| Memory overhead from transformations | Low | Medium | Profile memory usage; adjust strategy if needed |
| Functor law violations | Very Low | High | Extensive property-based testing with 100+ cases |
| Type inference issues | Very Low | Low | Provide clear type annotations in documentation |

## Dependencies

- Feature 004: Pattern Data Structure (complete) - Defines `Pattern<V>` type
- Feature 005: Basic Pattern Type (complete) - Defines construction and access functions
- Rust standard library `Fn` trait

## References

- **Haskell Implementation**: `../pattern-hs/libs/pattern/src/Pattern/Core.hs` (lines 536-617)
- **Haskell Tests**: `../pattern-hs/libs/pattern/tests/Spec/Pattern/Properties.hs` (lines 176-203)
- **Porting Guide**: `docs/porting-guide.md` - Section on "Idiomatic Rust vs Literal Translation"
- **Rust Standard Library**: `Option::map`, `Result::map` for API conventions
- **Implementation Notes**: [IMPLEMENTATION_NOTES.md](./IMPLEMENTATION_NOTES.md) - Design rationale
