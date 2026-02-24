# Tasks: GraphTransform — View-Based Graph Transformations

**Input**: Design documents from `specs/032-graph-transform/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Optional per spec. Test tasks appear in Polish phase only.

**Organization**: Tasks are grouped by user story so each story can be implemented and tested independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: User story (US1–US7)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `crates/pattern-core/src/`, `crates/pattern-core/tests/` (repository root relative)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Module layout and re-exports so GraphView and transform modules exist.

- [X] T001 Create `crates/pattern-core/src/graph/graph_view.rs` (empty or stub) and `crates/pattern-core/src/graph/transform/mod.rs` with placeholder re-exports
- [X] T002 Create `crates/pattern-core/src/graph/transform/types.rs` (stub) and `crates/pattern-core/src/graph/transform/map_filter_fold.rs`, `context.rs`, `para.rs`, `unfold_graph.rs` (empty stubs)
- [X] T003 Refactor pattern module into directory: create `crates/pattern-core/src/pattern/mod.rs` re-exporting from `pattern/core.rs`, move current `pattern.rs` content to `crates/pattern-core/src/pattern/core.rs`, add `crates/pattern-core/src/pattern/unfold.rs` (stub)
- [X] T004 Update `crates/pattern-core/src/graph/mod.rs` to add `pub mod graph_view` and `pub mod transform`
- [X] T005 Update `crates/pattern-core/src/lib.rs` to re-export `graph::graph_view::*` and `graph::transform::*` and `pattern::unfold` (once implemented)

---

## Phase 2: Foundational — User Story 1 (Universal Graph View) (Priority: P1) MVP

**Purpose**: GraphView, materialize, and shared types. Delivers US1. All other user stories depend on this.

**Independent Test**: Build a view from a PatternGraph, materialize with a policy, assert result is equivalent to source.

- [X] T006 [P] Define `Substitution<V>` enum (NoSubstitution, ReplaceWith, RemoveContainer) in `crates/pattern-core/src/graph/transform/types.rs`
- [X] T007 [P] Define `CategoryMappers<Extra, V>` struct and `identity()` in `crates/pattern-core/src/graph/transform/types.rs` per contracts/public-api.md
- [X] T008 Define `GraphView<Extra, V>` struct with `view_query` and `view_elements` in `crates/pattern-core/src/graph/graph_view.rs`
- [X] T009 Implement `from_pattern_graph(classifier, graph) -> GraphView` in `crates/pattern-core/src/graph/graph_view.rs` (build query from graph, build view_elements from graph contents classified)
- [X] T010 Implement `materialize(classifier, policy, view) -> PatternGraph` in `crates/pattern-core/src/graph/graph_view.rs`
- [X] T011 Add `from_graph_lens` placeholder (unimplemented! or todo! with comment) in `crates/pattern-core/src/graph/graph_view.rs`
- [X] T012 Update `crates/pattern-core/src/graph/transform/mod.rs` to export Substitution, CategoryMappers, and re-export from types
- [X] T013 Update `crates/pattern-core/src/lib.rs` to re-export GraphView, from_pattern_graph, materialize, Substitution, CategoryMappers

---

## Phase 3: User Story 2 — Build a Graph from Seeds (Unfold) (Priority: P2)

**Goal**: Single-pattern unfold and unfold_graph so users can build graphs from seeds (ETL).

**Independent Test**: Provide N seeds and an expander that returns one pattern per seed; unfold_graph produces a graph with N patterns (after reconciliation).

- [X] T014 [US2] Implement `unfold(expand, seed) -> Pattern<V>` iteratively (work stack) in `crates/pattern-core/src/pattern/unfold.rs`
- [X] T015 [US2] Implement `unfold_graph(classifier, policy, expand, seeds) -> PatternGraph` in `crates/pattern-core/src/graph/transform/unfold_graph.rs`
- [X] T016 [US2] Export `unfold` from `crates/pattern-core/src/pattern/mod.rs` and `unfold_graph` from `crates/pattern-core/src/graph/transform/mod.rs`; add lib.rs re-exports

---

## Phase 4: User Story 3 — Transform Elements by Category (Priority: P3)

**Goal**: map_graph and map_all_graph so users can transform by category or uniformly.

**Independent Test**: Build a view, apply map_graph with nodes-only mapper (others identity), materialize; assert nodes transformed and others unchanged.

- [X] T017 [US3] Implement `map_all_graph(f, view) -> GraphView` in `crates/pattern-core/src/graph/transform/map_filter_fold.rs`
- [X] T018 [US3] Implement `map_graph(classifier, mappers, view) -> GraphView` in `crates/pattern-core/src/graph/transform/map_filter_fold.rs`
- [X] T019 [US3] Re-export map_graph and map_all_graph from `crates/pattern-core/src/graph/transform/mod.rs` and `crates/pattern-core/src/lib.rs`

---

## Phase 5: User Story 4 — Filter Elements by Predicate (Priority: P4)

**Goal**: filter_graph with substitution policy for container behavior.

**Independent Test**: Build a view, filter with predicate keeping K elements and Substitution::NoSubstitution; materialize and assert K elements and walk gaps as specified.

- [X] T020 [US4] Implement `filter_graph(classifier, predicate, substitution, view) -> GraphView` in `crates/pattern-core/src/graph/transform/map_filter_fold.rs`
- [X] T021 [US4] Re-export filter_graph from `crates/pattern-core/src/graph/transform/mod.rs` and `crates/pattern-core/src/lib.rs`

---

## Phase 6: User Story 5 — Aggregate Over a Graph View (Priority: P5)

**Goal**: fold_graph so users can compute a single value from a view (e.g. count by class).

**Independent Test**: fold_graph with accumulator (e.g. HashMap category -> count); assert counts match view element counts per category.

- [X] T022 [US5] Implement `fold_graph(f, init, view) -> M` in `crates/pattern-core/src/graph/transform/map_filter_fold.rs`
- [X] T023 [US5] Re-export fold_graph from `crates/pattern-core/src/graph/transform/mod.rs` and `crates/pattern-core/src/lib.rs`

---

## Phase 7: User Story 6 — Map with Full Graph Context (Priority: P6)

**Goal**: map_with_context with snapshot semantics for context-aware enrichment.

**Independent Test**: View with annotated nodes; map_with_context sets node "count" from query_annotations_of snapshot; materialize and assert counts correct.

- [X] T024 [US6] Implement `map_with_context(classifier, f, view) -> GraphView` with snapshot semantics in `crates/pattern-core/src/graph/transform/context.rs`
- [X] T025 [US6] Re-export map_with_context from `crates/pattern-core/src/graph/transform/mod.rs` and `crates/pattern-core/src/lib.rs`

---

## Phase 8: User Story 7 — Topology-Aware Folding (Priority: P7)

**Goal**: para_graph (DAG, defined order) and para_graph_fixed (cyclic, converge).

**Independent Test**: DAG view + para_graph with max-predecessors-plus-one; root nodes 1, others max(pred)+1. Cyclic view + para_graph_fixed with convergence predicate; assert stabilization.

- [X] T026 [US7] Implement `para_graph(f, view) -> HashMap<V::Id, R>` with topological order for DAGs in `crates/pattern-core/src/graph/transform/para.rs`
- [X] T027 [US7] Implement `para_graph_fixed(converged, f, init, view) -> HashMap<V::Id, R>` in `crates/pattern-core/src/graph/transform/para.rs`
- [X] T028 [US7] Re-export para_graph and para_graph_fixed from `crates/pattern-core/src/graph/transform/mod.rs` and `crates/pattern-core/src/lib.rs`

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Tests, formatting, and validation.

- [X] T029 [P] Add tests for view construction and materialize round-trip in `crates/pattern-core/tests/graph_view.rs`
- [X] T030 [P] Add tests for transform (map, filter, fold, map_with_context, para, unfold_graph) in `crates/pattern-core/tests/transform.rs`
- [ ] T031 Add equivalence tests vs pattern-hs (or document how graph_view/transform tests satisfy FR-015 and SC-009) per constitution reference verification; reference `../pattern-hs/libs/pattern/tests/Spec/Pattern/Graph/TransformSpec.hs` and `GraphSpec.hs` as needed
- [X] T032 Run `cargo fmt --all` and `cargo clippy --workspace -- -D warnings`; fix any issues in `crates/pattern-core/`
- [ ] T033 Validate quickstart.md examples (doc tests or manual run) per `specs/032-graph-transform/quickstart.md`

**Note**: Multi-target verification (WASM, Python) is assumed to be covered by existing CI or workspace-level scripts; add an explicit task here if this feature introduces new target-specific code.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies — start immediately
- **Phase 2 (Foundational)**: Depends on Phase 1 — blocks all user stories
- **Phase 3–8 (User Stories)**: Depend on Phase 2; stories can be done in order P2→P3→…→P7 or parallel if staffed
- **Phase 9 (Polish)**: Depends on Phases 2–8 complete (or at least stories to be validated)

### User Story Dependencies

- **US1 (P1)**: Delivered by Phase 2 (Foundational) — view + materialize
- **US2 (P2)**: After Phase 2 — no dependency on US3–US7
- **US3 (P3)**: After Phase 2 — no dependency on US4–US7
- **US4 (P4)**: After Phase 2 — no dependency on US5–US7
- **US5 (P5)**: After Phase 2 — no dependency on US6–US7
- **US6 (P6)**: After Phase 2 — no dependency on US7
- **US7 (P7)**: After Phase 2 — uses view and query (from Phase 2)

### Within Each User Story

- Implement functions before re-export
- Core logic before lib re-exports

### Parallel Opportunities

- T006 and T007 (Substitution, CategoryMappers) can run in parallel (same file; [P] means order-independent—either can be implemented first without blocking the other)
- After Phase 2: US2, US3, US4, US5 can be developed in parallel (different files: unfold/unfold_graph, map_filter_fold, context, para)
- T029 and T030 (test files) can run in parallel

---

## Parallel Example: After Foundational

```bash
# Option A: Sequential by story
# Complete T014–T016 (US2), then T017–T019 (US3), then T020–T021 (US4), etc.

# Option B: Parallel by file (different owners)
# Dev A: pattern/unfold.rs + transform/unfold_graph.rs (US2)
# Dev B: transform/map_filter_fold.rs (US3, US4, US5)
# Dev C: transform/context.rs (US6)
# Dev D: transform/para.rs (US7)
```

---

## Implementation Strategy

### MVP First (User Story 1 = Phase 2)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (GraphView, from_pattern_graph, materialize, Substitution, CategoryMappers)
3. **Checkpoint**: Build view from PatternGraph, materialize, assert equivalence — MVP done
4. Then add US2–US7 in order or parallel

### Incremental Delivery

1. Phase 1 + 2 → View and materialize (US1)
2. Phase 3 → Unfold / unfold_graph (US2)
3. Phase 4 → map_graph / map_all_graph (US3)
4. Phase 5 → filter_graph (US4)
5. Phase 6 → fold_graph (US5)
6. Phase 7 → map_with_context (US6)
7. Phase 8 → para_graph / para_graph_fixed (US7)
8. Phase 9 → Tests and polish

### Parallel Team Strategy

- After Phase 2: split US2 (unfold), US3+US4+US5 (map/filter/fold in same file), US6 (context), US7 (para) across developers; then merge and run Phase 9.

---

## Notes

- [P] = different files, no shared mutable state
- [USn] = task belongs to that user story for traceability
- Each story is independently testable via its Independent Test in spec.md
- Commit after each task or logical group
- Pattern module refactor (T003): current `pattern.rs` becomes `pattern/core.rs`; `pattern/mod.rs` re-exports core and adds `mod unfold`; `pattern/unfold.rs` holds the anamorphism
