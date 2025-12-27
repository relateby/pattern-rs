# Research: Rust Project Initialization

**Feature**: 001-rust-init  
**Date**: 2025-12-27  
**Purpose**: Research Rust best practices for project initialization, multi-target compilation, and external language bindings

## Research Questions

### 1. Rust Minimum Supported Rust Version (MSRV) Policy

**Question**: What MSRV should be set for a new Rust library project in 2025?

**Decision**: MSRV 1.70.0 (released June 2023)

**Rationale**:
- 1.70.0 provides stable support for essential features needed for library development
- Balances compatibility with modern Rust features
- Allows use of stable async/await, const generics improvements, and other modern features
- 18+ months old at project start, providing reasonable ecosystem compatibility
- Can be updated if specific features require newer versions

**Alternatives Considered**:
- 1.65.0: Too conservative, misses useful features
- 1.75.0+: Too new, may limit ecosystem compatibility
- Latest stable: Not practical for library distribution

**References**:
- Rust release notes: https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html
- Rust MSRV policy discussions in ecosystem

### 2. Rust Edition Selection

**Question**: Which Rust edition should be used?

**Decision**: Edition 2021

**Rationale**:
- Current stable edition (2024 edition not yet released)
- Provides improved module system, better prelude, and other quality-of-life improvements
- Standard for new projects in 2025
- No migration burden (new project)

**Alternatives Considered**:
- Edition 2018: Outdated, missing modern features
- Edition 2024: Not yet available

### 3. WASM Target Configuration

**Question**: How should WASM compilation be configured for a library that must work in both native and WASM contexts?

**Decision**: 
- Use `wasm32-unknown-unknown` target (standard WASM target)
- Configure conditional compilation using `#[cfg(target_arch = "wasm32")]`
- Use feature flags for WASM-specific dependencies (e.g., `wasm-bindgen`)

**Rationale**:
- `wasm32-unknown-unknown` is the standard target for pure WASM without host bindings
- Conditional compilation allows platform-specific code paths
- Feature flags prevent WASM-only dependencies from affecting native builds
- Aligns with Rust ecosystem best practices

**Alternatives Considered**:
- `wasm32-wasi`: Includes WASI syscalls, but adds unnecessary complexity for library code
- Separate workspace crate: Overkill for initial setup, can be added later if needed

**References**:
- Rust WASM book: https://rustwasm.github.io/docs/book/
- wasm-bindgen documentation

### 4. Project Structure: Single Crate vs Workspace

**Question**: Should the project start as a single crate or a workspace?

**Decision**: Single library crate initially, with examples as separate example binaries

**Rationale**:
- Simpler initial setup
- Examples can be in `examples/` directory (standard Rust convention)
- Can migrate to workspace later if needed (e.g., if examples need their own dependencies)
- Follows YAGNI principle - start simple, add complexity when needed

**Alternatives Considered**:
- Workspace from start: Adds complexity without immediate benefit
- Multiple crates: Premature optimization

### 5. External Language Binding Strategy

**Question**: What approach should be used for external language bindings?

**Decision**: 
- **WASM/JavaScript**: Use `wasm-bindgen` crate (industry standard)
- **Python**: Use `pyo3` crate (when needed in future)
- **C**: Use `cbindgen` for generating C headers (when needed in future)
- Start with minimal WASM/JavaScript example, document others as placeholders

**Rationale**:
- `wasm-bindgen` is the de facto standard for Rust-WASM interop
- `pyo3` is the most mature Python binding solution
- `cbindgen` generates C-compatible headers automatically
- Starting with one working example (WASM) provides concrete guidance for API design
- Other bindings can be added incrementally

**Alternatives Considered**:
- Manual C FFI: Too error-prone, `cbindgen` is standard
- `cxx` for C++: Not needed for initial setup
- Custom bindings: Unnecessary complexity

**References**:
- wasm-bindgen: https://rustwasm.github.io/wasm-bindgen/
- pyo3: https://pyo3.rs/
- cbindgen: https://github.com/eqrion/cbindgen

### 6. Development Tooling Configuration

**Question**: What configuration should be used for rustfmt and clippy?

**Decision**:
- Use default `rustfmt.toml` with standard settings
- Configure `clippy.toml` with pedantic lints enabled
- Set up pre-commit or CI checks for formatting and linting

**Rationale**:
- Rustfmt defaults are well-designed and widely adopted
- Clippy pedantic mode catches more potential issues
- Automated checks ensure consistency
- Standard configuration reduces maintenance burden

**Alternatives Considered**:
- Custom formatting rules: Unnecessary unless specific team preferences
- Minimal clippy config: Misses valuable warnings

### 7. Cargo.lock Version Control Policy

**Question**: Should `Cargo.lock` be committed for a library project?

**Decision**: Yes, commit `Cargo.lock` for this project

**Rationale**:
- While libraries typically don't commit `Cargo.lock`, this project will have examples and may have binaries
- Ensures reproducible builds for examples and tests
- Helps with CI/CD consistency
- Can be removed later if project becomes pure library

**Alternatives Considered**:
- Don't commit: Standard for libraries, but examples may need it
- Conditional commit: Too complex for initial setup

### 8. License Selection

**Question**: What license should be used?

**Decision**: BSD-3-Clause (matching gram-hs)

**Rationale**:
- Matches reference implementation (gram-hs uses BSD-3-Clause)
- Ensures compatibility and consistency
- Permissive license suitable for library distribution

**Alternatives Considered**:
- MIT: Similar, but gram-hs uses BSD-3-Clause
- Apache-2.0: More complex, not matching reference

## Summary

All research questions resolved. Key decisions:
- MSRV: 1.70.0, Edition: 2021
- WASM: `wasm32-unknown-unknown` with conditional compilation
- Structure: Single crate with examples directory
- Bindings: `wasm-bindgen` for JavaScript, `pyo3`/`cbindgen` for future Python/C
- Tooling: Standard rustfmt, pedantic clippy
- License: BSD-3-Clause

No NEEDS CLARIFICATION markers remain. Ready for Phase 1 design.

