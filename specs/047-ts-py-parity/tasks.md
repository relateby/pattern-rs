# Tasks: TypeScript and Python Pattern API Parity

**Input**: Design documents from `/specs/047-ts-py-parity/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Organization**: Tasks grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to
- All file paths are relative to the repository root

---

## Phase 1: Setup

**Purpose**: Establish a clean test baseline before adding new operations.

- [x] T001 Run TypeScript test suite to confirm clean baseline: `cd typescript/packages/pattern && npx vitest run`
- [x] T002 [P] Run Python test suite to confirm clean baseline: `cd python/packages/relateby && pytest tests/`

**Checkpoint**: Both test suites pass before any modifications.

---

## Phase 2: Foundational

**Purpose**: No structural setup is required — both TypeScript and Python packages are complete and operational. Each user story phase adds directly to existing files.

**⚠️ No blocking prerequisite tasks**: All user stories can begin after Phase 1 passes.

---

## Phase 3: User Story 1 — Structural Pattern Comparison (Priority: P1) 🎯 MVP

**Goal**: Add `anyValue`, `allValues`, `matches`, and `contains` to both TypeScript and Python, enabling structural comparison and short-circuit predicate folds.

**Independent Test**: `anyValue(pred)(tree)` returns correct boolean; `matches(p, p)` returns `true`; `contains(outer, inner)` finds nested patterns. All acceptance scenarios from spec pass in both languages.

### TypeScript Implementation

- [x] T003 [P] [US1] Add `anyValue` (short-circuit pre-order predicate) to `typescript/packages/pattern/src/ops.ts`
- [x] T004 [P] [US1] Add `allValues` (short-circuit pre-order predicate) to `typescript/packages/pattern/src/ops.ts`
- [x] T005 [P] [US1] Add `matches` (structural equality via `Equal.equals`) to `typescript/packages/pattern/src/ops.ts`
- [x] T006 [P] [US1] Add `contains` (recursive subpattern search, curried `needle => haystack`) to `typescript/packages/pattern/src/ops.ts`
- [x] T007 [US1] Export `anyValue`, `allValues`, `matches`, `contains` from `typescript/packages/pattern/src/index.ts`

### Python Implementation

- [x] T008 [P] [US1] Add `any_value(predicate)` method to `python/packages/relateby/relateby/pattern/_pattern.py`
- [x] T009 [P] [US1] Add `all_values(predicate)` method to `python/packages/relateby/relateby/pattern/_pattern.py`
- [x] T010 [P] [US1] Add `matches(other)` method to `python/packages/relateby/relateby/pattern/_pattern.py`
- [x] T011 [P] [US1] Add `contains(needle)` method to `python/packages/relateby/relateby/pattern/_pattern.py`

### Tests

- [x] T012 [P] [US1] Add `anyValue`, `allValues`, `matches`, `contains` tests (including all spec acceptance scenarios and Haskell equivalence cases) to `typescript/packages/pattern/tests/pattern-ops.test.ts`
- [x] T013 [P] [US1] Add `any_value`, `all_values`, `matches`, `contains` tests (including all spec acceptance scenarios) to `python/packages/relateby/tests/test_pattern_ops.py`

**Checkpoint**: All US1 acceptance scenarios pass; `anyValue(const true)(p)` always `true`; `matches(p, p)` always `true`; `contains(p, p)` always `true` (reflexivity).

---

## Phase 4: User Story 2 — Structure-Aware Folding / Paramorphism (Priority: P1)

**Goal**: Add `para` to both TypeScript and Python, enabling fold functions to inspect the current sub-pattern alongside pre-computed child results.

**Independent Test**: `para((_p, rs) => rs.length === 0 ? 0 : 1 + max(rs))(nested)` returns the correct tree height. `para` on a leaf pattern passes empty child results to the fold function.

### TypeScript Implementation

- [x] T014 [P] [US2] Add `para` (curried paramorphism, bottom-up) to `typescript/packages/pattern/src/ops.ts`
- [x] T015 [US2] Export `para` from `typescript/packages/pattern/src/index.ts`

### Python Implementation

- [x] T016 [P] [US2] Add `para(f)` method to `python/packages/relateby/relateby/pattern/_pattern.py`

### Tests

- [x] T017 [P] [US2] Add `para` tests (leaf case, nested case, height computation, value-sum matching `fold` result) to `typescript/packages/pattern/tests/pattern-ops.test.ts`
- [x] T018 [P] [US2] Add `para` tests to `python/packages/relateby/tests/test_pattern_ops.py`

**Checkpoint**: `para` produces same result as equivalent Haskell `para` on matching test cases.

---

## Phase 5: User Story 3 — Programmatic Pattern Construction (Priority: P2)

**Goal**: Add `Pattern.pattern()`, `Pattern.fromList()`, and `unfold` to both TypeScript and Python, enabling composite pattern construction without gram string parsing.

**Independent Test**: `Pattern.pattern("root", [Pattern.point("a"), Pattern.point("b")])` creates a non-atomic pattern; `Pattern.fromList("root", ["a","b","c"])` creates a pattern with 3 atomic children; `unfold(expand, seed)` expands a countdown to the correct tree.

### TypeScript Implementation

- [x] T019 [P] [US3] Add `static pattern<V>(value, elements)` to the `Pattern` class in `typescript/packages/pattern/src/pattern.ts`
- [x] T020 [P] [US3] Add `static fromList<V>(value, values)` to the `Pattern` class in `typescript/packages/pattern/src/pattern.ts`
- [x] T021 [P] [US3] Add `unfold` (anamorphism, curried `expand => seed => Pattern<V>`) to `typescript/packages/pattern/src/ops.ts`
- [x] T022 [US3] Export `unfold` from `typescript/packages/pattern/src/index.ts`

### Python Implementation

- [x] T023 [P] [US3] Add `Pattern.pattern(value, elements)` classmethod to `python/packages/relateby/relateby/pattern/_pattern.py`
- [x] T024 [P] [US3] Add `Pattern.from_list(value, values)` classmethod to `python/packages/relateby/relateby/pattern/_pattern.py`
- [x] T025 [P] [US3] Add `Pattern.unfold(expand, seed)` classmethod to `python/packages/relateby/relateby/pattern/_pattern.py`
- [x] T026 [US3] Export module-level `unfold` alias from `python/packages/relateby/relateby/pattern/__init__.py`

### Tests

- [x] T027 [P] [US3] Add construction and unfold tests (including `fromList` empty list = `point`, `unfold` countdown, `unfold` binary tree) to `typescript/packages/pattern/tests/pattern-ops.test.ts`
- [x] T028 [P] [US3] Add construction and unfold tests to `python/packages/relateby/tests/test_pattern_ops.py`

**Checkpoint**: `Pattern.pattern(v, [])` equals `Pattern.point(v)`; `Pattern.fromList(v, vals).elements.length == vals.length`; `unfold` terminates on empty-children expand.

---

## Phase 6: User Story 4 — Pattern Combination / Semigroup (Priority: P2)

**Goal**: Add `combine` to both TypeScript and Python, enabling two patterns to be merged by combining their root values and concatenating their element lists.

**Independent Test**: `combine((a, b) => a + b)(pat1)(pat2)` merges root values and concatenates elements. Combining with an identity value produces the original pattern. Three-way combination is associative.

### TypeScript Implementation

- [x] T029 [P] [US4] Add `combine` (curried `combineValues => a => b => Pattern<V>`) to `typescript/packages/pattern/src/ops.ts`
- [x] T030 [US4] Export `combine` from `typescript/packages/pattern/src/index.ts`

### Python Implementation

- [x] T031 [P] [US4] Add `combine(other, combine_values)` method to `python/packages/relateby/relateby/pattern/_pattern.py`

### Tests

- [x] T032 [P] [US4] Add `combine` tests (string concat, identity, associativity law) to `typescript/packages/pattern/tests/pattern-ops.test.ts`
- [x] T033 [P] [US4] Add `combine` tests to `python/packages/relateby/tests/test_pattern_ops.py`

**Checkpoint**: `combine(f)(p, empty)` equals `p` (identity law); `combine` is associative with an associative `f`.

---

## Phase 7: User Story 5 — Comonad Position Helpers (Priority: P3)

**Goal**: Add `depthAt`, `sizeAt`, and `indicesAt` to both TypeScript and Python, annotating every position in a pattern tree with depth, subtree size, or root-path indices.

**Independent Test**: `depthAt(point(v))` equals `point(0)` (leaf depth is 0); `sizeAt(point(v))` equals `point(1)` (leaf size is 1); `indicesAt(point(v))` equals `point([])` (root has empty index path).

### TypeScript Implementation

- [x] T034 [P] [US5] Add `depthAt` (via `extend(sub => sub.depth)`) to `typescript/packages/pattern/src/ops.ts`
- [x] T035 [P] [US5] Add `sizeAt` (via `extend(sub => sub.size)`) to `typescript/packages/pattern/src/ops.ts`
- [x] T036 [P] [US5] Add `indicesAt` (position-aware recursive helper — cannot use `extend`) to `typescript/packages/pattern/src/ops.ts`
- [x] T037 [US5] Export `depthAt`, `sizeAt`, `indicesAt` from `typescript/packages/pattern/src/index.ts`

### Python Implementation

- [x] T038 [P] [US5] Add `depth_at()` method (via `self.extend(lambda s: s.depth)`) to `python/packages/relateby/relateby/pattern/_pattern.py`
- [x] T039 [P] [US5] Add `size_at()` method (via `self.extend(lambda s: s.size)`) to `python/packages/relateby/relateby/pattern/_pattern.py`
- [x] T040 [P] [US5] Add `indices_at()` method (position-aware recursive helper) to `python/packages/relateby/relateby/pattern/_pattern.py`

### Tests

- [x] T041 [P] [US5] Add `depthAt`, `sizeAt`, `indicesAt` tests (leaf invariants, nested tree, path correctness) to `typescript/packages/pattern/tests/pattern-ops.test.ts`
- [x] T042 [P] [US5] Add `depth_at`, `size_at`, `indices_at` tests to `python/packages/relateby/tests/test_pattern_ops.py`

**Checkpoint**: All comonad helper invariants hold; `indicesAt` root path is always `[]`; child at index `i` has path `[..., i]`.

---

## Phase 8: User Story 6 — Python Graph Transforms (Priority: P3)

**Goal**: Add `map_graph`, `map_all_graph`, `filter_graph`, `fold_graph`, `map_with_context`, and `para_graph` to Python, giving the Python `StandardGraph` a full transform layer equivalent to TypeScript.

**Independent Test**: `map_graph(graph, {"node": lambda p: p.map(fn)})` transforms all nodes without altering relationships; `filter_graph` removes elements per predicate and handles containers per substitution strategy.

### Implementation

- [x] T043 [US6] Create `python/packages/relateby/relateby/pattern/_graph_transforms.py` with `Substitution` type alias, internal `_build_graph_query` adapter that wraps `StandardGraph` state into a frozen query snapshot, and `_topological_sort` helper for `para_graph`
- [x] T044 [US6] Add `map_graph(graph, mappers)` and `map_all_graph(graph, f)` to `_graph_transforms.py` (depends on T043)
- [x] T045 [US6] Add `filter_graph(graph, pred, substitution)` with `delete_container`, `splice_gap`, and `replace_with_surrogate` substitution handling to `_graph_transforms.py` (depends on T043)
- [x] T046 [US6] Add `fold_graph(graph, f, empty, combine)` to `_graph_transforms.py` (depends on T043)
- [x] T047 [US6] Add `map_with_context(graph, f)` using frozen GraphQuery snapshot to `_graph_transforms.py` (depends on T043)
- [x] T048 [US6] Add `para_graph(graph, f)` processing elements bottom-up in topological order, returning `dict[str, R]` to `_graph_transforms.py` (depends on T043, T047 for pattern)
- [x] T049 [US6] Export `map_graph`, `map_all_graph`, `filter_graph`, `fold_graph`, `map_with_context`, `para_graph` from `python/packages/relateby/relateby/pattern/__init__.py`

### Tests

- [x] T050 [US6] Create `python/packages/relateby/tests/test_graph_transforms.py` with tests for all 6 transform functions, including: map_graph preserves non-node elements; filter_graph + splice_gap collapses containers; fold_graph reduces to single value; map_with_context snapshot is frozen; para_graph processes in topo order

**Checkpoint**: All 6 Python transform functions produce results equivalent to their TypeScript counterparts on matching `StandardGraph` inputs.

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Final validation, API surface audit, and cross-language equivalence confirmation.

- [x] T051 [P] Verify TypeScript export inventory: run `npx vitest run tests/public-api/export_inventory.test.ts` and confirm all 12 new ops appear in `typescript/packages/pattern/tests/public-api/export_inventory.test.ts`; add missing entries
- [x] T052 [P] Update Python public API inventory if `tests/test_public_api.py` tracks exported names: `python/packages/relateby/tests/test_public_api.py`
- [x] T053 [P] Run TypeScript type-check to confirm no `any` leaks: `cd typescript/packages/pattern && npx tsc --noEmit`
- [x] T054 [P] Run full TypeScript test suite: `cd typescript/packages/pattern && npx vitest run`
- [x] T055 [P] Run full Python test suite with coverage: `cd python/packages/relateby && pytest tests/ -v`
- [x] T056 Verify quickstart.md code examples run correctly in both languages: test each snippet in `specs/047-ts-py-parity/quickstart.md` manually

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: N/A — no blocking setup required
- **User Stories (Phases 3–8)**: All depend only on Phase 1 passing; stories are independent of each other
- **Polish (Phase 9)**: Depends on all desired user stories being complete

### User Story Dependencies

- **US1 (P1)**: No story dependencies — start after Phase 1
- **US2 (P1)**: No story dependencies — can start in parallel with US1
- **US3 (P2)**: No story dependencies — starts after Phase 1
- **US4 (P2)**: No story dependencies — can start in parallel with US3
- **US5 (P3)**: Depends only on Phase 1 — `depthAt`/`sizeAt` use `extend` which is already present; `indicesAt` is independent
- **US6 (P3)**: No story dependencies — Python `StandardGraph` is already complete

### Within Each Story

- TypeScript and Python implementations are independent and can proceed in parallel
- Tests should be written after implementation within the same story phase
- Export tasks (index.ts, __init__.py) depend on implementations being complete

### Parallel Opportunities

**US1 + US2 can run in parallel** (both P1, no shared files):
- Developer A: T003–T013 (US1 TypeScript)
- Developer B: T008–T013 (US1 Python) — same files as TS, so actually sequential within language
- After US1: Developer A + B start US2 immediately

Within each user story, TypeScript tasks and Python tasks are in separate files and can proceed in parallel.

---

## Parallel Example: User Story 1

```bash
# TypeScript ops (all in ops.ts — sequential within file, parallel with Python):
Task T003: Add anyValue to typescript/packages/pattern/src/ops.ts
Task T004: Add allValues to typescript/packages/pattern/src/ops.ts
Task T005: Add matches to typescript/packages/pattern/src/ops.ts
Task T006: Add contains to typescript/packages/pattern/src/ops.ts

# Python methods (all in _pattern.py — sequential within file, parallel with TypeScript ops.ts):
Task T008: Add any_value to python/packages/relateby/relateby/pattern/_pattern.py
Task T009: Add all_values to python/packages/relateby/relateby/pattern/_pattern.py
Task T010: Add matches to python/packages/relateby/relateby/pattern/_pattern.py
Task T011: Add contains to python/packages/relateby/relateby/pattern/_pattern.py

# Tests (after implementations, parallel between TS and Python):
Task T012: TypeScript tests in tests/pattern-ops.test.ts
Task T013: Python tests in tests/test_pattern_ops.py
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 2 Only — both P1)

1. Complete Phase 1: Run test baselines
2. Complete Phase 3: US1 (structural predicates) — independently testable
3. Complete Phase 4: US2 (paramorphism) — independently testable
4. **STOP and VALIDATE**: Run full test suite; verify against Haskell reference
5. Ship P1 feature set — provides the highest-value operations first

### Incremental Delivery

1. **Release 1**: US1 + US2 (P1) — Core query capabilities
2. **Release 2**: US3 + US4 (P2) — Programmatic construction + Semigroup algebra
3. **Release 3**: US5 + US6 (P3) — Comonad helpers + Python graph transforms
4. Each release passes full test suite independently

### Single-Developer Strategy

Recommended sequential order within each release:

1. TypeScript ops first (all in `ops.ts`, familiar Effect.ts patterns)
2. Python methods next (all in `_pattern.py`, straightforward class methods)
3. Tests last (validate both implementations in one pass per story)
4. Export updates (index.ts + __init__.py) after all ops confirmed

---

## Notes

- `[P]` tasks operate on different files — safe to run in parallel
- TypeScript tasks and Python tasks for the same story are always parallel-safe (different codebases)
- `indicesAt` (T036) and `indices_at` (T040) require a position-aware recursive helper — **cannot be expressed as `extend`** (see research.md Decision 6)
- `matches` in TypeScript uses `Equal.equals` from Effect — verify import in ops.ts
- `para_graph` (T048) requires topological ordering — reuse or reference the sort already in `StandardGraph`
- The export inventory test at `typescript/packages/pattern/tests/public-api/export_inventory.test.ts` will catch missing exports early
- All 12 new operations must be reachable from `@relateby/pattern` and `relateby.pattern` without internal import paths (SC-005)
