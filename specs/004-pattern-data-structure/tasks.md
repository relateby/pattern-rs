# Tasks: Core Pattern Data Structure

**Input**: Design documents from `/specs/004-pattern-data-structure/`
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

**Purpose**: Project initialization and basic structure

- [x] T001 Create pattern.rs module file in crates/pattern-core/src/pattern.rs
- [x] T002 Create subject.rs module file in crates/pattern-core/src/subject.rs
- [x] T003 Update lib.rs to export pattern and subject modules in crates/pattern-core/src/lib.rs
- [x] T004 [P] Create unit test file in tests/unit/pattern_core.rs
- [x] T005 [P] Create equivalence test file in tests/equivalence/pattern_structure.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core type definitions that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T006 [P] Define Symbol struct in crates/pattern-core/src/subject.rs
- [x] T007 [P] Define RangeValue struct in crates/pattern-core/src/subject.rs
- [x] T008 [P] Define Value enum in crates/pattern-core/src/subject.rs
- [x] T009 [P] Define PropertyRecord type alias in crates/pattern-core/src/subject.rs
- [x] T010 [P] Define Subject struct in crates/pattern-core/src/subject.rs
- [x] T011 [P] Define Pattern<V> struct in crates/pattern-core/src/pattern.rs
- [x] T012 [P] Implement Clone trait for Symbol in crates/pattern-core/src/subject.rs
- [x] T013 [P] Implement Clone trait for RangeValue in crates/pattern-core/src/subject.rs
- [x] T014 [P] Implement Clone trait for Value in crates/pattern-core/src/subject.rs
- [x] T015 [P] Implement Clone trait for Subject in crates/pattern-core/src/subject.rs
- [x] T016 [P] Implement Clone trait for Pattern<V> in crates/pattern-core/src/pattern.rs
- [x] T017 [P] Implement PartialEq trait for Symbol in crates/pattern-core/src/subject.rs
- [x] T018 [P] Implement PartialEq trait for RangeValue in crates/pattern-core/src/subject.rs
- [x] T019 [P] Implement PartialEq trait for Value in crates/pattern-core/src/subject.rs
- [x] T020 [P] Implement PartialEq trait for Subject in crates/pattern-core/src/subject.rs
- [x] T021 [P] Implement PartialEq trait for Pattern<V> in crates/pattern-core/src/pattern.rs
- [x] T022 [P] Implement Eq trait for Symbol in crates/pattern-core/src/subject.rs
- [x] T023 [P] Implement Eq trait for RangeValue in crates/pattern-core/src/subject.rs
- [x] T024 [P] Implement Eq trait for Value in crates/pattern-core/src/subject.rs
- [x] T025 [P] Implement Eq trait for Subject in crates/pattern-core/src/subject.rs
- [x] T026 [P] Implement Eq trait for Pattern<V> in crates/pattern-core/src/pattern.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Developer Creates Pattern Instances (Priority: P1) 🎯 MVP

**Goal**: Enable developers to create pattern instances as recursive, nested structures (s-expression-like) with different value types and element structures.

**Independent Test**: Verify that developers can create pattern instances with different values and element structures, and that these patterns can be inspected (via Debug/Display) to confirm their structure matches expectations.

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T027 [P] [US1] Test pattern creation with string value in tests/unit/pattern_core.rs
- [x] T028 [P] [US1] Test pattern creation with integer value in tests/unit/pattern_core.rs
- [x] T029 [P] [US1] Test pattern creation with empty elements in tests/unit/pattern_core.rs
- [x] T030 [P] [US1] Test pattern creation with nested elements in tests/unit/pattern_core.rs
- [x] T031 [P] [US1] Test pattern creation with custom value type in tests/unit/pattern_core.rs
- [x] T032 [P] [US1] Test pattern deep nesting (100+ levels) in tests/unit/pattern_core.rs
- [x] T033 [P] [US1] Test pattern with many elements (10,000+) in tests/unit/pattern_core.rs

### Implementation for User Story 1

- [x] T034 [US1] Verify Pattern<V> struct compiles and can be instantiated in crates/pattern-core/src/pattern.rs
- [x] T035 [US1] Verify Clone trait works for Pattern<V> with various value types in crates/pattern-core/src/pattern.rs
- [x] T036 [US1] Verify PartialEq and Eq traits work for Pattern<V> in crates/pattern-core/src/pattern.rs

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently. Developers can create patterns with any value type and nested structures.

---

## Phase 4: User Story 2 - Developer Works with Subject Type (Priority: P1)

**Goal**: Enable developers to work with the Subject type as a value type when building applications that use patterns to replace object-graphs.

**Independent Test**: Verify that Subject can be used as a pattern value in `Pattern<Subject>`, and that its structure can be inspected. Developers can create patterns with Subject values and verify the Subject information is preserved in the pattern value.

### Tests for User Story 2

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T037 [P] [US2] Test Subject creation with identity, labels, and properties in tests/unit/pattern_core.rs
- [x] T038 [P] [US2] Test Subject with labels preservation in tests/unit/pattern_core.rs
- [x] T039 [P] [US2] Test Subject with properties preservation in tests/unit/pattern_core.rs
- [x] T040 [P] [US2] Test Pattern<Subject> creation and value preservation in tests/unit/pattern_core.rs
- [x] T041 [P] [US2] Test Subject equality comparison in tests/unit/pattern_core.rs
- [x] T042 [P] [US2] Test Value enum variants (VInteger, VDecimal, VBoolean, VString, etc.) in tests/unit/pattern_core.rs
- [x] T043 [P] [US2] Test RangeValue with optional bounds in tests/unit/pattern_core.rs

### Implementation for User Story 2

- [x] T044 [US2] Verify Subject struct compiles and can be instantiated in crates/pattern-core/src/subject.rs
- [x] T045 [US2] Verify Symbol struct works correctly in crates/pattern-core/src/subject.rs
- [x] T046 [US2] Verify Value enum variants work correctly in crates/pattern-core/src/subject.rs
- [x] T047 [US2] Verify RangeValue struct works correctly in crates/pattern-core/src/subject.rs
- [x] T048 [US2] Verify PropertyRecord type alias works correctly in crates/pattern-core/src/subject.rs
- [x] T049 [US2] Verify Clone trait works for Subject and all components in crates/pattern-core/src/subject.rs
- [x] T050 [US2] Verify PartialEq and Eq traits work for Subject and all components in crates/pattern-core/src/subject.rs
- [x] T051 [US2] Verify Pattern<Subject> can be created and used in crates/pattern-core/src/pattern.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently. Developers can create patterns with Subject values.

---

## Phase 5: User Story 3 - Developer Inspects Pattern Structure (Priority: P2)

**Goal**: Enable developers to inspect and debug pattern structures during development with human-readable representations.

**Independent Test**: Verify that patterns implement Debug and Display traits, and that the output is readable and accurately represents the pattern structure. Developers can print patterns and see meaningful output.

### Tests for User Story 3

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T052 [P] [US3] Test Debug trait output for Pattern<V> with simple value in tests/unit/pattern_core.rs
- [x] T053 [P] [US3] Test Debug trait output for Pattern<V> with nested elements in tests/unit/pattern_core.rs
- [x] T054 [P] [US3] Test Debug trait output for Pattern<Subject> in tests/unit/pattern_core.rs
- [x] T055 [P] [US3] Test Debug trait truncation for deeply nested patterns in tests/unit/pattern_core.rs
- [x] T056 [P] [US3] Test Display trait output for Pattern<V> with simple value in tests/unit/pattern_core.rs
- [x] T057 [P] [US3] Test Display trait output for Pattern<V> with nested elements in tests/unit/pattern_core.rs
- [x] T058 [P] [US3] Test Display trait output for Pattern<Subject> in tests/unit/pattern_core.rs
- [x] T059 [P] [US3] Test Debug trait for Subject, Symbol, Value, RangeValue in tests/unit/pattern_core.rs
- [x] T060 [P] [US3] Test Display trait for Subject, Symbol, Value, RangeValue in tests/unit/pattern_core.rs

### Implementation for User Story 3

- [x] T061 [US3] Implement Debug trait for Pattern<V> with truncation for deep nesting in crates/pattern-core/src/pattern.rs
- [x] T062 [US3] Implement Display trait for Pattern<V> with human-readable output in crates/pattern-core/src/pattern.rs
- [x] T063 [US3] Implement Debug trait for Symbol in crates/pattern-core/src/subject.rs
- [x] T064 [US3] Implement Display trait for Symbol in crates/pattern-core/src/subject.rs
- [x] T065 [US3] Implement Debug trait for RangeValue in crates/pattern-core/src/subject.rs
- [x] T066 [US3] Implement Display trait for RangeValue in crates/pattern-core/src/subject.rs
- [x] T067 [US3] Implement Debug trait for Value enum in crates/pattern-core/src/subject.rs
- [x] T068 [US3] Implement Display trait for Value enum in crates/pattern-core/src/subject.rs
- [x] T069 [US3] Implement Debug trait for Subject in crates/pattern-core/src/subject.rs
- [x] T070 [US3] Implement Display trait for Subject in crates/pattern-core/src/subject.rs

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all work independently. Developers can create patterns and inspect them with Debug and Display.

---

## Phase 6: User Story 4 - Developer Verifies Behavioral Equivalence with gram-hs (Priority: P2)

**Goal**: Enable developers to verify that pattern instances created in gram-rs behave identically to patterns created in the gram-hs reference implementation.

**Independent Test**: Create equivalent patterns in both gram-rs and gram-hs, compare their structure and behavior, and verify they match. Test cases from gram-hs can be ported and executed in gram-rs with identical results.

### Tests for User Story 4

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T071 [P] [US4] Port test cases from gram-hs for Pattern structure in tests/equivalence/pattern_structure.rs
- [x] T072 [P] [US4] Port test cases from gram-hs for Pattern equality in tests/equivalence/pattern_structure.rs
- [x] T073 [P] [US4] Port test cases from gram-hs for Subject structure in tests/equivalence/pattern_structure.rs
- [x] T074 [P] [US4] Port test cases from gram-hs for Subject equality in tests/equivalence/pattern_structure.rs
- [x] T075 [P] [US4] Port test cases from gram-hs for Pattern<Subject> in tests/equivalence/pattern_structure.rs
- [x] T076 [P] [US4] Create equivalence checking utilities for comparing gram-rs and gram-hs patterns in crates/pattern-core/src/test_utils/equivalence.rs

### Implementation for User Story 4

- [x] T077 [US4] Extract test data from gram-hs reference implementation in tests/equivalence/pattern_structure.rs
- [x] T078 [US4] Implement test case execution framework for equivalence tests in tests/equivalence/pattern_structure.rs
- [x] T079 [US4] Verify at least 95% of test cases from gram-hs pass (SC-005) in tests/equivalence/pattern_structure.rs
- [x] T080 [US4] Document equivalence test results and any differences in tests/equivalence/pattern_structure.rs

**Checkpoint**: At this point, behavioral equivalence with gram-hs should be verified. Patterns created in gram-rs match the structure and behavior of equivalent patterns in gram-hs.

---

## Phase 7: User Story 5 - Developer Compiles Patterns for WASM (Priority: P3)

**Goal**: Enable developers to use patterns in web applications via WebAssembly with confidence that pattern types compile successfully for WASM targets.

**Independent Test**: Compile the pattern-core crate for `wasm32-unknown-unknown` target and verify successful compilation without errors. The patterns don't need to be usable from JavaScript yet, just compilable.

### Tests for User Story 5

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T081 [P] [US5] Test WASM compilation for pattern-core crate in tests/unit/pattern_core.rs
- [x] T082 [P] [US5] Verify Pattern<V> types are included in WASM module in tests/unit/pattern_core.rs
- [x] T083 [P] [US5] Verify Subject types are included in WASM module in tests/unit/pattern_core.rs

### Implementation for User Story 5

- [x] T084 [US5] Verify pattern-core crate compiles for wasm32-unknown-unknown target
- [x] T085 [US5] Fix any WASM compilation errors in crates/pattern-core/src/
- [x] T086 [US5] Verify all types compile successfully for WASM in crates/pattern-core/src/
- [x] T087 [US5] Document WASM compilation verification in crates/pattern-core/README.md or similar

**Checkpoint**: At this point, WASM compilation should succeed. Pattern types are compatible with WASM targets.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T088 [P] Update documentation in crates/pattern-core/src/lib.rs with module-level docs
- [x] T089 [P] Add doc comments to Pattern<V> struct in crates/pattern-core/src/pattern.rs
- [x] T090 [P] Add doc comments to Subject struct in crates/pattern-core/src/subject.rs
- [x] T091 [P] Add doc comments to Symbol, Value, RangeValue, PropertyRecord in crates/pattern-core/src/subject.rs
- [x] T092 Code cleanup and refactoring across all modules
- [x] T093 [P] Run quickstart.md validation to ensure examples work
- [x] T094 [P] Verify all public API exports are correct in crates/pattern-core/src/lib.rs
- [x] T095 [P] Add integration tests for cross-crate usage in tests/unit/pattern_core.rs
- [x] T096 [P] Performance testing for deep nesting (100+ levels) in tests/unit/pattern_core.rs
- [x] T097 [P] Performance testing for many elements (10,000+) in tests/unit/pattern_core.rs

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Phase 8)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - Uses Pattern<V> from US1 but should be independently testable
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - Depends on US1 and US2 for Debug/Display implementations
- **User Story 4 (P2)**: Can start after Foundational (Phase 2) - Depends on US1, US2, US3 for complete pattern functionality
- **User Story 5 (P3)**: Can start after Foundational (Phase 2) - Depends on all types being defined (US1, US2)

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Type definitions before trait implementations
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, User Stories 1 and 2 can start in parallel (both P1)
- All tests for a user story marked [P] can run in parallel
- Type definitions within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members (after dependencies are met)

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Test pattern creation with string value in tests/unit/pattern_core.rs"
Task: "Test pattern creation with integer value in tests/unit/pattern_core.rs"
Task: "Test pattern creation with empty elements in tests/unit/pattern_core.rs"
Task: "Test pattern creation with nested elements in tests/unit/pattern_core.rs"
Task: "Test pattern creation with custom value type in tests/unit/pattern_core.rs"
Task: "Test pattern deep nesting (100+ levels) in tests/unit/pattern_core.rs"
Task: "Test pattern with many elements (10,000+) in tests/unit/pattern_core.rs"
```

---

## Parallel Example: User Story 2

```bash
# Launch all Subject component tests together:
Task: "Test Subject creation with identity, labels, and properties in tests/unit/pattern_core.rs"
Task: "Test Subject with labels preservation in tests/unit/pattern_core.rs"
Task: "Test Subject with properties preservation in tests/unit/pattern_core.rs"
Task: "Test Pattern<Subject> creation and value preservation in tests/unit/pattern_core.rs"
Task: "Test Subject equality comparison in tests/unit/pattern_core.rs"
Task: "Test Value enum variants (VInteger, VDecimal, VBoolean, VString, etc.) in tests/unit/pattern_core.rs"
Task: "Test RangeValue with optional bounds in tests/unit/pattern_core.rs"
```

---

## Parallel Example: Foundational Phase

```bash
# Launch all type definitions together:
Task: "Define Symbol struct in crates/pattern-core/src/subject.rs"
Task: "Define RangeValue struct in crates/pattern-core/src/subject.rs"
Task: "Define Value enum in crates/pattern-core/src/subject.rs"
Task: "Define PropertyRecord type alias in crates/pattern-core/src/subject.rs"
Task: "Define Subject struct in crates/pattern-core/src/subject.rs"
Task: "Define Pattern<V> struct in crates/pattern-core/src/pattern.rs"

# Then launch all Clone implementations together:
Task: "Implement Clone trait for Symbol in crates/pattern-core/src/subject.rs"
Task: "Implement Clone trait for RangeValue in crates/pattern-core/src/subject.rs"
Task: "Implement Clone trait for Value in crates/pattern-core/src/subject.rs"
Task: "Implement Clone trait for Subject in crates/pattern-core/src/subject.rs"
Task: "Implement Clone trait for Pattern<V> in crates/pattern-core/src/pattern.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
6. Add User Story 5 → Test independently → Deploy/Demo
7. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Pattern creation)
   - Developer B: User Story 2 (Subject type) - can start in parallel with US1
3. After US1 and US2 complete:
   - Developer A: User Story 3 (Debug/Display)
   - Developer B: User Story 4 (Equivalence tests)
4. After US3 and US4 complete:
   - Developer A: User Story 5 (WASM compilation)
   - Developer B: Polish phase

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- All trait implementations should use derive macros where possible (Clone, PartialEq, Eq)
- Custom implementations needed for Debug and Display traits
- Reference gram-hs implementation at `../gram-hs/libs/` for behavioral equivalence

