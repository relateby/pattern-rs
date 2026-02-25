# WASM Usage Guide

This guide covers using pattern-rs from JavaScript/TypeScript via WebAssembly.

## Packages

Three scoped npm packages expose the Rust functionality:

| Package | Description | WASM? |
|---------|-------------|-------|
| `@relateby/pattern` | WASM-backed types and graph algorithms | Yes |
| `@relateby/gram` | Gram notation codec | Via `@relateby/pattern` |
| `@relateby/graph` | Pure TypeScript interfaces and transforms | No |

## Graph API

For the complete TypeScript graph API reference, see **[docs/typescript-graph.md](./typescript-graph.md)**.

This covers:
- Package installation and initialization
- `NativePatternGraph`, `NativeReconciliationPolicy`, `NativeGraphQuery`
- Algorithm functions (BFS, DFS, shortest path, centrality, etc.)
- Pure TypeScript transforms (`mapGraph`, `filterGraph`, `foldGraph`, `paraGraph`, etc.)
- WASM-free stub pattern
- Effect integration
- Performance notes

## Building the WASM Module

```bash
# Install wasm-pack
cargo install wasm-pack

# Build the WASM module (from repo root)
cd typescript/@relateby/pattern
npm run build:wasm  # runs wasm-pack --target bundler
npm run build:ts    # compiles TypeScript
```

The `build:wasm` script runs:
```bash
wasm-pack build ../../../crates/pattern-wasm --target bundler --out-dir ../../../typescript/@relateby/pattern/wasm
```

## Quick Start

```typescript
import { init, NativeSubject, NativePattern, NativePatternGraph, NativeGraphQuery, bfs } from "@relateby/pattern";
import { toGraphView, mapGraph } from "@relateby/graph";

// Initialize WASM (Node.js; bundlers auto-initialize)
await init();

// Build a graph
const alice = NativePattern.point(new NativeSubject("alice", ["Person"], {}));
const bob = NativePattern.point(new NativeSubject("bob", ["Person"], {}));
const graph = NativePatternGraph.fromPatterns([alice, bob]);

// Query
const query = NativeGraphQuery.fromPatternGraph(graph);
const aliceNode = query.nodeById("alice");
const traversal = bfs(query, aliceNode!);

// Transform (pure TypeScript, no WASM)
const view = toGraphView(graph);
const mapped = mapGraph({ mapNode: (p) => p })(view);
```

## Pattern Types

### `NativePattern` (formerly `Pattern`)

The foundational data structure. Wraps `Pattern<Subject>`.

```typescript
const atomic = NativePattern.point(subject);      // atomic pattern
const composite = NativePattern.pattern(subject); // pattern with children
composite.addElement(child);
```

### `NativeSubject` (formerly `Subject`)

A self-descriptive value with identity, labels, and properties.

```typescript
const subject = new NativeSubject(
  "alice",                    // identity
  ["Person", "User"],         // labels
  { name: Value.string("Alice"), age: Value.int(30) }  // properties
);
```

### `NativeValue` (formerly `Value`)

Factory for typed property values.

```typescript
import { NativeValue } from "@relateby/pattern";

NativeValue.string("hello")
NativeValue.int(42)
NativeValue.float(3.14)
NativeValue.bool(true)
NativeValue.null()
```

## Validation

```typescript
import { NativeValidationRules } from "@relateby/pattern";

const rules = NativeValidationRules.new(10, 100); // maxDepth=10, maxElements=100
const result = pattern.validate(rules);
// result: { _tag: 'Right', right: void } | { _tag: 'Left', left: ValidationError }
```

## CI/CD

Before pushing, run all CI checks:

```bash
./scripts/ci-local.sh
```

This validates:
1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace -- -D warnings`
3. `cargo build --workspace` (native)
4. `cargo build --workspace --target wasm32-unknown-unknown` (WASM)
5. `cargo test --workspace`
