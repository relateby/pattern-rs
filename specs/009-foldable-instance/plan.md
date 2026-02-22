# Implementation Plan: Foldable Instance for Pattern

**Branch**: `009-foldable-instance` | **Date**: 2026-01-04 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/009-foldable-instance/spec.md`

## Summary

Implement idiomatic Rust folding functionality for the `Pattern<V>` type that provides the same behavioral guarantees as Haskell's Foldable typeclass. This enables reducing pattern structures to single values using a direct `fold` method following Rust standard library conventions (similar to `Iterator::fold`, `Option::fold`, etc.).

**Key Principle**: We are porting the **concept** of a Foldable (aggregating values from a container) that was tested in Haskell, not the Haskell syntax itself. The implementation will be idiomatic Rust that maintains behavioral equivalence with the Haskell implementation.

## Technical Context

**Language/Version**: Rust 1.70.0+ (workspace MSRV), Edition 2021  
**Primary Dependencies**: 
- Standard library only (no new dependencies required)
- Existing Pattern<V> type from feature 004
- Functor instance from feature 008 (for integration)

**Storage**: N/A (in-memory operations only)  
**Testing**: 
- `cargo test` - Standard Rust test framework
- `proptest` (workspace) - Property-based testing for correctness
- Test utilities in `crates/pattern-core/src/test_utils/` for equivalence checking

**Target Platform**: 
- Native Rust targets (x86_64, ARM, etc.)
- WebAssembly (`wasm32-unknown-unknown`)

**Project Type**: Library crate (part of multi-crate workspace)  
**Performance Goals**: 
- Fold operations should be O(n) where n is total number of nodes
- <10ms for patterns with 1000 nodes
- Support 100+ nesting levels without stack overflow
- Minimal memory overhead (constant stack per recursion level)

**Constraints**: 
- MUST maintain behavioral equivalence with gram-hs Foldable instance
- MUST compile for `wasm32-unknown-unknown` target
- MUST use idiomatic Rust patterns (standard library conventions)
- MUST process values in depth-first, root-first order (matches Haskell)
- SHOULD follow Rust naming conventions (`fold` instead of `foldr`)
- SHOULD borrow pattern (not consume) for flexibility

**Scale/Scope**: 
- Implementation in `pattern-core` crate
- Property-based tests for correctness
- Integration with existing pattern construction functions
- Handles patterns with 10,000+ nodes efficiently

**Verified from gram-hs Implementation**:
- ✅ Foldable instance exists in `Pattern/Core.hs` (line 750-751)
- ✅ Implementation: `foldMap f (Pattern v es) = f v <> foldMap (foldMap f) es`
- ✅ Foldable tests in `Spec/Pattern/CoreSpec.hs` (lines 1054-1499)
- ✅ Order guarantee: root first, then elements
- ✅ Works with `toList`, `foldr`, counting, etc.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity ✅
- **Status**: PASS
- **Verification**: Feature spec references the actual Haskell Foldable instance in `../pattern-hs/libs/pattern/src/Pattern/Core.hs` as the authoritative source of behavioral requirements
- **Plan**: Implement Rust functionality that maintains the same behavior (depth-first, root-first traversal and aggregation) while using idiomatic Rust syntax
- **Reference Path**: `../pattern-hs/libs/pattern/src/Pattern/Core.hs` (line 750-751) and test file `../pattern-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs` (lines 1054-1499)
- **Porting Approach**: Per docs/porting-guide.md, we port **concepts and behavior**, not syntax. The Haskell implementation defines what the code should do; the Rust implementation achieves this idiomatically.

### II. Correctness & Compatibility (NON-NEGOTIABLE) ✅
- **Status**: PASS
- **Verification**: Spec requires comprehensive tests for order, completeness, and correctness
- **Plan**: Port fold tests from gram-hs and verify behavior matches exactly
- **Behavioral Equivalence**: Same traversal order, same aggregation semantics, same type flexibility

### III. Rust Native Idioms ✅
- **Status**: PASS  
- **Verification**: Plan uses standard Rust method naming (`fold` instead of `foldr`/`foldl`)
- **Plan**: Follow standard library conventions (similar to `Iterator::fold`)
- **Rationale**: Rust developers expect `fold` methods, not `foldr` from Haskell. Direct method is more idiomatic than a Foldable trait (which would require HKTs that Rust doesn't have). Borrowing receiver (`&self`) is more flexible than consuming.

### IV. Multi-Target Library Design ✅
- **Status**: PASS
- **Verification**: Spec requires WASM compilation
- **Plan**: Pure aggregation logic with no platform dependencies

### V. External Language Bindings & Examples ✅
- **Status**: DEFERRED
- **Verification**: WASM bindings are out of scope for this feature
- **Plan**: Methods must compile for WASM but bindings deferred to later features

**Note**: When porting features from gram-hs, **always use the Haskell implementation in `../pattern-hs/libs/` as the behavioral specification**. Per docs/porting-guide.md, we port concepts and behavior (what the code does), not syntax (how it looks in Haskell). The Rust implementation should be idiomatic while maintaining behavioral equivalence. See [docs/porting-guide.md](../../../docs/porting-guide.md) section on "Idiomatic Rust vs Literal Translation" for detailed guidance.

## Project Structure

### Documentation (this feature)

```text
specs/009-foldable-instance/
├── plan.md                  # This file
├── spec.md                  # Feature specification
├── research.md              # Phase 0 output ✅
├── data-model.md            # Phase 1 output ✅
├── quickstart.md            # Phase 1 output ✅
├── contracts/               # Phase 1 output ✅
│   └── type-signatures.md   # Method signatures ✅
├── checklists/
│   └── requirements.md      # Specification quality checklist (complete)
└── tasks.md                 # Phase 2 output (/speckit.tasks command)
```

### Implementation

```text
crates/pattern-core/
├── src/
│   ├── pattern.rs       # Add fold methods to impl<V> Pattern<V> block
│   └── lib.rs           # Export fold functionality (already exports Pattern)
└── tests/
    └── foldable_properties.rs  # New: Property-based tests for fold correctness
```

## Phase Breakdown

### Phase 0: Research & Validation ✅

**Status**: Complete

**Research Questions Resolved**:

1. **Q: Should we implement a Foldable trait or direct methods?**
   - **Decision**: Direct methods on `Pattern<V>` (no trait)
   - **Rationale**: Rust lacks HKTs, no standard Foldable trait. Direct methods more idiomatic. Follows same approach as Functor (direct `map` method).
   - **Alternatives considered**: Custom trait (rejected - adds complexity), Iterator trait (rejected - Pattern is a tree, not a sequence)

2. **Q: What methods should be provided?**
   - **Decision**: `fold<B, F>(&self, init: B, f: F) -> B` and `values(&self) -> Vec<&V>`
   - **Rationale**: `fold` is most versatile, matches Rust conventions. `values()` common use case.
   - **Alternatives considered**: Both foldr and foldl (rejected - confusing, can simulate left fold via collect)

3. **Q: What should the method signature be?**
   - **Decision**: `pub fn fold<B, F>(&self, init: B, f: F) -> B where F: Fn(B, &V) -> B`
   - **Rationale**: Borrows pattern (more flexible), matches Rust conventions, accumulator before value.
   - **Alternatives considered**: Consuming self (rejected - too restrictive), different parameter order (rejected - not idiomatic)

4. **Q: How to handle traversal order?**
   - **Decision**: Depth-first, root-first (pre-order)
   - **Rationale**: Matches Haskell `foldMap` semantics. Root provides context for elements.
   - **Alternatives considered**: Post-order, level-order (rejected - don't match Haskell)

5. **Q: How to handle closure capture for recursion?**
   - **Decision**: Helper method pattern - `fold` takes F by value, `fold_with` takes &F by reference
   - **Rationale**: Same pattern as Functor `map`/`map_with`. Avoids cloning, ergonomic API.
   - **Alternatives considered**: Clone closure (rejected - requires Clone bound), direct recursion (rejected - type errors)

**See**: [research.md](./research.md) for detailed analysis

### Phase 1: Design ✅

**Status**: Complete

**Deliverables**:
- ✅ [research.md](./research.md) - Design decisions and alternatives
- ✅ [data-model.md](./data-model.md) - Type relationships and data flow
- ✅ [contracts/type-signatures.md](./contracts/type-signatures.md) - API contracts
- ✅ [quickstart.md](./quickstart.md) - Usage examples

**Key Decisions**:
1. Method signature: `fold<B, F>(&self, init: B, f: F) -> B where F: Fn(B, &V) -> B`
2. Traversal order: Depth-first, root-first
3. Helper method: `fold_with<B, F>(&self, acc: B, f: &F) -> B` for recursion
4. Convenience method: `values(&self) -> Vec<&V>` using fold internally

**Acceptance Criteria**: ✅ All Phase 1 artifacts generated

### Phase 2: Core Implementation

**Tasks**:
1. Implement `Pattern::fold` method in `crates/pattern-core/src/pattern.rs`
2. Implement `Pattern::fold_with` internal helper
3. Implement `Pattern::values` convenience method
4. Add comprehensive documentation with examples
5. Ensure WASM compatibility
6. Add unit tests for basic cases

**Implementation Strategy**:
```rust
impl<V> Pattern<V> {
    /// Folds the pattern into a single value.
    ///
    /// Processes values in depth-first, root-first order.
    /// Root value processed first, then elements left to right.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// // Sum values
    /// let pattern = Pattern::pattern(10, vec![
    ///     Pattern::point(20),
    ///     Pattern::point(30),
    /// ]);
    /// let sum = pattern.fold(0, |acc, v| acc + v);
    /// assert_eq!(sum, 60);
    ///
    /// // Count values
    /// let count = pattern.fold(0, |acc, _| acc + 1);
    /// assert_eq!(count, 3);
    /// ```
    pub fn fold<B, F>(&self, init: B, f: F) -> B
    where
        F: Fn(B, &V) -> B,
    {
        self.fold_with(init, &f)
    }

    /// Internal helper for recursive fold.
    fn fold_with<B, F>(&self, acc: B, f: &F) -> B
    where
        F: Fn(B, &V) -> B,
    {
        // Process root value
        let acc = f(acc, &self.value);
        
        // Process elements recursively (left to right)
        self.elements
            .iter()
            .fold(acc, |acc, elem| elem.fold_with(acc, f))
    }

    /// Collects all values into a vector in traversal order.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("A", vec![
    ///     Pattern::point("B"),
    ///     Pattern::point("C"),
    /// ]);
    /// let values: Vec<&str> = pattern.values();
    /// assert_eq!(values, vec![&"A", &"B", &"C"]);
    /// ```
    pub fn values(&self) -> Vec<&V> {
        self.fold(Vec::new(), |mut acc, v| {
            acc.push(v);
            acc
        })
    }
}
```

**Acceptance Criteria**:
- ✅ Methods compile without errors
- ✅ Basic fold operations work (sum, count, concatenate)
- ✅ Type transformations work (V → different type B)
- ✅ Documentation includes clear examples
- ✅ WASM target compiles successfully

### Phase 3: Comprehensive Testing

**Tasks**:
1. Port fold tests from gram-hs (atomic, flat, nested patterns)
2. Port order verification tests (string concatenation)
3. Implement property-based tests using `proptest`
4. Test with various accumulator types (int, string, vec, bool)
5. Test type transformations (V ≠ B)
6. Verify tests pass with 100+ random cases

**Test Categories**:

**1. Order Verification** (Critical):
```rust
#[test]
fn fold_processes_root_first() {
    let pattern = Pattern::pattern("A", vec![
        Pattern::point("B"),
        Pattern::point("C"),
    ]);
    
    // String concatenation is order-dependent
    let result = pattern.fold(String::new(), |acc, s| acc + s);
    assert_eq!(result, "ABC");  // Root first, then elements
}
```

**2. Correctness** (from gram-hs tests):
```rust
#[test]
fn fold_sums_all_values() {
    let pattern = Pattern::pattern(100, vec![
        Pattern::point(10),
        Pattern::point(20),
        Pattern::point(30),
    ]);
    
    let sum = pattern.fold(0, |acc, v| acc + v);
    assert_eq!(sum, 160);  // 100 + 10 + 20 + 30
}
```

**3. Structure Coverage**:
```rust
proptest! {
    #[test]
    fn fold_count_equals_size(pattern in arbitrary_pattern()) {
        let count = pattern.fold(0, |acc, _| acc + 1);
        prop_assert_eq!(count, pattern.size());
    }
}
```

**4. Integration with Map**:
```rust
#[test]
fn fold_after_map() {
    let pattern = Pattern::pattern(1, vec![Pattern::point(2)]);
    
    let result = pattern
        .map(|v| v * 2)
        .fold(0, |acc, v| acc + v);
    
    assert_eq!(result, 6);  // (1*2) + (2*2) = 2 + 4
}
```

**Acceptance Criteria**:
- ✅ All ported gram-hs tests pass
- ✅ Order verification tests pass (non-commutative operations)
- ✅ Property tests pass with 100+ cases
- ✅ Type transformation tests pass
- ✅ Integration tests with map pass

### Phase 4: Performance Verification

**Tasks**:
1. Benchmark large patterns (1000+ nodes)
2. Test deep nesting (100+ levels) for stack safety
3. Test wide patterns (1000+ siblings)
4. Profile memory usage
5. Verify performance targets met

**Benchmarks**:
```rust
#[bench]
fn fold_1000_nodes(b: &mut Bencher) {
    let pattern = create_pattern_with_n_nodes(1000);
    b.iter(|| pattern.fold(0, |acc, v| acc + v));
}

#[bench]
fn fold_100_depth(b: &mut Bencher) {
    let pattern = create_deeply_nested_pattern(100);
    b.iter(|| pattern.fold(0, |acc, v| acc + v));
}
```

**Acceptance Criteria**:
- ✅ 1000 nodes: <10ms
- ✅ 100 nesting levels: No stack overflow
- ✅ Memory usage: <100MB for 10,000 elements
- ✅ Performance comparable to or better than Haskell

### Phase 5: Integration & Documentation

**Tasks**:
1. Update crate documentation
2. Add examples to README
3. Verify WASM compilation
4. Update TODO.md feature checklist
5. Create verification summary

**Acceptance Criteria**:
- ✅ Feature marked complete in TODO.md
- ✅ Documentation examples compile and run
- ✅ WASM target compiles successfully
- ✅ All success criteria from spec.md verified

## Implementation Approach: Idiomatic Rust

### Core Design Principles

1. **Direct methods, not traits**: No Foldable trait, just `fold` method on `Pattern<V>`
2. **Borrow, don't consume**: `&self` receiver preserves pattern for reuse
3. **Match Rust conventions**: `fold` name, accumulator-first parameter order
4. **Helper method pattern**: Public API ergonomic, internal API efficient
5. **Type flexibility**: Accumulator type `B` independent of value type `V`

### Why This Is Idiomatic Rust

- ✅ **Standard library pattern**: Matches `Iterator::fold`
- ✅ **Clear naming**: Rust developers immediately understand `fold`
- ✅ **Borrowing model**: Pattern reusable after fold
- ✅ **Type inference**: Works seamlessly with Rust's type system
- ✅ **No traits needed**: Simple method, no complex trait hierarchies
- ✅ **Composable**: Can fold multiple times, compose with map

### Behavioral Equivalence with Haskell

**Haskell**:
```haskell
foldMap f (Pattern v es) = f v <> foldMap (foldMap f) es
```

**Rust equivalent**:
```rust
let acc = f(acc, &self.value);  // f v
self.elements.iter().fold(acc, |acc, e| e.fold_with(acc, f))  // <> foldMap (foldMap f) es
```

**Key Properties** (must be maintained):
1. **Order**: Root first, then elements left-to-right, depth-first
2. **Completeness**: All values processed exactly once
3. **Structure preservation**: Pattern unchanged after fold
4. **Type flexibility**: Can fold to any type B

## Testing Strategy

### Property-Based Tests (Primary)

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn fold_count_equals_size(pattern in arbitrary_pattern()) {
        let count = pattern.fold(0, |acc, _: &i32| acc + 1);
        prop_assert_eq!(count, pattern.size());
    }

    #[test]
    fn values_length_equals_size(pattern in arbitrary_pattern()) {
        let values = pattern.values();
        prop_assert_eq!(values.len(), pattern.size());
    }

    #[test]
    fn pattern_unchanged_after_fold(pattern in arbitrary_pattern()) {
        let original = pattern.clone();
        let _ = pattern.fold(0, |acc, _: &i32| acc + 1);
        prop_assert_eq!(pattern, original);
    }
}
```

### Unit Tests (Ported from gram-hs)

Comprehensive test coverage matching gram-hs test suite:
- Atomic patterns
- Flat patterns (one level)
- Nested patterns (multiple levels)
- Different value types (int, string, custom types)
- Order verification (non-commutative operations)
- Type transformations

See `../pattern-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs` lines 1054-1499 for reference tests.

## Success Criteria

From spec.md (all must pass):

- ✅ **SC-001**: Fold operations correctly process all values in patterns with 1000 nodes
- ✅ **SC-002**: Fold operations complete on patterns with 1000 nodes in under 10 milliseconds
- ✅ **SC-003**: Fold operations complete on patterns with 100 nesting levels without stack overflow
- ✅ **SC-004**: Converting patterns to collections preserves exact order of values
- ✅ **SC-005**: 100% of existing gram-hs foldable tests are ported and pass
- ✅ **SC-006**: Foldable implementation compiles for WASM target without errors
- ✅ **SC-007**: Custom folding functions work correctly across all pattern structures
- ✅ **SC-008**: Fold operations use constant stack space or handle deep recursion gracefully
- ✅ **SC-009**: Pattern structures with 10,000 elements can be folded without exceeding 100MB

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Stack overflow on deep nesting | Low | High | Test with 100+ levels early; Rust has good stack management; use iterative approach if needed |
| Performance issues on large patterns | Low | Medium | Benchmark early with 1000+ nodes; fold is inherently O(n) |
| Traversal order confusion | Medium | Medium | Clear documentation with examples; comprehensive order tests |
| Integration issues with map | Very Low | Low | Test composition early in development |

## Dependencies

- Feature 004: Pattern Data Structure (complete) - Defines `Pattern<V>` type
- Feature 005: Basic Pattern Type (complete) - Defines construction and access functions
- Feature 008: Functor Instance (complete) - Provides `map` for integration testing
- Rust standard library `Fn` trait

## Next Steps (Phase 2)

1. Implement `fold`, `fold_with`, and `values` methods
2. Add comprehensive documentation
3. Port tests from gram-hs
4. Run `/speckit.tasks` to generate detailed task breakdown

## References

- **Haskell Implementation**: `../pattern-hs/libs/pattern/src/Pattern/Core.hs` (line 750-751)
- **Haskell Tests**: `../pattern-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs` (lines 1054-1499)
- **Porting Guide**: `docs/porting-guide.md` - Section on "Idiomatic Rust vs Literal Translation"
- **Rust Standard Library**: `Iterator::fold` for API conventions
- **Feature 008**: Functor instance plan for consistency
