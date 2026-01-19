# Rust Usage Guide

This guide provides practical examples for using the `gram-rs` crates in your Rust projects.

## Adding Dependencies

To use `gram-rs`, add the following crates to your `Cargo.toml`. Since the library is organized as a workspace, you can reference them by path if you're working within the repo, or by version once published.

```toml
[dependencies]
pattern-core = { path = "crates/pattern-core" }
gram-codec = { path = "crates/gram-codec" }
```

## Programmatic Construction

The `pattern-core` crate provides the `Pattern` type and its core constructors. Most patterns in `gram-rs` use `Subject` as their value type, which includes an identity, labels, and properties.

### Creating an Atomic Pattern (Node)
Use `Pattern::point` to create a pattern with no elements.

```rust
use pattern_core::{Pattern, Subject, Symbol};
use std::collections::{HashSet, HashMap};

// Create a node with identity "alice" and label "Person"
let subject = Subject {
    identity: Symbol("alice".to_string()),
    labels: {
        let mut s = HashSet::new();
        s.insert("Person".to_string());
        s
    },
    properties: HashMap::new(),
};
let node = Pattern::point(subject);

assert!(node.is_atomic());
assert_eq!(node.value().identity.0, "alice");
```

### Creating a Nested Pattern
Use `Pattern::pattern` to create a pattern with elements.

```rust
use pattern_core::{Pattern, Subject, Symbol};

// Simplified point patterns for elements
let alice = Pattern::point(Subject::from_identity("alice"));
let bob = Pattern::point(Subject::from_identity("bob"));

// Create a relationship pattern "Alice knows Bob"
let knows = Pattern::pattern(
    Subject::from_label("KNOWS"),
    vec![alice, bob]
);

assert_eq!(knows.elements().len(), 2);
```

## Parsing and Serialization

The `gram-codec` crate handles conversion between Gram notation and Rust `Pattern` structures. For more details on Gram syntax, see the **[Gram Notation Reference](gram-notation.md)**.

### Parsing Gram Notation
Use `parse_gram` to parse a string into a collection of patterns.

```rust
use gram_codec::parse_gram;

let gram = "(a:Person)-[r:KNOWS]->(b:Person)";
let patterns = parse_gram(gram).expect("Failed to parse Gram notation");

// parse_gram returns a Vec of all top-level patterns
assert_eq!(patterns.len(), 1);
println!("Relationship labels: {:?}", patterns[0].value().labels);
```

### Parsing with Headers
Use `parse_gram_with_header` when your document uses a leading record as metadata.

```rust
use gram_codec::parse_gram_with_header;

let input = "{version: 1.0} (a)-->(b)";
let (header, patterns) = parse_gram_with_header(input).unwrap();

if let Some(h) = header {
    println!("Document version: {:?}", h.get("version"));
}
assert_eq!(patterns.len(), 1);
```

### Serializing to Gram Notation
Use `to_gram` to serialize patterns back to notation.

```rust
use gram_codec::to_gram;
use pattern_core::{Pattern, Subject};

let node = Pattern::point(Subject::from_identity("node"));
// to_gram takes a Vec of patterns and an optional separator (defaults to space)
let gram_string = to_gram(vec![node], None).unwrap();

assert_eq!(gram_string, "(node)");
```

### Serializing with a Header
```rust
use gram_codec::{to_gram_with_header, Record};
use pattern_core::{Pattern, Subject, Value};

let mut header = Record::new();
header.insert("type".to_string(), Value::VString("graph".to_string()));

let node = Pattern::point(Subject::from_identity("a"));
let output = to_gram_with_header(header, vec![node]).unwrap();

assert_eq!(output, "{type: \"graph\"} (a)");
```

## Basic Queries

`Pattern` provides several utilities for inspecting and querying its structure and values.

### Checking Values
You can check if any or all values in a pattern satisfy a predicate.

```rust
use pattern_core::{Pattern, Subject};

let pattern = Pattern::pattern(
    Subject::from_identity("root"), 
    vec![
        Pattern::point(Subject::from_identity("child1")),
        Pattern::point(Subject::from_identity("child2")),
    ]
);

// Does any subject identity contain "child"?
let has_child = pattern.any_value(|v| v.identity.0.contains("child"));
assert!(has_child);
```

### Structural Inspection
```rust
use gram_codec::parse_gram;

let patterns = parse_gram("(a)-[r]->(b)").unwrap();
let rel = &patterns[0];

println!("Length: {}", rel.length()); // Direct elements: 2
println!("Size: {}", rel.size());     // Total subjects: 3
println!("Depth: {}", rel.depth());   // Max nesting: 1
```
