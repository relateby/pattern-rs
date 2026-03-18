/**
 * StandardGraph WASM/Node.js integration tests (T041)
 *
 * Run after building with wasm-pack (or ci-local.sh):
 *   node examples/pattern-core-wasm/test_standard_graph.mjs
 *
 * The wasm-node build uses CommonJS; imports via createRequire.
 */

import { createRequire } from "module";
const require = createRequire(import.meta.url);
const wasm = require("../../typescript/@relateby/pattern/wasm-node/pattern_wasm.js");
// The wasm-node package exposes Wasm-prefixed names for core types
// and the re-exported StandardGraph wrapper from pattern-wasm.
const { StandardGraph, WasmSubject: Subject } = wasm;

let passed = 0;
let failed = 0;

function assert(condition, msg) {
  if (condition) {
    console.log(`  ✓ ${msg}`);
    passed++;
  } else {
    console.error(`  ✗ FAIL: ${msg}`);
    failed++;
  }
}

function assertEqual(a, b, msg) {
  assert(a === b, `${msg} (expected ${b}, got ${a})`);
}

// --- Constructors ---
console.log("Constructors:");
{
  const g = new StandardGraph();
  assert(g.isEmpty, "new() is empty");
  assertEqual(g.nodeCount, 0, "nodeCount is 0");
  assertEqual(g.relationshipCount, 0, "relationshipCount is 0");
}

// --- addNode / nodeCount ---
console.log("\naddNode:");
{
  const g = new StandardGraph();
  const alice = new Subject("alice", ["Person"], { name: "Alice" });
  g.addNode(alice);
  assertEqual(g.nodeCount, 1, "nodeCount after addNode");
  assert(!g.isEmpty, "not empty after addNode");
}

// --- addRelationship / relationshipCount ---
console.log("\naddRelationship:");
{
  const g = new StandardGraph();
  const alice = new Subject("alice", ["Person"], {});
  const bob = new Subject("bob", ["Person"], {});
  const rel = new Subject("r1", ["KNOWS"], {});
  g.addNode(alice);
  g.addNode(bob);
  // Pass Subject objects directly
  g.addRelationship(rel, alice, bob);
  assertEqual(g.relationshipCount, 1, "relationshipCount after addRelationship");
}

// --- source / target ---
console.log("\nsource / target:");
{
  const g = new StandardGraph();
  const alice = new Subject("alice", ["Person"], {});
  const bob = new Subject("bob", ["Person"], {});
  g.addNode(alice);
  g.addNode(bob);
  g.addRelationship(new Subject("r1", ["KNOWS"], {}), alice, bob);
  const src = g.source("r1");
  const tgt = g.target("r1");
  assert(src !== null && src !== undefined, "source(r1) returns a value");
  assert(tgt !== null && tgt !== undefined, "target(r1) returns a value");
}

// --- fromGram ---
console.log("\nfromGram:");
{
  const g = StandardGraph.fromGram("(alice:Person) (bob:Person)");
  assertEqual(g.nodeCount, 2, "fromGram 2 nodes");
}
{
  const g = StandardGraph.fromGram("(a:Person)-[:KNOWS]->(b:Person)");
  assert(g.nodeCount >= 2, "fromGram relationship creates nodes");
  assert(g.relationshipCount >= 1, "fromGram creates relationship");
}
{
  const g = StandardGraph.fromGram("");
  assert(g.isEmpty, "fromGram empty string => empty graph");
}

// --- WasmSubject.build() fluent API ---
console.log("\nSubject.build() fluent API:");
{
  const builder = Subject.build("carol");
  builder.label("Person");
  builder.property("name", "Carol");
  const carol = builder.done();
  assert(carol instanceof Subject, "builder.done() returns Subject");
  assertEqual(carol.identity, "carol", "identity");
}

// --- Summary ---
console.log(`\nResults: ${passed} passed, ${failed} failed`);
if (failed > 0) {
  process.exit(1);
}
