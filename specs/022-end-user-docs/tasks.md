# Tasks: End-user documentation

**Input**: Design documents from `/specs/022-end-user-docs/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, quickstart.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and documentation structure

- [X] T001 Create `docs/` directory if not exists
- [X] T002 [P] Create placeholder files for `docs/introduction.md`, `docs/gram-notation.md`, and `docs/rust-usage.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core documentation structure and common assets

- [X] T003 Define common terminology and glossary in `docs/introduction.md`
- [X] T004 Setup link structure in root `README.md` to point to the new documentation files

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Understand Pattern Concepts (Priority: P1) 🎯 MVP

**Goal**: Explain the core "decorated sequence" concept and "explicit vs implicit" distinction.

**Independent Test**: Read `docs/introduction.md` and verify it covers the "decorated sequence" and "explicit patterns vs implicit traversals" concepts as defined in the spec.

### Implementation for User Story 1

- [X] T005 [US1] Author "What is a Pattern?" section in `docs/introduction.md` based on `gram-hs` inspiration
- [X] T006 [US1] Author "Why Patterns Matter?" section in `docs/introduction.md` (explicit vs implicit traversals)
- [X] T007 [US1] Add conceptual diagrams or examples (e.g., "Route 66") to `docs/introduction.md`

**Checkpoint**: User Story 1 complete - core conceptual documentation is ready.

---

## Phase 4: User Story 2 - Learn Gram Notation (Priority: P1)

**Goal**: Provide a reference for Gram syntax and its mapping to Pattern structures.

**Independent Test**: Verify `docs/gram-notation.md` contains specific examples for nodes, relationships, and annotations, with their equivalent Pattern representations.

### Implementation for User Story 2

- [X] T008 [US2] Author "Nodes" section in `docs/gram-notation.md` showing `(n)` mapping to atomic patterns
- [X] T009 [US2] Author "Relationships" section in `docs/gram-notation.md` showing `(a)-[r]->(b)` mapping to patterns with 2 elements
- [X] T010 [US2] Author "Annotations" section in `docs/gram-notation.md` showing `@k(v)` mapping
- [X] T011 [US2] Author "Nesting & Paths" section in `docs/gram-notation.md` covering complex structures

**Checkpoint**: User Story 2 complete - syntax reference is ready.

---

## Phase 5: User Story 3 - Use pattern-rs in Rust (Priority: P2)

**Goal**: Practical guide for using `pattern-rs` crates in Rust projects.

**Independent Test**: Copy code snippets from `docs/rust-usage.md` and verify they compile and run correctly in a sample Rust environment.

### Implementation for User Story 3

- [X] T012 [US3] Author dependency setup section in `docs/rust-usage.md` using `pattern-core` and `gram-codec`
- [X] T013 [P] [US3] Add programmatic construction examples (point, pattern) in `docs/rust-usage.md`
- [X] T014 [P] [US3] Add parsing and serialization examples in `docs/rust-usage.md`
- [X] T015 [US3] Add basic query examples (any_value, all_values) in `docs/rust-usage.md`

**Checkpoint**: All user stories should now be independently functional.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

### Documentation & Examples

- [X] T016 [P] Review all documentation files for consistent terminology
- [X] T017 [P] Cross-link between the new documentation files for better navigation
- [X] T018 Run `quickstart.md` validation to ensure code snippets are in sync

### Code Quality Checks (REQUIRED)

- [X] T019 Run `cargo fmt --all` to ensure no incidental formatting issues in code snippets
- [X] T020 Run `cargo clippy --workspace -- -D warnings` to ensure implementation remains clean
- [X] T021 Run full CI checks with `scripts/ci-local.sh`
- [X] T022 Verify all tests pass (`cargo test --workspace`)

### Final Verification

- [X] T023 Update `TODO.md` to mark documentation feature as complete
- [X] T024 Ensure all acceptance criteria from `spec.md` are met

---

## Phase 7: Parser & API Redesign (Priority: P1)

**Goal**: Update `gram-codec` API to better handle top-level records and headers.

### Implementation for Redesign

- [X] T025 Redesign `parse_gram` to return `Vec<Pattern<Subject>>` directly (no implicit wrapping)
- [X] T026 Implement `parse_gram_with_header` to separate optional leading records as document metadata
- [X] T027 Implement `to_gram` for space-delimited pattern serialization
- [X] T028 Implement `to_gram_with_header` for serialization with a leading header record
- [X] T029 Add internal AST helper to maintain the "single file-level pattern" contract for `AstPattern`
- [X] T030 Update `docs/rust-usage.md` to reflect the new `parse_gram` and header-aware functions

**Checkpoint**: Parser API refined - documentation and implementation are in sync.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies
- **Foundational (Phase 2)**: Depends on Setup completion
- **User Stories (Phase 3-5)**: All depend on Foundational phase completion
- **Polish (Final Phase)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Core foundation, should be done first
- **User Story 2 (P1)**: Independent of US1 but equally important
- **User Story 3 (P2)**: Depends on conceptual understanding from US1/US2

### Parallel Opportunities

- T002 (Placeholder creation)
- T013, T014 (Rust usage examples)
- T016, T017 (Documentation review and cross-linking)

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Verify introduction documentation meets spec

### Incremental Delivery

1. Complete Setup + Foundational
2. Add User Story 1 (Intro) -> Validate
3. Add User Story 2 (Notation) -> Validate
4. Add User Story 3 (Rust Usage) -> Validate
