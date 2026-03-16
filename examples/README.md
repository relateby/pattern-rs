# Examples

This directory contains examples demonstrating how to use the pattern-rs libraries across different platforms and languages.

> 📚 **For detailed gram-codec examples**, see [`gram-codec-README.md`](gram-codec-README.md)

## Directory Structure

```
examples/
├── pattern-core/              # Pattern core library examples
│   ├── comonad_usage.rs       # Comonad operations
│   └── README.md              # Pattern core guide
├── gram-codec/                # Gram codec Rust examples
│   ├── basic_usage.rs         # Basic parsing & serialization
│   ├── advanced_usage.rs      # Advanced patterns
│   └── (see gram-codec-README.md)
├── gram-codec-python/         # Python bindings examples
│   ├── demo.py                # Interactive demo with REPL
│   ├── quickstart.py          # 5-minute introduction
│   ├── gram_codec.py          # Extended template
│   └── README.md              # Python usage guide
├── gram-codec-wasm-web/       # Browser WASM examples
│   ├── index.html             # Interactive web UI
│   ├── gram_codec_wasm.js     # Template
│   └── README.md              # Browser usage guide
├── gram-codec-wasm-node/      # Node.js WASM examples
│   ├── index.js               # CLI examples
│   ├── package.json           # NPM configuration
│   └── README.md              # Node.js usage guide
├── wasm-js/                   # Legacy WASM example
│   └── ...
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
python examples/gram-codec-python/demo.py
python examples/gram-codec-python/quickstart.py
python examples/pattern-core-python/operations.py
```

### Gram Codec (WASM - Browser)

```bash
# Build WASM module
cd crates/gram-codec
wasm-pack build --target web . -- --features wasm

# Start server and open browser
cd ../..
python3 -m http.server 8000
# Open http://localhost:8000/examples/gram-codec-wasm-web/
```

### Gram Codec (WASM - Node.js)

```bash
# Build WASM module
cd crates/gram-codec
wasm-pack build --target nodejs . -- --features wasm

# Install and run
cd ../../examples/gram-codec-wasm-node
npm install ../../crates/gram-codec/pkg
node index.js
```

## Example Categories

### 🦀 Rust Examples

All Rust examples have been moved to the top-level `examples/` directory for clean separation of library code from usage examples.

- **pattern-core/**: Foundation library examples
  - Comonad operations and tree analysis
- **gram-codec/**: Codec library examples
  - Parsing and serialization
  - Round-trip correctness
  - Complex patterns

### 🐍 Python Examples

Python bindings via PyO3:
- Interactive REPL for testing gram notation
- Batch validation examples
- File processing patterns
- Error handling demonstrations

### 🌐 WASM Examples

WebAssembly bindings for multi-platform use:

**Browser** (`gram-codec-wasm-web/`):
- Beautiful interactive UI
- Real-time parsing and validation
- One-click examples
- Session statistics

**Node.js** (`gram-codec-wasm-node/`):
- CLI examples
- Batch processing
- Stream processing patterns
- Express.js integration

### 📜 Legacy Examples

- **wasm-js/**: Original WASM example (kept for reference)

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
- Source is in `examples/<crate>/`

### Python
- Install the combined package: `pip install relateby-pattern` (or from TestPyPI for pre-release)
- Examples use `relateby.pattern` and `relateby.gram`; see `docs/python-usage.md`
- To build from source: see `docs/release.md`

### WASM
- Requires `wasm-pack` for building
- Browser examples need an HTTP server (not `file://`)
- Node.js examples need npm package installation

## Getting Help

- **Pattern Core**: See `specs/018-comonad-instance/`
- **Gram Codec**: See `specs/019-gram-codec/`
- **API Docs**: Run `cargo doc --open`
- **Issues**: https://github.com/relateby/pattern-rs/issues

## License

All examples are licensed under Apache-2.0, same as the pattern-rs libraries.
