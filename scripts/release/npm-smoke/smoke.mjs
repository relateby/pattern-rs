import {
  Gram,
  GraphClass,
  NativePattern,
  NativeSubject,
  NativeValue,
  StandardGraph,
  init,
} from "@relateby/pattern";

let initFailure = null;
try {
  StandardGraph.fromPatterns([]);
} catch (error) {
  initFailure = error;
}

if (!(initFailure instanceof Error) || !initFailure.message.includes("await init()")) {
  throw new Error("Missing-init public error did not surface through @relateby/pattern");
}

await init();

if (GraphClass.NODE !== "node") {
  throw new Error("GraphClass constants not available");
}

if (typeof Gram?.parse !== "function" || typeof Gram?.stringify !== "function") {
  throw new Error("Gram API not available from @relateby/pattern");
}

if (typeof NativeValue?.string !== "function") {
  throw new Error("NativeValue factory is not available from @relateby/pattern");
}

if (typeof StandardGraph?.fromPatterns !== "function") {
  throw new Error("StandardGraph is not available from @relateby/pattern");
}

const alice = new NativeSubject("alice", ["Person"], {
  name: NativeValue.string("Alice"),
});
const graph = StandardGraph.fromPatterns([NativePattern.point(alice)]);
const parsed = await Gram.parse("(alice:Person)");
const first = await Gram.parseOne("(alice:Person)");
const serialized = await Gram.stringify(first);
const fromGram = StandardGraph.fromGram("(alice:Person)");

if (graph == null || fromGram == null) {
  throw new Error("StandardGraph.fromPatterns returned nullish");
}
if (!Array.isArray(parsed) || parsed.length !== 1) {
  throw new Error("Gram.parse returned an unexpected result");
}
if (typeof serialized !== "string" || !serialized.includes("alice")) {
  throw new Error("Gram.stringify returned an unexpected result");
}
if (fromGram.nodeCount !== 1) {
  throw new Error("StandardGraph.fromGram returned an unexpected graph");
}

let parseFailure = null;
try {
  await Gram.parse("(alice");
} catch (error) {
  parseFailure = error;
}

if (!(parseFailure instanceof Error) || !parseFailure.message.includes("Gram.parse")) {
  throw new Error("Invalid Gram input did not surface a public parse error");
}

console.log("npm smoke test passed");
