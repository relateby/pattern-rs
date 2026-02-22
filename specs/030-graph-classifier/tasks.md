# Tasks: Graph Classifier Port

**Input**: Design documents from `/specs/030-graph-classifier/`
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓, quickstart.md ✓

**Tests**: Included — the constitution and porting guide both mandate behavioral equivalence with the Haskell reference test suite.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no in-progress dependencies)
- **[Story]**: Which user story this task belongs to (US1–US4)

## Path Conventions

All paths are relative to repository root:
- Source: `crates/pattern-core/src/`
- Tests: `crates/pattern-core/tests/`
- Haskell reference: `../pattern-hs/libs/pattern/src/Pattern/`
- Haskell tests: `../pattern-hs/libs/pattern/tests/Spec/Pattern/`

---

## Phase 1: Setup

**Purpose**: Add required `Ord` bound to `Symbol`. Unblocks all user stories.

- [ ] T001 Add `PartialOrd, Ord` to the `#[derive(...)]` on `Symbol` in `crates/pattern-core/src/subject.rs` (change from `Clone, PartialEq, Eq, Hash` to `Clone, PartialEq, Eq, Hash, PartialOrd, Ord`); run `cargo test --workspace` to confirm no regressions

**Checkpoint**: `cargo build -p pattern-core` compiles. `Symbol` now satisfies `Ord + Hash + Clone` as required by `GraphValue::Id`.

---

## Phase 2: Foundational — Port Pattern.Reconcile

**Purpose**: Port reconciliation infrastructure from `../pattern-hs/libs/pattern/src/Pattern/Reconcile.hs`. Required by `PatternGraph` (US2, US3). Runs in parallel with Phase 3 (US1) after Phase 1 completes.

**⚠️ CRITICAL**: US2 and US3 cannot be implemented until this phase is complete.

- [ ] T002 Define traits `HasIdentity<V, I: Ord>` (fn `identity(v: &V) -> &I`), `Mergeable` (associated `type MergeStrategy`; fn `merge(strategy: &Self::MergeStrategy, a: Self, b: Self) -> Self`), and `Refinable` (fn `is_refinement_of(sup: &Self, sub: &Self) -> bool`) in new file `crates/pattern-core/src/reconcile.rs` (reference: Reconcile.hs lines 109–127)
- [ ] T003 Define `ReconciliationPolicy<S>` enum with variants `LastWriteWins`, `FirstWriteWins`, `Merge(ElementMergeStrategy, S)`, `Strict`; define `ElementMergeStrategy` enum with `ReplaceElements`, `AppendElements`, `UnionElements` in `crates/pattern-core/src/reconcile.rs` (reference: Reconcile.hs lines 134–162)
- [ ] T004 Define `SubjectMergeStrategy { label_merge: LabelMerge, property_merge: PropertyMerge }`, `LabelMerge` (`UnionLabels | IntersectLabels | ReplaceLabels`), and `PropertyMerge` (`ReplaceProperties | ShallowMerge | DeepMerge`) in `crates/pattern-core/src/reconcile.rs` (reference: Reconcile.hs lines 171–198)
- [ ] T005 Implement `HasIdentity<Subject, Symbol>` (delegates to `subject.identity`), `Mergeable for Subject` (MergeStrategy = SubjectMergeStrategy; merges labels by union, properties by right-bias), and `Refinable for Subject` (same identity + label/property subsets) in `crates/pattern-core/src/reconcile.rs` (reference: Reconcile.hs lines 167–197)
- [ ] T006 Define `ReconcileError` struct and implement `reconcile<V>(policy: &ReconciliationPolicy<V::MergeStrategy>, pattern: &Pattern<V>) -> Result<Pattern<V>, ReconcileError>` dispatching on policy variant in `crates/pattern-core/src/reconcile.rs` (reference: Reconcile.hs lines 239–248; `Strict` returns `Err` on differing duplicate content, all other variants return `Ok`)
- [ ] T007 Add `pub mod reconcile;` declaration to `crates/pattern-core/src/lib.rs`

**Checkpoint**: `cargo build -p pattern-core` compiles with reconcile module. `ReconciliationPolicy`, `HasIdentity`, `Mergeable`, `Refinable` available.

---

## Phase 3: User Story 1 — Classify a Pattern by Shape (Priority: P1) 🎯 MVP

**Goal**: Implement the full classification vocabulary (`GraphClass`), identity trait (`GraphValue`), injectable classifier (`GraphClassifier`), shape-based classification function (`classify_by_shape`), and standard classifier instance (`canonical_classifier`). After this phase, any `Pattern<V: GraphValue>` can be classified into one of five structural categories.

**Independent Test**: `cargo test -p pattern-core graph_classifier` — all 8 test cases ported from `GraphClassifierSpec.hs` pass.

### Implementation

- [ ] T008 [US1] Create `crates/pattern-core/src/graph/mod.rs` (empty re-export stub) and `crates/pattern-core/src/graph/graph_classifier.rs` (empty module file with `use` imports for `Pattern`, `Subject`, `Symbol`)
- [ ] T009 [US1] Define `GraphClass<Extra>` enum with variants `GNode`, `GRelationship`, `GAnnotation`, `GWalk`, `GOther(Extra)` and derive `Debug, Clone, PartialEq, Eq`; implement `map_other<F, B>(self, f: F) -> GraphClass<B>` method in `crates/pattern-core/src/graph/graph_classifier.rs` (reference: porting guide Part 1)
- [ ] T010 [US1] Define `GraphValue` trait with `type Id: Ord + Clone + Hash` and `fn identify(&self) -> &Self::Id`; implement `GraphValue for Subject` with `type Id = Symbol` returning `&self.identity` in `crates/pattern-core/src/graph/graph_classifier.rs` (reference: porting guide Part 2)
- [ ] T011 [US1] Define `GraphClassifier<Extra, V>` struct with field `classify: Box<dyn Fn(&Pattern<V>) -> GraphClass<Extra> + 'static>`; implement `GraphClassifier::new<F>(f: F) -> Self` constructor in `crates/pattern-core/src/graph/graph_classifier.rs` (reference: porting guide Part 3)
- [ ] T012 [US1] Implement private functions `is_node_like<V>(p: &Pattern<V>) -> bool` (returns `p.elements.is_empty()`), `is_relationship_like<V>(p: &Pattern<V>) -> bool` (returns `p.elements.len() == 2 && both elements is_node_like`), and `is_valid_walk<V: GraphValue>(rels: &[Pattern<V>]) -> bool` (frontier algorithm: seed with both endpoints of first rel; for each subsequent rel, advance frontier to opposite endpoint if one end matches; return false if frontier empties) in `crates/pattern-core/src/graph/graph_classifier.rs` (reference: porting guide Part 4)
- [ ] T013 [US1] Implement `pub fn classify_by_shape<V: GraphValue>(pattern: &Pattern<V>) -> GraphClass<()>` with 5-branch priority logic: `els.is_empty()` → `GNode`; `els.len() == 1` → `GAnnotation`; `els.len() == 2 && all is_node_like` → `GRelationship`; `all is_relationship_like && is_valid_walk` → `GWalk`; else → `GOther(())` in `crates/pattern-core/src/graph/graph_classifier.rs` (reference: porting guide Part 4)
- [ ] T014 [US1] Implement `pub fn canonical_classifier<V: GraphValue + 'static>() -> GraphClassifier<(), V>` that calls `GraphClassifier::new(|p| classify_by_shape(p))` in `crates/pattern-core/src/graph/graph_classifier.rs` (reference: porting guide Part 5)
- [ ] T015 [US1] Add `pub mod graph;` to `crates/pattern-core/src/lib.rs`; populate `crates/pattern-core/src/graph/mod.rs` with `pub use graph_classifier::{GraphClass, GraphClassifier, GraphValue, classify_by_shape, canonical_classifier};`

### Tests

- [ ] T016 [US1] Write `crates/pattern-core/tests/graph_classifier.rs`: define local helpers `fn node(s: &str) -> Pattern<Subject>` and `fn pat(s: &str, els: Vec<Pattern<Subject>>) -> Pattern<Subject>`; port all 8 test cases from `../pattern-hs/libs/pattern/tests/Spec/Pattern/Graph/GraphClassifierSpec.hs`: (1) atomic→GNode, (2) 1-element→GAnnotation, (3) 2-node-elements→GRelationship, (4) chain `r1=[A,B] r2=[B,C] r3=[D,C]`→GWalk (tests direction-agnostic chaining), (5) star `r1=[A,B] r2=[A,C] r3=[A,D]`→GOther, (6) rel-containing-non-node→GOther, (7) walk-containing-non-rel→GOther, (8) `canonical_classifier.classify(n) == GNode`

**Checkpoint**: `cargo test -p pattern-core graph_classifier` — 8 tests pass. Classification of any `Pattern<Subject>` works correctly.

---

## Phase 4: User Story 2 — Typed Graph Container (Priority: P2)

**Goal**: Implement `PatternGraph<Extra, V>` with all 6 typed collections, recursive sub-element decomposition on insert (walk→rels→nodes), and policy-aware construction. After this phase, a list of mixed patterns can be organized into named, keyed graph collections.

**Independent Test**: `cargo test -p pattern-core pattern_graph` — all tests ported from `PatternGraphSpec.hs` pass, including walk decomposition producing `pg_walks=1 / pg_relationships=2 / pg_nodes=3`.

### Implementation

- [ ] T017 [US2] Define `PatternGraph<Extra, V: GraphValue>` struct with fields `pg_nodes: HashMap<V::Id, Pattern<V>>`, `pg_relationships: HashMap<V::Id, Pattern<V>>`, `pg_walks: HashMap<V::Id, Pattern<V>>`, `pg_annotations: HashMap<V::Id, Pattern<V>>`, `pg_other: HashMap<V::Id, (Extra, Pattern<V>)>`, `pg_conflicts: HashMap<V::Id, Vec<Pattern<V>>>` and implement `empty() -> Self` in new file `crates/pattern-core/src/pattern_graph.rs`
- [ ] T018 [US2] Implement private `insert_node` and `insert_other` in `crates/pattern-core/src/pattern_graph.rs`: for each, call `identify(value(p))` to get the key; if key absent insert directly; if key present call `reconcile(policy, &twoOccurrences(existing, incoming))` — on `Ok(merged)` update the map, on `Err` push incoming to `pg_conflicts` (define `fn two_occurrences(existing: &Pattern<V>, p: Pattern<V>) -> Pattern<V>` as `Pattern { value: existing.value.clone(), elements: vec![p] }`)
- [ ] T019 [US2] Implement private `insert_relationship` in `crates/pattern-core/src/pattern_graph.rs`: first call `merge_with_policy(classifier, policy, elements[0].clone(), graph)` then `merge_with_policy(classifier, policy, elements[1].clone(), graph)` to recursively insert the 2 endpoint nodes, then insert the relationship itself into `pg_relationships` with collision handling
- [ ] T020 [US2] Implement private `insert_annotation` in `crates/pattern-core/src/pattern_graph.rs`: first call `merge_with_policy(classifier, policy, elements[0].clone(), graph)` to recursively insert the single inner element, then insert the annotation into `pg_annotations` with collision handling
- [ ] T021 [US2] Implement private `insert_walk` in `crates/pattern-core/src/pattern_graph.rs`: fold `merge_with_policy(classifier, policy, elem, graph)` over all elements (each relationship in turn, which recursively inserts their nodes), then insert the walk itself into `pg_walks` with collision handling
- [ ] T022 [US2] Implement `pub fn merge_with_policy<Extra, V>(classifier: &GraphClassifier<Extra, V>, policy: &ReconciliationPolicy<V::MergeStrategy>, pattern: Pattern<V>, graph: PatternGraph<Extra, V>) -> PatternGraph<Extra, V>` dispatching via `(classifier.classify)(&pattern)` to the 5 insert functions in `crates/pattern-core/src/pattern_graph.rs`
- [ ] T023 [US2] Implement `pub fn merge<Extra, V>(classifier: &GraphClassifier<Extra, V>, pattern: Pattern<V>, graph: PatternGraph<Extra, V>) -> PatternGraph<Extra, V>` (calls `merge_with_policy` with `ReconciliationPolicy::LastWriteWins`) and `pub fn from_patterns<Extra, V>` and `pub fn from_patterns_with_policy<Extra, V>` (each folds `merge`/`merge_with_policy` over `impl IntoIterator<Item = Pattern<V>>` starting from `PatternGraph::empty()`) in `crates/pattern-core/src/pattern_graph.rs`
- [ ] T024 [US2] Add `pub mod pattern_graph;` to `crates/pattern-core/src/lib.rs`

### Tests

- [ ] T025 [US2] Write `crates/pattern-core/tests/pattern_graph.rs`: define local helpers `fn node(s: &str) -> Pattern<Subject>` and `fn rel(r: &str, a: &str, b: &str) -> Pattern<Subject>`; port all tests from `../pattern-hs/libs/pattern/tests/Spec/Pattern/PatternGraphSpec.hs`: (1) empty graph — all 6 maps have size 0, (2) merge node — `pg_nodes` size 1, (3) merge relationship after nodes — `pg_nodes` size 2 / `pg_relationships` size 1, (4) `from_patterns` mixed list — correct counts, (5) unrecognized 3-node-element pattern — appears in `pg_other` not other maps, (6) duplicate identity `LastWriteWins` — single entry remains, (7) `from_patterns_with_policy FirstWriteWins` — first value kept, (8) walk decomposition — `pgWalks=1 / pgRelationships=2 / pgNodes=3` with correct identity keys present

**Checkpoint**: `cargo test -p pattern-core pattern_graph` passes. Full pipeline from pattern list → `PatternGraph` with correct collection membership.

---

## Phase 5: User Story 3 — Custom Domain Classifier (Priority: P3)

**Goal**: Verify that a `GraphClassifier<MyDomain, Subject>` with a user-defined `Extra` type routes patterns to `pg_other` with the typed tag preserved intact. No new implementation code is needed — the generic `Extra` parameter already supports this.

**Independent Test**: `cargo test -p pattern-core pattern_graph custom_classifier` — custom classifier test passes with `DomainHyperedge` tag retrievable from `pg_other`.

### Tests

- [ ] T026 [US3] Add custom classifier test to `crates/pattern-core/tests/pattern_graph.rs`: define `#[derive(Debug, PartialEq)] enum MyDomain { DomainHyperedge, DomainOther }`; build a `GraphClassifier<MyDomain, Subject>` whose closure routes patterns with `elements.len() > 2 && all elements node-shaped` to `GOther(DomainHyperedge)` and delegates everything else to `classify_by_shape` mapping `GOther(()) → GOther(DomainOther)`; call `from_patterns(myClassifier, [n1, n2, n3, hyperedge])`; assert `pg_nodes.len() == 3`, `pg_other.len() == 1`, and `pg_other["hyper"] == (DomainHyperedge, hyperedge_pattern)` (port from PatternGraphSpec.hs custom classifier test, lines 137–170)

**Checkpoint**: `cargo test -p pattern-core` — custom classifier tag is correctly preserved in `pg_other`.

---

## Phase 6: User Story 4 — Node-Predicate Bridge (Priority: P4)

**Goal**: Provide `from_test_node` as a compatibility bridge that lifts a boolean node-predicate into a two-category `GraphClassifier<(), V>`. Enables future `GraphLens` integration without changing its predicate-based API.

**Independent Test**: `cargo test -p pattern-core from_test_node` — predicate-based classifier returns `GNode` for matching patterns and `GOther(())` otherwise.

### Implementation

- [ ] T027 [US4] Implement `pub fn from_test_node<V, F>(test_node: F) -> GraphClassifier<(), V> where F: Fn(&Pattern<V>) -> bool + 'static` in `crates/pattern-core/src/graph/graph_classifier.rs`: wraps the predicate in `GraphClassifier::new(move |p| if test_node(p) { GNode } else { GOther(()) })` (reference: porting guide Part 6)

### Tests

- [ ] T028 [US4] Add `from_test_node` tests to `crates/pattern-core/tests/graph_classifier.rs`: (1) a predicate `|p| p.elements.is_empty()` wrapped via `from_test_node` classifies an atomic pattern as `GNode`; (2) the same classifier classifies a pattern with elements as `GOther(())`

**Checkpoint**: `cargo test -p pattern-core graph_classifier` still passes with `from_test_node` tests added.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Complete public API wiring, enforce code quality, verify multi-target compatibility.

- [ ] T029 Complete `pub use` re-exports in `crates/pattern-core/src/lib.rs` for all new public items: from `graph::graph_classifier` — `GraphClass, GraphClassifier, GraphValue, classify_by_shape, canonical_classifier, from_test_node`; from `reconcile` — `ReconciliationPolicy, ElementMergeStrategy, HasIdentity, Mergeable, Refinable`; from `pattern_graph` — `PatternGraph, merge as pg_merge, merge_with_policy as pg_merge_with_policy, from_patterns, from_patterns_with_policy`
- [ ] T030 [P] Run `cargo fmt --all` and commit any formatting fixes across all new source files
- [ ] T031 [P] Run `cargo clippy --workspace -- -D warnings` and fix all linting warnings in new files (`reconcile.rs`, `graph/graph_classifier.rs`, `pattern_graph.rs`)
- [ ] T032 Run `cargo build --workspace --target wasm32-unknown-unknown` to verify WASM compatibility (no blocking I/O, no filesystem access introduced by new code)
- [ ] T033 Run `./scripts/ci-local.sh` and confirm all checks pass (format, clippy, native build, WASM build, full test suite)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies — start immediately
- **Phase 2 (Reconcile)**: Depends on Phase 1 — BLOCKS Phase 4 and Phase 5
- **Phase 3 (US1)**: Depends on Phase 1 only — runs in parallel with Phase 2
- **Phase 4 (US2)**: Depends on Phase 2 AND Phase 3 — must wait for both
- **Phase 5 (US3)**: Depends on Phase 4
- **Phase 6 (US4)**: Depends on Phase 3 only — can start after Phase 3
- **Phase 7 (Polish)**: Depends on all previous phases

### User Story Dependencies

- **US1 (P1)**: Phase 1 only. No dependency on Reconcile.
- **US2 (P2)**: US1 (needs `GraphClassifier`) + Foundational Reconcile (needs `ReconciliationPolicy`).
- **US3 (P3)**: US2 (needs `PatternGraph` with `Extra` support).
- **US4 (P4)**: US1 only (`from_test_node` only needs `GraphClass` and `GraphClassifier`).

### Task-Level Dependencies (within phases)

Phase 2: T002 → T003 → T004 → T005 → T006 → T007 (sequential, all build on reconcile.rs)

Phase 3: T008 → T009 → T010 → T011 → T012 → T013 → T014 → T015 → T016 (sequential, all in graph_classifier.rs; T016 can start after T015)

Phase 4: T017 → T018 → T019 → T020 → T021 → T022 → T023 → T024 → T025 (sequential; T025 can start after T023)

---

## Parallel Execution Examples

### After Phase 1 — Two Independent Workstreams

```
Workstream A (Phase 2 — Reconcile):    Workstream B (Phase 3 — US1 Classifier):
T002 Define HasIdentity/Mergeable/      T008 Create graph/ module stubs
     Refinable traits                   T009 GraphClass<Extra> enum + map_other
T003 ReconciliationPolicy enum          T010 GraphValue trait + Subject impl
T004 SubjectMergeStrategy types         T011 GraphClassifier struct + new()
T005 Subject instances                  T012 is_node_like / is_relationship_like
T006 reconcile() function                    / is_valid_walk helpers
T007 lib.rs mod declaration             T013 classify_by_shape
                                        T014 canonical_classifier
                                        T015 lib.rs + graph/mod.rs exports
                                        T016 graph_classifier tests
                    ↓                               ↓
               Both complete → Phase 4 (PatternGraph) can begin
```

### Within Phase 7 (Polish)

```
T030 cargo fmt    ← [P] can run simultaneously
T031 cargo clippy ← [P] can run simultaneously
      ↓
T032 WASM build
      ↓
T033 Full CI
```

---

## Implementation Strategy

### MVP First (US1 Only — Phases 1 and 3)

1. Complete T001 (Phase 1: Symbol Ord)
2. Complete T008–T016 (Phase 3: classifier module)
3. **STOP and VALIDATE**: `cargo test -p pattern-core graph_classifier` — 8 tests pass
4. The classifier is independently usable as a pattern inspector

### Full Incremental Delivery

| Step | Phases | Deliverable | Validation |
|------|--------|-------------|------------|
| 1 | Phase 1 | Symbol Ord | `cargo test` clean |
| 2 | Phase 2 + 3 (parallel) | Reconcile + Classifier | `cargo test graph_classifier` 8 tests |
| 3 | Phase 4 | PatternGraph | `cargo test pattern_graph` all tests |
| 4 | Phase 5 | Custom classifier verified | custom classifier test passes |
| 5 | Phase 6 | `from_test_node` bridge | `from_test_node` tests pass |
| 6 | Phase 7 | Full API + CI clean | `./scripts/ci-local.sh` green |

---

## Notes

- Haskell reference for all types and behavior: `../pattern-hs/libs/pattern/src/Pattern/`
- Haskell tests to port: `../pattern-hs/libs/pattern/tests/Spec/Pattern/Graph/GraphClassifierSpec.hs` and `PatternGraphSpec.hs`
- `two_occurrences(existing, p)`: construct `Pattern { value: existing.value.clone(), elements: vec![p] }` — used in all insert collision handlers before calling `reconcile`
- `is_valid_walk` uses `identify()` for frontier comparison — not structural equality of `Pattern<V>`
- Walk/relationship/annotation insert functions are recursive via `merge_with_policy` — this is correct and expected behavior per Haskell reference
- Test helpers (`node`, `rel`, `pat`) should be defined locally in each test file, not in shared utilities
- [P] = different files, no in-progress dependencies
- Commit after each task or logical group of tasks
- Do not port `to_graph_view`, `materialize`, or `fromPatternGraph` — those are out of scope (belong to GraphTransform and GraphQuery features)
