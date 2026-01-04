# Implementation Plan: Pattern Structure Validation

**Branch**: `006-pattern-structure-review` | **Date**: 2025-01-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/006-pattern-structure-review/spec.md`

## Summary

Port pattern validation functions and structure analysis utilities from gram-hs to Rust, implementing configurable validation rules and detailed structural analysis capabilities. The implementation must maintain behavioral equivalence with the gram-hs reference implementation while using idiomatic Rust patterns. This includes validation functions that check structural constraints (nesting depth, element counts, etc.) and structure analysis utilities that provide detailed information about pattern characteristics.

**Note**: This feature adds validation and structure analysis functions to the existing `Pattern<V>` type defined in feature 004. The Pattern type structure itself is not modified. Validation focuses on structural properties rather than value-specific validation.

## Technical Context

**Language/Version**: Rust 1.70.0+ (workspace MSRV), Edition 2021  
**Primary Dependencies**: 
- Standard library only (no new dependencies required)
- Existing Pattern<V> type from feature 004
- Existing test utilities from feature 003 (ValidationRules, ValidationError already defined as placeholders)

**Storage**: N/A (in-memory data structures only)  
**Testing**: 
- `cargo test` - Standard Rust test framework
- `proptest` (workspace) - Property-based testing infrastructure (already configured)
- `insta` (workspace) - Snapshot testing infrastructure (already configured)
- Test utilities in `crates/pattern-core/src/test_utils/` for equivalence checking
- Test cases from gram-hs for behavioral verification

**Target Platform**: 
- Native Rust targets (x86_64, ARM, etc.)
- WebAssembly (`wasm32-unknown-unknown`)

**Project Type**: Library crate (part of multi-crate workspace)  
**Performance Goals**: 
- Validation functions must handle at least 100 nesting levels without stack overflow
- Validation and analysis functions must handle at least 10,000 elements efficiently
- Validation should be O(n) where n is the number of nodes in the pattern
- Analysis should be O(n) where n is the number of nodes in the pattern

**Constraints**: 
- MUST maintain behavioral equivalence with gram-hs reference implementation
- MUST compile for `wasm32-unknown-unknown` target
- MUST use idiomatic Rust patterns (Result types for validation errors)
- MUST work generically with any value type `V` that Pattern supports
- MUST provide detailed error information when validation fails
- MUST not modify the Pattern type structure (only add functions/methods)

**Scale/Scope**: 
- Validation functions in `pattern-core` crate
- Structure analysis utilities in `pattern-core` crate
- Configurable validation rules (ValidationRules type already defined as placeholder)
- Detailed error types (ValidationError already defined as placeholder)
- Test case porting from gram-hs for behavioral verification

**Verified from gram-hs Implementation**:
- ✅ Validation function pattern: `validate(&self, rules: &ValidationRules) -> Result<(), ValidationError>` (based on spec and existing placeholder, to be verified against gram-hs)
- ✅ Structure analysis pattern: `analyze_structure(&self) -> StructureAnalysis` (based on spec requirements, to be verified against gram-hs)
- ✅ Validation rules: `ValidationRules` with `max_depth`, `max_elements`, `required_fields` (based on existing placeholder, to be verified against gram-hs)
- **Note**: Final verification required by studying `../gram-hs/libs/pattern/src/Pattern.hs` during implementation

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity ✅
- **Status**: PASS
- **Verification**: Feature spec references the actual Haskell implementation in `../gram-hs/libs/` as the authoritative source of truth
- **Plan**: Port validation and structure analysis functions from Haskell to Rust, maintaining behavioral equivalence
- **Reference Path**: `../gram-hs/libs/pattern/src/Pattern.hs` (primary source) and `../gram-hs/specs/003-pattern-structure-review/` (context only)

### II. Correctness & Compatibility (NON-NEGOTIABLE) ✅
- **Status**: PASS
- **Verification**: Spec requires behavioral equivalence (SC-005, SC-006: 95% test case match)
- **Plan**: Port test cases from gram-hs and verify validation and analysis behavior matches

### III. Rust Native Idioms ✅
- **Status**: PASS
- **Verification**: Plan uses Rust Result types for error handling, standard library types
- **Plan**: Implement validation as functions returning `Result<(), ValidationError>`, using idiomatic Rust error handling

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
specs/006-pattern-structure-review/
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
├── src/
│   ├── lib.rs           # Re-exports pattern module
│   ├── pattern.rs       # Pattern type + validation/analysis functions (extend existing)
│   ├── subject.rs       # Subject type (from feature 004, unchanged)
│   └── test_utils/      # Test utilities (from feature 003)
│       ├── mod.rs
│       ├── helpers.rs   # ValidationRules, ValidationError (extend existing placeholders)
│       ├── equivalence.rs
│       └── generators.rs
└── tests/
    ├── equivalence/     # Equivalence tests with gram-hs
    │   └── pattern_structure.rs  # Validation and analysis equivalence tests
    ├── property/        # Property-based tests
    └── snapshot/        # Snapshot tests

tests/
└── equivalence/
    └── pattern_structure.rs  # Integration equivalence tests
```

**Structure Decision**: This feature extends the existing `pattern-core` crate by adding validation and structure analysis functions to the `Pattern` type. The functions will be implemented as methods or associated functions on `Pattern<V>`. The existing placeholder types (`ValidationRules`, `ValidationError`) in `test_utils/helpers.rs` will be fully implemented. No new modules or crates are needed.

## Constitution Check (Post-Design)

*Re-evaluated after Phase 1 design*

### I. Reference Implementation Fidelity ✅
- **Status**: PASS
- **Verification**: Research document identifies need to verify against `../gram-hs/libs/pattern/src/Pattern.hs`
- **Plan**: Implementation will verify exact function signatures and behavior against gram-hs during development

### II. Correctness & Compatibility (NON-NEGOTIABLE) ✅
- **Status**: PASS
- **Verification**: Design maintains behavioral equivalence requirements from spec
- **Plan**: Test cases will be ported from gram-hs to verify correctness

### III. Rust Native Idioms ✅
- **Status**: PASS
- **Verification**: Design uses `Result<(), ValidationError>` for error handling, idiomatic Rust patterns
- **Plan**: Implementation will follow Rust conventions while maintaining functional equivalence

### IV. Multi-Target Library Design ✅
- **Status**: PASS
- **Verification**: Design uses only standard library types, no platform-specific code
- **Plan**: Functions will compile for WASM target

### V. External Language Bindings & Examples ✅
- **Status**: DEFERRED
- **Verification**: WASM bindings are out of scope for this feature
- **Plan**: Functions must compile for WASM but bindings deferred to later features

## Complexity Tracking

No violations - all constitution checks pass. This feature extends existing functionality without adding complexity.

## Phase 1 Complete

**Generated Artifacts**:
- ✅ `research.md` - Research findings and implementation guidance
- ✅ `data-model.md` - Data model for validation and analysis entities
- ✅ `contracts/type-signatures.md` - API type signatures and contracts
- ✅ `quickstart.md` - Usage examples and quickstart guide
- ✅ Agent context updated for Cursor IDE

**Next Steps**:
- Phase 2: Use `/speckit.tasks` to break down implementation into tasks
- Implementation: Port validation and analysis functions from gram-hs
- Verification: Test behavioral equivalence with gram-hs reference implementation
