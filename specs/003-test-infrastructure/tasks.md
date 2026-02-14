# Tasks: Testing Infrastructure

**Input**: Design documents from `/specs/003-test-infrastructure/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure for testing infrastructure

- [x] T001 Add proptest dependency to workspace Cargo.toml in Cargo.toml
- [x] T002 [P] Add proptest dev-dependency to pattern-core crate in crates/pattern-core/Cargo.toml
- [x] T003 [P] Add proptest dev-dependency to pattern-ops crate in crates/pattern-ops/Cargo.toml
- [x] T004 [P] Add proptest dev-dependency to gram-codec crate in crates/gram-codec/Cargo.toml
- [x] T005 Add insta dependency to workspace Cargo.toml in Cargo.toml
- [x] T006 [P] Add insta dev-dependency to pattern-core crate in crates/pattern-core/Cargo.toml
- [x] T007 [P] Add insta dev-dependency to pattern-ops crate in crates/pattern-ops/Cargo.toml
- [x] T008 [P] Add insta dev-dependency to gram-codec crate in crates/gram-codec/Cargo.toml
- [x] T009 Add criterion dependency to workspace Cargo.toml in Cargo.toml
- [x] T010 Create benches directory structure at workspace root in benches/
- [x] T011 [P] Create tests/property directory in pattern-core crate in crates/pattern-core/tests/property/
- [x] T012 [P] Create tests/equivalence directory in pattern-core crate in crates/pattern-core/tests/equivalence/
- [x] T013 [P] Create tests/snapshot directory in pattern-core crate in crates/pattern-core/tests/snapshot/
- [x] T014 [P] Create tests/property directory in pattern-ops crate in crates/pattern-ops/tests/property/
- [x] T015 [P] Create tests/equivalence directory in pattern-ops crate in crates/pattern-ops/tests/equivalence/
- [x] T016 [P] Create tests/snapshot directory in pattern-ops crate in crates/pattern-ops/tests/snapshot/

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T017 Create test utility module structure in pattern-core crate in crates/pattern-core/src/test_utils/mod.rs
- [x] T018 [P] Create equivalence module placeholder in crates/pattern-core/src/test_utils/equivalence.rs
- [x] T019 [P] Create helpers module placeholder in crates/pattern-core/src/test_utils/helpers.rs
- [x] T020 [P] Create generators module placeholder in crates/pattern-core/src/test_utils/generators.rs
- [x] T021 Export test_utils module from pattern-core lib.rs in crates/pattern-core/src/lib.rs
- [x] T022 Verify workspace builds with test dependencies in Cargo.toml
- [x] T023 Verify workspace tests run successfully with cargo test --workspace

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Property-Based Testing for Pattern Operations (Priority: P1) 🎯 MVP

**Goal**: Enable property-based testing with proptest that generates random test inputs automatically and validates properties hold true across many test cases

**Independent Test**: Verify that property-based tests can be written, run successfully, and generate test cases automatically. Write a simple property test (e.g., "pattern equality is symmetric") and see it pass with generated inputs.

### Implementation for User Story 1

- [x] T024 [US1] Configure proptest dependency in crates/pattern-core/Cargo.toml
- [x] T025 [US1] Create example property test file in crates/pattern-core/tests/property/equality.rs
- [x] T026 [US1] Implement basic property test example (equality symmetry) in crates/pattern-core/tests/property/equality.rs
- [x] T027 [US1] Create pattern generator placeholder in crates/pattern-core/src/test_utils/generators.rs (will be implemented when pattern types are defined in feature 004)
- [x] T028 [US1] Configure proptest test case count (100+ cases per SC-001) in crates/pattern-core/tests/property/equality.rs
- [x] T029 [US1] Verify property tests run and generate test cases in crates/pattern-core/tests/property/
- [x] T030 [US1] Test property test failure reporting with counterexamples in crates/pattern-core/tests/property/equality.rs
- [x] T031 [US1] Verify property tests work on WASM target (wasm32-unknown-unknown) in crates/pattern-core/tests/property/

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently. Developers can write property-based tests that generate 100+ test cases automatically.

---

## Phase 4: User Story 2 - Equivalence Checking Between pattern-rs and gram-hs (Priority: P1)

**Goal**: Provide utilities for checking behavioral equivalence between pattern-rs and gram-hs implementations using test data comparison

**Independent Test**: Verify that equivalence checking utilities exist, can execute operations, and report differences clearly. Run an equivalence check and see whether outputs match.

### Implementation for User Story 2

- [x] T032 [US2] Define EquivalenceResult struct in crates/pattern-core/src/test_utils/equivalence.rs
- [x] T033 [US2] Define EquivalenceOptions struct in crates/pattern-core/src/test_utils/equivalence.rs
- [x] T034 [US2] Implement check_equivalence function in crates/pattern-core/src/test_utils/equivalence.rs
- [x] T035 [US2] Implement check_equivalence_from_test_data function in crates/pattern-core/src/test_utils/equivalence.rs
- [x] T036 [US2] Implement difference reporting with field-level details in crates/pattern-core/src/test_utils/equivalence.rs
- [x] T037 [US2] Add support for approximate equality for floating-point values in crates/pattern-core/src/test_utils/equivalence.rs
- [x] T038 [US2] Create example equivalence test using test data in crates/pattern-core/tests/equivalence/test_data.rs
- [x] T039 [US2] Integrate equivalence checking with tests/common/test_cases.json in crates/pattern-core/tests/equivalence/test_data.rs
- [x] T040 [US2] Verify equivalence checking completes within 1 second per comparison (SC-003) in crates/pattern-core/tests/equivalence/
- [x] T041 [US2] Test equivalence checking error reporting with mismatched outputs in crates/pattern-core/tests/equivalence/test_data.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently. Equivalence checking utilities can compare outputs and report differences clearly.

---

## Phase 5: User Story 3 - Snapshot Testing for Regression Prevention (Priority: P2)

**Goal**: Enable snapshot testing with insta that captures outputs and detects changes to catch regressions

**Independent Test**: Verify that snapshot testing can capture outputs, store them, and detect changes. Write a snapshot test, see it capture output, then verify it detects changes when outputs differ.

### Implementation for User Story 3

- [x] T042 [US3] Configure insta snapshot storage in crates/pattern-core/tests/snapshot/ (crate-level snapshots)
- [x] T043 [US3] Create example snapshot test file in crates/pattern-core/tests/snapshot/serialization.rs
- [x] T044 [US3] Implement basic snapshot test example in crates/pattern-core/tests/snapshot/serialization.rs
- [x] T045 [US3] Verify snapshots are created and stored in crates/pattern-core/tests/__snapshots__/
- [x] T046 [US3] Test snapshot change detection in crates/pattern-core/tests/snapshot/serialization.rs
- [x] T047 [US3] Verify snapshot change detection completes within 2 seconds per snapshot (SC-004) in crates/pattern-core/tests/snapshot/
- [x] T048 [US3] Document snapshot review workflow (cargo insta review) in crates/pattern-core/tests/snapshot/README.md
- [x] T049 [US3] Create snapshot test structure in pattern-ops crate in crates/pattern-ops/tests/snapshot/
- [x] T050 [US3] Verify snapshot testing works across multiple crates in crates/pattern-core/tests/snapshot/ and crates/pattern-ops/tests/snapshot/

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all work independently. Snapshot testing can capture outputs and detect changes.

---

## Phase 6: User Story 4 - Test Data Extraction from gram-hs (Priority: P2)

**Goal**: Enhance test extraction utilities to extract test cases from gram-hs and convert them to usable format for pattern-rs tests

**Independent Test**: Verify that test extraction utilities exist, can parse gram-hs test files (or demonstrate structure), and produce test data in expected format. Run extraction and see test cases converted to usable format.

### Implementation for User Story 4

- [x] T051 [US4] Create test case validation function in scripts/sync-tests/extract.rs
- [x] T052 [US4] Implement JSON test case format validation in scripts/sync-tests/extract.rs
- [x] T053 [US4] Create test case loading utility in crates/pattern-core/src/test_utils/equivalence.rs
- [x] T054 [US4] Implement test case extraction from JSON format in scripts/sync-tests/extract.rs
- [x] T055 [US4] Create example test using extracted test data in crates/pattern-core/tests/equivalence/extracted_data.rs
- [x] T056 [US4] Verify extraction can process at least 10 test cases from gram-hs (SC-005) in scripts/sync-tests/extract.rs
- [x] T057 [US4] Add error handling for invalid test case formats in scripts/sync-tests/extract.rs
- [x] T058 [US4] Document test extraction workflow in scripts/sync-tests/README.md
- [x] T059 [US4] Create test case comparison utility in scripts/sync-tests/compare.rs
- [x] T060 [US4] Verify extracted test cases can be used in equivalence checking in crates/pattern-core/tests/equivalence/extracted_data.rs

**Checkpoint**: At this point, User Stories 1-4 should all work independently. Test extraction utilities can extract and validate test cases from gram-hs.

---

## Phase 7: User Story 5 - Benchmark Suite for Performance Validation (Priority: P3)

**Goal**: Create benchmark suite using criterion to measure and track performance of pattern operations

**Independent Test**: Verify that benchmark suite exists, can measure pattern operation performance, and reports results. Run benchmarks and see performance metrics for operations.

### Implementation for User Story 5

- [x] T061 [US5] Configure criterion benchmark in workspace Cargo.toml in Cargo.toml
- [x] T062 [US5] Create benchmark file structure in benches/pattern_operations.rs
- [x] T063 [US5] Implement basic benchmark example in benches/pattern_operations.rs
- [x] T064 [US5] Configure criterion with appropriate sample size and measurement time in benches/pattern_operations.rs
- [x] T065 [US5] Create placeholder benchmarks for pattern operations (to be implemented when operations are available) in benches/pattern_operations.rs
- [x] T066 [US5] Verify benchmarks run successfully with cargo bench in benches/
- [x] T067 [US5] Test benchmark consistency (variance <10% per SC-006) in benches/pattern_operations.rs
- [x] T068 [US5] Add conditional compilation for WASM targets (disable or simplify benchmarks) in benches/pattern_operations.rs
- [x] T069 [US5] Create benchmark documentation in benches/README.md
- [x] T070 [US5] Verify benchmarks are executable independently of test suite (FR-023) in benches/

**Checkpoint**: At this point, User Stories 1-5 should all work independently. Benchmark suite can measure performance and produce consistent results.

---

## Phase 8: User Story 6 - Test Helpers for Pattern Comparison (Priority: P3)

**Goal**: Provide test helper utilities for comparing patterns, checking equality, and validating pattern structure to reduce boilerplate

**Independent Test**: Verify that test helpers exist, can be used in tests, and simplify pattern comparison operations. Use helpers in tests and see cleaner, more readable test code.

### Implementation for User Story 6

- [x] T071 [US6] Define PatternComparisonError struct in crates/pattern-core/src/test_utils/helpers.rs
- [x] T072 [US6] Define PatternComparisonOptions struct in crates/pattern-core/src/test_utils/helpers.rs
- [x] T073 [US6] Define ValidationRules struct in crates/pattern-core/src/test_utils/helpers.rs
- [x] T074 [US6] Implement assert_patterns_equal function in crates/pattern-core/src/test_utils/helpers.rs
- [x] T075 [US6] Implement assert_pattern_structure_valid function in crates/pattern-core/src/test_utils/helpers.rs
- [x] T076 [US6] Implement assert_patterns_equivalent function in crates/pattern-core/src/test_utils/helpers.rs
- [x] T077 [US6] Create example test using test helpers in crates/pattern-core/tests/helpers_example.rs
- [x] T078 [US6] Verify test helpers reduce boilerplate by 50%+ (SC-007) by comparing test code with and without helpers in crates/pattern-core/tests/
- [x] T079 [US6] Test helpers with edge cases (empty patterns, deeply nested patterns) in crates/pattern-core/tests/helpers_example.rs
- [x] T080 [US6] Make test helpers available across all workspace crates (FR-024) by exporting from pattern-core in crates/pattern-core/src/lib.rs
- [x] T081 [US6] Create test helper usage documentation in crates/pattern-core/src/test_utils/helpers.rs

**Checkpoint**: At this point, all user stories should be independently functional. Test helpers simplify pattern comparison and reduce boilerplate.

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories and final integration

- [x] T082 [P] Update quickstart.md with testing infrastructure examples in specs/003-test-infrastructure/quickstart.md
- [x] T083 [P] Create testing infrastructure documentation in docs/testing-infrastructure.md
- [x] T084 Verify all testing infrastructure integrates with workspace structure (SC-008) by running cargo test --workspace
- [x] T085 Verify testing infrastructure works across at least 3 different crates (SC-009) in crates/pattern-core/, crates/pattern-ops/, crates/gram-codec/
- [x] T086 Test WASM compilation for all testing infrastructure in crates/pattern-core/, crates/pattern-ops/, crates/gram-codec/
- [x] T087 [P] Add testing infrastructure examples to quickstart.md in specs/003-test-infrastructure/quickstart.md
- [x] T088 Verify new developer can write property test within 15 minutes using documentation (SC-010) in docs/testing-infrastructure.md
- [x] T089 Run full workspace test suite to verify no regressions in Cargo.toml
- [x] T090 [P] Code cleanup and refactoring across test utilities in crates/pattern-core/src/test_utils/
- [x] T091 Update README.md with testing infrastructure information in README.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-8)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Phase 9)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - May use test data from US4 but should be independently testable
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 4 (P2)**: Can start after Foundational (Phase 2) - Enhances US2 but should be independently testable
- **User Story 5 (P3)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 6 (P3)**: Can start after Foundational (Phase 2) - Enhances other stories but should be independently testable

### Within Each User Story

- Core infrastructure before examples
- Examples before integration
- Basic functionality before edge cases
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel (T002-T004, T006-T008, T011-T016)
- All Foundational tasks marked [P] can run in parallel (T018-T020)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- Different user stories can be worked on in parallel by different team members
- Polish tasks marked [P] can run in parallel (T082, T087, T090)

---

## Parallel Example: User Story 1

```bash
# Launch property test setup tasks in parallel:
Task: "Configure proptest with WASM feature flag in crates/pattern-core/Cargo.toml"
Task: "Create example property test file in crates/pattern-core/tests/property/equality.rs"
Task: "Create pattern generator placeholder in crates/pattern-core/src/test_utils/generators.rs"
```

---

## Parallel Example: User Story 2

```bash
# Launch equivalence checking implementation tasks in parallel:
Task: "Define EquivalenceResult struct in crates/pattern-core/src/test_utils/equivalence.rs"
Task: "Define EquivalenceOptions struct in crates/pattern-core/src/test_utils/equivalence.rs"
Task: "Create example equivalence test using test data in crates/pattern-core/tests/equivalence/test_data.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (Property-Based Testing)
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Verify property tests generate 100+ test cases and report failures with counterexamples

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 (Property-Based Testing) → Test independently → Validate (MVP!)
3. Add User Story 2 (Equivalence Checking) → Test independently → Validate
4. Add User Story 3 (Snapshot Testing) → Test independently → Validate
5. Add User Story 4 (Test Extraction) → Test independently → Validate
6. Add User Story 5 (Benchmarks) → Test independently → Validate
7. Add User Story 6 (Test Helpers) → Test independently → Validate
8. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Property-Based Testing) + User Story 3 (Snapshot Testing)
   - Developer B: User Story 2 (Equivalence Checking) + User Story 4 (Test Extraction)
   - Developer C: User Story 5 (Benchmarks) + User Story 6 (Test Helpers)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Pattern types are not yet defined (feature 004), so pattern-related utilities are placeholders
- Test utilities start as module in pattern-core (can migrate to separate crate if needed)
- Verify tests fail before implementing (where applicable)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- WASM compatibility: proptest works as-is, insta works as-is, criterion needs conditional compilation
