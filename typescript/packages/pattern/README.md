# `@relateby/pattern`

Native TypeScript `Pattern`, `Subject`, and `StandardGraph` APIs backed by the Rust gram codec's JSON interchange layer.

## Install

```bash
npm install @relateby/pattern effect
```

## Quick Start

```typescript
import { Effect, Equal, Option, pipe } from "effect"
import { Gram, Pattern, StandardGraph, Subject, Value, findFirst, fold } from "@relateby/pattern"

const alice = Subject.fromId("alice")
  .withLabel("Person")
  .withProperty("name", Value.String({ value: "Alice" }))
const bob = Subject.fromId("bob").withLabel("Person")

const relationship = new Pattern({
  value: Subject.fromId("r1").withLabel("KNOWS"),
  elements: [Pattern.point(alice), Pattern.point(bob)],
})

const graph = StandardGraph.fromPatterns([relationship])
const parsed = await Effect.runPromise(Gram.parse("(alice:Person)-->(bob:Person)"))

console.log(graph.nodeCount)
console.log(Option.getOrUndefined(graph.node("alice"))?.value.identity)
console.log(pipe(relationship, fold(0, (acc) => acc + 1)))
console.log(pipe(relationship, findFirst((subject) => subject.identity === "bob")))
console.log(Equal.equals(parsed[0]?.value, alice))
```

## Gram Effects

`Gram.parse`, `Gram.stringify`, and `Gram.validate` return `Effect` values:

```typescript
import { Effect, pipe } from "effect"
import { Gram } from "@relateby/pattern"

const rendered = await pipe(
  Gram.parse("(alice:Person)"),
  Effect.flatMap((patterns) => Gram.stringify(patterns)),
  Effect.runPromise
)
```

Use `Option.getOrUndefined()` for lookups and `Equal.equals()` for structural equality.
