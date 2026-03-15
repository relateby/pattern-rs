# Quickstart: StandardGraph

**Feature**: 035-standard-graph
**Date**: 2026-03-15

## Build and Test

```bash
# Build
cargo build -p pattern-core

# Run all tests
cargo test -p pattern-core

# Run StandardGraph tests specifically
cargo test -p pattern-core standard_graph

# Lint
cargo clippy --workspace

# Full CI check
./scripts/ci-local.sh
```

## Usage Examples

### 1. Build a Graph Element by Element

```rust
use pattern_core::graph::StandardGraph;
use pattern_core::subject::Subject;

let mut g = StandardGraph::new();

// Add nodes using the fluent builder
g.add_node(Subject::build("alice").label("Person").property("name", "Alice").done());
g.add_node(Subject::build("bob").label("Person").property("name", "Bob").done());

// Add a relationship
g.add_relationship(
    Subject::build("r1").label("KNOWS").property("since", 2020).done(),
    &"alice".into(),
    &"bob".into(),
);

assert_eq!(g.node_count(), 2);
assert_eq!(g.relationship_count(), 1);
```

### 2. Build a Graph from Gram Notation

```rust
use pattern_core::graph::StandardGraph;
use gram_codec::FromGram;

let g = StandardGraph::from_gram(
    "(alice:Person {name:'Alice'})-[:KNOWS {since:2020}]->(bob:Person {name:'Bob'})"
).unwrap();

assert_eq!(g.node_count(), 2);
assert_eq!(g.relationship_count(), 1);
```

### 3. Query the Graph

```rust
// Retrieve elements by identity
let alice = g.node(&"alice".into()).unwrap();
let knows = g.relationship(&"r1".into()).unwrap();

// Navigate relationships
let source = g.source(&"r1".into()).unwrap();
let target = g.target(&"r1".into()).unwrap();

// Neighbors and degree (both directions)
let neighbors = g.neighbors(&"alice".into());
let degree = g.degree(&"alice".into());
```

### 4. Convert to Abstract Types

```rust
// For advanced algorithms
let query = g.as_query();
let snapshot = g.as_snapshot();

// For full access to PatternGraph
let pg = g.as_pattern_graph();
```

### 5. Fluent Subject Builder (standalone)

```rust
use pattern_core::subject::Subject;

let subject = Subject::build("alice")
    .label("Person")
    .label("Employee")
    .property("name", "Alice Smith")
    .property("age", 30)
    .property("active", true)
    .done();

assert_eq!(subject.identity.0, "alice");
assert!(subject.labels.contains("Person"));
assert!(subject.labels.contains("Employee"));
```

## Key Files

| File | Purpose |
|------|---------|
| `crates/pattern-core/src/graph/standard.rs` | StandardGraph implementation |
| `crates/pattern-core/src/subject.rs` | SubjectBuilder (added to existing file) |
| `crates/pattern-core/src/graph/mod.rs` | Module re-exports |
| `crates/pattern-core/src/lib.rs` | Top-level re-exports |
| `crates/pattern-core/tests/standard_graph_tests.rs` | Integration tests |
