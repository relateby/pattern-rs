# Implementation Plan: Pattern Construction & Access

**Branch**: `005-basic-pattern-type` | **Date**: 2025-01-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/005-basic-pattern-type/spec.md`

## Summary

Port pattern construction functions, accessors, and inspection utilities from gram-hs to Rust, implementing convenient APIs for creating, accessing, and analyzing pattern instances. The implementation must maintain behavioral equivalence with the gram-hs reference implementation while using idiomatic Rust patterns. This includes construction functions for atomic and nested patterns, accessor methods for value and elements, and inspection utilities for structural analysis.

**Note**: This feature adds construction, access, and inspection functions to the existing `Pattern<V>` type defined in feature 004. The Pattern type structure itself is not modified.

## Technical Context

**Language/Version**: Rust 1.70.0+ (workspace MSRV), Edition 2021  
**Primary Dependencies**: 
- Standard library only (no new dependencies required)
- Existing Pattern<V> type from feature 004

**Storage**: N/A (in-memory data structures only)  
**Testing**: 
- `cargo test` - Standard Rust test framework
- `proptest` (workspace) - Property-based testing infrastructure (already configured)
- `insta` (workspace) - Snapshot testing infrastructure (already configured)
- Test utilities in `crates/pattern-core/src/test_utils/` for equivalence checking

**Target Platform**: 
- Native Rust targets (x86_64, ARM, etc.)
- WebAssembly (`wasm32-unknown-unknown`)

**Project Type**: Library crate (part of multi-crate workspace)  
**Performance Goals**: 
- Construction functions should be efficient (no unnecessary allocations)
- Accessors should be O(1) for value, O(1) for elements reference
- Inspection utilities must handle at least 100 nesting levels without stack overflow
- Inspection utilities must handle at least 10,000 elements efficiently

**Constraints**: 
- MUST maintain behavioral equivalence with gram-hs reference implementation
- MUST compile for `wasm32-unknown-unknown` target
- MUST use idiomatic Rust patterns (methods vs functions)
- MUST work generically with any value type `V` that Pattern supports
- MUST not modify the Pattern type structure (only add functions/methods)

**Scale/Scope**: 
- Construction functions in `pattern-core` crate
- Accessor methods/functions in `pattern-core` crate
- Inspection utilities in `pattern-core` crate
- Test case porting from gram-hs for behavioral verification

**Verified from gram-hs Implementation**:
- ✅ Construction functions: `point`, `pattern`, `fromList` (verified from `Pattern/Core.hs`)
- ✅ Accessors: `value` and `elements` are record field accessors in Haskell (verified)
- ✅ Inspection utilities: `length`, `size`, `depth` (verified from `Pattern/Core.hs`)
- ✅ No input validation needed (Pattern structure is always valid)
- ✅ Atomic patterns have depth 0 (corrected from previous inconsistency in gram-hs)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity ✅
- **Status**: PASS
- **Verification**: Feature spec references the actual Haskell implementation in `../gram-hs/libs/` as the authoritative source of truth
- **Plan**: Port construction, access, and inspection functions from Haskell to Rust, maintaining behavioral equivalence
- **Reference Path**: `../gram-hs/libs/pattern/src/Pattern.hs` (primary source) and `../gram-hs/specs/002-basic-pattern-type/` (context only)

### II. Correctness & Compatibility (NON-NEGOTIABLE) ✅
- **Status**: PASS
- **Verification**: Spec requires behavioral equivalence (SC-004, SC-005: 95% test case match)
- **Plan**: Port test cases from gram-hs and verify construction, access, and inspection behavior matches

### III. Rust Native Idioms ✅
- **Status**: PASS
- **Verification**: Plan uses Rust methods/functions, standard library types
- **Plan**: Implement as methods on Pattern type or associated functions, using idiomatic Rust patterns

### IV. Multi-Target Library Design ✅
- **Status**: PASS
- **Verification**: Spec requires WASM compilation (inherited from feature 004)
- **Plan**: Ensure no platform-specific code; use standard library only

### V. External Language Bindings & Examples ✅
- **Status**: DEFERRED
- **Verification**: WASM bindings are out of scope for this feature
- **Plan**: Functions must compile for WASM but bindings deferred to later features

**Note**: When porting features from gram-hs, **always use the Haskell implementation in `../gram-hs/libs/` as the authoritative source of truth**. Design documents in `../gram-hs/specs/` are useful for context but may contain outdated information or design mistakes that were corrected in the actual implementation. See [PORTING_GUIDE.md](../../../PORTING_GUIDE.md) for detailed porting instructions.

## Project Structure

### Documentation (this feature)

```text
specs/005-basic-pattern-type/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   └── type-signatures.md
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/pattern-core/
├── Cargo.toml
└── src/
    ├── lib.rs           # Public API exports (add construction, access, inspection functions)
    └── pattern.rs       # Pattern<V> type definition (add impl blocks for construction, access, inspection)
```

**Structure Decision**: Adding construction, access, and inspection functions to the existing `pattern.rs` module. Functions will be implemented as methods on `Pattern<V>` or as associated functions in `impl Pattern<V>` blocks. No new modules needed.

## Complexity Tracking

> **No violations detected** - All constitution principles are satisfied.

## Phase 0: Research Complete ✅

**Status**: Complete  
**Output**: `research.md`

All research tasks completed:
- ✅ Construction function patterns from gram-hs (new, atomic)
- ✅ Accessor method patterns (value, elements as methods)
- ✅ Inspection utility patterns (is_atomic, element_count, depth)
- ✅ Rust implementation patterns (methods for access/inspection, associated functions for construction)
- ✅ Behavioral equivalence testing strategy

## Phase 1: Design & Contracts Complete ✅

**Status**: Complete  
**Outputs**: 
- `data-model.md` - Function/method definitions and relationships
- `contracts/type-signatures.md` - API contracts
- `quickstart.md` - Usage examples and quickstart guide

### Data Model
- Construction functions defined (`point`, `pattern`, `from_list`) matching gram-hs
- Accessor methods defined (`value`, `elements`) matching gram-hs field accessors
- Inspection utilities defined (`length`, `size`, `depth`, `is_atomic`) matching gram-hs
- Relationships and validation rules documented
- Behavioral equivalence with gram-hs verified

### Contracts
- Type signatures for all construction functions (matching gram-hs `point`, `pattern`, `fromList`)
- Type signatures for all accessor methods (matching gram-hs field accessors)
- Type signatures for all inspection utilities (matching gram-hs `length`, `size`, `depth`)
- Usage examples provided with correct depth semantics (atomic patterns have depth 0)

### Quickstart
- Basic usage examples using verified function names
- Advanced patterns with correct depth calculations
- Testing examples demonstrating behavioral equivalence
- Performance considerations and WASM compatibility

## Phase 2: Tasks (Next Step)

**Status**: Pending  
**Next Command**: `/speckit.tasks` to break the plan into implementation tasks

The plan will be ready for task breakdown after Phase 0 and Phase 1 are complete.

