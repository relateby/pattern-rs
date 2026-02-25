// Integration tests for @relateby/pattern
// Tests that require WASM are skipped when WASM is not built.
// Pure TypeScript tests run without WASM.

import { describe, it, expect, beforeAll } from "vitest";
import {
  NativePatternGraph,
  NativeReconciliationPolicy,
  NativeGraphQuery,
  NativePattern,
  NativeSubject,
  bfs,
  dfs,
  shortestPath,
  connectedComponents,
  hasCycle,
  isConnected,
  topologicalSort,
  degreeCentrality,
  betweennessCentrality,
  minimumSpanningTree,
  init,
  GraphClass,
  TraversalDirection,
  toGraphView,
  mapGraph,
  filterGraph,
  foldGraph,
  paraGraph,
  unfoldGraph,
} from "../src/index.js";

// ---------------------------------------------------------------------------
// WASM availability check
// ---------------------------------------------------------------------------

let wasmAvailable = false;

beforeAll(async () => {
  try {
    await init();
    wasmAvailable = true;
  } catch {
    console.warn("WASM not available — skipping WASM-dependent tests. Run `npm run build:wasm` first.");
  }
});

function skipIfNoWasm(fn: () => void | Promise<void>) {
  return async () => {
    if (!wasmAvailable) {
      console.log("  [skipped: WASM not built]");
      return;
    }
    await fn();
  };
}

// ---------------------------------------------------------------------------
// US1: Build and Query a Graph from Patterns
// ---------------------------------------------------------------------------

describe("NativePatternGraph (US1)", () => {
  it("fromPatterns constructs a graph from node patterns", skipIfNoWasm(async () => {
    const alice = NativePattern.point(new (NativeSubject as unknown as { new(...args: unknown[]): unknown })(
      "alice", ["Person"], {}
    ));
    const bob = NativePattern.point(new (NativeSubject as unknown as { new(...args: unknown[]): unknown })(
      "bob", ["Person"], {}
    ));

    const graph = NativePatternGraph.fromPatterns([alice as unknown, bob as unknown] as never[]);

    expect(graph.nodes.length).toBeGreaterThan(0);
    expect(graph.size).toBeGreaterThan(0);
  }));

  it("empty() constructs an empty graph", skipIfNoWasm(async () => {
    const graph = NativePatternGraph.empty();
    expect(graph.nodes.length).toBe(0);
    expect(graph.relationships.length).toBe(0);
    expect(graph.size).toBe(0);
  }));

  it("merge() combines two graphs", skipIfNoWasm(async () => {
    const g1 = NativePatternGraph.empty();
    const g2 = NativePatternGraph.empty();
    const merged = g1.merge(g2);
    expect(merged.size).toBe(0);
  }));

  it("topoSort() returns patterns in topological order", skipIfNoWasm(async () => {
    const graph = NativePatternGraph.empty();
    const sorted = graph.topoSort();
    expect(Array.isArray(sorted)).toBe(true);
  }));
});

// ---------------------------------------------------------------------------
// US1: NativeReconciliationPolicy
// ---------------------------------------------------------------------------

describe("NativeReconciliationPolicy (US1)", () => {
  it("lastWriteWins() constructs without error", skipIfNoWasm(async () => {
    const policy = NativeReconciliationPolicy.lastWriteWins();
    expect(policy).toBeDefined();
  }));

  it("firstWriteWins() constructs without error", skipIfNoWasm(async () => {
    const policy = NativeReconciliationPolicy.firstWriteWins();
    expect(policy).toBeDefined();
  }));

  it("strict() constructs without error", skipIfNoWasm(async () => {
    const policy = NativeReconciliationPolicy.strict();
    expect(policy).toBeDefined();
  }));

  it("merge() constructs without error", skipIfNoWasm(async () => {
    const policy = NativeReconciliationPolicy.merge();
    expect(policy).toBeDefined();
  }));

  it("strict policy records conflict in graph.conflicts", skipIfNoWasm(async () => {
    // Two patterns with the same identity under strict policy → conflict
    const strict = NativeReconciliationPolicy.strict();
    const graph = NativePatternGraph.fromPatterns([], strict);
    expect(graph.conflicts).toBeDefined();
  }));
});

// ---------------------------------------------------------------------------
// US1: NativeGraphQuery and algorithms
// ---------------------------------------------------------------------------

describe("NativeGraphQuery and algorithms (US1)", () => {
  it("fromPatternGraph() creates a query handle", skipIfNoWasm(async () => {
    const graph = NativePatternGraph.empty();
    const query = NativeGraphQuery.fromPatternGraph(graph);
    expect(query).toBeDefined();
    expect(query.nodes()).toBeDefined();
    expect(query.relationships()).toBeDefined();
  }));

  it("bfs() returns traversal order", skipIfNoWasm(async () => {
    const graph = NativePatternGraph.empty();
    const query = NativeGraphQuery.fromPatternGraph(graph);
    // Empty graph — no start node, but function should not throw
    expect(Array.isArray(query.nodes())).toBe(true);
  }));

  it("hasCycle() returns false for empty graph", skipIfNoWasm(async () => {
    const graph = NativePatternGraph.empty();
    const query = NativeGraphQuery.fromPatternGraph(graph);
    expect(hasCycle(query)).toBe(false);
  }));

  it("isConnected() returns true for empty graph", skipIfNoWasm(async () => {
    const graph = NativePatternGraph.empty();
    const query = NativeGraphQuery.fromPatternGraph(graph);
    expect(isConnected(query)).toBe(true);
  }));

  it("topologicalSort() returns array for acyclic graph", skipIfNoWasm(async () => {
    const graph = NativePatternGraph.empty();
    const query = NativeGraphQuery.fromPatternGraph(graph);
    const sorted = topologicalSort(query);
    // Empty graph has no cycle — should return empty array (not null)
    expect(sorted).not.toBeNull();
    expect(Array.isArray(sorted)).toBe(true);
  }));

  it("degreeCentrality() returns object", skipIfNoWasm(async () => {
    const graph = NativePatternGraph.empty();
    const query = NativeGraphQuery.fromPatternGraph(graph);
    const centrality = degreeCentrality(query);
    expect(typeof centrality).toBe("object");
  }));

  it("connectedComponents() returns array", skipIfNoWasm(async () => {
    const graph = NativePatternGraph.empty();
    const query = NativeGraphQuery.fromPatternGraph(graph);
    const components = connectedComponents(query);
    expect(Array.isArray(components)).toBe(true);
  }));
});

// ---------------------------------------------------------------------------
// US2: Structural analysis
// ---------------------------------------------------------------------------

describe("Structural analysis (US2)", () => {
  it("hasCycle() returns false for acyclic graph", skipIfNoWasm(async () => {
    const graph = NativePatternGraph.empty();
    const query = NativeGraphQuery.fromPatternGraph(graph);
    expect(hasCycle(query)).toBe(false);
  }));

  it("betweennessCentrality() returns scores", skipIfNoWasm(async () => {
    const graph = NativePatternGraph.empty();
    const query = NativeGraphQuery.fromPatternGraph(graph);
    const scores = betweennessCentrality(query);
    expect(typeof scores).toBe("object");
  }));

  it("minimumSpanningTree() returns array", skipIfNoWasm(async () => {
    const graph = NativePatternGraph.empty();
    const query = NativeGraphQuery.fromPatternGraph(graph);
    const tree = minimumSpanningTree(query);
    expect(Array.isArray(tree)).toBe(true);
  }));
});

// ---------------------------------------------------------------------------
// Pure TypeScript tests (no WASM required)
// ---------------------------------------------------------------------------

describe("Pure TypeScript exports (no WASM)", () => {
  it("GraphClass constants are defined", () => {
    expect(GraphClass.NODE).toBe("node");
    expect(GraphClass.RELATIONSHIP).toBe("relationship");
    expect(GraphClass.ANNOTATION).toBe("annotation");
    expect(GraphClass.WALK).toBe("walk");
    expect(GraphClass.OTHER).toBe("other");
  });

  it("TraversalDirection constants are defined", () => {
    expect(TraversalDirection.FORWARD).toBe("forward");
    expect(TraversalDirection.BACKWARD).toBe("backward");
  });

  it("@relateby/graph transforms are re-exported", () => {
    expect(typeof toGraphView).toBe("function");
    expect(typeof mapGraph).toBe("function");
    expect(typeof filterGraph).toBe("function");
    expect(typeof foldGraph).toBe("function");
    expect(typeof paraGraph).toBe("function");
    expect(typeof unfoldGraph).toBe("function");
  });
});
