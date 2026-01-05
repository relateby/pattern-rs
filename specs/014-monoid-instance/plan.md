# Implementation Plan: Pattern Identity Element via Default Trait

**Branch**: `014-monoid-instance` | **Date**: 2026-01-05 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/014-monoid-instance/spec.md`

## Summary

Implement identity element support for Pattern<V> by adding a `Default` trait implementation that creates the "empty" or "neutral" pattern for combination operations. This completes the monoid algebraic structure (associative operation from feature 013 + identity element). The implementation follows idiomatic Rust conventions by using the standard library's `Default` trait rather than creating a custom Monoid trait, with monoid laws documented and verified through comprehensive property-based testing.

**Primary Value**: Enables clean iterator accumulation patterns with `fold`, provides a well-defined starting point for incremental pattern construction, and handles empty collections naturally while maintaining mathematical rigor through monoid laws.

## Technical Context

**Language/Version**: Rust 1.70.0+ (workspace MSRV), Edition 2021  
**Primary Dependencies**: None (standard library trait implementation)  
**Storage**: N/A (pure computation, no persistence)  
**Testing**: cargo test (unit tests), proptest (property-based testing for monoid laws)  
**Target Platform**: Multi-target (native Rust, WASM)  
**Project Type**: Single (library crate)  
**Performance Goals**: 
- Default pattern creation is O(1) (instantaneous)
- Combining with default pattern adds no overhead vs normal combination
- Identity laws verified for 10,000+ randomly generated patterns  
**Constraints**: 
- Must work in WASM (no OS-specific dependencies)
- Must maintain behavioral equivalence with gram-hs Monoid instance
- Must follow idiomatic Rust (use std::default::Default, not custom Monoid trait)  
- Must satisfy monoid identity laws
**Scale/Scope**: 
- Single trait implementation for Pattern<V> where V: Default
- Property-based tests for left and right identity laws
- Unit tests for common value types (String, Vec, (), integers)
- Integration with existing pattern operations and iterator methods

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity ✅

- **Status**: Will be verified
- **Action**: Port Monoid instance identity semantics from `../gram-hs/libs/pattern/src/Pattern.hs`
- **Verification**: Behavioral equivalence tests comparing with gram-hs reference
- **Reference Spec**: `../gram-hs/specs/011-monoid-instance/` (for context, verify against actual code)
- **Note**: Implementation uses `Default` trait (idiomatic Rust) instead of custom Monoid trait, but semantics remain equivalent

### II. Correctness & Compatibility ✅

- **Status**: Non-negotiable priority
- **Action**: 
  - Property-based tests for left identity law: `empty.combine(x) == x`
  - Property-based tests for right identity law: `x.combine(empty) == x`
  - Test with various pattern structures (atomic, nested, wide, deep)
  - Test with multiple value types (String, Vec, (), i32)
  - Verify no breaking changes to existing Pattern API
- **Verification**: 
  - proptest verification of both identity laws with 10,000+ patterns
  - Comparison with gram-hs test suite
  - Cargo test passes with no regressions
  - Integration tests show natural usage with iterators

### III. Rust Native Idioms ✅

- **Status**: Following Rust conventions
- **Approach**: 
  - Use `std::default::Default` trait (standard library, familiar to all Rust developers)
  - Will NOT create custom Monoid trait (non-idiomatic in Rust ecosystem)
  - Document monoid laws in code comments and test descriptions
  - Verify laws through comprehensive property-based testing
  - Follow patterns established by existing Pattern methods
- **Justification**: Rust ecosystem uses standard library traits, not custom algebraic typeclasses. `Default` provides the practical benefit of identity values while monoid laws are documented and tested rather than encoded in type system.

### IV. Multi-Target Library Design ✅

- **Status**: Compatible
- **Action**: 
  - No platform-specific code needed (pure trait implementation)
  - Test compilation for both native and WASM targets
- **Verification**: `cargo build --target wasm32-unknown-unknown` succeeds

### V. External Language Bindings & Examples ✅

- **Status**: Future consideration
- **Justification**: Default patterns are internal to Rust; WASM bindings may expose default pattern functionality in future if needed for JavaScript interop

**Note**: When porting features from gram-hs, reference the local implementation at `../gram-hs` and corresponding feature specifications in `../gram-hs/specs/`. See [PORTING_GUIDE.md](../../../PORTING_GUIDE.md) for detailed porting instructions.

## Project Structure

### Documentation (this feature)

```text
specs/014-monoid-instance/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Already created (Default vs Monoid trait decision)
├── data-model.md        # Already created (monoid structure and laws)
├── quickstart.md        # Already created (usage examples)
├── contracts/           # Already created (type signatures)
│   └── type-signatures.md
└── tasks.md             # Phase 2 output (/speckit.tasks command - to be created)
```

### Source Code (repository root)

```text
crates/pattern-core/
├── src/
│   ├── lib.rs           # Re-export Default trait impl (no changes needed)
│   └── pattern.rs       # Add Default trait implementation
├── tests/
│   ├── monoid_default.rs      # Basic default creation tests
│   ├── monoid_identity.rs     # Property-based identity law tests
│   ├── monoid_integration.rs  # Integration with iterators and fold
│   └── monoid_equivalence.rs  # Behavioral equivalence with gram-hs
└── benches/
    └── (reuse existing semigroup_benchmarks.rs) # Default is trivial to benchmark

```

**Structure Decision**: Single project layout - all changes are within the existing `crates/pattern-core` crate. No new crates or dependencies required. This is a purely additive feature.

## Complexity Tracking

> **No violations** - All constitution checks passed without requiring justification. The decision to use `Default` instead of a custom `Monoid` trait is explicitly aligned with Rust idioms (Constitution III).

## Phase 0: Research & Analysis

**Objective**: Understand gram-hs Monoid implementation and confirm Default trait approach

### Research Tasks ✅ COMPLETE

Research has been completed and documented in [research.md](research.md). Key findings:

1. **Haskell Monoid Instance Analysis** ✅
   - **Source**: `../gram-hs/libs/pattern/src/Pattern.hs`
   - **Finding**: Identity pattern is `Pattern mempty []` where `mempty` is the value type's monoid identity
   - **Combination**: Identity satisfies `mempty <> p = p` and `p <> mempty = p`
   - **Structure**: Default value component + empty elements list

2. **Idiomatic Rust Approach Decision** ✅
   - **Decision**: Use `std::default::Default` trait
   - **Rationale**: 
     - Idiomatic Rust using standard library
     - Works with existing ecosystem (mem::take, unwrap_or_default, etc.)
     - Familiar to all Rust developers
     - Already implemented by common types (String, Vec, etc.)
   - **Alternatives Considered**: Custom Monoid trait rejected as non-idiomatic
   - **Documentation Strategy**: Document monoid laws in comments and verify through testing

3. **Monoid Laws Testing Strategy** ✅
   - **Approach**: Property-based testing with proptest
   - **Laws to Verify**:
     - Left Identity: `Pattern::default().combine(x) == x`
     - Right Identity: `x.combine(Pattern::default()) == x`
   - **Coverage**: 10,000+ randomly generated patterns of various structures
   - **Value Types**: Test with String, Vec<T>, (), i32 for comprehensive coverage

4. **Integration with Existing Operations** ✅
   - Works naturally with `combine()` from feature 013
   - Enables clean `fold` patterns with default initial value
   - `map()` over default preserves identity
   - `values()` on default returns single default value

**Output**: `research.md` ✅ Complete with all decisions documented

## Phase 1: Design & Contracts

**Prerequisites**: ✅ `research.md` complete with Default trait approach confirmed

### 1. Data Model (`data-model.md`) ✅ COMPLETE

Already created and comprehensive. Key elements:

**Entities**:

- **Pattern<V>**: The type receiving Default implementation
  - Fields: `value: V`, `elements: Vec<Pattern<V>>`
  - Constraint: V must implement Default
  
- **Identity Pattern**: The default/empty pattern
  - Value: `V::default()` (default value for type V)
  - Elements: `vec![]` (empty vector)
  - Properties: Must satisfy left and right identity laws

**Monoid Laws**:

```rust
// Left Identity
∀ p: Pattern<V>, Pattern::default().combine(p) == p

// Right Identity
∀ p: Pattern<V>, p.combine(Pattern::default()) == p
```

**Type Constraints**:

```rust
impl<V: Default> Default for Pattern<V> {
    fn default() -> Self {
        Pattern::point(V::default())
    }
}
```

**Complete Monoid**: Requires both `Default` and `Combinable`:

```rust
Pattern<V> where V: Default + Combinable
```

### 2. API Contracts (`contracts/type-signatures.md`) ✅ COMPLETE

Already created with comprehensive type signatures:

**Default Trait Implementation**:

```rust
impl<V: Default> Default for Pattern<V> {
    fn default() -> Self
}
```

**Properties Guaranteed**:

1. **Left Identity**: For all patterns p: `Pattern::default().combine(p) == p`
2. **Right Identity**: For all patterns p: `p.combine(Pattern::default()) == p`
3. **Type safety**: Default only available when V implements Default
4. **Structural validity**: Result is always a well-formed Pattern
5. **Behavioral equivalence**: Matches gram-hs Monoid mempty behavior

**Integration with Standard Library**:

- Works with `mem::take()`, `unwrap_or_default()`, etc.
- Natural integration with iterator `fold` methods
- Compatible with all Rust standard library functions expecting Default

### 3. Quickstart Guide (`quickstart.md`) ✅ COMPLETE

Already created with practical examples:

**Basic Usage**:

```rust
use pattern_core::{Pattern, Combinable};

// Create identity pattern
let empty = Pattern::<String>::default();

// Identity laws
let p = Pattern::point("hello".to_string());
assert_eq!(empty.clone().combine(p.clone()), p);  // Left identity
assert_eq!(p.clone().combine(empty), p);           // Right identity

// Use with iterators
let result = patterns.into_iter()
    .fold(Pattern::default(), |acc, p| acc.combine(p));
```

**Common Patterns**:
- Pattern accumulation with fold
- Handling empty collections
- Building patterns incrementally
- Using default as placeholder

### 4. Agent Context Update

Since `.specify` scripts don't exist in this repository, I'll document what would be added to agent context:

**Pattern Default Implementation**:
- `Pattern<V>` implements `Default` where `V: Default`
- Creates identity pattern: `Pattern { value: V::default(), elements: vec![] }`
- Satisfies monoid identity laws with `combine()` operation
- Use with `fold(Pattern::default(), |acc, p| acc.combine(p))`

## Phase 2: Implementation Tasks

**Output of `/speckit.tasks` command** (to be generated separately)

Expected tasks will include:

### Core Implementation (Estimated: 1-2 hours)
1. Implement `Default` trait for `Pattern<V> where V: Default` in `src/pattern.rs`
2. Add comprehensive doc comments explaining monoid laws
3. Verify compilation and clippy compliance
4. Test WASM compilation

### Unit Tests (Estimated: 2-3 hours)
5. Test default creation for String patterns
6. Test default creation for Vec patterns
7. Test default creation for unit patterns
8. Test default creation for numeric types
9. Test identity laws with atomic patterns
10. Test identity laws with compound patterns
11. Test identity laws with deeply nested patterns

### Property-Based Tests (Estimated: 2-3 hours)
12. Implement/reuse pattern generators for proptest
13. Property test for left identity law with String patterns
14. Property test for right identity law with String patterns
15. Property test for left identity with Vec patterns
16. Property test for right identity with Vec patterns
17. Property test with edge cases (very deep nesting, many elements)

### Integration Tests (Estimated: 2-3 hours)
18. Test with iterator fold using default initial value
19. Test with empty collection fold returning default
20. Test with `reduce().unwrap_or_default()` pattern
21. Test with `mem::take()` standard library function
22. Test combination with existing pattern operations (map, values, etc.)
23. Test that default preserves identity under map

### Documentation (Estimated: 1-2 hours)
24. Add monoid law explanations to module docs in `src/pattern.rs`
25. Create usage examples in trait implementation doc comments
26. Update crate-level documentation in `src/lib.rs`
27. Verify all doc tests pass
28. Ensure examples compile and run correctly

### Verification (Estimated: 1-2 hours)
29. Compare behavior with gram-hs Haskell implementation
30. Port any additional monoid tests from gram-hs test suite
31. Run full test suite and ensure all tests pass
32. Verify no clippy warnings
33. Verify documentation builds without warnings
34. Confirm behavioral equivalence with gram-hs

**Total Estimated Time**: 10-15 hours (1-2 days)

## Success Verification

### Test Coverage

- **Unit Tests**: 15-20 tests covering:
  - Default pattern creation for multiple value types
  - Identity laws with atomic patterns
  - Identity laws with nested patterns
  - Identity laws with deeply nested patterns
  - Edge cases (empty elements, maximum depth patterns)

- **Property Tests**: Verify monoid identity laws:
  - For randomly generated pattern pairs (empty, x)
  - Verify: `Pattern::default().combine(x) == x` (left identity)
  - Verify: `x.combine(Pattern::default()) == x` (right identity)
  - Test with 10,000+ random cases per law
  - Cover various pattern structures and value types

- **Integration Tests**: Verify usage with:
  - Iterator fold with default initial value
  - Empty collection handling
  - Standard library functions (mem::take, unwrap_or_default)
  - Integration with existing map/fold/traverse operations

### Performance Targets

- ✅ Default pattern creation is O(1) - instantaneous
- ✅ No performance overhead when combining with default
- ✅ Property tests complete in reasonable time (<10 seconds for 20,000+ cases)

### Behavioral Equivalence

- ✅ 100% of ported test cases from gram-hs pass
- ✅ Identity behavior matches gram-hs Monoid instance
- ✅ Default pattern structure matches gram-hs mempty semantics

### Integration Quality

- ✅ Works naturally with iterator fold patterns
- ✅ Integrates with standard library Default-expecting functions
- ✅ No breaking changes to existing Pattern API
- ✅ Documentation clearly explains monoid laws and usage

## Risk Assessment

### Low Risk

- **Well-defined trait**: `Default` is a standard, simple trait
- **Clear semantics**: Identity laws are mathematically well-defined
- **No dependencies**: Uses only standard library
- **Existing infrastructure**: Combinable trait and tests already exist
- **Simple implementation**: Single straightforward trait impl

### Medium Risk

- **Value type constraints**: Need V to implement both Default and Combinable for full monoid
  - Mitigation: Document constraints clearly, provide examples for common types
- **Law verification**: Must ensure identity laws actually hold
  - Mitigation: Comprehensive property-based testing with many random patterns
- **Empty pattern semantics**: Must clarify what "empty" means for different value types
  - Mitigation: Document that empty = default value + no elements, test with multiple types

### Mitigation Strategies

1. **Type constraints**: Document clearly that full monoid requires `V: Default + Combinable`
2. **Law verification**: Use proptest with 10,000+ cases per law, test multiple value types
3. **Empty semantics**: Add clear doc comments explaining default pattern structure
4. **Behavioral equivalence**: Port gram-hs Monoid tests, verify outputs match exactly
5. **Integration testing**: Test with real-world iterator usage patterns

## Dependencies

### Upstream (must exist before implementation)

- ✅ Pattern<V> type (feature 004)
- ✅ Pattern construction methods (point, pattern) (feature 005)
- ✅ Combinable trait and combine() method (feature 013)
- ✅ Property testing infrastructure with proptest (feature 003)

### Downstream (will use this feature)

- Future features using pattern accumulation with default initial values
- Pattern collection operations that need identity element
- Any features requiring monoid-complete patterns

## References

- **Haskell Source**: `../gram-hs/libs/pattern/src/Pattern.hs` (Monoid instance)
- **Haskell Spec**: `../gram-hs/specs/011-monoid-instance/` (development notes)
- **Rust Default**: https://doc.rust-lang.org/std/default/trait.Default.html
- **Porting Guide**: `../../../PORTING_GUIDE.md`
- **Feature 013 Plan**: `../013-semigroup-instance/plan.md` (Semigroup/Combinable)

## Next Steps

1. ✅ **Phase 0 Complete**: Research documented in research.md
2. ✅ **Phase 1 Complete**: Data model, contracts, and quickstart created
3. **Phase 2 Pending**: Run `/speckit.tasks` to generate detailed task breakdown
4. **Implementation**: Follow task list to implement Default trait and tests
5. **Verification**: Ensure all success criteria met before feature completion

---

**Plan Status**: ✅ Complete and ready for task generation via `/speckit.tasks`

**Implementation Ready**: All design decisions made, all documentation complete, implementation can proceed immediately after task breakdown is generated.
