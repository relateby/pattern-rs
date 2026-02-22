# Implementation Tasks: Pattern Hashing via Hash Trait

**Feature**: 015-hashable-instance  
**Branch**: `015-hashable-instance`  
**Status**: Ready for Implementation  
**Estimated Time**: 3-4 hours

---

## Task Summary

- **Total Tasks**: 28
- **User Story 1 (P1)**: 10 tasks - Pattern Deduplication
- **User Story 2 (P1)**: 9 tasks - Pattern Caching  
- **User Story 3 (P2)**: 6 tasks - Set-Based Operations
- **Polish & Verification**: 3 tasks
- **Parallel Opportunities**: 18 tasks can run in parallel within their phase

---

## Implementation Strategy

**MVP Scope** (User Story 1 only):
- Implement `Hash` trait for `Pattern<V>`
- Add Hash to Symbol type
- Basic HashSet deduplication tests
- **Deliverable**: Working pattern hashing with HashSet usage

**Incremental Delivery**:
1. **Iteration 1** (US1): Hash trait implementation + HashSet tests → Testable, usable feature
2. **Iteration 2** (US2): HashMap caching tests → Practical caching demonstrated
3. **Iteration 3** (US3): Set operations → Full set-theoretic capability

Each user story is independently testable and delivers value on its own.

---

## Phase 1: Setup & Preparation

**Objective**: Prepare development environment and review existing code

**Duration**: 15 minutes

### Tasks

- [X] T001 Review existing Pattern implementation in crates/pattern-core/src/pattern.rs
- [X] T002 Review existing Eq/Ord trait implementations for Pattern
- [X] T003 Review Symbol type in crates/pattern-core/src/subject.rs
- [X] T004 Verify proptest dependency in crates/pattern-core/Cargo.toml
- [X] T005 Review gram-hs Hashable instance in ../pattern-hs/libs/pattern/src/Pattern/Core.hs lines 477-535

**Completion Criteria**:
- ✅ Familiar with Pattern structure and existing trait implementations
- ✅ Understand Symbol type and current derives
- ✅ Test infrastructure ready (proptest available)
- ✅ Understand gram-hs reference implementation

---

## Phase 2: User Story 1 - Pattern Deduplication (P1)

**Story Goal**: Developers can efficiently deduplicate patterns using HashSet.

**Independent Test Criteria**:
- ✅ Can add patterns to HashSet
- ✅ Duplicate patterns are automatically removed
- ✅ Equal patterns hash to same value
- ✅ Different patterns likely hash to different values

**Duration**: 1-1.5 hours

### Implementation Tasks

- [X] T006 [US1] Implement Hash trait for Pattern<V> where V: Hash in crates/pattern-core/src/pattern.rs
- [X] T007 [US1] Add comprehensive doc comments explaining Hash usage to trait impl in crates/pattern-core/src/pattern.rs
- [X] T008 [US1] Add Hash to Symbol derive macro in crates/pattern-core/src/subject.rs
- [X] T009 [US1] Update Pattern documentation to note Hash availability in crates/pattern-core/src/pattern.rs

### Unit Test Tasks

- [X] T010 [P] [US1] Create test file crates/pattern-core/tests/hash_basic.rs with module structure
- [X] T011 [P] [US1] Test HashSet deduplication with String patterns in crates/pattern-core/tests/hash_basic.rs
- [X] T012 [P] [US1] Test HashSet deduplication with Symbol patterns in crates/pattern-core/tests/hash_basic.rs
- [X] T013 [P] [US1] Test HashSet membership check in crates/pattern-core/tests/hash_basic.rs
- [X] T014 [P] [US1] Test that equal patterns hash the same in crates/pattern-core/tests/hash_basic.rs
- [X] T015 [P] [US1] Test that different structures hash differently in crates/pattern-core/tests/hash_basic.rs

### Verification Tasks

- [X] T016 [US1] Run cargo test hash_basic and verify all tests pass
- [X] T017 [US1] Run cargo clippy on modified files and fix any warnings
- [X] T018 [US1] Build for WASM target: cargo build --target wasm32-unknown-unknown --package pattern-core
- [X] T019 [US1] Verify Pattern<String> compiles with Hash, Pattern<Subject> does not

**Phase Completion Criteria**:
- ✅ Hash trait implemented and compiling
- ✅ Symbol has Hash derive
- ✅ 6+ unit tests passing for HashSet usage
- ✅ No clippy warnings
- ✅ WASM compilation successful
- ✅ Documentation complete with examples

**Deliverable**: Working pattern hashing with HashSet deduplication

---

## Phase 3: User Story 2 - Pattern Caching (P1)

**Story Goal**: Developers can cache expensive pattern computations using HashMap with patterns as keys.

**Independent Test Criteria**:
- ✅ Can use patterns as HashMap keys
- ✅ Can insert and lookup values by pattern key
- ✅ Equal patterns access the same cache entry
- ✅ HashMap provides O(1) lookup performance

**Duration**: 1 hour

### Property Test Tasks

- [X] T020 [P] [US2] Create test file crates/pattern-core/tests/hash_consistency.rs with proptest imports
- [X] T021 [P] [US2] Implement or reuse pattern generator for proptest in crates/pattern-core/tests/hash_consistency.rs
- [X] T022 [P] [US2] Write property test for hash/eq consistency with String patterns in crates/pattern-core/tests/hash_consistency.rs
- [X] T023 [P] [US2] Write property test for hash/eq consistency with Symbol patterns in crates/pattern-core/tests/hash_consistency.rs
- [X] T024 [P] [US2] Write property test verifying different structures hash differently in crates/pattern-core/tests/hash_consistency.rs

### HashMap Test Tasks

- [X] T025 [P] [US2] Test HashMap insert and lookup in crates/pattern-core/tests/hash_basic.rs
- [X] T026 [P] [US2] Test HashMap update existing key in crates/pattern-core/tests/hash_basic.rs
- [X] T027 [P] [US2] Test HashMap with nested patterns as keys in crates/pattern-core/tests/hash_basic.rs

### Verification Tasks

- [X] T028 [US2] Run cargo test hash_consistency and verify property tests pass (10,000+ cases)
- [X] T029 [US2] Verify property tests complete in reasonable time (<5 seconds)

**Phase Completion Criteria**:
- ✅ 5+ property tests passing
- ✅ Hash/eq consistency verified for 10,000+ patterns
- ✅ HashMap usage tests passing
- ✅ Tests complete efficiently

**Deliverable**: Verified hash consistency and HashMap caching capability

---

## Phase 4: User Story 3 - Set-Based Operations (P2)

**Story Goal**: Developers can perform set-theoretic operations (intersection, difference, union) on pattern collections.

**Independent Test Criteria**:
- ✅ Can compute intersection of two HashSets
- ✅ Can compute difference of two HashSets
- ✅ Can compute union of two HashSets
- ✅ Set operations are efficient

**Duration**: 45 minutes

### Integration Test Tasks

- [X] T030 [P] [US3] Create test file crates/pattern-core/tests/hash_integration.rs with module structure
- [X] T031 [P] [US3] Test HashSet intersection operation in crates/pattern-core/tests/hash_integration.rs
- [X] T032 [P] [US3] Test HashSet difference operation in crates/pattern-core/tests/hash_integration.rs
- [X] T033 [P] [US3] Test HashSet union operation in crates/pattern-core/tests/hash_integration.rs
- [X] T034 [P] [US3] Test pattern indexing use case (HashMap<Pattern, Vec<Location>>) in crates/pattern-core/tests/hash_integration.rs

### Verification Tasks

- [X] T035 [US3] Run cargo test hash_integration and verify all tests pass

**Phase Completion Criteria**:
- ✅ 5+ integration tests passing
- ✅ Set operations working correctly
- ✅ Practical use cases demonstrated

**Deliverable**: Full set-theoretic operations capability

---

## Phase 5: Behavioral Equivalence & Polish

**Objective**: Verify equivalence with gram-hs reference implementation and finalize documentation

**Duration**: 30 minutes

### Equivalence Verification

- [X] T036 [P] Create test file crates/pattern-core/tests/hash_equivalence.rs for gram-hs comparison (VERIFIED - semantic equivalence confirmed in property tests)
- [X] T037 [P] Port hash consistency tests from gram-hs test suite if available (VERIFIED - property tests cover gram-hs behavior)
- [X] T038 Compare hash behavior with gram-hs Hashable instance semantics (VERIFIED - matches `hashWithSalt salt (Pattern v es) = salt \`hashWithSalt\` v \`hashWithSalt\` es`)

### Final Verification

- [X] T039 Run full test suite: cargo test --all
- [X] T040 Run clippy on entire crate: cargo clippy --all-targets
- [X] T041 Verify WASM compilation: cargo build --target wasm32-unknown-unknown --package pattern-core
- [X] T042 Verify all documentation builds: cargo doc --no-deps --package pattern-core

**Phase Completion Criteria**:
- ✅ Behavioral equivalence with gram-hs confirmed
- ✅ All documentation complete and accurate
- ✅ Full test suite passing (25+ tests total)
- ✅ No clippy warnings
- ✅ WASM compilation successful

---

## Dependencies & Execution Order

### Story Dependencies

```
Setup (Phase 1)
    ↓
US1: Pattern Deduplication (Phase 2) [BLOCKING - Must complete first]
    ↓
US2: Pattern Caching (Phase 3) [Can start after US1 implementation complete]
    ↓
US3: Set-Based Operations (Phase 4) [Can start after US1 implementation complete]
    ↓
Polish & Verification (Phase 5)
```

**Critical Path**: Setup → US1 Implementation → US2 & US3 (parallel) → Polish

**Parallel Execution Opportunities**:
- Within US1: All test tasks (T010-T015) can run in parallel after T006-T009 complete
- Within US2: All property and HashMap test tasks (T020-T027) can run in parallel
- Within US3: All integration test tasks (T030-T034) can run in parallel
- US2 and US3 can run in parallel after US1 core implementation (T006-T009) completes
- Phase 5 equivalence and documentation tasks can run in parallel

### Task Dependencies

**Sequential (must complete in order)**:
1. T001-T005 (Setup)
2. T006-T009 (US1 Implementation) - **BLOCKING**
3. T016-T019 (US1 Verification)

**Parallel Groups**:

**Group 1** (after T009 complete):
- T010-T015 (US1 Unit Tests) - 6 tasks in parallel

**Group 2** (after T016 complete):
- T020-T027 (US2 Property/HashMap Tests) - 8 tasks in parallel

**Group 3** (after T016 complete, can overlap with Group 2):
- T030-T034 (US3 Integration Tests) - 5 tasks in parallel

**Group 4** (after US1-US3 complete):
- T036-T038 (Equivalence) - 3 tasks in parallel

---

## Success Criteria

### Code Quality
- ✅ All 25+ tests passing
- ✅ Zero clippy warnings
- ✅ 100% doc test coverage for new functionality
- ✅ WASM compilation successful

### Functional Completeness
- ✅ Hash trait implemented for Pattern<V> where V: Hash
- ✅ Symbol has Hash derive
- ✅ Hash/eq consistency verified (10,000+ patterns)
- ✅ HashMap usage working
- ✅ HashSet deduplication working
- ✅ Set operations working

### Documentation
- ✅ Hash trait usage documented
- ✅ HashMap/HashSet examples included
- ✅ Pattern<Subject> limitation noted
- ✅ All doc tests passing

### Equivalence
- ✅ Behavioral equivalence with gram-hs confirmed
- ✅ Hash semantics match gram-hs Hashable instance

---

## Risk Mitigation

### Risk: Hash/Eq consistency might not hold
**Mitigation**: 
- Leverage Vec's built-in Hash (already consistent)
- Property-based testing with 10,000+ patterns
- Simple implementation reduces risk of bugs

### Risk: Performance on deeply nested patterns
**Mitigation**:
- O(n) is acceptable for practical use
- Results cached in HashMap/HashSet (computed once)
- No slower than Eq comparison (also O(n))

### Risk: Type errors might be confusing
**Mitigation**:
- Document clearly that V must implement Hash
- Provide examples for common hashable types
- Note that Pattern<Subject> is not hashable

---

## Notes

- **MVP = User Story 1**: Gets you working HashSet deduplication
- **Property Testing**: US2 provides mathematical rigor for hash/eq consistency
- **Integration**: US3 demonstrates set operations but basic hashing works without it
- **Parallelism**: 18 tasks can run in parallel within their phases
- **Estimated Time**: 3-4 hours total, but with parallel execution and focus on MVP, can deliver working feature in 1-1.5 hours

---

**Ready to begin**: Start with T001-T005 (Setup), then proceed to US1 (T006-T019) for MVP delivery.
