# Gram Codec Examples

Comprehensive examples for using gram-codec across multiple platforms.

## 📦 Rust Examples

All Rust examples are now in the top-level `examples/` directory for clean separation of library code from usage examples.

### Pattern Core Examples

Located in `examples/pattern-core/`:

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

Located in `examples/gram-codec/`:

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

## 🌐 WASM Examples

### Browser (Web)
**Location:** `examples/gram-codec-wasm-web/`

**Build:**
```bash
cd crates/gram-codec
wasm-pack build --target web . -- --features wasm
```

**Run:**
```bash
# From project root
python3 -m http.server 8000
# Open http://localhost:8000/examples/gram-codec-wasm-web/
```

**Features:**
- ✨ Beautiful interactive UI
- 📥 Real-time parsing with visual feedback
- 📚 Quick example buttons
- 🔄 Round-trip testing
- 📊 Session statistics
- 🎨 Fully styled with gradients and animations

### Node.js
**Location:** `examples/gram-codec-wasm-node/`

**Build:**
```bash
cd crates/gram-codec
wasm-pack build --target nodejs . -- --features wasm
```

**Run:**
```bash
cd examples/gram-codec-wasm-node
npm install ../../crates/gram-codec/pkg
node index.js
```

**Features:**
- 8 comprehensive examples
- Batch validation
- Error handling
- Performance demonstration
- TypeScript support included

## 🐍 Python Examples

**Location:** `examples/gram-codec-python/`

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
python examples/gram-codec-python/demo.py
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

**Run:** `python examples/gram-codec-python/quickstart.py`

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
| **WASM (Web)** | Interactive browser | Open `examples/gram-codec-wasm-web/index.html` |
| **WASM (Node)** | Node.js CLI | `node examples/gram-codec-wasm-node/index.js` |
| **Python** | Interactive demo | `python examples/gram-codec-python/demo.py` |
| **Python** | Quick start | `python examples/gram-codec-python/quickstart.py` |

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
python ../../examples/gram-codec-python/quickstart.py
```

### 3-Minute Browser Example
```bash
cd pattern-rs/crates/gram-codec
wasm-pack build --target web . -- --features wasm
cd ../..
python3 -m http.server 8000
# Open http://localhost:8000/examples/gram-codec-wasm-web/
```

## 📖 Learning Path

**Beginner:**
1. Start with `pattern-core/comonad_usage.rs` to understand the foundation
2. Try `gram-codec/basic_usage.rs` or `gram-codec-python/quickstart.py`
3. Explore the interactive browser demo (`gram-codec-wasm-web/`)
4. Experiment with the Python REPL (`gram-codec-python/demo.py`)

**Intermediate:**
1. Study `gram-codec/advanced_usage.rs` for complex patterns
2. Dive deeper into comonad operations with `pattern-core/comonad_usage.rs`
3. Build a Node.js CLI tool with `gram-codec-wasm-node/`
4. Create a web app with `gram-codec-wasm-web/` as template

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
