// Fallback type declarations for the wasm-pack generated modules.
// The actual files are generated at build time by `npm run build:wasm`
// and `npm run build:wasm:node`.

interface PatternWasmSubjectLike {
  readonly identity: string;
  readonly labels: unknown;
  readonly properties: unknown;
}

interface PatternWasmStructureAnalysisLike {
  readonly summary: string;
  readonly depthDistribution: ReadonlyArray<number>;
  readonly elementCounts: ReadonlyArray<number>;
  readonly nestingPatterns: ReadonlyArray<string>;
}

interface PatternWasmPatternLike {
  readonly value: unknown;
  readonly elements: unknown;
  readonly identity: string | undefined;
  readonly length: number;
  addElement(element: PatternWasmPatternLike): void;
  allValues(predicate: (value: unknown) => boolean): boolean;
  analyzeStructure(): PatternWasmStructureAnalysisLike;
  anyValue(predicate: (value: unknown) => boolean): boolean;
  depth(): number;
  extract(): unknown;
  findFirst(predicate: (pattern: PatternWasmPatternLike) => boolean): PatternWasmPatternLike | null;
  fold(initial: unknown, reducer: (accumulator: unknown, value: unknown) => unknown): unknown;
  getElement(index: number): PatternWasmPatternLike | undefined;
  isAtomic(): boolean;
  map(mapper: (value: unknown) => unknown): PatternWasmPatternLike;
  size(): number;
  validate(rules: unknown): unknown;
  values(): unknown[];
}

interface PatternWasmPatternNamespaceLike {
  point(value: unknown): PatternWasmPatternLike;
  of(value: unknown): PatternWasmPatternLike;
  pattern(value: unknown): PatternWasmPatternLike;
  fromValues(values: unknown[]): PatternWasmPatternLike[];
}

interface PatternWasmValueFactoryLike {
  string(value: string): unknown;
  int(value: number): unknown;
  float(value: number): unknown;
  bool(value: boolean): unknown;
  null(): unknown;
}

interface PatternWasmGraphQueryLike {
  nodes(): unknown[];
  relationships(): unknown[];
  source(rel: PatternWasmPatternLike): PatternWasmPatternLike | null;
  target(rel: PatternWasmPatternLike): PatternWasmPatternLike | null;
  incidentRels(node: PatternWasmPatternLike): PatternWasmPatternLike[];
  degree(node: PatternWasmPatternLike): number;
  nodeById(identity: string): PatternWasmPatternLike | null;
  relationshipById(identity: string): PatternWasmPatternLike | null;
}

interface PatternWasmPatternGraphLike {
  readonly nodes: unknown[];
  readonly relationships: unknown[];
  readonly walks: unknown[];
  readonly annotations: unknown[];
  readonly conflicts: Record<string, unknown[]>;
  readonly size: number;
  merge(other: PatternWasmPatternGraphLike): PatternWasmPatternGraphLike;
  topoSort(): unknown[];
}

interface PatternWasmStandardGraphLike {
  readonly annotationCount: number;
  readonly annotations: unknown[];
  readonly hasConflicts: boolean;
  readonly isEmpty: boolean;
  readonly nodeCount: number;
  readonly nodes: unknown[];
  readonly relationshipCount: number;
  readonly relationships: unknown[];
  readonly walkCount: number;
  readonly walks: unknown[];
  addAnnotation(subject: PatternWasmSubjectLike, element: PatternWasmSubjectLike): void;
  addNode(subject: PatternWasmSubjectLike): void;
  addPattern(pattern: PatternWasmPatternLike): void;
  addPatterns(patterns: unknown[]): void;
  addRelationship(subject: PatternWasmSubjectLike, source: PatternWasmSubjectLike, target: PatternWasmSubjectLike): void;
  addWalk(subject: PatternWasmSubjectLike, relationships: unknown[]): void;
  annotation(id: string): PatternWasmPatternLike | undefined;
  asPatternGraph(): PatternWasmPatternGraphLike;
  asQuery(): PatternWasmGraphQueryLike;
  degree(nodeId: string): number;
  neighbors(nodeId: string): unknown[];
  node(id: string): PatternWasmPatternLike | undefined;
  relationship(id: string): PatternWasmPatternLike | undefined;
  source(id: string): PatternWasmPatternLike | undefined;
  target(id: string): PatternWasmPatternLike | undefined;
  walk(id: string): PatternWasmPatternLike | undefined;
}

interface PatternWasmGramLike {
  parse(input: string): unknown[];
  parseOne(input: string): unknown | null;
  stringify(value: unknown): string;
}

declare module "../wasm/pattern_wasm.js" {
  export class Subject implements PatternWasmSubjectLike {
    constructor(identity: string, labels: string[], properties: Record<string, unknown>);
    readonly identity: string;
    readonly labels: unknown;
    readonly properties: unknown;
    static fromId(identity: string): Subject;
    static build(identity: string): unknown;
  }

  export class Pattern implements PatternWasmPatternLike {
    readonly value: unknown;
    readonly elements: unknown;
    readonly identity: string | undefined;
    readonly length: number;
    static point(value: unknown): Pattern;
    static of(value: unknown): Pattern;
    static pattern(value: unknown): Pattern;
    static fromValues(values: unknown[]): Pattern[];
    addElement(element: Pattern): void;
    allValues(predicate: (value: unknown) => boolean): boolean;
    analyzeStructure(): PatternWasmStructureAnalysisLike;
    anyValue(predicate: (value: unknown) => boolean): boolean;
    depth(): number;
    extract(): unknown;
    findFirst(predicate: (pattern: Pattern) => boolean): Pattern | null;
    fold(initial: unknown, reducer: (accumulator: unknown, value: unknown) => unknown): unknown;
    getElement(index: number): Pattern | undefined;
    isAtomic(): boolean;
    map(mapper: (value: unknown) => unknown): Pattern;
    size(): number;
    validate(rules: unknown): unknown;
    values(): unknown[];
  }

  export class ValueFactory {
    static string(value: string): unknown;
    static int(value: number): unknown;
    static float(value: number): unknown;
    static bool(value: boolean): unknown;
    static null(): unknown;
  }

  export { ValueFactory as Value };

  export class ValidationRules {
    constructor(maxDepth?: number, maxElements?: number);
  }

  export class NativePatternGraph implements PatternWasmPatternGraphLike {
    readonly nodes: unknown[];
    readonly relationships: unknown[];
    readonly walks: unknown[];
    readonly annotations: unknown[];
    readonly conflicts: Record<string, unknown[]>;
    readonly size: number;
    static fromPatterns(patterns: unknown[], policy?: unknown): NativePatternGraph;
    static empty(): NativePatternGraph;
    merge(other: NativePatternGraph): NativePatternGraph;
    topoSort(): unknown[];
  }

  export class NativeReconciliationPolicy {
    static lastWriteWins(): NativeReconciliationPolicy;
    static firstWriteWins(): NativeReconciliationPolicy;
    static strict(): NativeReconciliationPolicy;
    static merge(options?: unknown): NativeReconciliationPolicy;
  }

  export class NativeGraphQuery implements PatternWasmGraphQueryLike {
    static fromPatternGraph(graph: NativePatternGraph): NativeGraphQuery;
    nodes(): unknown[];
    relationships(): unknown[];
    source(rel: Pattern): Pattern | null;
    target(rel: Pattern): Pattern | null;
    incidentRels(node: Pattern): Pattern[];
    degree(node: Pattern): number;
    nodeById(identity: string): Pattern | null;
    relationshipById(identity: string): Pattern | null;
  }

  export class StandardGraph implements PatternWasmStandardGraphLike {
    constructor();
    readonly annotationCount: number;
    readonly annotations: unknown[];
    readonly hasConflicts: boolean;
    readonly isEmpty: boolean;
    readonly nodeCount: number;
    readonly nodes: unknown[];
    readonly relationshipCount: number;
    readonly relationships: unknown[];
    readonly walkCount: number;
    readonly walks: unknown[];
    static fromGram(input: string): StandardGraph;
    static fromPatternGraph(graph: NativePatternGraph): StandardGraph;
    static fromPatterns(patterns: unknown[]): StandardGraph;
    addAnnotation(subject: Subject, element: Subject): void;
    addNode(subject: Subject): void;
    addPattern(pattern: Pattern): void;
    addPatterns(patterns: unknown[]): void;
    addRelationship(subject: Subject, source: Subject, target: Subject): void;
    addWalk(subject: Subject, relationships: unknown[]): void;
    annotation(id: string): Pattern | undefined;
    asPatternGraph(): NativePatternGraph;
    asQuery(): NativeGraphQuery;
    degree(nodeId: string): number;
    neighbors(nodeId: string): unknown[];
    node(id: string): Pattern | undefined;
    relationship(id: string): Pattern | undefined;
    source(id: string): Pattern | undefined;
    target(id: string): Pattern | undefined;
    walk(id: string): Pattern | undefined;
  }

  export class Gram {
    static parse(input: string): unknown[];
    static parseOne(input: string): unknown | null;
    static stringify(value: unknown): string;
  }

  export function graph_class_constants(): {
    NODE: string;
    RELATIONSHIP: string;
    ANNOTATION: string;
    WALK: string;
    OTHER: string;
  };
  export function traversal_direction_constants(): {
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

  export default function init(): Promise<void>;
}

declare module "../wasm-node/pattern_wasm.js" {
  export class WasmSubject implements PatternWasmSubjectLike {
    constructor(identity: string, labels: string[], properties: Record<string, unknown>);
    readonly identity: string;
    readonly labels: unknown;
    readonly properties: unknown;
    static fromId(identity: string): WasmSubject;
    static build(identity: string): unknown;
  }

  export class WasmPattern implements PatternWasmPatternLike {
    readonly value: unknown;
    readonly elements: unknown;
    readonly identity: string | undefined;
    readonly length: number;
    static point(value: unknown): WasmPattern;
    static of(value: unknown): WasmPattern;
    static pattern(value: unknown): WasmPattern;
    static fromValues(values: unknown[]): WasmPattern[];
    addElement(element: WasmPattern): void;
    allValues(predicate: (value: unknown) => boolean): boolean;
    analyzeStructure(): PatternWasmStructureAnalysisLike;
    anyValue(predicate: (value: unknown) => boolean): boolean;
    depth(): number;
    extract(): unknown;
    findFirst(predicate: (pattern: WasmPattern) => boolean): WasmPattern | null;
    fold(initial: unknown, reducer: (accumulator: unknown, value: unknown) => unknown): unknown;
    getElement(index: number): WasmPattern | undefined;
    isAtomic(): boolean;
    map(mapper: (value: unknown) => unknown): WasmPattern;
    size(): number;
    validate(rules: unknown): unknown;
    values(): unknown[];
  }

  export class ValueFactory {
    static string(value: string): unknown;
    static int(value: number): unknown;
    static float(value: number): unknown;
    static bool(value: boolean): unknown;
    static null(): unknown;
  }

  export class WasmValidationRules {
    constructor(maxDepth?: number, maxElements?: number);
  }

  export class WasmPatternGraph implements PatternWasmPatternGraphLike {
    static fromPatterns(patterns: unknown[], policy?: unknown): WasmPatternGraph;
    static empty(): WasmPatternGraph;
    readonly nodes: unknown[];
    readonly relationships: unknown[];
    readonly walks: unknown[];
    readonly annotations: unknown[];
    readonly conflicts: Record<string, unknown[]>;
    readonly size: number;
    merge(other: WasmPatternGraph): WasmPatternGraph;
    topoSort(): unknown[];
  }

  export class WasmReconciliationPolicy {
    static lastWriteWins(): WasmReconciliationPolicy;
    static firstWriteWins(): WasmReconciliationPolicy;
    static strict(): WasmReconciliationPolicy;
    static merge(options?: unknown): WasmReconciliationPolicy;
  }

  export class WasmGraphQuery implements PatternWasmGraphQueryLike {
    static fromPatternGraph(graph: WasmPatternGraph): WasmGraphQuery;
    nodes(): unknown[];
    relationships(): unknown[];
    source(rel: WasmPattern): WasmPattern | null;
    target(rel: WasmPattern): WasmPattern | null;
    incidentRels(node: WasmPattern): WasmPattern[];
    degree(node: WasmPattern): number;
    nodeById(identity: string): WasmPattern | null;
    relationshipById(identity: string): WasmPattern | null;
  }

  export class StandardGraph implements PatternWasmStandardGraphLike {
    constructor();
    readonly annotationCount: number;
    readonly annotations: unknown[];
    readonly hasConflicts: boolean;
    readonly isEmpty: boolean;
    readonly nodeCount: number;
    readonly nodes: unknown[];
    readonly relationshipCount: number;
    readonly relationships: unknown[];
    readonly walkCount: number;
    readonly walks: unknown[];
    static fromGram(input: string): StandardGraph;
    static fromPatternGraph(graph: WasmPatternGraph): StandardGraph;
    static fromPatterns(patterns: unknown[]): StandardGraph;
    addAnnotation(subject: WasmSubject, element: WasmSubject): void;
    addNode(subject: WasmSubject): void;
    addPattern(pattern: WasmPattern): void;
    addPatterns(patterns: unknown[]): void;
    addRelationship(subject: WasmSubject, source: WasmSubject, target: WasmSubject): void;
    addWalk(subject: WasmSubject, relationships: unknown[]): void;
    annotation(id: string): WasmPattern | undefined;
    asPatternGraph(): WasmPatternGraph;
    asQuery(): WasmGraphQuery;
    degree(nodeId: string): number;
    neighbors(nodeId: string): unknown[];
    node(id: string): WasmPattern | undefined;
    relationship(id: string): WasmPattern | undefined;
    source(id: string): WasmPattern | undefined;
    target(id: string): WasmPattern | undefined;
    walk(id: string): WasmPattern | undefined;
  }

  export class Gram {
    static parse(input: string): unknown[];
    static parseOne(input: string): unknown | null;
    static stringify(value: unknown): string;
  }

  export function graph_class_constants(): {
    NODE: string;
    RELATIONSHIP: string;
    ANNOTATION: string;
    WALK: string;
    OTHER: string;
  };
  export function traversal_direction_constants(): {
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
}
