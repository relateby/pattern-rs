# Tasks: StandardGraph

**Input**: Design documents from `/specs/035-standard-graph/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create module scaffolding and wire up re-exports

- [X] T001 Create `crates/pattern-core/src/graph/standard.rs` with `pub struct StandardGraph` wrapping `inner: PatternGraph<(), Subject>` (private field) and `pub fn new() -> Self` returning `StandardGraph { inner: PatternGraph::empty() }`
- [X] T002 [P] Register `mod standard;` in `crates/pattern-core/src/graph/mod.rs` and add `pub use standard::StandardGraph;` to the existing re-exports
- [X] T003 [P] Add `StandardGraph` to the top-level re-exports in `crates/pattern-core/src/lib.rs` alongside existing `graph::` re-exports

---

## Phase 2: US2 - Fluent Value Construction (Priority: P2)

**Goal**: Provide a fluent builder for constructing Subject values, eliminating boilerplate across all test code and application code.

**Independent Test**: Build a Subject using chained method calls and verify it has the expected identity, labels, and properties.

**Note**: Implemented before US1 because SubjectBuilder is a shared utility used by all subsequent stories. It modifies `subject.rs` only — no graph module dependency.

- [X] T004 [US2] Implement `SubjectBuilder` struct (fields: `identity: Symbol`, `labels: HashSet<String>`, `properties: HashMap<String, Value>`) and `Subject::build(identity: impl Into<String>) -> SubjectBuilder` associated function in `crates/pattern-core/src/subject.rs`
- [X] T005 [US2] Implement chaining methods on `SubjectBuilder` in `crates/pattern-core/src/subject.rs`: `pub fn label(mut self, label: impl Into<String>) -> Self`, `pub fn property(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self`, `pub fn done(self) -> Subject`, and `impl From<SubjectBuilder> for Subject`

**Checkpoint**: SubjectBuilder is usable standalone. Test: `Subject::build("alice").label("Person").property("name", "Alice").done()` produces correct Subject.

---

## Phase 3: US1 - Build a Graph Element by Element (Priority: P1) MVP

**Goal**: Users can create a StandardGraph, add nodes and relationships (and walks/annotations), and verify the graph contents via element access and counts.

**Independent Test**: Create graph, add nodes and relationships, verify element counts and retrieval by identity.

- [X] T006 [US1] Implement `pub fn add_node(&mut self, subject: Subject) -> &mut Self` in `crates/pattern-core/src/graph/standard.rs` — construct `Pattern::point(subject)`, extract identity via `GraphValue::identify()`, insert into `self.inner.pg_nodes` using last-write-wins (direct HashMap insert)
- [X] T007 [US1] Implement `pub fn add_relationship(&mut self, subject: Subject, source: &Symbol, target: &Symbol) -> &mut Self` in `crates/pattern-core/src/graph/standard.rs` — look up source/target in `pg_nodes` or create minimal placeholder nodes (`Pattern::point(Subject { identity: id.clone(), labels: HashSet::new(), properties: HashMap::new() })`), construct 2-element pattern `Pattern::pattern(subject, vec![source_pattern, target_pattern])`, insert into `pg_relationships`
- [X] T008 [US1] Implement `pub fn add_walk(&mut self, subject: Subject, relationships: &[Symbol]) -> &mut Self` in `crates/pattern-core/src/graph/standard.rs` — look up each relationship id in `pg_relationships` (or create minimal placeholder relationship patterns), construct N-element walk pattern, insert into `pg_walks`
- [X] T009 [US1] Implement `pub fn add_annotation(&mut self, subject: Subject, element: &Symbol) -> &mut Self` in `crates/pattern-core/src/graph/standard.rs` — look up element across all buckets (nodes, relationships, walks, annotations) or create minimal placeholder, construct 1-element annotation pattern, insert into `pg_annotations`
- [X] T010 [US1] Implement element access and count/health methods in `crates/pattern-core/src/graph/standard.rs`: `node(&Symbol) -> Option<&Pattern<Subject>>`, `relationship(&Symbol)`, `walk(&Symbol)`, `annotation(&Symbol)` (delegate to inner HashMap `.get()`); `node_count()`, `relationship_count()`, `walk_count()`, `annotation_count()` (delegate to `.len()`); `is_empty()` (all counts zero); `has_conflicts() -> bool`, `conflicts() -> &HashMap<Symbol, Vec<Pattern<Subject>>>`, `other() -> &HashMap<Symbol, ((), Pattern<Subject>)>` (delegate to inner fields)

**Checkpoint**: US1 complete. Can build a graph element by element, retrieve elements by identity, check counts. Placeholder nodes auto-created for missing references.

---

## Phase 4: US3 - Build a Graph from Text Notation (Priority: P3)

**Goal**: Users can create a StandardGraph from gram notation strings or from arbitrary Pattern collections, with automatic classification.

**Independent Test**: Parse `(alice:Person)-[:KNOWS]->(bob:Person)` and verify correct node/relationship counts.

**Note**: `from_gram` lives in gram-codec (not pattern-core) due to circular dependency constraint. gram-codec depends on pattern-core, so pattern-core cannot depend on gram-codec. An extension trait in gram-codec provides `StandardGraph::from_gram()` syntax.

- [X] T011 [US3] Implement `pub fn add_pattern(&mut self, pattern: Pattern<Subject>) -> &mut Self` and `pub fn add_patterns(&mut self, patterns: impl IntoIterator<Item = Pattern<Subject>>) -> &mut Self` in `crates/pattern-core/src/graph/standard.rs` — use `classify_by_shape(&pattern)` to determine `GraphClass`, then insert into appropriate bucket (`GNode` → `pg_nodes`, `GRelationship` → `pg_relationships`, `GWalk` → `pg_walks`, `GAnnotation` → `pg_annotations`, `GOther` → `pg_other`); handle reconciliation conflicts by storing in `pg_conflicts`
- [X] T012 [US3] Implement `pub fn from_patterns(patterns: impl IntoIterator<Item = Pattern<Subject>>) -> Self` and `pub fn from_pattern_graph(graph: PatternGraph<(), Subject>) -> Self` constructors in `crates/pattern-core/src/graph/standard.rs` — `from_patterns` creates empty graph then calls `add_patterns`; `from_pattern_graph` wraps the graph directly as `StandardGraph { inner: graph }`
- [X] T013 [US3] Implement `FromGram` extension trait in `crates/gram-codec/src/lib.rs` (or new file `crates/gram-codec/src/standard_graph.rs`): define `pub trait FromGram: Sized { fn from_gram(input: &str) -> Result<Self, ParseError>; }`, implement for `StandardGraph` by calling `parse_gram(input)?` then `StandardGraph::from_patterns(patterns)`, and re-export `FromGram` from gram-codec's public API

**Checkpoint**: US3 complete. Graphs can be built from gram notation, individual patterns, pattern collections, or existing PatternGraph. Unclassifiable patterns go to "other" bucket.

---

## Phase 5: US4 - Query Graph Elements (Priority: P4)

**Goal**: Users can iterate over all elements of a type and perform graph-native queries (source, target, neighbors, degree).

**Independent Test**: Build a known graph, verify iteration visits all elements, source/target return correct nodes, neighbors/degree are correct for bidirectional view.

- [X] T014 [US4] Implement iterator methods in `crates/pattern-core/src/graph/standard.rs`: `pub fn nodes(&self) -> impl Iterator<Item = (&Symbol, &Pattern<Subject>)>` (delegates to `self.inner.pg_nodes.iter()`), and analogous `relationships()`, `walks()`, `annotations()`
- [X] T015 [US4] Implement graph-native query methods in `crates/pattern-core/src/graph/standard.rs`: `pub fn source(&self, rel_id: &Symbol) -> Option<&Pattern<Subject>>` (get relationship, return first element), `pub fn target(&self, rel_id: &Symbol) -> Option<&Pattern<Subject>>` (get relationship, return second element), `pub fn neighbors(&self, node_id: &Symbol) -> Vec<&Pattern<Subject>>` (scan all relationships, collect opposite endpoint for any relationship where source or target matches node_id), `pub fn degree(&self, node_id: &Symbol) -> usize` (count relationships where source or target matches node_id — undirected/both directions)

**Checkpoint**: US4 complete. Full read access to graph elements via iteration and graph-native queries.

---

## Phase 6: US5 - Convert to Abstract Graph Types (Priority: P5)

**Goal**: Users can convert StandardGraph to PatternGraph, GraphQuery, and GraphView for interoperability with advanced algorithms.

**Independent Test**: Build graph, convert to each abstract type, verify all elements preserved.

- [X] T016 [US5] Implement `pub fn as_pattern_graph(&self) -> &PatternGraph<(), Subject>` and `pub fn into_pattern_graph(self) -> PatternGraph<(), Subject>` in `crates/pattern-core/src/graph/standard.rs` — trivial delegates to `&self.inner` and `self.inner`
- [X] T017 [US5] Implement `pub fn as_query(&self) -> GraphQuery<Subject>` and `pub fn as_snapshot(&self) -> GraphView<(), Subject>` in `crates/pattern-core/src/graph/standard.rs` — `as_query` wraps `self.inner` in `Rc` (or `Arc` with thread-safe feature) and calls `graph_query::from_pattern_graph()`; `as_snapshot` calls `graph_view::from_pattern_graph(&canonical_classifier(), &self.inner)`

**Checkpoint**: US5 complete. StandardGraph is fully interoperable with the abstract graph layer.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Integration tests, code quality, WASM validation

- [X] T018 Write integration tests in `crates/pattern-core/tests/standard_graph_tests.rs` covering all acceptance scenarios: SubjectBuilder fluent construction, element-by-element construction (nodes, relationships, walks, annotations), placeholder node auto-creation, duplicate identity last-write-wins, gram notation ingestion (in gram-codec test), element access by identity, iterator completeness, source/target/neighbors/degree queries (bidirectional), escape hatches (as_pattern_graph, as_query, as_snapshot), and edge cases (empty graph, missing identity returns None, unclassifiable patterns in "other", conflict accumulation)
- [X] T018a [US1] Verify SC-003 by identifying existing `PatternGraph` test scenarios in `crates/pattern-core/tests/` and replicating each using `StandardGraph` with equivalent or fewer lines of code
- [X] T018b Write a scale validation test in `crates/pattern-core/tests/standard_graph_tests.rs` that constructs a graph with 1,000 nodes and 5,000 relationships, then verifies all query methods (`node`, `nodes`, `source`, `target`, `neighbors`, `degree`, counts) return correct results
- [X] T019 Run code quality checks: `cargo fmt --all`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace` and fix all warnings/failures
- [X] T020 Verify WASM compilation with `cargo build --target wasm32-unknown-unknown -p pattern-core` and run full CI validation with `./scripts/ci-local.sh`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **US2 (Phase 2)**: Depends on Setup. Modifies `subject.rs` only (no graph module dependency)
- **US1 (Phase 3)**: Depends on Setup. Benefits from US2 (SubjectBuilder) for cleaner code but can work without it
- **US3 (Phase 4)**: Depends on US1 (needs StandardGraph struct and `from_patterns` method). T013 (gram-codec extension) additionally depends on T012
- **US4 (Phase 5)**: Depends on US1 (needs populated graph to query)
- **US5 (Phase 6)**: Depends on US1 (needs populated graph to convert)
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **US2 (SubjectBuilder)**: Independent — no dependencies on any other story. Can start after Setup.
- **US1 (Element Construction)**: Independent — can start after Setup. Uses SubjectBuilder if available.
- **US3 (Gram Ingestion)**: Requires US1's struct and element access methods (T010).
- **US4 (Queries)**: Requires US1's struct and construction methods.
- **US5 (Escape Hatches)**: Requires US1's struct.

### Parallel Opportunities

- T002 and T003 can run in parallel (different files: `graph/mod.rs` and `lib.rs`)
- US2 (Phase 2) and US1 (Phase 3) can run in parallel (different files: `subject.rs` and `graph/standard.rs`)
- US4 (Phase 5) and US5 (Phase 6) can run in parallel after US1 completes (both add methods to standard.rs, but independent logic)

---

## Parallel Example: US2 + US1

```bash
# These can run concurrently (different files):
Task: T004 [US2] "SubjectBuilder struct in subject.rs"
Task: T006 [US1] "add_node() in standard.rs"

# Both modify different files with no cross-dependencies
```

---

## Implementation Strategy

### MVP First (US1 Only)

1. Complete Phase 1: Setup (T001-T003)
2. Complete Phase 2: US2 - SubjectBuilder (T004-T005)
3. Complete Phase 3: US1 - Element Construction (T006-T010)
4. **STOP and VALIDATE**: Build graph, add nodes/relationships, verify counts and retrieval
5. This delivers a functional graph construction experience

### Incremental Delivery

1. Setup + US2 + US1 → Working graph with fluent construction (MVP)
2. Add US3 → Gram notation ingestion works
3. Add US4 → Full query capability
4. Add US5 → Interop with abstract layer
5. Polish → Tests, CI, WASM validation

### Circular Dependency Note

`from_gram` cannot be an inherent method on `StandardGraph` in pattern-core because gram-codec depends on pattern-core (circular). Instead, T013 implements a `FromGram` extension trait in gram-codec. Users write:
```rust
use pattern_core::graph::StandardGraph;
use gram_codec::FromGram;
let g = StandardGraph::from_gram("(a)-[:KNOWS]->(b)")?;
```

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- US2 is P2 but implemented before US1 (P1) because SubjectBuilder is a shared utility used by all other stories' code
- All atomic construction methods (`add_node`, `add_relationship`, etc.) return `&mut Self` for chaining and are infallible
- `from_gram` is the only fallible constructor (returns `Result`)
- Neighbors and degree use undirected view (both incoming and outgoing relationships)
