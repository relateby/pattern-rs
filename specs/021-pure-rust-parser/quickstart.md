# Quickstart: Pure Rust Gram Parser

**Feature**: 021-pure-rust-parser  
**Date**: 2026-01-09  
**Audience**: Developers using gram-codec  
**Purpose**: Migration guide and quick reference

## What Changed

### Before (tree-sitter-gram)

```toml
# Cargo.toml
[dependencies]
gram-codec = "0.1"
tree-sitter = "0.25"
tree-sitter-gram = "0.2"
```

**Build Requirements**:
- C compiler (gcc, clang)
- tree-sitter CLI
- Emscripten (for WASM)
- LLVM (for WASM on macOS)

**Build Command** (WASM):
```bash
# Complex setup required
export CC_wasm32_unknown_unknown=/opt/homebrew/opt/llvm/bin/clang
export CFLAGS_wasm32_unknown_unknown="-I$EMSDK/upstream/emscripten/cache/sysroot/include"
wasm-pack build --target web crates/gram-codec -- --features wasm
```

---

### After (Pure Rust nom)

```toml
# Cargo.toml
[dependencies]
gram-codec = "0.2"  # Pure Rust, no C dependencies
```

**Build Requirements**:
- Rust toolchain only (no C compiler, no emscripten)

**Build Command** (WASM):
```bash
# Just works™
wasm-pack build --target web crates/gram-codec -- --features wasm
```

---

## Migration Checklist

### For Library Users

- [ ] **No code changes required** - Public API is identical
- [ ] **Update Cargo.toml** - Bump gram-codec version to 0.2+
- [ ] **Remove build scripts** - Delete any tree-sitter build workarounds
- [ ] **Rebuild WASM** - Use simplified build command
- [ ] **Test** - Verify existing tests pass

### For Contributors/Developers

- [ ] **Update local environment** - Remove emscripten if only used for gram-codec
- [ ] **Update CI/CD** - Simplify build steps, remove C compiler setup
- [ ] **Update documentation** - Remove build complexity warnings
- [ ] **Run corpus tests** - Verify 100% conformance with `cargo test --package gram-codec corpus`

---

## Quick Reference

### Parsing

```rust
use gram_codec::parse_gram;

// Parse gram notation into patterns
let patterns = parse_gram("(hello)").unwrap();

// Parse with error handling
match parse_gram("(a)-->(b)") {
    Ok(patterns) => {
        println!("Parsed {} patterns", patterns.len());
    }
    Err(e) => {
        eprintln!("Parse error: {}", e);
        if let Some(loc) = e.location() {
            eprintln!("  at {}:{}", loc.line, loc.column);
        }
    }
}

// Validate syntax without constructing patterns
use gram_codec::validate_gram;
if validate_gram("(hello)").is_ok() {
    println!("Valid gram notation");
}
```

### Serialization

```rust
use gram_codec::{serialize_patterns, serialize_pattern};

// Serialize patterns to gram notation
let pattern = Pattern::from_subject(Subject {
    identifier: Some("hello".to_string()),
    ..Default::default()
});

let gram_text = serialize_pattern(&pattern).unwrap();
assert_eq!(gram_text, "(hello)");

// Serialize multiple patterns
let patterns = parse_gram("(a)\n(b)").unwrap();
let gram_text = serialize_patterns(&patterns).unwrap();
```

### Round-Trip (Semantic Equivalence)

```rust
// CORRECT: gram -> pattern -> gram -> pattern
// Tests semantic preservation of Pattern structures

let original = "(a:Label {key: \"value\"})";

// First parse
let patterns1 = parse_gram(original).unwrap();

// Serialize
let serialized = serialize_patterns(&patterns1).unwrap();

// Second parse
let patterns2 = parse_gram(&serialized).unwrap();

// Check semantic equivalence (Pattern structures match)
assert_eq!(patterns1, patterns2);

// Note: serialized gram notation may differ in formatting
// Original:  "(a:Label {key: \"value\"})"
// Serialized: "(a:Label {key: \"value\"})"  // Normalized spacing
// But Pattern structures are identical
```

---

## WASM Usage

### Building for Web

```bash
cd crates/gram-codec
wasm-pack build --target web . -- --features wasm
```

**Output**: `pkg/` directory with:
- `gram_codec_bg.wasm` - WebAssembly binary
- `gram_codec.js` - JavaScript bindings
- `gram_codec.d.ts` - TypeScript definitions

### Browser Usage

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Gram Parser Demo</title>
</head>
<body>
    <script type="module">
        import init, { parse_gram } from './pkg/gram_codec.js';

        async function main() {
            await init();
            
            try {
                const result = parse_gram("(hello)");
                console.log("Parsed:", result);
            } catch (error) {
                console.error("Error:", error.message);
                console.error("  at", error.line + ":" + error.column);
            }
        }

        main();
    </script>
</body>
</html>
```

### Node.js Usage

```bash
# Build for Node.js
wasm-pack build --target nodejs crates/gram-codec -- --features wasm
```

```javascript
// example.js
const { parse_gram, serialize_patterns } = require('./pkg/gram_codec.js');

try {
    const patterns = parse_gram("(a)-->(b)");
    console.log("Parsed:", patterns);
    
    const serialized = serialize_patterns(patterns);
    console.log("Serialized:", serialized);
} catch (error) {
    console.error("Error:", error.message);
}
```

---

## Python Usage

### Building Python Package

```bash
# Install maturin (build tool for PyO3)
pip install maturin

# Build Python wheel
cd crates/gram-codec
maturin build --release --features python

# Install locally
pip install target/wheels/gram_codec-*.whl
```

### Python API

```python
import gram_codec

# Parse gram notation
try:
    patterns = gram_codec.parse_gram("(hello)")
    print(f"Parsed {len(patterns)} patterns")
except ValueError as e:
    print(f"Parse error: {e}")

# Serialize patterns
gram_text = gram_codec.serialize_patterns(patterns)
print(f"Serialized: {gram_text}")

# Round-trip
re_parsed = gram_codec.parse_gram(gram_text)
assert patterns == re_parsed
```

---

## Testing

### Running Tests

```bash
# All tests
cargo test --package gram-codec

# Unit tests only
cargo test --package gram-codec --lib

# Corpus conformance tests
cargo test --package gram-codec corpus

# Integration tests
cargo test --package gram-codec --test '*'
```

### Benchmarks

```bash
# Run performance benchmarks
cargo bench --package gram-codec

# Compare to baseline (if available)
cargo bench --package gram-codec -- --baseline before_nom
```

---

## Performance

### Expected Performance

| Operation | Time | Comparison to tree-sitter |
|-----------|------|---------------------------|
| Simple node parse | <10μs | ~95% (comparable) |
| 1000 patterns | <120ms | ~90-95% (within 20%) |
| Round-trip | +10% | Minimal overhead |

### Optimization Tips

**1. Batch Parsing**:
```rust
// Efficient: Parse multiple patterns in one call
let patterns = parse_gram("(a)\n(b)\n(c)").unwrap();

// Less efficient: Parse one at a time
let a = parse_gram("(a)").unwrap();
let b = parse_gram("(b)").unwrap();
let c = parse_gram("(c)").unwrap();
```

**2. Validation Before Parsing**:
```rust
// Validate first to avoid expensive parse on invalid input
if validate_gram(input).is_ok() {
    let patterns = parse_gram(input).unwrap();  // Won't fail
}
```

**3. Reuse Allocations** (advanced):
```rust
// If parsing many inputs, consider String pooling
let mut buffer = String::new();
for input in inputs {
    buffer.clear();
    buffer.push_str(input);
    let patterns = parse_gram(&buffer)?;
    // Process patterns
}
```

---

## Troubleshooting

### Common Issues

**1. "expected ')', found 'x'"**

**Cause**: Syntax error in gram notation.

**Fix**: Check for:
- Unmatched delimiters: `(`, `)`, `[`, `]`, `{`, `}`
- Missing commas in lists
- Invalid characters in identifiers

**Example**:
```rust
// Error: expected ')', found 'b'
parse_gram("(a b)");  // ❌ Space not allowed in identifier

// Fixed: Use property record or separate patterns
parse_gram("(a)");    // ✅
parse_gram("(a) (b)"); // ✅
```

---

**2. "Unexpected input at..."**

**Cause**: Extra text after valid pattern.

**Fix**: Check for:
- Multiple patterns without proper separation
- Trailing characters

**Example**:
```rust
// Error: Unexpected input
parse_gram("(a)(b)");  // ❌ No separator between patterns

// Fixed: Add whitespace or newline
parse_gram("(a) (b)"); // ✅
parse_gram("(a)\n(b)"); // ✅
```

---

**3. "Invalid integer at..."**

**Cause**: Malformed number in property value.

**Fix**: Check number format:
- Integers: `42`, `-10`
- Decimals: `3.14`, `-0.5`

**Example**:
```rust
// Error: Invalid integer
parse_gram("(a {count: 12x})");  // ❌ 'x' not allowed

// Fixed: Use valid number
parse_gram("(a {count: 12})");   // ✅
```

---

**4. WASM build fails (permission errors)**

**Cause**: Leftover from tree-sitter C compilation setup.

**Fix**:
```bash
# Clean build artifacts
cargo clean
rm -rf target/

# Remove old WASM artifacts
rm -rf pkg/

# Rebuild
wasm-pack build --target web crates/gram-codec -- --features wasm
```

---

**5. Python import error**

**Cause**: Module not built or not in Python path.

**Fix**:
```bash
# Ensure maturin build succeeded
maturin build --release --features python

# Install the wheel
pip install target/wheels/gram_codec-*.whl --force-reinstall

# Verify installation
python -c "import gram_codec; print(gram_codec.__version__)"
```

---

## API Reference

### parse_gram

```rust
pub fn parse_gram(input: &str) -> Result<Vec<Pattern>, ParseError>
```

Parse gram notation text into Pattern structures.

**Returns**:
- `Ok(Vec<Pattern>)` - Parsed patterns
- `Err(ParseError)` - Syntax error with location

**Examples**:
```rust
parse_gram("(hello)");                   // Node
parse_gram("(a)-->(b)");                 // Relationship
parse_gram("[team | (alice), (bob)]");  // Subject pattern
parse_gram("");                          // Empty (Ok)
```

---

### serialize_patterns

```rust
pub fn serialize_patterns(patterns: &[Pattern]) -> Result<String, SerializeError>
```

Serialize Pattern structures to gram notation.

**Returns**:
- `Ok(String)` - Gram notation text
- `Err(SerializeError)` - Pattern cannot be represented

**Examples**:
```rust
let pattern = Pattern::from_subject(Subject {
    identifier: Some("hello".to_string()),
    ..Default::default()
});
serialize_patterns(&[pattern]);  // "(hello)"
```

---

### validate_gram

```rust
pub fn validate_gram(input: &str) -> Result<(), ParseError>
```

Validate gram notation syntax without constructing patterns.

**Returns**:
- `Ok(())` - Valid syntax
- `Err(ParseError)` - Syntax error

**Examples**:
```rust
validate_gram("(hello)");     // Ok
validate_gram("(unclosed");   // Err
```

---

## Next Steps

1. **Read the spec**: [spec.md](./spec.md) for feature details
2. **Review contracts**: [contracts/](./contracts/) for API contracts
3. **Run examples**: `examples/gram-codec/` for usage demonstrations
4. **Run tests**: `cargo test --package gram-codec` to verify implementation
5. **Benchmark**: `cargo bench --package gram-codec` to check performance

---

## FAQ

**Q: Do I need to change my code?**  
A: No, if you use the public API (`parse_gram`, `serialize_patterns`), no changes are needed.

**Q: Will performance be slower?**  
A: Slightly (within 20% of tree-sitter baseline), but the build simplicity is worth it.

**Q: Can I still use tree-sitter-gram for validation?**  
A: Yes, use the `gram-lint` CLI tool from tree-sitter-gram for independent validation.

**Q: How do I report a bug?**  
A: File an issue with: (1) input gram notation, (2) expected behavior, (3) actual behavior.

**Q: What if I find a conformance issue with tree-sitter-gram?**  
A: File an issue with the failing corpus test case. We prioritize 100% conformance.

**Q: Can I use both parsers?**  
A: No, this is a hard switch. The tree-sitter-gram dependency is removed entirely.

---

## Migration Timeline

| Phase | Status | Description |
|-------|--------|-------------|
| ✅ Spec | Complete | Feature specification finalized |
| 🚧 Implementation | In Progress | nom parser implementation |
| ⏳ Testing | Planned | Corpus conformance validation |
| ⏳ Documentation | Planned | Update examples and README |
| ⏳ Release | Planned | Publish gram-codec 0.2 |

**Estimated Completion**: TBD based on implementation progress
