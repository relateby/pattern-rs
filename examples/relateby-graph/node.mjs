/**
 * relateby-graph Node.js example
 *
 * Demonstrates:
 * 1. Building a graph from NativePattern/NativeSubject (WASM-backed)
 * 2. Running BFS traversal
 * 3. Computing degree centrality
 * 4. Applying mapGraph + filterGraph pipeline via @relateby/graph
 *
 * Prerequisites:
 *   cd typescript/@relateby/pattern && npm run build:wasm && npm run build:ts
 *   cd examples/relateby-graph && npm install
 *
 * Run:
 *   node node.mjs
 */

import {
  init,
  NativeSubject,
  NativePattern,
  NativePatternGraph,
  NativeGraphQuery,
  NativeReconciliationPolicy,
  bfs,
  degreeCentrality,
} from "@relateby/pattern";

import {
  toGraphView,
  mapGraph,
  filterGraph,
  SpliceGap,
} from "@relateby/graph";

// ---------------------------------------------------------------------------
// Initialize WASM
// ---------------------------------------------------------------------------

try {
  await init();
} catch (e) {
  console.error("Failed to initialize WASM:", e.message);
  console.error("Run: cd typescript/@relateby/pattern && npm run build:wasm && npm run build:ts");
  process.exit(1);
}

// ---------------------------------------------------------------------------
// Build a graph
// ---------------------------------------------------------------------------

// Create subjects
const aliceSubject = new NativeSubject("alice", ["Person"], { name: "Alice" });
const bobSubject = new NativeSubject("bob", ["Person"], { name: "Bob" });
const charlieSubject = new NativeSubject("charlie", ["Person"], { name: "Charlie" });

// Create node patterns
const alice = NativePattern.point(aliceSubject);
const bob = NativePattern.point(bobSubject);
const charlie = NativePattern.point(charlieSubject);

// Create relationship patterns (source → target as elements)
const aliceKnowsBob = NativePattern.pattern(
  new NativeSubject("r1", ["KNOWS"], {})
);
aliceKnowsBob.addElement(alice);
aliceKnowsBob.addElement(bob);

const bobKnowsCharlie = NativePattern.pattern(
  new NativeSubject("r2", ["KNOWS"], {})
);
bobKnowsCharlie.addElement(bob);
bobKnowsCharlie.addElement(charlie);

// Build the graph
const graph = NativePatternGraph.fromPatterns(
  [alice, bob, charlie, aliceKnowsBob, bobKnowsCharlie],
  NativeReconciliationPolicy.lastWriteWins()
);

console.log(`Graph: ${graph.nodes.length} nodes, ${graph.relationships.length} relationships`);

// ---------------------------------------------------------------------------
// Query the graph
// ---------------------------------------------------------------------------

const query = NativeGraphQuery.fromPatternGraph(graph);

// BFS from alice
const aliceNode = query.nodeById("alice");
if (aliceNode) {
  const traversal = bfs(query, aliceNode);
  const ids = traversal.map((p) => p.identity ?? "?");
  console.log(`BFS from alice: ${ids.join(", ")}`);
}

// Degree centrality
const centrality = degreeCentrality(query);
console.log("Degree centrality:", centrality);

// ---------------------------------------------------------------------------
// Pure TypeScript transforms via @relateby/graph
// ---------------------------------------------------------------------------

// Convert to GraphView for transform pipeline
const view = toGraphView(graph);

// Filter: keep only Person nodes
const personView = filterGraph(
  (cls, p) => {
    if (cls.tag === "GNode") {
      return p.value?.labels?.has?.("Person") ?? false;
    }
    return cls.tag !== "GNode"; // keep non-nodes
  },
  SpliceGap
)(view);

const personNodes = personView.viewElements.filter(([cls]) => cls.tag === "GNode");
console.log(`Nodes after filter: ${personNodes.length} (Person nodes only)`);

// Map: add a "processed" property to all nodes
const processedView = mapGraph({
  mapNode: (p) => ({
    ...p,
    identity: `processed:${p.identity}`,
  }),
})(view);

const processedIds = processedView.viewElements
  .filter(([cls]) => cls.tag === "GNode")
  .map(([, p]) => p.identity);
console.log("Processed node IDs:", processedIds);
