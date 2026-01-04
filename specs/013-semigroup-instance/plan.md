# Implementation Plan: Pattern Combination Operations

**Branch**: `013-semigroup-instance` | **Date**: 2026-01-04 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/013-semigroup-instance/spec.md`

## Summary

Implement associative combination operations for Pattern<V> to enable merging two patterns into a single pattern. This feature ports the Semigroup instance from the gram-hs Haskell reference implementation, but follows idiomatic Rust patterns rather than creating custom algebraic typeclasses. The implementation will provide a concrete method (or use std::ops trait if semantically appropriate) that satisfies the associativity law: (a ⊕ b) ⊕ c = a ⊕ (b ⊕ c).

**Primary Value**: Enables compositional pattern construction, allowing developers to build complex patterns incrementally from simpler components while maintaining predictable associative behavior.

## Technical Context

**Language/Version**: Rust 1.70.0+ (workspace MSRV), Edition 2021  
**Primary Dependencies**: None (core library functionality)  
**Storage**: N/A (pure computation, no persistence)  
**Testing**: cargo test (unit tests), proptest (property-based testing for associativity law)  
**Target Platform**: Multi-target (native Rust, WASM)  
**Project Type**: Single (library crate)  
**Performance Goals**: 
- Combine two patterns with 1000 elements each in <1ms
- Combine deep patterns (100+ levels) without stack overflow
- Fold 100 patterns in sequence in <100ms  
**Constraints**: 
- Must work in WASM (no OS-specific dependencies)
- Must maintain behavioral equivalence with gram-hs Semigroup instance
- Must follow idiomatic Rust patterns (no custom Semigroup trait unless justified)  
**Scale/Scope**: 
- Core combination operation implementation
- Property-based tests for associativity law
- 20-30 test cases ported from gram-hs
- Integration with existing Pattern operations (fold, map, etc.)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity ✅

- **Status**: Will be verified
- **Action**: Port Semigroup instance from `../gram-hs/libs/pattern/src/Pattern.hs`
- **Verification**: Behavioral equivalence tests comparing with gram-hs reference
- **Reference Spec**: `../gram-hs/specs/010-semigroup-instance/` (for context, verify against actual code)

### II. Correctness & Compatibility ✅

- **Status**: Non-negotiable priority
- **Action**: 
  - Property-based tests for associativity law
  - Test combination with various pattern structures (atomic, nested, wide, deep)
  - Verify no breaking changes to existing Pattern API
- **Verification**: 
  - proptest verification of associativity: (a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)
  - Comparison with gram-hs test suite
  - Cargo test passes with no regressions

### III. Rust Native Idioms ✅

- **Status**: Following Rust conventions
- **Approach**: 
  - **Research needed**: Determine if concrete method (e.g., `combine()`, `append()`) or std::ops::Add is more appropriate
  - Will NOT create custom Semigroup trait (non-idiomatic in Rust ecosystem)
  - Follow patterns established by existing Pattern methods (map, fold, traverse)
  - Use Rust ownership and borrowing idiomatically
- **Justification**: Custom algebraic typeclasses are non-idiomatic in Rust; prefer concrete methods or standard library traits

### IV. Multi-Target Library Design ✅

- **Status**: Compatible
- **Action**: 
  - No platform-specific code needed (pure combination logic)
  - Test compilation for both native and WASM targets
- **Verification**: `cargo build --target wasm32-unknown-unknown` succeeds

### V. External Language Bindings & Examples ✅

- **Status**: Future consideration
- **Justification**: Combination operations are internal to Rust; WASM bindings may expose combination functionality in future if needed

**Note**: When porting features from gram-hs, reference the local implementation at `../gram-hs` and corresponding feature specifications in `../gram-hs/specs/`. See [PORTING_GUIDE.md](../../../PORTING_GUIDE.md) for detailed porting instructions.

## Project Structure

### Documentation (this feature)

```text
specs/013-semigroup-instance/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (combination semantics from gram-hs)
├── data-model.md        # Phase 1 output (combination operation model)
├── quickstart.md        # Phase 1 output (usage examples)
├── contracts/           # Phase 1 output (type signatures, API definitions)
│   └── type-signatures.md
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/pattern-core/
├── src/
│   └── pattern.rs       # Add combination operation implementation
├── tests/
│   ├── semigroup_basic.rs      # Basic combination tests
│   ├── semigroup_property.rs   # Property-based tests (associativity)
│   ├── semigroup_integration.rs # Integration with fold, iterator methods
│   └── semigroup_equivalence.rs # Behavioral equivalence with gram-hs
└── benches/
    └── semigroup_benchmarks.rs # Performance benchmarks

```

**Structure Decision**: Single project layout - all changes are within the existing `crates/pattern-core` crate. No new crates or dependencies required.

## Complexity Tracking

> **No violations** - All constitution checks passed without requiring justification.

## Phase 0: Research & Analysis

**Objective**: Understand gram-hs Semigroup implementation and determine idiomatic Rust approach

### Research Tasks

1. **Haskell Semigroup Instance Analysis**
   - **Source**: `../gram-hs/libs/pattern/src/Pattern.hs`
   - **Focus**: How does the Semigroup instance combine patterns?
   - **Questions**:
     - What is the exact combination semantics? (value combination + element concatenation? other?)
     - How are values combined? (uses V's Semigroup instance? other strategy?)
     - How are element lists combined? (concatenation? merging? other?)
     - Are there any special cases (atomic patterns, empty elements)?
     - What constraints does it place on the value type V?

2. **Idiomatic Rust Approach Decision**
   - **Source**: Rust standard library patterns, existing Pattern methods
   - **Focus**: What's the most idiomatic way to express this in Rust?
   - **Questions**:
     - Should we use a concrete method (like `combine()` or `append()`)?
     - Would `std::ops::Add` be semantically appropriate?
     - How do similar Rust libraries handle combination operations?
     - What pattern do existing Pattern methods follow (map, fold, traverse)?
   - **Decision Criteria**:
     - Semantic fit (does `+` operator make sense for patterns?)
     - Consistency with existing API (matches `map()`, `fold()` style?)
     - Rust ecosystem conventions (what do similar libraries do?)

3. **Value Type Requirements**
   - **Focus**: What must value type V support for combination?
   - **Questions**:
     - Does V need to implement any trait?
     - How to handle different value types (String, Subject, custom types)?
     - Should we provide multiple combination strategies?
     - Can we make it work for any V, or only certain types?

4. **Property-Based Testing Strategy**
   - **Source**: proptest documentation, existing pattern tests
   - **Focus**: How to verify associativity law?
   - **Questions**:
     - What are the property tests needed?
     - How to generate test patterns for deep/wide structures?
     - What edge cases need coverage?
     - How to test equivalence with gram-hs?

**Output**: `research.md` with decisions on:
- Exact combination semantics (how values and elements are combined)
- API design decision (concrete method name and signature OR trait implementation)
- Value type requirements and constraints
- Property tests needed for verification
- Performance considerations

## Phase 1: Design & Contracts

**Prerequisites**: `research.md` complete with combination semantics and API design decided

### 1. Data Model (`data-model.md`)

**Entities**:

- **Pattern<V>**: The type being combined (existing struct)
  - Fields: `value: V`, `elements: Vec<Pattern<V>>`
  - Constraints: V must support combination (exact requirement from research)
  
- **Combination Operation**: Binary operation that merges patterns
  - Input: Two Pattern<V> instances
  - Output: New Pattern<V> with combined structure
  - Properties: Must be associative

**Combination Semantics** (from research):

```
combine(pattern1, pattern2):
  1. Combine values: pattern1.value ⊕ pattern2.value
     [exact semantics determined in research]
  2. Combine element vectors: pattern1.elements ⊕ pattern2.elements
     [exact semantics determined in research - likely concatenation]
  3. Return new pattern with combined value and elements
```

**Associativity Property**:

```
For all patterns a, b, c:
  combine(combine(a, b), c) == combine(a, combine(b, c))
```

### 2. API Contracts (`contracts/type-signatures.md`)

**API Design** (determined in research):

Option A: Concrete method
```rust
impl<V> Pattern<V> 
where
    V: /* trait for combinable values */
{
    pub fn combine(self, other: Self) -> Self;
}
```

Option B: std::ops::Add (if semantically appropriate)
```rust
impl<V> Add for Pattern<V>
where
    V: Add<Output = V>
{
    type Output = Pattern<V>;
    
    fn add(self, other: Self) -> Self::Output;
}
```

[Actual choice documented in research.md]

**Properties Guaranteed**:

1. **Associativity**: For all a, b, c: (a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)
2. **Type safety**: Combination only available when V supports it
3. **Structural validity**: Result is always a well-formed Pattern
4. **Behavioral equivalence**: Matches gram-hs Semigroup instance behavior

### 3. Quickstart Guide (`quickstart.md`)

**Basic Usage** (actual syntax depends on research decision):

```rust
use pattern_core::Pattern;

// Combining atomic patterns
let p1 = Pattern::point("hello");
let p2 = Pattern::point("world");
let combined = p1.combine(p2); // or p1 + p2 if using Add

// Combining nested patterns
let p3 = Pattern::pattern("a", vec![Pattern::point("b")]);
let p4 = Pattern::pattern("c", vec![Pattern::point("d")]);
let merged = p3.combine(p4);

// Combining multiple patterns (fold/reduce)
let patterns = vec![p1, p2, p3, p4];
let result = patterns.into_iter()
    .reduce(|acc, p| acc.combine(p))
    .unwrap();

// Associativity is guaranteed
let a = Pattern::point(1);
let b = Pattern::point(2);
let c = Pattern::point(3);
assert_eq!(
    a.clone().combine(b.clone()).combine(c.clone()),
    a.combine(b.combine(c))
);
```

### 4. Agent Context Update

Run `.specify/scripts/bash/update-agent-context.sh cursor-agent` to update Cursor's context with:
- Combination operation API (method or trait)
- Combination semantics (value combination + element handling)
- Associativity property and testing approach
- Integration with existing Pattern operations

## Phase 2: Implementation Tasks

**Output of `/speckit.tasks` command** (not generated by this plan command)

Expected tasks will include:
1. Implement combination operation (method or trait based on research)
2. Write basic combination tests (atomic patterns, nested patterns, various structures)
3. Write property-based tests for associativity law
4. Port and verify equivalence tests from gram-hs
5. Write integration tests with fold/reduce operations
6. Write performance benchmarks
7. Update Pattern API documentation
8. Verify WASM compatibility

## Success Verification

### Test Coverage

- **Unit Tests**: 20-30 tests covering:
  - Atomic pattern combination
  - Nested pattern combination
  - Patterns with different element counts
  - Edge cases (empty elements, deep nesting, wide patterns)

- **Property Tests**: Verify associativity law:
  - For randomly generated pattern triples (a, b, c)
  - Verify: (a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)
  - Test with 10,000+ random cases
  - Cover various pattern structures (atomic, nested, mixed depths)

- **Integration Tests**: Verify usage with:
  - Iterator fold/reduce operations
  - Combining pattern collections
  - Integration with existing map/fold/traverse operations

### Performance Targets

- ✅ Combine two patterns with 1000 elements each in <1ms
- ✅ Combine deep patterns (100+ levels) without stack overflow
- ✅ Fold 100 patterns in sequence in <100ms

### Behavioral Equivalence

- ✅ 100% of ported test cases from gram-hs pass
- ✅ Combination results match gram-hs for identical test inputs
- ✅ Combination semantics match gram-hs Semigroup instance

## Risk Assessment

### Low Risk

- **Well-defined mathematical property**: Associativity is clear and testable
- **Existing pattern infrastructure**: Map, fold, traverse already working
- **No new dependencies**: Pure Rust, no external libs needed

### Medium Risk

- **API design decision**: Choosing between concrete method vs trait requires careful consideration
  - Mitigation: Research phase will evaluate both options against Rust idioms
- **Value type constraints**: Need to determine what V must support
  - Mitigation: Research gram-hs implementation to understand exact requirements
- **Stack overflow for deep patterns**: Recursive combination might cause issues
  - Mitigation: Test with 100+ level patterns, use iterative approach if needed

### Mitigation Strategies

1. **API design**: Evaluate both concrete method and std::ops::Add, choose based on semantic fit and consistency with existing API
2. **Value type constraints**: Define trait bounds clearly based on gram-hs requirements
3. **Stack overflow**: If recursive combination causes issues, implement iterative version with explicit stack
4. **Behavioral equivalence**: Port all gram-hs Semigroup tests, verify outputs match exactly

## Dependencies

### Upstream (must exist before implementation)

- ✅ Pattern<V> type (feature 004)
- ✅ Pattern construction methods (point, pattern) (feature 005)
- ✅ Property testing infrastructure with proptest (feature 003)

### Downstream (will use this feature)

- Feature 014: Monoid instance (identity element for combination)
- Future pattern building and composition features
- Pattern collection operations (concatenation, merging)

## References

- **Haskell Source**: `../gram-hs/libs/pattern/src/Pattern.hs` (Semigroup instance)
- **Haskell Spec**: `../gram-hs/specs/010-semigroup-instance/` (development notes)
- **Rust Idioms**: https://rust-lang.github.io/api-guidelines/
- **Porting Guide**: `../../../PORTING_GUIDE.md`
