# Feature Specification: Multi-Crate Workspace Setup

**Feature Branch**: `002-workspace-setup`  
**Created**: 2025-01-27  
**Status**: Draft  
**Input**: User description: "Revise project into a multi-crate workspace as described in feature 002 of TODO.md"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Developer Works with Modular Crates (Priority: P1)

A Rust developer wants to work on a specific part of the pattern-rs library (e.g., pattern operations) without needing to build or understand the entire codebase. They need the project organized into separate, independently buildable crates that can be developed and tested in isolation.

**Why this priority**: Modular organization is foundational for maintainability, parallel development, and clear separation of concerns. Without this structure, all code lives in one crate, making it harder to understand dependencies, test components independently, and scale the project as it grows.

**Independent Test**: Can be fully tested by verifying that each crate can be built independently (`cargo build -p pattern-core`), that workspace commands work (`cargo build --workspace`), and that developers can work on one crate without building others.

**Acceptance Scenarios**:

1. **Given** the project is organized as a workspace, **When** a developer runs `cargo build --workspace`, **Then** all crates build successfully with proper dependency resolution
2. **Given** a developer wants to work only on pattern operations, **When** they run `cargo build -p pattern-ops`, **Then** only that crate and its dependencies build
3. **Given** the workspace structure exists, **When** a developer runs `cargo test --workspace`, **Then** all tests across all crates run successfully
4. **Given** crates are separated by concern, **When** a developer examines the project structure, **Then** they can easily identify which crate contains which functionality

---

### User Story 2 - CI/CD Pipeline Validates All Crates (Priority: P2)

A maintainer needs automated validation that all crates in the workspace build, test, and meet quality standards. They need a CI/CD pipeline that runs checks across the entire workspace to catch integration issues early.

**Why this priority**: Automated validation ensures that changes to one crate don't break others, and that the workspace remains in a consistent, buildable state. This is critical for collaborative development and prevents regressions.

**Independent Test**: Can be fully tested by verifying that CI/CD pipeline runs successfully, executes tests for all crates, and reports build/test failures clearly.

**Acceptance Scenarios**:

1. **Given** CI/CD pipeline is configured, **When** a developer pushes code, **Then** the pipeline automatically builds all crates and runs all tests
2. **Given** a crate fails to build, **When** CI runs, **Then** the pipeline reports which crate failed and why
3. **Given** workspace dependencies are configured, **When** CI runs, **Then** dependency resolution works correctly across all crates
4. **Given** code quality checks are configured, **When** CI runs, **Then** linting and formatting checks execute for all crates

---

### User Story 3 - Test Synchronization Infrastructure (Priority: P3)

A developer needs to ensure that pattern-rs tests remain synchronized with the gram-hs reference implementation. They need infrastructure to extract, compare, and validate test cases from both implementations to maintain behavioral equivalence.

**Why this priority**: While not blocking for initial workspace setup, test synchronization is essential for maintaining correctness as the port progresses. Establishing this infrastructure early ensures it's available when needed.

**Independent Test**: Can be fully tested by verifying that test synchronization utilities exist, can extract test data from gram-hs, and provide comparison mechanisms (even if initially as placeholders).

**Acceptance Scenarios**:

1. **Given** test synchronization infrastructure exists, **When** a developer wants to verify test parity, **Then** they can run synchronization utilities to compare test cases
2. **Given** test data extraction is configured, **When** gram-hs tests are updated, **Then** the infrastructure can extract updated test cases for comparison
3. **Given** test comparison utilities exist, **When** a developer runs comparison, **Then** they receive clear reports on test coverage differences

---

### Edge Cases

- What happens when a crate has circular dependencies? (Workspace configuration should prevent or clearly identify circular dependencies)
- How does the workspace handle optional features across crates? (Feature flags should work correctly across crate boundaries)
- What if a developer only wants to build for a specific target (e.g., WASM)? (Workspace should support target-specific builds)
- How are workspace-level dependencies managed vs crate-specific dependencies? (Clear separation between shared and crate-specific dependencies)
- What happens when placeholder crates (pattern-store, pattern-wasm) are created but not yet implemented? (Placeholder crates should have minimal structure that doesn't break builds)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Project MUST be organized as a Cargo workspace with a root `Cargo.toml` defining workspace members
- **FR-002**: Workspace MUST include a `crates/` directory containing all workspace member crates
- **FR-003**: Workspace MUST include `crates/pattern-core/` crate for core pattern data structures
- **FR-004**: Workspace MUST include `crates/pattern-ops/` crate for pattern operations and algorithms
- **FR-005**: Workspace MUST include `crates/gram-codec/` crate for gram notation serialization/deserialization
- **FR-006**: Workspace MUST include `crates/pattern-store/` crate as a placeholder for future optimized storage
- **FR-007**: Workspace MUST include `crates/pattern-wasm/` crate as a placeholder for future WASM bindings
- **FR-008**: Each crate MUST have its own `Cargo.toml` with appropriate metadata and dependencies
- **FR-009**: Workspace MUST configure shared dependencies in workspace-level `Cargo.toml` to avoid duplication
- **FR-010**: Workspace MUST support building all crates with `cargo build --workspace`
- **FR-011**: Workspace MUST support building individual crates with `cargo build -p <crate-name>`
- **FR-012**: Workspace MUST support testing all crates with `cargo test --workspace`
- **FR-013**: Workspace MUST support testing individual crates with `cargo test -p <crate-name>`
- **FR-014**: CI/CD pipeline MUST be configured to build and test all workspace crates
- **FR-015**: CI/CD pipeline MUST run on all pushes and pull requests
- **FR-016**: CI/CD pipeline MUST report build and test failures clearly with crate identification
- **FR-017**: Test synchronization infrastructure MUST be established (structure and utilities, even if initially minimal)
- **FR-018**: Test synchronization MUST support extracting test data from gram-hs reference implementation
- **FR-019**: Test synchronization MUST provide mechanisms for comparing test cases between gram-hs and pattern-rs
- **FR-020**: Placeholder crates (pattern-store, pattern-wasm) MUST have minimal valid structure that compiles without errors
- **FR-021**: Workspace MUST maintain compatibility with existing development workflows (rustfmt, clippy, etc.)
- **FR-022**: Workspace MUST support conditional compilation features that work across crate boundaries
- **FR-023**: Workspace MUST configure appropriate Rust edition and MSRV consistently across all crates

### Key Entities *(include if feature involves data)*

- **Workspace**: The top-level Cargo workspace structure that coordinates multiple related crates, defined by root `Cargo.toml` with `[workspace]` section
- **Crate**: An individual Rust library or binary package within the workspace, each with its own `Cargo.toml` and source code
- **Workspace Dependencies**: Shared dependencies defined at workspace level and reused across member crates to ensure version consistency
- **CI/CD Pipeline**: Automated build, test, and validation system that runs on code changes to ensure workspace integrity
- **Test Synchronization Infrastructure**: Utilities and processes for maintaining test parity between pattern-rs and gram-hs reference implementation

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can successfully build the entire workspace with `cargo build --workspace` within 2 minutes (assuming dependencies are cached)
- **SC-002**: Developers can build any individual crate independently within 30 seconds (e.g., `cargo build -p pattern-core`)
- **SC-003**: All workspace crates compile successfully for both native Rust targets and `wasm32-unknown-unknown` target
- **SC-004**: CI/CD pipeline completes successfully for all workspace crates within 10 minutes
- **SC-005**: Test suite runs successfully across all crates with `cargo test --workspace` without configuration errors
- **SC-006**: Workspace structure clearly separates concerns as evidenced by developers being able to identify crate purposes without reading source code
- **SC-007**: Test synchronization infrastructure is established and can extract at least basic test data from gram-hs (verified by successful extraction of sample test cases)
- **SC-008**: Placeholder crates compile successfully and don't break workspace builds (verified by successful `cargo check --workspace`)

## Assumptions

- Existing project structure and code from feature 001 (rust-init) will be migrated into appropriate crates
- Workspace will use Cargo's resolver version 2 for dependency resolution
- All crates will share the same Rust edition (2021) and minimum supported Rust version
- CI/CD will use GitHub Actions (as mentioned in TODO) unless otherwise specified
- Test synchronization with gram-hs will initially be manual or semi-automated, with full automation as a future enhancement
- Placeholder crates will contain minimal "hello world" style code to ensure they compile
- Workspace dependencies will be managed centrally to avoid version conflicts
- Development tooling (rustfmt, clippy) will work seamlessly with workspace structure
