# Data Model: StandardGraph Bindings

**Feature**: 036-standardgraph-bindings | **Date**: 2026-03-15

## Binding Wrapper Types

This feature introduces no new domain entities — it wraps existing Rust types for FFI exposure.

### WasmStandardGraph

| Field | Type | Notes |
|-------|------|-------|
| inner | `StandardGraph` | Owned Rust StandardGraph instance |

**Lifecycle**: Created via constructor, `fromGram`, `fromPatterns`, or `fromPatternGraph`. Mutated via `add*` methods. Queried via accessors and graph-native queries. Optionally converted to `WasmPatternGraph` or `WasmGraphQuery` via escape hatches.

### WasmSubjectBuilder

| Field | Type | Notes |
|-------|------|-------|
| identity | `String` | Required, set at construction |
| labels | `Vec<String>` | Accumulated via `.label()` calls |
| properties | `HashMap<String, Value>` | Accumulated via `.property()` calls |

**Lifecycle**: Created via `Subject.build(identity)`. Labels and properties added via chained `&mut self` methods. Finalized via `.done()` which constructs a `Subject` and returns `WasmSubject`.

### PyStandardGraph

| Field | Type | Notes |
|-------|------|-------|
| inner | `StandardGraph` | Owned Rust StandardGraph instance |

**Lifecycle**: Same as WASM, with snake_case API. No escape hatches in this phase.

### PySubjectBuilder

| Field | Type | Notes |
|-------|------|-------|
| identity | `String` | Required, set at construction |
| labels | `Vec<String>` | Accumulated via `.label()` calls |
| properties | `HashMap<String, Value>` | Accumulated via `.property()` calls |

**Lifecycle**: Same as WASM builder. Python `.property()` accepts native Python types directly (auto-converted via `python_to_value()`).

## Type Mapping at FFI Boundary

| Rust Type | WASM/TypeScript | Python |
|-----------|-----------------|--------|
| `StandardGraph` | `StandardGraph` (class) | `StandardGraph` (class) |
| `SubjectBuilder` | `SubjectBuilder` (class) | `SubjectBuilder` (class) |
| `Subject` | `Subject` (existing) | `Subject` (existing) |
| `Pattern<Subject>` | `Pattern` (existing `WasmPattern`) | `PatternSubject` (existing `PyPattern`) |
| `Symbol` | `string` | `str` |
| `Value` | `Value` (existing factory) | Native Python types (`str`, `int`, `float`, `bool`) |
| `Option<&Pattern<Subject>>` | `Pattern \| undefined` | `PatternSubject \| None` |
| `Vec<&Pattern<Subject>>` | `Array<Pattern>` | `list[PatternSubject]` |
| `impl Iterator<Item = (&Symbol, &Pattern<Subject>)>` | `Array<{id: string, pattern: Pattern}>` | `list[tuple[str, PatternSubject]]` |
| `usize` | `number` | `int` |
| `bool` | `boolean` | `bool` |
| `Result<_, ParseError>` | throws `Error` | raises `ValueError` |

## Conversion Flow

### WASM: addNode(subject)
```
JS Subject → WasmSubject.into_subject() → Rust Subject
  → StandardGraph.add_node(subject)
  → return &mut self (JS this)
```

### WASM: node(id) → Pattern
```
JS string → Symbol::from(id)
  → StandardGraph.node(&symbol) → Option<&Pattern<Subject>>
  → clone Pattern<Subject> → subject_pattern_to_wasm() → WasmPattern
  → return to JS (or undefined if None)
```

### WASM: neighbors(nodeId) → Pattern[]
```
JS string → Symbol::from(id)
  → StandardGraph.neighbors(&symbol) → Vec<&Pattern<Subject>>
  → for each: clone → subject_pattern_to_wasm() → WasmPattern
  → collect into js_sys::Array → return to JS
```

### Python: add_node(subject)
```
PySubject → &self.inner (borrows Rust Subject)
  → StandardGraph.add_node(subject.clone())
  → return PyRef<Self> (Python self)
```

### Python: from_gram(input)
```
Python str → &str
  → gram_codec::parse_gram(input) → Result<Vec<Pattern<Subject>>, ParseError>
  → map_err to PyValueError
  → StandardGraph::from_patterns(patterns) → StandardGraph
  → wrap in PyStandardGraph → return to Python
```
