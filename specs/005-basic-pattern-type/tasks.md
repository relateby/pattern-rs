# Tasks: Pattern Construction & Access

**Input**: Design documents from `/specs/005-basic-pattern-type/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are included for verification of each user story.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Library crate**: `crates/pattern-core/src/`, `tests/` at repository root
- Paths follow the project structure from plan.md

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and verification that Pattern type exists

- [x] T001 Verify Pattern<V> type exists from feature 004 in crates/pattern-core/src/pattern.rs
- [x] T002 [P] Create unit test file for construction/access/inspection in tests/unit/pattern_core.rs
- [x] T003 [P] Create equivalence test file for behavioral verification in tests/equivalence/pattern_construction_access.rs

---

## Phase 2: User Story 1 - Developer Constructs Patterns Using Functions (Priority: P1) 🎯 MVP

**Goal**: Enable developers to construct pattern instances using convenient construction functions without manually creating struct literals.

**Independent Test**: Verify that developers can use construction functions to create patterns with various values and element structures, and that the constructed patterns match the expected structure.

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T004 [P] [US1] Test Pattern::point() creates atomic pattern in tests/unit/pattern_core.rs
- [x] T005 [P] [US1] Test Pattern::pattern() creates pattern with elements in tests/unit/pattern_core.rs
- [x] T006 [P] [US1] Test Pattern::from_list() creates pattern from value list in tests/unit/pattern_core.rs
- [x] T007 [P] [US1] Test construction with string values in tests/unit/pattern_core.rs
- [x] T008 [P] [US1] Test construction with integer values in tests/unit/pattern_core.rs
- [x] T009 [P] [US1] Test construction with Subject values in tests/unit/pattern_core.rs
- [x] T010 [P] [US1] Test nested pattern construction in tests/unit/pattern_core.rs
- [x] T011 [P] [US1] Test from_list converts values to atomic patterns in tests/unit/pattern_core.rs

### Implementation for User Story 1

- [x] T012 [US1] Implement Pattern::point() associated function in crates/pattern-core/src/pattern.rs
- [x] T013 [US1] Implement Pattern::pattern() associated function in crates/pattern-core/src/pattern.rs
- [x] T014 [US1] Implement Pattern::from_list() associated function in crates/pattern-core/src/pattern.rs
- [x] T015 [US1] Add doc comments to construction functions in crates/pattern-core/src/pattern.rs
- [x] T016 [US1] Export construction functions in crates/pattern-core/src/lib.rs

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently. Developers can construct patterns using `point()`, `pattern()`, and `from_list()` functions.

---

## Phase 3: User Story 2 - Developer Accesses Pattern Components (Priority: P1)

**Goal**: Enable developers to access the value and elements of a pattern instance using accessor methods.

**Independent Test**: Verify that developers can access the value and elements of patterns, and that the accessors return the correct components that were used during construction.

### Tests for User Story 2

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T017 [P] [US2] Test value() returns correct value in tests/unit/pattern_core.rs
- [x] T018 [P] [US2] Test elements() returns correct elements slice in tests/unit/pattern_core.rs
- [x] T019 [P] [US2] Test value() preserves type information in tests/unit/pattern_core.rs
- [x] T020 [P] [US2] Test elements() allows iteration in tests/unit/pattern_core.rs
- [x] T021 [P] [US2] Test accessors work with nested patterns in tests/unit/pattern_core.rs
- [x] T022 [P] [US2] Test accessors work with different value types in tests/unit/pattern_core.rs

### Implementation for User Story 2

- [x] T023 [US2] Implement value() method returning &V in crates/pattern-core/src/pattern.rs
- [x] T024 [US2] Implement elements() method returning &[Pattern<V>] in crates/pattern-core/src/pattern.rs
- [x] T025 [US2] Add doc comments to accessor methods in crates/pattern-core/src/pattern.rs
- [x] T026 [US2] Verify accessors are exported in crates/pattern-core/src/lib.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently. Developers can construct patterns and access their components.

---

## Phase 4: User Story 3 - Developer Inspects Pattern Structure (Priority: P2)

**Goal**: Enable developers to inspect and analyze pattern structures using inspection utilities.

**Independent Test**: Verify that inspection utilities correctly analyze pattern structures and return accurate information about pattern characteristics.

### Tests for User Story 3

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T027 [P] [US3] Test length() returns direct element count in tests/unit/pattern_core.rs
- [x] T028 [P] [US3] Test size() returns total node count in tests/unit/pattern_core.rs
- [x] T029 [P] [US3] Test depth() returns maximum nesting depth in tests/unit/pattern_core.rs
- [x] T030 [P] [US3] Test is_atomic() identifies atomic patterns in tests/unit/pattern_core.rs
- [x] T031 [P] [US3] Test depth() returns 0 for atomic patterns in tests/unit/pattern_core.rs
- [x] T032 [P] [US3] Test depth() calculates nested depth correctly in tests/unit/pattern_core.rs
- [x] T033 [P] [US3] Test size() counts all nodes recursively in tests/unit/pattern_core.rs
- [x] T034 [P] [US3] Test inspection utilities handle 100+ nesting levels in tests/unit/pattern_core.rs
- [x] T035 [P] [US3] Test inspection utilities handle 10,000+ elements in tests/unit/pattern_core.rs

### Implementation for User Story 3

- [x] T036 [US3] Implement length() method returning usize in crates/pattern-core/src/pattern.rs
- [x] T037 [US3] Implement size() method returning usize in crates/pattern-core/src/pattern.rs
- [x] T038 [US3] Implement depth() method returning usize in crates/pattern-core/src/pattern.rs
- [x] T039 [US3] Implement is_atomic() convenience method returning bool in crates/pattern-core/src/pattern.rs
- [x] T040 [US3] Ensure depth() returns 0 for atomic patterns in crates/pattern-core/src/pattern.rs
- [x] T041 [US3] Ensure size() and depth() handle deep nesting safely (avoid stack overflow) in crates/pattern-core/src/pattern.rs
- [x] T042 [US3] Add doc comments to inspection utilities in crates/pattern-core/src/pattern.rs
- [x] T043 [US3] Verify inspection utilities are exported in crates/pattern-core/src/lib.rs

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all work independently. Developers can construct patterns, access components, and inspect structure.

---

## Phase 5: User Story 4 - Developer Verifies Behavioral Equivalence with gram-hs (Priority: P2)

**Goal**: Enable developers to verify that pattern construction, access, and inspection functions in gram-rs behave identically to the corresponding functions in the gram-hs reference implementation.

**Independent Test**: Create equivalent patterns using construction functions in both gram-rs and gram-hs, access their components, and verify they match. Test cases from gram-hs can be ported and executed in gram-rs with identical results.

### Tests for User Story 4

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T044 [P] [US4] Port test cases from gram-hs for point() construction in tests/equivalence/pattern_construction_access.rs
- [x] T045 [P] [US4] Port test cases from gram-hs for pattern() construction in tests/equivalence/pattern_construction_access.rs
- [x] T046 [P] [US4] Port test cases from gram-hs for fromList() construction in tests/equivalence/pattern_construction_access.rs
- [x] T047 [P] [US4] Port test cases from gram-hs for value accessor in tests/equivalence/pattern_construction_access.rs
- [x] T048 [P] [US4] Port test cases from gram-hs for elements accessor in tests/equivalence/pattern_construction_access.rs
- [x] T049 [P] [US4] Port test cases from gram-hs for length() inspection in tests/equivalence/pattern_construction_access.rs
- [x] T050 [P] [US4] Port test cases from gram-hs for size() inspection in tests/equivalence/pattern_construction_access.rs
- [x] T051 [P] [US4] Port test cases from gram-hs for depth() inspection in tests/equivalence/pattern_construction_access.rs
- [x] T052 [P] [US4] Create equivalence checking utilities for comparing gram-rs and gram-hs patterns in crates/pattern-core/src/test_utils/equivalence.rs

### Implementation for User Story 4

- [x] T053 [US4] Extract test data from gram-hs reference implementation in tests/equivalence/pattern_construction_access.rs
- [x] T054 [US4] Implement test case execution framework for equivalence tests in tests/equivalence/pattern_construction_access.rs
- [x] T055 [US4] Verify at least 95% of test cases from gram-hs pass (SC-004, SC-005) in tests/equivalence/pattern_construction_access.rs
- [x] T056 [US4] Document equivalence test results and any differences in tests/equivalence/pattern_construction_access.rs

**Checkpoint**: At this point, behavioral equivalence with gram-hs should be verified. Patterns constructed in gram-rs match the structure and behavior of equivalent patterns in gram-hs.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T057 [P] Update module-level documentation in crates/pattern-core/src/pattern.rs
- [x] T058 [P] Verify all public API exports are correct in crates/pattern-core/src/lib.rs
- [x] T059 [P] Run quickstart.md validation to ensure examples work
- [x] T060 [P] Add integration tests for cross-crate usage in tests/unit/pattern_core.rs
- [x] T061 [P] Performance testing for deep nesting (100+ levels) in tests/unit/pattern_core.rs
- [x] T062 [P] Performance testing for many elements (10,000+) in tests/unit/pattern_core.rs
- [x] T063 [P] Verify WASM compilation still works in crates/pattern-core/
- [x] T064 Code cleanup and refactoring across pattern.rs module

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **User Story 1 (Phase 2)**: Depends on Setup completion - Can start immediately after Setup
- **User Story 2 (Phase 3)**: Depends on User Story 1 completion (uses construction functions)
- **User Story 3 (Phase 4)**: Depends on User Story 1 and 2 completion (uses construction and access)
- **User Story 4 (Phase 5)**: Depends on User Stories 1, 2, and 3 completion (needs all functions)
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Setup (Phase 1) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after User Story 1 - Uses construction functions from US1
- **User Story 3 (P2)**: Can start after User Stories 1 and 2 - Uses construction and access functions
- **User Story 4 (P2)**: Can start after User Stories 1, 2, and 3 - Needs all functions for equivalence testing

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All tests for a user story marked [P] can run in parallel
- Different user stories can be worked on sequentially (after dependencies are met)

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Test Pattern::point() creates atomic pattern in tests/unit/pattern_core.rs"
Task: "Test Pattern::pattern() creates pattern with elements in tests/unit/pattern_core.rs"
Task: "Test Pattern::from_list() creates pattern from value list in tests/unit/pattern_core.rs"
Task: "Test construction with string values in tests/unit/pattern_core.rs"
Task: "Test construction with integer values in tests/unit/pattern_core.rs"
Task: "Test construction with Subject values in tests/unit/pattern_core.rs"
Task: "Test nested pattern construction in tests/unit/pattern_core.rs"
Task: "Test from_list converts values to atomic patterns in tests/unit/pattern_core.rs"
```

---

## Parallel Example: User Story 2

```bash
# Launch all tests for User Story 2 together:
Task: "Test value() returns correct value in tests/unit/pattern_core.rs"
Task: "Test elements() returns correct elements slice in tests/unit/pattern_core.rs"
Task: "Test value() preserves type information in tests/unit/pattern_core.rs"
Task: "Test elements() allows iteration in tests/unit/pattern_core.rs"
Task: "Test accessors work with nested patterns in tests/unit/pattern_core.rs"
Task: "Test accessors work with different value types in tests/unit/pattern_core.rs"
```

---

## Parallel Example: User Story 3

```bash
# Launch all tests for User Story 3 together:
Task: "Test length() returns direct element count in tests/unit/pattern_core.rs"
Task: "Test size() returns total node count in tests/unit/pattern_core.rs"
Task: "Test depth() returns maximum nesting depth in tests/unit/pattern_core.rs"
Task: "Test is_atomic() identifies atomic patterns in tests/unit/pattern_core.rs"
Task: "Test depth() returns 0 for atomic patterns in tests/unit/pattern_core.rs"
Task: "Test depth() calculates nested depth correctly in tests/unit/pattern_core.rs"
Task: "Test size() counts all nodes recursively in tests/unit/pattern_core.rs"
Task: "Test inspection utilities handle 100+ nesting levels in tests/unit/pattern_core.rs"
Task: "Test inspection utilities handle 10,000+ elements in tests/unit/pattern_core.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: User Story 1 (Construction functions)
3. **STOP and VALIDATE**: Test User Story 1 independently
4. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
6. Each story adds value without breaking previous stories

### Sequential Strategy

With a single developer:

1. Complete Setup
2. Complete User Story 1 (Construction) → Test → Commit
3. Complete User Story 2 (Access) → Test → Commit
4. Complete User Story 3 (Inspection) → Test → Commit
5. Complete User Story 4 (Equivalence) → Test → Commit
6. Complete Polish phase

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- Reference gram-hs implementation at `../gram-hs/libs/pattern/src/Pattern/Core.hs` for behavioral equivalence
- All functions must match gram-hs signatures: `point`, `pattern`, `fromList`, `length`, `size`, `depth`
- Accessors `value` and `elements` are field accessors in Haskell, methods in Rust
- Depth returns 0 for atomic patterns (corrected from previous inconsistency)

