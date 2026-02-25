# Proposal: TypeScript/WASM Package for `relateby` with Graph Features

**Date**: 2026-02-25  
**Status**: Draft  
**Replaces**: `wasm-typescript-bindings-update.md`, `wasm-boundary-analysis.md`,
`wasm-npm-package-structure.md`, `effect-ts-integration.md`

---

## Overview

The Rust core has grown significantly since the WASM bindings were last updated.
New graph-level capabilities — `PatternGraph`, `GraphClassifier`, `GraphQuery`,
graph algorithms, graph transforms, and `ReconciliationPolicy` — are fully
implemented in Rust but entirely absent from the JavaScript/TypeScript API.

This proposal covers three interconnected decisions:

1. **What to expose from WASM** — which Rust features cross the boundary and how
2. **What to implement in pure TypeScript** — graph transforms that would be
   slower wrapped in WASM than written directly in JS
3. **How to package and ship it** — directory layout, build process, Effect
   integration, and the `relateby` npm package structure

---

## Part 1: WASM Boundary Analysis

### The Core Trade-off

Every call from JavaScript into WASM has a cost: ~10–100ns per crossing for
argument marshalling and JS engine deoptimisation. Additionally, complex types
like `Pattern<Subject>` must be serialised/deserialised at each crossing.

The WASM boundary is **cheap** when:
- A single crossing triggers substantial Rust-side computation
- The algorithm has no need to call back into JS

The WASM boundary is **expensive** when:
- The algorithm calls a JS callback on every element (N crossings for N elements)
- The algorithm is simple enough that pure JS would be equally fast

### Decision Table

| Feature | Decision | Reason |
|---|---|---|
| `gram-codec` parse/serialize | ✅ WASM | 1 crossing, substantial computation, no callbacks |
| `PatternGraph.fromPatterns` | ✅ WASM | 1 crossing, non-trivial reconciliation |
| `PatternGraph.merge` | ✅ WASM | 1 crossing, no callbacks |
| `bfs` / `dfs` | ✅ WASM | 1 crossing, all computation Rust-internal |
| `shortestPath` (Dijkstra) | ✅ WASM | 1 crossing, non-trivial algorithm |
| `connectedComponents` | ✅ WASM | 1 crossing, Rust-internal union-find |
| `hasCycle` / `isConnected` | ✅ WASM | 1 crossing, Rust-internal |
| `topologicalSort` | ✅ WASM | 1 crossing, Rust-internal Kahn's |
| `degreeCentrality` | ✅ WASM | 1 crossing, Rust-internal |
| `betweennessCentrality` | ✅ WASM | 1 crossing, Rust-internal |
| `minimumSpanningTree` | ✅ WASM | 1 crossing, Rust-internal |
| `queryWalksContaining` | ✅ WASM | 1 crossing, Rust-internal |
| `queryCoMembers` | ✅ WASM | 1 crossing, Rust-internal |
| `queryAnnotationsOf` | ✅ WASM | 1 crossing, Rust-internal |
| `PatternGraph.topoSort()` | ✅ WASM | 1 crossing; exposes sort order for JS-side `paraGraph` |
| Custom weight callbacks | ⚠️ Escape hatch | N crossings per traversal; document perf cost |
| `mapGraph` (JS callback) | ❌ Pure TS | N crossings, trivial JS equivalent |
| `filterGraph` (JS callback) | ❌ Pure TS | N crossings, trivial JS equivalent |
| `foldGraph` (JS callback) | ❌ Pure TS | N crossings, trivial JS equivalent |
| `paraGraph` (JS callback) | ❌ Pure TS | N crossings; use `topoSort()` from WASM instead |
| `paraGraphFixed` (JS callback) | ❌ Pure TS | N×K crossings |
| `unfoldGraph` (JS callback) | ❌ Pure TS | K crossings, trivial JS equivalent |
| `mapWithContext` (JS callback) | ❌ Pure TS | N crossings + query handle overhead |

The graph transforms that should not be wrapped in WASM are not abandoned — they
become `typescript/relateby/src/graph/index.ts`, a pure TypeScript module
operating on WASM-provided types.

---

## Part 2: WASM Bindings

### Current State

The existing `wasm.rs` exposes `Value`, `Subject`, `Pattern`, `ValidationRules`,
and `StructureAnalysis`. The following Rust modules have no WASM exposure:
`pattern_graph`, `graph::graph_classifier`, `graph::graph_query`,
`graph::algorithms`, `graph::transform`, and `reconcile`.

### Known `wasm-bindgen` Constraints

1. No custom types in arrays — use `js_sys::Array` of `JsValue`
2. No generic WASM types — all types must be monomorphic
3. `JsValue` as the universal value type — `Pattern<JsValue>` is the WASM representation
4. Fallible operations return `{ _tag: 'Right', right: T } | { _tag: 'Left', left: E }`
5. `gram-codec` uses a pure Rust `nom` parser with no C dependencies; builds cleanly for `wasm32-unknown-unknown`

### Design Principles

- **TypeScript-first API** — accurate `.d.ts` declarations driven by `wasm-bindgen` JSDoc comments
- **Strings for enums** — `GraphClass` and `TraversalDirection` as string constants, not numeric enums
- **Either for fallible ops** — all operations that can fail return the existing Either-compatible shape
- **Array of JsValue** — collections return `js_sys::Array` of `JsValue`-wrapped items
- **Option B for Pattern types** — accept `WasmPattern` everywhere; at graph API boundaries, deserialize via the existing `_type: 'Subject'` marker convention (simpler JS API; revisit if type safety issues arise)

### Phase 1 API — PatternGraph and Classification

```typescript
class PatternGraph {
  static fromPatterns(patterns: Pattern[], policy?: ReconciliationPolicy): PatternGraph;
  static empty(): PatternGraph;

  get nodes(): Pattern[];
  get relationships(): Pattern[];
  get walks(): Pattern[];
  get annotations(): Pattern[];
  get conflicts(): Record<string, Pattern[]>;
  get size(): number;

  merge(other: PatternGraph): PatternGraph;

  /** Returns elements in bottom-up shape-class order for use by paraGraph. */
  topoSort(): Pattern[];
}

class ReconciliationPolicy {
  static lastWriteWins(): ReconciliationPolicy;
  static firstWriteWins(): ReconciliationPolicy;
  static strict(): ReconciliationPolicy;
  static merge(options?: {
    elementStrategy?: "replace" | "append" | "union";
    labelMerge?: "union" | "intersect" | "left" | "right";
    propertyMerge?: "left" | "right" | "merge";
  }): ReconciliationPolicy;
}

// String constants (not a class)
const GraphClass: {
  readonly NODE: "node";
  readonly RELATIONSHIP: "relationship";
  readonly ANNOTATION: "annotation";
  readonly WALK: "walk";
  readonly OTHER: "other";
};
type GraphClassValue = "node" | "relationship" | "annotation" | "walk" | "other";
```

### Phase 2 API — GraphQuery and Algorithms

```typescript
class GraphQuery {
  static fromPatternGraph(graph: PatternGraph): GraphQuery;

  nodes(): Pattern[];
  relationships(): Pattern[];
  source(rel: Pattern): Pattern | null;
  target(rel: Pattern): Pattern | null;
  incidentRels(node: Pattern): Pattern[];
  degree(node: Pattern): number;
  nodeById(identity: string): Pattern | null;
  relationshipById(identity: string): Pattern | null;
}

const TraversalDirection: {
  readonly FORWARD: "forward";
  readonly BACKWARD: "backward";
};

type WeightFn = (rel: Pattern, direction: "forward" | "backward") => number;
type Weight = "undirected" | "directed" | "directed_reverse" | WeightFn;

// All algorithms: 1 WASM crossing, all computation Rust-internal
function bfs(query: GraphQuery, start: Pattern, weight?: Weight): Pattern[];
function dfs(query: GraphQuery, start: Pattern, weight?: Weight): Pattern[];
function shortestPath(query: GraphQuery, start: Pattern, end: Pattern, weight?: Weight): Pattern[] | null;
function allPaths(query: GraphQuery, start: Pattern, end: Pattern, weight?: Weight): Pattern[][];
function connectedComponents(query: GraphQuery, weight?: Weight): Pattern[][];
function hasCycle(query: GraphQuery, weight?: Weight): boolean;
function isConnected(query: GraphQuery, weight?: Weight): boolean;
function topologicalSort(query: GraphQuery): Pattern[] | null;
function degreeCentrality(query: GraphQuery, weight?: Weight): Record<string, number>;
function betweennessCentrality(query: GraphQuery, weight?: Weight): Record<string, number>;
function minimumSpanningTree(query: GraphQuery, weight?: Weight): Pattern[];
function queryWalksContaining(query: GraphQuery, node: Pattern): Pattern[];
function queryCoMembers(query: GraphQuery, node: Pattern): Pattern[];
function queryAnnotationsOf(query: GraphQuery, target: Pattern): Pattern[];
```

**Custom weight callbacks** are an escape hatch. When a JS `WeightFn` is
provided, it is called once per traversed edge — potentially thousands of
crossings for a dense graph. Document this cost prominently.

### Implementation Notes

**`WasmPatternGraph`** wraps `PatternGraph<(), Subject>`. `from_patterns` accepts
a `js_sys::Array` of `WasmPattern`; node/relationship/walk/annotation getters
return `js_sys::Array` of `WasmPattern`. `topoSort()` calls `topo_shape_sort`
internally and returns a `js_sys::Array` in bottom-up shape-class order.

**`WasmGraphQuery`** wraps `GraphQuery<Subject>` via `Rc`. WASM runs
single-threaded, so `Rc` is safe. `source`/`target` return `JsValue::null()` if
absent; TypeScript declarations use `Pattern | null`.

**Weight function bridge** — accept `JsValue`; if a string, map to
`undirected()`/`directed()`/`directed_reverse()`; if a `Function`, wrap in an
`Rc<dyn Fn(...)>` closure that calls back into JS with a `WasmPattern` and
direction string.

**TypeScript declaration strategy** — `wasm-bindgen` auto-generates `.d.ts`.
Supplement with hand-written overrides for union return types (`Pattern[] | null`,
`Record<string, number>`) and the Either shape:

```typescript
// Augmentation file
export type Either<L, R> =
  | { _tag: "Left"; left: L }
  | { _tag: "Right"; right: R };
```

---

## Part 3: Pure TypeScript Graph Transforms

### Design: FP Style Following the Haskell Reference

The transform functions are pure TypeScript, curried, and follow the Haskell
reference signatures from `Pattern.Graph.Transform`. Key design decisions:

- **`mapGraph`** takes separate named functions per class, matching
  `mapGraph classifier fNode fRel fWalk fAnnot fOther view`
- **`filterGraph`** takes a `Substitution` discriminated union
  (`DeleteContainer | SpliceGap | ReplaceWithSurrogate`) rather than a boolean flag
- **`foldGraph`** uses an explicit `(empty, combine)` pair mirroring Haskell's
  `Monoid m =>` constraint
- **`mapWithContext`** passes a snapshot `GraphQuery` to each callback — later
  elements do not see mutations from earlier ones
- **`paraGraph`** returns `ReadonlyMap<string, R>` (identity → result), matching
  Haskell's `Map (Id v) r`
- **`paraGraphFixed`** takes a convergence predicate `(prev: R, next: R) => boolean`
- All transforms are **curried**: `mapGraph(mappers)(view)` enables point-free
  pipeline composition via `pipe`

### `GraphClass` and `Substitution` ADTs

```typescript
// Discriminated union mirroring Haskell's GraphClass ADT
export type GraphClass =
  | { readonly tag: "GNode" }
  | { readonly tag: "GRelationship" }
  | { readonly tag: "GWalk" }
  | { readonly tag: "GAnnotation" }
  | { readonly tag: "GOther"; readonly extra: unknown };

export const GNode: GraphClass         = { tag: "GNode" };
export const GRelationship: GraphClass = { tag: "GRelationship" };
export const GWalk: GraphClass         = { tag: "GWalk" };
export const GAnnotation: GraphClass   = { tag: "GAnnotation" };
export const GOther = (extra: unknown): GraphClass => ({ tag: "GOther", extra });

// Governs how container gaps are handled when filterGraph removes an element
// from inside a walk or annotation
export type Substitution =
  | { readonly tag: "DeleteContainer" }
  | { readonly tag: "SpliceGap" }
  | { readonly tag: "ReplaceWithSurrogate"; readonly surrogate: Pattern };

export const DeleteContainer: Substitution = { tag: "DeleteContainer" };
export const SpliceGap: Substitution       = { tag: "SpliceGap" };
export const ReplaceWithSurrogate = (surrogate: Pattern): Substitution =>
  ({ tag: "ReplaceWithSurrogate", surrogate });
```

### `GraphView`

```typescript
// Pairs a snapshot GraphQuery with a list of classified elements.
// Transforms consume a GraphView and produce a new one.
export interface GraphView {
  readonly viewQuery: GraphQuery;
  readonly viewElements: ReadonlyArray<readonly [GraphClass, Pattern]>;
}
```

### Transform Signatures

```typescript
// mapGraph: separate function per class
export const mapGraph:
  (mappers: CategoryMappers) => (view: GraphView) => GraphView;

// mapAllGraph: uniform function across all classes
export const mapAllGraph:
  (f: (p: Pattern) => Pattern) => (view: GraphView) => GraphView;

// filterGraph: Substitution ADT controls container repair
export const filterGraph:
  (keep: (cls: GraphClass, p: Pattern) => boolean, subst: Substitution) =>
  (view: GraphView) => GraphView;

// foldGraph: explicit (empty, combine) — mirrors Haskell's Monoid constraint
export const foldGraph:
  <M>(f: (cls: GraphClass, p: Pattern) => M, empty: M, combine: (a: M, b: M) => M) =>
  (view: GraphView) => M;

// mapWithContext: snapshot query passed to each callback
export const mapWithContext:
  (f: (query: GraphQuery, p: Pattern) => Pattern) =>
  (view: GraphView) => GraphView;

// paraGraph: identity → result map; uses graph.topoSort() for ordering
export const paraGraph:
  <R>(f: (query: GraphQuery, p: Pattern, subResults: readonly R[]) => R) =>
  (view: GraphView) => ReadonlyMap<string, R>;

// paraGraphFixed: iterate until convergence
export const paraGraphFixed:
  <R>(conv: (prev: R, next: R) => boolean,
      f: (query: GraphQuery, p: Pattern, subResults: readonly R[]) => R,
      init: R) =>
  (view: GraphView) => ReadonlyMap<string, R>;

// unfoldGraph: expand seeds into a PatternGraph
export const unfoldGraph:
  <S>(expand: (seed: S) => readonly Pattern[],
      build: (patterns: readonly Pattern[]) => PatternGraph) =>
  (seeds: readonly S[]) => PatternGraph;
```

`paraGraph` and `paraGraphFixed` call `graph.topoSort()` once (a single WASM
crossing) to get the correct bottom-up processing order, then iterate entirely
in TypeScript with no further boundary crossings.

---

## Part 4: Effect Integration

### Selected Modules

The `effect` library is adopted as an optional peer dependency. Only the modules
that solve real problems in `relateby` are used:

| Module | Decision | Reason |
|---|---|---|
| `pipe` | ✅ Re-export | Enables point-free composition; core to the FP style |
| `Either` | ✅ Use for fallible ops | Replaces hand-rolled `{ _tag }` shape; full combinator suite |
| `Option` | ✅ Use for nullable returns | Replaces `T \| null`; composable |
| `Match` | ✅ Use internally | Exhaustive dispatch on `GraphClass` and `Substitution` ADTs |
| `HashMap` | ❌ Skip | `ReadonlyMap` is sufficient; don't force the import |
| `Chunk` | ❌ Skip | `ReadonlyArray` is the right type |
| `Effect` (async) | ❌ Skip | All operations are synchronous |
| `Schema` | ❌ Defer | Future `relateby/io` module |

### `Either` and `Option` in the TypeScript Layer

The WASM bindings already return `{ _tag: 'Right', right: T } | { _tag: 'Left', left: E }`.
The TypeScript wrapper layer (`src/pattern/index.ts`) converts these to proper
`Either.Either<T, E>` and `Option.Option<T>` values so users get the full
combinator suite:

```typescript
// src/pattern/index.ts — wrapping WASM returns
import { Either, Option } from "effect";

export function validate(
  pattern: Pattern,
  rules: ValidationRules,
): Either.Either<void, ValidationError> {
  const raw = pattern._validate(rules);
  return raw._tag === "Right"
    ? Either.right(undefined)
    : Either.left(raw.left);
}

export function shortestPath(
  query: GraphQuery,
  start: Pattern,
  end: Pattern,
  weight?: Weight,
): Option.Option<Pattern[]> {
  const raw = _shortestPath(query, start, end, weight);
  return raw === null ? Option.none() : Option.some(raw);
}
```

### `Match` for `GraphClass` Dispatch

`Match.tag` + `Match.exhaustive` is used internally in `mapGraph` and
`filterGraph`. If a new `GraphClass` variant is added to the Rust type, TypeScript
will refuse to compile at every unhandled `Match` site:

```typescript
import { pipe, Match } from "effect";

const applyMapper = (cls: GraphClass, p: Pattern): Pattern =>
  pipe(
    Match.value(cls),
    Match.tag("GNode",         () => fNode(p)),
    Match.tag("GRelationship", () => fRel(p)),
    Match.tag("GWalk",         () => fWalk(p)),
    Match.tag("GAnnotation",   () => fAnnot(p)),
    Match.tag("GOther",        () => fOther(cls, p)),
    Match.exhaustive,  // compile error if any tag is missing
  );
```

### Peer Dependency

```json
{
  "peerDependencies": {
    "effect": ">=3.0.0"
  },
  "peerDependenciesMeta": {
    "effect": { "optional": true }
  }
}
```

Users who don't use Effect can still use `relateby`. The `pipe` function is
available from `effect` or from any compatible utility. All graph operations
are synchronous and have no Effect-specific return types in their core signatures.

---

## Part 5: Package Structure

### Guiding Principle: Mirror the Python Layout

The Python side already solved this:
- `crates/pattern-core/` and `crates/gram-codec/` — Rust crates with PyO3 bindings
- `python/relateby/` — unified packaging layer
- `relateby.pattern` and `relateby.gram` — public submodule namespaces

The JS/TS side follows the same shape:
- `crates/pattern-wasm/` — Rust crate with wasm-bindgen bindings (already exists)
- `typescript/relateby/` — unified packaging layer
- `relateby/pattern`, `relateby/gram`, `relateby/graph` — public entry points

### Directory Layout

```
pattern-rs/
├── crates/
│   └── pattern-wasm/          # existing Rust WASM crate
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           └── gram.rs
│
└── typescript/
    └── relateby/              # NEW — unified npm package
        ├── package.json
        ├── tsconfig.json
        ├── src/
        │   ├── index.ts           # re-exports + init()
        │   ├── pattern/
        │   │   └── index.ts       # WASM types + Either/Option wrappers
        │   ├── gram/
        │   │   └── index.ts       # Gram parse/stringify
        │   └── graph/
        │       └── index.ts       # pure-TS transforms (mapGraph, paraGraph, etc.)
        ├── wasm/                  # generated by wasm-pack (gitignored)
        │   ├── pattern_wasm_bg.wasm
        │   ├── pattern_wasm.js
        │   └── pattern_wasm.d.ts
        └── dist/                  # generated by tsc (gitignored)
            ├── esm/
            └── cjs/
```

### `package.json`

```json
{
  "name": "relateby",
  "version": "0.1.0",
  "description": "Pattern data structures and Gram notation for JavaScript/TypeScript",
  "license": "BSD-3-Clause",
  "type": "module",
  "exports": {
    ".":         { "import": "./dist/esm/index.js",          "types": "./dist/esm/index.d.ts" },
    "./pattern": { "import": "./dist/esm/pattern/index.js",  "types": "./dist/esm/pattern/index.d.ts" },
    "./gram":    { "import": "./dist/esm/gram/index.js",     "types": "./dist/esm/gram/index.d.ts" },
    "./graph":   { "import": "./dist/esm/graph/index.js",    "types": "./dist/esm/graph/index.d.ts" }
  },
  "files": ["dist/", "wasm/pattern_wasm_bg.wasm", "wasm/pattern_wasm.js", "wasm/pattern_wasm.d.ts"],
  "scripts": {
    "build:wasm": "wasm-pack build ../../crates/pattern-wasm --target bundler --out-dir ../../typescript/relateby/wasm",
    "build:ts":   "tsc -p tsconfig.json && tsc -p tsconfig.cjs.json",
    "build":      "npm run build:wasm && npm run build:ts",
    "test":       "vitest run",
    "prepublishOnly": "npm run build"
  },
  "peerDependencies": {
    "effect": ">=3.0.0"
  },
  "peerDependenciesMeta": {
    "effect": { "optional": true }
  },
  "devDependencies": {
    "typescript": "^5.0",
    "vitest": "^2.0"
  }
}
```

### Build Process

```
Step 1: wasm-pack  →  wasm/  (WASM binary + JS glue + .d.ts)
Step 2: tsc        →  dist/  (compiled TS, re-exporting from wasm/)
```

```bash
# From typescript/relateby/
npm run build:wasm   # ~10–30s (Rust compile + wasm-opt)
npm run build:ts     # ~2s (TypeScript compile)
npm run build        # both steps sequentially
```

The `--target bundler` flag produces ES module output compatible with Vite,
webpack, Rollup, and esbuild. For Node.js-only use, `--target nodejs` is an
alternative.

### WASM Initialisation

```typescript
// src/index.ts
export { default as init } from "../../wasm/pattern_wasm.js";
export * from "./pattern/index.js";
export * from "./gram/index.js";
export * as graph from "./graph/index.js";
```

```typescript
// Bundler (Vite, webpack) — init is automatic
import { Pattern, Gram } from "relateby";

// Node.js or explicit init
import { init, Pattern, Gram } from "relateby";
await init();
```

### `.gitignore` additions

```gitignore
typescript/relateby/wasm/
typescript/relateby/dist/
typescript/relateby/node_modules/
```

### Existing Code Migration

| Existing | Change |
|---|---|
| `crates/pattern-wasm/typescript/gram.d.ts` | Content moves into `typescript/relateby/src/gram/index.ts` as real TS source; standalone `.d.ts` deleted |
| `examples/wasm-js/` | Keep as-is; builds its own WASM separately |

---

## Usage Examples

### Graph construction and algorithms

```typescript
import { init, Pattern, Subject, Value, PatternGraph, GraphQuery,
         ReconciliationPolicy, bfs, shortestPath, degreeCentrality,
         connectedComponents } from "relateby";
import { pipe, Either, Option } from "effect";

await init();

const alice = new Subject("alice", ["Person"], { name: Value.string("Alice") });
const bob   = new Subject("bob",   ["Person"], { name: Value.string("Bob") });
const carol = new Subject("carol", ["Person"], { name: Value.string("Carol") });
const r1    = new Subject("r1", ["KNOWS"], {});

const pAlice = Pattern.point(alice);
const pBob   = Pattern.point(bob);
const pCarol = Pattern.point(carol);
const pRel   = Pattern.pattern(r1);
pRel.addElement(pAlice);
pRel.addElement(pBob);

const graph = PatternGraph.fromPatterns(
  [pAlice, pBob, pCarol, pRel],
  ReconciliationPolicy.lastWriteWins()
);

console.log(graph.nodes.length);         // 3
console.log(graph.relationships.length); // 1

const query = GraphQuery.fromPatternGraph(graph);

// BFS — returns Pattern[]
const visited = bfs(query, pAlice, "undirected");

// Shortest path — returns Option<Pattern[]>
const path: Option.Option<Pattern[]> = shortestPath(query, pAlice, pBob);
const names = pipe(
  path,
  Option.map(nodes => nodes.map(n => n.value.identity)),
  Option.getOrElse(() => []),
);

// Centrality
const centrality = degreeCentrality(query);
// { alice: 0.5, bob: 0.5, carol: 0 }

// Connected components
const components = connectedComponents(query);
// [[pAlice, pBob], [pCarol]]
```

### Graph transforms (pure TypeScript)

```typescript
import { pipe } from "effect";
import { mapAllGraph, filterGraph, mapWithContext, SpliceGap,
         foldGraph, paraGraph, GNode, GRelationship } from "relateby/graph";

// Point-free pipeline — reads like Haskell
const processed = pipe(
  view,
  mapAllGraph(updateTimestamp),
  filterGraph(isRelevant, SpliceGap),
  mapWithContext(enrich),
);

// Fold: count nodes and relationships
const counts = foldGraph(
  (cls, _p) => cls.tag === "GNode" ? { nodes: 1, rels: 0 }
              : cls.tag === "GRelationship" ? { nodes: 0, rels: 1 }
              : { nodes: 0, rels: 0 },
  { nodes: 0, rels: 0 },
  (a, b) => ({ nodes: a.nodes + b.nodes, rels: a.rels + b.rels }),
)(view);

// Para: bottom-up structural fold
const depths = paraGraph(
  (_query, p, subResults: number[]) =>
    subResults.length === 0 ? 0 : Math.max(...subResults) + 1,
)(view);
```

### Gram parsing

```typescript
import { init, Gram } from "relateby";
await init();

const pattern = Gram.parse("(alice:Person)-[:KNOWS]->(bob:Person)");
const text    = Gram.stringify(pattern);
```

---

## Implementation Plan

| Phase | Work | Estimate |
|---|---|---|
| 1 | `PatternGraph`, `ReconciliationPolicy`, `GraphClass` in `wasm.rs` | 3–4 days |
| 2 | `GraphQuery`, 13 algorithm functions, weight bridge, `topoSort()` | 4–5 days |
| 3 | `typescript/relateby/` package scaffold + pure-TS graph transforms | 2–3 days |
| 4 | Effect integration (`Either`/`Option` wrappers), TypeScript declaration augmentations | 1–2 days |
| **Total** | | **10–14 days** |

### Open Questions

1. **`Pattern<Subject>` vs `Pattern<JsValue>`**: The existing `WasmPattern` wraps
   `Pattern<JsValue>`. Graph operations work over `Pattern<Subject>`. Recommendation:
   accept `WasmPattern` everywhere; at graph API boundaries, deserialize via the
   `_type: 'Subject'` marker convention (Option B). Revisit if type safety issues arise.

2. **`PatternGraph` mutability**: `merge()` should return a new instance (functional
   style, consistent with Rust), not mutate in place.

3. **Bundle size**: Adding graph algorithms will increase WASM binary size. Consider
   `wasm-opt` and feature flags (e.g., `--features wasm-algorithms`) for users who
   don't need algorithms.

4. **`relateby/io` module**: `effect/Schema` is deferred to a future module for
   validating external JSON → `Pattern<Subject>`.
