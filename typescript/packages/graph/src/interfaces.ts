// Core structural interfaces for @relateby/graph.
// All interfaces are structural (duck-typed) — no runtime dependency on WASM.
// Satisfied by Native* classes from @relateby/pattern and plain TS stubs alike.

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

/**
 * Pairs a snapshot GraphQuery with a classified list of elements.
 * Transforms consume a GraphView and produce a new one.
 * The snapshot query reflects the graph state at the start of the transformation.
 */
export interface GraphView<V> {
  readonly viewQuery: GraphQuery<V>;
  readonly viewElements: ReadonlyArray<readonly [GraphClass, Pattern<V>]>;
}

/**
 * Per-class mapping functions for mapGraph.
 * Unspecified classes use the identity function.
 */
export interface CategoryMappers<V> {
  mapNode?: (p: Pattern<V>) => Pattern<V>;
  mapRelationship?: (p: Pattern<V>) => Pattern<V>;
  mapWalk?: (p: Pattern<V>) => Pattern<V>;
  mapAnnotation?: (p: Pattern<V>) => Pattern<V>;
  mapOther?: (cls: GraphClass, p: Pattern<V>) => Pattern<V>;
}

/**
 * Construct an initial GraphView from a PatternGraph.
 * Reads the pre-classified arrays from PatternGraph<V> and maps them to
 * [GraphClass, Pattern<V>] pairs. Does not re-classify elements.
 */
export function toGraphView<V>(graph: PatternGraph<V>): GraphView<V> {
  const elements: Array<readonly [GraphClass, Pattern<V>]> = [
    ...graph.nodes.map((p): readonly [GraphClass, Pattern<V>] => [GNode, p] as const),
    ...graph.relationships.map((p): readonly [GraphClass, Pattern<V>] => [GRelationship, p] as const),
    ...graph.walks.map((p): readonly [GraphClass, Pattern<V>] => [GWalk, p] as const),
    ...graph.annotations.map((p): readonly [GraphClass, Pattern<V>] => [GAnnotation, p] as const),
  ];

  // Cast PatternGraph as GraphQuery snapshot (structural compatibility)
  const viewQuery: GraphQuery<V> = {
    nodes: () => graph.nodes,
    relationships: () => graph.relationships,
    source: (rel) => rel.elements[0] ?? null,
    target: (rel) => rel.elements[1] ?? null,
    incidentRels: (node) =>
      graph.relationships.filter(
        (r) =>
          (r.elements[0]?.identity !== undefined &&
            r.elements[0].identity === node.identity) ||
          (r.elements[1]?.identity !== undefined &&
            r.elements[1].identity === node.identity)
      ),
    degree: (node) =>
      graph.relationships.filter(
        (r) =>
          (r.elements[0]?.identity !== undefined &&
            r.elements[0].identity === node.identity) ||
          (r.elements[1]?.identity !== undefined &&
            r.elements[1].identity === node.identity)
      ).length,
    nodeById: (id) => graph.nodes.find((n) => n.identity === id) ?? null,
    relationshipById: (id) =>
      graph.relationships.find((r) => r.identity === id) ?? null,
  };

  return { viewQuery, viewElements: elements };
}

// Import GraphClass for use in toGraphView
import { GNode, GRelationship, GWalk, GAnnotation, type GraphClass } from "./adts.js";
