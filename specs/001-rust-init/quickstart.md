# Quickstart: Rust Project Initialization

**Feature**: 001-rust-init  
**Date**: 2025-12-27

## Overview

This quickstart guide demonstrates how to set up and verify the gram-rs Rust project after initialization. It covers building, testing, and running examples.

## Prerequisites

- **Rust**: 1.70.0 or later (check with `rustc --version`)
- **Cargo**: Included with Rust (check with `cargo --version`)
- **WASM Target** (for WASM compilation): Install with `rustup target add wasm32-unknown-unknown`

## Setup

### 1. Verify Rust Installation

```bash
rustc --version  # Should show 1.70.0 or later
cargo --version   # Should show cargo version
```

### 2. Install WASM Target (Optional but Recommended)

```bash
rustup target add wasm32-unknown-unknown
```

### 3. Clone and Build

```bash
# Clone the repository (when available)
git clone <repository-url>
cd gram-rs

# Build the library
cargo build

# Build for WASM
cargo build --target wasm32-unknown-unknown
```

## Verification Steps

### Step 1: Native Build

```bash
cargo build
```

**Expected Output**: 
- Compilation succeeds
- Library artifact created in `target/debug/libgram.rlib` (or similar)

**If it fails**: Check Rust version, ensure `Cargo.toml` is valid

### Step 2: WASM Build

```bash
cargo build --target wasm32-unknown-unknown
```

**Expected Output**:
- Compilation succeeds for WASM target
- WASM artifact created in `target/wasm32-unknown-unknown/debug/`

**If it fails**: 
- Install WASM target: `rustup target add wasm32-unknown-unknown`
- Check for platform-specific code that needs feature flags

### Step 3: Run Tests

```bash
cargo test
```

**Expected Output**:
- Test suite runs (may be empty initially)
- All tests pass

### Step 4: Check Formatting

```bash
cargo fmt --check
```

**Expected Output**:
- All files are properly formatted
- No formatting changes needed

**If it fails**: Run `cargo fmt` to auto-format

### Step 5: Run Linter

```bash
cargo clippy
```

**Expected Output**:
- No clippy warnings or errors (or only acceptable warnings)

### Step 6: Verify Examples

```bash
# Build WASM example
cd examples/wasm-js
cargo build --target wasm32-unknown-unknown
```

**Expected Output**:
- Example compiles successfully
- WASM artifact created

## Development Workflow

### Daily Development

```bash
# Check code compiles
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy

# Build for both targets
cargo build
cargo build --target wasm32-unknown-unknown
```

### Before Committing

```bash
# Ensure everything passes
cargo fmt --check
cargo clippy
cargo test
cargo build --target wasm32-unknown-unknown
```

## Troubleshooting

### Issue: "could not find `Cargo.toml`"

**Solution**: Ensure you're in the repository root directory

### Issue: "error: failed to run `rustc`"

**Solution**: 
- Check Rust installation: `rustc --version`
- Update Rust: `rustup update stable`

### Issue: "error: target `wasm32-unknown-unknown` not found"

**Solution**: Install WASM target: `rustup target add wasm32-unknown-unknown`

### Issue: "error: failed to resolve: use of undeclared crate"

**Solution**: 
- Run `cargo build` to fetch dependencies
- Check `Cargo.toml` for correct dependency declarations

### Issue: Clippy warnings

**Solution**: 
- Review warnings and fix legitimate issues
- Suppress false positives in code if needed
- Update `clippy.toml` if project-wide configuration needed

## Next Steps

After project initialization is complete:

1. Review the project structure
2. Familiarize yourself with the gram-hs reference implementation
3. Wait for next feature specification to begin porting functionality
4. Set up IDE/editor with Rust support (rust-analyzer recommended)

## Resources

- **gram-hs Reference**: https://github.com/gram-data/gram-hs
- **Rust Book**: https://doc.rust-lang.org/book/
- **WASM Book**: https://rustwasm.github.io/docs/book/
- **Cargo Book**: https://doc.rust-lang.org/cargo/

## Project Structure Reference

```
gram-rs/
├── Cargo.toml          # Project manifest
├── Cargo.lock         # Dependency lock file
├── src/
│   └── lib.rs         # Library root
├── tests/             # Integration tests
├── examples/          # Example projects
│   └── wasm-js/      # WASM/JavaScript example
├── benches/          # Benchmarks (future)
├── .gitignore        # Git ignore rules
├── rustfmt.toml      # Formatting config
├── clippy.toml       # Linting config
├── README.md         # Project documentation
└── LICENSE           # BSD-3-Clause license
```

