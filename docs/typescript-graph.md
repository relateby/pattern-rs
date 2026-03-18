# TypeScript Graph API Reference

Use the single supported package boundary:

```bash
npm install @relateby/pattern effect
```

`@relateby/pattern` now exposes:

- native TypeScript `Pattern`, `Subject`, `Value`, and `StandardGraph`
- pure TypeScript graph transforms such as `toGraphView`, `mapGraph`, and `filterGraph`
- the Gram codec via `Gram`, backed by the Rust/WASM JSON interchange layer

## Gram

`Gram.parse`, `Gram.stringify`, and `Gram.validate` return `Effect` values:

```typescript
import { Effect } from "effect"
import { Gram } from "@relateby/pattern"

const patterns = await Effect.runPromise(
  Gram.parse("(alice:Person)-[:KNOWS]->(bob:Person)")
)

const rendered = await Effect.runPromise(Gram.stringify(patterns))
await Effect.runPromise(Gram.validate(rendered))
```

## StandardGraph

Use `StandardGraph` for graph classification and lookup over native `Pattern<Subject>` values:

```typescript
import { Effect, Option } from "effect"
import { Gram, StandardGraph } from "@relateby/pattern"

const graph = await Effect.runPromise(
  Effect.map(
    Gram.parse("(alice:Person)-[:KNOWS]->(bob:Person)"),
    StandardGraph.fromPatterns
  )
)

console.log(graph.nodeCount)
console.log(graph.relationshipCount)
console.log(Option.getOrUndefined(graph.node("alice"))?.value.identity)
```

`StandardGraph.fromGram(input)` is also available and returns `Effect<StandardGraph, GramParseError>`.

## Pure TypeScript Graph Utilities

The package also exports the graph view and transform helpers:

```typescript
import { DeleteContainer, Pattern, Subject, SpliceGap, filterGraph, toGraphView } from "@relateby/pattern"

const alice = Pattern.point(Subject.fromId("alice").withLabel("Person"))
const bob = Pattern.point(Subject.fromId("bob").withLabel("Person"))
const knows = new Pattern({
  value: Subject.fromId("r1").withLabel("KNOWS"),
  elements: [alice, bob],
})

const view = toGraphView({
  nodes: [alice, bob],
  relationships: [knows],
  walks: [],
  annotations: [],
  conflicts: {},
  size: 3,
  merge(other) {
    return {
      ...this,
      nodes: [...this.nodes, ...other.nodes],
      relationships: [...this.relationships, ...other.relationships],
      size: this.size + other.size,
    }
  },
  topoSort() {
    return [...this.nodes, ...this.relationships]
  },
})

const withoutBob = filterGraph((_cls, pattern) => pattern.identity !== "bob", SpliceGap)(view)
const withoutRelationships = filterGraph((cls) => cls.tag !== "GRelationship", DeleteContainer)(view)

console.log(withoutBob.viewElements.length)
console.log(withoutRelationships.viewElements.length)
```
