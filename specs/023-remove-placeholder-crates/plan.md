# Implementation Plan: Remove Unused Placeholder Crates

**Branch**: `023-remove-placeholder-crates` | **Date**: 2026-01-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/023-remove-placeholder-crates/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Remove three unused placeholder crates (pattern-store, pattern-ops, pattern-wasm) from the workspace to improve codebase clarity and reduce maintenance burden. This is a straightforward cleanup task that involves deleting crate directories, verifying no dependencies exist, and updating documentation references.

## Technical Context

**Language/Version**: Rust 1.70.0+ (edition 2021)  
**Primary Dependencies**: Cargo workspace management  
**Storage**: N/A (no data persistence involved)  
**Testing**: `cargo test --workspace` for verification  
**Target Platform**: All Rust targets (native, WASM)  
**Project Type**: Rust workspace cleanup  
**Performance Goals**: N/A (removal operation, not runtime performance)  
**Constraints**: Must not break existing builds or tests  
**Scale/Scope**: Remove 3 placeholder crates from workspace

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity
✅ **PASS** - This is a cleanup task, not a feature port. No reference implementation considerations apply.

### II. Correctness & Compatibility (NON-NEGOTIABLE)
✅ **PASS** - Removal must maintain workspace buildability and test compatibility. All existing functionality must continue to work.

### III. Rust Native Idioms
✅ **PASS** - Standard Cargo workspace management practices apply.

### IV. Multi-Target Library Design
✅ **PASS** - Removal does not affect multi-target compilation. No platform-specific code involved.

### V. External Language Bindings & Examples
✅ **PASS** - No impact on external bindings. Only unused placeholder crates are removed.

**Note**: When porting features from gram-hs, reference the local implementation at `../gram-hs` and corresponding feature specifications in `../gram-hs/specs/`. See [PORTING_GUIDE.md](../../../PORTING_GUIDE.md) for detailed porting instructions.

## Project Structure

### Documentation (this feature)

```text
specs/023-remove-placeholder-crates/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/
├── gram-codec/          # Active crate (kept)
├── pattern-core/        # Active crate (kept)
├── pattern-ops/         # Placeholder crate (REMOVE)
├── pattern-store/       # Placeholder crate (REMOVE)
└── pattern-wasm/        # Placeholder crate (REMOVE)
```

**Structure Decision**: The workspace uses `members = ["crates/*", "benches"]` in root Cargo.toml, which automatically includes all crates. After removal, only active crates (gram-codec, pattern-core) will remain. No changes needed to workspace configuration since it uses wildcard matching.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations - this is a straightforward cleanup task with no complexity concerns.
