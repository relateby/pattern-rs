# Tasks: Predicate-Based Pattern Matching

**Input**: Design documents from `/specs/016-predicate-matching/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/type-signatures.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US2, US3)
- Include exact file paths in descriptions

## Implementation Status

- ✅ **User Story 1 (P1) - Query by Value Properties**: COMPLETE (any_value, all_values already implemented with 66 tests each)
- 🚧 **User Story 2 (P2) - Find/Filter by Structure**: PARTIAL (filter complete with 66 tests, need find_first)
- ❌ **User Story 3 (P3) - Structural Matching**: NOT STARTED (need matches and contains)

---

## Phase 1: Setup & Verification

**Purpose**: Verify existing implementation and prepare for new methods

- [x] T001 Verify existing predicate methods compile and pass tests (any_value, all_values, filter)
- [x] T002 Review gram-hs reference implementation at `../gram-hs/specs/012-predicate-matching/` for behavioral equivalence requirements
- [x] T003 [P] Create .gitignore entries for Rust project (target/, *.rs.bk, Cargo.lock for libraries, .idea/, *.log, .env*)

---

## Phase 2: User Story 2 - Find First Pattern (Priority: P2)

**Goal**: Add `find_first` method to enable finding the first matching subpattern with Option return semantics

**Independent Test**: Can be tested by creating patterns with various structures, calling find_first with predicates, and verifying it returns Some for the first match in pre-order traversal or None when no match exists

### Tests for User Story 2 (TDD - Write First)

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T004 [US2] Create test file crates/pattern-core/tests/query_find_first.rs with test module structure
- [x] T005 [P] [US2] Write tests for find_first returning Some when root matches predicate in crates/pattern-core/tests/query_find_first.rs
- [x] T006 [P] [US2] Write tests for find_first returning Some when element matches predicate in crates/pattern-core/tests/query_find_first.rs
- [x] T007 [P] [US2] Write tests for find_first returning Some for deeply nested matching pattern in crates/pattern-core/tests/query_find_first.rs
- [x] T008 [P] [US2] Write tests for find_first returning None when no patterns match in crates/pattern-core/tests/query_find_first.rs
- [x] T009 [P] [US2] Write tests for find_first returning first match in pre-order when multiple match in crates/pattern-core/tests/query_find_first.rs
- [x] T010 [P] [US2] Write tests for find_first with atomic patterns in crates/pattern-core/tests/query_find_first.rs
- [x] T011 [P] [US2] Write tests for find_first with empty elements in crates/pattern-core/tests/query_find_first.rs
- [x] T012 [P] [US2] Write tests for find_first with deep nesting (100+ levels) in crates/pattern-core/tests/query_find_first.rs
- [x] T013 [P] [US2] Write tests for predicate examining value and structure in crates/pattern-core/tests/query_find_first.rs
- [x] T014 [P] [US2] Write tests for find_first integration with other Pattern methods in crates/pattern-core/tests/query_find_first.rs
- [x] T015 [US2] Run tests to verify they fail before implementation: `cargo test --test query_find_first`

### Implementation for User Story 2

- [x] T016 [US2] Implement find_first public method in crates/pattern-core/src/pattern.rs (signature: `pub fn find_first<F>(&self, predicate: F) -> Option<&Pattern<V>> where F: Fn(&Pattern<V>) -> bool`)
- [x] T017 [US2] Implement find_first_recursive helper method in crates/pattern-core/src/pattern.rs (recursive traversal with early termination)
- [x] T018 [US2] Add comprehensive documentation for find_first in crates/pattern-core/src/pattern.rs (purpose, parameters, returns, examples, complexity, panics, relationships)
- [x] T019 [US2] Run tests to verify implementation: `cargo test --test query_find_first`
- [x] T020 [US2] Verify WASM compatibility: `cargo build --target wasm32-unknown-unknown`

**Checkpoint**: At this point, find_first should be fully functional with all tests passing

---

## Phase 3: User Story 3 - Structural Matching (Priority: P3)

**Goal**: Add `matches` and `contains` methods to enable structural comparison and subpattern containment checking

**Independent Test**: Can be tested by creating pattern pairs with various structural relationships and verifying matches/contains return correct boolean results based on structural equality and containment

### Tests for User Story 3 - matches method (TDD - Write First)

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T021 [US3] Create test file crates/pattern-core/tests/predicate_matches.rs with test module structure
- [x] T022 [P] [US3] Write tests for matches returning true for identical patterns in crates/pattern-core/tests/predicate_matches.rs
- [x] T023 [P] [US3] Write tests for matches returning true for self-comparison (reflexive) in crates/pattern-core/tests/predicate_matches.rs
- [x] T024 [P] [US3] Write tests for matches returning false for different values in crates/pattern-core/tests/predicate_matches.rs
- [x] T025 [P] [US3] Write tests for matches returning false for different element counts in crates/pattern-core/tests/predicate_matches.rs
- [x] T026 [P] [US3] Write tests for matches returning false for different element structures in crates/pattern-core/tests/predicate_matches.rs
- [x] T027 [P] [US3] Write tests for matches distinguishing same values but different structures in crates/pattern-core/tests/predicate_matches.rs
- [x] T028 [P] [US3] Write tests for matches symmetry property in crates/pattern-core/tests/predicate_matches.rs
- [x] T029 [P] [US3] Write tests for matches with atomic patterns in crates/pattern-core/tests/predicate_matches.rs
- [x] T030 [P] [US3] Write tests for matches with empty elements in crates/pattern-core/tests/predicate_matches.rs
- [x] T031 [P] [US3] Write tests for matches with deeply nested structures in crates/pattern-core/tests/predicate_matches.rs
- [x] T032 [US3] Run tests to verify they fail before implementation: `cargo test --test predicate_matches`

### Tests for User Story 3 - contains method (TDD - Write First)

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T033 [US3] Create test file crates/pattern-core/tests/predicate_contains.rs with test module structure
- [x] T034 [P] [US3] Write tests for contains returning true for self-containment in crates/pattern-core/tests/predicate_contains.rs
- [x] T035 [P] [US3] Write tests for contains returning true when subpattern is direct element in crates/pattern-core/tests/predicate_contains.rs
- [x] T036 [P] [US3] Write tests for contains returning true when subpattern is nested descendant in crates/pattern-core/tests/predicate_contains.rs
- [x] T037 [P] [US3] Write tests for contains returning false when subpattern not found in crates/pattern-core/tests/predicate_contains.rs
- [x] T038 [P] [US3] Write tests for contains transitivity property in crates/pattern-core/tests/predicate_contains.rs
- [x] T039 [P] [US3] Write tests for contains being weaker than matches in crates/pattern-core/tests/predicate_contains.rs
- [x] T040 [P] [US3] Write tests for contains with atomic patterns in crates/pattern-core/tests/predicate_contains.rs
- [x] T041 [P] [US3] Write tests for contains with empty elements in crates/pattern-core/tests/predicate_contains.rs
- [x] T042 [P] [US3] Write tests for contains with deeply nested structures in crates/pattern-core/tests/predicate_contains.rs
- [x] T043 [P] [US3] Write tests for contains handling multiple occurrences in crates/pattern-core/tests/predicate_contains.rs
- [x] T044 [US3] Run tests to verify they fail before implementation: `cargo test --test predicate_contains`

### Implementation for User Story 3 - matches method

- [x] T045 [US3] Implement matches public method in crates/pattern-core/src/pattern.rs (signature: `pub fn matches(&self, other: &Pattern<V>) -> bool where V: PartialEq`)
- [x] T046 [US3] Add comprehensive documentation for matches in crates/pattern-core/src/pattern.rs (purpose, parameters, returns, examples, complexity, panics, relationships)
- [x] T047 [US3] Run matches tests to verify implementation: `cargo test --test predicate_matches`

### Implementation for User Story 3 - contains method

- [x] T048 [US3] Implement contains public method in crates/pattern-core/src/pattern.rs (signature: `pub fn contains(&self, subpattern: &Pattern<V>) -> bool where V: PartialEq`, using matches internally)
- [x] T049 [US3] Add comprehensive documentation for contains in crates/pattern-core/src/pattern.rs (purpose, parameters, returns, examples, complexity, panics, relationships)
- [x] T050 [US3] Run contains tests to verify implementation: `cargo test --test predicate_contains`

### User Story 3 Verification

- [x] T051 [US3] Run all US3 tests together: `cargo test predicate_matches predicate_contains`
- [x] T052 [US3] Verify WASM compatibility: `cargo build --target wasm32-unknown-unknown`

**Checkpoint**: At this point, matches and contains should be fully functional with all tests passing

---

## Phase 4: Property-Based Tests (Cross-Story Validation)

**Purpose**: Verify mathematical properties and relationships between all predicate functions

- [ ] T053 Create test file crates/pattern-core/tests/predicate_properties.rs with proptest infrastructure
- [ ] T054 [P] Write property test for find_first consistency with filter in crates/pattern-core/tests/predicate_properties.rs
- [ ] T055 [P] Write property test for find_first returning first element in crates/pattern-core/tests/predicate_properties.rs
- [ ] T056 [P] Write property test for matches reflexivity in crates/pattern-core/tests/predicate_properties.rs
- [ ] T057 [P] Write property test for matches symmetry in crates/pattern-core/tests/predicate_properties.rs
- [ ] T058 [P] Write property test for contains reflexivity in crates/pattern-core/tests/predicate_properties.rs
- [ ] T059 [P] Write property test for contains transitivity in crates/pattern-core/tests/predicate_properties.rs
- [ ] T060 [P] Write property test for matches implies contains in crates/pattern-core/tests/predicate_properties.rs
- [ ] T061 [P] Write property test generators for patterns with various structures in crates/pattern-core/tests/predicate_properties.rs
- [ ] T062 Run property tests: `cargo test --test predicate_properties`

---

## Phase 5: Performance Benchmarks

**Purpose**: Verify performance targets (SC-005: find_first <10ms for early matches, SC-006: all ops <100ms for 1000 nodes/100 depth)

- [ ] T063 Create benchmark file crates/pattern-core/benches/predicate_benchmarks.rs with criterion infrastructure
- [ ] T064 [P] Implement find_first benchmark for 1000-node pattern with match in first 10 nodes in crates/pattern-core/benches/predicate_benchmarks.rs
- [ ] T065 [P] Implement find_first benchmark for 1000-node pattern with no match in crates/pattern-core/benches/predicate_benchmarks.rs
- [ ] T066 [P] Implement matches benchmark for 1000-node patterns with 100-level depth in crates/pattern-core/benches/predicate_benchmarks.rs
- [ ] T067 [P] Implement contains benchmark for 1000-node patterns with 100-level depth in crates/pattern-core/benches/predicate_benchmarks.rs
- [ ] T068 [P] Implement deep nesting benchmark (100+ levels) to verify no stack overflow in crates/pattern-core/benches/predicate_benchmarks.rs
- [ ] T069 Run benchmarks: `cargo bench --bench predicate_benchmarks`
- [ ] T070 Verify performance targets are met (find_first <10ms early match, all <100ms for 1000 nodes/100 depth)

---

## Phase 6: Behavioral Equivalence with gram-hs

**Purpose**: Verify behavioral equivalence with reference implementation (SC-007: 100% equivalence in test cases)

- [ ] T071 Review gram-hs test suite in `../gram-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs` for find_first (findPattern) test cases
- [ ] T072 Review gram-hs test suite in `../gram-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs` for matches test cases
- [ ] T073 Review gram-hs test suite in `../gram-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs` for contains test cases
- [ ] T074 [P] Add equivalence tests for find_first in crates/pattern-core/tests/query_find_first.rs (compare with gram-hs expected output)
- [ ] T075 [P] Add equivalence tests for matches in crates/pattern-core/tests/predicate_matches.rs (compare with gram-hs expected output)
- [ ] T076 [P] Add equivalence tests for contains in crates/pattern-core/tests/predicate_contains.rs (compare with gram-hs expected output)
- [ ] T077 Run all equivalence tests: `cargo test equivalence`
- [ ] T078 Document any intentional deviations from gram-hs behavior in specs/016-predicate-matching/research.md

---

## Phase 7: Polish & Documentation

**Purpose**: Final improvements and documentation updates

- [ ] T079 [P] Add module-level documentation updates in crates/pattern-core/src/pattern.rs (update file header with new methods)
- [ ] T080 [P] Verify all new methods appear in rustdoc: `cargo doc --no-deps --open`
- [ ] T081 [P] Update crates/pattern-core/README.md with examples of new methods (if README exists)
- [ ] T082 [P] Verify code follows Rust formatting: `cargo fmt -- --check`
- [ ] T083 [P] Verify code passes clippy lints: `cargo clippy -- -D warnings`
- [ ] T084 Run full test suite: `cargo test --all-features`
- [ ] T085 Run full test suite in release mode: `cargo test --release`
- [ ] T086 Verify WASM build in release mode: `cargo build --release --target wasm32-unknown-unknown`
- [ ] T087 Update specs/016-predicate-matching/plan.md to mark Post-Implementation status as complete
- [ ] T088 Create implementation summary documenting any deviations or notable decisions

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **User Story 2 (Phase 2)**: Depends on Setup (Phase 1) completion
- **User Story 3 (Phase 3)**: Depends on Setup (Phase 1) completion - can run in parallel with US2
- **Property Tests (Phase 4)**: Depends on US2 and US3 implementation completion
- **Benchmarks (Phase 5)**: Depends on US2 and US3 implementation completion - can run in parallel with Phase 4
- **Equivalence (Phase 6)**: Depends on US2 and US3 implementation completion - can run in parallel with Phases 4 & 5
- **Polish (Phase 7)**: Depends on all previous phases completion

### User Story Dependencies

- **User Story 2 (find_first)**: Independent - can be implemented and tested standalone
- **User Story 3 (matches/contains)**: Independent - can be implemented in parallel with US2
  - Note: contains depends on matches implementation, but both are within US3

### Within Each User Story (TDD Approach)

1. **Tests FIRST**: Write all tests for a method and verify they FAIL
2. **Implementation**: Implement the method to make tests pass
3. **Documentation**: Add comprehensive docs
4. **Verification**: Run tests and verify WASM compatibility

### Parallel Opportunities

- **Phase 1**: T001, T002, T003 can run in parallel (different concerns)
- **US2 Tests**: T005-T014 can run in parallel (different test files/test functions)
- **US3 matches tests**: T022-T031 can run in parallel (different test functions)
- **US3 contains tests**: T034-T043 can run in parallel (different test functions)
- **Property tests**: T054-T061 can run in parallel (different properties)
- **Benchmarks**: T064-T068 can run in parallel (different benchmark functions)
- **Equivalence tests**: T074-T076 can run in parallel (different methods)
- **Polish tasks**: T079-T083, T085-T086 can run in parallel (different verification tasks)
- **US2 and US3 can be worked on in parallel** (different methods, no dependencies)

---

## Parallel Example: User Story 2 Tests

```bash
# Launch all tests for find_first together:
# All these create different test functions in query_find_first.rs
Task T005: Test root matches
Task T006: Test element matches
Task T007: Test nested matches
Task T008: Test no matches
Task T009: Test first match ordering
Task T010: Test atomic patterns
Task T011: Test empty elements
Task T012: Test deep nesting
Task T013: Test predicate examining structure
Task T014: Test integration with other methods
```

---

## Parallel Example: User Story 2 + User Story 3

```bash
# US2 and US3 can proceed in parallel (different methods, independent):
Developer A: T016-T020 (implement find_first)
Developer B: T045-T052 (implement matches and contains)

# These are completely independent - no file conflicts
```

---

## Implementation Strategy

### MVP First (User Story 2 Only)

1. Complete Phase 1: Setup & Verification (T001-T003)
2. Complete Phase 2: User Story 2 - find_first (T004-T020)
3. **STOP and VALIDATE**: Test find_first independently
4. Can deploy/use just find_first if needed

### Incremental Delivery

1. Complete Setup (Phase 1) → T001-T003
2. Add find_first (Phase 2) → T004-T020 → Test independently → MVP!
3. Add matches/contains (Phase 3) → T021-T052 → Test independently
4. Add property tests (Phase 4) → T053-T062 → Verify mathematical properties
5. Add benchmarks (Phase 5) → T063-T070 → Verify performance
6. Add equivalence tests (Phase 6) → T071-T078 → Verify gram-hs compatibility
7. Polish (Phase 7) → T079-T088 → Documentation and final checks

### Parallel Team Strategy

With multiple developers (after Phase 1):

1. Developer A: User Story 2 (find_first) → T004-T020
2. Developer B: User Story 3 (matches/contains) → T021-T052
3. Both complete independently, then:
   - Developer A: Property tests → T053-T062
   - Developer B: Benchmarks → T063-T070
   - Developer C: Equivalence tests → T071-T078
4. Team: Polish together → T079-T088

---

## Task Count Summary

- **Phase 1 (Setup)**: 3 tasks
- **Phase 2 (US2 - find_first)**: 17 tasks (11 tests + 6 implementation/docs)
- **Phase 3 (US3 - matches/contains)**: 32 tasks (22 tests + 10 implementation/docs)
- **Phase 4 (Property Tests)**: 10 tasks
- **Phase 5 (Benchmarks)**: 8 tasks
- **Phase 6 (Equivalence)**: 8 tasks
- **Phase 7 (Polish)**: 10 tasks

**Total**: 88 tasks

**Parallel Opportunities**: 52 tasks marked [P] can run in parallel (59% parallelizable)

**MVP Scope**: Phase 1 + Phase 2 = 20 tasks (23% of total) delivers find_first

---

## Notes

- [P] tasks = different files/functions, no dependencies, can run in parallel
- [US2] = User Story 2 (find_first method)
- [US3] = User Story 3 (matches and contains methods)
- Each user story is independently testable and deliverable
- TDD approach: Write tests first, verify they fail, then implement
- Verify WASM compatibility after each major implementation
- Property tests verify mathematical relationships between all methods
- Benchmarks verify performance targets from success criteria
- Equivalence tests verify behavioral equivalence with gram-hs reference implementation

