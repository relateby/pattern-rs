# Tasks: GraphQuery — Portable, Composable Graph Query Interface

**Input**: Design documents from `/specs/031-graph-query/`
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/public-api.md ✓

**Organization**: Tasks grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no shared state dependencies)
- **[Story]**: Maps to user story from spec.md (US1–US4)

---

## Phase 1: Setup

**Purpose**: Project configuration before any code is written.

- [X] T001 Add `thread-safe = []` feature flag to `crates/pattern-core/Cargo.toml`

---

## Phase 2: Foundational — Core Interface Types

**Purpose**: Define `TraversalDirection`, `TraversalWeight<V>`, and the `GraphQuery<V>` struct. All user stories depend on these types existing.

**⚠️ CRITICAL**: No user story implementation can begin until this phase is complete.

- [X] T002 Create `crates/pattern-core/src/graph/graph_query.rs` with `TraversalDirection` enum (`Forward`, `Backward`; derives `Debug, Clone, Copy, PartialEq, Eq`)
- [X] T003 [P] Add `TraversalWeight<V>` type alias (`Rc<dyn Fn(&Pattern<V>, TraversalDirection) -> f64>`) and feature-gated `Arc` variant in `crates/pattern-core/src/graph/graph_query.rs`
- [X] T004 [P] Add canonical weight functions `undirected`, `directed`, `directed_reverse` in `crates/pattern-core/src/graph/graph_query.rs`
- [X] T005 Define `GraphQuery<V>` struct with all 9 `Rc<dyn Fn(...)>` fields (`query_nodes`, `query_relationships`, `query_incident_rels`, `query_source`, `query_target`, `query_degree`, `query_node_by_id`, `query_relationship_by_id`, `query_containers`) with full doc comments and invariant list in `crates/pattern-core/src/graph/graph_query.rs`
- [X] T006 Implement manual `Clone` for `GraphQuery<V>` using `Rc::clone` per field in `crates/pattern-core/src/graph/graph_query.rs`
- [X] T007 Update `crates/pattern-core/src/graph/mod.rs` to add `pub mod graph_query;` and re-export `TraversalDirection`, `TraversalWeight`, `undirected`, `directed`, `directed_reverse`, `GraphQuery`
- [X] T008 Update `crates/pattern-core/src/lib.rs` to re-export `TraversalDirection`, `TraversalWeight`, `undirected`, `directed`, `directed_reverse`, `GraphQuery` from `graph` module

**Checkpoint**: `cargo build -p pattern-core` compiles with the new types; `GraphQuery<V>` and weight functions are accessible from crate root.

---

## Phase 3: User Story 1 — Run Algorithms on Any Backing Store (P1) 🎯 MVP

**Goal**: Wrap a `PatternGraph` in a `GraphQuery` and run core traversal and connectivity algorithms against it.

**Independent Test**: Construct a `GraphQuery` from a `PatternGraph` with 3–5 nodes and relationships; verify BFS, DFS, shortest-path, connected-components, and degree-centrality produce correct results. No other user story needed.

### Implementation for User Story 1

- [X] T009 [US1] Add `from_pattern_graph` constructor to `crates/pattern-core/src/pattern_graph.rs` implementing all 9 fields: `query_nodes` and `query_relationships` from HashMap values; `query_incident_rels` and `query_degree` via source/target identity scan; `query_source`/`query_target` from `rel.elements[0]`/`rel.elements[1]`; `query_node_by_id`/`query_relationship_by_id` as direct HashMap lookups; `query_containers` scanning `pg_relationships`, `pg_walks`, `pg_annotations`; add TODO comment for deferred `from_graph_lens`
- [X] T010 [US1] Update `crates/pattern-core/src/lib.rs` to re-export `from_pattern_graph`
- [X] T011 [US1] Create `crates/pattern-core/src/graph/algorithms.rs` with private `reachable_neighbors` helper (inline-annotated; filters incident rels by finite traversal cost; returns neighbor node for each passable rel) and `bfs` implementation (VecDeque queue, HashSet visited, returns nodes in visit order including start)
- [X] T012 [P] [US1] Add `dfs` to `crates/pattern-core/src/graph/algorithms.rs` (Vec stack, HashSet visited, returns nodes in DFS order including start)
- [X] T013 [P] [US1] Add `is_neighbor` to `crates/pattern-core/src/graph/algorithms.rs` (checks if any reachable neighbor has matching identity)
- [X] T014 [P] [US1] Add `is_connected` to `crates/pattern-core/src/graph/algorithms.rs` (empty graph = true; otherwise BFS length == query_nodes length)
- [X] T015 [US1] Add `shortest_path` to `crates/pattern-core/src/graph/algorithms.rs` (Dijkstra with BTreeMap priority queue; same-node returns `Some(vec![node])`; disconnected returns `None`)
- [X] T016 [P] [US1] Add `has_path` to `crates/pattern-core/src/graph/algorithms.rs` (delegates to `shortest_path`)
- [X] T017 [P] [US1] Add `connected_components` to `crates/pattern-core/src/graph/algorithms.rs` (BFS per unvisited node; returns `Vec<Vec<Pattern<V>>>`)
- [X] T018 [P] [US1] Add `degree_centrality` to `crates/pattern-core/src/graph/algorithms.rs` (no `TraversalWeight` param; uses `query_degree`; normalizes by `n-1`; returns `HashMap<V::Id, f64>`)
- [X] T019 Update `crates/pattern-core/src/graph/mod.rs` to add `pub mod algorithms;` and re-export `bfs`, `dfs`, `is_neighbor`, `is_connected`, `shortest_path`, `has_path`, `connected_components`, `degree_centrality`
- [X] T020 Update `crates/pattern-core/src/lib.rs` to re-export algorithms from T019
- [X] T021 [P] [US1] Create `crates/pattern-core/tests/graph_query.rs` with construction tests: HS-T015 (all 9 `GraphQuery` fields return correct results from `from_pattern_graph`), HS-T016 (structural invariants hold for any valid graph), HS-T017 (`undirected`/`directed`/`directed_reverse` return correct costs for Forward/Backward)
- [X] T022 [US1] Create `crates/pattern-core/tests/algorithms.rs` with correctness tests for `bfs` (visit order, includes start), `dfs` (DFS order), `shortest_path` (min-cost path, same-node, disconnected), `has_path`, `connected_components` (correct partition), `degree_centrality` (star graph: center=1.0, leaf=1/(n-1))
- [X] T022b [P] [US1] Add representation-independence test to `crates/pattern-core/tests/algorithms.rs`: construct a `GraphQuery<Subject>` by hand using literal closures (no `PatternGraph` backing); run `bfs` and `connected_components` against it; verify correct results — confirms algorithms depend only on the `GraphQuery` interface, not on `PatternGraph` internals (SC-002)

**Checkpoint**: `cargo test -p pattern-core -- graph_query algorithms` passes. A `PatternGraph` can be wrapped and queried with any of the 7 algorithms above.

---

## Phase 4: User Story 2 — Control Traversal Direction at Call Site (P2)

**Goal**: Demonstrate and verify that the same `GraphQuery` produces different traversal results when `directed`, `undirected`, or `directed_reverse` weights are supplied.

**Independent Test**: Build a directed graph A→B→C; run BFS with `directed()` from A (reaches B, C) and from C with `directed_reverse()` (reaches B, A); run same graph with `undirected()` from any node (reaches all nodes). All verifiable with `from_pattern_graph` from US1.

### Implementation for User Story 2

- [X] T023 [P] [US2] Add `topological_sort` to `crates/pattern-core/src/graph/algorithms.rs` (DFS post-order with in-stack cycle detection; ignores `TraversalWeight`; uses `query_source`/`query_target` only; returns `None` on cycle)
- [X] T024 [P] [US2] Add `has_cycle` to `crates/pattern-core/src/graph/algorithms.rs` (delegates to `topological_sort`)
- [X] T025 [P] [US2] Add `all_paths` to `crates/pattern-core/src/graph/algorithms.rs` (DFS simple-path enumeration; no repeated nodes; returns `Vec<Vec<Pattern<V>>>`)
- [X] T026 Update `crates/pattern-core/src/graph/mod.rs` and `crates/pattern-core/src/lib.rs` to re-export `topological_sort`, `has_cycle`, `all_paths`
- [X] T027 [US2] Add traversal direction tests to `crates/pattern-core/tests/algorithms.rs`: directed BFS on A→B→C from A returns only forward-reachable nodes; backward traversal from C returns A and B; undirected from any node reaches all nodes; custom weighted `shortest_path` respects costs
- [X] T028 [US2] Add structural algorithm tests to `crates/pattern-core/tests/algorithms.rs`: `topological_sort` on a DAG returns valid topological order; cyclic graph returns `None`; `has_cycle` returns correct boolean; `all_paths` on simple graph returns all simple paths

**Checkpoint**: `cargo test -p pattern-core -- algorithms` passes all traversal direction tests. Same `GraphQuery` value used with different weights produces verifiably different results.

---

## Phase 5: User Story 3 — Compose and Restrict Graph Views (P3)

**Goal**: Create restricted graph view (frame) and memoized queries that work transparently with all existing algorithms.

**Independent Test**: Wrap a mixed-type graph in `frame_query` with a label predicate; verify only matching nodes appear in algorithm results. Wrap the same graph in `memoize_incident_rels`; verify results match non-memoized version. Requires US1 complete.

### Implementation for User Story 3

- [X] T029 [US3] Add `frame_query` combinator to `crates/pattern-core/src/graph/graph_query.rs`: filter `query_nodes` and `query_relationships` by predicate; filter `query_incident_rels` to exclude rels with endpoints outside frame; filter `query_node_by_id` and `query_relationship_by_id` results by predicate; filter `query_containers` results by predicate; recalculate `query_degree` as filtered incident rel count
- [X] T030 [US3] Add `memoize_incident_rels` combinator to `crates/pattern-core/src/graph/graph_query.rs`: eagerly build `HashMap<V::Id, Vec<Pattern<V>>>` from all nodes at construction time; wrap `query_incident_rels` and `query_degree` to serve from cache; all other fields pass through unchanged
- [X] T031 [P] [US3] Add `minimum_spanning_tree` to `crates/pattern-core/src/graph/algorithms.rs` (Kruskal's with path-compression union-find; edge cost = `min(fwd, bwd)`; infinite-cost edges excluded; returns `Vec<Pattern<V>>` of nodes in MST)
- [X] T032 [P] [US3] Add `betweenness_centrality` to `crates/pattern-core/src/graph/algorithms.rs` (Brandes algorithm: BFS phase to compute sigma/pred/dist per source node; back-propagation phase for delta accumulation; unnormalized scores; returns `HashMap<V::Id, f64>`)
- [X] T033 Update `crates/pattern-core/src/graph/mod.rs` and `crates/pattern-core/src/lib.rs` to re-export `frame_query`, `memoize_incident_rels`, `minimum_spanning_tree`, `betweenness_centrality`
- [X] T034 [US3] Add frame combinator tests to `crates/pattern-core/tests/graph_query.rs`: HS-T047 (nodes outside predicate excluded from `query_nodes`), HS-T048 (`query_incident_rels` excludes rels with endpoints outside frame), HS-T049 (`memoize_incident_rels` returns same results as base), HS-T050 (`query_degree` equals `len(query_incident_rels)` after memoize), HS-T051 (all 7 structural invariants hold on framed query)
- [X] T035 [US3] Add composition and advanced algorithm tests to `crates/pattern-core/tests/algorithms.rs`: `minimum_spanning_tree` on 3-node weighted graph; `betweenness_centrality` on path graph (middle node scores higher than endpoints); `frame_query` + BFS produces results consistent with predicate filter

**Checkpoint**: `cargo test -p pattern-core -- graph_query algorithms` passes frame and combinator tests. Frame results are consistent across all algorithms.

---

## Phase 6: User Story 4 — Query Context and Containers (P4)

**Goal**: Find annotations attached to elements, walks containing relationships, and co-members sharing a container.

**Independent Test**: Build a graph with annotations and walks; call each context helper and verify only the correct elements are returned. Requires US1 (`from_pattern_graph` with `query_containers`) complete.

### Implementation for User Story 4

- [X] T036 [P] [US4] Add `query_annotations_of(classifier: &GraphClassifier<Extra, V>, q: &GraphQuery<V>, element: &Pattern<V>)` to `crates/pattern-core/src/graph/algorithms.rs`: filters `query_containers` results by `GraphClass::GAnnotation` using classifier
- [X] T037 [P] [US4] Add `query_walks_containing(classifier: &GraphClassifier<Extra, V>, q: &GraphQuery<V>, element: &Pattern<V>)` to `crates/pattern-core/src/graph/algorithms.rs`: filters `query_containers` results by `GraphClass::GWalk` using classifier
- [X] T038 [P] [US4] Add `query_co_members(q: &GraphQuery<V>, element: &Pattern<V>, container: &Pattern<V>)` to `crates/pattern-core/src/graph/algorithms.rs`: returns `container.elements` excluding `element` (matched by identity)
- [X] T039 Update `crates/pattern-core/src/graph/mod.rs` and `crates/pattern-core/src/lib.rs` to re-export `query_annotations_of`, `query_walks_containing`, `query_co_members`
- [X] T040 [US4] Add context helper tests to `crates/pattern-core/tests/graph_query.rs`: HS-T056 (`query_containers` returns correct containers); `query_annotations_of` returns only annotation patterns; `query_walks_containing` returns only walk patterns; `query_co_members` returns correct co-members within a walk, excluding the queried element

**Checkpoint**: `cargo test -p pattern-core -- graph_query` passes all context helper tests. All 4 user stories are independently functional.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Code quality, WASM verification, thread-safe feature validation, constitution compliance.

- [X] T041 Run `cargo fmt --all` in repo root and fix any formatting issues across all modified files
- [X] T042 [P] Run `cargo clippy --workspace -- -D warnings` and fix all warnings in new files (`graph_query.rs`, `algorithms.rs`, modified `pattern_graph.rs`, `lib.rs`, `mod.rs`)
- [X] T043 Run `cargo build --workspace` and confirm native build succeeds with no errors
- [X] T044 Run `cargo build --workspace --target wasm32-unknown-unknown` and confirm all new closures and data structures compile for WASM target
- [X] T045 Run `cargo build --workspace --features thread-safe` and confirm the `thread-safe` Cargo feature compiles (Arc variant of GraphQuery and TraversalWeight must build without errors)
- [X] T046 Run `cargo test --workspace` and confirm all tests pass

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on T001 — **BLOCKS all user stories**
- **US1 (Phase 3)**: Depends on Foundational completion (T002–T008)
- **US2 (Phase 4)**: Depends on US1 completion (particularly T011–T020 for algorithms.rs and tests)
- **US3 (Phase 5)**: Depends on Foundational completion; benefits from US1 for testing
- **US4 (Phase 6)**: Depends on US1 completion (needs `from_pattern_graph` with `query_containers`)
- **Polish (Phase 7)**: Depends on all user stories complete

### User Story Dependencies

| Story | Depends on | Shares files with |
|-------|-----------|-------------------|
| US1 (P1) | Foundational types (Phase 2) | `algorithms.rs` (create) |
| US2 (P2) | US1 types + algorithms file | `algorithms.rs`, test file |
| US3 (P3) | Foundational types; benefits from US1 | `graph_query.rs`, `algorithms.rs` |
| US4 (P4) | US1 `from_pattern_graph` (`query_containers`) | `algorithms.rs`, test file |

### Within Each User Story

- Constructor/struct tasks before algorithm tasks (need the type to call functions)
- Core algorithms before tests
- All `[P]`-marked tasks within a phase can run in parallel

---

## Parallel Opportunities

### Phase 2 (Foundational)

```
T002 (TraversalDirection) → T005 (GraphQuery struct) → T006 (Clone)
T003 [P] (TraversalWeight alias)    ↗
T004 [P] (canonical weights)        ↗
```

### Phase 3 (US1)

```
T009 (from_pattern_graph) → T011 (algorithms.rs + bfs)
T012 [P] (dfs)                ↗  all can start after T011 creates the file
T013 [P] (is_neighbor)        ↗
T014 [P] (is_connected)       ↗
T015 (shortest_path)          →  T016 [P] (has_path)
T017 [P] (connected_components) ↗
T018 [P] (degree_centrality)  ↗
T021 [P] (graph_query tests)  — independent of algorithm tasks
```

### Phase 5 (US3)

```
T029 (frame_query) → T034 (frame tests)
T030 (memoize)
T031 [P] (MST)     ↗  both can start after algorithms.rs exists
T032 [P] (betweenness) ↗
```

---

## Implementation Strategy

### MVP (User Story 1 Only)

1. Complete Phase 1: T001
2. Complete Phase 2: T002–T008 (types and struct)
3. Complete Phase 3: T009–T022 (constructor + 7 algorithms + tests)
4. **STOP and VALIDATE**: `cargo test -p pattern-core -- graph_query algorithms`
5. MVP complete — `PatternGraph` graphs are fully queryable with core algorithms

### Incremental Delivery

1. Phases 1–2 → Core types ready
2. Phase 3 (US1) → MVP with 7 algorithms; independently testable
3. Phase 4 (US2) → 3 more algorithms + traversal direction verification
4. Phase 5 (US3) → Composable views + 2 advanced algorithms
5. Phase 6 (US4) → Context queries for higher-level tooling
6. Phase 7 → CI green, all targets verified

---

## Notes

- `[P]` tasks share no file with other `[P]` tasks in the same phase, or write to different sections
- Test file tasks (`T021`, `T022`, `T027`, `T028`, `T031`, `T032`, `T034`, `T035`, `T040`) write to existing test files — execute sequentially within each phase, but can run in parallel with implementation tasks in the same phase
- `algorithms.rs` is created in T011; subsequent algorithm tasks append to it — execute T011 first, then T012–T018 can proceed in parallel
- Haskell test IDs referenced with `HS-` prefix to distinguish from task IDs: HS-T015–HS-T017 (construction), HS-T047–HS-T051 (frame + memoize), HS-T056 (containers)
- `from_graph_lens` is intentionally absent — a TODO comment is added in T009; requires a separate GraphLens feature
