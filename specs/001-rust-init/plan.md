# Implementation Plan: Rust Project Initialization

**Branch**: `001-rust-init` | **Date**: 2025-12-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-rust-init/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Initialize gram-rs as a Rust library project following Rust best practices, configured for multi-target compilation (native Rust and WASM) with development tooling, documentation structure, and example scaffolding for external language bindings. The project structure will support faithful porting of gram-hs functionality while adopting Rust-native idioms.

## Technical Context

**Language/Version**: Rust (MSRV: 1.70.0, edition: 2021)  
**Primary Dependencies**: None initially. Future dependencies must support WASM targets. Potential future: `wasm-bindgen` for JavaScript bindings, `pyo3` for Python bindings (evaluated in research phase)  
**Storage**: N/A (project initialization only)  
**Testing**: `cargo test` (built-in Rust test framework), `cargo clippy` for linting, `rustfmt` for formatting  
**Target Platform**: Native Rust (default host: x86_64, ARM, etc.) and WebAssembly (`wasm32-unknown-unknown`)  
**Project Type**: Library crate (may expand to workspace if examples require separate crates)  
**Performance Goals**: N/A for initialization phase (performance targets will be defined when porting functionality)  
**Constraints**: 
- All public APIs must be WASM-compatible (no blocking I/O, no file system access unless feature-flagged)
- Must support conditional compilation for target-specific code
- Must follow Rust community best practices for project structure
- Must enable easy integration with external language bindings  
**Scale/Scope**: Single library crate initially, designed to grow into a comprehensive port of gram-hs pattern data structures and graph views

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity
✅ **PASS**: Project initialization establishes foundation for faithful porting. No functionality ported yet, so no fidelity checks required at this stage.

### II. Correctness & Compatibility (NON-NEGOTIABLE)
✅ **PASS**: Project structure and build configuration prioritize correctness. Multi-target support ensures compatibility across platforms from the start.

### III. Rust Native Idioms
✅ **PASS**: Project will use standard Rust project structure, Cargo configuration, and Rust naming conventions. Development tooling (rustfmt, clippy) enforces idiomatic Rust.

### IV. Multi-Target Library Design
✅ **PASS**: Project initialization includes WASM target configuration and conditional compilation support. Structure designed as shared library from the start.

### V. External Language Bindings & Examples
✅ **PASS**: Example structure will be created with minimal WASM/JavaScript example and placeholders for other languages. Build instructions will be included.

**Gate Status**: ✅ **ALL GATES PASS** - Project initialization aligns with all constitutional principles.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
└── lib.rs              # Library root (will contain Pattern type in future features)

tests/
├── integration/        # Integration tests (test library as external crate)
└── unit/              # Unit tests (inline #[cfg(test)] modules in src/)

examples/
├── wasm-js/           # WASM/JavaScript example
│   ├── Cargo.toml
│   ├── src/
│   ├── www/           # Web assets for WASM example
│   └── README.md
└── README.md          # Examples overview

benches/               # Benchmark tests (optional, for future performance work)

contracts/             # API contracts and type signatures (Phase 1 output)
└── type-signatures.md # Type definitions matching gram-hs
```

**Structure Decision**: Single library crate structure following Rust conventions. The `src/lib.rs` will be the main entry point. Examples are organized in `examples/` directory with subdirectories for different language bindings. The `wasm-js` example will be created as a minimal working example, with placeholders for other language bindings (Python, C) in documentation.

