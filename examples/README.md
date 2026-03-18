# Examples

This directory contains examples demonstrating how to use the pattern-rs libraries across different platforms and languages.

> 📚 **For detailed gram-codec examples**, see [`gram-codec-README.md`](gram-codec-README.md)
> Archived and superseded examples live under [`examples/archive/`](archive/).

## Directory Structure

```
examples/
├── rust/
│   ├── pattern-core/          # Pattern core Rust examples
│   └── gram-codec/            # Gram codec Rust examples
├── python/
│   ├── pattern/               # relateby.pattern examples
│   └── gram/                  # relateby.gram examples
├── typescript/
│   └── graph/                 # @relateby/pattern + @relateby/graph example
├── archive/                   # Historical and superseded examples
│   ├── gram-codec-wasm-web/
│   ├── gram-codec-wasm-node/
│   ├── pattern-core-wasm/
│   └── wasm-js/
├── gram-codec-README.md       # Master guide for all gram-codec examples
└── README.md                  # This file
```

## Quick Start

### Pattern Core (Comonad Operations)

```bash
# Run the comonad example
cargo run --package pattern-core --example comonad_usage
```

Demonstrates:
- Extract and extend operations
- Tree traversal and analysis
- Context-aware transformations
- Hierarchical computations

### Gram Codec (Rust)

```bash
# Basic usage (parsing, serialization, round-trip)
cargo run --package gram-codec --example basic_usage

# Advanced usage (complex patterns, Unicode, annotations)
cargo run --package gram-codec --example advanced_usage
```

### Python (`relateby-pattern`)

One package provides both `relateby.pattern` and `relateby.gram`:

```bash
# Install from PyPI
pip install relateby-pattern

# Or from TestPyPI (pre-release)
pip install --index-url https://test.pypi.org/simple/ relateby-pattern

# Run examples
python examples/python/gram/demo.py
python examples/python/gram/quickstart.py
python examples/python/pattern/operations.py
```

### TypeScript (`@relateby/pattern` + `@relateby/graph`)

```bash
npm install
npm run build --workspace=@relateby/pattern
npm run build --workspace=@relateby/graph

cd examples/typescript/graph
npm install
npm start
```

### Historical WASM examples (archived)

Current WASM and TypeScript usage is documented in `docs/wasm-usage.md`.

Historical browser and Node.js samples remain available under:

- `examples/archive/gram-codec-wasm-web/`
- `examples/archive/gram-codec-wasm-node/`
- `examples/archive/pattern-core-wasm/`
- `examples/archive/wasm-js/`

## Example Categories

### 🦀 Rust Examples

- **rust/pattern-core/**: Foundation library examples
  - Comonad operations and tree analysis
- **rust/gram-codec/**: Codec library examples
  - Parsing and serialization
  - Round-trip correctness
  - Complex patterns

### 🐍 Python Examples

Python bindings via PyO3:
- Interactive REPL for testing gram notation
- Batch validation examples
- File processing patterns
- Error handling demonstrations

Active Python example roots:

- `examples/python/pattern/`
- `examples/python/gram/`

### 📦 TypeScript Examples

The active TypeScript example root is:

- `examples/typescript/graph/`

### 🌐 WASM Examples

The historical browser and Node.js WASM examples are preserved in `examples/archive/`.
Use `docs/wasm-usage.md` for the current package-oriented guidance.

### 📜 Legacy Examples

- `examples/archive/wasm-js/`: original WASM example kept for reference
- `examples/archive/pattern-core-wasm/`: earlier native TypeScript/WASM bridge examples
- `examples/archive/gram-codec-wasm-web/`: earlier browser WASM examples
- `examples/archive/gram-codec-wasm-node/`: earlier Node.js WASM examples

## Documentation

Each example directory includes:
- **README.md**: Setup, build, and usage instructions
- **API examples**: Practical code samples
- **Common patterns**: Real-world usage scenarios
- **Troubleshooting**: Solutions to common issues

## Contributing

To add a new example:

1. Create the example in the appropriate directory
2. Add comprehensive inline documentation
3. Create or update the directory's README.md
4. Test thoroughly across platforms
5. Update this README with a link
6. Submit a PR!

## Platform-Specific Notes

### Rust
- Examples use the workspace `Cargo.toml` for dependencies
- Run from workspace root: `cargo run --example <name>`
- Source is in `examples/rust/<crate>/`

### Python
- Install the combined package: `pip install relateby-pattern` (or from TestPyPI for pre-release)
- Examples use `relateby.pattern` and `relateby.gram`; see `docs/python-usage.md`
- To build from source: see `docs/release.md`

### WASM
- Current TypeScript/WASM guidance is in `docs/wasm-usage.md`
- Archived browser examples need an HTTP server (not `file://`)
- Archived Node.js examples need npm package installation

## Getting Help

- **Pattern Core**: See `specs/018-comonad-instance/`
- **Gram Codec**: See `specs/019-gram-codec/`
- **API Docs**: Run `cargo doc --open`
- **Issues**: https://github.com/relateby/pattern-rs/issues

## License

All examples are licensed under Apache-2.0, same as the pattern-rs libraries.
