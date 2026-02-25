# Tasks: TypeScript/WASM Graph API

**Input**: Design documents from `/specs/033-typescript-wasm-graph/`  
**Branch**: `033-typescript-wasm-graph`  
**Date**: 2026-02-25

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story. User Story 4 (Install and Initialize) is P1 alongside US1 and is placed first as it establishes the package scaffolds that all other stories depend on.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (US1–US4)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the three scoped npm package directories and shared build tooling. No user story work can begin until this scaffold exists.

- [X] T001 Create `typescript/@relateby/graph/` directory with `package.json` (no runtime deps, optional `effect` peer), `tsconfig.json`, and `src/index.ts` stub
- [X] T002 [P] Create `typescript/@relateby/pattern/` directory with `package.json` (depends on `@relateby/graph`, optional `effect` peer), `tsconfig.json`, and `src/index.ts` stub
- [X] T003 [P] Create `typescript/@relateby/gram/` directory with `package.json` (depends on `@relateby/pattern`), `tsconfig.json`, and `src/index.ts` stub
- [X] T004 Add `.gitignore` entries for `typescript/@relateby/*/wasm/`, `typescript/@relateby/*/dist/`, and `typescript/@relateby/*/node_modules/` in repo root `.gitignore`
- [X] T005 Add `wasm-pack` build script to `typescript/@relateby/pattern/package.json`: `build:wasm` (wasm-pack → `wasm/`), `build:ts` (tsc), `build` (both), `test` (vitest run)
- [X] T006 [P] Add `vitest` dev dependency and `vitest.config.ts` to each of the three packages

**Checkpoint**: Three empty package scaffolds exist; build scripts are wired; no implementation yet.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Rust WASM additions that all TypeScript packages depend on. Must complete before any TypeScript implementation can be tested end-to-end.

**⚠️ CRITICAL**: No WASM-dependent TypeScript work can be tested until this phase is complete. `@relateby/graph` (pure TS) can be developed in parallel.

- [X] T007 Add `WasmPatternGraph` struct to `crates/pattern-core/src/wasm.rs` wrapping `PatternGraph<(), Subject>` with `#[wasm_bindgen]` and `js_name = "NativePatternGraph"`: static `from_patterns`, `empty`; getters `nodes`, `relationships`, `walks`, `annotations`, `conflicts`, `size`; methods `merge`, `topo_sort`
- [X] T008 Add `WasmReconciliationPolicy` struct to `crates/pattern-core/src/wasm.rs` with `js_name = "NativeReconciliationPolicy"`: static constructors `last_write_wins`, `first_write_wins`, `strict`, `merge`
- [X] T009 [P] Add `WasmGraphClass` constant object to `crates/pattern-core/src/wasm.rs` (string constants `NODE`, `RELATIONSHIP`, `ANNOTATION`, `WALK`, `OTHER`)
- [X] T010 Add `WasmGraphQuery` struct to `crates/pattern-core/src/wasm.rs` wrapping `GraphQuery<Subject>` via `Rc` with `js_name = "NativeGraphQuery"`: static `from_pattern_graph`; methods `nodes`, `relationships`, `source`, `target`, `incident_rels`, `degree`, `node_by_id`, `relationship_by_id`
- [X] T011 [P] Add `WasmTraversalDirection` constant object to `crates/pattern-core/src/wasm.rs` (string constants `FORWARD`, `BACKWARD`)
- [X] T012 Add free algorithm functions to `crates/pattern-core/src/wasm.rs` with `#[wasm_bindgen]`: `bfs`, `dfs`, `shortest_path`, `all_paths`, `connected_components`, `has_cycle`, `is_connected`, `topological_sort`, `degree_centrality`, `betweenness_centrality`, `minimum_spanning_tree`, `query_walks_containing`, `query_co_members`, `query_annotations_of`
- [X] T013 Implement weight bridge in `crates/pattern-core/src/wasm.rs`: `JsValue` weight param → map string constants to `TraversalWeight` constructors; wrap JS `Function` in `Rc<dyn Fn(...)>` closure for custom weight callbacks
- [X] T014 Re-export all new WASM types from `crates/pattern-wasm/src/lib.rs` under their `Native*` JS names
- [X] T015 Verify `cargo build --workspace --target wasm32-unknown-unknown` passes; fix any `wasm-bindgen` constraint violations in `crates/pattern-core/src/wasm.rs`
- [X] T016 [P] Verify `cargo test --workspace` and `cargo clippy --workspace -- -D warnings` pass; fix any issues in `crates/pattern-core/src/`

**Checkpoint**: `wasm-pack build crates/pattern-wasm --target bundler` produces a valid `wasm/` directory with `NativePatternGraph`, `NativeReconciliationPolicy`, `NativeGraphQuery`, and all algorithm functions exported.

---

## Phase 3: User Story 4 — Install and Initialize the Package (Priority: P1) 🎯 MVP prerequisite

**Goal**: All three scoped packages are installable, `@relateby/pattern` initializes correctly in Node.js and bundler environments, and `@relateby/graph` works without WASM.

**Independent Test**: Install each package in a fresh project; call `init()` from `@relateby/pattern`; verify `NativePattern`, `Gram`, and all graph functions are accessible from their documented entry points. Install only `@relateby/graph` and verify all interfaces and transforms are available without WASM.

### Implementation for User Story 4

- [X] T017 [US4] Implement `typescript/@relateby/pattern/src/index.ts`: export `init()` (wasm-pack glue), `NativePattern`, `NativeSubject`, `NativeValue`, `NativePatternGraph`, `NativeReconciliationPolicy`, `NativeGraphQuery`, `NativeValidationRules`, `NativeStructureAnalysis`, `GraphClass`, `TraversalDirection`, and all algorithm functions; return types declared against `@relateby/graph` interfaces (e.g., `NativePatternGraph.fromPatterns()` returns `PatternGraph<Subject>`)
- [X] T018 [US4] Add Effect detection shim to `typescript/@relateby/pattern/src/index.ts`: when `effect` is available, wrap `{ _tag: 'Right'/'Left' }` WASM returns as `Either.Either<T,E>` and nullable returns as `Option.Option<T>`; when absent, return raw shapes or `T | null`
- [X] T019 [P] [US4] Implement `typescript/@relateby/gram/src/index.ts`: re-export `Gram.parse` and `Gram.stringify` from WASM via `@relateby/pattern`
- [X] T020 [US4] Add bundler-auto-init path to `typescript/@relateby/pattern/src/index.ts`: rely on the `wasm-pack --target bundler` generated glue, which handles automatic initialization via ES module top-level await; preserve explicit `await init()` export for Node.js and other non-bundler environments (do NOT use `import.meta` or `document` detection — the bundler target handles this at build time)
- [X] T021 [US4] Build `@relateby/pattern` end-to-end: run `npm run build` in `typescript/@relateby/pattern/`; confirm `dist/` and `wasm/` are produced
- [X] T022 [P] [US4] Build `@relateby/gram` end-to-end: run `npm run build` in `typescript/@relateby/gram/`; confirm `dist/` is produced

**Checkpoint**: `@relateby/pattern` and `@relateby/gram` build and initialize. `@relateby/graph` scaffold exists. US4 acceptance scenarios 1–6 are verifiable.

---

## Phase 4: User Story 1 — Build and Query a Graph from Patterns (Priority: P1) 🎯 MVP

**Goal**: A developer can construct a `NativePatternGraph` from patterns, create a `NativeGraphQuery`, and run traversal and path-finding functions.

**Independent Test**: Construct a `NativePatternGraph` from node and relationship patterns; create a `NativeGraphQuery`; verify `nodes()`, `relationships()`, `source()`, `target()`, `bfs()`, `dfs()`, `shortestPath()`, and `merge()` return correct results.

### Implementation for User Story 1

- [X] T023 [US1] Implement `NativePatternGraph` TypeScript declaration augmentation in `typescript/@relateby/pattern/src/index.ts` (extends T017's exports — do not replace): declare `fromPatterns` return type as `PatternGraph<Subject>`, `nodes`/`relationships`/`walks`/`annotations` as `readonly Pattern<Subject>[]`, `conflicts` as `Record<string, readonly Pattern<Subject>[]>`, `topoSort()` as `readonly Pattern<Subject>[]`
- [X] T024 [US1] Implement `NativeGraphQuery` TypeScript declaration augmentation in `typescript/@relateby/pattern/src/index.ts` (extends T017's exports): declare `fromPatternGraph` return type as `GraphQuery<Subject>`; all method signatures typed against `Pattern<Subject>` from `@relateby/graph`
- [X] T025 [US1] Type algorithm function signatures in `typescript/@relateby/pattern/src/index.ts` (extends T017's exports): `bfs`, `dfs`, `shortestPath`, `allPaths`, `connectedComponents`, `queryWalksContaining`, `queryCoMembers`, `queryAnnotationsOf` typed against `GraphQuery<Subject>` and `Pattern<Subject>`; `shortestPath` returns `Pattern<Subject>[] | null` (or `Option<Pattern<Subject>[]>` with Effect)
- [X] T026 [US1] Write vitest integration test in `typescript/@relateby/pattern/tests/pattern.test.ts`: construct graph from node + relationship patterns; verify `nodes.length`, `relationships.length`; call `bfs` and verify traversal order; call `shortestPath` and verify path; call `merge` with strict policy and verify `conflicts`
- [X] T027 [US1] Write vitest integration test in `typescript/@relateby/pattern/tests/pattern.test.ts` (separate `describe` block from T026): verify `NativeReconciliationPolicy.lastWriteWins()`, `firstWriteWins()`, `strict()`, and `merge()` all construct without error; verify `strict` records conflict in `graph.conflicts`
- [X] T028 [US1] Run `npm test` in `typescript/@relateby/pattern/`; fix any failures

**Checkpoint**: US1 acceptance scenarios 1–5 pass. A developer can build a graph, query it, and run traversals in under 10 lines (SC-001).

---

## Phase 5: User Story 2 — Analyze Graph Structure (Priority: P2)

**Goal**: A developer can call structural analysis functions (`hasCycle`, `isConnected`, `connectedComponents`, `degreeCentrality`, `betweennessCentrality`, `topologicalSort`, `minimumSpanningTree`) on a `NativeGraphQuery` and receive correct results.

**Independent Test**: Construct known graphs (cyclic, acyclic, disconnected, weighted); verify each analysis function returns the expected value.

### Implementation for User Story 2

- [X] T029 [US2] Type analysis algorithm signatures in `typescript/@relateby/pattern/src/index.ts`: `hasCycle`, `isConnected` → `boolean`; `connectedComponents` → `Pattern<Subject>[][]`; `topologicalSort` → `Pattern<Subject>[] | null` (or `Option` with Effect); `degreeCentrality`, `betweennessCentrality` → `Record<string, number>`; `minimumSpanningTree` → `Pattern<Subject>[]`
- [X] T030 [US2] Write vitest tests in `typescript/@relateby/pattern/tests/pattern.test.ts` for structural analysis: cyclic graph → `hasCycle` returns `true`; acyclic → `false`; disconnected graph → `connectedComponents` returns correct groups; DAG → `topologicalSort` returns valid order; cyclic → `topologicalSort` returns `null`/`None`
- [X] T031 [P] [US2] Write vitest tests in `typescript/@relateby/pattern/tests/pattern.test.ts` for centrality: `degreeCentrality` returns normalized scores; `betweennessCentrality` returns scores; `minimumSpanningTree` returns correct relationship set on a weighted graph
- [X] T032 [US2] Run `npm test` in `typescript/@relateby/pattern/`; fix any failures

**Checkpoint**: US2 acceptance scenarios 1–5 pass. All structural analysis functions produce results consistent with Haskell reference (SC-002).

---

## Phase 6: User Story 3 — Transform Graphs with Pure TypeScript Functions (Priority: P3)

**Goal**: `@relateby/graph` exports `Subject`, `Pattern<V>`, `PatternGraph<V>`, `GraphQuery<V>`, `GraphView<V>`, `toGraphView`, all ADTs, and all transform functions. All transforms work with plain TypeScript stubs (no WASM).

**Independent Test**: Call `toGraphView` with a plain TS stub satisfying `PatternGraph<Subject>`; verify `mapGraph`, `filterGraph`, `foldGraph`, `mapWithContext`, `paraGraph`, `paraGraphFixed`, and `unfoldGraph` produce correct results without WASM initialization.

### Implementation for User Story 3

- [X] T033 [US3] Implement core interfaces in `typescript/@relateby/graph/src/index.ts`: export `Subject`, `Pattern<V>`, `PatternGraph<V>`, `GraphQuery<V>`, `GraphView<V>` as TypeScript interfaces per `contracts/ts-api.md`
- [X] T034 [US3] Implement `toGraphView<V>` in `typescript/@relateby/graph/src/index.ts`: read the pre-classified arrays (`nodes`, `relationships`, `walks`, `annotations`) from `PatternGraph<V>` and map them to `[GraphClass, Pattern<V>]` pairs (no re-classification); pair with the `PatternGraph<V>` itself cast as `GraphQuery<V>` snapshot; return `GraphView<V>`
- [X] T035 [P] [US3] Implement `GraphClass` discriminated union and smart constructors (`GNode`, `GRelationship`, `GWalk`, `GAnnotation`, `GOther`) in `typescript/@relateby/graph/src/index.ts`
- [X] T036 [P] [US3] Implement `Substitution` discriminated union and smart constructors (`DeleteContainer`, `SpliceGap`, `ReplaceWithSurrogate`) in `typescript/@relateby/graph/src/index.ts`
- [X] T037 [US3] Implement `mapGraph<V>` and `mapAllGraph<V>` in `typescript/@relateby/graph/src/index.ts`: curried; dispatch per `GraphClass` tag using `Match.tag` + `Match.exhaustive` when Effect available, `switch` fallback otherwise
- [X] T038 [US3] Implement `filterGraph<V>` in `typescript/@relateby/graph/src/index.ts`: curried; apply `Substitution` strategy when removing elements from inside walks/annotations
- [X] T039 [P] [US3] Implement `foldGraph<V, M>` in `typescript/@relateby/graph/src/index.ts`: curried; reduce all `[GraphClass, Pattern<V>]` pairs with explicit `empty` and `combine`
- [X] T040 [US3] Implement `mapWithContext<V>` in `typescript/@relateby/graph/src/index.ts`: curried; pass snapshot `GraphQuery<V>` from `view.viewQuery` to each element callback
- [X] T041 [US3] Implement `paraGraph<V, R>` in `typescript/@relateby/graph/src/index.ts`: curried; call `view.viewQuery`'s underlying `topoSort()` once for ordering; iterate bottom-up in TypeScript; return `ReadonlyMap<string, R>`
- [X] T042 [US3] Implement `paraGraphFixed<V, R>` in `typescript/@relateby/graph/src/index.ts`: curried; iterate `paraGraph` until convergence predicate `conv(prev, next)` returns `true`
- [X] T043 [P] [US3] Implement `unfoldGraph<S, V>` in `typescript/@relateby/graph/src/index.ts`: curried; expand seeds → patterns via `expand`; construct `PatternGraph<V>` via `build`
- [X] T044 [US3] Write vitest tests in `typescript/@relateby/graph/tests/graph.test.ts` using plain TS stubs only (no WASM, no `init()`): verify `toGraphView` classifies correctly; `mapGraph` applies per-class mappers; `filterGraph` with `SpliceGap` removes and splices; `foldGraph` accumulates correctly; `paraGraph` returns bottom-up depths; `unfoldGraph` expands seeds
- [X] T045 [US3] Write vitest tests in `typescript/@relateby/graph/tests/graph.test.ts` for pipe composition: (a) SC-011 — compose three transforms with `pipe` using plain TS stubs; confirm no `@relateby/pattern` import appears in test file; (b) SC-010 — assert `pipe(view, f, g, h)` produces the same `GraphView` as `h(g(f(view)))` applied sequentially on identical input
- [X] T046 [US3] Run `npm test` in `typescript/@relateby/graph/`; fix any failures; confirm zero imports of `@relateby/pattern` in `src/index.ts`

**Checkpoint**: US3 acceptance scenarios 1–7 pass. SC-003 and SC-011 verified. `@relateby/graph` tests pass without WASM initialization.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Examples, documentation, Effect integration completeness, and CI validation.

- [X] T047 [P] Create `examples/relateby-graph/package.json` and `examples/relateby-graph/README.md` with prerequisites and run instructions
- [X] T048 [P] Create `examples/relateby-graph/node.mjs`: build graph from `NativePattern`/`NativeSubject`, run `bfs`, compute `degreeCentrality`, apply `mapGraph` + `filterGraph` pipeline via `@relateby/graph`; verify expected console output matches quickstart.md
- [X] T049 [P] Create `examples/relateby-graph/browser.html`: import `@relateby/pattern` without explicit `init()` (bundler auto-init via wasm-pack `--target bundler` glue); verify `NativePatternGraph` is usable after page load; document CDN/bundler usage (US4 scenario 2, SC-005)
- [X] T050 Create `docs/typescript-graph.md`: package installation (`@relateby/pattern`, `@relateby/gram`, `@relateby/graph`); `init()` usage; graph construction with `NativePatternGraph` + `NativeReconciliationPolicy`; querying with `NativeGraphQuery`; algorithms; pure TS transforms via `@relateby/graph`; WASM-free stub pattern (SC-011 example); Effect integration; weight callback performance note (one WASM crossing per traversed edge)
- [X] T051 [P] Update `docs/wasm-usage.md`: add "Graph API" section pointing to `docs/typescript-graph.md`; update package name references to `@relateby/pattern`, `@relateby/gram`, `@relateby/graph`
- [x] T052 ~~Update `quickstart.md` in `specs/033-typescript-wasm-graph/`~~ — completed during analysis remediation; all import strings and directory paths updated to `@relateby/*`
- [ ] T053 [P] Verify Effect integration end-to-end: install `effect` in `typescript/@relateby/pattern/`; confirm `shortestPath` returns `Option.Option<Pattern<Subject>[]>`; confirm `topologicalSort` returns `Option.Option<Pattern<Subject>[]>`; confirm `validate` returns `Either.Either<void, ValidationError>`
- [ ] T054 Run `node examples/relateby-graph/node.mjs` and verify output matches expected output in quickstart.md
- [ ] T055 [P] Port graph algorithm test cases from `../pattern-hs/libs/pattern/tests/` to `typescript/@relateby/pattern/tests/pattern.test.ts`: identify at least 3 reference test cases for BFS/DFS traversal, shortest path, and connected components; assert outputs match reference expected values (SC-002, Constitution §I)
- [ ] T056 [P] Port graph transform test cases from `../pattern-hs/libs/pattern/src/Pattern/Graph/Transform.hs` tests to `typescript/@relateby/graph/tests/graph.test.ts`: identify at least 3 reference test cases for `mapGraph`, `filterGraph`, and `paraGraph`; assert outputs match reference expected values (SC-003, Constitution §I)
- [ ] T057 [P] Write scale smoke test in `typescript/@relateby/pattern/tests/pattern.test.ts`: construct a `NativePatternGraph` from 10,000 node patterns and 50,000 relationship patterns; call `bfs`, `connectedComponents`, `degreeCentrality`, and `topologicalSort`; assert all complete without error (SC-004)
- [ ] T058 Write edge-case tests in `typescript/@relateby/pattern/tests/pattern.test.ts` covering all spec.md edge cases (SC-008): `fromPatterns([])` returns valid empty graph; `shortestPath` with no connecting path returns `null`/`None` without throwing; `topologicalSort` on a cyclic graph returns `null`/`None` without looping; `filterGraph` removing a node referenced in a walk applies `SpliceGap` correctly; `strict` policy records conflict without overwriting either pattern
- [ ] T059 Run `./scripts/ci-local.sh` and fix any remaining issues (format, clippy, WASM build, all tests)

**Checkpoint**: All success criteria SC-001 through SC-011 verified. Reference equivalence tests pass (Constitution §I). CI passes.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies — start immediately
- **Phase 2 (Foundational)**: Depends on Phase 1 — BLOCKS all WASM-dependent TypeScript work
- **Phase 3 (US4 — Packaging)**: Depends on Phase 2 — establishes `init()` and package exports
- **Phase 4 (US1 — Build & Query)**: Depends on Phase 3 — requires `NativePatternGraph` and `NativeGraphQuery` to be exported
- **Phase 5 (US2 — Analysis)**: Depends on Phase 4 — reuses graph construction and query infrastructure
- **Phase 6 (US3 — Transforms)**: Can start after Phase 1 for pure TS work (T033–T043); requires Phase 3 for end-to-end tests with `NativePatternGraph`
- **Phase 7 (Polish)**: Depends on Phases 3–6 complete

### User Story Dependencies

- **US4 (P1)**: Depends on Foundational (Phase 2) — establishes package scaffold
- **US1 (P1)**: Depends on US4 — requires working `init()` and `NativePatternGraph`
- **US2 (P2)**: Depends on US1 — reuses graph construction; algorithm functions already exposed
- **US3 (P3)**: Pure TS implementation (T033–T043) is independent of US1/US2; end-to-end tests with `NativePatternGraph` depend on US4

### Parallel Opportunities Within Phases

- **Phase 1**: T002, T003, T006 can run in parallel with T001
- **Phase 2**: T009, T011 can run in parallel with T007/T008/T010; T016 can run in parallel with T015
- **Phase 3**: T019, T022 can run in parallel with T017/T018/T020
- **Phase 4**: T027 writes to a separate `describe` block in the same test file as T026 — run after T026 completes; T023–T025 extend T017's `index.ts` and must run after T017
- **Phase 5**: T031 can run in parallel with T029–T030
- **Phase 6**: T035, T036, T039, T043 can run in parallel with T033/T034/T037/T038/T040–T042
- **Phase 7**: T047, T048, T049, T051, T053, T055, T056, T057 can run in parallel; T058 depends on Phase 4–6 implementations; T059 runs last

---

## Parallel Example: User Story 3

```bash
# These tasks have no dependencies on each other — launch together:
T035  Implement GraphClass discriminated union in typescript/@relateby/graph/src/index.ts
T036  Implement Substitution discriminated union in typescript/@relateby/graph/src/index.ts
T039  Implement foldGraph in typescript/@relateby/graph/src/index.ts
T043  Implement unfoldGraph in typescript/@relateby/graph/src/index.ts

# Then sequentially (each depends on T033/T034):
T037  Implement mapGraph + mapAllGraph
T038  Implement filterGraph
T040  Implement mapWithContext
T041  Implement paraGraph
T042  Implement paraGraphFixed

# Then tests (depend on all implementations above):
T044  Write WASM-free vitest tests
T045  Write pipe composition tests (SC-010 result equivalence + SC-011 WASM-free)
T046  Run npm test
```

## Parallel Example: Phase 7

```bash
# Launch these together (independent files/concerns):
T047  Create examples/relateby-graph/package.json + README.md
T048  Create examples/relateby-graph/node.mjs
T049  Create examples/relateby-graph/browser.html (bundler auto-init verification)
T051  Update docs/wasm-usage.md
T053  Verify Effect integration end-to-end
T055  Port algorithm reference tests from ../pattern-hs
T056  Port transform reference tests from ../pattern-hs
T057  Write scale smoke test (SC-004, 10k nodes / 50k rels)

# Then sequentially:
T050  Create docs/typescript-graph.md (references all of the above)
T058  Write edge-case tests (SC-008, depends on Phase 4–6 implementations)
T054  Run node examples/relateby-graph/node.mjs
T059  Run ./scripts/ci-local.sh (final gate)
```

---

## Implementation Strategy

### MVP Scope (US4 + US1 only)

1. Complete Phase 1: Setup (T001–T006)
2. Complete Phase 2: Foundational Rust WASM additions (T007–T016)
3. Complete Phase 3: US4 packaging and init (T017–T022)
4. Complete Phase 4: US1 graph construction and querying (T023–T028)
5. **STOP and VALIDATE**: `node examples/relateby-graph/node.mjs` runs; SC-001 verified
6. A developer can install `@relateby/pattern`, call `init()`, build a graph, and run BFS in under 10 lines

### Incremental Delivery

1. Phase 1 + 2 → Rust WASM foundation ready
2. Phase 3 (US4) → Packages installable and initializable
3. Phase 4 (US1) → Graph construction and traversal working → **MVP**
4. Phase 5 (US2) → Structural analysis working
5. Phase 6 (US3) → Pure TS transforms working (can develop in parallel with US1/US2)
6. Phase 7 → Examples, docs, Effect integration, reference equivalence tests, scale test, edge-case tests, CI green

### Pure TypeScript Fast Path (US3 only)

`@relateby/graph` (US3) can be fully implemented and tested without completing Phase 2 (Rust WASM). After Phase 1 setup:

1. Implement T033–T043 (all pure TS, no WASM dependency)
2. Write and run T044–T046 (WASM-free tests using plain TS stubs; SC-010 and SC-011)
3. US3 is independently verifiable and deliverable before WASM work completes

---

## Notes

- `[P]` tasks operate on different files or independent concerns — safe to run in parallel
- `[Story]` label maps each task to its user story for traceability
- `@relateby/graph` has zero runtime dependency on `@relateby/pattern` — dependency flows the other way
- All WASM concrete classes use `Native*` prefix; TypeScript interfaces have no prefix
- `Pattern<V>` interface field is `elements` (matching the Rust WASM binding `WasmPattern::elements()`) — not `children`
- T023–T025 extend T017's `index.ts` (add type augmentations); they do not replace T017's work
- T027 writes to a separate `describe` block in the same test file as T026 — not a parallel task
- Bundler auto-init (T020, T049) relies on wasm-pack `--target bundler` glue, not runtime environment detection
- `toGraphView` reads pre-classified arrays from `PatternGraph<V>` — it does not re-classify elements
- `NativePatternGraph.fromPatterns()` return type MUST be declared as `PatternGraph<Subject>` (not `NativePatternGraph`) to propagate the generic type assertion (FR-031)
- Custom weight callbacks incur one WASM crossing per traversed edge — document prominently in T050
- `paraGraph` calls `topoSort()` once (one WASM crossing when WASM-backed); all subsequent iteration is pure TypeScript
- T055/T056 require access to `../pattern-hs/libs/pattern/tests/` — verify the reference repo is available locally before starting Phase 7
- Run `./scripts/ci-local.sh` (T059) before any push to validate format, clippy, WASM build, and all tests
