# Implementation Plan: Pattern Hashing via Hash Trait

**Branch**: `015-hashable-instance` | **Date**: 2026-01-05 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/015-hashable-instance/spec.md`

## Summary

Implement hashing support for Pattern<V> by adding a `std::hash::Hash` trait implementation that enables patterns to be used in HashMap and HashSet. This follows idiomatic Rust by using conditional trait bounds and leveraging the standard library's hashing infrastructure. The implementation is simple (~10 lines of code) but provides high practical value for pattern deduplication, caching, and set-based operations.

**Primary Value**: Enables O(1) average-case lookups in HashMap, O(n) deduplication in HashSet, and efficient set-theoretic operations, replacing O(n²) naive approaches for duplicate checking and search.

## Technical Context

**Language/Version**: Rust 1.70.0+ (workspace MSRV), Edition 2021  
**Primary Dependencies**: None (standard library trait implementation)  
**Storage**: N/A (pure computation, no persistence)  
**Testing**: cargo test (unit tests), proptest (property-based testing for hash laws)  
**Target Platform**: Multi-target (native Rust, WASM)  
**Project Type**: Single (library crate)  
**Performance Goals**: 
- Hash computation is O(n) in pattern size
- HashMap lookups remain O(1) average case
- HashSet deduplication is O(n) for n patterns
- Hash consistency verified for 10,000+ randomly generated patterns  
**Constraints**: 
- Must work in WASM (no OS-specific dependencies)
- Must maintain behavioral equivalence with gram-hs Hashable instance
- Must follow idiomatic Rust (use std::hash::Hash with conditional bounds)  
- Must be consistent with Eq implementation
**Scale/Scope**: 
- Single trait implementation for Pattern<V> where V: Hash
- Add Hash derive to Symbol type
- Property-based tests for hash/eq consistency
- Unit tests for HashMap and HashSet usage

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity ✅

- **Status**: Will be verified
- **Action**: Port Hashable instance semantics from `../pattern-hs/libs/pattern/src/Pattern/Core.hs` (lines 477-535)
- **Verification**: Behavioral equivalence tests comparing with gram-hs reference
- **Reference Spec**: `../pattern-hs/specs/012-hashable-instance/` (for context, verify against actual code)
- **Note**: Implementation uses `Hash` trait (idiomatic Rust), semantics remain equivalent

### II. Correctness & Compatibility ✅

- **Status**: Non-negotiable priority
- **Action**: 
  - Property-based tests for hash/eq consistency: if `p1 == p2` then `hash(p1) == hash(p2)`
  - Test with various pattern structures (atomic, nested, wide, deep)
  - Test that different structures produce different hashes
  - Verify Pattern<String> works, Pattern<Subject> correctly fails to compile
- **Verification**: 
  - proptest verification of hash/eq consistency with 10,000+ patterns
  - HashMap and HashSet integration tests
  - Comparison with gram-hs test suite
  - Cargo test passes with no regressions

### III. Rust Native Idioms ✅

- **Status**: Following Rust conventions
- **Approach**: 
  - Use `std::hash::Hash` trait (standard library)
  - Conditional trait bounds: `impl<V: Hash> Hash for Pattern<V>`
  - Leverage Vec's built-in Hash implementation for elements
  - Follow patterns established by existing Eq/Ord implementations
- **Justification**: Rust's type system with conditional trait bounds is the idiomatic way to express "hashable if value is hashable". Hash trait is universally understood by Rust developers.

### IV. Multi-Target Library Design ✅

- **Status**: Compatible
- **Action**: 
  - No platform-specific code needed (pure trait implementation)
  - Test compilation for both native and WASM targets
- **Verification**: `cargo build --target wasm32-unknown-unknown` succeeds

### V. External Language Bindings & Examples ✅

- **Status**: Future consideration
- **Justification**: Hash trait enables efficient JavaScript Map/Set usage in WASM bindings (future work)

**Note**: When porting features from gram-hs, reference the local implementation at `../pattern-hs` and corresponding feature specifications in `../pattern-hs/specs/`. See [porting guide](../../../docs/porting-guide.md) for detailed porting instructions.

## Project Structure

### Documentation (this feature)

```text
specs/015-hashable-instance/
├── ANALYSIS.md          # Already created (comprehensive analysis)
├── spec.md              # This feature specification
├── plan.md              # This file (implementation plan)
└── tasks.md             # Detailed task breakdown (to be created)
```

### Source Code (repository root)

```text
crates/pattern-core/
├── src/
│   ├── lib.rs           # Re-export Hash trait impl (no changes needed)
│   ├── pattern.rs       # Add Hash trait implementation
│   └── subject.rs       # Add Hash to Symbol derive macro
├── tests/
│   ├── hash_basic.rs           # Basic hash tests (creation, HashMap, HashSet)
│   ├── hash_consistency.rs     # Property-based hash/eq consistency tests
│   ├── hash_integration.rs     # Integration with collections
│   └── hash_equivalence.rs     # Behavioral equivalence with gram-hs
└── benches/
    └── (optional) hash_benchmarks.rs  # Performance validation if needed
```

**Structure Decision**: Single project layout - all changes within existing `crates/pattern-core` crate. No new crates or dependencies required. Purely additive feature.

## Complexity Tracking

> **No violations** - All constitution checks passed. Using standard Hash trait with conditional bounds is explicitly aligned with Rust idioms (Constitution III).

## Phase 0: Research & Analysis

**Objective**: Understand gram-hs Hashable implementation and confirm Hash trait approach

### Research Complete ✅

Research documented in [ANALYSIS.md](ANALYSIS.md). Key findings:

1. **Haskell Hashable Instance Analysis** ✅
   - **Source**: `../pattern-hs/libs/pattern/src/Pattern/Core.hs` lines 477-535
   - **Implementation**: `hashWithSalt salt (Pattern v es) = salt \`hashWithSalt\` v \`hashWithSalt\` es`
   - **Properties**: Structure-preserving, consistent with Eq, distinguishes structures
   
2. **Idiomatic Rust Approach** ✅
   - **Decision**: Use `std::hash::Hash` with conditional bounds
   - **Rationale**: Standard, familiar, works with HashMap/HashSet, type-safe
   - **Implementation**:
     ```rust
     impl<V: Hash> Hash for Pattern<V> {
         fn hash<H: Hasher>(&self, state: &mut H) {
             self.value.hash(state);
             self.elements.hash(state);
         }
     }
     ```

3. **Type Compatibility** ✅
   - Pattern<String>: ✅ Hashable
   - Pattern<Symbol>: ✅ Hashable (add Hash to Symbol)
   - Pattern<Subject>: ❌ Not hashable (contains f64, correct behavior)
   - Type system enforces correctness at compile time

4. **Testing Strategy** ✅
   - Property tests for hash/eq consistency
   - Unit tests for HashMap/HashSet usage
   - Structure distinction tests
   - Equivalence tests with gram-hs

## Phase 1: Design & Contracts

**Prerequisites**: ✅ Research complete, approach confirmed

### Implementation Design ✅

**Hash Implementation** (crates/pattern-core/src/pattern.rs):
```rust
use std::hash::{Hash, Hasher};

impl<V: Hash> Hash for Pattern<V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.elements.hash(state);  // Vec's Hash handles recursion
    }
}
```

**Symbol Enhancement** (crates/pattern-core/src/subject.rs):
```rust
#[derive(Clone, PartialEq, Eq, Hash)]  // Add Hash
pub struct Symbol(pub String);
```

**Documentation Requirements**:
- Explain hash/eq consistency requirement
- Show HashMap and HashSet examples
- Note that Pattern<Subject> is not hashable
- Reference gram-hs Hashable instance

**Type Constraints**:
- Only Pattern<V> where V: Hash can be hashed
- Compilation error for Pattern<Subject>::hash() (helpful error message)
- Works with all standard hashable types (String, integers, Symbol, etc.)

## Phase 2: Implementation Tasks

See [tasks.md](tasks.md) for detailed task breakdown.

**Summary**:
1. **Core Implementation** (1 hour)
   - Implement Hash for Pattern<V>
   - Add Hash to Symbol
   - Add documentation

2. **Unit Tests** (1 hour)
   - HashMap usage tests
   - HashSet deduplication tests
   - Structure distinction tests

3. **Property Tests** (1 hour)
   - Hash/eq consistency verification
   - Test with various pattern structures
   - 10,000+ random patterns

4. **Integration Tests** (30 min)
   - Collection operations
   - Caching patterns
   - Set operations

5. **Verification** (30 min)
   - Equivalence with gram-hs
   - Documentation review
   - WASM compilation

**Total Estimated Time**: 4 hours

## Success Verification

### Test Coverage

- **Unit Tests**: 12-15 tests covering:
  - Hash trait implementation
  - HashMap usage (insert, lookup, update)
  - HashSet usage (deduplication, membership)
  - Structure distinguishes hashes
  - Symbol hashing

- **Property Tests**: Verify hash/eq consistency:
  - For randomly generated pattern pairs (p1, p2)
  - Verify: if `p1 == p2` then `hash(p1) == hash(p2)`
  - Test with 10,000+ random cases
  - Cover various pattern structures (atomic, nested, deep, wide)

- **Integration Tests**: Verify usage with:
  - HashMap as cache (pattern → result lookups)
  - HashSet for deduplication
  - Set operations (intersection, difference, union)
  - Pattern indexing use cases

### Performance Targets

- ✅ Hash computation is O(n) in pattern size
- ✅ HashMap lookup remains O(1) average case
- ✅ HashSet deduplication is O(n) for n patterns
- ✅ Property tests complete in reasonable time (<5 seconds for 30,000+ cases)

### Behavioral Equivalence

- ✅ 100% of ported test cases from gram-hs pass
- ✅ Hash behavior matches gram-hs Hashable instance
- ✅ Structure-preserving hashing verified

### Integration Quality

- ✅ Works with HashMap and HashSet naturally
- ✅ No breaking changes to existing Pattern API
- ✅ Documentation clearly explains usage and limitations
- ✅ Type errors for non-hashable patterns are clear

## Risk Assessment

### Low Risk

- **Well-defined trait**: `Hash` is standard and simple
- **Clear semantics**: Hash/eq consistency is well-understood
- **No dependencies**: Uses only standard library
- **Simple implementation**: ~10 lines of code
- **Type safety**: Compiler enforces correct usage

### Medium Risk

- **Hash/Eq consistency**: Must ensure equal patterns hash equally
  - Mitigation: Leverage Vec's Hash impl, use property tests
- **Deep nesting performance**: Could be slow for very deep patterns
  - Mitigation: O(n) is acceptable, hashing typically fast
  
### Mitigation Strategies

1. **Consistency verification**: Property-based testing with 10,000+ patterns
2. **Performance**: Hash only when needed, results cached in HashMap/HashSet
3. **Type safety**: Conditional bounds prevent incorrect usage
4. **Documentation**: Clear examples and limitations documented

## Dependencies

### Upstream (must exist before implementation)

- ✅ Pattern<V> type (feature 004)
- ✅ Eq/PartialEq implementations (feature 004)
- ✅ Property testing infrastructure with proptest (feature 003)
- ✅ Ord/PartialOrd implementations (feature 012)

### Downstream (will use this feature)

- Pattern storage systems (future)
- Pattern indexing (future)
- Code generation and caching (future)
- WASM bindings with JavaScript Map/Set (future)

## References

- **Haskell Source**: `../pattern-hs/libs/pattern/src/Pattern/Core.hs` (lines 477-535)
- **Haskell Spec**: `../pattern-hs/specs/012-hashable-instance/` (development notes)
- **Rust Hash**: https://doc.rust-lang.org/std/hash/trait.Hash.html
- **Porting Guide**: `../../../docs/porting-guide.md`
- **Analysis**: [ANALYSIS.md](ANALYSIS.md) (comprehensive evaluation)

## Next Steps

1. ✅ **Phase 0 Complete**: Analysis and research documented
2. ✅ **Phase 1 Complete**: Design and plan created
3. **Phase 2 Pending**: Generate detailed task breakdown (tasks.md)
4. **Implementation**: Follow task list to implement Hash trait and tests
5. **Verification**: Ensure all success criteria met before feature completion

---

**Plan Status**: ✅ Complete and ready for task generation

**Implementation Ready**: All design decisions made, all documentation complete, implementation can proceed immediately.
