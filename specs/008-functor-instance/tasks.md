# Implementation Tasks: Functor Instance for Pattern

**Feature**: 008-functor-instance  
**Branch**: `008-functor-instance`  
**Created**: 2026-01-04

## Task Overview

**Total Tasks**: 36  
**User Stories**: 3 (US1: P1, US2: P2, US3: P3)  
**Parallel Opportunities**: 12 tasks can run in parallel  
**MVP Scope**: User Story 1 (basic value transformation - 19 tasks)

## Implementation Strategy

This feature follows an **incremental delivery approach** where each user story is independently testable and delivers value:

1. **MVP (US1)**: Basic value transformation with structure preservation - delivers core functionality
2. **US2**: Composition law verification - ensures correctness for complex transformations
3. **US3**: Identity law verification - ensures mathematical correctness

Each user story can be developed, tested, and delivered independently.

## Dependencies & Execution Order

### Story Completion Order

```
Setup (Phase 1)
    ↓
Foundational (Phase 2) ← Must complete before any user story
    ↓
    ├─→ US1 (P1) ← MVP, no dependencies on other stories
    ├─→ US2 (P2) ← Can start after US1 complete  
    └─→ US3 (P3) ← Can start after US1 complete
         ↓
    Polish (Phase 6)
```

### Parallel Execution Opportunities

- **Phase 1**: All setup tasks can run in parallel
- **Phase 2**: Tests can run in parallel with implementation (TDD)
- **US1**: Property tests can run in parallel with unit tests
- **US2/US3**: Can be developed in parallel after US1 complete

---

## Phase 1: Setup & Documentation

**Goal**: Initialize documentation and verify prerequisites

### Tasks

- [x] T001 [P] Verify Pattern<V> type exists in crates/pattern-core/src/pattern.rs
- [x] T002 [P] Verify proptest dependency available in Cargo.toml
- [x] T003 [P] Review gram-hs Functor implementation at ../gram-hs/libs/pattern/src/Pattern/Core.hs lines 536-617
- [x] T004 [P] Review gram-hs Functor tests at ../gram-hs/libs/pattern/tests/Spec/Pattern/Properties.hs lines 176-203

**Acceptance**: All prerequisites verified, gram-hs implementation reviewed

---

## Phase 2: Foundational Implementation

**Goal**: Implement core `map` method that all user stories will use

### Tasks

- [x] T005 Add method signature for Pattern::map in crates/pattern-core/src/pattern.rs
- [x] T006 Implement map method body with value transformation in crates/pattern-core/src/pattern.rs
- [x] T007 Implement recursive element transformation in crates/pattern-core/src/pattern.rs
- [x] T008 Add comprehensive doc comments with examples to map method in crates/pattern-core/src/pattern.rs
- [x] T009 Export map functionality if needed in crates/pattern-core/src/lib.rs
- [x] T010 Verify WASM compilation: cargo build --target wasm32-unknown-unknown

**Acceptance**: Pattern::map method compiles, documented, WASM compatible

**Blocking**: This phase MUST complete before starting any user story phase

---

## Phase 3: User Story 1 - Transform Pattern Values While Preserving Structure (P1)

**Story Goal**: Enable developers to transform all values in a pattern while preserving structure

**Independent Test Criteria**:
- Atomic patterns can be transformed
- Nested patterns preserve element count
- Deep nesting preserves depth
- Mixed structures preserve shape
- Type transformations work (String → Int, etc.)

### Property-Based Tests

- [x] T011 [P] [US1] Create test file crates/pattern-core/tests/functor_laws.rs
- [x] T012 [P] [US1] Add proptest imports and pattern generators in crates/pattern-core/tests/functor_laws.rs
- [x] T013 [P] [US1] Implement structure_preservation property test in crates/pattern-core/tests/functor_laws.rs
- [x] T014 [US1] Run structure preservation test: cargo test functor_laws::structure_preservation

### Unit Tests

- [x] T015 [P] [US1] Implement test_map_atomic_pattern in crates/pattern-core/tests/functor_laws.rs
- [x] T016 [P] [US1] Implement test_map_nested_pattern in crates/pattern-core/tests/functor_laws.rs
- [x] T017 [P] [US1] Implement test_map_type_conversion in crates/pattern-core/tests/functor_laws.rs
- [x] T018 [P] [US1] Implement test_map_preserves_structure in crates/pattern-core/tests/functor_laws.rs
- [x] T019 [US1] Run all US1 tests: cargo test functor_laws

**US1 Deliverable**: Working map method with structure preservation verified

**Independent Test**: Run `cargo test functor_laws` - all tests pass, structure metrics unchanged

---

## Phase 4: User Story 2 - Compose Multiple Transformations Safely (P2)

**Story Goal**: Verify composition law holds, enabling safe refactoring

**Independent Test Criteria**:
- `pattern.map(|x| g(&f(x)))` equals `pattern.map(f).map(g)`
- Works with numeric transformations
- Works with string transformations
- Works with nested patterns

**Dependencies**: Requires US1 (map method) complete

### Property-Based Tests

- [x] T020 [P] [US2] Implement composition_law property test in crates/pattern-core/tests/functor_laws.rs
- [x] T021 [US2] Run composition law test: cargo test functor_laws::composition_law
- [x] T022 [P] [US2] Add test for composition with type transformations in crates/pattern-core/tests/functor_laws.rs

**US2 Deliverable**: Composition law verified through property tests

**Independent Test**: Run `cargo test functor_laws::composition` - 100+ random cases pass

---

## Phase 5: User Story 3 - Apply Identity Transformation Without Side Effects (P3)

**Story Goal**: Verify identity law holds, ensuring mathematical correctness

**Independent Test Criteria**:
- `pattern.map(|x| x.clone())` equals `pattern`
- Works with various value types (strings, integers, custom types)
- Preserves equality

**Dependencies**: Requires US1 (map method) complete

### Property-Based Tests

- [x] T023 [P] [US3] Implement identity_law property test in crates/pattern-core/tests/functor_laws.rs
- [x] T024 [US3] Run identity law test: cargo test functor_laws::identity_law

**US3 Deliverable**: Identity law verified through property tests

**Independent Test**: Run `cargo test functor_laws::identity` - 100+ random cases pass

---

## Phase 6: Polish & Cross-Cutting Concerns

**Goal**: Finalize implementation, documentation, and integration

### Performance Verification

- [x] T025 Benchmark transformation with 1000-node patterns in crates/pattern-core/benches/ (if benchmark infrastructure exists)
- [x] T026 Test stack safety with 100+ nesting levels in crates/pattern-core/tests/functor_laws.rs
- [x] T027 Profile memory usage with 10,000-element patterns (manual verification)

### Documentation & Integration

- [x] T028 [P] Update crate documentation in crates/pattern-core/src/lib.rs
- [x] T029 [P] Add examples to crates/pattern-core/README.md
- [x] T030 [P] Update TODO.md to mark feature 008 complete
- [x] T031 Verify all success criteria from specs/008-functor-instance/spec.md

### Final Verification

- [x] T032 Run full test suite: cargo test
- [x] T033 Run clippy: cargo clippy --all-targets --all-features
- [x] T034 Run rustfmt: cargo fmt --check
- [x] T035 Verify WASM compilation: cargo build --target wasm32-unknown-unknown
- [x] T036 Review and update PORTING_GUIDE.md examples if needed

**Phase Complete**: All tests pass, documentation complete, feature ready for merge

---

## Task Execution Guide

### For MVP (User Story 1 Only)

To deliver minimum viable product:
1. Complete Phase 1 (Setup): T001-T004
2. Complete Phase 2 (Foundational): T005-T010
3. Complete Phase 3 (US1): T011-T019
4. Skip to basic polish: T032-T035

**MVP Delivers**: Working `map` method with verified structure preservation

### For Full Feature

Complete all phases in order:
1. Phase 1: Setup (T001-T004)
2. Phase 2: Foundational (T005-T010)
3. Phase 3: US1 (T011-T019)
4. Phase 4: US2 (T020-T022) - Can parallelize with Phase 5
5. Phase 5: US3 (T023-T024) - Can parallelize with Phase 4
6. Phase 6: Polish (T025-T036)

### Parallel Execution Examples

**Per User Story (US1)**:
```
# Start US1
T011 (property test setup) || T015 (unit test 1) || T016 (unit test 2)
    ↓                           ↓                     ↓
T013 (property test impl) || T017 (unit test 3) || T018 (unit test 4)
    ↓                           ↓                     ↓
         T014 + T019 (run all tests)
```

**Across User Stories (after US1 complete)**:
```
US2 (T020-T022) || US3 (T023-T024)
        ↓                ↓
    Phase 6 Polish (T025-T036)
```

---

## Success Criteria Verification

From [spec.md](./spec.md):

- [ ] **SC-001**: All functor law property tests pass with at least 100 randomly generated test cases each (T020, T023, T024)
- [ ] **SC-002**: Transformations complete on patterns with 1000 nodes in under 10 milliseconds (T025)
- [ ] **SC-003**: Transformations complete on patterns with 100 nesting levels without stack overflow (T026)
- [ ] **SC-004**: Code can transform patterns between different value types without type errors (T017)
- [ ] **SC-005**: 100% of existing gram-hs functor tests are ported and pass (T011-T024)
- [ ] **SC-006**: Functor implementation compiles for WASM target without errors (T010, T035)
- [ ] **SC-007**: Pattern structures with 10,000 elements can be transformed without exceeding 100MB memory overhead (T027)

---

## File Checklist

Files to be created/modified:

### Implementation
- [ ] `crates/pattern-core/src/pattern.rs` - Add map method (T005-T008)
- [ ] `crates/pattern-core/src/lib.rs` - Export map if needed (T009)

### Tests
- [ ] `crates/pattern-core/tests/functor_laws.rs` - New file for all tests (T011-T024, T026)
- [ ] `crates/pattern-core/benches/` - Benchmarks if infrastructure exists (T025)

### Documentation
- [ ] `crates/pattern-core/README.md` - Add examples (T029)
- [ ] `TODO.md` - Mark feature 008 complete (T030)

---

## Notes

### Implementation Approach

Per [research.md](./research.md) and [IMPLEMENTATION_NOTES.md](./IMPLEMENTATION_NOTES.md):
- Use direct `map` method (no Functor trait)
- Method signature: `pub fn map<W, F>(self, f: F) -> Pattern<W> where F: Fn(&V) -> W`
- Capture `&f` in recursive calls for efficiency
- Follow Rust standard library conventions

### Testing Philosophy

From [plan.md](./plan.md):
- Property-based tests are PRIMARY (verify functor laws)
- Unit tests are SUPPLEMENTARY (verify specific behaviors)
- Each user story has independent test criteria
- Tests verify behavior, not syntax

### Behavioral Equivalence

Per [PORTING_GUIDE.md](../../../PORTING_GUIDE.md):
- Port concepts and behavior from gram-hs, not syntax
- Maintain functor laws through property tests
- Use idiomatic Rust patterns
- Document relationship to Haskell implementation

