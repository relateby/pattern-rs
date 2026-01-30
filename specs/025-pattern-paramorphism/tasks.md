# Tasks: Pattern Paramorphism

**Input**: Design documents from `/specs/025-pattern-paramorphism/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Included per spec success criteria SC-004 (property-based tests) and SC-005 (gram-hs equivalence).

**Organization**: Tasks are grouped by user story so each story can be verified independently. The core implementation (para method) is in Phase 2; Phases 3–6 add tests that validate each story’s acceptance criteria.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files or independent tests)
- **[Story]**: User story (US1–US4) for traceability
- Include exact file paths in descriptions

## Path Conventions

- **Implementation**: `crates/pattern-core/src/pattern.rs`
- **Tests**: `crates/pattern-core/src/pattern.rs` (`#[cfg(test)] mod tests`) or `crates/pattern-core/tests/` for integration
- **Reference**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` (lines 1188–1190), `../gram-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs`, `../gram-hs/libs/pattern/tests/Spec/Pattern/Properties.hs`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm reference and workspace; no new project structure.

- [x] T001 Verify gram-hs reference at `../gram-hs/libs/pattern/src/Pattern/Core.hs` (para at lines 1188–1190) and plan.md in `specs/025-pattern-paramorphism/plan.md`
- [x] T002 [P] Confirm pattern-core builds and tests pass with `cargo build -p pattern-core && cargo test -p pattern-core` from repo root

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Implement the para API so all user stories can be exercised. No user story testing can be completed until para exists.

**CRITICAL**: Phase 2 must be complete before Phases 3–6.

- [x] T003 Add private helper `fn para_with<R, F>(&self, f: &F) -> R where F: Fn(&Pattern<V>, &[R]) -> R` in `crates/pattern-core/src/pattern.rs` that recursively computes results for each element (left-to-right), collects them into a `Vec<R>`, and calls `f(self, &results)` per research.md
- [x] T004 Add public method `pub fn para<R, F>(&self, f: F) -> R where F: Fn(&Pattern<V>, &[R]) -> R` in `crates/pattern-core/src/pattern.rs` that delegates to `para_with(self, &f)` so the closure is not moved on each recursive call
- [x] T005 Add doc comment and module-level docs for `para` in `crates/pattern-core/src/pattern.rs` describing signature, bottom-up order, atomic pattern (empty slice), and relationship to fold/extend per `specs/025-pattern-paramorphism/contracts/type-signatures.md`

**Checkpoint**: Para is implemented and baseline tests pass. User story test phases can proceed.

---

## Phase 3: User Story 1 – Pattern-of-Elements Analysis (Priority: P1) – MVP

**Goal**: Paramorphism supports pattern-of-elements analysis (e.g. A, B, A): folding function receives current pattern and element results in order; atomic pattern receives empty slice.

**Independent Test**: Build patterns with ordered elements; run para with a function that collects or inspects the sequence of element results; assert order and structure; assert atomic pattern gets empty slice.

- [x] T007 [P] [US1] Add unit test in `crates/pattern-core/src/pattern.rs`: atomic pattern receives empty slice (e.g. `point(42).para(|p, rs| { assert!(rs.is_empty()); *p.value() })`)
- [x] T008 [P] [US1] Add unit test in `crates/pattern-core/src/pattern.rs`: pattern with elements receives results in element order (e.g. para that builds `Vec` via `value : concat rs` equals pre-order values)
- [x] T009 [US1] Add unit test in `crates/pattern-core/src/pattern.rs`: structure access – `para(|p, _| p.depth())` equals `depth()` and `para(|p, _| p.elements().len())` equals `elements().len()` for a few fixtures

**Checkpoint**: US1 acceptance scenarios (pattern-of-elements, order, atomic base case) are covered by tests.

---

## Phase 4: User Story 2 – Element-Count-Aware Computation (Priority: P1)

**Goal**: Paramorphism supports aggregations that depend on how many elements each pattern has (e.g. value * element count + sum of element results).

**Independent Test**: Build patterns with 2 elements, then nested (e.g. 3 elements each with 2); run para with element-count-weighted formula; assert results match expected values.

- [ ] T010 [P] [US2] Add unit test in `crates/pattern-core/src/pattern.rs`: pattern with 2 elements – `para(|p, rs| *p.value() * p.elements().len() as i32 + rs.iter().sum::<i32>())` equals expected (e.g. 10*2 + (5+3) = 28 for pattern 10 [point 5, point 3])
- [ ] T011 [US2] Add unit test in `crates/pattern-core/src/pattern.rs`: nested pattern with varying element counts – aggregate element counts and assert correct totals at each level per spec acceptance scenario 2

**Checkpoint**: US2 (element-count-aware) is verified by tests.

---

## Phase 5: User Story 3 – Nesting Statistics (Priority: P2)

**Goal**: Paramorphism supports computing (sum, count, max depth) in a single traversal.

**Independent Test**: Run para that returns `(sum, count, max_depth)`; assert atomic gives (value, 1, 0); assert nested pattern gives correct tuple.

- [ ] T012 [P] [US3] Add unit test in `crates/pattern-core/src/pattern.rs`: atomic pattern – para returning (sum, count, max_depth) gives (value, 1, 0)
- [ ] T013 [US3] Add unit test in `crates/pattern-core/src/pattern.rs`: nested pattern – para computing (sum, count, maxDepth) in one traversal matches hand-computed tuple (e.g. pattern 1 [pattern 2 [point 3]] gives (6, 3, 1))

**Checkpoint**: US3 (nesting statistics) is verified by tests.

---

## Phase 6: User Story 4 – Custom Structure-Aware Folding (Priority: P2)

**Goal**: Any folding function `(pattern, element_results) -> R` is applied correctly at each node with recursive results; para can simulate Foldable (sum) and preserve order.

**Independent Test**: Use custom folding functions (e.g. build sequence representation, extract path info); assert para matches expected; assert `para(|p, rs| *p.value() + rs.iter().sum())` equals `fold(0, |a, v| a + v)`.

- [ ] T014 [P] [US4] Add unit test in `crates/pattern-core/src/pattern.rs`: para simulates fold – `pattern.para(|p, rs| *p.value() + rs.iter().sum::<i32>())` equals `pattern.fold(0, |acc, v| acc + v)` for integer patterns (port gram-hs T030)
- [ ] T015 [US4] Add unit test in `crates/pattern-core/src/pattern.rs`: para preserves element order when building value list – `para(|p, rs| value : concat rs)` equals pre-order values (port gram-hs T029 / toList property)
- [ ] T016 [US4] Add unit test or property test in `crates/pattern-core/src/pattern.rs`: custom folding function receiving (pattern, element_results) is invoked at each node with correct recursive results (e.g. structure-preserving transformation returning Pattern)

**Checkpoint**: US4 (custom folding, Foldable equivalence, order) is verified.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, property tests from gram-hs, code quality, and completion markers.

### Documentation & Examples

- [ ] T017 [P] Add inline doc examples for `para` in `crates/pattern-core/src/pattern.rs` (sum, depth-weighted sum, atomic) per `specs/025-pattern-paramorphism/quickstart.md`
- [ ] T018 [P] Document relationship para vs fold vs extend in `crates/pattern-core/src/pattern.rs` (para doc block or adjacent) per spec SC-006 and contracts/type-signatures.md
- [ ] T019 Run through `specs/025-pattern-paramorphism/quickstart.md` examples in doc tests or a small example and fix any API drift

### Property Tests (gram-hs equivalence)

- [ ] T020 Port property tests from `../gram-hs/libs/pattern/tests/Spec/Pattern/Properties.hs` (T025–T030): structure access (depth, element count), value access (toList), relationship to Foldable, order preservation; add in `crates/pattern-core/src/pattern.rs` or `crates/pattern-core/tests/` using existing test_utils generators
- [ ] T021 Port unit tests from `../gram-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs` (paramorphism describe block, e.g. T001–T010, T041–T048) into `crates/pattern-core/src/pattern.rs` so gram-hs examples produce equivalent results (SC-005)

### Code Quality Checks (REQUIRED)

- [ ] T022 Run `cargo fmt --all` from repo root and fix any formatting in `crates/pattern-core/src/pattern.rs`
- [ ] T023 Run `cargo clippy --workspace -- -D warnings` and fix any clippy warnings in pattern-core
- [ ] T024 Run full CI with `scripts/ci-local.sh` (if present) or `cargo test --workspace` and fix any failures
- [ ] T025 Fix any formatting, lint, or test failures before marking feature complete

### Final Verification

- [ ] T026 Update `TODO.md`: mark Phase 5 “PARAMORPHISM: Structure-Aware Folding” tasks complete and remove or update the implementation checklist for para
- [ ] T027 Ensure all acceptance criteria from `specs/025-pattern-paramorphism/spec.md` (FR-001–FR-008, SC-001–SC-006) are met and documented or tested

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies – run first.
- **Phase 2 (Foundational)**: Depends on Phase 1 – implements para; blocks all story phases.
- **Phases 3–6 (User Stories)**: Depend on Phase 2 only – add tests per story; can be done in parallel (T007–T009, T010–T011, T012–T013, T014–T016) or in order P1→P2→P2.
- **Phase 7 (Polish)**: Depends on Phases 2–6 – docs, property tests, quality checks, TODO.

### User Story Dependencies

- **US1 (P1)**: After Phase 2 – no dependency on US2–US4.
- **US2 (P1)**: After Phase 2 – no dependency on US1, US3, US4.
- **US3 (P2)**: After Phase 2 – no dependency on US1, US2, US4.
- **US4 (P2)**: After Phase 2 – no dependency on US1–US3.

### Within Each User Story

- Tests in a story can be implemented in any order; T007–T009 [P], T010–T011, T012–T013, T014–T016 can be parallelized where marked.

### Parallel Opportunities

- T001 and T002 can run in parallel.
- After Phase 2: T007, T008, T009 (US1); T010 (US2); T012 (US3); T014, T015 (US4) are [P] and can run in parallel.
- T017, T018 (docs) can run in parallel.
- T022 (fmt) and T023 (clippy) are sequential after code freeze; T024 (CI) after those.

---

## Parallel Example: User Story 1

```bash
# After Phase 2, run US1 tests in parallel (different test names in same file):
Task T007: "Add unit test: atomic pattern receives empty slice in crates/pattern-core/src/pattern.rs"
Task T008: "Add unit test: element order (value : concat rs) in crates/pattern-core/src/pattern.rs"
Task T009: "Add unit test: structure access (depth, elements().len()) in crates/pattern-core/src/pattern.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 (Setup).
2. Complete Phase 2 (Foundational) – para implemented and baseline tests pass.
3. Complete Phase 3 (US1) – pattern-of-elements tests pass.
4. **STOP and VALIDATE**: Run `cargo test -p pattern-core`; demo para for pattern-of-elements use case.
5. Optionally deploy/doc release for MVP.

### Incremental Delivery

1. Phase 1 + Phase 2 → para API available.
2. Phase 3 (US1) → pattern-of-elements verified (MVP).
3. Phase 4 (US2) → element-count-aware verified.
4. Phase 5 (US3) → nesting statistics verified.
5. Phase 6 (US4) → custom folding and Foldable equivalence verified.
6. Phase 7 → docs, property tests, quality, TODO updated.

### Single-Developer Order

1. T001 → T002 → T003 → T004 → T005 → T006 (Setup + Foundational).
2. T007 → T008 → T009 (US1).
3. T010 → T011 (US2).
4. T012 → T013 (US3).
5. T014 → T015 → T016 (US4).
6. T017 → T018 → T019 → T020 → T021 → T022 → T023 → T024 → T025 → T026 → T027 (Polish).

---

## Notes

- [P] = different test cases or files; safe to run in parallel.
- [USn] links task to spec user story for traceability.
- Each user story phase is independently testable once Phase 2 is done.
- Commit after each task or after each phase checkpoint.
- Reference: `../gram-hs/libs/pattern/src/Pattern/Core.hs` (para), CoreSpec.hs and Properties.hs (tests).
