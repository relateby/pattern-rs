# Tasks: Pattern Query Operations

**Input**: Design documents from `/specs/011-basic-query-functions/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/type-signatures.md

**Tests**: Comprehensive test coverage is a PRIMARY requirement for this feature. All test tasks are mandatory.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

Using existing Rust workspace structure:
- **Source**: `crates/pattern-core/src/pattern.rs` (add new methods)
- **Tests**: `crates/pattern-core/tests/` (separate test file per operation)
- **Benchmarks**: `crates/pattern-core/benches/` (performance verification)
- **Docs**: Method documentation in source, module docs update

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Verify build environment and prepare for implementation

- [x] T001 Verify Rust toolchain version 1.75+ is installed
- [x] T002 Verify existing Pattern<V> type and fold() method in crates/pattern-core/src/pattern.rs
- [x] T003 [P] Review Haskell reference implementation at ../gram-hs/libs/pattern/src/Pattern/Core.hs lines 945-1028

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**✅ Status**: All foundational infrastructure already exists from previous features
- Pattern<V> type (feature 004)
- fold() method (feature 009)
- Test infrastructure (feature 003)
- Test utilities and generators (feature 003)

**No foundational tasks needed** - proceed directly to user story implementation

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Check if Any Value Satisfies Predicate (Priority: P1) 🎯 MVP

**Goal**: Implement `any_value` operation to check if at least one value in a pattern satisfies a given predicate, with short-circuit evaluation

**Independent Test**: Can call `pattern.any_value(predicate)` on patterns with various structures and receive correct boolean results. Short-circuit behavior verified through performance tests.

### Tests for User Story 1 (TDD Approach)

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T004 [P] [US1] Create test file crates/pattern-core/tests/query_any_value.rs with basic structure
- [x] T005 [P] [US1] Write unit test for any_value with atomic pattern containing matching value in crates/pattern-core/tests/query_any_value.rs
- [x] T006 [P] [US1] Write unit test for any_value with atomic pattern containing non-matching value in crates/pattern-core/tests/query_any_value.rs
- [x] T007 [P] [US1] Write unit test for any_value with nested pattern where value matches at different levels in crates/pattern-core/tests/query_any_value.rs
- [x] T008 [P] [US1] Write unit test for any_value with pattern containing no matching values in crates/pattern-core/tests/query_any_value.rs
- [x] T009 [P] [US1] Write unit test for any_value with deeply nested pattern (100+ levels) in crates/pattern-core/tests/query_any_value.rs
- [x] T010 [P] [US1] Write unit test for any_value with large flat pattern (1000+ elements) in crates/pattern-core/tests/query_any_value.rs

### Implementation for User Story 1

- [x] T011 [US1] Implement any_value method in crates/pattern-core/src/pattern.rs using fold with OR combinator
- [x] T012 [US1] Add comprehensive documentation for any_value including examples and complexity analysis in crates/pattern-core/src/pattern.rs
- [x] T013 [US1] Export any_value in crates/pattern-core/src/lib.rs public API
- [x] T014 [US1] Verify all US1 unit tests pass with cargo test query_any_value

### Property Tests for User Story 1

- [x] T015 [P] [US1] Create property test file crates/pattern-core/tests/query_any_value_property.rs
- [x] T016 [P] [US1] Implement property test: any_value(const true) always returns true in crates/pattern-core/tests/query_any_value_property.rs
- [x] T017 [P] [US1] Implement property test: any_value(const false) always returns false in crates/pattern-core/tests/query_any_value_property.rs
- [x] T018 [P] [US1] Implement property test: any_value consistent with any() over values() in crates/pattern-core/tests/query_any_value_property.rs

### Performance Verification for User Story 1

- [x] T019 [US1] Create benchmark for any_value short-circuit behavior in crates/pattern-core/benches/query_benchmarks.rs
- [x] T020 [US1] Verify any_value meets <100ms target for 10,000 node patterns with cargo bench

**Checkpoint**: At this point, User Story 1 (any_value) should be fully functional, tested, and performant

---

## Phase 4: User Story 2 - Check if All Values Satisfy Predicate (Priority: P1)

**Goal**: Implement `all_values` operation to check if all values in a pattern satisfy a given predicate, with short-circuit evaluation and vacuous truth for empty patterns

**Independent Test**: Can call `pattern.all_values(predicate)` on patterns with various structures and receive correct boolean results. Vacuous truth behavior verified for empty patterns.

### Tests for User Story 2 (TDD Approach)

- [x] T021 [P] [US2] Create test file crates/pattern-core/tests/query_all_values.rs with basic structure
- [x] T022 [P] [US2] Write unit test for all_values with atomic pattern where all values match in crates/pattern-core/tests/query_all_values.rs
- [x] T023 [P] [US2] Write unit test for all_values with atomic pattern where not all values match in crates/pattern-core/tests/query_all_values.rs
- [x] T024 [P] [US2] Write unit test for all_values with empty pattern (vacuous truth) in crates/pattern-core/tests/query_all_values.rs
- [x] T025 [P] [US2] Write unit test for all_values with nested pattern where all values match in crates/pattern-core/tests/query_all_values.rs
- [x] T026 [P] [US2] Write unit test for all_values with nested pattern where one value fails in crates/pattern-core/tests/query_all_values.rs
- [x] T027 [P] [US2] Write unit test for all_values with deeply nested pattern (100+ levels) in crates/pattern-core/tests/query_all_values.rs
- [x] T028 [P] [US2] Write unit test for all_values with large flat pattern (1000+ elements) in crates/pattern-core/tests/query_all_values.rs

### Implementation for User Story 2

- [x] T029 [US2] Implement all_values method in crates/pattern-core/src/pattern.rs using fold with AND combinator
- [x] T030 [US2] Add comprehensive documentation for all_values including vacuous truth explanation in crates/pattern-core/src/pattern.rs
- [x] T031 [US2] Export all_values in crates/pattern-core/src/lib.rs public API (already exported via Pattern type)
- [x] T032 [US2] Verify all US2 unit tests pass with cargo test query_all_values

### Property Tests for User Story 2

- [x] T033 [P] [US2] Create property test file crates/pattern-core/tests/query_all_values_property.rs (updated path to match project structure)
- [x] T034 [P] [US2] Implement property test: all_values(const true) always returns true in crates/pattern-core/tests/query_all_values_property.rs
- [x] T035 [P] [US2] Implement property test: all_values(const false) returns false for non-empty patterns in crates/pattern-core/tests/query_all_values_property.rs
- [x] T036 [P] [US2] Implement property test: all_values consistent with all() over values() in crates/pattern-core/tests/query_all_values_property.rs

### Complementarity Tests (any_value vs all_values)

- [x] T037 [P] [US2] Complementarity tests integrated into property test files (more efficient than separate file)
- [x] T038 [P] [US2] Implement property test: any_value(p) == !all_values(!p) in crates/pattern-core/tests/query_all_values_property.rs
- [x] T039 [P] [US2] Implement property test: all_values(p) == !any_value(!p) in crates/pattern-core/tests/query_all_values.rs (unit test)

### Performance Verification for User Story 2

- [x] T040 [US2] Add benchmark for all_values short-circuit behavior in crates/pattern-core/benches/query_benchmarks.rs
- [x] T041 [US2] Verify all_values meets <100ms target for 10,000 node patterns with cargo bench

**Checkpoint**: ✅ At this point, User Stories 1 AND 2 (any_value, all_values) should both work independently with verified complementarity

---

## Phase 5: User Story 3 - Filter Patterns by Predicate (Priority: P2)

**Goal**: Implement `filter` operation to extract all subpatterns (including root) that satisfy a given pattern predicate, returning references in pre-order traversal

**Independent Test**: Can call `pattern.filter(predicate)` on patterns with various structures and receive complete collections of matching pattern references. Order verified as pre-order traversal.

### Tests for User Story 3 (TDD Approach)

- [x] T042 [P] [US3] Create test file crates/pattern-core/tests/query_filter.rs with basic structure
- [x] T043 [P] [US3] Write unit test for filter with predicate matching atomic patterns only in crates/pattern-core/tests/query_filter.rs
- [x] T044 [P] [US3] Write unit test for filter with predicate matching root pattern in crates/pattern-core/tests/query_filter.rs
- [x] T045 [P] [US3] Write unit test for filter with predicate matching no patterns (empty result) in crates/pattern-core/tests/query_filter.rs
- [x] T046 [P] [US3] Write unit test for filter with predicate matching all patterns (const true) in crates/pattern-core/tests/query_filter.rs
- [x] T047 [P] [US3] Write unit test for filter with complex structural predicate (length > 0 && depth < 3) in crates/pattern-core/tests/query_filter.rs
- [x] T048 [P] [US3] Write unit test for filter with predicates combining structural and value properties in crates/pattern-core/tests/query_filter.rs
- [x] T049 [P] [US3] Write unit test verifying filter returns results in pre-order traversal order in crates/pattern-core/tests/query_filter.rs
- [x] T050 [P] [US3] Write unit test for filter with deeply nested pattern (100+ levels) in crates/pattern-core/tests/query_filter.rs
- [x] T051 [P] [US3] Write unit test for filter with large flat pattern (1000+ elements) in crates/pattern-core/tests/query_filter.rs

### Implementation for User Story 3

- [x] T052 [US3] Implement filter method with custom recursive implementation in crates/pattern-core/src/pattern.rs
- [x] T053 [US3] Add comprehensive documentation for filter including traversal order explanation in crates/pattern-core/src/pattern.rs
- [x] T054 [US3] Export filter in crates/pattern-core/src/lib.rs public API (already exported via Pattern type)
- [x] T055 [US3] Verify all US3 unit tests pass with cargo test query_filter

### Property Tests for User Story 3

- [x] T056 [P] [US3] Create property test file crates/pattern-core/tests/query_filter_property.rs (updated path to match project structure)
- [x] T057 [P] [US3] Implement property test: filter(const true) returns all subpatterns in crates/pattern-core/tests/query_filter_property.rs
- [x] T058 [P] [US3] Implement property test: filter(const false) returns empty vec in crates/pattern-core/tests/query_filter_property.rs
- [x] T059 [P] [US3] Implement property test: filter(predicate).len() <= size() in crates/pattern-core/tests/query_filter_property.rs

### Performance Verification for User Story 3

- [x] T060 [US3] Add benchmark for filter operation in crates/pattern-core/benches/query_benchmarks.rs
- [x] T061 [US3] Verify filter meets <200ms target for 10,000 node patterns with cargo bench

**Checkpoint**: ✅ At this point, all three new query operations (any_value, all_values, filter) should be fully functional and tested

---

## Phase 6: User Story 4 - Verify Existing Query Operations (Priority: P3)

**Goal**: Add comprehensive test coverage for existing structural query operations (length, size, depth, values) to ensure behavioral equivalence with Haskell reference implementation and prevent regressions

**Independent Test**: Existing operations have comprehensive test suites covering all edge cases. Cross-implementation equivalence verified against gram-hs outputs. Performance targets met.

### Comprehensive Tests for Existing Operations

- [x] T062 [P] [US4] Create test file crates/pattern-core/tests/query_existing.rs with tests for length operation
- [x] T063 [P] [US4] Write unit tests for length with atomic patterns (should return 0) in crates/pattern-core/tests/query_existing.rs
- [x] T064 [P] [US4] Write unit tests for length with patterns having 1, 2, many direct elements in crates/pattern-core/tests/query_existing.rs
- [x] T065 [P] [US4] Write unit tests for length verifying it only counts direct elements (not nested descendants) in crates/pattern-core/tests/query_existing.rs
- [x] T066 [P] [US4] Write unit tests for size with atomic patterns (should return 1) in crates/pattern-core/tests/query_existing.rs
- [x] T067 [P] [US4] Write unit tests for size with flat patterns (1 + direct element count) in crates/pattern-core/tests/query_existing.rs
- [x] T068 [P] [US4] Write unit tests for size with deeply nested patterns (correct total count) in crates/pattern-core/tests/query_existing.rs
- [x] T069 [P] [US4] Write unit tests for size with patterns having varying branch depths in crates/pattern-core/tests/query_existing.rs
- [x] T070 [P] [US4] Write unit tests for depth with atomic patterns (should return 0) in crates/pattern-core/tests/query_existing.rs
- [x] T071 [P] [US4] Write unit tests for depth with one level of nesting (should return 1) in crates/pattern-core/tests/query_existing.rs
- [x] T072 [P] [US4] Write unit tests for depth with deeply nested patterns (correct max depth) in crates/pattern-core/tests/query_existing.rs
- [x] T073 [P] [US4] Write unit tests for depth with patterns having branches of different depths (returns maximum) in crates/pattern-core/tests/query_existing.rs
- [x] T074 [P] [US4] Write unit tests for values with atomic patterns (single-element list) in crates/pattern-core/tests/query_existing.rs
- [x] T075 [P] [US4] Write unit tests for values with nested patterns (all values in pre-order) in crates/pattern-core/tests/query_existing.rs
- [x] T076 [P] [US4] Write unit tests for values verifying order consistency (parent first, then elements) in crates/pattern-core/tests/query_existing.rs
- [x] T077 [P] [US4] Write unit tests for values with duplicate values (should return all including duplicates) in crates/pattern-core/tests/query_existing.rs

### Cross-Implementation Equivalence Tests

- [ ] T078 [P] [US4] Create equivalence test file crates/pattern-core/tests/equivalence/query_functions.rs
- [ ] T079 [P] [US4] Port test case T001 from gram-hs CoreSpec.hs (any_value atomic pattern) to crates/pattern-core/tests/equivalence/query_functions.rs
- [ ] T080 [P] [US4] Port test case T002 from gram-hs CoreSpec.hs (any_value nested pattern) to crates/pattern-core/tests/equivalence/query_functions.rs
- [ ] T081 [P] [US4] Port test case T003 from gram-hs CoreSpec.hs (any_value no matches) to crates/pattern-core/tests/equivalence/query_functions.rs
- [ ] T082 [P] [US4] Port test case T004 from gram-hs CoreSpec.hs (all_values atomic pattern) to crates/pattern-core/tests/equivalence/query_functions.rs
- [ ] T083 [P] [US4] Port test case T007 from gram-hs CoreSpec.hs (all_values vacuous truth) to crates/pattern-core/tests/equivalence/query_functions.rs
- [ ] T084 [P] [US4] Port test case T008 from gram-hs CoreSpec.hs (deep nesting) to crates/pattern-core/tests/equivalence/query_functions.rs
- [ ] T085 [P] [US4] Port test case T019 from gram-hs CoreSpec.hs (filter matching some) to crates/pattern-core/tests/equivalence/query_functions.rs
- [ ] T086 [P] [US4] Port test case T020 from gram-hs CoreSpec.hs (filter matching root) to crates/pattern-core/tests/equivalence/query_functions.rs
- [ ] T087 [P] [US4] Port test case T021 from gram-hs CoreSpec.hs (filter matching none) to crates/pattern-core/tests/equivalence/query_functions.rs
- [ ] T088 [P] [US4] Port test case T063 from gram-hs CoreSpec.hs (any/all with fmap) to crates/pattern-core/tests/equivalence/query_functions.rs
- [ ] T089 [P] [US4] Port test case T064 from gram-hs CoreSpec.hs (any/all consistency with toList) to crates/pattern-core/tests/equivalence/query_functions.rs
- [ ] T090 [P] [US4] Port test case T066 from gram-hs CoreSpec.hs (large patterns 1000+ elements) to crates/pattern-core/tests/equivalence/query_functions.rs

### Performance Benchmarks for Existing Operations

- [ ] T091 [P] [US4] Add benchmarks for length operation in crates/pattern-core/benches/query_benchmarks.rs
- [ ] T092 [P] [US4] Add benchmarks for size operation with large patterns (10,000+ nodes) in crates/pattern-core/benches/query_benchmarks.rs
- [ ] T093 [P] [US4] Add benchmarks for depth operation with very deep patterns (100+ levels) in crates/pattern-core/benches/query_benchmarks.rs
- [ ] T094 [P] [US4] Add benchmarks for values operation with large patterns (10,000+ nodes) in crates/pattern-core/benches/query_benchmarks.rs
- [ ] T095 [US4] Run cargo bench and verify all existing operations meet performance targets

**Checkpoint**: All query operations (new and existing) have comprehensive test coverage and verified behavioral equivalence

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, integration, and final validation

- [x] T096 [P] Update module-level documentation in crates/pattern-core/src/pattern.rs to list new query operations
- [x] T097 [P] Add "Query Functions" section to crates/pattern-core/src/pattern.rs module docs
- [x] T098 [P] Verify WASM compilation with cargo build --target wasm32-unknown-unknown
- [x] T099 [P] Run cargo doc --open and verify all new methods are documented (comprehensive doc comments added)
- [x] T100 [P] Run cargo clippy and fix any warnings in crates/pattern-core/src/pattern.rs (no warnings)
- [x] T101 [P] Run cargo fmt and ensure code follows formatting standards (formatted)
- [x] T102 Validate quickstart.md examples by running code snippets from specs/011-basic-query-functions/quickstart.md (examples verified in tests)
- [x] T103 Run full test suite with cargo test and verify all 75+ tests pass (117 tests pass)
- [x] T104 Run full benchmark suite with cargo bench and verify all performance targets met (all targets met)
- [ ] T105 Update CHANGELOG.md with new query operations (any_value, all_values, filter) - DEFERRED (no CHANGELOG.md in repo)
- [x] T106 Review contracts/type-signatures.md and verify implementation matches all behavioral contracts (verified during implementation)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: N/A - all infrastructure exists
- **User Stories (Phase 3-6)**: Can proceed immediately after Setup
  - User Story 1 (any_value): Independent - can start after Setup
  - User Story 2 (all_values): Independent - can run parallel with US1
  - User Story 3 (filter): Independent - can run parallel with US1, US2
  - User Story 4 (verify existing): Independent - can run parallel with US1-3
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: No dependencies - can start immediately after Setup
- **User Story 2 (P1)**: No dependencies on US1 - can run in parallel
  - Note: Complementarity tests (T037-T039) logically depend on both US1 and US2 implementations
- **User Story 3 (P2)**: No dependencies on US1/US2 - can run in parallel
- **User Story 4 (P3)**: No dependencies on US1-3 - tests existing operations

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Tests (T004-T010) before implementation (T011-T014) for US1
- Tests (T021-T028) before implementation (T029-T032) for US2
- Tests (T042-T051) before implementation (T052-T055) for US3
- Property tests can run parallel with unit tests within each story
- Benchmarks after implementation is complete

### Parallel Opportunities

- All Setup tasks (T001-T003) can run in parallel
- **After Setup**, all 4 user stories can start in parallel (if team capacity allows)
- Within US1: Tests T004-T010 can all run in parallel, property tests T015-T018 can run in parallel
- Within US2: Tests T021-T028 can all run in parallel, property tests T033-T036 can run in parallel
- Within US3: Tests T042-T051 can all run in parallel, property tests T056-T059 can run in parallel
- Within US4: All tests T062-T095 can run in parallel (different test categories)
- All Polish tasks T096-T101 can run in parallel

---

## Parallel Example: User Story 1

```bash
# Write all unit tests for US1 in parallel (TDD - tests FIRST):
Task T004: "Create test file crates/pattern-core/tests/query_any_value.rs"
Task T005: "Unit test for any_value with atomic pattern containing matching value"
Task T006: "Unit test for any_value with atomic pattern containing non-matching value"
Task T007: "Unit test for any_value with nested pattern at different levels"
Task T008: "Unit test for any_value with no matching values"
Task T009: "Unit test for any_value with deeply nested pattern (100+ levels)"
Task T010: "Unit test for any_value with large flat pattern (1000+ elements)"

# After tests are written and failing, implement:
Task T011: "Implement any_value method in crates/pattern-core/src/pattern.rs"
Task T012: "Add comprehensive documentation for any_value"
Task T013: "Export any_value in public API"

# Then property tests can run in parallel:
Task T016: "Property test: any_value(const true) always returns true"
Task T017: "Property test: any_value(const false) always returns false"
Task T018: "Property test: any_value consistent with any() over values()"
```

---

## Parallel Example: All User Stories

```bash
# After Setup (Phase 1), all 4 user stories can proceed in parallel:

Developer A works on User Story 1 (any_value):
  - Phase 3 tasks T004-T020

Developer B works on User Story 2 (all_values):
  - Phase 4 tasks T021-T041

Developer C works on User Story 3 (filter):
  - Phase 5 tasks T042-T061

Developer D works on User Story 4 (existing operations):
  - Phase 6 tasks T062-T095

# Stories complete independently and can be validated separately
```

---

## Implementation Strategy

### MVP First (User Story 1 Only - any_value)

1. Complete Phase 1: Setup (T001-T003)
2. Complete Phase 3: User Story 1 - any_value (T004-T020)
3. **STOP and VALIDATE**: Test any_value independently
4. Demo/review if ready

**Benefits**:
- Fastest path to demonstrable value
- Validates approach before implementing other operations
- Provides reference for implementing all_values (similar pattern)

### Incremental Delivery

1. Setup complete → Environment ready
2. Add US1 (any_value) → Test independently → Demo (MVP!)
3. Add US2 (all_values) → Test independently + complementarity → Demo
4. Add US3 (filter) → Test independently → Demo
5. Add US4 (comprehensive tests) → Verify all operations → Final demo
6. Polish → Documentation complete → Release

Each story adds value without breaking previous stories.

### Parallel Team Strategy

With 4 developers available:

1. All: Complete Setup together (T001-T003) - ~1 hour
2. Once Setup done, split work:
   - Developer A: User Story 1 (any_value) - T004-T020
   - Developer B: User Story 2 (all_values) - T021-T041
   - Developer C: User Story 3 (filter) - T042-T061
   - Developer D: User Story 4 (tests) - T062-T095
3. All stories complete and integrate independently
4. All: Polish together (T096-T106)

**Estimated Timeline** (with parallel execution):
- Setup: 1-2 hours
- User Stories (parallel): 2-3 days
- Polish: 1-2 hours
- **Total: 2-3 days** (vs 5-6 days sequential)

---

## Task Summary

**Total Tasks**: 106

**Task Count by Phase**:
- Phase 1 (Setup): 3 tasks
- Phase 2 (Foundational): 0 tasks (infrastructure exists)
- Phase 3 (US1 - any_value): 17 tasks
- Phase 4 (US2 - all_values): 21 tasks
- Phase 5 (US3 - filter): 20 tasks
- Phase 6 (US4 - verify existing): 34 tasks
- Phase 7 (Polish): 11 tasks

**Parallel Opportunities**: 89 tasks marked [P] can run in parallel within their phase

**Independent Test Criteria**:
- US1: Can call any_value() and get correct boolean results with short-circuit behavior
- US2: Can call all_values() and get correct boolean results with vacuous truth
- US3: Can call filter() and get correct pattern reference collections in pre-order
- US4: All existing operations have comprehensive test coverage and meet performance targets

**MVP Scope**: User Story 1 only (17 tasks) - delivers any_value operation as proof of concept

**Format Validation**: ✅ All 106 tasks follow checklist format (checkbox, ID, optional [P], optional [Story], description with file path)

---

## Notes

- [P] tasks = different files or test categories, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable
- TDD approach: Write tests FIRST, ensure they FAIL, then implement
- Tests are MANDATORY for this feature (not optional)
- Commit after each logical group of tasks
- Stop at any checkpoint to validate story independently
- Reference Haskell implementation at ../gram-hs/libs/pattern/src/Pattern/Core.hs
- Cross-implementation testing uses gram-hs test cases as source of truth

