# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

pattern-rs is a Rust port of the gram-hs reference implementation, providing a Pattern data structure and Gram notation codec. The library compiles for native Rust, WebAssembly, and Python (via PyO3 bindings). This is a faithful port emphasizing correctness and compatibility while adopting Rust-native idioms.

## Key Reference: gram-hs

The Haskell reference implementation is located at `../gram-hs` (relative to this repository root):
- **Source Code (Authoritative)**: `../gram-hs/libs/` - Haskell library implementations (source of truth for behavior)
- **Documentation**: `../gram-hs/docs/` - Up-to-date implementation documentation
- **Tests (Authoritative)**: `../gram-hs/libs/*/tests/` - Test suites showing expected behavior
- **Historical Notes**: `../gram-hs/specs/` - Historical context only (may be outdated)

When porting features, study the Haskell source in `../gram-hs/libs/` as authoritative, and port to idiomatic Rust (not literal translation).

## Core Concepts

**Pattern<V>**: A recursive, nested structure (s-expression-like) generic over value type `V`. The foundational data structure for representing hierarchical data interpretable as graphs.

**Subject**: A self-descriptive value type with identity, labels, and properties. Commonly used as `Pattern<Subject>` for object-graph replacements.

**Gram Notation**: A human-readable notation for patterns, handled by the gram-codec crate (bidirectional serialization/deserialization).

## Common Commands

### Building

```bash
# Build all workspace crates (native)
cargo build --workspace

# Build specific crate
cargo build -p pattern-core
cargo build -p gram-codec

# Build for WebAssembly
cargo build --workspace --target wasm32-unknown-unknown

# Release build
cargo build --workspace --release
```

### Testing

```bash
# Run all workspace tests
cargo test --workspace

# Test specific crate
cargo test -p pattern-core
cargo test -p gram-codec

# Run specific test
cargo test <test_name>

# Run tests with output
cargo test -- --nocapture
```

### Python Bindings (unified package: relateby)

End users install the single PyPI project **relateby**; one install provides both subpackages:

```bash
pip install relateby
```

Use only the public imports (no legacy `pattern_core` or `gram_codec`):

```python
import relateby.pattern
import relateby.gram
```

See `docs/python-usage.md` and `docs/release.md`. For **development** of the pattern-core or gram-codec crates (building from source, running crate-level tests):

```bash
# Build Python extension (requires Python 3.8+, uv)
cd crates/pattern-core
maturin develop --uv --features python

# Run Python tests (crate-level)
cd crates/pattern-core
pytest tests/python/

# Build unified wheel (from repo)
cd python/relateby
pip wheel . -w dist
```

### Code Quality

```bash
# Format all code
cargo fmt --all

# Lint all crates
cargo clippy --workspace

# Run all CI checks locally (fastest validation before push)
./scripts/ci-local.sh
```

### Validation Tools

```bash
# Validate gram notation syntax
gram-lint path/to/file.gram
gram-lint -e "your gram expression"
echo "gram expression" | gram-lint

# See parse tree
gram-lint -t -e "your expression"
```

## Architecture

### Workspace Structure

```
pattern-rs/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── pattern-core/       # Core Pattern<V> and Subject types
│   │   ├── src/
│   │   │   ├── lib.rs           # Main exports, Combinable trait
│   │   │   ├── pattern.rs       # Pattern<V> implementation
│   │   │   ├── pattern/         # Pattern modules (comonad, etc.)
│   │   │   ├── subject.rs       # Subject type
│   │   │   ├── python.rs        # PyO3 bindings (feature-gated)
│   │   │   └── test_utils/      # Testing utilities
│   │   ├── tests/               # Integration tests
│   │   ├── tests/python/        # Python binding tests
│   │   ├── pyproject.toml       # Python packaging (maturin)
│   │   └── Cargo.toml
│   └── gram-codec/         # Gram notation parser/serializer
│       ├── src/
│       ├── tests/
│       └── Cargo.toml
├── benches/                # Performance benchmarks (criterion)
├── examples/               # Usage examples
├── docs/                   # Documentation
└── specs/                  # Feature specifications
```

### Key Modules

**pattern-core/src/pattern.rs**: Core `Pattern<V>` type with:
- Constructors: `point()`, `pattern()`, `from_values()`
- Operations: `map()`, `fold()`, `para()`, `combine()`
- Queries: `any_value()`, `all_values()`, `filter()`, `matches()`
- Comonad ops: `extract()`, `extend()`, `duplicate()`
- Validation: `validate()`, `analyze_structure()`
- Traits: `Clone`, `Debug`, `PartialEq`, `Eq`, `Hash`, `Ord`

**pattern-core/src/subject.rs**: `Subject` type with identity (Symbol), labels (HashSet<String>), properties (HashMap<String, Value>). Implements `Combinable` (merges labels/properties, keeps first identity).

**pattern-core/src/python.rs**: PyO3 Python bindings providing `Pattern`, `PatternSubject`, `Subject`, `Value`, `ValidationRules`, `ValidationError`, `StructureAnalysis` classes with full Python API surface.

**gram-codec**: Parser (nom-based) and serializer for Gram notation. Bidirectional conversion between text and Pattern structures.

## Development Workflows

### Porting Features from gram-hs

1. Read the authoritative Haskell source in `../gram-hs/libs/`
2. Review up-to-date documentation in `../gram-hs/docs/`
3. Study test cases in `../gram-hs/libs/*/tests/` for expected behavior
4. Review historical notes in `../gram-hs/specs/XXX-feature-name/` for context (may be outdated)
5. Create feature specification using `/speckit.specify` (if available)
6. Port to idiomatic Rust (NOT literal translation)
7. Verify behavior matches reference tests

**Idiomatic Rust Principles**:
- Use direct methods instead of traits where Rust lacks HKTs (e.g., `map()` not Functor trait)
- Follow Rust naming conventions (e.g., `map` not `fmap`)
- Use ownership patterns idiomatically (consume `self` when appropriate)
- Preserve concepts and laws, not Haskell syntax

See `docs/porting-guide.md` for detailed guidance.

### Testing with gramref CLI

The `gramref` CLI tool (from gram-hs) generates test cases and validates outputs. Install from gram-hs repository.

```bash
# Generate test patterns
gramref generate --type suite --count 100

# Validate gram notation
gramref parse "your gram expression"
```

See `docs/gramref-cli-testing-guide.md` for detailed usage.

### Adding Python Bindings

When adding new Rust functionality that should be exposed to Python:

1. Implement the Rust feature in the core module
2. Add PyO3 wrapper in `src/python.rs` (feature-gated: `#[cfg(feature = "python")]`)
3. Add Python tests in `tests/python/`
4. Update type stubs in `pattern_core/__init__.pyi`
5. Document in `docs/python-usage.md`
6. Build and test: `cd crates/pattern-core && maturin develop --uv --features python && pytest tests/python/`

See `crates/pattern-core/PYTHON-DEVELOPMENT.md` for current status.

## Testing Infrastructure

The project uses comprehensive testing:
- **Property-based testing**: `proptest` for automated test case generation
- **Snapshot testing**: `insta` for regression detection
- **Benchmarks**: `criterion` for performance tracking
- **Equivalence checking**: Utilities for comparing pattern-rs and gram-hs implementations

See `docs/testing-infrastructure.md` for detailed documentation.

## WASM Compatibility

All public APIs avoid blocking I/O and file system access unless explicitly feature-flagged. Platform-specific code uses conditional compilation.

```bash
# Install WASM target
rustup target add wasm32-unknown-unknown

# Build for WASM
cargo build --target wasm32-unknown-unknown
```

## CI/CD

Before pushing, run all CI checks locally:

```bash
./scripts/ci-local.sh
```

This runs format check, clippy, build (native + WASM), and tests - identical to GitHub Actions.

For full workflow simulation with Docker:
```bash
# Install act: brew install act

# Run all jobs
act push

# Run specific job
act -j build
act -j test
```

See `.github/workflows/README.md` for details.

## Key Development Guidelines

1. **Read before modifying**: Always read existing files before suggesting changes
2. **Idiomatic Rust**: Write idiomatic Rust, not translated Haskell
3. **No over-engineering**: Only make directly requested changes; avoid "improvements"
4. **Validate Gram notation**: Use `gram-lint` for all gram notation in docs/tests
5. **Test thoroughly**: Property tests, unit tests, integration tests, equivalence checks
6. **WASM-first**: Ensure changes work on wasm32-unknown-unknown target
7. **Python bindings**: Update Python API surface when changing core functionality

## Important File Locations

- **Main exports**: `crates/pattern-core/src/lib.rs`
- **Pattern impl**: `crates/pattern-core/src/pattern.rs`
- **Subject impl**: `crates/pattern-core/src/subject.rs`
- **Python bindings**: `crates/pattern-core/src/python.rs`
- **Codec**: `crates/gram-codec/src/lib.rs`
- **Porting guide**: `docs/porting-guide.md`
- **Python usage**: `docs/python-usage.md`
- **Testing guide**: `docs/testing-infrastructure.md`
- **CI script**: `./scripts/ci-local.sh`

## Cursor Rules

The repository includes cursor rules in `.cursor/rules/specify-rules.mdc` that provide auto-generated guidelines from feature plans. These are dynamically maintained as features are developed.

Key cursor rule highlights:
- Use Rust 1.70.0+ (workspace MSRV), Edition 2021
- Python bindings require Python 3.8+ with type hints
- Use `cargo test` and `cargo clippy` for validation
- Validate gram notation with `gram-lint` tool
