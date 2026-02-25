# TypeScript Graph API Reference

This document describes the TypeScript/WASM graph API exposed via three scoped npm packages:

- **`@relateby/pattern`**: WASM-backed types and algorithms (`NativePattern`, `NativePatternGraph`, `NativeGraphQuery`, etc.)
- **`@relateby/gram`**: Gram notation codec (`Gram.parse`, `Gram.stringify`)
- **`@relateby/graph`**: Pure TypeScript interfaces and transforms (no WASM dependency)

## Package Installation

```bash
npm install @relateby/pattern @relateby/gram @relateby/graph
```

After installing, build the WASM module:

```bash
cd node_modules/@relateby/pattern
npm run build:wasm
```

## Initialization

### Node.js

In Node.js environments, call `init()` before using any WASM types:

```typescript
import { init, NativePatternGraph } from "@relateby/pattern";

await init();

const graph = NativePatternGraph.empty();
```

### Bundler (Vite, webpack, Rollup, esbuild)

In bundler environments, the wasm-pack `--target bundler` glue handles initialization automatically via ES module top-level await. No explicit `init()` call is needed:

```typescript
import { NativePatternGraph } from "@relateby/pattern";
// WASM is already initialized — use types directly
const graph = NativePatternGraph.empty();
```

## Graph Construction

### `NativePatternGraph`

```typescript
import { init, NativeSubject, NativePattern, NativePatternGraph, NativeReconciliationPolicy } from "@relateby/pattern";

await init();

// Create subjects
const aliceSubject = new NativeSubject("alice", ["Person"], { name: "Alice" });
const bobSubject = new NativeSubject("bob", ["Person"], { name: "Bob" });

// Create node patterns
const alice = NativePattern.point(aliceSubject);
const bob = NativePattern.point(bobSubject);

// Create a relationship pattern
const rel = NativePattern.pattern(new NativeSubject("r1", ["KNOWS"], {}));
rel.addElement(alice);
rel.addElement(bob);

// Build the graph (LastWriteWins is the default policy)
const graph = NativePatternGraph.fromPatterns(
  [alice, bob, rel],
  NativeReconciliationPolicy.lastWriteWins()
);

console.log(graph.nodes.length);         // 2
console.log(graph.relationships.length); // 1
console.log(graph.size);                 // 3
```

### `NativeReconciliationPolicy`

Controls how identity conflicts are resolved when building a graph:

```typescript
import { NativeReconciliationPolicy } from "@relateby/pattern";

// Incoming pattern replaces existing (default)
const lww = NativeReconciliationPolicy.lastWriteWins();

// Existing pattern is kept; incoming discarded
const fww = NativeReconciliationPolicy.firstWriteWins();

// Conflict recorded in graph.conflicts; neither wins
const strict = NativeReconciliationPolicy.strict();

// Merge labels and properties
const merge = NativeReconciliationPolicy.merge({
  elementStrategy: "union",   // "replace" | "append" | "union"
  labelMerge: "union",        // "union" | "intersect" | "left" | "right"
  propertyMerge: "merge",     // "left" | "right" | "merge"
});
```

## Querying with `NativeGraphQuery`

```typescript
import { NativeGraphQuery } from "@relateby/pattern";

const query = NativeGraphQuery.fromPatternGraph(graph);

// Navigation
const nodes = query.nodes();
const rels = query.relationships();
const aliceNode = query.nodeById("alice");
const r1 = query.relationshipById("r1");

// Structural queries
const src = query.source(r1!);        // alice
const tgt = query.target(r1!);        // bob
const degree = query.degree(aliceNode!); // 1
const incident = query.incidentRels(aliceNode!); // [r1]
```

## Algorithm Functions

All algorithm functions are free functions exported from `@relateby/pattern`. Each is one WASM crossing; all computation is Rust-internal.

```typescript
import {
  bfs, dfs, shortestPath, allPaths, connectedComponents,
  hasCycle, isConnected, topologicalSort,
  degreeCentrality, betweennessCentrality, minimumSpanningTree,
  queryWalksContaining, queryCoMembers, queryAnnotationsOf,
} from "@relateby/pattern";

// Traversal
const bfsOrder = bfs(query, aliceNode!);
const dfsOrder = dfs(query, aliceNode!);

// Paths
const path = shortestPath(query, aliceNode!, bobNode!);  // Pattern[] | null
const allP = allPaths(query, aliceNode!, bobNode!);       // Pattern[][]

// Structure
const components = connectedComponents(query);  // Pattern[][]
const cyclic = hasCycle(query);                  // boolean
const connected = isConnected(query);            // boolean
const topoOrder = topologicalSort(query);        // Pattern[] | null (null if cyclic)

// Centrality
const degree = degreeCentrality(query);          // Record<string, number>
const between = betweennessCentrality(query);    // Record<string, number>

// Spanning tree
const mst = minimumSpanningTree(query);          // Pattern[]

// Context queries
const walks = queryWalksContaining(query, node);
const coMembers = queryCoMembers(query, node, container);
const annotations = queryAnnotationsOf(query, target);
```

### Weight Functions

All traversal algorithms accept an optional `weight` parameter:

```typescript
import type { Weight } from "@relateby/pattern";

// String constants (zero overhead)
bfs(query, start, "undirected");       // all edges bidirectional (default)
bfs(query, start, "directed");         // follow edge direction only
bfs(query, start, "directed_reverse"); // follow edges in reverse only

// Custom weight function (one WASM crossing per traversed edge)
// WARNING: For a dense graph with 50,000 edges, this is 50,000 WASM crossings per traversal.
// Use string constants when possible.
const customWeight: Weight = (rel, direction) => {
  const cost = rel.value?.properties?.cost as number ?? 1;
  return direction === "forward" ? cost : cost * 2;
};
shortestPath(query, start, end, customWeight);
```

## Pure TypeScript Transforms (`@relateby/graph`)

`@relateby/graph` provides pure TypeScript interfaces and transform functions. It has **no runtime dependency on WASM** and works with any object satisfying the structural interfaces.

```typescript
import {
  toGraphView,
  mapGraph, mapAllGraph, filterGraph, foldGraph,
  mapWithContext, paraGraph, paraGraphFixed, unfoldGraph,
  GNode, GRelationship, GWalk, GAnnotation, GOther,
  SpliceGap, DeleteContainer, ReplaceWithSurrogate,
} from "@relateby/graph";
import type { Subject, Pattern, PatternGraph, GraphQuery, GraphView } from "@relateby/graph";

// Convert PatternGraph to GraphView for transforms
const view = toGraphView(graph); // graph: PatternGraph<Subject>

// Map nodes with a custom function
const mapped = mapGraph({
  mapNode: (p) => ({ ...p, identity: `node:${p.identity}` }),
  mapRelationship: (p) => p,
})(view);

// Filter: keep only Person nodes (SpliceGap repairs walks)
const filtered = filterGraph(
  (cls, p) => cls.tag !== "GNode" || (p.value?.labels.has("Person") ?? false),
  SpliceGap
)(view);

// Fold: count elements by class
const counts = foldGraph(
  (cls, _p) => cls.tag === "GNode" ? { nodes: 1, rels: 0 }
              : cls.tag === "GRelationship" ? { nodes: 0, rels: 1 }
              : { nodes: 0, rels: 0 },
  { nodes: 0, rels: 0 },
  (a, b) => ({ nodes: a.nodes + b.nodes, rels: a.rels + b.rels })
)(view);

// Para: bottom-up depth computation
// paraGraph calls topoSort() once (one WASM crossing when WASM-backed)
const depths = paraGraph(
  (_query, _p, subResults: number[]) =>
    subResults.length === 0 ? 0 : Math.max(...subResults) + 1
)(view);

// Unfold: expand seeds into a graph
const expanded = unfoldGraph(
  (seed: string) => [{ identity: seed, value: { identity: seed, labels: new Set(), properties: {} }, elements: [] }],
  (patterns) => graph // use your PatternGraph constructor
)(["alice", "bob"]);
```

### Point-Free Pipeline Composition

All transforms are curried and compose with `pipe` from Effect (or any compatible utility):

```typescript
import { pipe } from "effect";
import { toGraphView, mapAllGraph, filterGraph, mapWithContext, SpliceGap } from "@relateby/graph";

const processed = pipe(
  toGraphView(graph),
  mapAllGraph(updateTimestamp),
  filterGraph(isRelevant, SpliceGap),
  mapWithContext(enrich),
);
```

## WASM-Free Stub Pattern (SC-011)

`@relateby/graph` can be used entirely without WASM. Implement the interfaces as plain TypeScript objects:

```typescript
import { toGraphView, mapGraph } from "@relateby/graph";
import type { Subject, Pattern, PatternGraph } from "@relateby/graph";

// No NativePattern, no init() required
const stubNode = (id: string): Pattern<Subject> => ({
  identity: id,
  value: { identity: id, labels: new Set(["Person"]), properties: {} },
  elements: [],
});

const stubGraph: PatternGraph<Subject> = {
  nodes: [stubNode("alice"), stubNode("bob")],
  relationships: [],
  walks: [],
  annotations: [],
  conflicts: {},
  size: 2,
  merge: (other) => stubGraph,
  topoSort: () => [stubNode("alice"), stubNode("bob")],
};

const view = toGraphView(stubGraph);
// All transforms work identically regardless of whether graph is WASM-backed
```

## Effect Integration

When the `effect` package is installed (`npm install effect`), `@relateby/pattern` wraps nullable returns as `Option.Option<T>` and fallible operations as `Either.Either<T, E>`:

```typescript
import { Option, Either } from "effect";
import { shortestPath, topologicalSort, validate } from "@relateby/pattern";

// shortestPath returns Option<Pattern<Subject>[]> when effect is available
const path = shortestPath(query, start, end);
if (Option.isSome(path)) {
  console.log("Path:", path.value);
}

// topologicalSort returns Option<Pattern<Subject>[]> (None if cyclic)
const sorted = topologicalSort(query);
Option.match(sorted, {
  onNone: () => console.log("Graph has a cycle"),
  onSome: (order) => console.log("Topological order:", order),
});
```

## Performance Notes

- **Algorithm functions**: One WASM crossing per call; all computation is Rust-internal. Efficient for large graphs.
- **Custom weight callbacks**: Called once per traversed edge. For a dense graph with 50,000 edges, this is 50,000 WASM crossings per traversal. **Use string constants (`"undirected"`, `"directed"`) when possible.**
- **`paraGraph`**: Calls `topoSort()` once (one WASM crossing when backed by `NativePatternGraph`); all subsequent iteration is pure TypeScript.
- **Pure TS transforms**: `mapGraph`, `filterGraph`, `foldGraph`, etc. have zero WASM crossings. Callback functions are called in TypeScript.

## Gram Codec

```typescript
import { init } from "@relateby/pattern";
import { Gram } from "@relateby/gram";

await init();

// Parse Gram notation
const pattern = await Gram.parse("(alice:Person {name: 'Alice'})");

// Serialize to Gram notation
const gramStr = await Gram.stringify(pattern);
```
