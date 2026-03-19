# Tasks: CST-Preserving Gram Parser

**Input**: Design documents from `/specs/042-gram-cst-parser/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/api.md

**Baseline**: `cargo test -p relateby-gram corpus -- --nocapture` reports 90.1% (128/142 tests),
14 failures across 3 root causes. Phase 0 must reach 100% before Phase 1 begins.

**Organization**: Tasks are grouped by phase. Phase 0 aligns the nom parser with v0.3.4 grammar.
Phases 1–7 implement the new CST parser.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (US1–US4)

---

## Phase 0: Nom Parser Alignment (Prerequisite — MUST complete before Phase 1)

**Purpose**: Bring `test_corpus_conformance` to 100% pass rate on all runnable tests against
tree-sitter-gram v0.3.4. Three independent root causes; each has its own sub-group.

**⚠️ CRITICAL**: No CST parser work (Phase 1+) begins until this phase is complete and
`cargo test -p relateby-gram test_corpus_conformance` reports 100% of runnable tests passing.

### 0A — Corpus runner: `:error` test handling

- [X] T001 Update `crates/gram-codec/tests/corpus/parser.rs`: after reading the test name line, check whether the next line is `:error` (exact match after trim); if so, read and discard it before expecting the closing `==================` separator; record a boolean `is_error_test` on `CorpusTest`
- [X] T002 Add `is_error: bool` field to `CorpusTest` in `crates/gram-codec/tests/corpus/mod.rs` and update `CorpusTest::new` to accept it; update all call sites
- [X] T003 Update `CorpusTest::run` in `crates/gram-codec/tests/corpus/mod.rs`: if `self.is_error`, return a new `CorpusTestResult::SkippedExpectedError` variant rather than running the nom parser
- [X] T004 Update `CorpusTestResult` in `crates/gram-codec/tests/corpus/mod.rs`: add `SkippedExpectedError` variant; update `is_pass()` to return `true` for this variant (skipped-error is not a failure); update `failure_message()` to return `None`
- [X] T005 Update `CorpusTestReport::print_summary` in `crates/gram-codec/tests/corpus/runner.rs` to show a "Skipped (expected error)" count alongside passed/failed

**Checkpoint 0A**: Running the corpus test suite produces no "Skipping test" warnings and the skipped-error count matches the number of `:error` tests in the corpus files.

### 0B — Corpus validator: multi-pattern document count fix

- [X] T006 Replace `count_gram_patterns` in `crates/gram-codec/tests/corpus/validator.rs`: instead of counting `(gram_pattern` occurrences, count the number of direct named-pattern children of the single outer `gram_pattern` root — specifically, count lines that match `^\s{2}\(` (two-space indent, opening paren) to identify top-level children, filtering for known pattern node types (`node_pattern`, `relationship_pattern`, `subject_pattern`, `annotated_pattern`, `comment`); return 1 if only one such child exists, otherwise return the count
- [X] T007 Verify the fix against each of the 9 currently-failing multi-pattern tests by running `cargo test -p relateby-gram test_corpus_conformance -- --nocapture` and confirming `empty_nodes.txt`, `empty_relationships.txt`, `identifiers.txt`, `records.txt`, `comments.txt`, and `graph_global.txt` failures are resolved

**Checkpoint 0B**: 9 multi-pattern failures resolved; corpus pass rate ≥ 96.5%.

### 0C — Nom parser: `@@` identified annotation support

- [X] T008 Add `identified_annotation` parser function to `crates/gram-codec/src/parser/annotation.rs`: parse `@@` (two `@` chars using `tag("@@")`), then parse an optional identifier (`opt(identifier)`), then parse optional labels (`opt(many1(preceded(alt((char(':'), tag("::"))), identifier)))`); return a new `IdentifiedAnnotation { identity: Option<String>, labels: Vec<String> }` struct
- [X] T009 Update the `annotations` combinator in `crates/gram-codec/src/parser/annotation.rs`: replace `repeat1(annotation)` with a choice that accepts either (a) one `identified_annotation` optionally followed by zero or more `property_annotation` entries, or (b) one or more `property_annotation` entries — mirroring the v0.3.4 grammar's `choice(seq(identified_annotation, repeat(property_annotation)), repeat1(property_annotation))`; rename the old `annotation` function to `property_annotation` for clarity
- [X] T010 Update `annotated_pattern` in `crates/gram-codec/src/parser/mod.rs` to call the updated `annotations` combinator (rename reference from `annotation::annotation` to `annotation::annotations` or equivalent); annotation content continues to be dropped when constructing `Pattern<Subject>` (the pre-existing TODO is not resolved in this phase)
- [X] T011 Verify the 5 `extended_annotations.txt` failures are resolved: run `cargo test -p relateby-gram test_corpus_conformance -- --nocapture` and confirm `@@p (a)`, `@@r1 (a)-[r]->(b)`, `@@:L (a)`, `@@::Label (a)`, and `@@p:L (a)` all pass

**Checkpoint 0C**: All 5 `@@` failures resolved; corpus pass rate reaches 100% of runnable tests.

### Phase 0 Final Validation

- [X] T012 Run `cargo test -p relateby-gram` (all tests, no feature flags) and confirm zero regressions — existing non-corpus tests must all continue to pass
- [X] T013 Run `cargo clippy -p relateby-gram -- -D warnings` and resolve any new warnings introduced in Phase 0 changes

**Phase 0 Complete**: `test_corpus_conformance` reports 100% of runnable tests passing (`:error` tests explicitly skipped), zero regressions in existing test suite.

---

## Phase 1: Setup (Shared Infrastructure for CST Parser)

**Purpose**: Wire the `cst` Cargo feature and create the module skeleton. No logic implemented yet.

- [X] T014 Add `cst` feature to `crates/gram-codec/Cargo.toml` with `tree-sitter = { version = "0.25", optional = true }` and `tree-sitter-gram = { path = "../../external/tree-sitter-gram", optional = true }` under `[features] cst = ["dep:tree-sitter", "dep:tree-sitter-gram"]` and `[dependencies]`
- [X] T015 Create `crates/gram-codec/src/cst/` with stub files: `mod.rs`, `syntax_node.rs`, `parser.rs`, `lowering.rs` (each file contains only a module-level comment and no code yet)
- [X] T016 Add `#[cfg(feature = "cst")] pub mod cst;` to `crates/gram-codec/src/lib.rs` and a `#[cfg(feature = "cst")] pub use cst::{parse_gram_cst, lower, CstParseResult};` re-export stub
- [X] T017 [P] Create `crates/gram-codec/tests/cst/` with `mod.rs` (empty), `parse_tests.rs`, `lowering_tests.rs`, `comment_tests.rs`, `error_recovery_tests.rs` (each file contains only a `#[cfg(test)]` block with one `todo!()` placeholder test); wire into the integration test entry point
- [X] T018 Verify the feature-gated skeleton compiles cleanly: `cargo check -p relateby-gram --features cst`

**Checkpoint**: `cargo check --features cst` passes — scaffolding is in place.

---

## Phase 2: Foundational (Blocking Prerequisites for CST Parser)

**Purpose**: Define all data types that every subsequent phase depends on.

**⚠️ CRITICAL**: No user story implementation can begin until this phase is complete.

- [X] T019 Implement `SourceSpan { start: usize, end: usize }` and `ArrowKind { Right, Left, Bidirectional, Undirected }` with derives `Clone, Debug, PartialEq, Eq` in `crates/gram-codec/src/cst/syntax_node.rs`
- [X] T020 Implement `SyntaxKind` enum (`Document`, `Node`, `Relationship(ArrowKind)`, `Subject`, `Annotated`, `Comment`) with derives `Clone, Debug, PartialEq, Eq` in `crates/gram-codec/src/cst/syntax_node.rs`
- [X] T021 Implement `Annotation` enum (`Property { key: String, value: Value }`, `Identified { identity: Option<Symbol>, labels: Vec<String> }`) with derives `Clone, Debug` in `crates/gram-codec/src/cst/syntax_node.rs`; import `Value` from `crate::Value` and `Symbol` from `pattern_core::Symbol`
- [X] T022 Implement `SyntaxNode { kind: SyntaxKind, subject: Option<Subject>, span: SourceSpan, annotations: Vec<Annotation>, text: Option<String> }` and `CstParseResult { tree: Pattern<SyntaxNode>, errors: Vec<SourceSpan> }` with `impl CstParseResult { pub fn is_valid(&self) -> bool }` in `crates/gram-codec/src/cst/syntax_node.rs`
- [X] T023 Export all types from `crates/gram-codec/src/cst/mod.rs`: `pub use syntax_node::{SourceSpan, ArrowKind, SyntaxKind, Annotation, SyntaxNode, CstParseResult};`
- [X] T024 Verify all type definitions compile and exports resolve: `cargo check -p relateby-gram --features cst`

**Checkpoint**: All types compile and are re-exported — user story phases can now begin.

---

## Phase 3: User Story 1 — Parse Gram into Syntax-Preserving Tree (Priority: P1) 🎯 MVP

**Goal**: `parse_gram_cst(input)` returns a `CstParseResult` with a `Pattern<SyntaxNode>` tree
retaining source spans, arrow kinds, annotation content, comment nodes, and error spans.

**Independent Test**: `cargo test -p relateby-gram --features cst cst::parse` — parse fixture
files, verify spans reproduce source text, arrow kinds match, annotations present, comments
interleaved in source order.

### Implementation

- [X] T025 [US1] Implement `parse_gram_cst(input: &str) -> CstParseResult` skeleton in `crates/gram-codec/src/cst/parser.rs`: create tree-sitter `Parser`, call `parser.set_language(&tree_sitter_gram::LANGUAGE.into())`, call `parser.parse(input, None)`, assert root node kind is `"gram_pattern"`, return placeholder `CstParseResult`
- [X] T026 [US1] Implement document-root traversal in `crates/gram-codec/src/cst/parser.rs`: iterate named children of the `gram_pattern` root; dispatch each to the appropriate conversion function (stubs initially); collect `ERROR` node byte ranges into `CstParseResult.errors` via `node.is_error()` and `node.has_error()`
- [X] T027 [US1] Implement `node_pattern` → `Pattern<SyntaxNode>` in `crates/gram-codec/src/cst/parser.rs`: extract optional subject fields (identifier, labels, record) into `Subject`; set `SyntaxKind::Node`; populate `span`; return `Pattern::point(syntax_node)`
- [X] T028 [US1] Implement `relationship_pattern` → `Pattern<SyntaxNode>` in `crates/gram-codec/src/cst/parser.rs`: extract `left`, `kind`, `right` fields; map `kind.kind()` string to `ArrowKind`; extract edge subject from the arrow node's optional fields; return `Pattern { value: SyntaxNode { kind: Relationship(arrow), .. }, elements: [left_pat, right_pat] }`
- [X] T029 [US1] Implement `subject_pattern` → `Pattern<SyntaxNode>` in `crates/gram-codec/src/cst/parser.rs`: extract subject fields and `subject_pattern_elements` children; return `Pattern { value: SyntaxNode { kind: Subject, .. }, elements }`
- [X] T030 [US1] Implement annotation extraction helpers in `crates/gram-codec/src/cst/parser.rs`: `extract_property_annotation` (matches `"property_annotation"`, reads `key` and `value` fields); `extract_identified_annotation` (matches `"identified_annotation"`, reads optional `identifier` and `labels` fields); `extract_annotations(annotations_node) -> Vec<Annotation>` dispatches both
- [X] T031 [US1] Implement `annotated_pattern` → `Pattern<SyntaxNode>` in `crates/gram-codec/src/cst/parser.rs`: call `extract_annotations`, recurse into `elements` field, return `Pattern { value: SyntaxNode { kind: Annotated, annotations, .. }, elements: [inner] }`
- [X] T032 [US1] Implement `comment` → `Pattern<SyntaxNode>` in `crates/gram-codec/src/cst/parser.rs`: read `node.utf8_text(input.as_bytes())` as `text`; return `Pattern::point(SyntaxNode { kind: Comment, text: Some(comment_text), span, .. })`

### Tests

- [X] T033 [P] [US1] Write parse fixture tests in `crates/gram-codec/tests/cst/parse_tests.rs`: for inline gram strings covering each construct, assert `is_valid()` returns true, and `&input[span.start..span.end]` reproduces the source text of each top-level node; assert all four arrow kinds map to the correct `ArrowKind` variant
- [X] T034 [P] [US1] Write comment tests in `crates/gram-codec/tests/cst/comment_tests.rs`: assert comment nodes appear in source order interleaved with patterns, `text` field contains the full `// …` text, and `span` is byte-accurate
- [X] T035 [P] [US1] Write error recovery tests in `crates/gram-codec/tests/cst/error_recovery_tests.rs`: assert malformed input returns `is_valid() == false` with a non-empty `errors` list; assert the partial `tree` is still present

**Checkpoint**: All US1 tests pass — `parse_gram_cst` is fully functional.

---

## Phase 4: User Story 2 — Lower Syntax Tree to Semantic Pattern (Priority: P2)

**Goal**: `lower(tree)` maps `Pattern<SyntaxNode>` → `Vec<Pattern<Subject>>` with output identical
to `parse_gram(input)` for all valid input the current nom parser accepts.

**Independent Test**: `cargo test -p relateby-gram --features cst cst::lowering` — compare
`lower(parse_gram_cst(s).tree)` against `parse_gram(s)` on all existing corpus fixtures
(excluding `@@` input).

### Implementation

- [X] T036 [US2] Implement `lower(tree: Pattern<SyntaxNode>) -> Vec<Pattern<Subject>>` skeleton in `crates/gram-codec/src/cst/lowering.rs`: assert root `kind == SyntaxKind::Document`, iterate `tree.elements`, dispatch each to `lower_node`, collect non-`None` results
- [X] T037 [US2] Implement `lower_node(node: Pattern<SyntaxNode>) -> Option<Pattern<Subject>>` in `crates/gram-codec/src/cst/lowering.rs`: handle `Node` → `Pattern::point(subject)`, `Subject` → `Pattern { value, elements }`, `Comment` → `None`
- [X] T038 [US2] Implement `Relationship` lowering in `crates/gram-codec/src/cst/lowering.rs`: `Right | Bidirectional | Undirected` → preserve element order; `Left` → reverse to `[lower(right), lower(left)]`
- [X] T039 [US2] Implement `Annotated` lowering in `crates/gram-codec/src/cst/lowering.rs`: build annotation `Subject` from `Property` annotations only; drop `Identified` entries; return `Pattern { value: annotation_subject, elements: [lower(inner)] }`
- [X] T040 [US2] Export `lower` from `crates/gram-codec/src/cst/mod.rs` and `crates/gram-codec/src/lib.rs` under `#[cfg(feature = "cst")]`

### Tests

- [X] T041 [US2] Write equivalence tests in `crates/gram-codec/tests/cst/lowering_tests.rs`: for every gram fixture (excluding `@@` input), assert `lower(parse_gram_cst(s).tree) == parse_gram(s).unwrap()` using `Pattern`'s `PartialEq`; cover node, relationship (all four arrow kinds), subject pattern, annotated pattern, header record

**Checkpoint**: All US2 tests pass — `lower` is complete and equivalence verified.

---

## Phase 5: User Story 3 — Diagnostics Access (Priority: P3)

**Goal**: Confirm span data is precise enough for real tooling — byte-accurate spans enable correct
source-location reporting.

**Independent Test**: `cargo test -p relateby-gram --features cst cst::parse::diagnostic` — span
byte-accuracy sweep and a duplicate-identity lint simulation.

### Tests

- [X] T042 [US3] Extend `crates/gram-codec/tests/cst/parse_tests.rs`: for each named node in a parsed tree, assert `input[span.start..span.end]` is non-empty and matches the source fragment; cover identifiers, arrow tokens, annotation keys, and comment text
- [X] T043 [US3] Write a diagnostic demonstration test in `crates/gram-codec/tests/cst/parse_tests.rs`: given `"(alice) (alice)"`, parse with `parse_gram_cst`, collect all `SyntaxKind::Node` nodes whose `subject.identity == "alice"`, assert exactly two found with non-overlapping spans

**Checkpoint**: Span data confirmed usable for diagnostic tooling.

---

## Phase 6: User Story 4 — Structural Alignment with Pattern Abstraction (Priority: P4)

**Goal**: Confirm `Pattern<SyntaxNode>` is a valid `Pattern<T>` — standard traversal operations
work, and `lower` is expressible as a `Pattern::map`-style transformation.

**Independent Test**: `cargo test -p relateby-gram --features cst cst::lowering::alignment`

### Tests

- [X] T044 [US4] Write traversal tests in `crates/gram-codec/tests/cst/lowering_tests.rs`: call `pattern.fold(0, |acc, _| acc + 1)` and assert the count; call `pattern.map(|n| n.kind.clone())` and assert the returned `Pattern<SyntaxKind>` structure is correct
- [X] T045 [US4] Write a map-style lowering test in `crates/gram-codec/tests/cst/lowering_tests.rs`: implement a local `lower_value(node: SyntaxNode) -> Subject`, call `tree.map(lower_value)`, assert the result type is `Pattern<Subject>` and its structure matches `lower(tree)` — demonstrating lowering is a value-level map

**Checkpoint**: All four user stories fully verified.

---

## Phase 7: Polish & Cross-Cutting Concerns

- [X] T046 [P] Write `examples/rust/cst_parse.rs`: parse `"// greeting\n(alice)->(bob)"`, print document tree showing kinds and spans, lower to `Vec<Pattern<Subject>>`; add `[[example]] name = "cst_parse" required-features = ["cst"]` to `crates/gram-codec/Cargo.toml`
- [X] T047 [P] Run `cargo clippy -p relateby-gram --features cst -- -D warnings` and resolve all warnings in `src/cst/`
- [X] T048 [P] Run `cargo fmt --all -- --check` and fix any formatting in `src/cst/` and `tests/cst/`
- [X] T049 Run `cargo test -p relateby-gram` (without `--features cst`) and confirm all tests pass including `test_corpus_conformance` at 100% — no regressions
- [X] T050 Run `cargo test -p relateby-gram --features cst` and confirm all CST tests pass
- [X] T051 Run `./scripts/ci-local.sh` to validate the full CI pipeline

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0**: No dependencies — start immediately; BLOCKS all subsequent phases
- **Phase 1 (Setup)**: Requires Phase 0 complete
- **Phase 2 (Foundational)**: Requires Phase 1 complete — blocks all user story phases
- **US1 (Phase 3)**: Requires Phase 2 complete
- **US2 (Phase 4)**: Requires Phase 2 complete; tests use `parse_gram_cst` output
- **US3 (Phase 5)**: Requires US1 complete (extends US1 tests)
- **US4 (Phase 6)**: Requires US1 and US2 complete
- **Polish (Phase 7)**: Requires all user stories complete

### Phase 0 Internal Dependencies

- T001–T005 (0A) can proceed independently of T006–T007 (0B) and T008–T011 (0C) — all three sub-groups address different files
- T012–T013 (final validation) must follow completion of all three sub-groups

### Parallel Opportunities

- Phase 0 sub-groups 0A, 0B, 0C can be worked in parallel (different files throughout)
- T017 (test scaffolding) can run in parallel with T015–T016 once T014 is done
- T027, T028, T029 (US1 node/relationship/subject conversion) can be done in any order
- T033, T034, T035 (US1 tests) can be written in parallel — different files
- T046, T047, T048 (polish) can run in parallel

---

## Parallel Example: Phase 0

```bash
# All three sub-groups can proceed concurrently:
Sub-group 0A: "T001–T005 — corpus runner :error handling in corpus/parser.rs and corpus/mod.rs"
Sub-group 0B: "T006–T007 — validator multi-pattern count fix in corpus/validator.rs"
Sub-group 0C: "T008–T011 — @@ annotation parser in src/parser/annotation.rs and parser/mod.rs"
# Then T012–T013: final validation
```

---

## Implementation Strategy

### MVP (Phase 0 + User Story 1 Only)

1. Complete Phase 0: Nom alignment — corpus at 100%
2. Complete Phase 1: Setup (T014–T018)
3. Complete Phase 2: Foundational types (T019–T024)
4. Complete Phase 3: US1 parser (T025–T035)
5. **STOP and VALIDATE**: `cargo test -p relateby-gram --features cst` — all CST parse tests pass

### Incremental Delivery

1. Phase 0 → corpus 100% aligned
2. Phase 1–2 → CST scaffolding and types compile
3. US1 (Phase 3) → `parse_gram_cst` working, syntax fully preserved
4. US2 (Phase 4) → `lower` working, equivalence proven
5. US3 (Phase 5) → diagnostic span accuracy confirmed
6. US4 (Phase 6) → architectural hypothesis confirmed
7. Phase 7 → CI green, example ships

---

## Notes

- Phase 0 modifies **existing** files only (`annotation.rs`, `parser/mod.rs`, `corpus/parser.rs`, `corpus/mod.rs`, `corpus/validator.rs`, `corpus/runner.rs`) — no new files
- The `cst` feature must be off by default — non-CST builds must compile without tree-sitter
- `transform.rs` is dormant; do not modify it
- `@@` identified annotation content is dropped in `Pattern<Subject>` (pre-existing TODO, not resolved here)
- `cargo test -p relateby-gram` (without `--features cst`) must pass after every phase
