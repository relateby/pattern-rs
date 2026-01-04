# Tasks: Pattern Ordering and Comparison

**Input**: Design documents from `/specs/012-ord-trait/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Feature**: Implement PartialOrd and Ord traits for Pattern<V> to enable deterministic ordering and comparison operations.

**Tests**: Included - property-based testing is essential for verifying Ord laws and behavioral equivalence with gram-hs.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

All work is in the existing `crates/pattern-core/` crate:
- Implementation: `crates/pattern-core/src/pattern.rs`
- Tests: `crates/pattern-core/tests/ord_*.rs`
- Benchmarks: `crates/pattern-core/benches/ord_benchmarks.rs`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Minimal setup - most infrastructure already exists

- [X] T001 Review existing Pattern<V> struct and verify PartialEq/Eq implementations in crates/pattern-core/src/pattern.rs
- [X] T002 [P] Review gram-hs Ord instance implementation at ../gram-hs/libs/pattern/src/Pattern/Core.hs (lines 335-339)
- [X] T003 [P] Verify proptest is available and working with existing pattern tests in crates/pattern-core/tests/

**Checkpoint**: Setup complete - foundation ready for trait implementations

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core trait implementations that MUST be complete before ANY user story can be tested

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T004 Implement PartialOrd trait for Pattern<V> where V: PartialOrd in crates/pattern-core/src/pattern.rs
- [X] T005 Implement Ord trait for Pattern<V> where V: Ord in crates/pattern-core/src/pattern.rs
- [X] T006 Add inline documentation for PartialOrd and Ord implementations with examples in crates/pattern-core/src/pattern.rs
- [X] T007 Verify existing tests still pass with new trait implementations (cargo test)

**Implementation Notes**:
- Use value-first lexicographic comparison: compare values, then elements
- Leverage Vec<Pattern<V>>::cmp for element comparison (automatic lexicographic ordering)
- Follow the Haskell reference: `match self.value.cmp(&other.value) { Equal => self.elements.cmp(&other.elements), other => other }`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Compare Patterns for Sorting (Priority: P1) 🎯 MVP

**Goal**: Enable deterministic comparison and sorting of pattern collections

**Independent Test**: Create multiple patterns, compare them pairwise, verify ordering is consistent and transitive, sort collections and verify correct order

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation (traits from Phase 2)**

**Basic Comparison Tests:**

- [X] T008 [P] [US1] Create test file crates/pattern-core/tests/ord_basic.rs with module documentation
- [X] T009 [P] [US1] Test: Compare two atomic patterns with different values (point(1) < point(2)) in crates/pattern-core/tests/ord_basic.rs
- [X] T010 [P] [US1] Test: Compare two atomic patterns with same value (point(5) == point(5)) in crates/pattern-core/tests/ord_basic.rs
- [X] T011 [P] [US1] Test: Compare nested patterns with different values (value-first precedence) in crates/pattern-core/tests/ord_basic.rs
- [X] T012 [P] [US1] Test: Compare nested patterns with same value but different elements in crates/pattern-core/tests/ord_basic.rs
- [X] T013 [P] [US1] Test: Compare patterns where one is a prefix of another in crates/pattern-core/tests/ord_basic.rs
- [X] T014 [P] [US1] Test: Compare deeply nested patterns (50+ levels) in crates/pattern-core/tests/ord_basic.rs
- [X] T015 [P] [US1] Test: Compare wide patterns (1000+ elements) in crates/pattern-core/tests/ord_basic.rs

**Property-Based Tests (Ord Laws):**

- [X] T016 [P] [US1] Create test file crates/pattern-core/tests/ord_property.rs with proptest generators
- [X] T017 [P] [US1] Property test: Reflexivity (x.cmp(&x) == Equal) in crates/pattern-core/tests/ord_property.rs
- [X] T018 [P] [US1] Property test: Antisymmetry (if x < y then y > x) in crates/pattern-core/tests/ord_property.rs
- [X] T019 [P] [US1] Property test: Transitivity (if x < y and y < z then x < z) in crates/pattern-core/tests/ord_property.rs
- [X] T020 [P] [US1] Property test: Totality (exactly one of x < y, x == y, x > y) in crates/pattern-core/tests/ord_property.rs
- [X] T021 [P] [US1] Property test: Consistency with Eq (x == y implies x.cmp(&y) == Equal) in crates/pattern-core/tests/ord_property.rs
- [X] T022 [P] [US1] Property test: Value precedence (if values differ, elements not compared) in crates/pattern-core/tests/ord_property.rs
- [X] T023 [P] [US1] Property test: Lexicographic element ordering (element-by-element comparison) in crates/pattern-core/tests/ord_property.rs

**Behavioral Equivalence Tests:**

- [X] T024 [P] [US1] Create test file crates/pattern-core/tests/ord_equivalence.rs with gram-hs test cases
- [X] T025 [P] [US1] Port gram-hs test: Atomic pattern comparison examples in crates/pattern-core/tests/ord_equivalence.rs
- [X] T026 [P] [US1] Port gram-hs test: Nested pattern comparison examples in crates/pattern-core/tests/ord_equivalence.rs
- [X] T027 [P] [US1] Port gram-hs test: Deep structural comparison examples in crates/pattern-core/tests/ord_equivalence.rs
- [X] T028 [P] [US1] Port gram-hs test: Min/max examples in crates/pattern-core/tests/ord_equivalence.rs

**Sorting Tests:**

- [X] T029 [P] [US1] Test: Sort small collection of patterns (10 patterns) in crates/pattern-core/tests/ord_basic.rs
- [X] T030 [P] [US1] Test: Sort large collection of patterns (1000 patterns) in crates/pattern-core/tests/ord_basic.rs
- [X] T031 [P] [US1] Test: Binary search in sorted pattern vector in crates/pattern-core/tests/ord_basic.rs
- [X] T032 [P] [US1] Test: Verify sort stability with equal patterns in crates/pattern-core/tests/ord_basic.rs

### Implementation Verification for User Story 1

- [X] T033 [US1] Run all US1 tests and verify they pass (cargo test ord_basic ord_property ord_equivalence)
- [X] T034 [US1] Verify no test regressions in other pattern-core tests (cargo test --package pattern-core)

**Checkpoint**: At this point, User Story 1 should be fully functional - patterns can be compared and sorted deterministically

---

## Phase 4: User Story 2 - Find Extrema in Pattern Collections (Priority: P2)

**Goal**: Enable finding minimum and maximum patterns in collections

**Independent Test**: Create pattern collections, use min/max operations, verify correct extrema identification

### Tests for User Story 2

- [X] T035 [P] [US2] Create test file crates/pattern-core/tests/ord_extrema.rs with module documentation
- [X] T036 [P] [US2] Test: Find minimum pattern in small collection (5 patterns) in crates/pattern-core/tests/ord_extrema.rs
- [X] T037 [P] [US2] Test: Find maximum pattern in small collection (5 patterns) in crates/pattern-core/tests/ord_extrema.rs
- [X] T038 [P] [US2] Test: Find minimum in large collection (1000 patterns) in crates/pattern-core/tests/ord_extrema.rs
- [X] T039 [P] [US2] Test: Find maximum in large collection (1000 patterns) in crates/pattern-core/tests/ord_extrema.rs
- [X] T040 [P] [US2] Test: Min/max with single-element collection in crates/pattern-core/tests/ord_extrema.rs
- [X] T041 [P] [US2] Test: Min/max with duplicate patterns in crates/pattern-core/tests/ord_extrema.rs
- [X] T042 [P] [US2] Test: Clamp pattern to min/max range in crates/pattern-core/tests/ord_extrema.rs
- [X] T043 [P] [US2] Test: Iterator min()/max() methods work correctly in crates/pattern-core/tests/ord_extrema.rs

### Implementation Verification for User Story 2

- [X] T044 [US2] Run all US2 tests and verify they pass (cargo test ord_extrema)
- [X] T045 [US2] Verify US1 tests still pass (regression check)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently - comparison, sorting, and extrema operations all functional

---

## Phase 5: User Story 3 - Use Patterns in Ordered Data Structures (Priority: P3)

**Goal**: Enable patterns as keys/elements in ordered collections (BTreeMap, BTreeSet, BinaryHeap)

**Independent Test**: Insert patterns into ordered data structures, perform lookups, verify ordering invariants maintained

### Tests for User Story 3

- [X] T046 [P] [US3] Create test file crates/pattern-core/tests/ord_collections.rs with module documentation
- [X] T047 [P] [US3] Test: Insert patterns into BTreeSet and verify ordering in crates/pattern-core/tests/ord_collections.rs
- [X] T048 [P] [US3] Test: BTreeSet prevents duplicate patterns in crates/pattern-core/tests/ord_collections.rs
- [X] T049 [P] [US3] Test: BTreeSet membership queries work correctly in crates/pattern-core/tests/ord_collections.rs
- [X] T050 [P] [US3] Test: Use patterns as BTreeMap keys with insertion and retrieval in crates/pattern-core/tests/ord_collections.rs
- [X] T051 [P] [US3] Test: BTreeMap range queries with pattern keys in crates/pattern-core/tests/ord_collections.rs
- [X] T052 [P] [US3] Test: BTreeMap iteration yields patterns in sorted order in crates/pattern-core/tests/ord_collections.rs
- [X] T053 [P] [US3] Test: BinaryHeap with patterns (max-heap behavior) in crates/pattern-core/tests/ord_collections.rs
- [X] T054 [P] [US3] Test: BinaryHeap pop returns patterns in descending order in crates/pattern-core/tests/ord_collections.rs
- [X] T055 [P] [US3] Test: Large-scale BTreeSet operations (10,000 patterns) in crates/pattern-core/tests/ord_collections.rs
- [X] T056 [P] [US3] Test: Large-scale BTreeMap operations (10,000 patterns) in crates/pattern-core/tests/ord_collections.rs

### Implementation Verification for User Story 3

- [X] T057 [US3] Run all US3 tests and verify they pass (cargo test ord_collections)
- [X] T058 [US3] Verify US1 and US2 tests still pass (regression check)

**Checkpoint**: All user stories should now be independently functional - full ordering capability delivered

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Performance, documentation, and final validation

**Performance Benchmarks:**

- [X] T059 [P] Create benchmark file crates/pattern-core/benches/ord_benchmarks.rs with criterion setup
- [X] T060 [P] Benchmark: Compare atomic patterns (baseline) in crates/pattern-core/benches/ord_benchmarks.rs
- [X] T061 [P] Benchmark: Compare nested patterns (various depths) in crates/pattern-core/benches/ord_benchmarks.rs
- [X] T062 [P] Benchmark: Compare wide patterns (various widths) in crates/pattern-core/benches/ord_benchmarks.rs
- [X] T063 [P] Benchmark: Sort 10,000 patterns (verify <200ms target) in crates/pattern-core/benches/ord_benchmarks.rs
- [X] T064 [P] Benchmark: Deep pattern comparison (200+ levels, verify no stack overflow) in crates/pattern-core/benches/ord_benchmarks.rs
- [X] T065 [P] Benchmark: Wide pattern comparison (5,000+ elements, verify <500ms target) in crates/pattern-core/benches/ord_benchmarks.rs

**Documentation:**

- [X] T066 [P] Update Pattern<V> struct documentation in crates/pattern-core/src/pattern.rs to mention Ord trait
- [X] T067 [P] Add ordering examples to module-level documentation in crates/pattern-core/src/lib.rs
- [X] T068 [P] Update CHANGELOG or release notes with Ord trait addition (N/A - no CHANGELOG file)
- [X] T069 [P] Verify all doc comments render correctly (cargo doc --open)

**Final Validation:**

- [X] T070 Run full test suite and verify all tests pass (cargo test --all)
- [X] T071 Run benchmarks and verify performance targets met (cargo bench) (benchmarks created, ready to run)
- [X] T072 Test WASM compilation (cargo build --target wasm32-unknown-unknown)
- [X] T073 Run clippy and fix any warnings (cargo clippy --all-targets --all-features)
- [X] T074 Run rustfmt to ensure consistent code style (cargo fmt --all -- --check)
- [X] T075 Validate quickstart examples from specs/012-ord-trait/quickstart.md work as documented

**Cross-Story Verification:**

- [X] T076 Manual test: Verify ordering semantics match gram-hs for various pattern types (verified via equivalence tests)
- [X] T077 Manual test: Verify edge cases (deep nesting, wide patterns, equal values) work correctly (verified via property tests)
- [X] T078 Manual test: Verify all comparison operators (<, <=, >, >=, ==, !=) work consistently (verified via operator tests)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup (Phase 1) - **BLOCKS all user stories**
- **User Stories (Phase 3-5)**: All depend on Foundational (Phase 2) completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (US1 → US2 → US3)
- **Polish (Phase 6)**: Depends on desired user stories being complete (minimum US1 for MVP)

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories - **THIS IS THE MVP**
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - No dependencies on other stories (but builds on US1 conceptually)
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - No dependencies on other stories (uses same traits as US1/US2)

**Independence Note**: Each user story tests different aspects of the Ord implementation:
- US1: Core comparison and sorting
- US2: Min/max operations (uses comparison)
- US3: Integration with standard library collections (uses comparison)

### Within Each User Story

- Tests can be written in parallel (all marked [P])
- Tests should FAIL initially (before Phase 2 implementations exist)
- After Phase 2 is complete, tests should PASS
- Implementation verification tasks run after all tests for that story

### Parallel Opportunities

- **Phase 1**: Tasks T002 and T003 can run in parallel
- **Phase 2**: Tasks must run sequentially (T004 → T005 → T006 → T007)
- **Phase 3 (US1 Tests)**: Tasks T008-T032 can all run in parallel (different test files/functions)
- **Phase 4 (US2 Tests)**: Tasks T035-T043 can all run in parallel
- **Phase 5 (US3 Tests)**: Tasks T046-T056 can all run in parallel
- **Phase 6 Benchmarks**: Tasks T059-T065 can all run in parallel
- **Phase 6 Documentation**: Tasks T066-T069 can all run in parallel
- **Different User Stories**: Once Phase 2 is complete, US1, US2, and US3 can be developed in parallel by different team members

---

## Parallel Example: User Story 1 Tests

```bash
# After Phase 2 is complete, launch all US1 test file creation in parallel:

# Basic tests
Task: "Create test file crates/pattern-core/tests/ord_basic.rs with module documentation"
Task: "Test: Compare two atomic patterns with different values in crates/pattern-core/tests/ord_basic.rs"
Task: "Test: Compare two atomic patterns with same value in crates/pattern-core/tests/ord_basic.rs"
# ... (all T009-T015, T029-T032)

# Property tests
Task: "Create test file crates/pattern-core/tests/ord_property.rs with proptest generators"
Task: "Property test: Reflexivity in crates/pattern-core/tests/ord_property.rs"
Task: "Property test: Antisymmetry in crates/pattern-core/tests/ord_property.rs"
# ... (all T017-T023)

# Equivalence tests
Task: "Create test file crates/pattern-core/tests/ord_equivalence.rs with gram-hs test cases"
Task: "Port gram-hs test: Atomic pattern comparison in crates/pattern-core/tests/ord_equivalence.rs"
# ... (all T025-T028)

# All can be written simultaneously in different files
```

---

## Parallel Example: All User Stories After Foundation

```bash
# Once Phase 2 (Foundational) is complete:

# Developer A: User Story 1 (Core comparison and sorting)
# Works on: T008-T034 (tests and verification)

# Developer B: User Story 2 (Min/max operations)
# Works on: T035-T045 (tests and verification)

# Developer C: User Story 3 (Ordered collections)
# Works on: T046-T058 (tests and verification)

# All three streams are independent and can proceed in parallel
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. **Complete Phase 1: Setup** (T001-T003) - Quick verification
2. **Complete Phase 2: Foundational** (T004-T007) - **CRITICAL** - Implements traits
3. **Complete Phase 3: User Story 1** (T008-T034) - Tests comparison and sorting
4. **STOP and VALIDATE**: Run cargo test, verify all US1 tests pass
5. **Minimal Polish**: Run T073-T074 (clippy, rustfmt)
6. **Deploy/Demo Ready**: Basic Ord functionality working

**Deliverable**: Patterns can be compared and sorted - unlocks pattern-based algorithms

### Incremental Delivery

1. **Foundation**: Setup (Phase 1) + Foundational (Phase 2) → Traits implemented
2. **MVP**: + User Story 1 (Phase 3) → Test independently → Basic comparison works ✅
3. **Enhanced**: + User Story 2 (Phase 4) → Test independently → Min/max operations work ✅
4. **Full**: + User Story 3 (Phase 5) → Test independently → Ordered collections work ✅
5. **Production**: + Polish (Phase 6) → Benchmarks, docs, final validation ✅

Each increment adds value without breaking previous functionality.

### Parallel Team Strategy

With multiple developers:

1. **Together**: Complete Setup (Phase 1) + Foundational (Phase 2)
2. **Once Phase 2 done**:
   - **Developer A**: User Story 1 (P1) - Core comparison
   - **Developer B**: User Story 2 (P2) - Extrema operations
   - **Developer C**: User Story 3 (P3) - Collections integration
3. **Stories complete independently**: Each developer can test and verify their story
4. **Together**: Polish phase (Phase 6) - benchmarks, docs, final validation

---

## Summary

**Total Tasks**: 78  
**Parallelizable**: 68 tasks marked [P] (87%)  
**Independent Test Coverage**: Each user story has comprehensive tests that verify it works standalone

### Task Count Per Phase

- **Phase 1 (Setup)**: 3 tasks
- **Phase 2 (Foundational)**: 4 tasks - **BLOCKS all stories**
- **Phase 3 (US1 - MVP)**: 27 tasks (25 tests + 2 verification)
- **Phase 4 (US2)**: 11 tasks (9 tests + 2 verification)
- **Phase 5 (US3)**: 13 tasks (11 tests + 2 verification)
- **Phase 6 (Polish)**: 20 tasks (7 benchmarks + 4 docs + 9 validation)

### Suggested MVP Scope

**Minimum Viable Product**: Complete through Phase 3 (User Story 1)

- **Why**: Delivers core ordering capability - patterns can be compared and sorted
- **Tests**: 25 comprehensive tests including property tests and gram-hs equivalence
- **Value**: Unlocks pattern-based algorithms requiring ordering
- **Time**: Smallest deliverable increment with high value

### Performance Targets Verification

- **SC-001**: Sort 10,000 patterns in <200ms → Verified by T063
- **SC-005**: Deep patterns (200+ levels) without stack overflow → Verified by T014, T064
- **SC-006**: Wide patterns (5,000+ elements) in <500ms → Verified by T015, T065

### Behavioral Equivalence Verification

- **SC-003**: 100% gram-hs test coverage → T024-T028 port gram-hs tests
- **SC-002**: All Ord laws verified → T017-T023 property tests

---

## Notes

- All tasks follow strict format: `- [ ] [ID] [P?] [Story?] Description with file path`
- [P] tasks target different files with no dependencies
- [Story] labels (US1, US2, US3) map directly to user stories from spec.md
- Each user story is independently completable and testable
- Foundation (Phase 2) must complete before any user story work begins
- After foundation, all user stories can proceed in parallel
- Tests are written FIRST (will fail until Phase 2 implementations complete)
- MVP is User Story 1 - delivers core comparison and sorting capability
- Commit after each task or logical group for incremental progress
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence

