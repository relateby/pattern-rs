import { Effect, Equal, Option, pipe } from "effect"
import {
  Gram,
  Pattern,
  StandardGraph,
  Subject,
  Value,
  findFirst,
  fold,
  toGraphView,
  values,
} from "@relateby/pattern"

export async function exercisePublicSurface(): Promise<void> {
  const alice = Subject.fromId("alice")
    .withLabel("Person")
    .withProperty("name", Value.String({ value: "Alice" }))
  const bob = Subject.fromId("bob").withLabel("Person")

  const alicePattern = Pattern.point(alice)
  const bobPattern = Pattern.point(bob)
  const relationship = new Pattern({
    value: Subject.fromId("r1").withLabel("KNOWS"),
    elements: [alicePattern, bobPattern],
  })

  const graph = StandardGraph.fromPatterns([relationship])
  const parsed = await Effect.runPromise(Gram.parse("(alice:Person)-->(bob:Person)"))
  const serialized = await Effect.runPromise(Gram.stringify(parsed))
  await Effect.runPromise(Gram.validate("(alice:Person)-->(bob:Person)"))

  const allValues = values(relationship)
  const count = pipe(relationship, fold(0, (acc) => acc + 1))
  const match = pipe(relationship, findFirst((subject) => subject.identity === "bob"))
  const aliceNode = Option.getOrUndefined(graph.node("alice"))
  const graphView = toGraphView({
    nodes: [alicePattern, bobPattern],
    relationships: [],
    walks: [],
    annotations: [],
    conflicts: {},
    size: 2,
    merge: (other) => other,
    topoSort: () => [alicePattern, bobPattern],
  })

  void graph
  void parsed
  void serialized
  void allValues
  void count
  void match
  void aliceNode
  void graphView
  void Equal.equals(alice, alice)
}
