# Data Model: Multi-Crate Workspace Setup

**Feature**: 002-workspace-setup  
**Date**: 2025-01-27

## Overview

This feature establishes the workspace structure and infrastructure. The data model focuses on the organizational entities rather than runtime data structures.

## Entities

### Workspace

**Description**: The top-level Cargo workspace that coordinates multiple related crates.

**Attributes**:
- **Root Cargo.toml**: Contains `[workspace]` section defining members and configuration
- **Resolver Version**: Version 2 (as per research findings)
- **Members**: List of crate paths (e.g., `["crates/*"]` or explicit list)
- **Shared Metadata**: Version, edition, license, repository (defined in `[workspace.package]`)
- **Shared Dependencies**: Common dependencies available to all crates (defined in `[workspace.dependencies]`)

**Relationships**:
- Contains multiple **Crate** entities
- Defines **Workspace Dependencies** available to all crates
- Configured by **CI/CD Pipeline** for automated validation

**Validation Rules**:
- Must have at least one member crate
- All members must have valid `Cargo.toml` files
- Resolver version must be specified
- Shared metadata must be consistent across workspace

### Crate

**Description**: An individual Rust library or binary package within the workspace.

**Attributes**:
- **Name**: Crate identifier (e.g., `pattern-core`, `pattern-ops`)
- **Path**: Directory path relative to workspace root (e.g., `crates/pattern-core/`)
- **Cargo.toml**: Crate-specific configuration file
- **Source Directory**: `src/` directory containing Rust source code
- **Type**: Library crate (`[lib]`) or binary crate (`[[bin]]`)
- **Dependencies**: Crate-specific dependencies (beyond workspace dependencies)
- **Features**: Optional feature flags for conditional compilation

**Relationships**:
- Belongs to one **Workspace**
- May depend on other **Crate** entities in the workspace
- Uses **Workspace Dependencies** from parent workspace
- Validated by **CI/CD Pipeline**

**Validation Rules**:
- Must have valid `Cargo.toml` with package metadata
- Must compile successfully for all supported targets
- Must not create circular dependencies with other crates
- Placeholder crates must have minimal valid structure

**Crate Types in This Feature**:
1. **pattern-core**: Core pattern data structures
2. **pattern-ops**: Pattern operations and algorithms
3. **gram-codec**: Gram notation serialization/deserialization
4. **pattern-store**: Placeholder for optimized storage (future)
5. **pattern-wasm**: Placeholder for WASM bindings (future)

### Workspace Dependencies

**Description**: Shared dependencies defined at workspace level and reused across member crates.

**Attributes**:
- **Name**: Dependency name (e.g., `serde`, `serde_json`)
- **Version**: Version constraint (e.g., `"1.0"`)
- **Features**: Optional feature flags (e.g., `["derive"]`)
- **Source**: Workspace-local path or crates.io (default)

**Relationships**:
- Defined in **Workspace** configuration
- Used by multiple **Crate** entities via `{ workspace = true }`

**Validation Rules**:
- Versions must be compatible across all crates using the dependency
- Feature flags must be compatible with crate needs
- Must support all target platforms (native, WASM)

### CI/CD Pipeline

**Description**: Automated build, test, and validation system that runs on code changes.

**Attributes**:
- **Platform**: GitHub Actions (as per spec assumptions)
- **Triggers**: Push events, pull request events
- **Jobs**: Separate jobs for build, test, lint, format
- **Matrix Strategy**: Multiple Rust versions and targets (native, WASM)
- **Caching**: Cargo registry and target directory caching
- **Artifacts**: Build outputs, test reports

**Relationships**:
- Validates **Workspace** structure
- Tests all **Crate** entities
- Reports failures with crate identification

**Validation Rules**:
- Must run on all pushes and pull requests (FR-015)
- Must build and test all workspace crates (FR-014)
- Must report failures clearly with crate identification (FR-016)
- Must complete within 10 minutes (SC-004)

### Test Synchronization Infrastructure

**Description**: Utilities and processes for maintaining test parity between pattern-rs and gram-hs.

**Attributes**:
- **Test Case Format**: JSON schema for test case representation
- **Extraction Tools**: Utilities to extract test cases from gram-hs
- **Comparison Tools**: Utilities to compare test cases between implementations
- **Storage**: Test case files in `tests/common/` or similar location

**Relationships**:
- Extracts test data from gram-hs reference implementation
- Compares test cases between pattern-rs and gram-hs
- Supports **Workspace** test coverage

**Validation Rules**:
- Must support extracting test data from gram-hs (FR-018)
- Must provide comparison mechanisms (FR-019)
- Must be established (structure and utilities) even if initially minimal (FR-017)

## State Transitions

### Workspace Creation
1. **Initial State**: Single crate project (from feature 001)
2. **Transition**: Convert to workspace structure
3. **Final State**: Multi-crate workspace with 5 crates

### Crate Addition
1. **Initial State**: Workspace with N crates
2. **Transition**: Add new crate to `crates/` directory
3. **Final State**: Workspace with N+1 crates (automatically included via `members = ["crates/*"]`)

### CI/CD Execution
1. **Initial State**: Code change pushed
2. **Transition**: CI/CD pipeline triggered
3. **Final State**: Build/test results reported (pass/fail with crate identification)

## Notes

- This data model represents organizational structure, not runtime data
- Entities are validated through build system and CI/CD rather than runtime checks
- Placeholder crates follow the same structure as regular crates but with minimal implementation
