# Data Model: Rust Project Initialization

**Feature**: 001-rust-init  
**Date**: 2025-12-27

## Overview

This feature involves project structure and configuration, not runtime data structures. The "data model" here represents the project configuration entities and their relationships.

## Configuration Entities

### Cargo Project Configuration

**Entity**: `Cargo.toml` (Cargo Project Manifest)

**Fields**:
- `package.name`: String - Project name (e.g., "gram" or "gram-rs")
- `package.version`: String - Semantic version (e.g., "0.1.0")
- `package.edition`: String - Rust edition ("2021")
- `package.authors`: Array of Strings - Project authors
- `package.license`: String - License identifier ("BSD-3-Clause")
- `package.description`: String - Project description
- `package.repository`: String - Git repository URL
- `package.rust-version`: String - Minimum supported Rust version ("1.70.0")
- `lib.name`: String - Library name (defaults to package name)
- `lib.path`: String - Path to library root ("src/lib.rs")
- `[dependencies]`: Table - Runtime dependencies (empty initially)
- `[dev-dependencies]`: Table - Development-only dependencies
- `[build-dependencies]`: Table - Build script dependencies (if needed)

**Relationships**:
- Contains workspace configuration if multiple crates are used
- References example crates in `examples/` directory
- Defines feature flags for conditional compilation

**Validation Rules**:
- Version must follow semantic versioning (MAJOR.MINOR.PATCH)
- Edition must be valid ("2018", "2021", or future editions)
- MSRV must be a valid Rust version string
- License must be a valid SPDX identifier

### Workspace Configuration (Optional)

**Entity**: Workspace (if multiple crates needed)

**Fields**:
- `workspace.members`: Array of Strings - Crate paths
- `workspace.default-members`: Array of Strings - Default crates to build
- `workspace.package`: Table - Default package settings inherited by members

**Relationships**:
- Contains member crates (library, examples, etc.)
- Shares dependency versions across members

**Validation Rules**:
- All members must be valid Cargo projects
- Default members must be subset of members

### Rustfmt Configuration

**Entity**: `rustfmt.toml` or `.rustfmt.toml`

**Fields**:
- Standard rustfmt options (edition, max_width, etc.)
- Custom formatting preferences

**Relationships**:
- Applies to all Rust source files in the project

**Validation Rules**:
- Must be valid TOML
- Options must be valid rustfmt configuration keys

### Clippy Configuration

**Entity**: `clippy.toml` or `.clippy.toml`

**Fields**:
- Lint level settings (warn, deny, allow)
- Custom lint configurations

**Relationships**:
- Applies to all Rust source files when running `cargo clippy`

**Validation Rules**:
- Must be valid TOML
- Lint names must be valid clippy lints

### Git Configuration

**Entity**: `.gitignore`

**Fields**:
- Patterns for ignored files and directories

**Key Patterns**:
- `target/` - Cargo build artifacts
- `Cargo.lock` - Dependency lock file (included for this project)
- `**/*.rs.bk` - Rustfmt backup files
- IDE-specific ignores

**Relationships**:
- Applies to entire repository

### Example Project Structure

**Entity**: Example Crate (e.g., `examples/wasm-js/`)

**Fields**:
- `Cargo.toml` - Example-specific manifest
- `src/main.rs` or `src/lib.rs` - Example source code
- `README.md` - Example documentation
- `www/` - Web assets (for WASM examples)

**Relationships**:
- Depends on parent library crate
- May have its own dependencies (e.g., `wasm-bindgen`)

**Validation Rules**:
- Must compile successfully
- Must demonstrate library usage
- Must include build/run instructions

## State Transitions

### Project Initialization Flow

1. **Empty Repository** → **Initialized Project**
   - Create `Cargo.toml` with metadata
   - Create `src/lib.rs` (empty or minimal)
   - Create directory structure
   - Add configuration files (rustfmt, clippy, gitignore)
   - Add README and LICENSE

2. **Single Crate** → **Workspace** (future, if needed)
   - Create `Cargo.toml` at root with `[workspace]`
   - Move library to `crates/gram/` or similar
   - Update paths and references

## Notes

- This is a structural/configuration model, not a runtime data model
- Actual data structures (Pattern, GraphView, etc.) will be defined in future features when porting gram-hs functionality
- The configuration entities here establish the foundation for those future data structures

