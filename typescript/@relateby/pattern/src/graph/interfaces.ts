// Core structural interfaces for @relateby/pattern graph exports.
// These mirror the former @relateby/graph package so @relateby/pattern
// can publish as a single self-contained artifact.

/**
 * Structural interface for a self-descriptive value with identity, labels, and properties.
 */
export interface Subject {
  readonly identity: string | undefined;
  readonly labels: ReadonlySet<string>;
  readonly properties: Readonly<Record<string, unknown>>;
}

/**
 * Structural interface for a recursive, nested pattern generic over value type V.
 */
export interface Pattern<V> {
  readonly identity: string | undefined;
  readonly value: V | undefined;
  readonly elements: ReadonlyArray<Pattern<V>>;
}

/**
 * Structural interface for a classified collection of patterns.
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
 * Snapshot graph view used by pure TypeScript transforms.
 */
export interface GraphView<V> {
  readonly viewQuery: GraphQuery<V>;
  readonly viewElements: ReadonlyArray<readonly [GraphClass, Pattern<V>]>;
}

/**
 * Per-class mapping functions for mapGraph.
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
 */
export function toGraphView<V>(graph: PatternGraph<V>): GraphView<V> {
  const elements: Array<readonly [GraphClass, Pattern<V>]> = [
    ...graph.nodes.map((p): readonly [GraphClass, Pattern<V>] => [GNode, p] as const),
    ...graph.relationships.map((p): readonly [GraphClass, Pattern<V>] => [GRelationship, p] as const),
    ...graph.walks.map((p): readonly [GraphClass, Pattern<V>] => [GWalk, p] as const),
    ...graph.annotations.map((p): readonly [GraphClass, Pattern<V>] => [GAnnotation, p] as const),
  ];

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

import { GAnnotation, GNode, GRelationship, GWalk, type GraphClass } from "./adts.js";
