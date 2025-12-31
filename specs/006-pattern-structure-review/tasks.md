# Tasks: Pattern Structure Validation

**Input**: Design documents from `/specs/006-pattern-structure-review/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Tests are included as they are critical for verifying behavioral equivalence with gram-hs (User Story 3).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., [US1], [US2], [US3])
- Include exact file paths in descriptions

## Path Conventions

- **Library crate**: `crates/pattern-core/src/`, `crates/pattern-core/tests/`
- **Integration tests**: `tests/equivalence/`
- Paths follow the multi-crate workspace structure from plan.md

---

## Phase 1: Setup (Research & Verification)

**Purpose**: Study gram-hs reference implementation to understand exact function signatures and behavior

- [ ] T001 Study gram-hs validation functions in `../gram-hs/libs/pattern/src/Pattern.hs` and document function signatures
- [ ] T002 Study gram-hs structure analysis utilities in `../gram-hs/libs/pattern/src/Pattern.hs` and document function signatures
- [ ] T003 Review gram-hs test cases in `../gram-hs/libs/pattern/tests/` to understand expected validation behavior
- [ ] T004 Review gram-hs test cases in `../gram-hs/libs/pattern/tests/` to understand expected analysis behavior
- [ ] T005 Verify ValidationRules structure matches gram-hs implementation (compare with existing placeholder in `crates/pattern-core/src/test_utils/helpers.rs`)
- [ ] T006 Verify ValidationError structure matches gram-hs implementation (compare with existing placeholder in `crates/pattern-core/src/test_utils/helpers.rs`)

---

## Phase 2: Foundational (Type Definitions)

**Purpose**: Move and finalize type definitions that all user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T007 Move ValidationRules from `crates/pattern-core/src/test_utils/helpers.rs` to `crates/pattern-core/src/pattern.rs` or keep in test_utils based on gram-hs structure
- [ ] T008 Move ValidationError from `crates/pattern-core/src/test_utils/helpers.rs` to `crates/pattern-core/src/pattern.rs` or keep in test_utils based on gram-hs structure
- [ ] T009 Create StructureAnalysis struct in `crates/pattern-core/src/pattern.rs` with fields: depth_distribution, element_counts, nesting_patterns, summary
- [ ] T010 [P] Implement Default trait for ValidationRules in `crates/pattern-core/src/pattern.rs` (or test_utils/helpers.rs)
- [ ] T011 [P] Implement Debug, Clone, PartialEq, Eq traits for ValidationError in `crates/pattern-core/src/pattern.rs` (or test_utils/helpers.rs)
- [ ] T012 [P] Implement Debug, Clone traits for StructureAnalysis in `crates/pattern-core/src/pattern.rs`
- [ ] T013 Export ValidationRules, ValidationError, StructureAnalysis from `crates/pattern-core/src/lib.rs`

**Checkpoint**: Foundation ready - type definitions complete, user story implementation can now begin

---

## Phase 3: User Story 1 - Developer Validates Pattern Structure (Priority: P1) 🎯 MVP

**Goal**: Developers can validate pattern structure using configurable validation rules, receiving detailed error information when validation fails.

**Independent Test**: Can be fully tested by verifying that developers can call validation functions on patterns with various structures, and that validation correctly identifies valid and invalid patterns according to specified rules.

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T014 [P] [US1] Create unit test for validation with valid pattern in `crates/pattern-core/tests/unit/validation.rs`
- [ ] T015 [P] [US1] Create unit test for validation with max_depth constraint in `crates/pattern-core/tests/unit/validation.rs`
- [ ] T016 [P] [US1] Create unit test for validation with max_elements constraint in `crates/pattern-core/tests/unit/validation.rs`
- [ ] T017 [P] [US1] Create unit test for validation error location path in `crates/pattern-core/tests/unit/validation.rs`
- [ ] T018 [P] [US1] Create property-based test for validation with various pattern structures in `crates/pattern-core/tests/property/validation.rs`
- [ ] T019 [US1] Create test for validation with 100+ nesting levels (stack overflow prevention) in `crates/pattern-core/tests/unit/validation.rs`

### Implementation for User Story 1

- [ ] T020 [US1] Implement Pattern::validate() method signature in `crates/pattern-core/src/pattern.rs` returning `Result<(), ValidationError>`
- [ ] T021 [US1] Implement depth calculation and max_depth validation check in `crates/pattern-core/src/pattern.rs`
- [ ] T022 [US1] Implement element count checking and max_elements validation check in `crates/pattern-core/src/pattern.rs`
- [ ] T023 [US1] Implement location path tracking for validation errors in `crates/pattern-core/src/pattern.rs`
- [ ] T024 [US1] Implement error message construction for validation failures in `crates/pattern-core/src/pattern.rs`
- [ ] T025 [US1] Ensure validation handles 100+ nesting levels without stack overflow (use iterative algorithm if needed) in `crates/pattern-core/src/pattern.rs`
- [ ] T026 [US1] Add documentation and examples for Pattern::validate() in `crates/pattern-core/src/pattern.rs`

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently. Developers can validate patterns with configurable rules and receive detailed error information.

---

## Phase 4: User Story 2 - Developer Analyzes Pattern Structure (Priority: P1)

**Goal**: Developers can analyze pattern structure using analysis utilities, receiving detailed information about structural characteristics (depth distribution, element counts, nesting patterns, summaries).

**Independent Test**: Can be fully tested by verifying that developers can use structure analysis utilities on patterns with various structures, and that analysis functions return accurate and useful information about pattern characteristics.

### Tests for User Story 2

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T027 [P] [US2] Create unit test for analyze_structure with atomic pattern in `crates/pattern-core/tests/unit/analysis.rs`
- [ ] T028 [P] [US2] Create unit test for analyze_structure depth_distribution in `crates/pattern-core/tests/unit/analysis.rs`
- [ ] T029 [P] [US2] Create unit test for analyze_structure element_counts in `crates/pattern-core/tests/unit/analysis.rs`
- [ ] T030 [P] [US2] Create unit test for analyze_structure nesting_patterns identification in `crates/pattern-core/tests/unit/analysis.rs`
- [ ] T031 [P] [US2] Create unit test for analyze_structure summary generation in `crates/pattern-core/tests/unit/analysis.rs`
- [ ] T032 [P] [US2] Create property-based test for analysis with various pattern structures in `crates/pattern-core/tests/property/analysis.rs`
- [ ] T033 [US2] Create test for analysis with 10,000+ elements (performance) in `crates/pattern-core/tests/unit/analysis.rs`
- [ ] T034 [US2] Create test for analysis with 100+ nesting levels (stack overflow prevention) in `crates/pattern-core/tests/unit/analysis.rs`

### Implementation for User Story 2

- [ ] T035 [US2] Implement Pattern::analyze_structure() method signature in `crates/pattern-core/src/pattern.rs` returning `StructureAnalysis`
- [ ] T036 [US2] Implement depth_distribution calculation in `crates/pattern-core/src/pattern.rs`
- [ ] T037 [US2] Implement element_counts calculation in `crates/pattern-core/src/pattern.rs`
- [ ] T038 [US2] Implement nesting_patterns identification logic in `crates/pattern-core/src/pattern.rs`
- [ ] T039 [US2] Implement summary text generation in `crates/pattern-core/src/pattern.rs`
- [ ] T040 [US2] Ensure analysis handles 100+ nesting levels without stack overflow (use iterative algorithm if needed) in `crates/pattern-core/src/pattern.rs`
- [ ] T041 [US2] Ensure analysis handles 10,000+ elements efficiently in `crates/pattern-core/src/pattern.rs`
- [ ] T042 [US2] Add documentation and examples for Pattern::analyze_structure() in `crates/pattern-core/src/pattern.rs`

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently. Developers can validate and analyze pattern structures.

---

## Phase 5: User Story 3 - Developer Verifies Behavioral Equivalence with gram-hs (Priority: P2)

**Goal**: Validation and structure analysis functions in gram-rs behave identically to the corresponding functions in the gram-hs reference implementation, ensuring consistency and correctness.

**Independent Test**: Can be fully tested by running equivalent validation and analysis operations on identical patterns in both gram-rs and gram-hs, and verifying that validation results and analysis outputs match.

### Tests for User Story 3

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T043 [P] [US3] Extract test cases from gram-hs for validation functions in `crates/pattern-core/tests/equivalence/pattern_structure.rs`
- [ ] T044 [P] [US3] Extract test cases from gram-hs for structure analysis functions in `crates/pattern-core/tests/equivalence/pattern_structure.rs`
- [ ] T045 [US3] Create equivalence test framework for validation results comparison in `crates/pattern-core/tests/equivalence/pattern_structure.rs`
- [ ] T046 [US3] Create equivalence test framework for analysis results comparison in `crates/pattern-core/tests/equivalence/pattern_structure.rs`
- [ ] T047 [US3] Implement validation equivalence tests using extracted gram-hs test cases in `crates/pattern-core/tests/equivalence/pattern_structure.rs`
- [ ] T048 [US3] Implement analysis equivalence tests using extracted gram-hs test cases in `crates/pattern-core/tests/equivalence/pattern_structure.rs`
- [ ] T049 [US3] Create integration equivalence tests in `tests/equivalence/pattern_structure.rs`
- [ ] T050 [US3] Verify 95%+ test case match with gram-hs (per SC-005, SC-006) and document any intentional differences

### Implementation for User Story 3

- [ ] T051 [US3] Fix any validation behavior discrepancies found during equivalence testing in `crates/pattern-core/src/pattern.rs`
- [ ] T052 [US3] Fix any analysis behavior discrepancies found during equivalence testing in `crates/pattern-core/src/pattern.rs`
- [ ] T053 [US3] Document any intentional behavioral differences from gram-hs with rationale in `crates/pattern-core/src/pattern.rs`

**Checkpoint**: All user stories should now be independently functional and verified against gram-hs reference implementation.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T054 [P] Verify WASM compilation for validation and analysis functions: `cargo build --package pattern-core --target wasm32-unknown-unknown`
- [ ] T055 [P] Run clippy and fix any warnings in `crates/pattern-core/src/pattern.rs`
- [ ] T056 [P] Update crate documentation in `crates/pattern-core/src/lib.rs` to include validation and analysis functions
- [ ] T057 [P] Add usage examples to module documentation in `crates/pattern-core/src/pattern.rs`
- [ ] T058 [P] Run quickstart.md validation: verify all examples in `specs/006-pattern-structure-review/quickstart.md` compile and run correctly
- [ ] T059 [P] Performance benchmarking: verify validation handles 10,000 elements efficiently
- [ ] T060 [P] Performance benchmarking: verify analysis handles 10,000 elements efficiently
- [ ] T061 [P] Create snapshot tests for validation error messages in `crates/pattern-core/tests/snapshot/validation.rs`
- [ ] T062 [P] Create snapshot tests for analysis results in `crates/pattern-core/tests/snapshot/analysis.rs`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion (need to know where types should live) - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User Story 1 and User Story 2 can proceed in parallel after Foundational (both P1)
  - User Story 3 depends on User Stories 1 and 2 completion (needs functions to test)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories (can run in parallel with US1)
- **User Story 3 (P2)**: Depends on User Stories 1 and 2 completion - Needs validation and analysis functions to test for equivalence

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Type definitions before method implementations
- Core validation/analysis logic before error handling and edge cases
- Basic functionality before performance optimizations
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks (T001-T006) can run in parallel (different aspects of research)
- Foundational tasks T010, T011, T012 can run in parallel (different type implementations)
- User Stories 1 and 2 can run in parallel after Foundational phase (different functionality)
- All tests for a user story marked [P] can run in parallel
- Polish phase tasks marked [P] can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Create unit test for validation with valid pattern in crates/pattern-core/tests/unit/validation.rs"
Task: "Create unit test for validation with max_depth constraint in crates/pattern-core/tests/unit/validation.rs"
Task: "Create unit test for validation with max_elements constraint in crates/pattern-core/tests/unit/validation.rs"
Task: "Create unit test for validation error location path in crates/pattern-core/tests/unit/validation.rs"
Task: "Create property-based test for validation with various pattern structures in crates/pattern-core/tests/property/validation.rs"
```

---

## Parallel Example: User Stories 1 and 2

```bash
# After Foundational phase, User Stories 1 and 2 can run in parallel:

# Developer A: User Story 1
Task: "Create unit test for validation with valid pattern..."
Task: "Implement Pattern::validate() method signature..."
Task: "Implement depth calculation and max_depth validation check..."

# Developer B: User Story 2  
Task: "Create unit test for analyze_structure with atomic pattern..."
Task: "Implement Pattern::analyze_structure() method signature..."
Task: "Implement depth_distribution calculation..."
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (research gram-hs implementation)
2. Complete Phase 2: Foundational (type definitions)
3. Complete Phase 3: User Story 1 (validation functions)
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Verify equivalence → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (validation)
   - Developer B: User Story 2 (analysis)
3. Once User Stories 1 and 2 are complete:
   - Developer C: User Story 3 (equivalence testing)
4. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Study gram-hs implementation first (Phase 1) to ensure correct function signatures
- ValidationRules and ValidationError already exist as placeholders - may need to move or extend
- All functions must work generically with any value type `V`
- Performance requirements: 100+ nesting levels, 10,000+ elements
- WASM compilation must succeed (verify in Polish phase)

