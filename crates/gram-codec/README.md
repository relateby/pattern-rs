# Gram Codec

Bidirectional codec between [Gram notation](https://github.com/gram-data/tree-sitter-gram) (human-readable text format) and Pattern data structures.

[![CI Status](https://github.com/relateby/pattern-rs/workflows/CI/badge.svg)](https://github.com/relateby/pattern-rs/actions)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Features

- **Full Grammar Support**: Parses all Gram syntax forms (nodes, relationships, subject patterns, annotations)
- **Round-Trip Correctness**: Parse → serialize → parse produces structurally equivalent patterns
- **Error Recovery**: Reports all syntax errors, not just the first
- **Value Types**: Supports strings, integers, decimals, booleans, arrays, ranges, tagged strings
- **Unicode Support**: Full Unicode identifier and property value support
- **Comprehensive Testing**: 139+ tests covering all syntax forms and edge cases

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gram-codec = { path = "../gram-codec" }
pattern-core = { path = "../pattern-core" }
```

## Quick Start

### Parsing Gram Notation

```rust
use gram_codec::parse_gram;

// Parse a simple node
let patterns = parse_gram("(hello)")?;
println!("Identifier: {}", patterns[0].value.identity.0);

// Parse a relationship
let patterns = parse_gram("(alice)-[:KNOWS]->(bob)")?;
println!("Relationship with {} elements", patterns[0].elements.len());

// Parse a subject pattern
let patterns = parse_gram("[team | (alice), (bob)]")?;
println!("Team has {} members", patterns[0].elements.len());
```

### Serializing Patterns

```rust
use gram_codec::{to_gram_pattern, to_gram};
use pattern_core::{Pattern, Subject, Symbol};
use std::collections::{HashMap, HashSet};

// Create a pattern
let subject = Subject {
    identity: Symbol("hello".to_string()),
    labels: HashSet::new(),
    properties: HashMap::new(),
};
let pattern = Pattern::point(subject);

// Serialize a single pattern
let gram_text = to_gram_pattern(&pattern)?;
println!("{}", gram_text); // Output: (hello)

// Serialize multiple patterns
let multiple = to_gram(&[pattern.clone(), pattern])?;
println!("{}", multiple); // Output: (hello)\n(hello)
```

### Round-Trip Correctness

```rust
use gram_codec::{parse_gram, to_gram_pattern};

let original = "(alice:Person {name: \"Alice\"})";

// Parse
let parsed = parse_gram(original)?;

// Serialize
let serialized = to_gram_pattern(&parsed[0])?;

// Re-parse
let reparsed = parse_gram(&serialized)?;

// Verify structural equivalence
assert_eq!(parsed[0].value.identity, reparsed[0].value.identity);
```

## Supported Gram Syntax

### Node Patterns (0 elements)

```gram
()                              // Empty node
(hello)                         // Node with identifier
(a:Person)                      // Node with label
(a:Person {name: "Alice"})      // Node with properties
```

### Relationship Patterns (2 elements)

The grammar accepts **multiple visual arrow styles** that normalize to **4 semantic arrow kinds**:

#### Right Arrow (`right_arrow`) - Directed left-to-right
```gram
(a)-->(b)                       // Single-stroke (canonical)
(a)==>(b)                       // Double-stroke
(a)~~>(b)                       // Squiggle
(a)-[:KNOWS]->(b)               // With label
(a)-[:KNOWS {since: 2020}]->(b) // With properties
```

#### Left Arrow (`left_arrow`) - Directed right-to-left (elements reversed)
```gram
(a)<--(b)                       // Single-stroke (canonical)
(a)<==(b)                       // Double-stroke
(a)<~~(b)                       // Squiggle
// Note: (a)<--(b) stores elements as [b, a] (reversed!)
```

#### Bidirectional Arrow (`bidirectional_arrow`) - Mutual connection
```gram
(a)<-->(b)                      // Single-stroke (canonical)
(a)<==>(b)                      // Double-stroke
```

#### Undirected Arrow (`undirected_arrow`) - No directionality
```gram
(a)~~(b)                        // Squiggle (canonical)
(a)==(b)                        // Double-stroke
```

**Element Ordering:**
- **Right arrows**: Elements stored as `[left, right]` (as written)
- **Left arrows**: Elements stored as `[right, left]` (reversed from visual order!)
- **Bidirectional/Undirected**: Elements stored as `[first, second]` (as written)

### Subject Patterns (N elements)

```gram
[team | (alice), (bob)]         // Pattern with elements
[outer | [inner | (leaf)]]      // Nested patterns
[root:Type {p: 1} | e1, e2]     // With identifier, label, properties
```

### Annotated Patterns (1 element)

```gram
@type(node) (a)                 // Annotation on node
@depth(2) [x | y, z]            // Annotation on subject pattern
```

### Property Values

```gram
{
  name: "Alice",                // String
  age: 30,                      // Integer
  score: 95.5,                  // Decimal
  active: true,                 // Boolean
  tags: ["rust", "wasm"],       // Array
  range: 1..10                  // Range
}
```

### Comments

```gram
// This is a line comment
(hello)-->(world)  // End-of-line comment
```

## Advanced Usage

### Complex Relationships

```rust
let gram_text = "(alice:Person {name: \"Alice\", age: 30})-[:KNOWS {since: 2020}]->(bob:Person {name: \"Bob\"})";
let patterns = parse_gram_notation(gram_text)?;

// Access left node
println!("Left: {}", patterns[0].elements[0].value.identity.0);

// Access edge
println!("Edge labels: {:?}", patterns[0].value.labels);

// Access right node
println!("Right: {}", patterns[0].elements[1].value.identity.0);
```

### Nested Patterns

```rust
let gram_text = "[outer:Group | [inner:Team | (alice), (bob)], (charlie)]";
let patterns = parse_gram_notation(gram_text)?;

println!("Outer: {}", patterns[0].value.identity.0);
println!("Elements: {}", patterns[0].elements.len());
println!("First element is nested: {}", 
    !patterns[0].elements[0].elements.is_empty());
```

### Path Patterns

```rust
// Chained relationships
let gram_text = "(a)-->(b)-->(c)-->(d)";
let patterns = parse_gram_notation(gram_text)?;

// Creates nested relationship structure
println!("Path pattern parsed");
```

### Unicode Support

```rust
// Unicode identifiers must be quoted
let gram_text = "(\"世界\" {greeting: \"こんにちは\"})";
let patterns = parse_gram_notation(gram_text)?;

println!("Identifier: {}", patterns[0].value.identity.0);
```

### Error Handling

```rust
use gram_codec::parse_gram_notation;

let invalid_gram = "(unclosed";
match parse_gram_notation(invalid_gram) {
    Ok(_) => println!("Success"),
    Err(e) => {
        println!("Parse error: {}", e.message);
        println!("Error count: {}", e.error_count());
        // Access location information
        println!("Location: line {}, column {}", 
            e.location.line, e.location.column);
    }
}
```

## Examples

> 📚 **See [`../../examples/gram-codec-README.md`](../../examples/gram-codec-README.md) for complete examples across all platforms!**

Run the included examples:

```bash
# Basic usage (10 examples)
cargo run --package gram-codec --example basic_usage

# Advanced usage (10 examples)
cargo run --package gram-codec --example advanced_usage

# Interactive Python demo
python examples/gram-codec-python/demo.py

# Interactive browser demo
# (After building: wasm-pack build --target web . -- --features wasm)
python3 -m http.server 8000
# Open http://localhost:8000/examples/gram-codec-wasm-web/
```

## Testing

Run the comprehensive test suite:

```bash
# All tests (159 tests total)
cargo test --package gram-codec

# Specific test suite
cargo test --package gram-codec --test parser_tests              # 18 tests
cargo test --package gram-codec --test serializer_tests          # 15 tests
cargo test --package gram-codec --test value_types_tests         # 13 tests
cargo test --package gram-codec --test arrow_types_tests         # 14 tests (1 ignored)
cargo test --package gram-codec --test arrow_style_variants_tests # 19 tests
cargo test --package gram-codec --test edge_cases_tests          # 28 tests
cargo test --package gram-codec --test advanced_features_tests   # 22 tests
```

### Test Coverage

- **159 tests total** (158 passing + 1 ignored)
- Unit tests for error handling, parser core, serializer core
- Comprehensive arrow style variant tests (all 10 visual forms documented)
- Integration tests for all gram syntax forms
- Edge case handling (nesting, whitespace, comments, error recovery)
- Round-trip correctness validation

## Grammar Authority

This codec uses [`tree-sitter-gram`](https://github.com/gram-data/tree-sitter-gram) as the authoritative grammar specification. The pure Rust implementation is validated for 100% conformance with the tree-sitter-gram test corpus.

## Architecture

The codec consists of several modules:

- **`parser`**: Transforms Gram notation text → Pattern structures using a pure Rust `nom` implementation
- **`serializer`**: Transforms Pattern structures → Gram notation text
- **`ast`**: AST (Abstract Syntax Tree) types for cross-language JSON serialization
- **`value`**: Value enum for property types (String, Integer, Decimal, Boolean, Array, Range)
- **`error`**: Error types with location information

## Multi-Platform Support

### WASM (WebAssembly)

Build for browser and Node.js environments:

```bash
# Using wasm-pack
wasm-pack build --target web crates/gram-codec -- --features wasm

# JavaScript usage
import init, { parse_gram, validate_gram, round_trip } from './pkg/gram_codec.js';
await init();
const result = parse_gram("(hello)-->(world)");
```

**Examples:**
- `../../examples/gram-codec-wasm-web/` - Interactive browser demo with UI
- `../../examples/gram-codec-wasm-node/` - Node.js command-line examples

### Python

Build and install Python bindings:

```bash
pip install maturin
cd crates/gram-codec
maturin develop --features python

# Python usage
from gram_codec import parse_gram, validate_gram, round_trip
result = parse_gram("(hello)-->(world)")
print(f"Parsed {result['pattern_count']} patterns")
```

**Examples:**
- `../../examples/gram-codec-python/demo.py` - Interactive demo with REPL mode
- `../../examples/gram-codec-python/quickstart.py` - Quick 5-minute introduction
- `../../examples/gram-codec-python/gram_codec.py` - Template with 10 example functions

See `pyproject.toml` for packaging configuration.

## Performance

The codec includes comprehensive benchmarks:

```bash
# Run all benchmarks
cargo bench --package gram-codec

# Specific benchmarks
cargo bench --package gram-codec --bench codec_benchmarks -- parse_simple_nodes
cargo bench --package gram-codec --bench codec_benchmarks -- round_trip
```

Benchmarks cover:
- Parsing simple nodes (10, 100, 1000 patterns)
- Parsing relationships and chains
- Parsing subject patterns with many elements
- Serialization (single and multiple patterns)
- Round-trip correctness (parse → serialize → parse)
- Complex patterns with mixed syntax

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../../LICENSE) for details.

## Contributing

Contributions are welcome! Please ensure:

- All tests pass: `cargo test --package gram-codec`
- Code is formatted: `cargo fmt --all`
- No clippy warnings: `cargo clippy --package gram-codec -- -D warnings`
- Examples run successfully

## Resources

- [Gram Notation Specification](https://github.com/gram-data/tree-sitter-gram)
- [Pattern Core Library](../pattern-core/)
- [Project Documentation](../../specs/019-gram-codec/)
