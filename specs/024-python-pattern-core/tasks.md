# Tasks: Python Pattern-Core Bindings

**Input**: Design documents from `/specs/024-python-pattern-core/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are included as requested in the feature specification and user input.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- Python bindings: `crates/pattern-core/src/python.rs`
- Python tests: `crates/pattern-core/tests/python/`
- Type stubs: `crates/pattern-core/pattern_core/__init__.pyi`
- Examples: `examples/pattern-core-python/`
- Documentation: `docs/python-usage.md`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Add pyo3 dependency with python feature flag to crates/pattern-core/Cargo.toml
- [x] T002 Create crates/pattern-core/pyproject.toml with maturin configuration
- [x] T003 [P] Create crates/pattern-core/tests/python/ directory structure
- [x] T004 [P] Create crates/pattern-core/pattern_core/ directory for type stubs
- [x] T005 [P] Create examples/pattern-core-python/ directory structure

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T006 Create crates/pattern-core/src/python.rs module with feature gate
- [x] T007 [P] Implement error conversion helpers in crates/pattern-core/src/python.rs (Rust errors → Python exceptions)
- [x] T008 [P] Implement type conversion helpers in crates/pattern-core/src/python.rs (Python ↔ Rust types)
- [x] T009 Implement Python module initialization function in crates/pattern-core/src/python.rs
- [x] T010 Update crates/pattern-core/src/lib.rs to conditionally re-export python module when feature enabled
- [x] T011 [P] Create crates/pattern-core/tests/python/conftest.py for pytest configuration

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Construct Patterns Programmatically (Priority: P1) 🎯 MVP

**Goal**: Enable Python developers to create Pattern instances programmatically, including atomic patterns, nested patterns, and patterns with Subject values.

**Independent Test**: Create a simple atomic pattern and a nested pattern with Subject values, verify structure and values are correct. Can be tested independently by running Python script that constructs patterns and accesses their attributes.

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T012 [P] [US1] Create test_pattern_construction in crates/pattern-core/tests/python/test_pattern.py
- [x] T013 [P] [US1] Create test_subject_construction in crates/pattern-core/tests/python/test_subject.py
- [x] T014 [P] [US1] Create test_pattern_subject_construction in crates/pattern-core/tests/python/test_pattern.py

### Implementation for User Story 1

- [x] T015 [US1] Implement Value Python class with all variants in crates/pattern-core/src/python.rs
- [x] T016 [US1] Implement Value automatic conversion from Python types in crates/pattern-core/src/python.rs
- [x] T017 [US1] Implement Subject Python class with identity, labels, properties in crates/pattern-core/src/python.rs
- [x] T018 [US1] Implement Subject methods (add_label, remove_label, has_label, get_property, set_property, remove_property) in crates/pattern-core/src/python.rs
- [x] T019 [US1] Implement Pattern Python class with value and elements attributes in crates/pattern-core/src/python.rs
- [x] T020 [US1] Implement Pattern.point static method in crates/pattern-core/src/python.rs
- [x] T021 [US1] Implement Pattern.pattern static method in crates/pattern-core/src/python.rs
- [x] T022 [US1] Implement Pattern.from_list static method in crates/pattern-core/src/python.rs
- [x] T023 [US1] Implement PatternSubject Python class extending Pattern in crates/pattern-core/src/python.rs
- [x] T024 [US1] Register all Python classes in module initialization in crates/pattern-core/src/python.rs
- [x] T025 [US1] Add docstrings to all Python classes and methods in crates/pattern-core/src/python.rs

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently. Python developers can create Pattern and Subject instances programmatically.

---

## Phase 4: User Story 2 - Perform Pattern Operations (Priority: P2)

**Goal**: Enable Python developers to perform functional programming operations on patterns including transformations, queries, structural analysis, and combination operations.

**Independent Test**: Create a pattern, apply map/filter operations, and verify results match expected transformations. Can be tested independently by running Python script that performs operations on patterns.

### Tests for User Story 2

- [x] T026 [P] [US2] Create test_pattern_operations in crates/pattern-core/tests/python/test_operations.py
- [x] T027 [P] [US2] Create test_pattern_inspection in crates/pattern-core/tests/python/test_operations.py
- [x] T028 [P] [US2] Create test_pattern_queries in crates/pattern-core/tests/python/test_operations.py
- [x] T029 [P] [US2] Create test_pattern_combination in crates/pattern-core/tests/python/test_operations.py
- [x] T030 [P] [US2] Create test_pattern_comonad in crates/pattern-core/tests/python/test_operations.py

### Implementation for User Story 2

- [x] T031 [US2] Implement Pattern.length method in crates/pattern-core/src/python.rs
- [x] T032 [US2] Implement Pattern.size method in crates/pattern-core/src/python.rs
- [x] T033 [US2] Implement Pattern.depth method in crates/pattern-core/src/python.rs
- [x] T034 [US2] Implement Pattern.is_atomic method in crates/pattern-core/src/python.rs
- [x] T035 [US2] Implement Pattern.values method in crates/pattern-core/src/python.rs
- [x] T036 [US2] Implement Pattern.any_value method with Python callback support in crates/pattern-core/src/python.rs
- [x] T037 [US2] Implement Pattern.all_values method with Python callback support in crates/pattern-core/src/python.rs
- [x] T038 [US2] Implement Pattern.filter method with Python callback support in crates/pattern-core/src/python.rs
- [x] T039 [US2] Implement Pattern.find_first method with Python callback support in crates/pattern-core/src/python.rs
- [x] T040 [US2] Implement Pattern.matches method in crates/pattern-core/src/python.rs
- [x] T041 [US2] Implement Pattern.contains method in crates/pattern-core/src/python.rs
- [x] T042 [US2] Implement Pattern.map method with Python callback support in crates/pattern-core/src/python.rs
- [x] T043 [US2] Implement Pattern.fold method with Python callback support in crates/pattern-core/src/python.rs
- [x] T044 [US2] Implement Pattern.combine method in crates/pattern-core/src/python.rs
- [x] T045 [US2] Implement Pattern.extract method (comonad) in crates/pattern-core/src/python.rs
- [x] T046 [US2] Implement Pattern.extend method with Python callback support (comonad) in crates/pattern-core/src/python.rs
- [x] T047 [US2] Implement Pattern.depth_at method in crates/pattern-core/src/python.rs
- [x] T048 [US2] Implement Pattern.size_at method in crates/pattern-core/src/python.rs
- [x] T049 [US2] Implement Pattern.indices_at method in crates/pattern-core/src/python.rs
- [x] T050 [US2] Implement Pattern.validate method with ValidationRules in crates/pattern-core/src/python.rs
- [x] T051 [US2] Implement Pattern.analyze_structure method returning StructureAnalysis in crates/pattern-core/src/python.rs
- [x] T052 [US2] Implement ValidationRules Python class in crates/pattern-core/src/python.rs
- [x] T053 [US2] Implement ValidationError Python exception class in crates/pattern-core/src/python.rs
- [x] T054 [US2] Implement StructureAnalysis Python class in crates/pattern-core/src/python.rs
- [x] T055 [US2] Add docstrings to all new methods in crates/pattern-core/src/python.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently. Python developers can create patterns and perform operations on them.

---

## Phase 5: User Story 3 - Type-Safe Python Development (Priority: P3)

**Goal**: Enable Python developers using type checkers (mypy, pyright) to have type hints and annotations that enable static type checking and IDE autocomplete.

**Independent Test**: Write Python code with type annotations, run mypy/pyright, and verify no type errors occur. Can be tested independently by running type checkers on sample Python code using pattern-core.

### Tests for User Story 3

- [ ] T056 [P] [US3] Create test_type_safety in crates/pattern-core/tests/python/test_type_safety.py
- [ ] T057 [P] [US3] Create test_type_checking_validation in crates/pattern-core/tests/python/test_type_safety.py

### Implementation for User Story 3

- [ ] T058 [US3] Create crates/pattern-core/pattern_core/__init__.pyi with Pattern class type hints
- [ ] T059 [US3] Add PatternSubject type hints to crates/pattern-core/pattern_core/__init__.pyi
- [ ] T060 [US3] Add Subject class type hints to crates/pattern-core/pattern_core/__init__.pyi
- [ ] T061 [US3] Add Value class type hints with all variants to crates/pattern-core/pattern_core/__init__.pyi
- [ ] T062 [US3] Add ValidationRules type hints to crates/pattern-core/pattern_core/__init__.pyi
- [ ] T063 [US3] Add ValidationError type hints to crates/pattern-core/pattern_core/__init__.pyi
- [ ] T064 [US3] Add StructureAnalysis type hints to crates/pattern-core/pattern_core/__init__.pyi
- [ ] T065 [US3] Add type hints for all Pattern methods with Callable signatures in crates/pattern-core/pattern_core/__init__.pyi
- [ ] T066 [US3] Add docstrings to type stubs for IDE tooltips in crates/pattern-core/pattern_core/__init__.pyi
- [ ] T067 [US3] Validate type stubs with mypy in crates/pattern-core/pattern_core/__init__.pyi
- [ ] T068 [US3] Validate type stubs with pyright in crates/pattern-core/pattern_core/__init__.pyi

**Checkpoint**: At this point, all user stories should now be independently functional. Python developers can use pattern-core with full type safety support.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

### Documentation & Examples

- [ ] T069 [P] Create docs/python-usage.md with comprehensive API reference
- [ ] T070 [P] Create examples/pattern-core-python/README.md with quickstart guide
- [ ] T071 [P] Create examples/pattern-core-python/basic_usage.py with construction examples
- [ ] T072 [P] Create examples/pattern-core-python/operations.py with pattern operations examples
- [ ] T073 [P] Create examples/pattern-core-python/type_safety.py with type hints examples
- [ ] T074 [P] Create examples/pattern-core-python/advanced.py with advanced use cases (comonad, complex subjects)
- [ ] T075 Run quickstart.md validation against examples

### Testing & Integration

- [ ] T076 [P] Add edge case tests for None values in crates/pattern-core/tests/python/test_edge_cases.py
- [ ] T077 [P] Add edge case tests for deep nesting in crates/pattern-core/tests/python/test_edge_cases.py
- [ ] T078 [P] Add edge case tests for type conversion errors in crates/pattern-core/tests/python/test_edge_cases.py
- [ ] T079 [P] Add integration test for complete workflow in crates/pattern-core/tests/python/test_integration.py
- [ ] T080 [P] Add performance test for large patterns in crates/pattern-core/tests/python/test_performance.py
- [ ] T081 Verify all Python tests pass with pytest crates/pattern-core/tests/python/

### Build & Packaging

- [ ] T082 Test building Python wheel with maturin build --release --features python
- [ ] T083 Test installing Python wheel in virtual environment
- [ ] T084 Verify Python module imports correctly after installation
- [ ] T085 Test Python examples run successfully after installation

### Code Quality Checks (REQUIRED)

- [ ] T086 Run cargo fmt --all to ensure consistent code formatting
- [ ] T087 Run cargo clippy --workspace -- -D warnings to check for issues
- [ ] T088 Run full CI checks with scripts/ci-local.sh (if available) or equivalent CI validation
- [ ] T089 Verify all tests pass (cargo test --workspace and pytest crates/pattern-core/tests/python/)
- [ ] T090 Fix any formatting, linting, or test failures before completion

### Performance & Optimization

- [ ] T091 Benchmark Python bindings performance against native Rust operations
- [ ] T092 Verify performance targets are met (<2x overhead for patterns with up to 1000 nodes)
- [ ] T093 Optimize Python-Rust boundary crossing if needed

### Final Verification

- [ ] T094 Update crates/pattern-core/CHANGELOG.md with Python bindings feature
- [ ] T095 Update TODO.md to mark feature as complete
- [ ] T096 Ensure all acceptance criteria from spec.md are met
- [ ] T097 Verify all user stories can be tested independently
- [ ] T098 Verify type stubs work correctly with mypy and pyright
- [ ] T099 Verify examples demonstrate all user stories
- [ ] T100 Verify documentation is complete and accurate

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Depends on US1 for Pattern class, but operations can be implemented independently
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Depends on US1 and US2 for complete API, but type stubs can be written incrementally

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Core classes before methods
- Basic methods before advanced methods
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Different methods within a story marked [P] can run in parallel (different methods, same file)
- Documentation and examples marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Create test_pattern_construction in crates/pattern-core/tests/python/test_pattern.py"
Task: "Create test_subject_construction in crates/pattern-core/tests/python/test_subject.py"
Task: "Create test_pattern_subject_construction in crates/pattern-core/tests/python/test_pattern.py"

# Launch Value and Subject implementation together (different classes):
Task: "Implement Value Python class with all variants in crates/pattern-core/src/python.rs"
Task: "Implement Subject Python class with identity, labels, properties in crates/pattern-core/src/python.rs"
```

---

## Parallel Example: User Story 2

```bash
# Launch all inspection methods together (different methods, same file):
Task: "Implement Pattern.length method in crates/pattern-core/src/python.rs"
Task: "Implement Pattern.size method in crates/pattern-core/src/python.rs"
Task: "Implement Pattern.depth method in crates/pattern-core/src/python.rs"
Task: "Implement Pattern.is_atomic method in crates/pattern-core/src/python.rs"

# Launch query methods together:
Task: "Implement Pattern.any_value method with Python callback support in crates/pattern-core/src/python.rs"
Task: "Implement Pattern.all_values method with Python callback support in crates/pattern-core/src/python.rs"
Task: "Implement Pattern.filter method with Python callback support in crates/pattern-core/src/python.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Build Python wheel and verify installation
6. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Build wheel → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Build wheel → Deploy/Demo
4. Add User Story 3 → Test independently → Build wheel → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Pattern/Subject construction)
   - Developer B: User Story 2 (Pattern operations) - can start after US1 Pattern class exists
   - Developer C: User Story 3 (Type safety) - can start after US1/US2 API exists
3. Stories complete and integrate independently
4. Polish phase can run in parallel with final user story

---

## Notes

- [P] tasks = different files or different methods, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- Python callbacks require careful PyO3 handling - test edge cases
- Type stubs must be kept in sync with Rust implementation
- Examples should demonstrate all user stories and common patterns
