# Quickstart: StandardGraph Bindings

**Feature**: 036-standardgraph-bindings | **Date**: 2026-03-15

## TypeScript/WASM

### Build

```bash
# Build WASM package
cd crates/pattern-wasm
wasm-pack build --target nodejs
```

### Usage

```typescript
import { StandardGraph, Subject, Value } from './pkg/pattern_wasm.js';

// Create a graph from gram notation
const g = StandardGraph.fromGram('(alice:Person{name:"Alice"})-[:KNOWS]->(bob:Person{name:"Bob"})');
console.log(g.nodeCount);        // 2
console.log(g.relationshipCount); // 1

// Or build programmatically
const g2 = new StandardGraph();
g2.addNode(Subject.build("alice").label("Person").property("name", Value.string("Alice")).done())
  .addNode(Subject.build("bob").label("Person").property("name", Value.string("Bob")).done())
  .addRelationship(Subject.build("r1").label("KNOWS").done(), "alice", "bob");

// Query the graph
const neighbors = g2.neighbors("alice"); // [Pattern for bob]
const degree = g2.degree("alice");       // 1
const source = g2.source("r1");          // Pattern for alice
const target = g2.target("r1");          // Pattern for bob

// Escape to existing algorithm interface
const query = g2.asQuery();
// pass query to bfs(), shortestPath(), etc.
```

## Python

### Build

```bash
cd crates/pattern-core
maturin develop --uv --features python
```

### Usage

```python
from pattern_core import StandardGraph, Subject

# Create a graph from gram notation
g = StandardGraph.from_gram('(alice:Person{name:"Alice"})-[:KNOWS]->(bob:Person{name:"Bob"})')
print(g.node_count)         # 2
print(g.relationship_count) # 1

# Or build programmatically
g2 = StandardGraph()
g2.add_node(Subject.build("alice").label("Person").property("name", "Alice").done()) \
  .add_node(Subject.build("bob").label("Person").property("name", "Bob").done()) \
  .add_relationship(Subject.build("r1").label("KNOWS").done(), "alice", "bob")

# Query the graph
neighbors = g2.neighbors("alice")  # [PatternSubject for bob]
degree = g2.degree("alice")        # 1
source = g2.source("r1")           # PatternSubject for alice
target = g2.target("r1")           # PatternSubject for bob

# Iterate
for node_id, pattern in g2.nodes():
    print(f"{node_id}: {pattern}")

# Pythonic
print(len(g2))   # total element count
print(repr(g2))  # StandardGraph(nodes=2, relationships=1, walks=0, annotations=0)
```

## Development Workflow

```bash
# Run all Rust tests (includes StandardGraph unit tests)
cargo test --workspace

# Run WASM build check
cargo build --workspace --target wasm32-unknown-unknown

# Run Python binding tests
cd crates/pattern-core
maturin develop --uv --features python && pytest tests/python/

# Full CI check
./scripts/ci-local.sh
```
