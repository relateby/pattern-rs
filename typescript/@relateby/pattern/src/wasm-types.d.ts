// Type declarations for the wasm-pack generated module.
// The actual file is generated at build time by `npm run build:wasm`.
// This declaration allows TypeScript to compile without the generated file.

declare module "../wasm/pattern_wasm.js" {
  export const Subject: {
    new(identity: string, labels: unknown, properties: unknown): {
      readonly identity: string;
      readonly labels: unknown;
      readonly properties: unknown;
    };
  };

  export const Pattern: {
    point(value: unknown): {
      readonly value: unknown;
      readonly elements: unknown;
      readonly identity: string | undefined;
      readonly length: number;
      isAtomic(): boolean;
    };
    of(value: unknown): ReturnType<typeof Pattern.point>;
    pattern(value: unknown): ReturnType<typeof Pattern.point>;
  };

  export const Value: {
    string(s: string): unknown;
    int(n: number): unknown;
    float(n: number): unknown;
    bool(b: boolean): unknown;
    null(): unknown;
  };

  export const ValidationRules: {
    new(maxDepth: unknown, maxElements: unknown): unknown;
  };

  export const NativePatternGraph: {
    fromPatterns(patterns: unknown[], policy?: unknown): unknown;
    empty(): unknown;
  };

  export const NativeReconciliationPolicy: {
    lastWriteWins(): unknown;
    firstWriteWins(): unknown;
    strict(): unknown;
    merge(options?: unknown): unknown;
  };

  export const NativeGraphQuery: {
    fromPatternGraph(graph: unknown): unknown;
  };

  export function GraphClassConstants(): {
    NODE: string;
    RELATIONSHIP: string;
    ANNOTATION: string;
    WALK: string;
    OTHER: string;
  };

  export function TraversalDirectionConstants(): {
    FORWARD: string;
    BACKWARD: string;
  };

  export function bfs(query: unknown, start: unknown, weight: unknown): unknown[];
  export function dfs(query: unknown, start: unknown, weight: unknown): unknown[];
  export function shortestPath(query: unknown, start: unknown, end: unknown, weight: unknown): unknown[] | null;
  export function allPaths(query: unknown, start: unknown, end: unknown, weight: unknown): unknown[][];
  export function connectedComponents(query: unknown, weight: unknown): unknown[][];
  export function hasCycle(query: unknown): boolean;
  export function isConnected(query: unknown, weight: unknown): boolean;
  export function topologicalSort(query: unknown): unknown[] | null;
  export function degreeCentrality(query: unknown): Record<string, number>;
  export function betweennessCentrality(query: unknown, weight: unknown): Record<string, number>;
  export function minimumSpanningTree(query: unknown, weight: unknown): unknown[];
  export function queryWalksContaining(query: unknown, node: unknown): unknown[];
  export function queryCoMembers(query: unknown, node: unknown, container: unknown): unknown[];
  export function queryAnnotationsOf(query: unknown, target: unknown): unknown[];

  export const Gram: {
    parse(input: string): unknown;
    stringify(value: unknown): string;
  };

  export default function init(): Promise<void>;
}
