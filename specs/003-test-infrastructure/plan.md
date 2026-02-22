# Implementation Plan: Testing Infrastructure

**Branch**: `003-test-infrastructure` | **Date**: 2025-01-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-test-infrastructure/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Establish comprehensive testing infrastructure for pattern-rs including property-based testing (proptest), equivalence checking with gram-hs, snapshot testing (insta), test data extraction from gram-hs, benchmark suite (criterion), and test helper utilities. The infrastructure must integrate with the existing Cargo workspace structure, support both unit and integration tests, work across all workspace crates, and enable developers to verify behavioral equivalence with the gram-hs reference implementation. Testing infrastructure is foundational for ensuring correctness as features are ported from gram-hs.

## Technical Context

**Language/Version**: Rust (MSRV: 1.70.0, edition: 2021)  
**Primary Dependencies**: 
- Property-based testing: proptest (assumed from spec assumptions)
- Snapshot testing: insta (assumed from spec assumptions)
- Benchmarking: criterion (assumed from spec assumptions)
- Test helpers: Custom utilities (to be designed)
- Equivalence checking: Custom utilities (to be designed)
- Test extraction: Custom utilities building on feature 002 infrastructure
- Existing workspace dependencies: serde, serde_json, thiserror  
**Storage**: N/A (test infrastructure only, no persistent data storage)  
**Testing**: 
- Built-in: `cargo test` (Rust standard library)
- Property-based: proptest (to be added)
- Snapshot: insta (to be added)
- Benchmark: criterion (to be added)
- Custom: Equivalence checking utilities, test helpers, test extraction utilities  
**Target Platform**: Native Rust (default host) and WebAssembly (`wasm32-unknown-unknown`) - all testing infrastructure must support both targets where applicable  
**Project Type**: Cargo workspace (multi-crate library structure) - testing infrastructure must work across all crates  
**Performance Goals**: 
- Property-based tests: Generate 100+ test cases per property (SC-001)
- Property test failure reporting: <5 seconds for counterexamples (SC-002)
- Equivalence checking: <1 second per comparison (SC-003)
- Snapshot change detection: <2 seconds per snapshot (SC-004)
- Benchmark consistency: <10% variance across runs (SC-006)  
**Constraints**: 
- Must integrate with existing workspace structure from feature 002
- Must work with pattern types (to be defined in feature 004)
- Must support both unit and integration tests
- Must be usable across all workspace crates
- Must support WASM target compilation where applicable
- Test helpers must reduce boilerplate by 50%+ (SC-007)  
**Scale/Scope**: 
- Testing infrastructure for all workspace crates (pattern-core, pattern-ops, gram-codec, pattern-store, pattern-wasm)
- Support for property-based testing across pattern operations
- Equivalence checking for behavioral verification with gram-hs
- Snapshot testing for regression prevention
- Test extraction from gram-hs reference implementation
- Benchmark suite for performance tracking
- Test helper utilities for pattern comparison

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity
✅ **PASS**: Testing infrastructure supports behavioral equivalence checking with gram-hs reference implementation (User Story 2, FR-004, FR-005). Test extraction utilities enable reuse of gram-hs test cases (User Story 4, FR-009, FR-010). Infrastructure is designed to verify correctness against `../pattern-hs` reference implementation.

### II. Correctness & Compatibility (NON-NEGOTIABLE)
✅ **PASS**: Testing infrastructure prioritizes correctness verification through property-based testing, equivalence checking, and snapshot testing. All infrastructure is designed to ensure compatibility with gram-hs reference behavior. No breaking changes from reference implementation.

### III. Rust Native Idioms
✅ **PASS**: Testing infrastructure uses Rust-native testing frameworks (proptest, insta, criterion) and follows Rust testing conventions. Test helpers will use idiomatic Rust patterns. Integration with `cargo test` follows standard Rust workflows.

### IV. Multi-Target Library Design
⚠️ **CONDITIONAL PASS**: Testing infrastructure must support both native Rust and WASM targets. Property-based testing, snapshot testing, and benchmarks may have WASM compatibility considerations that need research. Test helpers and equivalence checking should work across targets. **Research needed**: WASM compatibility of proptest, insta, and criterion.

### V. External Language Bindings & Examples
✅ **PASS**: Testing infrastructure is internal to the Rust codebase and does not directly affect external language bindings. However, ensuring correctness through testing indirectly supports reliable external bindings.

**Note**: When porting features from gram-hs, reference the local implementation at `../pattern-hs` and corresponding feature specifications in `../pattern-hs/specs/`. See [porting guide](../../../docs/porting-guide.md) for detailed porting instructions.

## Project Structure

### Documentation (this feature)

```text
specs/003-test-infrastructure/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   ├── test-utilities-api.md    # API contracts for test utilities
│   └── benchmark-api.md         # API contracts for benchmark suite
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Workspace root (existing structure from feature 002)
Cargo.toml               # Workspace configuration with test dependencies
tests/                   # Workspace-level integration tests
├── common/              # Shared test data (from feature 002)
│   └── test_cases.json  # Test cases from gram-hs
├── integration/        # Integration tests
└── unit/               # Unit tests

# Individual crate structure (applies to all crates)
crates/
├── pattern-core/
│   ├── Cargo.toml       # Add test dependencies (proptest, insta, etc.)
│   ├── src/
│   │   └── lib.rs
│   └── tests/          # Crate-specific tests
│       ├── property/   # Property-based tests
│       ├── equivalence/ # Equivalence checking tests
│       ├── snapshot/   # Snapshot tests
│       └── helpers.rs  # Test helper utilities (shared)
│
├── pattern-ops/
│   ├── Cargo.toml       # Add test dependencies
│   ├── src/
│   └── tests/          # Similar structure as pattern-core
│
├── gram-codec/
│   ├── Cargo.toml       # Add test dependencies
│   ├── src/
│   └── tests/          # Similar structure
│
└── [other crates]/     # Similar structure

# Test utilities (shared across crates)
# Option 1: Shared test utilities crate (recommended)
crates/
└── test-utils/         # New crate for shared test utilities
    ├── Cargo.toml
    ├── src/
    │   ├── lib.rs
    │   ├── equivalence.rs  # Equivalence checking utilities
    │   ├── helpers.rs      # Pattern comparison helpers
    │   └── generators.rs   # Property-based test generators
    └── tests/

# Option 2: Test utilities in pattern-core (if simpler)
# Test helpers can be in pattern-core and re-exported for other crates

# Benchmark suite
benches/                # Criterion benchmarks (workspace-level)
├── pattern_operations.rs
└── codec_operations.rs

# Test extraction utilities
scripts/
└── sync-tests/         # Existing from feature 002
    ├── extract.rs      # Enhanced extraction (to be implemented)
    └── compare.rs      # Comparison utilities (to be implemented)
```

**Structure Decision**: Testing infrastructure will be distributed across the workspace:
- **Test dependencies**: Added to individual crate `Cargo.toml` files as dev-dependencies
- **Test utilities**: Shared test utilities will be provided as a new `test-utils` crate (Option 1) or as a module in `pattern-core` (Option 2) - decision to be made in research phase
- **Test files**: Organized by test type (property/, equivalence/, snapshot/) within each crate's `tests/` directory
- **Benchmarks**: Workspace-level `benches/` directory using Criterion
- **Test extraction**: Enhanced utilities in existing `scripts/sync-tests/` directory
- **Test data**: Shared test cases in `tests/common/` (from feature 002)

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
