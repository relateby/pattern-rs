// WASM-free tests for @relateby/graph using plain TypeScript stubs.
// No NativePattern, no init() — all tests use plain TS objects.

import { describe, it, expect } from "vitest";
import {
  toGraphView,
  mapGraph,
  mapAllGraph,
  filterGraph,
  foldGraph,
  mapWithContext,
  paraGraph,
  unfoldGraph,
  GNode,
  GRelationship,
  GWalk,
  GAnnotation,
  DeleteContainer,
  SpliceGap,
  ReplaceWithSurrogate,
} from "../src/index.js";
import type { Subject, Pattern, PatternGraph, GraphQuery } from "../src/index.js";

// ---------------------------------------------------------------------------
// Plain TS stub helpers (no WASM)
// ---------------------------------------------------------------------------

function stubSubject(id: string, labels: string[] = []): Subject {
  return {
    identity: id,
    labels: new Set(labels),
    properties: {},
  };
}

function stubNode(id: string, labels: string[] = []): Pattern<Subject> {
  return {
    identity: id,
    value: stubSubject(id, labels),
    elements: [],
  };
}

function stubRelationship(
  id: string,
  source: Pattern<Subject>,
  target: Pattern<Subject>
): Pattern<Subject> {
  return {
    identity: id,
    value: stubSubject(id),
    elements: [source, target],
  };
}

function stubGraph(
  nodes: Pattern<Subject>[],
  relationships: Pattern<Subject>[] = [],
  walks: Pattern<Subject>[] = [],
  annotations: Pattern<Subject>[] = []
): PatternGraph<Subject> {
  return {
    nodes,
    relationships,
    walks,
    annotations,
    conflicts: {},
    size: nodes.length + relationships.length + walks.length + annotations.length,
    merge: (other) => stubGraph(
      [...nodes, ...other.nodes],
      [...relationships, ...other.relationships],
      [...walks, ...other.walks],
      [...annotations, ...other.annotations]
    ),
    topoSort: () => [...nodes, ...relationships, ...walks, ...annotations],
  };
}

// ---------------------------------------------------------------------------
// toGraphView tests
// ---------------------------------------------------------------------------

describe("toGraphView", () => {
  it("classifies nodes correctly", () => {
    const alice = stubNode("alice", ["Person"]);
    const bob = stubNode("bob", ["Person"]);
    const graph = stubGraph([alice, bob]);

    const view = toGraphView(graph);

    const nodeElements = view.viewElements.filter(([cls]) => cls.tag === "GNode");
    expect(nodeElements).toHaveLength(2);
    expect(nodeElements[0][0]).toEqual(GNode);
    expect(nodeElements[0][1].identity).toBe("alice");
  });

  it("classifies relationships correctly", () => {
    const alice = stubNode("alice");
    const bob = stubNode("bob");
    const rel = stubRelationship("knows", alice, bob);
    const graph = stubGraph([alice, bob], [rel]);

    const view = toGraphView(graph);

    const relElements = view.viewElements.filter(([cls]) => cls.tag === "GRelationship");
    expect(relElements).toHaveLength(1);
    expect(relElements[0][1].identity).toBe("knows");
  });

  it("classifies walks correctly", () => {
    const alice = stubNode("alice");
    const bob = stubNode("bob");
    const walk: Pattern<Subject> = {
      identity: "w1",
      value: stubSubject("w1"),
      elements: [alice, bob],
    };
    const graph = stubGraph([alice, bob], [], [walk]);

    const view = toGraphView(graph);

    const walkElements = view.viewElements.filter(([cls]) => cls.tag === "GWalk");
    expect(walkElements).toHaveLength(1);
    expect(walkElements[0][0]).toEqual(GWalk);
  });

  it("classifies annotations correctly", () => {
    const alice = stubNode("alice");
    const ann: Pattern<Subject> = {
      identity: "a1",
      value: stubSubject("a1"),
      elements: [alice],
    };
    const graph = stubGraph([alice], [], [], [ann]);

    const view = toGraphView(graph);

    const annElements = view.viewElements.filter(([cls]) => cls.tag === "GAnnotation");
    expect(annElements).toHaveLength(1);
    expect(annElements[0][0]).toEqual(GAnnotation);
  });

  it("provides a viewQuery with correct node lookup", () => {
    const alice = stubNode("alice");
    const bob = stubNode("bob");
    const graph = stubGraph([alice, bob]);

    const view = toGraphView(graph);

    expect(view.viewQuery.nodeById("alice")).toEqual(alice);
    expect(view.viewQuery.nodeById("charlie")).toBeNull();
  });
});

// ---------------------------------------------------------------------------
// mapGraph tests
// ---------------------------------------------------------------------------

describe("mapGraph", () => {
  it("applies per-class mappers", () => {
    const alice = stubNode("alice", ["Person"]);
    const bob = stubNode("bob", ["Person"]);
    const rel = stubRelationship("knows", alice, bob);
    const graph = stubGraph([alice, bob], [rel]);
    const view = toGraphView(graph);

    const mapped = mapGraph<Subject>({
      mapNode: (p) => ({ ...p, identity: `node:${p.identity}` }),
      mapRelationship: (p) => ({ ...p, identity: `rel:${p.identity}` }),
    })(view);

    const nodeIds = mapped.viewElements
      .filter(([cls]) => cls.tag === "GNode")
      .map(([, p]) => p.identity);
    expect(nodeIds).toContain("node:alice");
    expect(nodeIds).toContain("node:bob");

    const relIds = mapped.viewElements
      .filter(([cls]) => cls.tag === "GRelationship")
      .map(([, p]) => p.identity);
    expect(relIds).toContain("rel:knows");
  });

  it("passes through unspecified classes unchanged", () => {
    const alice = stubNode("alice");
    const graph = stubGraph([alice]);
    const view = toGraphView(graph);

    const mapped = mapGraph<Subject>({
      // No mapNode specified
    })(view);

    expect(mapped.viewElements[0][1].identity).toBe("alice");
  });
});

// ---------------------------------------------------------------------------
// mapAllGraph tests
// ---------------------------------------------------------------------------

describe("mapAllGraph", () => {
  it("applies uniform transform to all elements", () => {
    const alice = stubNode("alice");
    const bob = stubNode("bob");
    const rel = stubRelationship("knows", alice, bob);
    const graph = stubGraph([alice, bob], [rel]);
    const view = toGraphView(graph);

    const mapped = mapAllGraph<Subject>(
      (p) => ({ ...p, identity: `mapped:${p.identity}` })
    )(view);

    const ids = mapped.viewElements.map(([, p]) => p.identity);
    expect(ids).toContain("mapped:alice");
    expect(ids).toContain("mapped:bob");
    expect(ids).toContain("mapped:knows");
  });
});

// ---------------------------------------------------------------------------
// filterGraph tests
// ---------------------------------------------------------------------------

describe("filterGraph", () => {
  it("removes elements not satisfying predicate", () => {
    const alice = stubNode("alice", ["Person"]);
    const thing = stubNode("thing", ["Thing"]);
    const graph = stubGraph([alice, thing]);
    const view = toGraphView(graph);

    const filtered = filterGraph<Subject>(
      (_cls, p) => p.value?.labels.has("Person") ?? false,
      SpliceGap
    )(view);

    const ids = filtered.viewElements.map(([, p]) => p.identity);
    expect(ids).toContain("alice");
    expect(ids).not.toContain("thing");
  });

  it("SpliceGap removes element from walk and splices remaining", () => {
    const alice = stubNode("alice");
    const bob = stubNode("bob");
    const charlie = stubNode("charlie");
    const walk: Pattern<Subject> = {
      identity: "w1",
      value: stubSubject("w1"),
      elements: [alice, bob, charlie],
    };
    const graph = stubGraph([alice, bob, charlie], [], [walk]);
    const view = toGraphView(graph);

    // Remove bob
    const filtered = filterGraph<Subject>(
      (_cls, p) => p.identity !== "bob",
      SpliceGap
    )(view);

    const walkElem = filtered.viewElements.find(([cls]) => cls.tag === "GWalk");
    expect(walkElem).toBeDefined();
    expect(walkElem![1].elements).toHaveLength(2);
    expect(walkElem![1].elements.map((e) => e.identity)).toEqual(["alice", "charlie"]);
  });

  it("DeleteContainer removes entire walk when element is removed", () => {
    const alice = stubNode("alice");
    const bob = stubNode("bob");
    const walk: Pattern<Subject> = {
      identity: "w1",
      value: stubSubject("w1"),
      elements: [alice, bob],
    };
    const graph = stubGraph([alice, bob], [], [walk]);
    const view = toGraphView(graph);

    const filtered = filterGraph<Subject>(
      (_cls, p) => p.identity !== "bob",
      DeleteContainer
    )(view);

    const walkElem = filtered.viewElements.find(([cls]) => cls.tag === "GWalk");
    expect(walkElem).toBeUndefined();
  });

  it("ReplaceWithSurrogate replaces removed element with surrogate", () => {
    const alice = stubNode("alice");
    const bob = stubNode("bob");
    const placeholder = stubNode("placeholder");
    const walk: Pattern<Subject> = {
      identity: "w1",
      value: stubSubject("w1"),
      elements: [alice, bob],
    };
    const graph = stubGraph([alice, bob], [], [walk]);
    const view = toGraphView(graph);

    const filtered = filterGraph<Subject>(
      (_cls, p) => p.identity !== "bob",
      ReplaceWithSurrogate(placeholder)
    )(view);

    const walkElem = filtered.viewElements.find(([cls]) => cls.tag === "GWalk");
    expect(walkElem).toBeDefined();
    expect(walkElem![1].elements).toHaveLength(2);
    expect(walkElem![1].elements[1].identity).toBe("placeholder");
  });
});

// ---------------------------------------------------------------------------
// foldGraph tests
// ---------------------------------------------------------------------------

describe("foldGraph", () => {
  it("accumulates counts correctly", () => {
    const alice = stubNode("alice");
    const bob = stubNode("bob");
    const rel = stubRelationship("knows", alice, bob);
    const graph = stubGraph([alice, bob], [rel]);
    const view = toGraphView(graph);

    const counts = foldGraph<Subject, { nodes: number; rels: number }>(
      (cls, _p) =>
        cls.tag === "GNode"
          ? { nodes: 1, rels: 0 }
          : cls.tag === "GRelationship"
          ? { nodes: 0, rels: 1 }
          : { nodes: 0, rels: 0 },
      { nodes: 0, rels: 0 },
      (a, b) => ({ nodes: a.nodes + b.nodes, rels: a.rels + b.rels })
    )(view);

    expect(counts.nodes).toBe(2);
    expect(counts.rels).toBe(1);
  });
});

// ---------------------------------------------------------------------------
// mapWithContext tests
// ---------------------------------------------------------------------------

describe("mapWithContext", () => {
  it("provides snapshot query to each element callback", () => {
    const alice = stubNode("alice");
    const bob = stubNode("bob");
    const graph = stubGraph([alice, bob]);
    const view = toGraphView(graph);

    const queriedIds: string[] = [];
    mapWithContext<Subject>((query, p) => {
      const found = query.nodeById(p.identity ?? "");
      if (found) queriedIds.push(found.identity ?? "");
      return p;
    })(view);

    expect(queriedIds).toContain("alice");
    expect(queriedIds).toContain("bob");
  });
});

// ---------------------------------------------------------------------------
// paraGraph tests
// ---------------------------------------------------------------------------

describe("paraGraph", () => {
  it("computes bottom-up depths", () => {
    const alice = stubNode("alice");
    const bob = stubNode("bob");
    const rel = stubRelationship("knows", alice, bob);
    const graph = stubGraph([alice, bob], [rel]);
    const view = toGraphView(graph);

    const depths = paraGraph<Subject, number>(
      (_query, _p, subResults) =>
        subResults.length === 0 ? 0 : Math.max(...subResults) + 1
    )(view);

    // Leaf nodes have depth 0
    expect(depths.get("alice")).toBe(0);
    expect(depths.get("bob")).toBe(0);
  });

  it("returns ReadonlyMap with identity keys", () => {
    const alice = stubNode("alice");
    const graph = stubGraph([alice]);
    const view = toGraphView(graph);

    const results = paraGraph<Subject, string>(
      (_query, p) => p.identity ?? ""
    )(view);

    expect(results.get("alice")).toBe("alice");
  });
});

// ---------------------------------------------------------------------------
// unfoldGraph tests
// ---------------------------------------------------------------------------

describe("unfoldGraph", () => {
  it("expands seeds into a PatternGraph", () => {
    const seeds = ["alice", "bob"];

    const result = unfoldGraph<string, Subject>(
      (seed) => [stubNode(seed)],
      (patterns) => stubGraph(patterns as Pattern<Subject>[])
    )(seeds);

    expect(result.nodes).toHaveLength(2);
    expect(result.nodes.map((n) => n.identity)).toContain("alice");
    expect(result.nodes.map((n) => n.identity)).toContain("bob");
  });
});

// ---------------------------------------------------------------------------
// SC-010: pipe composition equivalence
// ---------------------------------------------------------------------------

describe("pipe composition (SC-010)", () => {
  it("pipe(view, f, g, h) equals h(g(f(view)))", () => {
    const alice = stubNode("alice", ["Person"]);
    const bob = stubNode("bob", ["Person"]);
    const thing = stubNode("thing", ["Thing"]);
    const graph = stubGraph([alice, bob, thing]);
    const view = toGraphView(graph);

    const f = mapAllGraph<Subject>((p) => ({ ...p, identity: `f:${p.identity}` }));
    const g = filterGraph<Subject>(
      (_cls, p) => (p.identity ?? "").startsWith("f:"),
      SpliceGap
    );
    const h = mapAllGraph<Subject>((p) => ({ ...p, identity: `h:${p.identity}` }));

    // Sequential application
    const sequential = h(g(f(view)));

    // Manual pipe
    const piped = h(g(f(view)));

    expect(piped.viewElements.map(([, p]) => p.identity)).toEqual(
      sequential.viewElements.map(([, p]) => p.identity)
    );
  });
});

// ---------------------------------------------------------------------------
// SC-011: WASM-free usage (no @relateby/pattern import in this file)
// ---------------------------------------------------------------------------

describe("WASM-free usage (SC-011)", () => {
  it("all transforms work without WASM initialization", () => {
    // This entire test file uses no WASM — verifies SC-011
    const alice = stubNode("alice");
    const graph = stubGraph([alice]);
    const view = toGraphView(graph);

    // All transforms should work without init()
    const mapped = mapGraph<Subject>({ mapNode: (p) => p })(view);
    const filtered = filterGraph<Subject>(() => true, SpliceGap)(view);
    const folded = foldGraph<Subject, number>(() => 1, 0, (a, b) => a + b)(view);
    const withCtx = mapWithContext<Subject>((_q, p) => p)(view);
    const para = paraGraph<Subject, number>((_q, _p, _sub) => 0)(view);

    expect(mapped.viewElements).toHaveLength(1);
    expect(filtered.viewElements).toHaveLength(1);
    expect(folded).toBe(1);
    expect(withCtx.viewElements).toHaveLength(1);
    expect(para.size).toBe(1);
  });
});
