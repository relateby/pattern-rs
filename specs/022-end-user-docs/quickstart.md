# Quickstart: pattern-rs in 5 Minutes

Learn how to use the Pattern data structure in your Rust projects.

## 1. Add Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
pattern-core = { path = "crates/pattern-core" }
gram-codec = { path = "crates/gram-codec" }
```

## 2. Create a Pattern Programmatically

```rust
use pattern_core::Pattern;

// Create an atomic pattern (a node)
let node = Pattern::point("Person");

// Create a pattern with elements
let pattern = Pattern::pattern("KNOWS", vec![node.clone(), node.clone()]);

println!("Pattern: {:?}", pattern);
```

## 3. Parse from Gram Notation

```rust
use gram_codec::parse_gram;

let gram = "(a:Person)-[r:KNOWS]->(b:Person)";
let patterns = parse_gram(gram).expect("Failed to parse");
let pattern = &patterns[0];

println!("Parsed value: {:?}", pattern.value());
println!("Element count: {}", pattern.elements().len());
```

## 4. Query Values

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("root", vec![
    Pattern::point("child1"),
    Pattern::point("child2"),
]);

// Check if any value contains "child"
let has_child = pattern.any_value(|v| v.contains("child"));
assert!(has_child);
```
