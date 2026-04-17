# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

pattern-rs is a Rust port of the gram-hs reference implementation, providing a Pattern data structure and Gram notation codec. The library compiles for native Rust, WebAssembly, and Python (via PyO3 bindings). This is a faithful port emphasizing correctness and compatibility while adopting Rust-native idioms.

## Key Reference: gram-hs

The Haskell reference implementation is located at `../pattern-hs` (relative to this repository root):
- **Source Code (Authoritative)**: `../pattern-hs/libs/` - Haskell library implementations (source of truth for behavior)
- **Documentation**: `../pattern-hs/docs/` - Up-to-date implementation documentation
- **Tests (Authoritative)**: `../pattern-hs/libs/*/tests/` - Test suites showing expected behavior
- **Historical Notes**: `../pattern-hs/specs/` - Historical context only (may be outdated)

When porting features, study the Haskell source in `../pattern-hs/libs/` as authoritative, and port to idiomatic Rust (not literal translation).

## Core Concepts

**Pattern<V>**: A value paired with an ordered list of elements, each itself a `Pattern<V>`. This is the *decorated sequence* model: elements form the pattern concept; the value decorates it. An atomic pattern has no elements. Graph elements (nodes, relationships, walks) are one specialisation of this structure; see `docs/introduction.md` for the full model. Do not describe Pattern as a "tree", "hierarchical data", or frame graphs as its primary purpose — these lead to the wrong mental model.

**Subject**: A self-descriptive value type with identity, labels, and properties. Commonly used as `Pattern<Subject>` for property-graph data.

**Gram Notation**: A human-readable serialisation for patterns. The square-bracket form `["decoration" | element, element, ...]` is the general pattern notation; the parenthesis/arrow forms `(node)`, `(a)-[:rel]->(b)` are syntactic sugar for common graph-element shapes. Handled by the gram-codec crate (bidirectional serialisation/deserialisation).

## Common Commands

### Building

```bash
# Build all workspace crates (native)
cargo build --workspace

# Build specific crate
cargo build -p relateby-pattern
cargo build -p relateby-gram

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

### Python Bindings (published package: relateby-pattern)

End users install the single PyPI project **relateby-pattern**; one install provides both subpackages:

```bash
pip install relateby-pattern
```

Use only the public imports (no legacy `pattern_core` or `gram_codec`):

```python
import relateby.pattern
import relateby.gram
```

See `docs/python-usage.md` and `docs/release.md`. For **publishing** (PyPI or crates.io), see **`docs/release.md`** for prerequisites, tag format, workflow, and recovery. For Python package development, prefer `uv` with a project-local `.venv` and stay within the supported range `>=3.8,<3.14` (Python `3.13` is the safest local default while PyO3 support for `3.14` is pending).

```bash
# Create local virtual environment
cd python/packages/relateby
uv venv --python 3.13 .venv
source .venv/bin/activate
uv pip install '.[dev]'

# Build unified wheel (from the combined Python package root)
CARGO_TARGET_DIR=../../../target uv build --wheel --python 3.13 --out-dir dist
```

### Code Quality

```bash
# Validate GitHub workflows and related shell helpers
./scripts/check-workflows.sh

# Format all code
cargo fmt --all

# Lint all crates
cargo clippy --workspace

# Run all CI checks locally (fastest validation before push)
# Includes workflow validation when actionlint is installed.
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

```text
pattern-rs/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── pattern-core/       # Core Pattern<V> and Subject types
│   └── gram-codec/         # Gram notation parser/serializer
├── adapters/
│   └── wasm/
│       └── pattern-wasm/   # WASM adapter crate
├── typescript/
│   └── packages/
│       ├── pattern/
│       ├── graph/
│       └── gram/
├── python/
│   └── packages/
│       └── relateby/       # Combined Python distribution root
├── benches/                # Performance benchmarks (criterion)
├── examples/
│   ├── rust/
│   ├── python/
│   ├── typescript/
│   └── archive/
├── docs/
│   └── archive/
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

1. Read the authoritative Haskell source in `../pattern-hs/libs/`
2. Review up-to-date documentation in `../pattern-hs/docs/`
3. Study test cases in `../pattern-hs/libs/*/tests/` for expected behavior
4. Review historical notes in `../pattern-hs/specs/XXX-feature-name/` for context (may be outdated)
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
./scripts/check-workflows.sh
./scripts/ci-local.sh
```

`./scripts/check-workflows.sh` catches GitHub Actions YAML/expression issues early with `actionlint` and shell helper linting. `./scripts/ci-local.sh` runs the main project checks and now includes workflow validation automatically when `actionlint` is available.

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
- **Release / publishing**: `docs/release.md` (PyPI and crates.io)
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

## Active Technologies
- Rust 1.70.0 (MSRV), Edition 2021 + std (HashMap, Vec, HashSet) — no new external crates required (030-graph-classifier)
- N/A (in-memory data structures only) (030-graph-classifier)
- Rust 1.70.0 (workspace MSRV), Edition 2021 + `std` only — `HashMap`, `HashSet`, `VecDeque`, `BTreeMap`, `Rc`, `Arc` (no new external crates) (031-graph-query)
- N/A (in-memory only; all state is in `PatternGraph<V>` via `Rc`) (031-graph-query)
- Rust 1.70.0 (workspace MSRV), Edition 2021 + pattern-core (PatternGraph, GraphClassifier, GraphQuery, GraphValue, Subject, Symbol, reconcile), gram-codec (parse_gram) — no new external crates (035-standard-graph)
- Rust 1.70.0 (MSRV), Edition 2021; TypeScript (type definitions); Python 3.8+ (PyO3) + wasm-bindgen 0.2, js-sys 0.3, PyO3 (existing); pattern-core, gram-codec (workspace crates) (036-standardgraph-bindings)
- N/A (in-memory graph structures) (036-standardgraph-bindings)
- N/A (in-memory only) (039-native-bindings)
- Rust 1.70.0 (workspace MSRV), Edition 2021 + `relateby-pattern` (workspace), `relateby-gram` (workspace), `clap` v4 with derive, `serde`/`serde_json` (workspace), `thiserror` (workspace), `strsim` v0.11 (new) (041-pato-cli)
- Local filesystem — gram files read/written in-place. Atomic writes (temp-file + rename). No database. (041-pato-cli)
- Rust 1.70.0 (workspace MSRV), Edition 2021 + `tree-sitter` 0.25, `tree-sitter-gram` 0.3.4 (via path: `external/tree-sitter-gram/`), `pattern-core` (workspace), `nom` (retained for existing parser) (042-gram-cst-parser)
- N/A — in-memory only (042-gram-cst-parser)
- Rust 1.70.0 (workspace MSRV), Edition 2021 + `clap` v4 with derive (existing), no new dependencies (045-pato-help)
- Topic content embedded in binary via `include_str!` (compile-time static) (045-pato-help)
- TypeScript 5.x (all three changes), Markdown (documentation) + `vitest` (existing test runner), `@relateby/pattern` (existing), `effect >= 3.0.0` (peer) (046-ts-downstream-polish)

## Recent Changes
- 030-graph-classifier: Added Rust 1.70.0 (MSRV), Edition 2021 + std (HashMap, Vec, HashSet) — no new external crates required
