# TypeScript API Contract: Interfaces and Pure Graph Transforms

**Branch**: `033-typescript-wasm-graph`  
**Date**: 2026-02-25  
**TypeScript source**: `typescript/@relateby/graph/src/index.ts`  
**Entry point**: `@relateby/graph`  
**Haskell reference**: `../pattern-hs/libs/pattern/src/Pattern/Graph/Transform.hs`

All types and functions in this module are pure TypeScript. The module has no runtime dependency on WASM; it defines the structural interfaces that WASM-backed `Native*` classes (from `@relateby/pattern`) satisfy, and provides transform functions that operate against those interfaces. All transform functions are curried to enable point-free pipeline composition via `pipe`.

---

## Core Interfaces

### `Subject`

```typescript
/**
 * Structural interface for a self-descriptive value with identity, labels, and properties.
 * Satisfied by NativeSubject from @relateby/pattern.
 * Can also be implemented as a plain TypeScript object for WASM-free usage.
 */
export interface Subject {
  readonly identity: string | undefined;
  readonly labels: ReadonlySet<string>;
  readonly properties: Readonly<Record<string, unknown>>;
}
```

---

### `Pattern<V>`

```typescript
/**
 * Structural interface for a recursive, nested pattern generic over value type V.
 * Satisfied by NativePattern from @relateby/pattern.
 * Can also be implemented as a plain TypeScript object for WASM-free usage.
 */
export interface Pattern<V> {
  readonly identity: string | undefined;
  readonly value: V | undefined;
  readonly elements: ReadonlyArray<Pattern<V>>;
}
```

---

### `PatternGraph<V>`

```typescript
/**
 * Structural interface for a classified collection of patterns.
 * Satisfied by NativePatternGraph from @relateby/pattern.
 * Can also be implemented as a plain TypeScript object for WASM-free usage.
 */
export interface PatternGraph<V> {
  readonly nodes: ReadonlyArray<Pattern<V>>;
  readonly relationships: ReadonlyArray<Pattern<V>>;
  readonly walks: ReadonlyArray<Pattern<V>>;
  readonly annotations: ReadonlyArray<Pattern<V>>;
  readonly conflicts: Readonly<Record<string, ReadonlyArray<Pattern<V>>>>;
  readonly size: number;
  merge(other: PatternGraph<V>): PatternGraph<V>;
  topoSort(): ReadonlyArray<Pattern<V>>;
}
```

---

### `GraphQuery<V>`

```typescript
/**
 * Structural interface for graph traversal and lookup operations.
 * Satisfied by NativeGraphQuery from @relateby/pattern.
 * Can also be implemented as a plain TypeScript object for WASM-free usage.
 */
export interface GraphQuery<V> {
  nodes(): ReadonlyArray<Pattern<V>>;
  relationships(): ReadonlyArray<Pattern<V>>;
  source(rel: Pattern<V>): Pattern<V> | null;
  target(rel: Pattern<V>): Pattern<V> | null;
  incidentRels(node: Pattern<V>): ReadonlyArray<Pattern<V>>;
  degree(node: Pattern<V>): number;
  nodeById(identity: string): Pattern<V> | null;
  relationshipById(identity: string): Pattern<V> | null;
}
```

---

### `GraphView<V>`

```typescript
/**
 * Pairs a snapshot GraphQuery with a classified list of elements.
 * Transforms consume a GraphView and produce a new one.
 * The snapshot query reflects the graph state at the start of the transformation.
 */
export interface GraphView<V> {
  readonly viewQuery:    GraphQuery<V>;
  readonly viewElements: ReadonlyArray<readonly [GraphClass, Pattern<V>]>;
}
```

---

### `toGraphView`

```typescript
/**
 * Construct an initial GraphView from a PatternGraph.
 * Classifies all elements and pairs them with a query snapshot.
 */
export function toGraphView<V>(graph: PatternGraph<V>): GraphView<V>;
```

---

## ADT Definitions

### `GraphClass` discriminated union

```typescript
/**
 * Discriminated union mirroring Haskell's GraphClass ADT.
 * Used as the class discriminant in transform callbacks.
 */
export type GraphClass =
  | { readonly tag: "GNode" }
  | { readonly tag: "GRelationship" }
  | { readonly tag: "GWalk" }
  | { readonly tag: "GAnnotation" }
  | { readonly tag: "GOther"; readonly extra: unknown };

export const GNode:         GraphClass = { tag: "GNode" };
export const GRelationship: GraphClass = { tag: "GRelationship" };
export const GWalk:         GraphClass = { tag: "GWalk" };
export const GAnnotation:   GraphClass = { tag: "GAnnotation" };
export const GOther = (extra: unknown): GraphClass => ({ tag: "GOther", extra });
```

---

### `Substitution` discriminated union

```typescript
/**
 * Governs how container integrity is maintained when filterGraph removes
 * an element from inside a walk or annotation.
 *
 * - DeleteContainer: remove the entire containing walk/annotation
 * - SpliceGap: remove the element and close the gap
 * - ReplaceWithSurrogate: replace the removed element with a surrogate pattern
 */
export type Substitution =
  | { readonly tag: "DeleteContainer" }
  | { readonly tag: "SpliceGap" }
  | { readonly tag: "ReplaceWithSurrogate"; readonly surrogate: Pattern<unknown> };

export const DeleteContainer: Substitution = { tag: "DeleteContainer" };
export const SpliceGap:       Substitution = { tag: "SpliceGap" };
export const ReplaceWithSurrogate = <V>(surrogate: Pattern<V>): Substitution =>
  ({ tag: "ReplaceWithSurrogate", surrogate });
```

---

### `CategoryMappers<V>` interface

```typescript
/**
 * Per-class mapping functions for mapGraph.
 * Unspecified classes use the identity function.
 */
export interface CategoryMappers<V> {
  mapNode?:         (p: Pattern<V>) => Pattern<V>;
  mapRelationship?: (p: Pattern<V>) => Pattern<V>;
  mapWalk?:         (p: Pattern<V>) => Pattern<V>;
  mapAnnotation?:   (p: Pattern<V>) => Pattern<V>;
  mapOther?:        (cls: GraphClass, p: Pattern<V>) => Pattern<V>;
}
```

---

## Transform Functions

### `mapGraph`

```typescript
/**
 * Transform each element of a graph view using separate mapping functions per class.
 * Unspecified classes are passed through unchanged (identity function).
 *
 * Curried: mapGraph(mappers)(view)
 *
 * Haskell reference: mapGraph classifier fNode fRel fWalk fAnnot fOther view
 */
export const mapGraph:
  <V>(mappers: CategoryMappers<V>) => (view: GraphView<V>) => GraphView<V>;
```

---

### `mapAllGraph`

```typescript
/**
 * Transform every element of a graph view with a single uniform function,
 * regardless of class.
 *
 * Curried: mapAllGraph(f)(view)
 *
 * Haskell reference: mapAllGraph f view
 */
export const mapAllGraph:
  <V>(f: (p: Pattern<V>) => Pattern<V>) => (view: GraphView<V>) => GraphView<V>;
```

---

### `filterGraph`

```typescript
/**
 * Remove elements from a graph view that do not satisfy the predicate.
 * The substitution strategy governs how container integrity is maintained
 * when an element inside a walk or annotation is removed.
 *
 * Curried: filterGraph(keep, subst)(view)
 *
 * Haskell reference: filterGraph classifier keep subst view
 */
export const filterGraph:
  <V>(
    keep: (cls: GraphClass, p: Pattern<V>) => boolean,
    subst: Substitution
  ) => (view: GraphView<V>) => GraphView<V>;
```

---

### `foldGraph`

```typescript
/**
 * Reduce a graph view to a single value.
 * The (empty, combine) pair mirrors Haskell's Monoid constraint.
 *
 * Curried: foldGraph(f, empty, combine)(view)
 *
 * Haskell reference: foldGraph classifier f mempty mappend view
 */
export const foldGraph:
  <V, M>(
    f: (cls: GraphClass, p: Pattern<V>) => M,
    empty: M,
    combine: (a: M, b: M) => M
  ) => (view: GraphView<V>) => M;
```

---

### `mapWithContext`

```typescript
/**
 * Transform each element while receiving a snapshot GraphQuery.
 * The snapshot reflects the graph state at the start of the transformation;
 * later elements do not see mutations from earlier ones.
 *
 * Curried: mapWithContext(f)(view)
 *
 * Haskell reference: mapWithContext classifier f view
 */
export const mapWithContext:
  <V>(f: (query: GraphQuery<V>, p: Pattern<V>) => Pattern<V>) =>
  (view: GraphView<V>) => GraphView<V>;
```

---

### `paraGraph`

```typescript
/**
 * Bottom-up structural fold. Each element receives the pre-computed results
 * of its structural dependencies (sub-results).
 *
 * Calls view.viewQuery's underlying PatternGraph.topoSort() once (one WASM
 * crossing when backed by NativePatternGraph) to determine processing order,
 * then iterates entirely in TypeScript.
 *
 * Returns ReadonlyMap<string, R> mapping identity string → result.
 *
 * Curried: paraGraph(f)(view)
 *
 * Haskell reference: paraGraph classifier f view  →  Map (Id v) r
 */
export const paraGraph:
  <V, R>(
    f: (query: GraphQuery<V>, p: Pattern<V>, subResults: readonly R[]) => R
  ) => (view: GraphView<V>) => ReadonlyMap<string, R>;
```

---

### `paraGraphFixed`

```typescript
/**
 * Iterate paraGraph until a convergence predicate is satisfied.
 * init is the initial result for all elements before the first pass.
 * conv(prev, next) returns true when the result has converged.
 *
 * Curried: paraGraphFixed(conv, f, init)(view)
 *
 * Haskell reference: paraGraphFixed classifier conv f init view
 */
export const paraGraphFixed:
  <V, R>(
    conv: (prev: R, next: R) => boolean,
    f: (query: GraphQuery<V>, p: Pattern<V>, subResults: readonly R[]) => R,
    init: R
  ) => (view: GraphView<V>) => ReadonlyMap<string, R>;
```

---

### `unfoldGraph`

```typescript
/**
 * Expand a set of seed values into a PatternGraph.
 * expand(seed) returns the patterns produced by that seed.
 * build(patterns) constructs the PatternGraph from all expanded patterns.
 *
 * Curried: unfoldGraph(expand, build)(seeds)
 *
 * Haskell reference: unfoldGraph expand build seeds
 */
export const unfoldGraph:
  <S, V>(
    expand: (seed: S) => readonly Pattern<V>[],
    build: (patterns: readonly Pattern<V>[]) => PatternGraph<V>
  ) => (seeds: readonly S[]) => PatternGraph<V>;
```

---

## Composition Example

All transforms are curried and compose with `pipe` from Effect (or any compatible utility):

```typescript
import { pipe } from "effect";
import {
  toGraphView,
  mapAllGraph, filterGraph, mapWithContext, SpliceGap,
  foldGraph, paraGraph, GNode, GRelationship
} from "@relateby/graph";
import type { Subject } from "@relateby/graph";

// Works with NativePatternGraph from @relateby/pattern OR a plain TS stub
const view = toGraphView(graph); // graph: PatternGraph<Subject>

// Point-free pipeline
const processed = pipe(
  view,
  mapAllGraph(updateTimestamp),
  filterGraph(isRelevant, SpliceGap),
  mapWithContext(enrich),
);

// Fold: count nodes and relationships
const counts = foldGraph(
  (cls, _p) =>
    cls.tag === "GNode"         ? { nodes: 1, rels: 0 }
    : cls.tag === "GRelationship" ? { nodes: 0, rels: 1 }
    : { nodes: 0, rels: 0 },
  { nodes: 0, rels: 0 },
  (a, b) => ({ nodes: a.nodes + b.nodes, rels: a.rels + b.rels }),
)(view);

// Para: bottom-up depth computation
const depths = paraGraph(
  (_query, _p, subResults: number[]) =>
    subResults.length === 0 ? 0 : Math.max(...subResults) + 1,
)(view);
```

---

## WASM-Free Usage Example

`@relateby/graph` can be used entirely without WASM. Implement the interfaces as plain TypeScript objects:

```typescript
import {
  toGraphView, mapGraph, GNode, GRelationship
} from "@relateby/graph";
import type { Subject, Pattern, PatternGraph, GraphQuery } from "@relateby/graph";

// Plain TS stub — no NativePattern, no init()
const stubNode = (id: string): Pattern<Subject> => ({
  identity: id,
  value: { identity: id, labels: new Set(["Person"]), properties: {} },
  elements: [],
});

const stubGraph: PatternGraph<Subject> = {
  nodes: [stubNode("alice"), stubNode("bob")],
  relationships: [],
  walks: [],
  annotations: [],
  conflicts: {},
  size: 2,
  merge: (other) => stubGraph,
  topoSort: () => [stubNode("alice"), stubNode("bob")],
};

const view = toGraphView(stubGraph);
// All transforms work identically regardless of whether the graph is WASM-backed
```

---

## Internal Implementation Notes

### `Match` for `GraphClass` dispatch (with Effect)

`mapGraph` and `filterGraph` use `Match.tag` + `Match.exhaustive` internally when Effect is available. This ensures a compile error if a new `GraphClass` variant is added without updating the dispatch:

```typescript
import { pipe, Match } from "effect";
import type { Pattern } from "@relateby/graph";

const applyMapper = <V>(cls: GraphClass, p: Pattern<V>): Pattern<V> =>
  pipe(
    Match.value(cls),
    Match.tag("GNode",         () => mappers.mapNode?.(p) ?? p),
    Match.tag("GRelationship", () => mappers.mapRelationship?.(p) ?? p),
    Match.tag("GWalk",         () => mappers.mapWalk?.(p) ?? p),
    Match.tag("GAnnotation",   () => mappers.mapAnnotation?.(p) ?? p),
    Match.tag("GOther",        () => mappers.mapOther?.(cls, p) ?? p),
    Match.exhaustive,
  );
```

Without Effect, a standard `switch` on `cls.tag` with a default branch is used.

### `paraGraph` ordering

`paraGraph` and `paraGraphFixed` call `view.viewQuery`'s underlying `PatternGraph.topoSort()` once to get the bottom-up processing order. When backed by `NativePatternGraph` this is one WASM crossing; when backed by a plain TS stub it is a pure function call. All subsequent iteration is pure TypeScript with no further WASM crossings.
