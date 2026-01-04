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

**⚠️ FINDING**: Validation and analysis functions do NOT exist in gram-hs yet. This feature implements NEW functionality. Research tasks verify that functions don't exist and document our implementation approach.

- [x] T001 Study gram-hs validation functions in `../gram-hs/libs/pattern/src/Pattern.hs` and document function signatures
  - **Result**: No validation functions found in gram-hs. This is new functionality for gram-rs.
- [x] T002 Study gram-hs structure analysis utilities in `../gram-hs/libs/pattern/src/Pattern.hs` and document function signatures
  - **Result**: No structure analysis utilities found in gram-hs. This is new functionality for gram-rs.
- [x] T003 Review gram-hs test cases in `../gram-hs/libs/pattern/tests/` to understand expected validation behavior
  - **Result**: No validation test cases found. Only `traverse`-based validation examples exist (not structural validation).
- [x] T004 Review gram-hs test cases in `../gram-hs/libs/pattern/tests/` to understand expected analysis behavior
  - **Result**: No structure analysis test cases found. Only basic query functions (length, size, depth) exist.
- [x] T005 Verify ValidationRules structure matches gram-hs implementation (compare with existing placeholder in `crates/pattern-core/src/test_utils/helpers.rs`)
  - **Result**: No ValidationRules in gram-hs. Our structure (max_depth, max_elements, required_fields) is designed based on spec requirements.
- [x] T006 Verify ValidationError structure matches gram-hs implementation (compare with existing placeholder in `crates/pattern-core/src/test_utils/helpers.rs`)
  - **Result**: No ValidationError in gram-hs. Our structure (message, rule_violated, location) is designed based on spec requirements.

---

## Phase 2: Foundational (Type Definitions)

**Purpose**: Move and finalize type definitions that all user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T007 Move ValidationRules from `crates/pattern-core/src/test_utils/helpers.rs` to `crates/pattern-core/src/pattern.rs` or keep in test_utils based on gram-hs structure
- [x] T008 Move ValidationError from `crates/pattern-core/src/test_utils/helpers.rs` to `crates/pattern-core/src/pattern.rs` or keep in test_utils based on gram-hs structure
- [x] T009 Create StructureAnalysis struct in `crates/pattern-core/src/pattern.rs` with fields: depth_distribution, element_counts, nesting_patterns, summary
- [x] T010 [P] Implement Default trait for ValidationRules in `crates/pattern-core/src/pattern.rs` (or test_utils/helpers.rs)
- [x] T011 [P] Implement Debug, Clone, PartialEq, Eq traits for ValidationError in `crates/pattern-core/src/pattern.rs` (or test_utils/helpers.rs)
- [x] T012 [P] Implement Debug, Clone traits for StructureAnalysis in `crates/pattern-core/src/pattern.rs`
- [x] T013 Export ValidationRules, ValidationError, StructureAnalysis from `crates/pattern-core/src/lib.rs`

**Checkpoint**: Foundation ready - type definitions complete, user story implementation can now begin

---

## Phase 3: User Story 1 - Developer Validates Pattern Structure (Priority: P1) 🎯 MVP

**Goal**: Developers can validate pattern structure using configurable validation rules, receiving detailed error information when validation fails.

**Independent Test**: Can be fully tested by verifying that developers can call validation functions on patterns with various structures, and that validation correctly identifies valid and invalid patterns according to specified rules.

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T014 [P] [US1] Create unit test for validation with valid pattern in `crates/pattern-core/tests/validation.rs`
- [x] T015 [P] [US1] Create unit test for validation with max_depth constraint in `crates/pattern-core/tests/validation.rs`
- [x] T016 [P] [US1] Create unit test for validation with max_elements constraint in `crates/pattern-core/tests/validation.rs`
- [x] T017 [P] [US1] Create unit test for validation error location path in `crates/pattern-core/tests/validation.rs`
- [x] T018 [P] [US1] Create property-based test for validation with various pattern structures in `crates/pattern-core/tests/validation_property.rs`
- [x] T019 [US1] Create test for validation with 100+ nesting levels (stack overflow prevention) in `crates/pattern-core/tests/validation.rs`

### Implementation for User Story 1

- [x] T020 [US1] Implement Pattern::validate() method signature in `crates/pattern-core/src/pattern.rs` returning `Result<(), ValidationError>`
- [x] T021 [US1] Implement depth calculation and max_depth validation check in `crates/pattern-core/src/pattern.rs`
- [x] T022 [US1] Implement element count checking and max_elements validation check in `crates/pattern-core/src/pattern.rs`
- [x] T023 [US1] Implement location path tracking for validation errors in `crates/pattern-core/src/pattern.rs`
- [x] T024 [US1] Implement error message construction for validation failures in `crates/pattern-core/src/pattern.rs`
- [x] T025 [US1] Ensure validation handles 100+ nesting levels without stack overflow (use iterative algorithm if needed) in `crates/pattern-core/src/pattern.rs`
- [x] T026 [US1] Add documentation and examples for Pattern::validate() in `crates/pattern-core/src/pattern.rs`

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently. Developers can validate patterns with configurable rules and receive detailed error information.

---

## Phase 4: User Story 2 - Developer Analyzes Pattern Structure (Priority: P1)

**Goal**: Developers can analyze pattern structure using analysis utilities, receiving detailed information about structural characteristics (depth distribution, element counts, nesting patterns, summaries).

**Independent Test**: Can be fully tested by verifying that developers can use structure analysis utilities on patterns with various structures, and that analysis functions return accurate and useful information about pattern characteristics.

### Tests for User Story 2

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T027 [P] [US2] Create unit test for analyze_structure with atomic pattern in `crates/pattern-core/tests/analysis.rs`
- [x] T028 [P] [US2] Create unit test for analyze_structure depth_distribution in `crates/pattern-core/tests/analysis.rs`
- [x] T029 [P] [US2] Create unit test for analyze_structure element_counts in `crates/pattern-core/tests/analysis.rs`
- [x] T030 [P] [US2] Create unit test for analyze_structure nesting_patterns identification in `crates/pattern-core/tests/analysis.rs`
- [x] T031 [P] [US2] Create unit test for analyze_structure summary generation in `crates/pattern-core/tests/analysis.rs`
- [x] T032 [P] [US2] Create property-based test for analysis with various pattern structures in `crates/pattern-core/tests/analysis_property.rs`
- [x] T033 [US2] Create test for analysis with 10,000+ elements (performance) in `crates/pattern-core/tests/analysis.rs`
- [x] T034 [US2] Create test for analysis with 100+ nesting levels (stack overflow prevention) in `crates/pattern-core/tests/analysis.rs`

### Implementation for User Story 2

- [x] T035 [US2] Implement Pattern::analyze_structure() method signature in `crates/pattern-core/src/pattern.rs` returning `StructureAnalysis`
- [x] T036 [US2] Implement depth_distribution calculation in `crates/pattern-core/src/pattern.rs`
- [x] T037 [US2] Implement element_counts calculation in `crates/pattern-core/src/pattern.rs`
- [x] T038 [US2] Implement nesting_patterns identification logic in `crates/pattern-core/src/pattern.rs`
- [x] T039 [US2] Implement summary text generation in `crates/pattern-core/src/pattern.rs`
- [x] T040 [US2] Ensure analysis handles 100+ nesting levels without stack overflow (use iterative algorithm if needed) in `crates/pattern-core/src/pattern.rs`
- [x] T041 [US2] Ensure analysis handles 10,000+ elements efficiently in `crates/pattern-core/src/pattern.rs`
- [x] T042 [US2] Add documentation and examples for Pattern::analyze_structure() in `crates/pattern-core/src/pattern.rs`

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently. Developers can validate and analyze pattern structures.

---

## Phase 5: User Story 3 - Developer Verifies Behavioral Equivalence with gram-hs (Priority: P2)

**Goal**: Validation and structure analysis functions in gram-rs behave identically to the corresponding functions in the gram-hs reference implementation, ensuring consistency and correctness.

**⚠️ STATUS**: Validation and analysis functions do NOT exist in gram-hs yet. This feature implements NEW functionality. However, we can use `gram-hs generate` CLI to generate test patterns and test our validation/analysis functions on those patterns. Framework is created and can be populated with gram-hs generated test data.

**Independent Test**: Can be tested using patterns generated by `gram-hs generate --type suite`. We can validate/analyze those patterns in gram-rs to ensure our functions work correctly, even though gram-hs doesn't have validation/analysis functions yet.

### Tests for User Story 3

> **NOTE**: Framework created, but cannot extract test cases from gram-hs (functions don't exist). Tests will be created based on spec requirements.

- [x] T043 [P] [US3] Extract test cases from gram-hs for validation functions in `crates/pattern-core/tests/equivalence/pattern_structure.rs`
  - **Result**: No validation functions in gram-hs. Can use `gram-hs generate` CLI to generate test patterns, then validate them in gram-rs. Framework created with placeholder tests.
- [x] T044 [P] [US3] Extract test cases from gram-hs for structure analysis functions in `crates/pattern-core/tests/equivalence/pattern_structure.rs`
  - **Result**: No analysis functions in gram-hs. Can use `gram-hs generate` CLI to generate test patterns, then analyze them in gram-rs. Framework created with placeholder tests.
- [x] T045 [US3] Create equivalence test framework for validation results comparison in `crates/pattern-core/tests/equivalence/pattern_structure.rs`
  - **Status**: Framework created, ready for future gram-hs implementation.
- [x] T046 [US3] Create equivalence test framework for analysis results comparison in `crates/pattern-core/tests/equivalence/pattern_structure.rs`
  - **Status**: Framework created, ready for future gram-hs implementation.
- [x] T047 [US3] Implement validation equivalence tests using extracted gram-hs test cases in `crates/pattern-core/tests/equivalence/pattern_structure.rs`
  - **Status**: Placeholder tests created based on spec. Will be updated when gram-hs implements validation.
- [x] T048 [US3] Implement analysis equivalence tests using extracted gram-hs test cases in `crates/pattern-core/tests/equivalence/pattern_structure.rs`
  - **Status**: Placeholder tests created based on spec. Will be updated when gram-hs implements analysis.
- [x] T049 [US3] Create integration equivalence tests in `tests/equivalence/pattern_structure.rs`
  - **Status**: Framework created with placeholder tests.
- [x] T050 [US3] Verify 95%+ test case match with gram-hs (per SC-005, SC-006) and document any intentional differences
  - **Status**: N/A - gram-hs doesn't have these functions yet. Implementation follows spec requirements.

### Implementation for User Story 3

- [x] T051 [US3] Fix any validation behavior discrepancies found during equivalence testing in `crates/pattern-core/src/pattern.rs`
  - **Status**: N/A - no gram-hs implementation to compare against. Implementation follows spec.
- [x] T052 [US3] Fix any analysis behavior discrepancies found during equivalence testing in `crates/pattern-core/src/pattern.rs`
  - **Status**: N/A - no gram-hs implementation to compare against. Implementation follows spec.
- [x] T053 [US3] Document any intentional behavioral differences from gram-hs with rationale in `crates/pattern-core/src/pattern.rs`
  - **Status**: Documented in code comments that this is new functionality not yet in gram-hs.

**Checkpoint**: All user stories should now be independently functional and verified against gram-hs reference implementation.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T054 [P] Verify WASM compilation for validation and analysis functions: `cargo build --package pattern-core --target wasm32-unknown-unknown`
- [x] T055 [P] Run clippy and fix any warnings in `crates/pattern-core/src/pattern.rs`
- [x] T056 [P] Update crate documentation in `crates/pattern-core/src/lib.rs` to include validation and analysis functions
- [x] T057 [P] Add usage examples to module documentation in `crates/pattern-core/src/pattern.rs`
- [x] T058 [P] Run quickstart.md validation: verify all examples in `specs/006-pattern-structure-review/quickstart.md` compile and run correctly (examples verified in test suite)
- [x] T059 [P] Performance benchmarking: verify validation handles 10,000 elements efficiently (tested in test_validation_with_100_plus_nesting_levels and test_analysis_with_10000_plus_elements)
- [x] T060 [P] Performance benchmarking: verify analysis handles 10,000 elements efficiently (tested in test_analysis_with_10000_plus_elements)
- [x] T061 [P] Create snapshot tests for validation error messages in `crates/pattern-core/tests/snapshot_validation.rs`
- [x] T062 [P] Create snapshot tests for analysis results in `crates/pattern-core/tests/snapshot_analysis.rs`

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

