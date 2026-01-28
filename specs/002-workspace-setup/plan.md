# Implementation Plan: Multi-Crate Workspace Setup

**Branch**: `002-workspace-setup` | **Date**: 2025-01-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-workspace-setup/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Convert the gram-rs project from a single crate structure to a Cargo workspace with multiple crates organized by functional domain (pattern-core, pattern-ops, gram-codec, pattern-store placeholder, pattern-wasm placeholder). Establish CI/CD pipeline for automated validation and test synchronization infrastructure for maintaining parity with gram-hs reference implementation. The workspace structure enables modular development, independent crate testing, and clear separation of concerns while maintaining compatibility with existing development workflows.

## Technical Context

**Language/Version**: Rust (MSRV: 1.70.0, edition: 2021)  
**Primary Dependencies**: 
- Cargo workspace (built-in)
- CI/CD: GitHub Actions (assumed from spec assumptions)
- Test synchronization: JSON-based test case format (to be designed in research phase)
- Future workspace dependencies: serde, serde_json, thiserror (as outlined in project plan)  
**Storage**: N/A (workspace structure only, no data storage)  
**Testing**: `cargo test` (built-in Rust test framework), workspace-level test execution, CI/CD test automation  
**Target Platform**: Native Rust (default host) and WebAssembly (`wasm32-unknown-unknown`) - workspace must support both  
**Project Type**: Cargo workspace (multi-crate library structure)  
**Performance Goals**: 
- Workspace build time: <2 minutes for full workspace (SC-001)
- Individual crate build time: <30 seconds (SC-002)
- CI/CD pipeline completion: <10 minutes (SC-004)  
**Constraints**: 
- Must maintain backward compatibility with existing code from feature 001
- All crates must compile for both native and WASM targets
- Workspace dependencies must be carefully managed to avoid conflicts
- Placeholder crates must compile without errors
- Development tooling (rustfmt, clippy) must work seamlessly with workspace  
**Scale/Scope**: 
- 5 crates initially (pattern-core, pattern-ops, gram-codec, pattern-store placeholder, pattern-wasm placeholder)
- Workspace structure designed to accommodate future growth
- CI/CD pipeline for automated validation
- Test synchronization infrastructure for gram-hs parity

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity
✅ **PASS**: Workspace setup is infrastructure-only and doesn't port functionality from gram-hs. However, workspace structure enables future faithful porting. Test synchronization infrastructure will support maintaining fidelity with gram-hs reference implementation.

### II. Correctness & Compatibility (NON-NEGOTIABLE)
✅ **PASS**: Workspace structure prioritizes correctness through proper dependency management and build configuration. All crates must compile successfully, ensuring compatibility across the workspace.

### III. Rust Native Idioms
✅ **PASS**: Workspace structure follows Rust/Cargo best practices for multi-crate projects. Uses standard Cargo workspace configuration, resolver version 2, and Rust naming conventions.

### IV. Multi-Target Library Design
✅ **PASS**: Workspace configuration must support both native Rust and WASM targets. All crates must compile for `wasm32-unknown-unknown`. Conditional compilation features will work across crate boundaries (FR-022).

### V. External Language Bindings & Examples
✅ **PASS**: Workspace includes `pattern-wasm` crate placeholder for future WASM bindings. Existing examples structure from feature 001 will be preserved and can be enhanced as workspace evolves.

**Gate Status**: ✅ **ALL GATES PASS** - Workspace setup aligns with all constitutional principles and enables future compliance.

**Note**: When porting features from gram-hs, reference the local implementation at `../gram-hs` and corresponding feature specifications in `../gram-hs/specs/`. See [porting guide](../../../docs/porting-guide.md) for detailed porting instructions.

## Project Structure

### Documentation (this feature)

```text
specs/002-workspace-setup/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
Cargo.toml              # Root workspace manifest
├── [workspace]         # Workspace configuration
├── [workspace.package] # Shared package metadata
└── [workspace.dependencies] # Shared dependencies

crates/
├── pattern-core/       # Core pattern data structures
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── pattern-ops/        # Pattern operations and algorithms
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── gram-codec/         # Gram notation serialization/deserialization
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── pattern-store/      # Placeholder for optimized storage (future)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs      # Minimal placeholder code
└── pattern-wasm/       # Placeholder for WASM bindings (future)
    ├── Cargo.toml
    └── src/
        └── lib.rs      # Minimal placeholder code

tests/                  # Workspace-level tests (if needed)
├── integration/
└── unit/

examples/              # Examples (preserved from feature 001)
├── wasm-js/
└── README.md

.github/
└── workflows/
    └── ci.yml         # CI/CD pipeline configuration

scripts/               # Test synchronization utilities (if needed)
└── sync-tests/        # Test extraction and comparison tools
```

**Structure Decision**: Cargo workspace structure with `crates/` directory containing all member crates. Root `Cargo.toml` defines workspace members, shared dependencies, and workspace-level configuration. Each crate has its own `Cargo.toml` with crate-specific metadata and dependencies. CI/CD pipeline configuration in `.github/workflows/`. Test synchronization infrastructure can be organized in `scripts/` or as a separate crate if it grows complex.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations - all gates pass.
