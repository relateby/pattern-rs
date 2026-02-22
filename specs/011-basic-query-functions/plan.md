# Implementation Plan: Pattern Query Operations

**Branch**: `011-basic-query-functions` | **Date**: 2025-01-04 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/011-basic-query-functions/spec.md`

**Note**: This plan focuses on porting missing predicate/search functions (`any_value`, `all_values`, `filter`) from the Haskell reference implementation and adding comprehensive test coverage for existing query operations.

## Summary

Port three missing predicate/search functions from the gram-hs reference implementation to enable pattern querying based on value and pattern predicates:

1. **`any_value`** - Check if at least one value in a pattern satisfies a predicate (with short-circuit evaluation)
2. **`all_values`** - Check if all values in a pattern satisfy a predicate (with short-circuit evaluation)
3. **`filter`** - Extract subpatterns that satisfy a pattern predicate

Additionally, add comprehensive test coverage for existing query operations (`length`, `size`, `depth`, `values`) to ensure behavioral equivalence with the Haskell reference implementation.

**Technical Approach**: Implement as methods on the `Pattern<V>` type in `crates/pattern-core/src/pattern.rs`, leveraging the existing fold/traverse infrastructure from feature 009. Use Rust closures for predicates, enabling idiomatic Rust usage while maintaining semantic equivalence with Haskell's higher-order functions.

## Technical Context

**Language/Version**: Rust 1.75+ (matches existing codebase)  
**Primary Dependencies**: None (core library functionality, no external dependencies needed)  
**Storage**: N/A (pure computation)  
**Testing**: `cargo test` (unit tests), `proptest` (property-based tests), cross-implementation equivalence tests with gram-hs CLI  
**Target Platform**: Multi-target (native Rust, WASM)  
**Project Type**: Library crate (`pattern-core`)  
**Performance Goals**: 
- `any_value`/`all_values`: <100ms for patterns with 10,000 nodes
- `filter`: <200ms for patterns with 10,000 nodes
- Short-circuit evaluation verified through performance tests  
**Constraints**: 
- Must work without stack overflow for 100+ nesting levels
- Must be WASM-compatible (no platform-specific code)
- Must maintain O(n) time complexity for predicate functions  
**Scale/Scope**: 3 new public methods + comprehensive test suite (est. 50-75 tests)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### ✅ I. Reference Implementation Fidelity

**Status**: PASS

- Porting from `../pattern-hs/libs/pattern/src/Pattern/Core.hs` lines 945-1010
- Reference spec: `../pattern-hs/specs/008-basic-query-functions/spec.md`
- Functions to port: `anyValue`, `allValues`, `filter` (exact Haskell implementations identified)
- Will verify behavioral equivalence through cross-implementation tests

**Action**: Review Haskell source code for exact semantics before implementation

### ✅ II. Correctness & Compatibility

**Status**: PASS

- No breaking changes to existing API (adding new methods only)
- New methods follow same patterns as existing query operations
- Will maintain behavioral equivalence with reference implementation
- Edge cases explicitly documented (empty patterns, short-circuit behavior)

**Action**: Comprehensive test suite must verify correctness before merge

### ✅ III. Rust Native Idioms

**Status**: PASS

- Will use Rust closures/`Fn` traits for predicates (idiomatic equivalent to Haskell functions)
- Method naming follows Rust conventions (`any_value` vs Haskell's `anyValue`)
- Return types use native Rust types (`bool`, `Vec<&Pattern<V>>`)
- Leverages existing fold infrastructure (zero-cost abstractions)

**Action**: Code review must verify idiomatic Rust usage

### ✅ IV. Multi-Target Library Design

**Status**: PASS

- Pure computation, no platform-specific code
- No I/O, no file system access
- Uses only core Rust features compatible with WASM
- Existing pattern-core compiles to WASM (verified in feature 004)

**Action**: Verify WASM compilation in CI

### ✅ V. External Language Bindings & Examples

**Status**: PASS (no action required for this feature)

- New methods follow same patterns as existing exported methods
- No changes to WASM bindings required (pattern-wasm crate handles export)
- Examples can be updated in future feature if needed

**Action**: None required for this feature

**Note**: When porting features from gram-hs, reference the local implementation at `../pattern-hs` and corresponding feature specifications in `../pattern-hs/specs/`. See [porting guide](../../../docs/porting-guide.md) for detailed porting instructions.

## Project Structure

### Documentation (this feature)

```text
specs/011-basic-query-functions/
├── spec.md              # Feature specification (complete)
├── plan.md              # This file
├── research.md          # Phase 0: Implementation research (to be created)
├── data-model.md        # Phase 1: Data structures (to be created)
├── quickstart.md        # Phase 1: Quick reference (to be created)
├── contracts/           # Phase 1: API contracts (to be created)
│   └── type-signatures.md
├── checklists/          # Specification validation
│   └── requirements.md
└── tasks.md             # Phase 2: Implementation tasks (created by /speckit.tasks)
```

### Source Code (repository root)

```text
crates/pattern-core/
├── src/
│   ├── lib.rs                    # Re-exports (add new functions)
│   ├── pattern.rs                # Main implementation (add 3 new methods)
│   └── test_utils/               # Shared test utilities (may extend)
│       ├── mod.rs
│       ├── generators.rs
│       ├── helpers.rs
│       └── reference_data.rs
├── tests/
│   ├── query_any_value.rs        # NEW: Tests for any_value
│   ├── query_all_values.rs       # NEW: Tests for all_values
│   ├── query_filter.rs            # NEW: Tests for filter
│   ├── query_existing.rs          # NEW: Tests for length/size/depth/values
│   ├── property/
│   │   └── query_operations.rs   # NEW: Property tests for query operations
│   └── equivalence/
│       └── query_functions.rs    # NEW: Cross-implementation tests
└── benches/
    └── query_benchmarks.rs        # NEW: Performance benchmarks
```

**Structure Decision**: Single library crate (`pattern-core`). New functionality added as methods on existing `Pattern<V>` type. Tests organized by operation (one test file per function) with additional property tests and equivalence tests.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations - all gates pass without justification needed.

---

## Post-Design Constitution Check

*Re-evaluated after Phase 1 design completion*

### ✅ I. Reference Implementation Fidelity - VERIFIED

**Status**: PASS (verified through research)

Research confirms exact behavioral equivalence:
- `any_value` implementation matches Haskell's `foldr (\v acc -> p v || acc) False`
- `all_values` implementation matches Haskell's `foldr (\v acc -> p v && acc) True`
- `filter` implementation matches Haskell's recursive pattern with pre-order traversal
- Test cases ported directly from gram-hs test suite (81 test cases identified)
- Property tests maintain complementarity relationships from Haskell

**Verification Method**: 
- Direct source code comparison (`../pattern-hs/libs/pattern/src/Pattern/Core.hs` lines 945-1028)
- Test suite analysis (`../pattern-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs`)
- Documented in `research.md`

### ✅ II. Correctness & Compatibility - VERIFIED

**Status**: PASS (design complete)

- API contracts documented in `contracts/type-signatures.md`
- Behavioral contracts specify exact semantics (traversal order, short-circuit, edge cases)
- Comprehensive test strategy defined (unit, property, equivalence, performance)
- No breaking changes to existing APIs
- Additive only (3 new methods)

### ✅ III. Rust Native Idioms - VERIFIED

**Status**: PASS (design follows Rust conventions)

Idiomatic choices confirmed:
- `Fn` trait bounds for predicates (not function pointers)
- `snake_case` naming (`any_value` not `anyValue`)
- References in return types (`Vec<&Pattern<V>>` not owned)
- Leverages existing infrastructure (`fold()` method)
- Zero-cost abstractions (monomorphization, no heap allocation for boolean operations)

**Documented in**: `data-model.md`, `contracts/type-signatures.md`

### ✅ IV. Multi-Target Library Design - VERIFIED

**Status**: PASS (no platform-specific code)

- Pure computation only
- No I/O, filesystem, or platform APIs
- Leverages existing WASM-compatible pattern infrastructure
- No new dependencies (core library only)
- Will compile to WASM without modifications

### ✅ V. External Language Bindings & Examples - VERIFIED

**Status**: PASS (no action required)

- New methods follow same patterns as existing exported methods
- Quick reference guide created (`quickstart.md`)
- WASM bindings handled by existing `pattern-wasm` crate
- Examples provided in quickstart guide

**Final Assessment**: All constitutional principles satisfied. Design maintains reference implementation fidelity while following Rust idioms. Ready for Phase 2 (Task Breakdown via `/speckit.tasks`).
