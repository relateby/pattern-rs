import { Effect, Equal, Option, pipe } from "effect";
import {
  Gram,
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

if (!Array.isArray(values(relationship)) || parsed.length !== 1) {
  throw new Error("Native Pattern operations or Gram.parse returned an unexpected result");
}

if (typeof serialized !== "string" || !serialized.includes("alice")) {
  throw new Error("Gram.stringify returned an unexpected result");
}

let parseFailure = null;
try {
  await Effect.runPromise(Gram.parse("(alice"));
} catch (error) {
  parseFailure = error;
}

if (!(parseFailure instanceof Error) || parseFailure.input !== "(alice")) {
  throw new Error("Invalid Gram input did not surface a structured public parse error");
}

console.log("npm smoke test passed");
