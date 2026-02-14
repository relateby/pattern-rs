# Quickstart: Multi-Crate Workspace

**Feature**: 002-workspace-setup  
**Date**: 2025-01-27

## Overview

This guide provides a quick introduction to working with the pattern-rs multi-crate workspace structure.

## Workspace Structure

The project is organized as a Cargo workspace with multiple crates:

```
pattern-rs/
├── Cargo.toml              # Workspace root configuration
├── crates/
│   ├── pattern-core/        # Core pattern data structures
│   ├── pattern-ops/         # Pattern operations
│   ├── gram-codec/          # Gram notation codec
│   ├── pattern-store/       # Storage (placeholder)
│   └── pattern-wasm/        # WASM bindings (placeholder)
└── .github/workflows/       # CI/CD configuration
```

## Basic Commands

### Building the Workspace

Build all crates:
```bash
cargo build --workspace
```

Build a specific crate:
```bash
cargo build -p pattern-core
```

Build for WASM target:
```bash
cargo build --workspace --target wasm32-unknown-unknown
```

### Testing

Run all tests:
```bash
cargo test --workspace
```

Test a specific crate:
```bash
cargo test -p pattern-ops
```

### Code Quality

Run clippy on all crates:
```bash
cargo clippy --workspace -- -D warnings
```

Check formatting:
```bash
cargo fmt --all -- --check
```

Format code:
```bash
cargo fmt --all
```

## Working with Individual Crates

### Adding a Dependency

1. **Workspace Dependency** (shared across crates):
   - Add to `[workspace.dependencies]` in root `Cargo.toml`
   - Reference in crate `Cargo.toml` using `{ workspace = true }`

2. **Crate-Specific Dependency**:
   - Add directly to crate's `Cargo.toml` `[dependencies]` section

### Creating a New Crate

1. Create directory: `crates/my-new-crate/`
2. Create `Cargo.toml` with package metadata
3. Create `src/lib.rs` with minimal code
4. Crate is automatically included via `members = ["crates/*"]`

Example `Cargo.toml`:
```toml
[package]
name = "my-new-crate"
version.workspace = true
edition.workspace = true
# ... other workspace-inherited fields

[dependencies]
serde = { workspace = true }

[lib]
name = "my_new_crate"
path = "src/lib.rs"
```

## CI/CD

The workspace includes automated CI/CD via GitHub Actions:

- **Build**: Compiles all crates for native and WASM targets
- **Test**: Runs all workspace tests
- **Lint**: Checks code with clippy
- **Format**: Verifies code formatting

CI runs automatically on:
- Push to main/develop branches
- Pull requests to main/develop branches

## Test Synchronization

Test synchronization infrastructure supports maintaining parity with gram-hs:

- Test cases stored in JSON format (see `contracts/test-sync-format.md`)
- Extraction tools for gram-hs test cases
- Comparison utilities for identifying differences

Location: `tests/common/` or `scripts/sync-tests/`

## Workspace Dependencies

Common dependencies are defined at workspace level:

- `serde` - Serialization framework
- `serde_json` - JSON support
- `thiserror` - Error handling

Crates reference these using `{ workspace = true }`:

```toml
[dependencies]
serde = { workspace = true }
```

## Troubleshooting

### Build Fails for One Crate

1. Check crate's `Cargo.toml` for errors
2. Verify dependencies are correct
3. Try building just that crate: `cargo build -p <crate-name>`

### Circular Dependency Error

1. Review crate dependencies
2. Ensure no crate depends on itself (directly or transitively)
3. Consider extracting shared code to a common crate

### WASM Build Fails

1. Verify `wasm32-unknown-unknown` target is installed:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. Check that all dependencies support WASM
3. Review conditional compilation features

### CI Fails Locally

1. Run the same commands locally:
   ```bash
   cargo build --workspace
   cargo test --workspace
   cargo clippy --workspace -- -D warnings
   cargo fmt --all -- --check
   ```
2. Check for platform-specific issues
3. Verify Rust toolchain version

## Next Steps

- See `plan.md` for detailed implementation plan
- See `contracts/` for configuration contracts
- See `data-model.md` for workspace entity definitions
- Reference `../gram-hs` for gram-hs implementation details
