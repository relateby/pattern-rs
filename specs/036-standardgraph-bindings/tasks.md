# Tasks: StandardGraph TypeScript/WASM and Python Bindings

**Input**: Design documents from `/specs/036-standardgraph-bindings/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: Integration tests and examples are included as they are part of the feature's success criteria (SC-005, SC-006, SC-007).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup

**Purpose**: No new project structure needed. All target files exist. This phase validates the starting point.

- [ ] T001 Verify `StandardGraph` Rust API is complete and all tests pass by running `cargo test -p pattern-core -- standard_graph`
- [ ] T002 Verify WASM build compiles by running `cargo build --workspace --target wasm32-unknown-unknown`

**Checkpoint**: Rust StandardGraph is stable, WASM target compiles cleanly.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared conversion helpers needed by both WASM and Python user stories.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [ ] T003 Add `subject_to_wasm_subject()` and `wasm_subject_to_subject()` helper functions in `crates/pattern-core/src/wasm.rs` if not already extractable from existing `WasmSubject::from_subject()` / `WasmSubject::into_subject()`. Confirm these existing methods are sufficient for StandardGraph's needs (they take/return owned `Subject`, which `add_node` requires).

**Checkpoint**: Conversion helpers confirmed or added. User story implementation can begin.

---

## Phase 3: User Story 1 + 2 — Build, Query, and Parse Graphs in TypeScript (Priority: P1) 🎯 MVP

**Goal**: TypeScript developers can create a `StandardGraph` (empty, from patterns, from gram), add nodes/relationships, and query neighbors/source/target/degree.

**Independent Test**: Create a graph with 3 nodes and 2 relationships in TypeScript, parse gram notation, query neighbors, and verify results.

### Implementation for User Story 1 + 2

- [ ] T004 [US1] Add `WasmStandardGraph` struct wrapping `StandardGraph` with `#[wasm_bindgen]`, implement `new()` constructor returning empty graph in `crates/pattern-core/src/wasm.rs`
- [ ] T005 [US1] Implement `WasmStandardGraph::fromPatterns(patterns: &js_sys::Array)` static method that converts JS array of `WasmPattern` to `Vec<Pattern<Subject>>` and calls `StandardGraph::from_patterns()` in `crates/pattern-core/src/wasm.rs`
- [ ] T006 [US1] Implement `WasmStandardGraph::fromPatternGraph(graph: &WasmPatternGraph)` static method that extracts inner `PatternGraph` and wraps via `StandardGraph::from_pattern_graph()` in `crates/pattern-core/src/wasm.rs`
- [ ] T007 [US1] Implement `addNode(&mut self, subject: &WasmSubject)` that calls `self.inner.add_node(subject.as_subject().clone())` in `crates/pattern-core/src/wasm.rs`. Use wasm-bindgen's `&mut self` support so the method mutates in place and JS receives `this` back for chaining (no explicit return needed).
- [ ] T008 [US1] Implement `addRelationship(&mut self, subject: &WasmSubject, source_id: &str, target_id: &str)` that converts ids to `Symbol` and calls `self.inner.add_relationship()` in `crates/pattern-core/src/wasm.rs`. Same `&mut self` chaining pattern as `addNode`.
- [ ] T009 [US1] Implement element access methods `node(&self, id: &str)`, `relationship(&self, id: &str)`, `walk(&self, id: &str)`, `annotation(&self, id: &str)` returning `Option<WasmPattern>` via `subject_pattern_to_wasm()` in `crates/pattern-core/src/wasm.rs`
- [ ] T010 [US1] Implement count getters `nodeCount`, `relationshipCount`, `walkCount`, `annotationCount`, `isEmpty`, `hasConflicts` as `#[wasm_bindgen(getter)]` properties in `crates/pattern-core/src/wasm.rs`
- [ ] T011 [US1] Implement graph-native query methods `source(relId)`, `target(relId)`, `neighbors(nodeId)`, `degree(nodeId)` in `crates/pattern-core/src/wasm.rs`. `neighbors` returns `js_sys::Array` of `WasmPattern`, `degree` returns `usize`.
- [ ] T012 [US1] Implement iteration getters `nodes`, `relationships`, `walks`, `annotations` as `#[wasm_bindgen(getter)]` returning `js_sys::Array` of JS objects `{id: string, pattern: WasmPattern}` in `crates/pattern-core/src/wasm.rs`
- [ ] T013 [US1] Implement less-common construction methods `addWalk(&mut self, subject, relationship_ids: &js_sys::Array)`, `addAnnotation(&mut self, subject, element_id)`, `addPattern(&mut self, pattern)`, `addPatterns(&mut self, patterns)` in `crates/pattern-core/src/wasm.rs`. Same `&mut self` chaining pattern as `addNode`.
- [ ] T014 [US2] Implement `fromGram(input: &str) -> WasmStandardGraph` in `crates/pattern-wasm/src/lib.rs` (where gram-codec dependency is available — pattern-core cannot depend on gram-codec due to circular dependency). Call `gram_codec::parse_gram(input)` then `StandardGraph::from_patterns()`, mapping `ParseError` to `JsValue::from_str()`. Wire as a `#[wasm_bindgen]` static method on `WasmStandardGraph` using `#[wasm_bindgen(js_class = "StandardGraph")]`.
- [ ] T015 [US1] Re-export `WasmStandardGraph as StandardGraph` from `crates/pattern-wasm/src/lib.rs`
- [ ] T016 [P] [US1] Add `StandardGraph` class definition with JSDoc to `crates/pattern-core/typescript/pattern_core.d.ts` covering constructor, static factories, all mutation methods, accessors, getters, counts, queries

**Checkpoint**: TypeScript users can `new StandardGraph()`, `fromGram()`, add nodes/relationships, and query neighbors. All WASM compilation passes.

---

## Phase 4: User Story 3 — Build and Query a Graph in Python (Priority: P2)

**Goal**: Python developers can create a `StandardGraph`, add nodes/relationships, parse gram notation, and query neighbors using snake_case API.

**Independent Test**: Create a graph in Python with nodes and relationships, parse gram, query neighbors and counts.

### Implementation for User Story 3

- [ ] T017 [US3] Add `PyStandardGraph` struct wrapping `StandardGraph` with `#[pyclass]`, implement `__init__` (new empty graph) in `crates/pattern-core/src/python.rs`
- [ ] T018 [US3] Implement `from_patterns(patterns: &Bound<'_, PyList>)` classmethod that converts Python list to `Vec<Pattern<Subject>>` and calls `StandardGraph::from_patterns()` in `crates/pattern-core/src/python.rs`
- [ ] T019 [US3] Implement `from_gram(input: &str)` as a Python-side classmethod on `StandardGraph` in `python/relateby/pattern/__init__.py` (unified package layer where both pattern-core and gram-codec are available — pattern-core cannot depend on gram-codec due to circular dependency). Call gram's `parse_gram()` then pass results to `StandardGraph.from_patterns()`. Raise `ValueError` on invalid syntax.
- [ ] T020 [US3] Implement `add_node(&mut self, subject: &PySubject)` and `add_relationship(&mut self, subject: &PySubject, source_id: &str, target_id: &str)` returning `PyRefMut<Self>` for chaining in `crates/pattern-core/src/python.rs`
- [ ] T021 [US3] Implement element access methods `node(&self, id: &str)`, `relationship(&self, id: &str)`, `walk(&self, id: &str)`, `annotation(&self, id: &str)` returning `Option<PyPattern>` in `crates/pattern-core/src/python.rs`. Convert `Pattern<Subject>` to `PyPattern` by cloning Subject into `PySubject` and wrapping.
- [ ] T022 [US3] Implement count properties `node_count`, `relationship_count`, `walk_count`, `annotation_count`, `is_empty`, `has_conflicts` as `#[getter]` properties in `crates/pattern-core/src/python.rs`
- [ ] T023 [US3] Implement graph-native query methods `source(rel_id)`, `target(rel_id)`, `neighbors(node_id)`, `degree(node_id)` in `crates/pattern-core/src/python.rs`. `neighbors` returns `Vec<PyPattern>`, `degree` returns `usize`.
- [ ] T024 [US3] Implement iteration methods `nodes()`, `relationships()`, `walks()`, `annotations()` returning `Vec<(String, PyPattern)>` (exposed as `list[tuple[str, PatternSubject]]`) in `crates/pattern-core/src/python.rs`
- [ ] T025 [US3] Implement less-common construction methods `add_walk`, `add_annotation`, `add_pattern` in `crates/pattern-core/src/python.rs`
- [ ] T026 [US3] Implement `__repr__` returning `StandardGraph(nodes=N, relationships=N, walks=N, annotations=N)` and `__len__` returning total element count in `crates/pattern-core/src/python.rs`
- [ ] T027 [US3] Register `PyStandardGraph` in `#[pymodule] fn pattern_core` in `crates/pattern-core/src/python.rs`
- [ ] T028 [P] [US3] Add `StandardGraph` class with full type hints to `crates/pattern-core/pattern_core/__init__.pyi`

**Checkpoint**: Python users can create, populate, and query StandardGraph with full snake_case API.

---

## Phase 5: User Story 4 — Fluent Subject Construction (Priority: P2)

**Goal**: Developers in both TypeScript and Python can build Subject values with a fluent builder: `Subject.build("id").label("L").property("k", v).done()`.

**Independent Test**: Build a Subject with multiple labels and properties in each language, verify all attributes.

### Implementation for User Story 4

- [ ] T029 [P] [US4] Add `WasmSubjectBuilder` struct with fields `identity: String`, `labels: Vec<String>`, `properties: HashMap<String, Value>` in `crates/pattern-core/src/wasm.rs`. Implement `label(&mut self, label: &str)`, `property(&mut self, key: &str, value: JsValue)`, and `done(&self)` returning `WasmSubject`.
- [ ] T030 [P] [US4] Add `PySubjectBuilder` struct with same fields in `crates/pattern-core/src/python.rs`. Implement `label(&mut self, label: &str)`, `property(&mut self, key: &str, value: &Bound<'_, PyAny>)` using `python_to_value()`, and `done(&self)` returning `PySubject`.
- [ ] T031 [US4] Add `build(identity: &str) -> WasmSubjectBuilder` static method to existing `WasmSubject` class in `crates/pattern-core/src/wasm.rs`
- [ ] T032 [US4] Add `build(identity: &str) -> PySubjectBuilder` static method to existing `PySubject` class in `crates/pattern-core/src/python.rs`
- [ ] T033 [US4] Re-export `WasmSubjectBuilder as SubjectBuilder` from `crates/pattern-wasm/src/lib.rs`
- [ ] T034 [US4] Register `PySubjectBuilder` in `#[pymodule] fn pattern_core` in `crates/pattern-core/src/python.rs`
- [ ] T035 [P] [US4] Add `SubjectBuilder` class and `Subject.build()` static method to TypeScript definitions in `crates/pattern-core/typescript/pattern_core.d.ts`
- [ ] T036 [P] [US4] Add `SubjectBuilder` class and `Subject.build()` static method to Python type stubs in `crates/pattern-core/pattern_core/__init__.pyi`

**Checkpoint**: `Subject.build("id").label("L").property("k", v).done()` works in both TypeScript and Python.

---

## Phase 6: User Story 5 — Escape Hatches to Graph Query Interface (Priority: P3)

**Goal**: TypeScript developers can call `asQuery()` and `asPatternGraph()` on StandardGraph to access existing algorithm and graph query interfaces.

**Independent Test**: Create StandardGraph, call `asQuery()`, verify returned `NativeGraphQuery` works with existing algorithm functions.

### Implementation for User Story 5

- [ ] T037 [US5] Implement `asPatternGraph(&self) -> WasmPatternGraph` on `WasmStandardGraph` that clones inner `PatternGraph` into `Rc` and wraps in `crates/pattern-core/src/wasm.rs`
- [ ] T038 [US5] Implement `asQuery(&self) -> WasmGraphQuery` on `WasmStandardGraph` that calls `self.inner.as_query()` and wraps in `crates/pattern-core/src/wasm.rs`
- [ ] T039 [US5] Add `asPatternGraph()` and `asQuery()` return types to `StandardGraph` class in `crates/pattern-core/typescript/pattern_core.d.ts`

**Checkpoint**: Escape hatches to `NativePatternGraph` and `NativeGraphQuery` work from StandardGraph.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Tests, examples, validation, and code quality.

- [ ] T040 [P] Create Python integration tests in `crates/pattern-core/tests/python/test_standard_graph.py` covering: empty graph, add_node, add_relationship, from_gram (valid + invalid), neighbors, degree, source, target, counts, is_empty, chaining, SubjectBuilder, __repr__, __len__. Include edge cases: neighbors of non-existent node (empty list), degree of non-existent node (0), source/target of non-existent relationship (None), duplicate identity merge behavior, empty gram string (empty graph).
- [ ] T041 [P] Create WASM/Node.js integration tests in `examples/pattern-core-wasm/test_standard_graph.mjs` covering: empty graph construction, addNode + addRelationship + chaining, fromGram (valid + invalid input), fromPatterns, fromPatternGraph, node/relationship access, nodeCount/relationshipCount/isEmpty, neighbors/degree/source/target queries, SubjectBuilder chaining, asQuery escape hatch, asPatternGraph escape hatch. Include edge cases: neighbors of non-existent node (empty array), degree of non-existent node (0), source/target of non-existent relationship (undefined), empty gram string (empty graph).
- [ ] T042 [P] Create WASM/Node.js example in `examples/pattern-core-wasm/standard_graph.mjs` demonstrating construction, gram parsing, querying, and SubjectBuilder
- [ ] T043 [P] Create Python example in `examples/pattern-core-python/standard_graph.py` demonstrating construction, gram parsing, querying, and SubjectBuilder
- [ ] T044 Run `cargo fmt --all` and `cargo clippy --workspace -- -D warnings` to verify code quality
- [ ] T045 Run `cargo test --workspace` to verify no regressions across all crates
- [ ] T046 Run `cargo build --workspace --target wasm32-unknown-unknown` to verify WASM compilation
- [ ] T047 Run `cd crates/pattern-core && maturin develop --uv --features python && pytest tests/python/` to verify Python bindings
- [ ] T048 Run `./scripts/ci-local.sh` for full CI validation. Additionally verify WASM integration tests pass via `node examples/pattern-core-wasm/test_standard_graph.mjs`.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — verify existing state
- **Foundational (Phase 2)**: Depends on Phase 1 — confirm conversion helpers
- **US1+US2 (Phase 3)**: Depends on Phase 2 — WASM StandardGraph + fromGram
- **US3 (Phase 4)**: Depends on Phase 2 — Python StandardGraph (can run in parallel with Phase 3)
- **US4 (Phase 5)**: Depends on Phase 2 — SubjectBuilder (can run in parallel with Phases 3-4)
- **US5 (Phase 6)**: Depends on Phase 3 — escape hatches need WasmStandardGraph
- **Polish (Phase 7)**: Depends on all prior phases

### User Story Dependencies

- **US1+US2 (P1)**: No dependencies on other stories. MVP deliverable.
- **US3 (P2)**: Independent of US1+US2 (different file: `python.rs` vs `wasm.rs`). Can run in parallel.
- **US4 (P2)**: Independent of US1-US3 (adds new types, doesn't modify StandardGraph). Can run in parallel.
- **US5 (P3)**: Depends on US1 (needs WasmStandardGraph to exist for escape hatch methods).

### Within Each User Story

- Struct + constructor first
- Core mutation methods (add_node, add_relationship)
- Accessors and counts
- Query methods
- Iteration methods
- Less-common methods last
- Re-exports and type definitions can run in parallel with implementation

### Parallel Opportunities

```
Phase 3 (WASM) ─────────────────────────────────────────────┐
Phase 4 (Python) ──────────────────────────────────────┐     │
Phase 5 (SubjectBuilder) ─────────────────────────┐    │     │
                                                   └────┴─────┴──→ Phase 7 (Polish)
```

Within Phase 3: T016 (TypeScript defs) can run in parallel with T004-T015 (Rust implementation).
Within Phase 4: T028 (Python stubs) can run in parallel with T017-T027 (Rust implementation).
Within Phase 5: T029+T030 (WASM+Python builders) can run in parallel. T035+T036 (defs+stubs) can run in parallel.
Within Phase 7: T040, T041, T042, T043 (tests+examples) can all run in parallel.

---

## Parallel Example: User Story 1+2

```bash
# After foundational phase, launch WASM implementation and TypeScript defs in parallel:
Task T004-T014: "Implement WasmStandardGraph in crates/pattern-core/src/wasm.rs"
Task T016: "Add StandardGraph TypeScript definitions in crates/pattern-core/typescript/pattern_core.d.ts"
```

## Parallel Example: User Story 4

```bash
# WASM and Python builders are in different files, can run in parallel:
Task T029: "Add WasmSubjectBuilder in crates/pattern-core/src/wasm.rs"
Task T030: "Add PySubjectBuilder in crates/pattern-core/src/python.rs"

# Type defs and stubs can run in parallel:
Task T035: "Add SubjectBuilder to TypeScript definitions"
Task T036: "Add SubjectBuilder to Python type stubs"
```

---

## Implementation Strategy

### MVP First (User Story 1+2 Only)

1. Complete Phase 1: Setup (verify starting state)
2. Complete Phase 2: Foundational (confirm helpers)
3. Complete Phase 3: US1+US2 (WASM StandardGraph + fromGram)
4. **STOP and VALIDATE**: Build WASM target, test construction and querying
5. This alone delivers full TypeScript graph capability

### Incremental Delivery

1. Setup + Foundational → Ready
2. US1+US2 (WASM StandardGraph) → Test → MVP!
3. US3 (Python StandardGraph) → Test → Python users unblocked
4. US4 (SubjectBuilder) → Test → Ergonomic construction in both targets
5. US5 (Escape hatches) → Test → Advanced users can access algorithms
6. Polish → Full CI validation, examples, tests

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- US1 and US2 are combined into one phase since they're both P1 and tightly coupled (fromGram is a construction method)
- WASM and Python implementations are in separate files and can proceed fully in parallel
- SubjectBuilder is independent of StandardGraph (new types, no modifications to graph)
- Python escape hatches are explicitly deferred per research.md (R9)
