# Proposal: TypeScript/WASM and Python Bindings for StandardGraph

**Date**: 2026-03-15
**Status**: Draft
**Depends on**: `standard-graph-proposal.md` (implemented), `typescript-graph-proposal.md`, `python-graph-proposal.md`
**Scope**: `crates/pattern-core/src/wasm.rs`, `crates/pattern-core/src/python.rs`, `crates/pattern-core/typescript/pattern_core.d.ts`, `crates/pattern-core/pattern_core/__init__.pyi`, `crates/pattern-wasm/src/lib.rs`

---

## Summary

StandardGraph is now implemented in Rust. It is the recommended entry point for building and querying graphs — concrete type, no generics, no classifier/policy boilerplate. This proposal covers exposing it through the existing WASM/TypeScript and Python binding layers.

StandardGraph is the most binding-friendly type in the crate: it is monomorphic (`PatternGraph<(), Subject>` with fixed classifier), its API surface uses only simple boundary types (`Subject`, `Symbol`, `Pattern<Subject>`, `&str`, `usize`, `bool`), and it has no closures, trait objects, or lifetime complexity.

---

## Motivation

### The gap today

The existing bindings expose the *abstract* graph layer:

| | WASM/TypeScript | Python |
|---|---|---|
| `PatternGraph` | `NativePatternGraph` (immutable, from-patterns only) | Not exposed |
| `GraphQuery` | `NativeGraphQuery` (read-only closures) | Not exposed |
| `GraphClassifier` | Not exposed (hardcoded canonical) | Not exposed |
| Construction | `NativePatternGraph.fromPatterns(array)` | N/A |
| Subject creation | `new Subject("id", ["Label"], {k: Value.string("v")})` | `Subject(identity="id", labels={"Label"}, properties={...})` |

This means:
- **TypeScript users** must construct full `Pattern` objects, put them in an array, and call `fromPatterns`. There is no element-by-element construction, no `from_gram` shortcut, no neighbor/degree queries.
- **Python users** have no graph capability at all.

### What StandardGraph adds

A single type that covers 90% of graph use cases:

```typescript
// TypeScript
const g = new StandardGraph();
g.addNode(Subject.build("alice").label("Person").property("name", Value.string("Alice")).done());
g.addRelationship(Subject.build("r1").label("KNOWS").done(), "alice", "bob");
g.neighbors("bob");  // → [Pattern<Subject>]
```

```python
# Python
g = StandardGraph()
g.add_node(Subject.build("alice").label("Person").property("name", "Alice").done())
g.add_relationship(Subject.build("r1").label("KNOWS").done(), "alice", "bob")
g.neighbors("bob")  # → [PatternSubject]
```

---

## Design Decisions

### D1: Mutable builder across the FFI boundary

StandardGraph's Rust API uses `&mut self` for add methods. This is new territory — the existing graph bindings (`NativePatternGraph`) are immutable (construct once from patterns).

**WASM**: `wasm_bindgen` supports `&mut self` directly. The WASM object holds ownership and mutation works transparently. The existing `WasmPattern.addElement()` already uses this pattern.

**Python**: PyO3 supports `&mut self` via `#[pymethods]`. The existing `PySubject.add_label()` and `PySubject.set_property()` already mutate through PyO3.

**Decision**: Expose StandardGraph as a mutable type in both targets. No interior mutability or clone-on-write needed.

### D2: SubjectBuilder chaining

The Rust `SubjectBuilder` uses consuming `self` (`fn label(self, ...) -> Self`). This maps differently per target:

**WASM**: `wasm_bindgen` doesn't support consuming `self` in chained methods well — the JS object is invalidated after the first call. Two options:
1. **Use `&mut self` instead** — modify the builder in place, return `this`. Natural in JS.
2. **Skip the builder** — expose `Subject.build(identity, labels, properties)` as a single factory method that takes all fields at once.

**Recommendation**: Option 1 for WASM. A `SubjectBuilder` class with `&mut self` methods works naturally in JS/TS (`builder.label("X").property("k", v).done()`). Internally, the WASM wrapper accumulates state and constructs the Rust `SubjectBuilder` only at `.done()`.

**Python**: Same approach — `&mut self` methods returning `self` for chaining. Pythonic and matches existing `PySubject` mutation patterns.

### D3: Symbol as string at the boundary

Rust methods take `&Symbol`. The existing bindings already convert to/from strings at the boundary (e.g., `WasmSubject.identity` returns `String`).

**Decision**: All identity parameters are strings at the FFI boundary. The wrapper converts `&str` → `Symbol` internally. Users never see `Symbol` in TypeScript or Python.

### D4: `from_gram` location

In Rust, `from_gram` is an extension trait in gram-codec (circular dependency avoidance). In the bindings, there's no such constraint — the WASM crate already depends on both `pattern-core` and `gram-codec`.

**Decision**: `StandardGraph.fromGram(text)` is a static method on the WASM/Python class, implemented in the binding layer by calling `parse_gram` + `StandardGraph::from_patterns`. No need to expose the `FromGram` trait.

### D5: Iterator return types

Rust iterators (`nodes()`, `relationships()`) return `impl Iterator`. These don't cross FFI boundaries.

**WASM**: Return `js_sys::Array` (same as `NativePatternGraph.nodes` getter). Each element is a `[string, WasmPattern]` tuple or a JS object `{id: string, pattern: Pattern}`.

**Python**: Return `list[tuple[str, PatternSubject]]`. Alternatively, implement `__iter__` for lazy iteration.

**Recommendation**: Arrays/lists for simplicity. Graph sizes in practice are small enough that materialization is fine. Lazy iteration can be added later if profiling warrants it.

### D6: Escape hatches

`as_query()` returns `GraphQuery<Subject>`, which already has a WASM wrapper (`WasmGraphQuery`/`NativeGraphQuery`). `as_snapshot()` returns `GraphView` which has no bindings yet.

**Decision**: Expose `as_query()` → `NativeGraphQuery` in WASM (already exists). Defer `as_snapshot()` until `GraphView` bindings exist. `as_pattern_graph()` → `NativePatternGraph` is straightforward. Python defers all escape hatches until the abstract graph layer has Python bindings (per `python-graph-proposal.md`).

---

## Proposed API

### TypeScript / WASM

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

/** Fluent Subject builder. */
export class SubjectBuilder {
  label(label: string): SubjectBuilder;
  property(key: string, value: Value): SubjectBuilder;
  done(): Subject;
}
```

**Subject.build** is added as a static method on the existing `Subject` class:

```typescript
export class Subject {
  // ... existing methods ...
  static build(identity: string): SubjectBuilder;
}
```

### Python

```python
class StandardGraph:
    """Ergonomic graph builder and query interface."""

    def __init__(self) -> None: ...

    @classmethod
    def from_gram(cls, input: str) -> "StandardGraph":
        """Parse gram notation into a graph. Raises ValueError on invalid syntax."""
        ...

    @classmethod
    def from_patterns(cls, patterns: list[PatternSubject]) -> "StandardGraph": ...

    # --- Element addition (mutating, chainable) ---
    def add_node(self, subject: Subject) -> "StandardGraph": ...
    def add_relationship(self, subject: Subject, source_id: str, target_id: str) -> "StandardGraph": ...
    def add_walk(self, subject: Subject, relationship_ids: list[str]) -> "StandardGraph": ...
    def add_annotation(self, subject: Subject, element_id: str) -> "StandardGraph": ...
    def add_pattern(self, pattern: PatternSubject) -> "StandardGraph": ...

    # --- Element access ---
    def node(self, id: str) -> PatternSubject | None: ...
    def relationship(self, id: str) -> PatternSubject | None: ...
    def walk(self, id: str) -> PatternSubject | None: ...
    def annotation(self, id: str) -> PatternSubject | None: ...

    # --- Iteration ---
    def nodes(self) -> list[tuple[str, PatternSubject]]: ...
    def relationships(self) -> list[tuple[str, PatternSubject]]: ...
    def walks(self) -> list[tuple[str, PatternSubject]]: ...
    def annotations(self) -> list[tuple[str, PatternSubject]]: ...

    # --- Counts and health ---
    @property
    def node_count(self) -> int: ...
    @property
    def relationship_count(self) -> int: ...
    @property
    def walk_count(self) -> int: ...
    @property
    def annotation_count(self) -> int: ...
    @property
    def is_empty(self) -> bool: ...
    @property
    def has_conflicts(self) -> bool: ...

    # --- Graph-native queries ---
    def source(self, rel_id: str) -> PatternSubject | None: ...
    def target(self, rel_id: str) -> PatternSubject | None: ...
    def neighbors(self, node_id: str) -> list[PatternSubject]: ...
    def degree(self, node_id: str) -> int: ...

    # --- Representation ---
    def __repr__(self) -> str: ...
    def __len__(self) -> int: ...


class SubjectBuilder:
    """Fluent Subject builder."""
    def label(self, label: str) -> "SubjectBuilder": ...
    def property(self, key: str, value: object) -> "SubjectBuilder": ...
    def done(self) -> Subject: ...


class Subject:
    # ... existing methods ...
    @staticmethod
    def build(identity: str) -> SubjectBuilder: ...
```

**Python-specific note**: `.property()` accepts native Python types (`str`, `int`, `float`, `bool`) directly, matching the Rust `impl From<T> for Value` conversions. No need for explicit `Value.string(...)` wrapping.

---

## Implementation Plan

### Phase 1: WASM StandardGraph (highest value)

**Files**: `crates/pattern-core/src/wasm.rs`, `crates/pattern-wasm/src/lib.rs`

1. Add `WasmStandardGraph` struct wrapping `StandardGraph`
2. Implement constructor, `fromGram`, `fromPatterns`
3. Implement `addNode`, `addRelationship` (core construction)
4. Implement accessors: `node`, `relationship`, counts, `isEmpty`
5. Implement queries: `source`, `target`, `neighbors`, `degree`
6. Implement iteration getters: `nodes`, `relationships`, `walks`, `annotations`
7. Implement `addWalk`, `addAnnotation`, `addPattern`, `addPatterns` (less common)
8. Implement escape hatches: `asPatternGraph`, `asQuery`
9. Add `WasmSubjectBuilder` with `&mut self` chaining
10. Add `Subject.build()` static method to existing `WasmSubject`
11. Re-export from `crates/pattern-wasm/src/lib.rs`

**Estimated size**: ~400-500 lines in wasm.rs

### Phase 2: TypeScript definitions

**Files**: `crates/pattern-core/typescript/pattern_core.d.ts`

1. Add `StandardGraph` class definition with JSDoc
2. Add `SubjectBuilder` class definition
3. Add `Subject.build()` static method to existing `Subject` class

**Estimated size**: ~150 lines

### Phase 3: Python StandardGraph

**Files**: `crates/pattern-core/src/python.rs`, `crates/pattern-core/pattern_core/__init__.pyi`

1. Add `PyStandardGraph` struct wrapping `StandardGraph`
2. Implement `__init__`, `from_gram`, `from_patterns`
3. Implement `add_node`, `add_relationship` (core construction)
4. Implement accessors and counts
5. Implement queries: `source`, `target`, `neighbors`, `degree`
6. Implement iteration methods
7. Add `PySubjectBuilder` with `&mut self` chaining
8. Add `Subject.build()` to existing `PySubject`
9. Register classes in `#[pymodule]`
10. Update type stubs in `pattern_core/__init__.pyi`

**Estimated size**: ~300-350 lines in python.rs

### Phase 4: Integration and testing

1. WASM integration tests (Node.js): construct graph, query, verify
2. Python pytest tests: construct graph, query, verify
3. `from_gram` round-trip tests in both targets
4. Example files: `examples/pattern-core-wasm/standard_graph.mjs`, `examples/pattern-core-python/standard_graph.py`
5. Update `examples/relateby-graph/` with StandardGraph usage

---

## Complexity Assessment

### Why this is straightforward

1. **No generic type wrangling** — StandardGraph is concrete. No need for the `Pattern<JsValue>` / `Py<PyAny>` generic dance.
2. **All boundary types already wrapped** — `WasmSubject`, `WasmPattern`, `PySubject`, `PyPattern` exist. StandardGraph methods just compose these.
3. **Existing conversion helpers reusable** — `js_value_to_subject_pattern`, `python_to_value`, etc. handle all the type bridging needed.
4. **Pattern already established** — `WasmPatternGraph` demonstrates the graph-wrapper pattern for WASM. `PySubject` demonstrates mutable PyO3 types. This is incremental work.

### Where care is needed

1. **Mutable return for chaining** — `addNode` must return `&mut Self` or `Self` depending on target. WASM returns `this` (JS convention), Python returns `self`.
2. **SubjectBuilder ownership** — WASM's consuming-self limitation means the builder must use `&mut self` internally and construct the Rust builder only at `.done()`.
3. **Pattern conversion direction** — `node()` returns `&Pattern<Subject>` in Rust. WASM must clone and wrap as `WasmPattern`. Python must clone and wrap as `PyPattern`/`PatternSubject`. This is the same pattern used by all existing accessors.
4. **`from_gram` error mapping** — `ParseError` from gram-codec needs mapping to JS `Error` (WASM) or `ValueError` (Python). The WASM crate's `Gram` namespace already demonstrates this pattern.

---

## What This Does NOT Cover

- **GraphView bindings** — deferred until `GraphView` has its own binding story
- **Graph algorithm bindings** — covered by `typescript-graph-proposal.md` and `python-graph-proposal.md`; StandardGraph's `as_query()` provides the bridge
- **Graph transform bindings** — same; transforms operate on `GraphView`, not `StandardGraph`
- **Custom classifiers or reconciliation policies** — StandardGraph is opinionated by design; advanced users use `PatternGraph` directly

All of these are deferred, not unsupportable. StandardGraph bindings are independently useful from day one — `fromGram`, element construction, neighbor queries — without any of the deferred pieces.

---

## Next Steps: Dependency Picture

Each deferred item unlocks additional capability on top of StandardGraph bindings. Nothing is blocked; each layer is independently shippable.

```
StandardGraph bindings (this proposal)
  ├── works standalone ✓
  ├── + GraphView bindings  → unlocks asSnapshot()
  ├── + Algorithm bindings  → unlocks g.asQuery() → bfs/dfs/etc.
  ├── + Transform bindings  → unlocks map/filter/fold on graph views
  └── + PatternGraph bindings → unlocks custom classifiers/policies
```

### GraphView bindings → unlocks `asSnapshot()`

Pure timing dependency. `as_snapshot()` returns `GraphView<(), Subject>`, which has no WASM or Python wrapper yet. The Rust type is fully concrete when parameterized this way, so wrapping it is the same mechanical work as StandardGraph. Once `GraphView` gets bindings, wiring up `StandardGraph.asSnapshot()` is a one-liner.

### Algorithm bindings → unlocks `g.asQuery()` → bfs/dfs/etc.

Already partially done for WASM — BFS, DFS, shortest path, connected components, etc. are all exported as free functions in `pattern-wasm`. Completely missing for Python. The algorithms operate on `GraphQuery<V>`, not on `StandardGraph` directly. The bridge is `StandardGraph.asQuery()` → pass the query to algorithm functions. StandardGraph doesn't need to *wrap* algorithms; it just needs to produce a `GraphQuery`, which it already does.

A future convenience layer could add algorithm methods directly on StandardGraph (e.g., `g.shortestPath(a, b)` as sugar for `shortestPath(g.asQuery(), a, b)`). That's a convenience question, not a capability gap — decide based on whether the two-step pattern proves annoying in practice.

### Transform bindings → unlocks map/filter/fold on graph views

The most genuinely deferred item, for a real reason. Transforms (`map_graph`, `filter_graph`, `fold_graph`, `para_graph`) take closures/callbacks. In WASM, every callback invocation crosses the JS↔WASM boundary (~10-100ns per crossing), and transforms call back per-element. The existing `typescript-graph-proposal.md` explicitly analyzed this and concluded some transforms should be pure TypeScript rather than WASM-wrapped for performance. For Python, PyO3 handles callbacks more naturally (the GIL serializes anyway), but the API design needs its own thought. None of this is unsupportable, but it's design work that doesn't block StandardGraph bindings.

### PatternGraph bindings → unlocks custom classifiers/policies

Supportable but deliberately excluded from StandardGraph's *design scope*, not just the bindings. StandardGraph's entire value proposition is "you don't need to think about classifiers or reconciliation policies." Users who need those use `PatternGraph` directly, which means they need `PatternGraph` bindings — partially done for WASM (`NativePatternGraph`), not started for Python. There's no technical barrier, just a scoping choice: StandardGraph is opinionated, PatternGraph is flexible, and they serve different users.

---

## Success Criteria

1. TypeScript users can `new StandardGraph()`, add nodes/relationships, and query neighbors — all in <10 lines
2. Python users can do the same with snake_case API
3. `StandardGraph.fromGram("(a)-[:KNOWS]->(b)")` works in both targets
4. `Subject.build("id").label("L").property("k", v).done()` works in both targets
5. All existing binding tests continue to pass
6. Examples demonstrate the complete workflow in each target language
