# gram-rs

Rust port of [gram-hs](https://github.com/gram-data/gram-hs) pattern data structure and graph views.

This library provides a faithful port of the gram-hs reference implementation, emphasizing correctness and compatibility while adopting Rust-native idioms. The library is designed as a shared library that compiles for native Rust, WebAssembly, and other target environments.

## Reference Implementation

This project is a port of the [gram-hs](https://github.com/gram-data/gram-hs) reference implementation. The reference implementation is available locally at `../gram-hs` (relative to this repository root).

**Key Reference Locations**:
- **Feature Specifications**: `../gram-hs/specs/` - Incremental feature development documentation
- **Source Code**: `../gram-hs/libs/` - Haskell library implementations
- **Tests**: `../gram-hs/libs/*/tests/` - Test suites for behavioral equivalence verification
- **Online Repository**: https://github.com/gram-data/gram-hs

All functionality is designed to faithfully replicate the behavior of the Haskell implementation. When porting features, developers should reference the corresponding feature specification in `../gram-hs/specs/` and study the Haskell implementation to ensure correctness.

## Quick Start

### Prerequisites

- **Rust**: 1.70.0 or later (check with `rustc --version`)
- **Cargo**: Included with Rust (check with `cargo --version`)
- **WASM Target** (for WebAssembly compilation): Install with `rustup target add wasm32-unknown-unknown`

### Setup

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd gram-rs
   ```

2. **Build the library**:
   ```bash
   cargo build
   ```

3. **Run tests**:
   ```bash
   cargo test
   ```

4. **Build for WASM** (after installing WASM target):
   ```bash
   cargo build --target wasm32-unknown-unknown
   ```

## Workspace Structure

This project is organized as a Cargo workspace with multiple crates:

```
gram-rs/
├── Cargo.toml              # Workspace root configuration
├── crates/
│   ├── pattern-core/        # Core pattern data structures
│   ├── pattern-ops/          # Pattern operations and algorithms
│   ├── gram-codec/          # Gram notation serialization/deserialization
│   ├── pattern-store/       # Optimized storage (placeholder)
│   └── pattern-wasm/        # WASM bindings (placeholder)
└── .github/workflows/       # CI/CD configuration
```

## Building

### Build Commands

```bash
# Build all workspace crates (native target)
cargo build --workspace

# Build a specific crate
cargo build -p pattern-core

# Build for WebAssembly
cargo build --workspace --target wasm32-unknown-unknown

# Run all workspace tests
cargo test --workspace

# Test a specific crate
cargo test -p pattern-core

# Check all crates
cargo check --workspace

# Format all crates
cargo fmt --all

# Lint all crates
cargo clippy --workspace
```

### WASM Compatibility

The library is designed to be WASM-compatible. All public APIs avoid blocking I/O and file system access unless explicitly feature-flagged. Platform-specific code uses conditional compilation with the `wasm` feature flag.

## Testing Infrastructure

The project includes comprehensive testing infrastructure:

- **Property-Based Testing**: Using `proptest` for automated test case generation
- **Equivalence Checking**: Utilities for comparing gram-rs and gram-hs implementations
- **Snapshot Testing**: Using `insta` for regression detection
- **Benchmarks**: Using `criterion` for performance tracking
- **Test Helpers**: Utilities for pattern comparison and validation

See [docs/testing-infrastructure.md](docs/testing-infrastructure.md) for detailed documentation and [specs/003-test-infrastructure/quickstart.md](specs/003-test-infrastructure/quickstart.md) for usage examples.

For using the `gram-hs` CLI tool for testing and equivalence checking, see [gram-hs CLI Testing Guide](docs/gram-hs-cli-testing-guide.md).

## Examples

See the [examples/](examples/) directory for usage examples:

- **WASM/JavaScript**: `examples/wasm-js/` - Demonstrates WebAssembly compilation and JavaScript integration

## Troubleshooting

### Issue: "could not find `Cargo.toml`"

**Solution**: Ensure you're in the repository root directory

### Issue: "error: failed to run `rustc`"

**Solution**: 
- Check Rust installation: `rustc --version`
- Update Rust: `rustup update stable`
- Ensure Rust 1.70.0 or later is installed

### Issue: "error: target `wasm32-unknown-unknown` not found"

**Solution**: Install WASM target: `rustup target add wasm32-unknown-unknown`

### Issue: "error: failed to resolve: use of undeclared crate"

**Solution**: 
- Run `cargo build` to fetch dependencies
- Check `Cargo.toml` for correct dependency declarations
- Ensure network access is available for downloading dependencies

### Issue: Clippy warnings

**Solution**: 
- Review warnings and fix legitimate issues
- Suppress false positives in code if needed
- Update `clippy.toml` if project-wide configuration needed

### Issue: Formatting errors

**Solution**: 
- Run `cargo fmt` to auto-format code
- Ensure `rustfmt.toml` is properly configured

## Porting Features

When porting features from gram-hs, see [PORTING_GUIDE.md](PORTING_GUIDE.md) for detailed instructions. The guide covers:

- How to reference the local gram-hs implementation at `../gram-hs`
- Systematic workflow for porting features
- Haskell → Rust translation patterns
- Verification and testing strategies

**Quick Start for Porting**:
1. Review the feature specification in `../gram-hs/specs/XXX-feature-name/`
2. Study the Haskell implementation in `../gram-hs/libs/`
3. Create a new feature specification using `/speckit.specify`
4. Follow the porting guide for implementation

## License

BSD-3-Clause (see [LICENSE](LICENSE) file)

