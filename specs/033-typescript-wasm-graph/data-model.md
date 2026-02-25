# Data Model: TypeScript/WASM Graph API

**Branch**: `033-typescript-wasm-graph`  
**Date**: 2026-02-25

This document describes the entities exposed at the TypeScript/WASM boundary and their relationships. All entities map directly to existing Rust types in `crates/pattern-core/src/`.

---

## Core Entities

### `Pattern` (existing, `WasmPattern`)

The foundational data structure. An s-expression-like recursive structure holding a value and zero or more child patterns.

| Field | Type | Description |
|-------|------|-------------|
| `value` | `JsValue` | The value at this node (any JS value; Subject objects use `_type: 'Subject'` marker) |
| `elements` | `Pattern[]` | Child patterns (empty for atomic/point patterns) |

**Rust backing**: `Pattern<JsValue>` in `wasm.rs`  
**Existing WASM type**: `WasmPattern` (already exposed)

---

### `PatternGraph` (new, `WasmPatternGraph`)

A classified, indexed collection of patterns organized by graph role. Immutable after construction; `merge` returns a new instance.

| Field | Type | Description |
|-------|------|-------------|
| `nodes` | `Pattern[]` | Patterns classified as graph nodes |
| `relationships` | `Pattern[]` | Patterns classified as graph relationships |
| `walks` | `Pattern[]` | Patterns classified as walks (sequences of nodes/rels) |
| `annotations` | `Pattern[]` | Patterns classified as annotations |
| `conflicts` | `Record<string, Pattern[]>` | Identity → conflicting patterns (for strict policy) |
| `size` | `number` | Total count of non-conflict elements |

**Operations**:
- `PatternGraph.fromPatterns(patterns, policy?)` → `PatternGraph`
- `PatternGraph.empty()` → `PatternGraph`
- `graph.merge(other)` → `PatternGraph`
- `graph.topoSort()` → `Pattern[]` (bottom-up shape-class order; used by `paraGraph`)

**Rust backing**: `PatternGraph<(), Subject>` in `pattern_graph.rs`  
**Classification**: Performed by `canonical_classifier` from `graph_classifier.rs`

---

### `ReconciliationPolicy` (new, `WasmReconciliationPolicy`)

A rule governing how identity conflicts are resolved when patterns with the same identity are combined into a graph.

**Variants** (static constructors):

| Constructor | Behavior |
|-------------|----------|
| `ReconciliationPolicy.lastWriteWins()` | Incoming pattern replaces existing |
| `ReconciliationPolicy.firstWriteWins()` | Existing pattern is kept; incoming discarded |
| `ReconciliationPolicy.strict()` | Conflict recorded in `graph.conflicts`; neither wins |
| `ReconciliationPolicy.merge(options?)` | Merge labels and properties per strategy |

**Merge options** (all optional):

| Option | Values | Default |
|--------|--------|---------|
| `elementStrategy` | `"replace" \| "append" \| "union"` | `"union"` |
| `labelMerge` | `"union" \| "intersect" \| "left" \| "right"` | `"union"` |
| `propertyMerge` | `"left" \| "right" \| "merge"` | `"merge"` |

**Rust backing**: `ReconciliationPolicy` enum in `reconcile.rs`

---

### `GraphQuery` (new, `WasmGraphQuery`)

A read-only query handle over a `PatternGraph`. Provides structural navigation without exposing the underlying storage.

| Method | Signature | Description |
|--------|-----------|-------------|
| `nodes()` | `() → Pattern[]` | All node patterns |
| `relationships()` | `() → Pattern[]` | All relationship patterns |
| `source(rel)` | `(Pattern) → Pattern \| null` | Source node of a relationship |
| `target(rel)` | `(Pattern) → Pattern \| null` | Target node of a relationship |
| `incidentRels(node)` | `(Pattern) → Pattern[]` | All relationships incident to a node |
| `degree(node)` | `(Pattern) → number` | Count of incident relationships |
| `nodeById(id)` | `(string) → Pattern \| null` | Look up node by identity string |
| `relationshipById(id)` | `(string) → Pattern \| null` | Look up relationship by identity string |

**Construction**: `GraphQuery.fromPatternGraph(graph)` → `GraphQuery`

**Rust backing**: `GraphQuery<Subject>` (struct of closures) in `graph_query.rs`, wrapped via `Rc`

---

### `GraphClass` (new, constant object)

A string-constant object classifying a pattern's role in a graph. Used as a discriminant in transform callbacks.

```typescript
const GraphClass: {
  readonly NODE:         "node";
  readonly RELATIONSHIP: "relationship";
  readonly ANNOTATION:   "annotation";
  readonly WALK:         "walk";
  readonly OTHER:        "other";
};
type GraphClassValue = "node" | "relationship" | "annotation" | "walk" | "other";
```

**Rust backing**: `GraphClass` enum in `graph_classifier.rs`

---

### `TraversalDirection` (new, constant object)

Direction of traversal for algorithm weight functions.

```typescript
const TraversalDirection: {
  readonly FORWARD:  "forward";
  readonly BACKWARD: "backward";
};
```

**Rust backing**: `TraversalDirection` enum in `graph_query.rs`

---

### `Weight` (TypeScript type, not a WASM class)

Specifies how edges are weighted during traversal. Accepted by all algorithm functions.

```typescript
type WeightFn = (rel: Pattern, direction: "forward" | "backward") => number;
type Weight = "undirected" | "directed" | "directed_reverse" | WeightFn;
```

**Default**: `"undirected"` (all edges traversable in both directions with cost 1)  
**Escape hatch**: `WeightFn` — called once per traversed edge; document performance cost

**Rust backing**: `TraversalWeight<Subject>` in `graph_query.rs`

---

## Pure TypeScript Entities (no WASM class)

These entities exist only in the TypeScript layer (`typescript/relateby/src/graph/index.ts`).

### `GraphClass` (TypeScript discriminated union)

Mirrors the Haskell `GraphClass` ADT for use in transform callbacks.

```typescript
type GraphClass =
  | { readonly tag: "GNode" }
  | { readonly tag: "GRelationship" }
  | { readonly tag: "GWalk" }
  | { readonly tag: "GAnnotation" }
  | { readonly tag: "GOther"; readonly extra: unknown };
```

Smart constructors: `GNode`, `GRelationship`, `GWalk`, `GAnnotation`, `GOther(extra)`

**Note**: The WASM `GraphClass` constant object uses lowercase string values (`"node"`, etc.) for the JS API. The TypeScript discriminated union uses `"GNode"` etc. for the functional transform API. These are two representations of the same concept at different API layers.

---

### `Substitution` (TypeScript discriminated union)

Governs container repair when `filterGraph` removes an element from inside a walk or annotation.

```typescript
type Substitution =
  | { readonly tag: "DeleteContainer" }
  | { readonly tag: "SpliceGap" }
  | { readonly tag: "ReplaceWithSurrogate"; readonly surrogate: Pattern };
```

Smart constructors: `DeleteContainer`, `SpliceGap`, `ReplaceWithSurrogate(surrogate)`

**Semantics**:
- `DeleteContainer`: Remove the entire containing walk/annotation
- `SpliceGap`: Remove the element and close the gap (splice remaining elements together)
- `ReplaceWithSurrogate`: Replace the removed element with a provided surrogate pattern

**Rust backing**: `Substitution` enum in `graph/transform/types.rs`

---

### `GraphView` (TypeScript interface)

Pairs a snapshot `GraphQuery` with a classified list of elements. Consumed and produced by transform functions.

```typescript
interface GraphView {
  readonly viewQuery:    GraphQuery;
  readonly viewElements: ReadonlyArray<readonly [GraphClass, Pattern]>;
}
```

**Construction**: `PatternGraph.toGraphView()` (produces initial view) or transform output  
**Rust backing**: `GraphView<Subject>` in `graph_view.rs`

---

### `CategoryMappers` (TypeScript interface)

Per-class mapping functions used by `mapGraph`.

```typescript
interface CategoryMappers {
  mapNode?:         (p: Pattern) => Pattern;
  mapRelationship?: (p: Pattern) => Pattern;
  mapWalk?:         (p: Pattern) => Pattern;
  mapAnnotation?:   (p: Pattern) => Pattern;
  mapOther?:        (cls: GraphClass, p: Pattern) => Pattern;
}
```

All fields optional; identity function used for unspecified classes.

---

## Entity Relationships

```
PatternGraph
  ├── constructed from: Pattern[] + ReconciliationPolicy
  ├── produces: GraphQuery (via GraphQuery.fromPatternGraph)
  ├── produces: GraphView (via toGraphView — TypeScript layer)
  └── produces: Pattern[] (via topoSort — for paraGraph ordering)

GraphQuery
  ├── wraps: PatternGraph (read-only view)
  ├── used by: all algorithm functions (bfs, dfs, shortestPath, etc.)
  └── used by: mapWithContext, paraGraph, paraGraphFixed (snapshot)

GraphView
  ├── contains: GraphQuery (snapshot)
  ├── contains: [(GraphClass, Pattern)] (classified elements)
  ├── consumed by: mapGraph, mapAllGraph, filterGraph, foldGraph, mapWithContext
  └── produced by: mapGraph, mapAllGraph, filterGraph, mapWithContext (new GraphView)

Pattern
  ├── atomic: Pattern.point(value)
  └── composite: Pattern.pattern(value) + addElement(child)
```

---

## State Transitions

### `PatternGraph` construction

```
Pattern[] + ReconciliationPolicy
  → classify each pattern (canonical_classifier)
  → route to pg_nodes / pg_relationships / pg_walks / pg_annotations / pg_other
  → resolve identity conflicts per policy
  → PatternGraph (immutable)
```

### `filterGraph` with `Substitution`

```
GraphView + predicate + Substitution
  → for each element:
      if keep(cls, p): include unchanged
      if !keep(cls, p) and p is inside a walk/annotation:
          DeleteContainer → remove entire container
          SpliceGap       → remove element, splice remaining
          ReplaceWithSurrogate → replace with surrogate pattern
  → new GraphView
```

### `paraGraph` ordering

```
GraphView
  → call graph.topoSort() [1 WASM crossing]
  → iterate elements in bottom-up topological order (entirely TypeScript)
  → for each element: f(query, pattern, subResults[])
  → ReadonlyMap<string, R>
```
