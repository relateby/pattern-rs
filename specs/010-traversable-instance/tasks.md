# Tasks: Traversable Instance for Pattern

**Input**: Design documents from `/specs/010-traversable-instance/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: Property-based tests (proptest) for traversable laws, unit tests for behavior, integration tests with map/fold

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Library crate**: `crates/pattern-core/src/`, `crates/pattern-core/tests/`
- Tasks reference absolute paths from repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare testing infrastructure for traversable implementation

- [x] T001 Review existing Pattern<V> implementation in crates/pattern-core/src/pattern.rs
- [x] T002 Review existing map (Functor) implementation for consistency in crates/pattern-core/src/pattern.rs
- [x] T003 Review existing fold (Foldable) implementation for consistency in crates/pattern-core/src/pattern.rs
- [x] T004 [P] Create test file structure: crates/pattern-core/tests/traversable_option.rs
- [x] T005 [P] Create test file structure: crates/pattern-core/tests/traversable_result.rs
- [x] T006 [P] Create test file structure: crates/pattern-core/tests/traversable_validate.rs
- [x] T007 [P] Create test file structure: crates/pattern-core/tests/traversable_laws.rs
- [x] T008 [P] Create test file structure: crates/pattern-core/tests/traversable_integration.rs

**Checkpoint**: Test infrastructure ready for TDD approach

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core helper methods and infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T009 Study gram-hs Traversable instance in ../gram-hs/libs/pattern/src/Pattern/Core.hs
- [x] T010 Study gram-hs Traversable tests in ../gram-hs/libs/pattern/tests/Spec/Pattern/Properties.hs
- [x] T011 Document traversal order requirements (depth-first, root-first) in implementation notes
- [x] T012 Add test utilities for effect counting (short-circuit verification) in crates/pattern-core/src/test_utils/
- [x] T013 [P] Add proptest pattern generators with effects in crates/pattern-core/src/test_utils/

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Apply Effectful Transformations (Priority: P1) 🎯 MVP

**Goal**: Enable developers to apply Option/Result-returning functions to all pattern values with automatic effect sequencing

**Independent Test**: Create patterns, apply traverse_option/traverse_result, verify all values processed and effects properly sequenced (all Some → Some, any None → None; all Ok → Ok, any Err → Err)

### Implementation for User Story 1 - Part A: traverse_option

- [x] T014 [P] [US1] Write failing law test: identity law for Option in crates/pattern-core/tests/traversable_laws.rs
- [x] T015 [P] [US1] Write failing law test: structure preservation for Option in crates/pattern-core/tests/traversable_laws.rs
- [x] T016 [P] [US1] Write failing unit test: atomic pattern with Some in crates/pattern-core/tests/traversable_option.rs
- [x] T017 [P] [US1] Write failing unit test: atomic pattern with None in crates/pattern-core/tests/traversable_option.rs
- [x] T018 [P] [US1] Write failing unit test: nested pattern all Some in crates/pattern-core/tests/traversable_option.rs
- [x] T019 [P] [US1] Write failing unit test: nested pattern with None in crates/pattern-core/tests/traversable_option.rs
- [x] T020 [US1] Implement traverse_option method in crates/pattern-core/src/pattern.rs
- [x] T021 [US1] Implement traverse_option_with helper method (for recursion) in crates/pattern-core/src/pattern.rs
- [x] T022 [US1] Implement comprehensive documentation with examples to traverse_option in crates/pattern-core/src/pattern.rs
- [x] T023 [US1] Verify all Option tests pass in crates/pattern-core/tests/traversable_option.rs
- [x] T024 [US1] Verify Option law tests pass in crates/pattern-core/tests/traversable_laws.rs

### Implementation for User Story 1 - Part B: traverse_result ✅ COMPLETED

- [x] T025 [P] [US1] Write failing law test: identity law for Result in crates/pattern-core/tests/traversable_laws.rs
- [x] T026 [P] [US1] Write failing law test: structure preservation for Result in crates/pattern-core/tests/traversable_laws.rs
- [x] T027 [P] [US1] Write failing unit test: atomic pattern with Ok in crates/pattern-core/tests/traversable_result.rs
- [x] T028 [P] [US1] Write failing unit test: atomic pattern with Err in crates/pattern-core/tests/traversable_result.rs
- [x] T029 [P] [US1] Write failing unit test: nested pattern all Ok in crates/pattern-core/tests/traversable_result.rs
- [x] T030 [P] [US1] Write failing unit test: nested pattern with Err (short-circuit test) in crates/pattern-core/tests/traversable_result.rs
- [x] T031 [P] [US1] Write failing unit test: verify short-circuit behavior with side-effect counting in crates/pattern-core/tests/traversable_result.rs
- [x] T032 [US1] Implement traverse_result method in crates/pattern-core/src/pattern.rs
- [x] T033 [US1] Implement traverse_result_with helper method (for recursion) in crates/pattern-core/src/pattern.rs
- [x] T034 [US1] Add comprehensive documentation with examples to traverse_result in crates/pattern-core/src/pattern.rs
- [x] T035 [US1] Verify all Result tests pass in crates/pattern-core/tests/traversable_result.rs
- [x] T036 [US1] Verify Result law tests pass in crates/pattern-core/tests/traversable_laws.rs
- [x] T037 [US1] Verify short-circuit test passes (T031) in crates/pattern-core/tests/traversable_result.rs

**Checkpoint**: traverse_option and traverse_result fully functional and tested independently

---

## Phase 4: User Story 2 - Sequence Nested Effect Structures (Priority: P1)

**Goal**: Enable developers to flip structure layers (Pattern<Option<T>> → Option<Pattern<T>>, Pattern<Result<T, E>> → Result<Pattern<T>, E>)

**Independent Test**: Create Pattern<Option<i32>> and Pattern<Result<String, Error>>, apply sequence operations, verify correct layer flipping and all-or-nothing semantics

### Implementation for User Story 2 ✅ COMPLETED

- [x] T038 [P] [US2] Write failing unit test: sequence_option with all Some in crates/pattern-core/tests/traversable_option.rs
- [x] T039 [P] [US2] Write failing unit test: sequence_option with at least one None in crates/pattern-core/tests/traversable_option.rs
- [x] T040 [P] [US2] Write failing unit test: sequence_result with all Ok in crates/pattern-core/tests/traversable_result.rs
- [x] T041 [P] [US2] Write failing unit test: sequence_result with at least one Err in crates/pattern-core/tests/traversable_result.rs
- [x] T042 [P] [US2] Write failing unit test: nested pattern structure sequencing in crates/pattern-core/tests/traversable_option.rs
- [x] T043 [US2] Implement sequence_option method in crates/pattern-core/src/pattern.rs
- [x] T044 [US2] Implement sequence_result method in crates/pattern-core/src/pattern.rs
- [x] T045 [US2] Add comprehensive documentation with examples to sequence methods in crates/pattern-core/src/pattern.rs
- [x] T046 [US2] Verify all sequence_option tests pass in crates/pattern-core/tests/traversable_option.rs
- [x] T047 [US2] Verify all sequence_result tests pass in crates/pattern-core/tests/traversable_result.rs
- [x] T048 [US2] Add quickstart examples demonstrating sequence operations in crates/pattern-core/src/pattern.rs docs

**Checkpoint**: sequence_option and sequence_result fully functional, both US1 and US2 independently testable ✅

---

## Phase 5: User Story 3 - Validate with Error Collection (Priority: P2)

**Goal**: Enable comprehensive validation that collects ALL errors (not just first) for better user feedback

**Independent Test**: Create patterns with multiple invalid values, apply validate, verify all errors collected and reported

### Implementation for User Story 3 ✅ COMPLETED

- [x] T049 [P] [US3] Write failing unit test: validate_all with all valid values in crates/pattern-core/tests/traversable_validate.rs
- [x] T050 [P] [US3] Write failing unit test: validate_all with one invalid value in crates/pattern-core/tests/traversable_validate.rs
- [x] T051 [P] [US3] Write failing unit test: validate_all with multiple invalid values (verify all collected) in crates/pattern-core/tests/traversable_validate.rs
- [x] T052 [P] [US3] Write failing unit test: validate_all error ordering (root first, then elements) in crates/pattern-core/tests/traversable_validate.rs
- [x] T053 [P] [US3] Write failing unit test: validate_all processes all values (no short-circuit) in crates/pattern-core/tests/traversable_validate.rs
- [x] T054 [US3] Implement validate_all method with error collection in crates/pattern-core/src/pattern.rs (renamed from validate to avoid conflict)
- [x] T055 [US3] Implement validate_all_with helper method (for recursion) in crates/pattern-core/src/pattern.rs
- [x] T056 [US3] Add comprehensive documentation explaining difference from traverse_result in crates/pattern-core/src/pattern.rs
- [x] T057 [US3] Verify all validate_all tests pass in crates/pattern-core/tests/traversable_validate.rs
- [x] T058 [US3] Add quickstart examples demonstrating validate_all vs traverse_result tradeoffs in docs

**Checkpoint**: validate_all method fully functional with comprehensive error collection, all three user stories (US1, US2, US3) independently testable ✅

**Note**: Method renamed from `validate` to `validate_all` to avoid conflict with existing structural validation method.

---

## Phase 6: User Story 4 - Compose with Functor/Foldable (Priority: P3)

**Goal**: Ensure traversable operations compose cleanly with existing map and fold operations for complex pipelines

**Independent Test**: Build pipelines combining map, traverse, and fold; verify operations compose correctly and produce expected results

### Implementation for User Story 4 ✅ COMPLETED

- [x] T059 [P] [US4] Write integration test: map then traverse_result in crates/pattern-core/tests/traversable_integration.rs
- [x] T060 [P] [US4] Write integration test: traverse_option then map in crates/pattern-core/tests/traversable_integration.rs
- [x] T061 [P] [US4] Write integration test: traverse_result then fold in crates/pattern-core/tests/traversable_integration.rs
- [x] T062 [P] [US4] Write integration test: map, traverse_result, map pipeline in crates/pattern-core/tests/traversable_integration.rs
- [x] T063 [P] [US4] Write integration test: complex multi-step pipeline in crates/pattern-core/tests/traversable_integration.rs
- [x] T064 [US4] Verify traverse methods work with existing map implementation in crates/pattern-core/tests/traversable_integration.rs
- [x] T065 [US4] Verify traverse methods work with existing fold implementation in crates/pattern-core/tests/traversable_integration.rs
- [x] T066 [US4] Add documentation examples for common composition patterns in crates/pattern-core/src/pattern.rs
- [x] T067 [US4] Verify all integration tests pass in crates/pattern-core/tests/traversable_integration.rs

**Checkpoint**: Traversable composes cleanly with Functor and Foldable, all user stories (US1-4) independently testable ✅

---

## Phase 7: User Story 5 - Async Traverse Operations (Priority: P3)

**Goal**: Enable async effectful transformations with Future types (feature-gated)

**Independent Test**: Create patterns with async operations (simulated with tokio), apply traverse_future, verify sequential execution and proper error handling

**Note**: This phase is OPTIONAL and feature-gated behind "async" feature flag

### Implementation for User Story 5

- [ ] T068 [US5] Add async feature flag to Cargo.toml in crates/pattern-core/Cargo.toml
- [ ] T069 [US5] Add futures/tokio dev-dependency (feature-gated) in crates/pattern-core/Cargo.toml
- [ ] T070 [P] [US5] Create test file: crates/pattern-core/tests/traversable_async.rs (feature-gated)
- [ ] T071 [P] [US5] Write failing async test: sequential execution order in crates/pattern-core/tests/traversable_async.rs
- [ ] T072 [P] [US5] Write failing async test: successful async operations in crates/pattern-core/tests/traversable_async.rs
- [ ] T073 [P] [US5] Write failing async test: async operation with error (short-circuit) in crates/pattern-core/tests/traversable_async.rs
- [ ] T074 [US5] Implement traverse_future method (feature-gated) in crates/pattern-core/src/pattern.rs
- [ ] T075 [US5] Implement sequential .await pattern for ordering guarantee in crates/pattern-core/src/pattern.rs
- [ ] T076 [US5] Add comprehensive async documentation with examples in crates/pattern-core/src/pattern.rs
- [ ] T077 [US5] Verify all async tests pass (feature-gated) in crates/pattern-core/tests/traversable_async.rs
- [ ] T078 [US5] Document async feature flag usage in README

**Checkpoint**: Optional async support complete, all five user stories independently testable

---

## Phase 8: Traversable Laws & Property Testing

**Purpose**: Verify traversable laws hold for all effect types (ensures correctness)

- [ ] T079 [P] Write property test: identity law for Option (100+ cases) in crates/pattern-core/tests/traversable_laws.rs
- [ ] T080 [P] Write property test: identity law for Result (100+ cases) in crates/pattern-core/tests/traversable_laws.rs
- [ ] T081 [P] Write property test: structure preservation for Option (100+ cases) in crates/pattern-core/tests/traversable_laws.rs
- [ ] T082 [P] Write property test: structure preservation for Result (100+ cases) in crates/pattern-core/tests/traversable_laws.rs
- [ ] T083 [P] Write property test: naturality law (if applicable to Rust) in crates/pattern-core/tests/traversable_laws.rs
- [ ] T084 Verify all property tests pass with 100+ random test cases in crates/pattern-core/tests/traversable_laws.rs
- [ ] T085 Document law test adaptations for Rust in crates/pattern-core/tests/traversable_laws.rs

**Checkpoint**: Traversable laws verified for correctness

---

## Phase 9: Performance & Edge Cases

**Purpose**: Verify performance targets and handle edge cases

- [ ] T086 [P] Write performance test: 1000 nodes in <50ms in crates/pattern-core/benches/ (if using criterion)
- [ ] T087 [P] Write stack safety test: 100+ nesting levels in crates/pattern-core/tests/traversable_option.rs
- [ ] T088 [P] Write memory test: 10,000 elements in <100MB in crates/pattern-core/tests/traversable_result.rs
- [ ] T089 [P] Write edge case test: atomic pattern (no elements) in crates/pattern-core/tests/traversable_option.rs
- [ ] T090 [P] Write edge case test: deeply nested pattern in crates/pattern-core/tests/traversable_result.rs
- [ ] T091 [P] Write edge case test: wide pattern (many siblings) in crates/pattern-core/tests/traversable_validate.rs
- [ ] T092 Verify all performance targets met per spec.md success criteria
- [ ] T093 Verify all edge cases handled correctly

**Checkpoint**: Performance and edge cases verified

---

## Phase 10: WASM Compatibility & Documentation

**Purpose**: Ensure WASM compilation and complete documentation

- [ ] T094 [P] Verify traverse_option compiles for wasm32-unknown-unknown target
- [ ] T095 [P] Verify traverse_result compiles for wasm32-unknown-unknown target
- [ ] T096 [P] Verify validate compiles for wasm32-unknown-unknown target
- [ ] T097 [P] Verify sequence operations compile for wasm32-unknown-unknown target
- [ ] T098 Add WASM compilation test to CI in .github/workflows/ (if not already present)
- [ ] T099 [P] Review and update quickstart.md examples
- [ ] T100 [P] Review and update contracts/type-signatures.md
- [ ] T101 [P] Review and update data-model.md
- [ ] T102 Update crate-level documentation in crates/pattern-core/src/lib.rs
- [ ] T103 Add changelog entry for traversable instance

**Checkpoint**: WASM compatible, documentation complete

---

## Phase 11: Polish & Cross-Cutting Concerns

**Purpose**: Final improvements affecting multiple user stories

- [ ] T104 [P] Run cargo clippy and fix any warnings in crates/pattern-core/src/pattern.rs
- [ ] T105 [P] Run cargo fmt to ensure consistent formatting
- [ ] T106 Verify all tests pass: cargo test --all-features
- [ ] T107 Verify WASM target: cargo build --target wasm32-unknown-unknown
- [ ] T108 [P] Add inline documentation examples (doc tests) for all public methods
- [ ] T109 Port remaining gram-hs traversable tests (if any not yet ported)
- [ ] T110 Update TODO.md to mark feature 010 as complete
- [ ] T111 Create feature completion summary in specs/010-traversable-instance/

**Checkpoint**: Feature complete, all success criteria verified

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-7)**: All depend on Foundational phase completion
  - User Story 1 (P1): traverse_option, traverse_result - Core functionality
  - User Story 2 (P1): sequence operations - Depends on US1 methods
  - User Story 3 (P2): validate - Independent of US2, but uses similar patterns to US1
  - User Story 4 (P3): Integration - Can verify composition at any point after US1
  - User Story 5 (P3): Async - Independent, feature-gated
- **Laws (Phase 8)**: Can start after US1 (Option/Result methods exist)
- **Performance (Phase 9)**: Can start after US1
- **WASM/Docs (Phase 10)**: Can start after any implementation complete
- **Polish (Phase 11)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories ✅ MVP
- **User Story 2 (P1)**: Depends on US1 (uses traverse_option/traverse_result internally)
- **User Story 3 (P2)**: Can start after Foundational - Independent of US2
- **User Story 4 (P3)**: Depends on US1 (needs traverse methods to test composition)
- **User Story 5 (P3)**: Can start after Foundational - Independent (feature-gated)

### Within Each User Story

- Tests written FIRST and must FAIL before implementation
- Helper methods (_with variants) implemented alongside public methods
- Documentation added immediately after implementation
- All tests for a story pass before moving to next story

### Parallel Opportunities

- **Setup (Phase 1)**: All test file creation tasks (T004-T008) can run in parallel
- **Foundational (Phase 2)**: Test utilities (T012, T013) can run in parallel
- **User Story 1**: 
  - All Option test writing tasks (T014-T019) can run in parallel
  - All Result test writing tasks (T025-T031) can run in parallel
- **User Story 2**: All test writing tasks (T038-T042) can run in parallel
- **User Story 3**: All test writing tasks (T049-T053) can run in parallel
- **User Story 4**: All integration test writing tasks (T059-T063) can run in parallel
- **User Story 5**: All async test writing tasks (T071-T073) can run in parallel
- **Phase 8 (Laws)**: All property test writing tasks (T079-T083) can run in parallel
- **Phase 9 (Performance)**: All performance/edge case tests (T086-T091) can run in parallel
- **Phase 10 (WASM)**: All WASM compilation verification (T094-T097) and doc reviews (T099-T101) can run in parallel
- **Phase 11 (Polish)**: Clippy, fmt, doc tests (T104, T105, T108) can run in parallel

**After Foundational phase completes**:
- US1, US3, and US5 can all start in parallel (independent)
- US2 can start after US1 completes
- US4 can start after US1 completes

---

## Parallel Example: User Story 1 (Part A - Option)

```bash
# Launch all Option test tasks together (tests written BEFORE implementation):
Task T014: "Write failing law test: identity law for Option"
Task T015: "Write failing law test: structure preservation for Option"
Task T016: "Write failing unit test: atomic pattern with Some"
Task T017: "Write failing unit test: atomic pattern with None"
Task T018: "Write failing unit test: nested pattern all Some"
Task T019: "Write failing unit test: nested pattern with None"

# After tests written, implement:
Task T020: "Implement traverse_option method"
Task T021: "Implement traverse_option_with helper method"

# Then verify:
Task T022: "Add documentation"
Task T023: "Verify all Option tests pass"
Task T024: "Verify Option law tests pass"
```

---

## Parallel Example: User Story 1 (Part B - Result)

```bash
# Launch all Result test tasks together:
Task T025: "Write failing law test: identity law for Result"
Task T026: "Write failing law test: structure preservation for Result"
Task T027: "Write failing unit test: atomic pattern with Ok"
Task T028: "Write failing unit test: atomic pattern with Err"
Task T029: "Write failing unit test: nested pattern all Ok"
Task T030: "Write failing unit test: nested pattern with Err"
Task T031: "Write failing unit test: verify short-circuit with side-effect counting"

# After tests written, implement:
Task T032: "Implement traverse_result method"
Task T033: "Implement traverse_result_with helper method"

# Then verify:
Task T034: "Add documentation"
Task T035: "Verify all Result tests pass"
Task T036: "Verify Result law tests pass"
Task T037: "Verify short-circuit test passes"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (traverse_option and traverse_result)
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Optionally add Phase 8 (laws) and Phase 9 (performance) for US1
6. Deploy/demo if ready

**MVP Delivers**: Core traversable functionality (traverse_option, traverse_result) enabling effectful transformations on patterns

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → **MVP COMPLETE** ✅
3. Add User Story 2 → Test independently → Sequence operations available
4. Add User Story 3 → Test independently → Validation with error collection available
5. Add User Story 4 → Test independently → Confirmed composition with map/fold
6. Add User Story 5 (optional) → Test independently → Async support available
7. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (traverse_option, traverse_result)
   - Developer B: User Story 3 (validate) - independent of US2
   - Developer C: User Story 5 (async) - independent, feature-gated
3. After US1 completes:
   - Developer D: User Story 2 (sequence operations) - needs US1
   - Developer E: User Story 4 (integration tests) - needs US1
4. Stories complete and integrate independently

---

## Notes

- **[P] tasks** = different files, no dependencies, can run in parallel
- **[Story] label** maps task to specific user story for traceability
- **Each user story** should be independently completable and testable
- **TDD approach**: Verify tests fail before implementing
- **Commit** after each task or logical group
- **Stop at any checkpoint** to validate story independently
- **WASM compatibility** verified throughout (core features only, async feature-gated)
- **Behavioral equivalence** with gram-hs verified via ported tests and property tests
- **Performance targets**: <50ms for 1000 nodes, 100+ nesting levels, <100MB for 10K elements

## Task Count Summary

- **Total Tasks**: 111
- **Phase 1 (Setup)**: 8 tasks
- **Phase 2 (Foundational)**: 5 tasks (BLOCKS all user stories)
- **Phase 3 (US1 - P1)**: 24 tasks (traverse_option, traverse_result) 🎯 **MVP**
- **Phase 4 (US2 - P1)**: 11 tasks (sequence operations)
- **Phase 5 (US3 - P2)**: 10 tasks (validate with error collection)
- **Phase 6 (US4 - P3)**: 9 tasks (composition with map/fold)
- **Phase 7 (US5 - P3)**: 11 tasks (async support, optional/feature-gated)
- **Phase 8 (Laws)**: 7 tasks (property testing)
- **Phase 9 (Performance)**: 8 tasks (perf & edge cases)
- **Phase 10 (WASM/Docs)**: 10 tasks (compilation & documentation)
- **Phase 11 (Polish)**: 8 tasks (final improvements)

**Parallel Opportunities**: 47 tasks marked [P] can run in parallel within their phases

**Independent Stories**: US1, US3, US5 can start in parallel after Foundational phase

**MVP Scope**: Phases 1, 2, and 3 (37 tasks total) deliver core traversable functionality

