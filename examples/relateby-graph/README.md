# relateby-graph Example

Demonstrates the TypeScript/WASM graph API using `@relateby/pattern` and `@relateby/graph`.

## Prerequisites

1. **Build the WASM module** (from the repository root):

   > **Note**: If Rust is installed via Homebrew (not rustup), the `build:wasm` scripts set
   > `PATH` to use the rustup toolchain automatically. Ensure rustup is installed and the
   > `stable` toolchain with `wasm32-unknown-unknown` target is available:
   > ```bash
   > rustup target add wasm32-unknown-unknown
   > ```

   ```bash
   cd typescript/@relateby/graph
   npm install && npm run build

   cd ../pattern
   npm install
   npm run build:wasm        # bundler target (for browser/bundler use)
   npm run build:wasm:node   # nodejs target (for Node.js use)
   npm run build:ts
   ```

2. **Install example dependencies**:
   ```bash
   cd examples/relateby-graph
   npm install
   ```

## Running

### Node.js

```bash
node node.mjs
```

Expected output:
```
Graph: 3 nodes, 2 relationships
BFS from alice: alice, bob, charlie
Degree centrality: { bob: 1, alice: 0.5, charlie: 0.5 }
Nodes after filter: 3 (Person nodes only)
Processed node IDs: [ 'processed:alice', 'processed:bob', 'processed:charlie' ]
```

### Browser

Open `browser.html` in a browser after building the WASM module with bundler target:
```bash
cd typescript/@relateby/pattern
npm run build:wasm  # uses --target bundler
```

## Architecture

- `@relateby/pattern`: WASM-backed types (`NativePattern`, `NativePatternGraph`, `NativeGraphQuery`, algorithms)
- `@relateby/graph`: Pure TypeScript interfaces and transforms (`toGraphView`, `mapGraph`, `filterGraph`, etc.)
- The `Native*` classes satisfy the TypeScript interfaces from `@relateby/graph` structurally.

## WASM-Free Usage

`@relateby/graph` works entirely without WASM. See the WASM-free stub pattern:

```typescript
import { toGraphView, mapGraph, GNode } from "@relateby/graph";
import type { Subject, Pattern, PatternGraph } from "@relateby/graph";

const stubNode = (id: string): Pattern<Subject> => ({
  identity: id,
  value: { identity: id, labels: new Set(["Person"]), properties: {} },
  elements: [],
});

const graph: PatternGraph<Subject> = {
  nodes: [stubNode("alice"), stubNode("bob")],
  relationships: [],
  walks: [],
  annotations: [],
  conflicts: {},
  size: 2,
  merge: (other) => graph,
  topoSort: () => [stubNode("alice"), stubNode("bob")],
};

const view = toGraphView(graph);
// All transforms work identically regardless of whether graph is WASM-backed
```
