# Quickstart: TypeScript/WASM Graph API

**Branch**: `033-typescript-wasm-graph`  
**Date**: 2026-02-25

This guide covers building the `@relateby/pattern` npm package from source and running the graph example. For end-user installation, see `docs/typescript-graph.md` (created as part of this feature).

---

## Prerequisites

- Rust 1.70.0+ with `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- `wasm-pack`: `cargo install wasm-pack`
- Node.js 18+ and npm

---

## Build

```bash
# From the repo root
cd typescript/@relateby/pattern

# Step 1: Compile Rust → WASM (produces typescript/@relateby/pattern/wasm/)
npm run build:wasm

# Step 2: Compile TypeScript → dist/ (ESM + CJS)
npm run build:ts

# Or both steps at once
npm run build
```

To build the pure TypeScript transforms package (no WASM required):

```bash
cd typescript/@relateby/graph
npm run build
```

---

## Run Tests

```bash
# WASM-backed types and algorithms
cd typescript/@relateby/pattern
npm test

# Pure TypeScript transforms (no WASM, no init() required)
cd typescript/@relateby/graph
npm test
```

---

## Run the Graph Example

```bash
# From the repo root
cd examples/relateby-graph

npm install
node node.mjs
```

Expected output:

```
Graph: 3 nodes, 1 relationship
BFS from alice: alice, bob
Shortest path alice → bob: alice, r1, bob
Degree centrality: { alice: 0.5, bob: 0.5, carol: 0 }
Connected components: [[alice, bob], [carol]]
mapGraph result: 3 nodes (timestamps updated)
filterGraph result: 2 nodes (carol removed)
paraGraph depths: { alice: 1, bob: 0, r1: 0 }
```

---

## Quick Code Example

```typescript
import { init, NativeSubject, NativeValue, NativePattern, NativePatternGraph,
         NativeGraphQuery, NativeReconciliationPolicy,
         bfs, shortestPath, degreeCentrality,
         connectedComponents } from "@relateby/pattern";
import { pipe, Option } from "effect";

await init();

// Build subjects
const alice = new NativeSubject("alice", ["Person"], { name: NativeValue.string("Alice") });
const bob   = new NativeSubject("bob",   ["Person"], { name: NativeValue.string("Bob") });
const carol = new NativeSubject("carol", ["Person"], { name: NativeValue.string("Carol") });
const rel   = new NativeSubject("r1",    ["KNOWS"],  {});

// Build patterns
const pAlice = NativePattern.point(alice);
const pBob   = NativePattern.point(bob);
const pCarol = NativePattern.point(carol);
const pRel   = NativePattern.pattern(rel);
pRel.addElement(pAlice);
pRel.addElement(pBob);

// Construct graph
const graph = NativePatternGraph.fromPatterns(
  [pAlice, pBob, pCarol, pRel],
  NativeReconciliationPolicy.lastWriteWins()
);

console.log(graph.nodes.length);         // 3
console.log(graph.relationships.length); // 1

// Query
const query = NativeGraphQuery.fromPatternGraph(graph);

// BFS
const visited = bfs(query, pAlice, "undirected");
console.log(visited.map(p => p.value.identity)); // ["alice", "bob"]

// Shortest path (with Effect Option)
const path = shortestPath(query, pAlice, pBob);
const names = pipe(
  path,
  Option.map(nodes => nodes.map(n => n.value.identity)),
  Option.getOrElse(() => []),
);
console.log(names); // ["alice", "r1", "bob"]

// Centrality
const centrality = degreeCentrality(query);
console.log(centrality); // { alice: 0.5, bob: 0.5, carol: 0 }
```

---

## Pure TypeScript Transforms

```typescript
import { pipe } from "effect";
import { toGraphView, mapAllGraph, filterGraph, foldGraph, paraGraph,
         SpliceGap, GNode, GRelationship } from "@relateby/graph";

// Construct a GraphView from any PatternGraph<V> — WASM-backed or plain TS stub
const view = toGraphView(graph);

// Point-free pipeline
const processed = pipe(
  view,
  mapAllGraph(addTimestamp),
  filterGraph((cls, p) => isRelevant(p), SpliceGap),
);

// Count nodes and relationships
const { nodes, rels } = foldGraph(
  (cls, _p) =>
    cls.tag === "GNode"         ? { nodes: 1, rels: 0 }
    : cls.tag === "GRelationship" ? { nodes: 0, rels: 1 }
    : { nodes: 0, rels: 0 },
  { nodes: 0, rels: 0 },
  (a, b) => ({ nodes: a.nodes + b.nodes, rels: a.rels + b.rels }),
)(view);

// Bottom-up depth computation
const depths = paraGraph(
  (_query, _p, subResults: number[]) =>
    subResults.length === 0 ? 0 : Math.max(...subResults) + 1,
)(view);
```

---

## Development Workflow

```bash
# After changing Rust code in crates/pattern-core/src/wasm.rs:
cd typescript/@relateby/pattern && npm run build:wasm   # recompile WASM

# After changing TypeScript in typescript/@relateby/pattern/src/:
cd typescript/@relateby/pattern && npm run build:ts     # recompile TypeScript

# After changing TypeScript in typescript/@relateby/graph/src/:
cd typescript/@relateby/graph && npm run build          # pure TS, no WASM step

# Run all CI checks before pushing:
./scripts/ci-local.sh
```

---

## Verification Against Haskell Reference

```bash
# Verify Rust graph implementations match pattern-hs
cargo test -p pattern-core -- graph

# Verify TypeScript transforms match reference behavior
cd typescript/@relateby/graph && npm test

# Verify WASM-backed algorithms match reference behavior
cd typescript/@relateby/pattern && npm test
```

The TypeScript test suites include equivalence tests ported from `../pattern-hs/libs/pattern/tests/` for both graph algorithm functions (`typescript/@relateby/pattern/tests/pattern.test.ts`) and pure TypeScript transform functions (`typescript/@relateby/graph/tests/graph.test.ts`).
