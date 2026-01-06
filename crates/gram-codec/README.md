# Gram Codec

Bidirectional codec between [Gram notation](https://github.com/gram-data/tree-sitter-gram) (human-readable text format) and Pattern data structures.

[![CI Status](https://github.com/gram-data/gram-rs/workflows/CI/badge.svg)](https://github.com/gram-data/gram-rs/actions)
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
use gram_codec::parse_gram_notation;

// Parse a simple node
let patterns = parse_gram_notation("(hello)")?;
println!("Identifier: {}", patterns[0].value.identity.0);

// Parse a relationship
let patterns = parse_gram_notation("(alice)-[:KNOWS]->(bob)")?;
println!("Relationship with {} elements", patterns[0].elements.len());

// Parse a subject pattern
let patterns = parse_gram_notation("[team | (alice), (bob)]")?;
println!("Team has {} members", patterns[0].elements.len());
```

### Serializing Patterns

```rust
use gram_codec::serialize_pattern;
use pattern_core::{Pattern, Subject, Symbol};
use std::collections::{HashMap, HashSet};

// Create a pattern
let subject = Subject {
    identity: Symbol("hello".to_string()),
    labels: HashSet::new(),
    properties: HashMap::new(),
};
let pattern = Pattern::point(subject);

// Serialize to Gram notation
let gram_text = serialize_pattern(&pattern)?;
println!("{}", gram_text); // Output: (hello)
```

### Round-Trip Correctness

```rust
use gram_codec::{parse_gram_notation, serialize_pattern};

let original = "(alice:Person {name: \"Alice\"})";

// Parse
let parsed = parse_gram_notation(original)?;

// Serialize
let serialized = serialize_pattern(&parsed[0])?;

// Re-parse
let reparsed = parse_gram_notation(&serialized)?;

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

Run the included examples:

```bash
# Basic usage
cargo run --package gram-codec --example basic_usage

# Advanced usage
cargo run --package gram-codec --example advanced_usage
```

## Testing

Run the comprehensive test suite:

```bash
# All tests
cargo test --package gram-codec

# Specific test suite
cargo test --package gram-codec --test parser_tests
cargo test --package gram-codec --test serializer_tests
cargo test --package gram-codec --test value_types_tests
cargo test --package gram-codec --test arrow_types_tests
cargo test --package gram-codec --test edge_cases_tests
cargo test --package gram-codec --test advanced_features_tests
```

## Grammar Authority

This codec uses [`tree-sitter-gram`](https://github.com/gram-data/tree-sitter-gram) as the authoritative grammar specification. The grammar is included as a git submodule:

```bash
# Initialize submodule
git submodule update --init --recursive

# Validate gram notation with gram-lint
gram-lint -e "(hello)-->(world)"
gram-lint -e "(hello)-->(world)" --tree  # Show parse tree
```

## Architecture

The codec consists of several modules:

- **`parser`**: Transforms Gram notation text → Pattern structures using tree-sitter-gram
- **`serializer`**: Transforms Pattern structures → Gram notation text
- **`transform`**: CST (Concrete Syntax Tree) → AST (Abstract Syntax Tree) transformation
- **`value`**: Value enum for property types (String, Integer, Decimal, Boolean, Array, Range)
- **`error`**: Error types with location information and error recovery

## Future Work

- **WASM Support**: WebAssembly bindings for browser/Node.js usage (see `src/wasm.rs`)
- **Python Bindings**: PyO3 bindings for Python integration (see `src/python.rs`)
- **Performance Optimization**: Benchmarks and performance tuning
- **Corpus Testing**: Automated testing against tree-sitter-gram test corpus

## Constitutional Compliance

**Note**: This codec deviates from the standard gram-rs constitution (Principle I: Reference Implementation Fidelity) by using `tree-sitter-gram` instead of `gram-hs` as the authoritative grammar reference. This deviation is justified because:

1. **Explicit Requirement**: Feature specification requires tree-sitter-gram as authority
2. **Validation Integration**: Enables `gram-lint` validation of all codec output
3. **Multi-Platform Support**: Leverages tree-sitter's WASM support for browser deployment

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

