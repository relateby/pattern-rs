# Tasks: Foldable Instance for Pattern

**Input**: Design documents from `/specs/009-foldable-instance/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/type-signatures.md  
**Feature Branch**: `009-foldable-instance`

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

This is a Rust library crate in a multi-crate workspace:
- Implementation: `crates/pattern-core/src/pattern.rs`
- Tests: `crates/pattern-core/tests/`
- Documentation: In-code doc comments and crate-level docs

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Verify prerequisites are in place

**Status**: ✅ Complete - Pattern type exists from features 004, 005, 008

No additional setup tasks required. Pattern<V> type already exists with:
- Core data structure (feature 004)
- Construction functions (feature 005) 
- Map method (feature 008)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**Status**: ✅ Complete - All prerequisites met

No foundational tasks required. Dependencies complete:
- Feature 004: Pattern Data Structure ✅
- Feature 005: Basic Pattern Type ✅
- Feature 008: Functor Instance ✅

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Aggregate Pattern Values into Single Result (Priority: P1) 🎯 MVP

**Goal**: Implement core `fold` operation that processes all pattern values into a single result, maintaining depth-first root-first order

**Independent Test**: Create patterns with various structures, apply fold operations (sum, concatenation, counting), verify all values processed in correct order (root first, then elements). Verify atomic patterns, flat patterns, and nested patterns all work correctly.

### Implementation for User Story 1

- [ ] T001 [US1] Implement `fold<B, F>(&self, init: B, f: F) -> B` public method in crates/pattern-core/src/pattern.rs
- [ ] T002 [US1] Implement `fold_with<B, F>(&self, acc: B, f: &F) -> B` internal helper in crates/pattern-core/src/pattern.rs
- [ ] T003 [US1] Add comprehensive doc comments with examples for fold method in crates/pattern-core/src/pattern.rs
- [ ] T004 [P] [US1] Create test file crates/pattern-core/tests/foldable_basic.rs for basic fold tests
- [ ] T005 [P] [US1] Port atomic pattern fold tests from gram-hs in crates/pattern-core/tests/foldable_basic.rs
- [ ] T006 [P] [US1] Port flat pattern fold tests (one level, multiple elements) from gram-hs in crates/pattern-core/tests/foldable_basic.rs
- [ ] T007 [P] [US1] Port nested pattern fold tests (multiple levels) from gram-hs in crates/pattern-core/tests/foldable_basic.rs
- [ ] T008 [US1] Implement order verification test with string concatenation in crates/pattern-core/tests/foldable_basic.rs
- [ ] T009 [US1] Implement sum test (root + elements) in crates/pattern-core/tests/foldable_basic.rs
- [ ] T010 [US1] Implement count test (verify count equals size) in crates/pattern-core/tests/foldable_basic.rs
- [ ] T011 [US1] Run tests and verify all US1 acceptance scenarios pass

**Checkpoint**: At this point, core fold operation is fully functional - can aggregate any pattern to single value

---

## Phase 4: User Story 2 - Convert Pattern Values to Collections (Priority: P1) 🎯 MVP

**Goal**: Implement `values()` convenience method that extracts all pattern values into a Vec, enabling interoperability with standard library operations

**Independent Test**: Create patterns (atomic, flat, nested), convert to Vec, verify all values present in correct depth-first order. Verify length equals pattern.size(). Test integration with standard Iterator methods.

### Implementation for User Story 2

- [ ] T012 [US2] Implement `values(&self) -> Vec<&V>` method in crates/pattern-core/src/pattern.rs (uses fold internally)
- [ ] T013 [US2] Add comprehensive doc comments with examples for values method in crates/pattern-core/src/pattern.rs
- [ ] T014 [P] [US2] Create test file crates/pattern-core/tests/foldable_collections.rs for collection conversion tests
- [ ] T015 [P] [US2] Implement atomic pattern to Vec test in crates/pattern-core/tests/foldable_collections.rs
- [ ] T016 [P] [US2] Implement flat pattern to Vec test with order verification in crates/pattern-core/tests/foldable_collections.rs
- [ ] T017 [P] [US2] Implement nested pattern to Vec test in crates/pattern-core/tests/foldable_collections.rs
- [ ] T018 [US2] Implement values length equals size test in crates/pattern-core/tests/foldable_collections.rs
- [ ] T019 [US2] Implement integration test with Iterator operations (filter, map on returned Vec) in crates/pattern-core/tests/foldable_collections.rs
- [ ] T020 [US2] Run tests and verify all US2 acceptance scenarios pass

**Checkpoint**: At this point, both fold and values methods work - patterns can be aggregated and converted to collections

---

## Phase 5: User Story 3 - Build Custom Aggregations (Priority: P2)

**Goal**: Verify fold supports custom aggregation logic with various accumulator types and folding functions (counting, max/min, building maps/sets, domain-specific logic)

**Independent Test**: Define custom folding functions for various use cases (counting, finding extrema, building data structures, validation). Verify they process all values correctly with proper accumulator threading.

### Implementation for User Story 3

- [ ] T021 [P] [US3] Create test file crates/pattern-core/tests/foldable_custom.rs for custom aggregation tests
- [ ] T022 [P] [US3] Implement counting aggregation test in crates/pattern-core/tests/foldable_custom.rs
- [ ] T023 [P] [US3] Implement max/min finding test in crates/pattern-core/tests/foldable_custom.rs
- [ ] T024 [P] [US3] Implement HashMap building test in crates/pattern-core/tests/foldable_custom.rs
- [ ] T025 [P] [US3] Implement HashSet building test in crates/pattern-core/tests/foldable_custom.rs
- [ ] T026 [P] [US3] Implement boolean validation (all/any) test in crates/pattern-core/tests/foldable_custom.rs
- [ ] T027 [P] [US3] Implement type transformation test (fold string pattern to usize) in crates/pattern-core/tests/foldable_custom.rs
- [ ] T028 [US3] Implement custom struct accumulator test in crates/pattern-core/tests/foldable_custom.rs
- [ ] T029 [US3] Run tests and verify all US3 acceptance scenarios pass

**Checkpoint**: At this point, fold supports diverse custom aggregations - verified with multiple accumulator types

---

## Phase 6: User Story 4 - Chain Foldable Operations with Other Functional Patterns (Priority: P3)

**Goal**: Verify fold composes seamlessly with map (functor) and other Pattern operations to enable functional programming pipelines

**Independent Test**: Chain map and fold operations, verify composition works correctly. Test multiple folds on same pattern. Verify pattern preservation after fold (can reuse).

### Implementation for User Story 4

- [ ] T030 [P] [US4] Create test file crates/pattern-core/tests/foldable_integration.rs for integration tests
- [ ] T031 [P] [US4] Implement map-then-fold composition test in crates/pattern-core/tests/foldable_integration.rs
- [ ] T032 [P] [US4] Implement fold-multiple-times test (pattern reuse) in crates/pattern-core/tests/foldable_integration.rs
- [ ] T033 [P] [US4] Implement pattern-unchanged-after-fold test in crates/pattern-core/tests/foldable_integration.rs
- [ ] T034 [US4] Implement complex pipeline test (map, fold, compare) in crates/pattern-core/tests/foldable_integration.rs
- [ ] T035 [US4] Run tests and verify all US4 acceptance scenarios pass

**Checkpoint**: All user stories now complete - fold fully integrated with Pattern API

---

## Phase 7: Property-Based Testing & Verification

**Purpose**: Comprehensive property-based tests and gram-hs test parity verification

- [ ] T036 [P] Create property-based test file crates/pattern-core/tests/foldable_properties.rs
- [ ] T037 [P] Implement arbitrary pattern generator for proptest in crates/pattern-core/tests/foldable_properties.rs
- [ ] T038 [P] Implement fold count equals size property test (100+ cases) in crates/pattern-core/tests/foldable_properties.rs
- [ ] T039 [P] Implement values length equals size property test (100+ cases) in crates/pattern-core/tests/foldable_properties.rs
- [ ] T040 [P] Implement pattern unchanged property test (100+ cases) in crates/pattern-core/tests/foldable_properties.rs
- [ ] T041 [P] Implement fold deterministic property test (100+ cases) in crates/pattern-core/tests/foldable_properties.rs
- [ ] T042 Port remaining gram-hs foldable tests from ../gram-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs lines 1054-1499
- [ ] T043 Run full test suite and verify 100% gram-hs test parity (SC-005)

---

## Phase 8: Performance & Scale Testing

**Purpose**: Verify performance targets and scale requirements

- [ ] T044 [P] Create benchmark file benches/fold_benchmarks.rs if needed
- [ ] T045 [P] Implement large pattern benchmark (1000 nodes) in benches/fold_benchmarks.rs
- [ ] T046 [P] Implement deep nesting test (100 levels) in crates/pattern-core/tests/foldable_scale.rs
- [ ] T047 [P] Implement wide pattern test (1000 siblings) in crates/pattern-core/tests/foldable_scale.rs
- [ ] T048 Run benchmarks and verify 1000 nodes <10ms (SC-002)
- [ ] T049 Run deep nesting test and verify no stack overflow (SC-003)
- [ ] T050 Profile memory usage for 10,000 element pattern and verify <100MB (SC-009)

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, WASM verification, and final validation

- [ ] T051 [P] Update crate-level documentation in crates/pattern-core/src/lib.rs with fold examples
- [ ] T052 [P] Add fold examples to crate README in crates/pattern-core/README.md
- [ ] T053 [P] Verify WASM compilation with cargo build --target wasm32-unknown-unknown
- [ ] T054 [P] Add fold operations to quickstart examples if not already present
- [ ] T055 Update TODO.md to mark feature 009-foldable-instance as complete
- [ ] T056 Create verification summary document confirming all success criteria (SC-001 through SC-009)
- [ ] T057 Run full test suite one final time: cargo test --package pattern-core
- [ ] T058 Run clippy and address any warnings: cargo clippy --package pattern-core
- [ ] T059 Run rustfmt: cargo fmt --package pattern-core

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: ✅ Complete (no work needed)
- **Foundational (Phase 2)**: ✅ Complete (no work needed)
- **User Stories (Phases 3-6)**: Can proceed immediately
  - Phase 3 (US1): No dependencies - can start immediately
  - Phase 4 (US2): Depends on US1 (T001-T002 must complete first - values uses fold)
  - Phase 5 (US3): Depends on US1 (tests fold with custom functions)
  - Phase 6 (US4): Depends on US1 (tests fold integration)
- **Property Tests (Phase 7)**: Depends on Phases 3-4 completion
- **Performance (Phase 8)**: Depends on Phase 3 completion
- **Polish (Phase 9)**: Depends on all user stories (Phases 3-6) being complete

### User Story Dependencies

- **User Story 1 (P1)**: ✅ Can start immediately - core fold implementation
- **User Story 2 (P1)**: Depends on US1 completion (T001-T002) - values() uses fold internally
- **User Story 3 (P2)**: Depends on US1 completion - tests custom uses of fold
- **User Story 4 (P3)**: Depends on US1 completion - tests fold integration

### Critical Path

1. **T001-T002**: Implement fold and fold_with (blocks everything else)
2. **T003-T011**: Complete US1 tests
3. **T012**: Implement values() (blocks US2)
4. **US2, US3, US4 can proceed in parallel** after US1 complete
5. **Property tests, performance, polish** can proceed after core implementation

### Parallel Opportunities

**Within US1** (after T001-T002 complete):
- T004-T007: All test files can be created in parallel
- T008-T010: All tests can be written in parallel

**Within US2** (after T012-T013 complete):
- T015-T019: All tests can be written in parallel

**Within US3** (after US1 complete):
- T022-T027: All custom aggregation tests can be written in parallel

**Within US4** (after US1 complete):
- T031-T034: All integration tests can be written in parallel

**Across User Stories** (after T001-T002 complete):
- US3 tests (T021-T029) can run in parallel with US2 implementation
- US4 tests (T030-T035) can run in parallel with US2 and US3

**Property & Performance** (after US1-US2 complete):
- T036-T041: All property tests can be written in parallel
- T044-T047: All benchmarks/scale tests can be written in parallel

**Polish** (after all user stories complete):
- T051-T054: All documentation tasks can run in parallel

---

## Parallel Example: User Story 1

```bash
# After T001-T002 (fold implementation) completes:

# Launch all test creation tasks together:
Task T004: "Create test file crates/pattern-core/tests/foldable_basic.rs"
Task T005: "Port atomic pattern fold tests from gram-hs" 
Task T006: "Port flat pattern fold tests from gram-hs"
Task T007: "Port nested pattern fold tests from gram-hs"

# Then launch all test implementation tasks together:
Task T008: "Implement order verification test with string concatenation"
Task T009: "Implement sum test (root + elements)"
Task T010: "Implement count test (verify count equals size)"
```

---

## Parallel Example: After US1 Complete

```bash
# US2, US3, US4 can all start in parallel:

# Developer A: User Story 2 (Collection Conversion)
Task T012: Implement values() method
Task T015-T019: Collection conversion tests

# Developer B: User Story 3 (Custom Aggregations)  
Task T022-T028: Custom aggregation tests

# Developer C: User Story 4 (Integration)
Task T031-T034: Integration tests with map
```

---

## Implementation Strategy

### MVP First (User Stories 1 & 2 Only)

Both US1 and US2 are P1 priority and form the minimum viable feature:

1. Complete Phase 3: User Story 1 (core fold)
2. Complete Phase 4: User Story 2 (values convenience)
3. **STOP and VALIDATE**: Test both US1 and US2 independently
4. Mark feature as MVP-complete (fold and values both work)

### Incremental Delivery

1. US1 + US2 → Test independently → **MVP Ready** ✅
2. Add US3 → Test custom aggregations → **Extended Capabilities**
3. Add US4 → Test integration → **Full Feature Complete**
4. Add property tests → **Comprehensive Verification**
5. Add performance tests → **Production Ready**
6. Polish & docs → **Release Ready**

### Parallel Team Strategy

With multiple developers:

1. **Phase 3 (US1)**: Team works together on core implementation
   - One dev: fold/fold_with implementation (T001-T003)
   - Other devs: Prepare test infrastructure in parallel (T004)
   - All devs: Write tests in parallel after implementation (T005-T010)

2. **Phase 4-6**: Stories can proceed in parallel
   - Developer A: US2 (collection conversion)
   - Developer B: US3 (custom aggregations) 
   - Developer C: US4 (integration)

3. **Phase 7-8**: Verification in parallel
   - Developer A: Property tests
   - Developer B: Performance tests
   - Developer C: gram-hs test parity

---

## Notes

- [P] tasks = different files or independent tests, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story independently testable once complete
- US1 is foundation for all others (fold implementation)
- US2 depends on US1 (values uses fold)
- US3 and US4 test US1 but don't modify implementation
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- **Total Tasks**: 59
- **MVP Tasks** (US1 + US2): T001-T020 (20 tasks)
- **Full Feature Tasks** (US1-US4): T001-T035 (35 tasks)
- **Property/Performance Tasks**: T036-T050 (15 tasks)
- **Polish Tasks**: T051-T059 (9 tasks)

---

## Success Criteria Checklist

From spec.md - verify these after implementation:

- [ ] **SC-001**: Fold operations correctly process all values in patterns with 1000 nodes (verified by T048)
- [ ] **SC-002**: Fold operations complete on patterns with 1000 nodes in under 10ms (verified by T048)
- [ ] **SC-003**: Fold operations complete on patterns with 100 nesting levels without stack overflow (verified by T049)
- [ ] **SC-004**: Converting patterns to collections preserves exact order (verified by T016, T017)
- [ ] **SC-005**: 100% of existing gram-hs foldable tests ported and pass (verified by T042, T043)
- [ ] **SC-006**: Foldable implementation compiles for WASM target (verified by T053)
- [ ] **SC-007**: Custom folding functions work correctly (verified by T022-T028)
- [ ] **SC-008**: Fold operations use constant stack space or handle deep recursion (verified by T049)
- [ ] **SC-009**: Pattern structures with 10,000 elements can be folded <100MB (verified by T050)

