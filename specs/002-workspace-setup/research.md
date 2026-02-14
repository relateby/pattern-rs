# Research: Multi-Crate Workspace Setup

**Feature**: 002-workspace-setup  
**Date**: 2025-01-27  
**Purpose**: Research best practices and patterns for Cargo workspace setup, CI/CD configuration, and test synchronization infrastructure

## Research Questions

### 1. Cargo Workspace Best Practices

**Question**: What are the recommended patterns for organizing a Cargo workspace with multiple crates?

**Findings**:
- **Workspace Structure**: Use `crates/` directory to contain all workspace member crates. This provides clear separation and makes it easy to identify workspace members.
- **Resolver Version**: Use resolver version 2 (`resolver = "2"`) for better dependency resolution, especially for workspaces with multiple crates.
- **Workspace Dependencies**: Define shared dependencies in `[workspace.dependencies]` to ensure version consistency across all crates. Individual crates reference workspace dependencies using `{ workspace = true }`.
- **Workspace Package Metadata**: Use `[workspace.package]` to define shared metadata (version, edition, license, etc.) that can be inherited by member crates using `{ workspace = true }`.
- **Member Selection**: Use `members = ["crates/*"]` to automatically include all crates in the `crates/` directory, or explicitly list members for more control.

**Decision**: Use `crates/` directory structure with resolver version 2. Define shared dependencies and package metadata at workspace level.

**Rationale**: This follows Rust community best practices and ensures consistent dependency versions across the workspace.

**Alternatives Considered**:
- Flat structure (all crates at root): Rejected - harder to organize and identify workspace members
- Explicit member listing: Rejected - `crates/*` is more maintainable as new crates are added

### 2. CI/CD for Rust Workspaces

**Question**: What are the best practices for setting up GitHub Actions CI/CD for a Cargo workspace?

**Findings**:
- **Matrix Strategy**: Use matrix builds to test multiple Rust versions and targets (native, WASM) efficiently.
- **Caching**: Cache Cargo registry and target directory to speed up builds. Use `actions/cache@v3` with key based on `Cargo.lock`.
- **Workspace Commands**: Use `cargo build --workspace` and `cargo test --workspace` to build and test all crates.
- **Individual Crate Testing**: Can test individual crates with `cargo test -p <crate-name>` for faster feedback on specific changes.
- **Job Organization**: Separate jobs for different concerns (build, test, lint, format) allow parallel execution and clearer failure reporting.
- **WASM Target**: Add `wasm32-unknown-unknown` target and test WASM compilation separately.

**Decision**: Use GitHub Actions with matrix strategy for Rust versions and targets. Separate jobs for build, test, lint, and format. Cache Cargo artifacts.

**Rationale**: Matrix builds provide comprehensive coverage while caching reduces CI time. Separate jobs enable parallel execution and clear failure reporting.

**Alternatives Considered**:
- Single job for all checks: Rejected - slower and harder to identify specific failures
- No caching: Rejected - significantly increases CI time

### 3. Test Synchronization Infrastructure

**Question**: How should test synchronization between gram-hs and pattern-rs be structured?

**Findings**:
- **JSON Test Format**: Use a structured JSON format to represent test cases that both implementations can produce and consume.
- **Extraction Scripts**: Create utilities to extract test cases from gram-hs (Haskell) and convert to common format.
- **Comparison Tools**: Build comparison utilities to identify test coverage differences and behavioral mismatches.
- **Incremental Approach**: Start with manual/semi-automated extraction, evolve to full automation as patterns emerge.
- **Test Case Schema**: Define a schema that captures:
  - Test name and description
  - Input (gram notation or pattern)
  - Expected output (pattern or operation result)
  - Operations to test (match, transform, etc.)

**Decision**: Create initial infrastructure with:
- JSON schema for test cases
- Basic extraction utilities (can be manual initially)
- Comparison scripts to identify differences
- Documentation for the synchronization process

**Rationale**: Starting with a structured approach enables future automation while providing immediate value for maintaining test parity.

**Alternatives Considered**:
- Full automation from start: Rejected - too complex without understanding test patterns first
- No synchronization: Rejected - violates requirement FR-017, FR-018, FR-019

### 4. Workspace Dependency Management

**Question**: How should dependencies be managed across workspace crates?

**Findings**:
- **Workspace Dependencies**: Define common dependencies in `[workspace.dependencies]` section.
- **Crate-Specific Dependencies**: Each crate can have its own `[dependencies]` section for crate-specific needs.
- **Version Consistency**: Workspace dependencies ensure all crates use the same version, preventing conflicts.
- **Feature Flags**: Workspace dependencies can define features that crates can enable/disable.
- **Development Dependencies**: Can be defined at workspace level or crate level depending on scope.

**Decision**: 
- Define shared dependencies (serde, serde_json, thiserror, etc.) in `[workspace.dependencies]`
- Each crate references workspace dependencies using `{ workspace = true }`
- Crate-specific dependencies go in individual crate `Cargo.toml` files
- Development dependencies at workspace level for shared tooling

**Rationale**: Centralized dependency management prevents version conflicts while allowing crate-specific needs.

**Alternatives Considered**:
- All dependencies in each crate: Rejected - leads to version conflicts and duplication
- All dependencies at workspace level: Rejected - too restrictive, some crates may need unique dependencies

### 5. Placeholder Crate Structure

**Question**: What minimal structure should placeholder crates have to compile successfully?

**Findings**:
- **Minimal lib.rs**: A simple `pub fn` or empty library is sufficient for compilation.
- **Cargo.toml**: Must have valid package metadata (name, version, edition).
- **Dependencies**: Can be empty or reference workspace dependencies if needed.
- **Documentation**: Should include `//!` doc comments explaining the placeholder nature.

**Decision**: Placeholder crates will have:
- Minimal `lib.rs` with a simple public function or module
- Valid `Cargo.toml` with package metadata
- Documentation comments explaining placeholder status
- No dependencies initially (can add workspace dependencies if needed)

**Rationale**: Minimal structure ensures compilation success without adding unnecessary complexity.

**Alternatives Considered**:
- Empty crates: Rejected - may cause compilation issues
- Full skeleton: Rejected - too much work for placeholders that will be replaced

## Summary of Decisions

1. **Workspace Structure**: `crates/` directory with resolver version 2
2. **CI/CD**: GitHub Actions with matrix builds, caching, and separate jobs
3. **Test Synchronization**: JSON-based format with extraction and comparison utilities
4. **Dependency Management**: Workspace-level shared dependencies, crate-specific as needed
5. **Placeholder Crates**: Minimal valid structure that compiles

All research questions resolved. No NEEDS CLARIFICATION markers remain.
