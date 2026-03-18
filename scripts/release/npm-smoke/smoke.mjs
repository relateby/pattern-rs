import { Effect, Either, Equal, Option, pipe } from "effect";
import { filterGraph, mapGraph, SpliceGap, toGraphView } from "@relateby/graph";
import { Gram } from "@relateby/gram";
import {
  Pattern,
  StandardGraph,
  Subject,
  Value,
  findFirst,
  fold,
  values,
} from "@relateby/pattern";

const alice = Subject.fromId("alice")
  .withLabel("Person")
  .withProperty("name", Value.String({ value: "Alice" }));
const bob = Subject.fromId("bob").withLabel("Person");

if (!Equal.equals(alice, alice)) {
  throw new Error("Native Subject equality is not available");
}

const relationship = new Pattern({
  value: Subject.fromId("r1").withLabel("KNOWS"),
  elements: [Pattern.point(alice), Pattern.point(bob)],
});

const graph = StandardGraph.fromPatterns([relationship]);
const parsed = await Effect.runPromise(Gram.parse("(alice:Person)-->(bob:Person)"));
const serialized = await Effect.runPromise(Gram.stringify(parsed));
await Effect.runPromise(Gram.validate("(alice:Person)-->(bob:Person)"));

if (graph.nodeCount !== 2 || graph.relationshipCount !== 1) {
  throw new Error("StandardGraph.fromPatterns returned an unexpected graph");
}

if (Option.getOrUndefined(graph.node("alice"))?.value.identity !== "alice") {
  throw new Error("StandardGraph.node did not return the expected node");
}

if (pipe(relationship, fold(0, (acc) => acc + 1)) !== 3) {
  throw new Error("fold did not visit each pattern value");
}

if (pipe(relationship, findFirst((subject) => subject.identity === "bob"))._tag !== "Some") {
  throw new Error("findFirst did not locate the expected subject");
}

const view = toGraphView({
  nodes: [Pattern.point(alice), Pattern.point(bob)],
  relationships: [relationship],
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
    };
  },
  topoSort() {
    return [...this.nodes, ...this.relationships];
  },
});

const filtered = filterGraph((cls) => cls.tag !== "GRelationship", SpliceGap)(view);
const mapped = mapGraph({
  mapNode: (pattern) =>
    new Pattern({
      ...pattern,
      value: pattern.value.withProperty("processed", Value.Bool({ value: true })),
    }),
})(view);

if (filtered.viewElements.length === 0 || mapped.viewElements.length !== view.viewElements.length) {
  throw new Error("@relateby/graph helpers returned an unexpected view");
}

if (!Array.isArray(values(relationship)) || parsed.length !== 1) {
  throw new Error("Native Pattern operations or Gram.parse returned an unexpected result");
}

if (typeof serialized !== "string" || !serialized.includes("alice")) {
  throw new Error("Gram.stringify returned an unexpected result");
}

const stringifyFailure = await Effect.runPromise(
  Effect.either(
    Gram.stringify([
      Pattern.point(
        Subject.fromId("alice").withProperty("nickname", Value.Null({}))
      ),
    ])
  )
);

if (!Either.isLeft(stringifyFailure) || !String(stringifyFailure.left.cause).includes("not representable")) {
  throw new Error("Unsupported null values did not surface a structured stringify error");
}

const parseFailure = await Effect.runPromise(Effect.either(Gram.parse("(alice")));

if (!Either.isLeft(parseFailure) || parseFailure.left.input !== "(alice") {
  throw new Error("Invalid Gram input did not surface a structured public parse error");
}

console.log("npm smoke test passed");
