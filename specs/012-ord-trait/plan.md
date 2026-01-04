# Implementation Plan: Pattern Ordering and Comparison

**Branch**: `012-ord-trait` | **Date**: 2025-01-04 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/012-ord-trait/spec.md`

## Summary

Implement PartialOrd and Ord traits for Pattern<V> to enable deterministic ordering and comparison operations. This feature ports the Haskell Ord typeclass instance, allowing patterns to be sorted, compared, and used in ordered data structures (BTreeMap, BTreeSet, binary heaps). The implementation uses a value-first comparison strategy: compare pattern values first, then recursively compare elements if values are equal.

**Primary Value**: Unlocks pattern-based algorithms requiring ordering (sorting, min/max, ordered collections) while maintaining behavioral equivalence with the gram-hs reference implementation.

## Technical Context

**Language/Version**: Rust 1.70.0+ (workspace MSRV), Edition 2021  
**Primary Dependencies**: None (core library functionality, uses std::cmp only)  
**Storage**: N/A (pure computation, no persistence)  
**Testing**: cargo test (unit tests), proptest (property-based testing for ordering invariants)  
**Target Platform**: Multi-target (native Rust, WASM)  
**Project Type**: Single (library crate)  
**Performance Goals**: 
- Sort 10,000 patterns in <200ms
- Compare deep patterns (200+ levels) without stack overflow
- Compare wide patterns (5,000+ elements) in <500ms  
**Constraints**: 
- Must work in WASM (no OS-specific dependencies)
- Must not introduce stack overflow for deep recursion
- Must maintain behavioral equivalence with gram-hs Ord instance  
**Scale/Scope**: 
- Core trait implementations (PartialOrd, Ord)
- Property-based tests for ordering invariants
- 20-30 test cases ported from gram-hs

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity ✅

- **Status**: Will be verified
- **Action**: Port Ord typeclass instance from `../gram-hs/libs/pattern/src/Pattern.hs`
- **Verification**: Behavioral equivalence tests comparing with gram-hs reference
- **Reference Spec**: `../gram-hs/specs/009-ord-instance/` (for context, verify against actual code)

### II. Correctness & Compatibility ✅

- **Status**: Non-negotiable priority
- **Action**: 
  - Property-based tests for mathematical properties (transitivity, asymmetry, consistency with equality)
  - Test ordering consistency with PartialEq/Eq implementations
  - Verify no breaking changes to existing Pattern API
- **Verification**: 
  - proptest verification of Ord laws
  - Comparison with gram-hs test suite
  - Cargo test passes with no regressions

### III. Rust Native Idioms ✅

- **Status**: Following Rust conventions
- **Approach**: 
  - Use standard library Ord trait (not custom trait)
  - Implement PartialOrd first, then Ord (following Rust trait hierarchy)
  - Use std::cmp::Ordering enum (not custom ordering type)
  - Follow derive-friendly patterns where possible
- **Justification**: Rust's Ord trait is the idiomatic way to define ordering, matches std library conventions

### IV. Multi-Target Library Design ✅

- **Status**: Compatible
- **Action**: 
  - No platform-specific code needed (pure comparison logic)
  - Test compilation for both native and WASM targets
- **Verification**: `cargo build --target wasm32-unknown-unknown` succeeds

### V. External Language Bindings & Examples ✅

- **Status**: No changes needed
- **Justification**: Ordering is internal to Rust; WASM bindings can expose comparison results if needed in future

**Note**: When porting features from gram-hs, reference the local implementation at `../gram-hs` and corresponding feature specifications in `../gram-hs/specs/`. See [PORTING_GUIDE.md](../../../PORTING_GUIDE.md) for detailed porting instructions.

## Project Structure

### Documentation (this feature)

```text
specs/012-ord-trait/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (Ord instance semantics from gram-hs)
├── data-model.md        # Phase 1 output (ordering relationships)
├── quickstart.md        # Phase 1 output (usage examples)
├── contracts/           # Phase 1 output (type signatures, trait definitions)
│   └── type-signatures.md
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/pattern-core/
├── src/
│   └── pattern.rs       # Add PartialOrd and Ord implementations
├── tests/
│   ├── ord_basic.rs     # Basic comparison tests (atomic, nested patterns)
│   ├── ord_property.rs  # Property-based tests (transitivity, asymmetry, etc.)
│   ├── ord_collections.rs # Tests with BTreeMap, BTreeSet, binary heaps
│   └── ord_equivalence.rs # Behavioral equivalence with gram-hs
└── benches/
    └── ord_benchmarks.rs # Performance benchmarks

```

**Structure Decision**: Single project layout - all changes are within the existing `crates/pattern-core` crate. No new crates or dependencies required.

## Complexity Tracking

> **No violations** - All constitution checks passed without requiring justification.

## Phase 0: Research & Analysis

**Objective**: Understand gram-hs Ord implementation and ordering semantics

### Research Tasks

1. **Haskell Ord Instance Analysis**
   - **Source**: `../gram-hs/libs/pattern/src/Pattern.hs`
   - **Focus**: How does the Ord instance compare patterns?
   - **Questions**:
     - Is it lexicographic comparison (value first, then elements)?
     - How does it handle element-by-element comparison?
     - Are there any special cases (atomic patterns, equal values)?

2. **Rust Ord Trait Requirements**
   - **Source**: Rust standard library documentation
   - **Focus**: What are the trait requirements and laws?
   - **Questions**:
     - What are the relationships between PartialOrd, Ord, PartialEq, Eq?
     - What methods must be implemented?
     - What properties must hold (transitivity, asymmetry, etc.)?

3. **Comparison Strategy for Recursive Structures**
   - **Focus**: How to efficiently compare nested patterns
   - **Questions**:
     - Should comparison be recursive or iterative?
     - How to handle deep nesting without stack overflow?
     - How to short-circuit comparison early?

4. **Property-Based Testing for Ordering**
   - **Source**: proptest documentation, existing pattern tests
   - **Focus**: What properties need verification?
   - **Questions**:
     - What are the Ord laws that must hold?
     - How to generate test cases for deep/wide patterns?
     - What are common edge cases for ordering?

**Output**: `research.md` with decisions on:
- Exact comparison algorithm (lexicographic? other?)
- Implementation strategy (recursive vs iterative)
- Property tests needed for verification
- Performance considerations for large patterns

## Phase 1: Design & Contracts

**Prerequisites**: `research.md` complete with comparison strategy defined

### 1. Data Model (`data-model.md`)

**Entities**:

- **Pattern<V>**: The type being ordered (existing struct)
  - Fields: `value: V`, `elements: Vec<Pattern<V>>`
  - Constraints: V must implement PartialOrd (for PartialOrd) or Ord (for Ord)
  
- **Ordering**: Result of comparison (std::cmp::Ordering enum)
  - Values: Less, Equal, Greater
  - Used by: PartialOrd::partial_cmp, Ord::cmp

**Relationships**:

```
Pattern<V> where V: Ord
    ↓ implements
Ord trait
    ↓ requires
PartialOrd, Eq, PartialEq (already implemented)
    ↓ provides
cmp() method → Ordering
```

**Comparison Algorithm** (from research):

```
compare(pattern1, pattern2):
  1. Compare values: pattern1.value vs pattern2.value
     - If not equal, return result
  2. Compare element vectors lexicographically:
     - Compare lengths first (shorter < longer if prefixes match)
     - Then element-by-element comparison
  3. Return final ordering
```

### 2. API Contracts (`contracts/type-signatures.md`)

**Trait Implementations**:

```rust
// PartialOrd for patterns where V: PartialOrd
impl<V: PartialOrd> PartialOrd for Pattern<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
}

// Ord for patterns where V: Ord  
impl<V: Ord> Ord for Pattern<V> {
    fn cmp(&self, other: &Self) -> Ordering;
}
```

**Properties Guaranteed**:

1. **Reflexivity**: For all x, x == x implies !(x < x) and !(x > x)
2. **Antisymmetry**: For all x, y, if x < y then !(y < x)
3. **Transitivity**: For all x, y, z, if x < y and y < z then x < z
4. **Totality** (for Ord): For all x, y, exactly one of x < y, x == y, or x > y holds
5. **Consistency with Eq**: If x == y, then x.cmp(y) == Ordering::Equal

### 3. Quickstart Guide (`quickstart.md`)

**Basic Usage**:

```rust
use pattern_core::Pattern;

// Comparing atomic patterns
let p1 = Pattern::point(1);
let p2 = Pattern::point(2);
assert!(p1 < p2);

// Comparing nested patterns
let p3 = Pattern::pattern(1, vec![Pattern::point(2)]);
let p4 = Pattern::pattern(1, vec![Pattern::point(3)]);
assert!(p3 < p4);  // Same value, elements differ

// Sorting patterns
let mut patterns = vec![p2, p1];
patterns.sort();
assert_eq!(patterns, vec![p1, p2]);

// Using in BTreeMap
use std::collections::BTreeMap;
let mut map = BTreeMap::new();
map.insert(p1, "first");
map.insert(p2, "second");
```

### 4. Agent Context Update

Run `.specify/scripts/bash/update-agent-context.sh cursor-agent` to update Cursor's context with:
- Ord trait implementation for Pattern
- Comparison semantics (value-first lexicographic)
- Property-based testing approach

## Phase 2: Implementation Tasks

**Output of `/speckit.tasks` command** (not generated by this plan command)

Expected tasks will include:
1. Implement PartialOrd trait for Pattern<V> where V: PartialOrd
2. Implement Ord trait for Pattern<V> where V: Ord
3. Write basic comparison tests (atomic, nested, equal values)
4. Write property-based tests (transitivity, asymmetry, etc.)
5. Write collection integration tests (BTreeMap, BTreeSet, binary heap)
6. Port and verify equivalence tests from gram-hs
7. Write performance benchmarks
8. Update documentation with ordering semantics

## Success Verification

### Test Coverage

- **Unit Tests**: 20-30 tests covering:
  - Atomic pattern comparison
  - Nested pattern comparison
  - Edge cases (equal values, different structures)
  - Deep nesting (200+ levels)
  - Wide patterns (5,000+ elements)

- **Property Tests**: Verify Ord laws:
  - Reflexivity: !(x < x) for all x
  - Antisymmetry: if x < y then !(y < x)
  - Transitivity: if x < y and y < z then x < z
  - Totality: exactly one of x < y, x == y, x > y
  - Consistency with Eq: x == y implies x.cmp(y) == Equal

- **Integration Tests**: Verify usage in:
  - BTreeMap (pattern keys)
  - BTreeSet (pattern elements)
  - Binary heap (pattern priorities)
  - Standard sorting (Vec::sort)

### Performance Targets

- ✅ Sort 10,000 patterns in <200ms
- ✅ Compare deep patterns (200+ levels) without stack overflow
- ✅ Compare wide patterns (5,000+ elements) in <500ms

### Behavioral Equivalence

- ✅ 100% of ported test cases from gram-hs pass
- ✅ Comparison results match gram-hs for identical test inputs
- ✅ Ordering is consistent with gram-hs ordering semantics

## Risk Assessment

### Low Risk

- **Standard trait implementation**: Ord is well-understood in Rust
- **Existing Eq implementation**: PartialEq/Eq already working, provides foundation
- **No new dependencies**: Pure Rust, no external libs needed

### Medium Risk

- **Stack overflow for deep patterns**: Mitigated by testing with 200+ level patterns
- **Performance for wide patterns**: Mitigated by benchmarking with 5,000+ element patterns
- **Semantic drift from gram-hs**: Mitigated by comprehensive equivalence testing

### Mitigation Strategies

1. **Stack overflow**: If recursive comparison causes issues, use iterative comparison with explicit stack
2. **Performance**: Profile and optimize hot paths, consider short-circuit optimizations
3. **Behavioral equivalence**: Port all gram-hs Ord tests, verify outputs match exactly

## Dependencies

### Upstream (must exist before implementation)

- ✅ Pattern<V> type (feature 004)
- ✅ PartialEq and Eq implementations (feature 004)
- ✅ Property testing infrastructure with proptest (feature 003)

### Downstream (will use this feature)

- Future features requiring pattern ordering (sorting, indexing, caching)
- Ordered data structure integrations (pattern stores, indices)

## References

- **Haskell Source**: `../gram-hs/libs/pattern/src/Pattern.hs` (Ord instance)
- **Haskell Spec**: `../gram-hs/specs/009-ord-instance/` (development notes)
- **Rust Ord Trait**: https://doc.rust-lang.org/std/cmp/trait.Ord.html
- **Porting Guide**: `../../../PORTING_GUIDE.md`
