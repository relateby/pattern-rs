# Tasks: Pattern Combination Operations

**Input**: Design documents from `/specs/013-semigroup-instance/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Property-based testing is explicitly required for associativity verification (User Story 2)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

This is a library crate feature. All implementation goes in:
- **Implementation**: `crates/pattern-core/src/`
- **Tests**: `crates/pattern-core/tests/`
- **Benchmarks**: `crates/pattern-core/benches/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Verify project structure is ready for implementation

- [x] T001 Verify cargo workspace structure for pattern-core crate
- [x] T002 [P] Verify proptest dependency is available in Cargo.toml for property-based testing
- [x] T003 [P] Review existing Pattern<V> implementation in crates/pattern-core/src/pattern.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core trait definition that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Define Combinable trait in crates/pattern-core/src/lib.rs with associativity documentation
- [x] T005 Add Combinable trait to public exports in crates/pattern-core/src/lib.rs
- [x] T006 [P] Implement Combinable for String in crates/pattern-core/src/lib.rs
- [x] T007 [P] Implement Combinable for Vec<T> in crates/pattern-core/src/lib.rs
- [x] T008 [P] Implement Combinable for unit type () in crates/pattern-core/src/lib.rs

**Checkpoint**: ✅ Combinable trait defined and standard implementations complete - user story implementation can now begin

---

## Phase 3: User Story 1 - Combine Two Patterns (Priority: P1) 🎯 MVP

**Goal**: Implement core pattern combination operation that merges two patterns by combining values and concatenating elements

**Independent Test**: Can be fully tested by combining two patterns and verifying the resulting pattern structure has combined value and concatenated elements

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T009 [P] [US1] Create basic combination test file crates/pattern-core/tests/semigroup_basic.rs
- [x] T010 [P] [US1] Add test for combining atomic patterns (no elements) in crates/pattern-core/tests/semigroup_basic.rs
- [x] T011 [P] [US1] Add test for combining patterns with elements in crates/pattern-core/tests/semigroup_basic.rs
- [x] T012 [P] [US1] Add test for combining mixed structures (one atomic, one with elements) in crates/pattern-core/tests/semigroup_basic.rs
- [x] T013 [P] [US1] Add test for self-combination (pattern combined with itself) in crates/pattern-core/tests/semigroup_basic.rs
- [x] T014 [P] [US1] Add test for deep nesting preservation (100+ levels) in crates/pattern-core/tests/semigroup_basic.rs
- [x] T015 [P] [US1] Add test for wide patterns (1000+ elements) in crates/pattern-core/tests/semigroup_basic.rs

### Implementation for User Story 1

- [x] T016 [US1] Implement Pattern::combine() method in crates/pattern-core/src/pattern.rs with V: Combinable constraint
- [x] T017 [US1] Add combine() to Pattern's module documentation in crates/pattern-core/src/pattern.rs
- [x] T018 [US1] Verify all User Story 1 tests now pass

**Checkpoint**: ✅ User Story 1 is fully functional - two patterns can be combined with correct value combination and element concatenation

---

## Phase 4: User Story 2 - Verify Associativity Law (Priority: P1)

**Goal**: Ensure pattern combination is associative through comprehensive property-based testing: (a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)

**Independent Test**: Property-based tests with 10,000+ randomly generated pattern triples verify associativity holds for all structures

### Tests for User Story 2

> **NOTE: These ARE the implementation for this story - property tests verify the mathematical law**

- [x] T019 [P] [US2] Create property test file crates/pattern-core/tests/semigroup_property.rs
- [x] T020 [US2] Add proptest strategy for generating random Pattern<String> in crates/pattern-core/tests/semigroup_property.rs
- [x] T021 [P] [US2] Add associativity property test for atomic patterns in crates/pattern-core/tests/semigroup_property.rs
- [x] T022 [P] [US2] Add associativity property test for patterns with varying depths in crates/pattern-core/tests/semigroup_property.rs
- [x] T023 [P] [US2] Add associativity property test for patterns with varying element counts in crates/pattern-core/tests/semigroup_property.rs
- [x] T024 [P] [US2] Add associativity property test for deeply nested patterns (50-100 levels) in crates/pattern-core/tests/semigroup_property.rs
- [x] T025 [P] [US2] Add associativity property test for wide patterns (100-1000 elements) in crates/pattern-core/tests/semigroup_property.rs
- [x] T026 [P] [US2] Add property test for element preservation (result has all elements from both inputs) in crates/pattern-core/tests/semigroup_property.rs
- [x] T027 [P] [US2] Add property test for element order (left elements before right elements) in crates/pattern-core/tests/semigroup_property.rs
- [x] T028 [P] [US2] Add property test for value combination delegation in crates/pattern-core/tests/semigroup_property.rs

### Verification for User Story 2

- [x] T029 [US2] Run property tests with 10,000+ cases and verify 100% success rate
- [x] T030 [US2] Add property test documentation explaining associativity law in crates/pattern-core/tests/semigroup_property.rs

**Checkpoint**: At this point, associativity is mathematically verified through comprehensive property-based testing

---

## Phase 5: User Story 3 - Combine Multiple Patterns in Sequence (Priority: P2)

**Goal**: Enable combining multiple patterns using iterator fold/reduce operations

**Independent Test**: Can fold/reduce a collection of patterns and verify result matches sequential pairwise combination

### Tests for User Story 3

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T031 [P] [US3] Create integration test file crates/pattern-core/tests/semigroup_integration.rs
- [x] T032 [P] [US3] Add test for fold/reduce with 4 patterns in crates/pattern-core/tests/semigroup_integration.rs
- [x] T033 [P] [US3] Add test for fold/reduce with single pattern in crates/pattern-core/tests/semigroup_integration.rs
- [x] T034 [P] [US3] Add test for fold/reduce with varying structures in crates/pattern-core/tests/semigroup_integration.rs
- [x] T035 [P] [US3] Add test for empty collection behavior in crates/pattern-core/tests/semigroup_integration.rs
- [x] T036 [P] [US3] Add test for combining 100 patterns in sequence in crates/pattern-core/tests/semigroup_integration.rs

### Implementation for User Story 3

- [x] T037 [US3] Add documentation examples showing Iterator::reduce usage in crates/pattern-core/src/pattern.rs
- [x] T038 [US3] Add documentation examples showing Iterator::fold usage in crates/pattern-core/src/pattern.rs
- [x] T039 [US3] Verify all User Story 3 integration tests pass

**Checkpoint**: ✅ All user stories are now complete - patterns can be combined pairwise and in sequences using standard iterator methods

---

## Phase 6: Gram-HS Equivalence Verification

**Purpose**: Verify behavioral equivalence with gram-hs reference implementation

- [x] T040 [P] Create equivalence test file crates/pattern-core/tests/semigroup_equivalence.rs
- [x] T041 [P] Research gram-hs Semigroup instance in ../gram-hs/libs/pattern/src/Pattern/Core.hs to identify test cases
- [x] T042 Port test cases from gram-hs test suite to crates/pattern-core/tests/semigroup_equivalence.rs
- [x] T043 Verify all ported tests pass with identical behavior
- [x] T044 Document any intentional deviations from gram-hs in IMPLEMENTATION_NOTES.md

---

## Phase 7: Performance Validation

**Purpose**: Verify performance targets are met

- [x] T045 [P] Create benchmark file crates/pattern-core/benches/semigroup_benchmarks.rs
- [x] T046 [P] Add benchmark for combining 1000-element patterns in crates/pattern-core/benches/semigroup_benchmarks.rs
- [x] T047 [P] Add benchmark for combining deep patterns (100+ levels) in crates/pattern-core/benches/semigroup_benchmarks.rs
- [x] T048 [P] Add benchmark for folding 100 patterns in crates/pattern-core/benches/semigroup_benchmarks.rs
- [x] T049 Run benchmarks and verify performance targets: <1ms for 1000 elements, <100ms for 100 pattern fold
- [x] T050 Document performance characteristics in crates/pattern-core/src/pattern.rs combine() method docs

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, examples, and final verification

- [x] T051 [P] Update crates/pattern-core/README.md with combination operation examples
- [x] T052 [P] Add combination examples to main library documentation in crates/pattern-core/src/lib.rs
- [x] T053 [P] Verify WASM compatibility: cargo build --target wasm32-unknown-unknown
- [x] T054 [P] Run cargo clippy and address any warnings in pattern.rs or lib.rs
- [x] T055 [P] Run cargo fmt to ensure consistent formatting
- [x] T056 Verify all tests pass: cargo test in pattern-core crate
- [x] T057 Run full test suite across entire workspace: cargo test --workspace
- [x] T058 Generate and review documentation: cargo doc --no-deps --open
- [x] T059 Validate quickstart.md examples match actual API
- [x] T060 Update TODO.md to mark feature 013-semigroup-instance as complete

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-5)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 US1 → P1 US2 → P2 US3)
- **Equivalence (Phase 6)**: Depends on User Stories 1-2 being complete
- **Performance (Phase 7)**: Depends on User Stories 1-3 being complete
- **Polish (Phase 8)**: Depends on all previous phases

### User Story Dependencies

- **User Story 1 (P1)**: Depends on Foundational (Phase 2) - Core combination operation
- **User Story 2 (P1)**: Depends on User Story 1 - Verifies US1 is associative
- **User Story 3 (P2)**: Depends on User Story 1 - Uses combination in fold/reduce

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Implementation makes tests pass
- Story complete when all tests pass and checkpoint criteria met

### Parallel Opportunities

- **Phase 1**: All 3 tasks can run in parallel (different focus areas)
- **Phase 2**: Tasks T006, T007, T008 (standard Combinable implementations) can run in parallel
- **User Story 1 Tests**: T009-T015 can all be written in parallel
- **User Story 2 Tests**: T021-T028 can all be written in parallel
- **User Story 3 Tests**: T031-T036 can all be written in parallel
- **Phase 6**: T040 and T041 can run in parallel
- **Phase 7**: All benchmarks (T045-T048) can run in parallel
- **Phase 8**: Documentation tasks (T051, T052, T054, T055) can run in parallel

---

## Parallel Example: User Story 1

```bash
# Write all tests for User Story 1 together (T009-T015):
Task: "Create basic combination test file"
Task: "Add test for combining atomic patterns"
Task: "Add test for combining patterns with elements"
Task: "Add test for combining mixed structures"
Task: "Add test for self-combination"
Task: "Add test for deep nesting preservation"
Task: "Add test for wide patterns"

# Verify all tests FAIL before proceeding to implementation
```

---

## Parallel Example: User Story 2

```bash
# Write all property tests together (T021-T028):
Task: "Add associativity property test for atomic patterns"
Task: "Add associativity property test for varying depths"
Task: "Add associativity property test for varying element counts"
Task: "Add associativity property test for deeply nested patterns"
Task: "Add associativity property test for wide patterns"
Task: "Add property test for element preservation"
Task: "Add property test for element order"
Task: "Add property test for value combination delegation"

# These tests verify the mathematical properties of the implementation
```

---

## Implementation Strategy

### MVP First (User Stories 1 & 2 Only)

1. Complete Phase 1: Setup (verify structure)
2. Complete Phase 2: Foundational (Combinable trait + implementations)
3. Complete Phase 3: User Story 1 (core combination operation)
4. Complete Phase 4: User Story 2 (associativity verification)
5. **STOP and VALIDATE**: Combination works and is proven associative
6. This is the minimal viable feature

### Incremental Delivery

1. **MVP**: Setup + Foundational + US1 + US2 → Combination proven correct
2. **Enhanced**: Add US3 → Enables fold/reduce patterns
3. **Verified**: Add Phase 6 → Gram-hs equivalence confirmed
4. **Optimized**: Add Phase 7 → Performance validated
5. **Polished**: Add Phase 8 → Documentation complete

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 tests + implementation
   - Developer B: User Story 2 property tests (waits for US1 completion)
   - Developer C: User Story 3 tests (can write in parallel)
3. Stories complete in order due to dependencies

---

## Task Count Summary

- **Total Tasks**: 60
- **Phase 1 (Setup)**: 3 tasks
- **Phase 2 (Foundational)**: 5 tasks
- **Phase 3 (User Story 1)**: 10 tasks (7 tests + 3 implementation)
- **Phase 4 (User Story 2)**: 12 tasks (10 property tests + 2 verification)
- **Phase 5 (User Story 3)**: 9 tasks (6 tests + 3 implementation)
- **Phase 6 (Equivalence)**: 5 tasks
- **Phase 7 (Performance)**: 6 tasks
- **Phase 8 (Polish)**: 10 tasks

**Parallel Opportunities Identified**: 35 tasks marked [P] can run in parallel with others in their phase

**Independent Test Criteria**:
- **US1**: Combine two patterns, verify value combined and elements concatenated
- **US2**: Run 10,000+ associativity property tests, verify 100% pass
- **US3**: Fold collection of patterns, verify result matches pairwise combination

**Suggested MVP Scope**: 
- Phase 1-4 (User Stories 1 & 2)
- Total: 30 tasks
- Delivers: Working combination operation with mathematically verified associativity

---

## Notes

- [P] tasks = different files, no dependencies within phase
- [Story] label maps task to specific user story for traceability
- Each user story has clear independent test criteria
- Property-based testing is core to this feature (not optional)
- Tests written first (TDD approach) for all user stories
- MVP delivers mathematically verified combination operation
- Phase 2 is critical blocking phase - must complete before any user story work
- User Story 2 depends on User Story 1 (can't test associativity without combine())
- Follow test → implementation → verify pattern for each story

