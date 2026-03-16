/**
 * StandardGraph WASM/Node.js example (T042)
 *
 * Demonstrates using the StandardGraph binding from Node.js.
 * Run after ci-local.sh builds the wasm-node package:
 *   node examples/pattern-core-wasm/standard_graph.mjs
 *
 * The wasm-node build uses CommonJS; imports via createRequire.
 */

import { createRequire } from "module";
const require = createRequire(import.meta.url);
const wasm = require("../../typescript/@relateby/pattern/wasm-node/pattern_wasm.js");
const { StandardGraph, WasmSubject: Subject } = wasm;

// --- Build a small social graph ---
const g = new StandardGraph();

const alice = new Subject("alice", ["Person"], { name: "Alice" });
const bob = new Subject("bob", ["Person"], { name: "Bob" });
const rel = new Subject("r1", ["KNOWS"], {});

g.addNode(alice);
g.addNode(bob);
// Pass Subject objects directly — no need to spell out identity strings
g.addRelationship(rel, alice, bob);

console.log(`Node count: ${g.nodeCount}`);                        // 2
console.log(`Relationship count: ${g.relationshipCount}`);        // 1
console.log(`Source of r1: ${g.source("r1")}`);
console.log(`Target of r1: ${g.target("r1")}`);

// --- fromGram ---
const g2 = StandardGraph.fromGram("(carol:Person {name:'Carol'})-[:KNOWS]->(dave:Person {name:'Dave'})");
console.log(`fromGram node count: ${g2.nodeCount}`);
console.log(`fromGram relationship count: ${g2.relationshipCount}`);

// --- Subject.build() fluent API ---
const builder = Subject.build("eve");
builder.label("Person");
builder.property("name", "Eve");
const eve = builder.done();
g.addNode(eve);
console.log(`After adding Eve: ${g.nodeCount} nodes`);

console.log("✓ StandardGraph WASM example complete");
