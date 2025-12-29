# Implementation Plan: Core Pattern Data Structure

**Branch**: `004-pattern-data-structure` | **Date**: 2025-01-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/004-pattern-data-structure/spec.md`

## Summary

Port the core Pattern data structure from gram-hs to Rust, implementing `Pattern<V>` as a generic recursive nested structure (s-expression-like) that works with any value type `V`. The implementation must maintain behavioral equivalence with the gram-hs reference implementation while using idiomatic Rust patterns. This includes implementing Debug and Display traits, supporting WASM compilation, and enabling test case porting from gram-hs.

**Note**: This feature defines both `Pattern<V>` (generic recursive nested structure) and `Subject` (self-descriptive value type with identity, labels, and properties). `Pattern<Subject>` is a common use case.

## Technical Context

**Language/Version**: Rust 1.70.0+ (workspace MSRV), Edition 2021  
**Primary Dependencies**: 
- `serde` (workspace) - Serialization support
- `serde_json` (workspace) - JSON serialization for test data
- Standard library traits: `Debug`, `Display`, `Clone`, `PartialEq`, `Eq`

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
- Support patterns with at least 100 nesting levels without stack overflow
- Support patterns with at least 10,000 elements efficiently
- WASM compilation must succeed (size optimization deferred to later features)

**Constraints**: 
- MUST maintain behavioral equivalence with gram-hs reference implementation
- MUST compile for `wasm32-unknown-unknown` target
- MUST use idiomatic Rust patterns while maintaining functional equivalence
- MUST support Debug and Display traits for inspection

**Scale/Scope**: 
- Core data structure implementation in `pattern-core` crate
- Pattern<V> type as generic recursive nested structure (s-expression-like)
- Subject type as self-descriptive value type (identity, labels, properties)
- Traits: Debug, Display, Clone, PartialEq, Eq for both Pattern<V> and Subject
- Test case porting from gram-hs for behavioral verification

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity ✅
- **Status**: PASS
- **Verification**: Feature spec references the actual Haskell implementation in `../gram-hs/libs/` as the authoritative source of truth
- **Plan**: Port `Pattern v` from Haskell to Rust `Pattern<V>`, maintaining structural equivalence
- **Reference Path**: `../gram-hs/libs/` (primary source) and `../gram-hs/specs/001-pattern-data-structure/` (context only)

### II. Correctness & Compatibility (NON-NEGOTIABLE) ✅
- **Status**: PASS
- **Verification**: Spec requires behavioral equivalence (SC-005: 95% test case match)
- **Plan**: Port test cases from gram-hs and verify structural/equality behavior matches

### III. Rust Native Idioms ✅
- **Status**: PASS
- **Verification**: Plan uses Rust struct with generics, standard traits (Debug, Display, Clone, PartialEq, Eq)
- **Plan**: Implement as `pub struct Pattern<V>` with idiomatic Rust trait implementations

### IV. Multi-Target Library Design ✅
- **Status**: PASS
- **Verification**: Spec requires WASM compilation (FR-008, SC-004)
- **Plan**: Ensure no platform-specific code; use standard library and workspace dependencies only

### V. External Language Bindings & Examples ✅
- **Status**: DEFERRED
- **Verification**: WASM bindings are out of scope for this feature (spec notes "not usable from JavaScript yet")
- **Plan**: WASM compilation verification only; bindings deferred to later features

**Note**: When porting features from gram-hs, **always use the Haskell implementation in `../gram-hs/libs/` as the authoritative source of truth**. Design documents in `../gram-hs/specs/` are useful for context but may contain outdated information or design mistakes that were corrected in the actual implementation. See [PORTING_GUIDE.md](../../../PORTING_GUIDE.md) for detailed porting instructions.

## Project Structure

### Documentation (this feature)

```text
specs/004-pattern-data-structure/
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
    ├── lib.rs           # Public API exports
    ├── pattern.rs       # Pattern<V> type definition
    └── test_utils/       # Test utilities (already exists)
        ├── mod.rs
        ├── equivalence.rs
        ├── generators.rs
        └── helpers.rs

tests/
├── equivalence/         # Behavioral equivalence tests with gram-hs
│   └── pattern_structure.rs
└── unit/                # Unit tests
    └── pattern_core.rs
```

**Structure Decision**: Using existing `pattern-core` crate structure. Adding `pattern.rs` module for the Pattern<V> type definition and `subject.rs` module for the Subject type definition. Test utilities already exist in `test_utils/` directory.

## Complexity Tracking

> **No violations detected** - All constitution principles are satisfied.

## Phase 0: Research Complete ✅

**Status**: Complete  
**Output**: `research.md`

All research tasks completed:
- ✅ Pattern type structure from gram-hs
- ✅ Rust trait implementation strategy
- ✅ WASM compilation compatibility
- ✅ Behavioral equivalence testing strategy
- ✅ S-expression vs tree terminology clarification

## Phase 1: Design & Contracts Complete ✅

**Status**: Complete  
**Outputs**: 
- `data-model.md` - Entity definitions and relationships
- `contracts/type-signatures.md` - API contracts
- `quickstart.md` - Usage examples and quickstart guide

### Data Model
- Pattern<V> structure defined
- Subject type structure defined (identity, labels, properties)
- Value types section clarifies Pattern<V> works with any type V, including Subject
- Relationships and validation rules documented

### Contracts
- Type signatures for Pattern<V> and Subject
- Trait implementations specified for both types
- Usage examples provided for both Pattern<V> and Pattern<Subject>

### Quickstart
- Basic usage examples
- Advanced patterns
- WASM usage
- Testing examples

## Phase 2: Tasks (Next Step)

**Status**: Pending  
**Next Command**: `/speckit.tasks` to break the plan into implementation tasks

The plan is ready for task breakdown. All design artifacts are complete.
