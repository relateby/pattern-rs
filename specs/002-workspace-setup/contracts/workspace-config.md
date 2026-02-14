# Workspace Configuration Contract

**Feature**: 002-workspace-setup  
**Date**: 2025-01-27

## Overview

This contract defines the structure and requirements for the Cargo workspace configuration.

## Root Cargo.toml Structure

### Workspace Section

```toml
[workspace]
members = ["crates/*"]
resolver = "2"
```

**Requirements**:
- `members` MUST include all crates in the `crates/` directory
- `resolver` MUST be set to `"2"` for proper dependency resolution
- All member crates MUST be valid Rust packages

### Workspace Package Metadata

```toml
[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.70.0"
authors = ["gram-data"]
license = "BSD-3-Clause"
description = "Rust port of gram-hs pattern data structure and graph views"
repository = "https://github.com/relateby/pattern-rs"
```

**Requirements**:
- All fields MUST be defined
- Member crates can inherit these values using `{ workspace = true }`
- Version, edition, and rust-version MUST be consistent across workspace

### Workspace Dependencies

```toml
[workspace.dependencies]
# Shared dependencies available to all crates
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0"
```

**Requirements**:
- Dependencies MUST support all target platforms (native, WASM)
- Versions MUST be compatible across all crates
- Crates reference workspace dependencies using `{ workspace = true }`

## Member Crate Cargo.toml Structure

### Package Section

```toml
[package]
name = "pattern-core"  # Crate-specific name
version.workspace = true
edition.workspace = true
rust-version.workspace = true
authors.workspace = true
license.workspace = true
description = "Core pattern data structures"
```

**Requirements**:
- `name` MUST be unique within workspace
- Other fields can inherit from workspace using `.workspace = true`
- Crate-specific fields (like `description`) can override workspace defaults

### Dependencies Section

```toml
[dependencies]
# Reference workspace dependencies
serde = { workspace = true }
serde_json = { workspace = true }

# Crate-specific dependencies (if needed)
# some-crate = "1.0"
```

**Requirements**:
- Workspace dependencies MUST use `{ workspace = true }`
- Crate-specific dependencies MUST be compatible with workspace dependencies
- All dependencies MUST support target platforms (native, WASM)

### Library Section

```toml
[lib]
name = "pattern_core"  # Library name (snake_case)
path = "src/lib.rs"
```

**Requirements**:
- Library name MUST be valid Rust identifier (snake_case)
- Path MUST point to valid source file
- For placeholder crates, minimal valid structure is sufficient

## Validation Rules

1. **Workspace Integrity**:
   - All member crates MUST compile successfully
   - No circular dependencies between crates
   - All workspace dependencies MUST resolve correctly

2. **Target Compatibility**:
   - All crates MUST compile for native target
   - All crates MUST compile for `wasm32-unknown-unknown` target
   - Conditional compilation features MUST work across crate boundaries

3. **Build Commands**:
   - `cargo build --workspace` MUST build all crates
   - `cargo build -p <crate-name>` MUST build individual crate
   - `cargo test --workspace` MUST test all crates
   - `cargo test -p <crate-name>` MUST test individual crate

## Example: Complete Workspace Cargo.toml

```toml
[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.70.0"
authors = ["gram-data"]
license = "BSD-3-Clause"
description = "Rust port of gram-hs pattern data structure and graph views"
repository = "https://github.com/relateby/pattern-rs"

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0"
```

## Example: Member Crate Cargo.toml

```toml
[package]
name = "pattern-core"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
authors.workspace = true
license.workspace = true
description = "Core pattern data structures"

[dependencies]
serde = { workspace = true }

[lib]
name = "pattern_core"
path = "src/lib.rs"
```
