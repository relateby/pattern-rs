# TypeScript Graph API Reference

Use the single supported package boundary:

```bash
npm install @relateby/pattern
```

Everything in this guide comes from `@relateby/pattern`, including the WASM-backed graph types, the `Gram` facade, and the pure TypeScript graph utilities.

## Initialization

In Node.js, call `init()` before using WASM-backed types:

```typescript
import { init, NativePatternGraph } from "@relateby/pattern";

await init();
const graph = NativePatternGraph.empty();
```

Bundlers can usually rely on the generated module init path, but calling `await init()` is always safe.

## Public Workflow

```typescript
import {
  Gram,
  StandardGraph,
  TraversalDirection,
  bfs,
  init,
  NativeGraphQuery,
  NativePattern,
  NativePatternGraph,
  NativeReconciliationPolicy,
  NativeSubject,
  NativeValue,
  toGraphView,
} from "@relateby/pattern";

await init();

const alice = NativePattern.point(
  new NativeSubject("alice", ["Person"], { active: NativeValue.bool(true) })
);
const bob = NativePattern.point(new NativeSubject("bob", ["Person"], {}));
const knows = NativePattern.pattern(new NativeSubject("r1", ["KNOWS"], {}));
knows.addElement(alice);
knows.addElement(bob);

const nativeGraph = NativePatternGraph.fromPatterns(
  [alice, bob, knows],
  NativeReconciliationPolicy.lastWriteWins()
);
const query = NativeGraphQuery.fromPatternGraph(nativeGraph);
const traversal = bfs(query, alice);

const parsed = await Gram.parse("(alice:Person)-[:KNOWS]->(bob:Person)");
const standardGraph = StandardGraph.fromPatterns(parsed as never[]);

console.log(traversal.length);
console.log(standardGraph.nodeCount);
console.log(TraversalDirection.FORWARD);
console.log(toGraphView(nativeGraph));
```

## Export Families

`@relateby/pattern` exposes these public families:

- `init`
- `NativeSubject`, `NativePattern`, `NativeValue`, `NativeValidationRules`
- `NativePatternGraph`, `NativeGraphQuery`, `NativeReconciliationPolicy`
- `StandardGraph`
- `Gram`
- `GraphClass`, `TraversalDirection`
- Pure TypeScript graph helpers such as `toGraphView`, `mapGraph`, `filterGraph`, `foldGraph`, `paraGraph`, and `unfoldGraph`

## Gram

The package-level `Gram` namespace is the supported parser/serializer entry point:

```typescript
const allPatterns = await Gram.parse("(alice:Person) (bob:Person)");
const firstPattern = await Gram.parseOne("(alice:Person)");
const serialized = await Gram.stringify(firstPattern);
```

## StandardGraph

Use `StandardGraph` when you want a higher-level graph workflow from the same package boundary:

```typescript
const graph = StandardGraph.fromGram("(alice:Person)-[:KNOWS]->(bob:Person)");
console.log(graph.nodeCount);
console.log(graph.relationshipCount);
console.log(graph.node("alice"));
```

## Pure TypeScript Utilities

Graph utilities remain available from the same package:

```typescript
import { filterGraph, mapGraph, toGraphView } from "@relateby/pattern";

const view = toGraphView(nativeGraph);
const mapped = mapGraph({ mapNode: (pattern) => pattern })(view);
const filtered = filterGraph(() => true)(view);

console.log(mapped, filtered);
```
