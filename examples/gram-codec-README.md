# Gram Codec Examples

Comprehensive examples for using gram-codec across multiple platforms.

## 📦 Rust Examples

All Rust examples are now in the top-level `examples/` directory for clean separation of library code from usage examples.

### Pattern Core Examples

Located in `examples/rust/pattern-core/`:

#### `comonad_usage.rs`
**Run:** `cargo run --package pattern-core --example comonad_usage`

Comprehensive comonad demonstrations:
- Extract and extend operations
- Depth, size, and path calculations
- Tree traversal (collect_all, collect_leaves, find)
- Context-aware transformations
- Metadata annotations
- Advanced tree analysis

### Gram Codec Examples

Located in `examples/rust/gram-codec/`:

#### `basic_usage.rs`
**Run:** `cargo run --package gram-codec --example basic_usage`

10 examples covering:
- Parsing simple nodes, labels, properties
- Parsing relationships and subject patterns
- Serializing patterns
- Round-trip testing
- Error handling

#### `advanced_usage.rs`
**Run:** `cargo run --package gram-codec --example advanced_usage`

10 advanced examples:
- Complex relationships with properties
- Nested subject patterns
- Path patterns (chained relationships)
- Different property value types
- Annotations
- Multiple labels
- Unicode support
- Programmatic pattern building

## 🌐 Historical WASM Examples

The older browser and Node.js gram-codec WASM samples are preserved for reference in `examples/archive/`, but they are no longer the active example surface for this feature.

Archived locations:

- `examples/archive/gram-codec-wasm-web/`
- `examples/archive/gram-codec-wasm-node/`

For the current package-oriented WASM guidance, use `docs/wasm-usage.md`.

## 🐍 Python Examples

**Location:** `examples/python/gram/`

### Interactive Demo
**File:** `demo.py`

**Build:**
```bash
pip install maturin
cd crates/gram-codec
maturin develop --features python
```

**Run:**
```bash
python examples/python/gram/demo.py
```

**Features:**
- 6 comprehensive example sections
- Interactive REPL mode
- Batch validation
- Complex pattern handling
- Beautiful formatted output
- Error demonstrations

### Quick Start
**File:** `quickstart.py`

**Run:** `python examples/python/gram/quickstart.py`

5-minute introduction covering:
- Parse and validate
- Round-trip testing
- Complex patterns
- Error handling

### Template
**File:** `gram_codec.py`

Extended template with 10 example functions:
- Parsing various forms
- Validation
- Round-trip tests
- Multiple patterns
- Complex structures
- Annotations
- Unicode
- Properties
- Batch validation

## 📚 Documentation

Each example directory includes:

- **README.md** - Setup, building, and usage instructions
- **Complete API reference** - All functions with examples
- **Common patterns** - Real-world usage scenarios
- **Troubleshooting** - Solutions to common issues
- **Performance tips** - Optimization guidance

## 🎯 Quick Reference

| Platform | Example | Run Command |
|----------|---------|-------------|
| **Rust (pattern-core)** | Comonad operations | `cargo run --package pattern-core --example comonad_usage` |
| **Rust (gram-codec)** | Basic usage | `cargo run --package gram-codec --example basic_usage` |
| **Rust (gram-codec)** | Advanced usage | `cargo run --package gram-codec --example advanced_usage` |
| **WASM (Archived Web)** | Historical browser sample | `examples/archive/gram-codec-wasm-web/` |
| **WASM (Archived Node)** | Historical Node.js sample | `examples/archive/gram-codec-wasm-node/` |
| **Python** | Interactive demo | `python examples/python/gram/demo.py` |
| **Python** | Quick start | `python examples/python/gram/quickstart.py` |

## 🔥 Try It Now

### 1-Minute Rust Examples
```bash
git clone https://github.com/relateby/pattern-rs
cd pattern-rs

# Pattern core - comonad operations
cargo run --package pattern-core --example comonad_usage

# Gram codec - parsing and serialization
cargo run --package gram-codec --example basic_usage
```

### 2-Minute Python Example
```bash
cd pattern-rs/crates/gram-codec
pip install maturin
maturin develop --features python
python ../../examples/python/gram/quickstart.py
```

### 3-Minute WASM Orientation
```bash
open docs/wasm-usage.md
```

## 📖 Learning Path

**Beginner:**
1. Start with `pattern-core/comonad_usage.rs` to understand the foundation
2. Try `gram-codec/basic_usage.rs` or `gram-codec-python/quickstart.py`
3. Review `docs/wasm-usage.md` for the current WASM/package entrypoint
4. Experiment with the Python REPL (`gram-codec-python/demo.py`)

**Intermediate:**
1. Study `gram-codec/advanced_usage.rs` for complex patterns
2. Dive deeper into comonad operations with `pattern-core/comonad_usage.rs`
3. Inspect `examples/archive/gram-codec-wasm-node/` for historical Node.js usage patterns
4. Inspect `examples/archive/gram-codec-wasm-web/` for historical browser usage patterns

**Advanced:**
1. Read the source code in `crates/pattern-core/src/` and `crates/gram-codec/src/`
2. Study the benchmarks in `crates/gram-codec/benches/codec_benchmarks.rs`
3. Review the specifications in `specs/018-comonad-instance/` and `specs/019-gram-codec/`
4. Explore the test suites for comprehensive usage patterns

## 🤝 Contributing Examples

To add a new example:

1. Create the example file in the appropriate directory
2. Add documentation (inline comments + README)
3. Test it thoroughly
4. Update this README
5. Submit a PR!

## 📝 License

All examples are licensed under Apache-2.0, same as the main gram-codec library.
