# WASM/TypeScript API Contract: StandardGraph

**Feature**: 036-standardgraph-bindings | **Date**: 2026-03-15

## StandardGraph Class

```typescript
/** Ergonomic graph builder and query interface. */
export class StandardGraph {
  /** Create an empty graph. */
  constructor();

  /** Parse gram notation into a graph. Throws on invalid syntax. */
  static fromGram(input: string): StandardGraph;

  /** Create from an array of Pattern<Subject> instances. */
  static fromPatterns(patterns: Pattern[]): StandardGraph;

  /** Wrap an existing NativePatternGraph. */
  static fromPatternGraph(graph: NativePatternGraph): StandardGraph;

  // --- Element addition (mutating, chainable) ---
  addNode(subject: Subject): StandardGraph;
  addRelationship(subject: Subject, sourceId: string, targetId: string): StandardGraph;
  addWalk(subject: Subject, relationshipIds: string[]): StandardGraph;
  addAnnotation(subject: Subject, elementId: string): StandardGraph;
  addPattern(pattern: Pattern): StandardGraph;
  addPatterns(patterns: Pattern[]): StandardGraph;

  // --- Element access ---
  node(id: string): Pattern | undefined;
  relationship(id: string): Pattern | undefined;
  walk(id: string): Pattern | undefined;
  annotation(id: string): Pattern | undefined;

  // --- Iteration (getters returning arrays) ---
  readonly nodes: Array<{ id: string; pattern: Pattern }>;
  readonly relationships: Array<{ id: string; pattern: Pattern }>;
  readonly walks: Array<{ id: string; pattern: Pattern }>;
  readonly annotations: Array<{ id: string; pattern: Pattern }>;

  // --- Counts and health ---
  readonly nodeCount: number;
  readonly relationshipCount: number;
  readonly walkCount: number;
  readonly annotationCount: number;
  readonly isEmpty: boolean;
  readonly hasConflicts: boolean;

  // --- Graph-native queries ---
  source(relId: string): Pattern | undefined;
  target(relId: string): Pattern | undefined;
  neighbors(nodeId: string): Pattern[];
  degree(nodeId: string): number;

  // --- Escape hatches ---
  asPatternGraph(): NativePatternGraph;
  asQuery(): NativeGraphQuery;
}
```

## SubjectBuilder Class

```typescript
/** Fluent Subject builder. */
export class SubjectBuilder {
  label(label: string): SubjectBuilder;
  property(key: string, value: Value): SubjectBuilder;
  done(): Subject;
}
```

## Subject.build() Extension

```typescript
export class Subject {
  // ... existing methods ...
  static build(identity: string): SubjectBuilder;
}
```

## Re-exports from pattern-wasm

| Internal Name | Exported Name |
|---------------|---------------|
| `WasmStandardGraph` | `StandardGraph` |
| `WasmSubjectBuilder` | `SubjectBuilder` |

## Implementation Notes

**`fromGram` location**: Implemented in `crates/pattern-wasm/src/lib.rs` (not `pattern-core/src/wasm.rs`) because `pattern-core` cannot depend on `gram-codec` (circular dependency — gram-codec depends on pattern-core for types). The `pattern-wasm` crate already depends on both. Uses `#[wasm_bindgen(js_class = "StandardGraph")]` to attach the method to the exported class.

## Error Contract

| Method | Error Condition | Error Type |
|--------|----------------|------------|
| `fromGram(input)` | Invalid gram syntax | `Error` with descriptive message |
| `fromPatterns(patterns)` | Invalid pattern in array | `Error` with descriptive message |
