# Type Signatures: Rust Project Initialization

**Feature**: 001-rust-init  
**Date**: 2025-12-27

## Overview

This feature establishes the project structure and configuration. No runtime types are defined yet - those will come when porting gram-hs functionality. This document outlines the configuration "contracts" and structure that will be created.

## Project Structure Contracts

### Cargo.toml Contract

```toml
[package]
name = "gram"  # or "gram-rs"
version = "0.1.0"
edition = "2021"
rust-version = "1.70.0"
authors = ["..."]

license = "BSD-3-Clause"
description = "Rust port of gram-hs pattern data structure and graph views"
repository = "https://github.com/gram-data/gram-rs"

[lib]
name = "gram"
path = "src/lib.rs"

[dependencies]
# Initially empty - dependencies added as functionality is ported

[dev-dependencies]
# Development tools and test utilities

[features]
default = []
# Feature flags for conditional compilation (e.g., "wasm" for WASM-specific code)
```

### Library Root Contract

**File**: `src/lib.rs`

```rust
// Initially minimal - will contain Pattern type and other types as ported
// Must compile successfully for both native and WASM targets

#[cfg(test)]
mod tests {
    // Unit tests will be added here
}
```

**Requirements**:
- Must compile for `wasm32-unknown-unknown` target
- Must not include platform-specific code without feature flags
- Must follow Rust naming conventions

### Example Contract (WASM/JavaScript)

**Structure**: `examples/wasm-js/`

```toml
# examples/wasm-js/Cargo.toml
[package]
name = "gram-wasm-example"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
gram = { path = "../.." }
wasm-bindgen = "0.2"

[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-Os", "--enable-mutable-globals"]
```

**Source**: `examples/wasm-js/src/lib.rs`

```rust
// Minimal example demonstrating library usage from WASM
// Will be expanded as functionality is ported
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! (from gram-rs)", name)
}
```

**Requirements**:
- Must compile to WASM successfully
- Must include build instructions in README
- Must demonstrate basic library integration

## Configuration File Contracts

### rustfmt.toml

```toml
edition = "2021"
# Use default settings - can be customized if needed
```

### clippy.toml (or in Cargo.toml)

```toml
# Enable pedantic lints for better code quality
# Specific lint configurations can be added
```

### .gitignore

```
# Rust
target/
**/*.rs.bk
Cargo.lock  # Note: We're including this for examples

# IDE
.idea/
.vscode/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db
```

## Build Target Contracts

### Native Target

**Command**: `cargo build`

**Requirements**:
- Must compile without errors
- Must produce `target/debug/libgram.rlib` (or similar)
- Must pass `cargo check`
- Must pass `cargo clippy` (with appropriate config)

### WASM Target

**Command**: `cargo build --target wasm32-unknown-unknown`

**Requirements**:
- Must compile without errors
- Must produce `target/wasm32-unknown-unknown/debug/libgram.rlib`
- Must not include blocking I/O or file system access in public API
- Platform-specific code must be behind feature flags

## Future Type Contracts

When gram-hs functionality is ported, the following types will be defined (outline only):

- `Pattern<T>` - Core pattern data structure (equivalent to gram-hs `Pattern v`)
- `GraphView` - Graph view trait/type (equivalent to gram-hs `GraphView`)
- Associated types and traits matching gram-hs API

These will be defined in future feature specifications.

## Validation

All contracts must be validated by:
1. Successful compilation (`cargo build`)
2. Successful WASM compilation (`cargo build --target wasm32-unknown-unknown`)
3. Passing development tooling (`cargo fmt --check`, `cargo clippy`)
4. Successful test execution (`cargo test`)

