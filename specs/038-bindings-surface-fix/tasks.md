# Tasks: TypeScript and Python Surface Improvements

**Input**: Design documents from `/specs/038-bindings-surface-fix/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`

**Tests**: Tests are required for this feature because the specification explicitly requires regression-detecting verification coverage for the supported TypeScript and Python public workflows.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g. `US1`, `US2`, `US3`)
- Every task includes an exact file path

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the public-surface validation harnesses and script hooks the rest of the feature will build on.

- [ ] T001 Create the TypeScript public-consumer harness config in `typescript/@relateby/pattern/tests/public-api/tsconfig.json`
- [ ] T002 Create the Python public-package test package scaffold in `python/relateby/tests/__init__.py`
- [ ] T003 [P] Wire public-surface validation commands into `typescript/@relateby/pattern/package.json` and `python/relateby/pyproject.toml`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish shared verification assets that block all user story work.

**⚠️ CRITICAL**: No user story work should begin until this phase is complete.

- [ ] T004 Create the TypeScript runtime export inventory test in `typescript/@relateby/pattern/tests/public-api/export_inventory.test.ts`
- [ ] T005 [P] Create the TypeScript packed-consumer import and typecheck fixture in `typescript/@relateby/pattern/tests/public-api/consumer.ts`
- [ ] T006 [P] Create the Python public-package workflow regression test file in `python/relateby/tests/test_public_api.py`
- [ ] T007 Update packaged smoke entrypoints in `scripts/release/npm-smoke/smoke.mjs` and `scripts/release/python-smoke.py`

**Checkpoint**: Public-surface harnesses exist for TypeScript and Python, and packaged-artifact smoke coverage is ready for story implementation.

---

## Phase 3: User Story 1 - Consistent Public Workflows (Priority: P1) 🎯 MVP

**Goal**: Make the supported TypeScript and Python package entry points usable for common workflows without missing exports, broken wrappers, or runtime/package mismatches.

**Independent Test**: Follow the documented quick-start workflows for TypeScript and Python using only public imports and confirm that the referenced symbols are importable, callable, and behaviorally correct.

### Implementation for User Story 1

- [ ] T008 [P] [US1] Fix top-level TypeScript export resolution and missing public aliases in `typescript/@relateby/pattern/src/index.ts`
- [ ] T009 [US1] Align the package-level Gram runtime surface and expose missing documented workflows in `typescript/@relateby/pattern/src/gram.ts`
- [ ] T010 [P] [US1] Fix Python wrapper pattern reconstruction and public graph workflows in `python/relateby/relateby/pattern/__init__.py`
- [ ] T011 [P] [US1] Fix native Python public workflow behavior in `crates/pattern-core/src/python.rs`
- [ ] T012 [US1] Expand TypeScript runtime workflow coverage in `typescript/@relateby/pattern/tests/pattern.test.ts`
- [ ] T013 [US1] Expand Python public workflow coverage in `python/relateby/tests/test_public_api.py`

**Checkpoint**: TypeScript and Python developers can complete the core documented workflows from the supported package entry points without internal imports.

---

## Phase 4: User Story 2 - Clear and Trustworthy Guidance (Priority: P2)

**Goal**: Make the docs, examples, type declarations, and stubs match the real public API so developers can trust the published guidance.

**Independent Test**: Review and run the official docs/examples and confirm that the published type or stub surfaces match the runtime public API for the same workflows.

### Implementation for User Story 2

- [ ] T014 [P] [US2] Align the package-level TypeScript declarations with the curated runtime surface in `typescript/@relateby/pattern/src/wasm-types.d.ts`
- [ ] T015 [P] [US2] Align shared generated-facing TypeScript declarations in `crates/pattern-core/typescript/pattern_core.d.ts` and `crates/pattern-wasm/typescript/gram.d.ts`
- [ ] T016 [P] [US2] Create shipped public Python stubs in `python/relateby/relateby/pattern/__init__.pyi` and `python/relateby/relateby/gram/__init__.pyi`
- [ ] T017 [US2] Remove stale native-only assumptions from `crates/pattern-core/pattern_core/__init__.pyi`
- [ ] T018 [US2] Update public guides and examples in `docs/wasm-usage.md`, `docs/typescript-graph.md`, `docs/python-usage.md`, `python/relateby/README.md`, and `examples/pattern-core-python/standard_graph.py`
- [ ] T019 [US2] Add public import type/stub validation in `typescript/@relateby/pattern/tests/public-api/consumer.ts` and `python/relateby/tests/test_public_api.py`

**Checkpoint**: Official guidance and shipped type surfaces now describe the same public workflows that users get at runtime.

---

## Phase 5: User Story 3 - Predictable Errors and Package Boundaries (Priority: P3)

**Goal**: Ensure invalid input, missing setup, and unsupported usage fail through the documented public package surface with predictable behavior and release-blocking verification.

**Independent Test**: Attempt invalid or incomplete public workflows in both languages and verify that failures surface through the documented package boundary and are caught by packaged-artifact verification.

### Implementation for User Story 3

- [ ] T020 [P] [US3] Normalize TypeScript public initialization and parse failure behavior in `typescript/@relateby/pattern/src/index.ts` and `typescript/@relateby/pattern/src/gram.ts`
- [ ] T021 [P] [US3] Normalize Python public exception mapping and wrapper failure behavior in `crates/pattern-core/src/python.rs`, `python/relateby/relateby/pattern/__init__.py`, and `python/relateby/relateby/gram/__init__.py`
- [ ] T022 [US3] Expand npm packed-artifact smoke coverage in `scripts/release/npm-smoke/smoke.mjs`
- [ ] T023 [US3] Expand Python wheel smoke coverage in `scripts/release/python-smoke.py`
- [ ] T024 [US3] Wire release-blocking public-surface verification into `scripts/ci-local.sh` and `.github/workflows/ci.yml`

**Checkpoint**: Public failure behavior is predictable and packaged-artifact verification blocks regressions before release.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Finish cross-language release guidance and close remaining public-surface drift outside a single story.

- [ ] T025 [P] Update release and packaging guidance for the public package boundaries in `docs/release.md` and `docs/python-packaging.md`
- [ ] T026 [P] Remove or update stale public-surface examples in `examples/gram-codec-python/README.md`, `examples/gram-codec-python/quickstart.py`, and `examples/gram-codec-python/demo.py`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies; can start immediately.
- **Foundational (Phase 2)**: Depends on Phase 1; blocks all user stories.
- **User Story 1 (Phase 3)**: Depends on Phase 2; this is the MVP and should be completed first.
- **User Story 2 (Phase 4)**: Depends on User Story 1 stabilizing the real public runtime surface.
- **User Story 3 (Phase 5)**: Depends on User Story 1; may proceed in parallel with User Story 2 once the public runtime behavior is stable.
- **Polish (Phase 6)**: Depends on all targeted user stories being complete.

### User Story Dependencies

- **US1 (P1)**: No dependencies on other user stories once foundational harnesses are in place.
- **US2 (P2)**: Depends on the stable public runtime/export behavior delivered by US1.
- **US3 (P3)**: Depends on the stable public runtime/export behavior delivered by US1; can run alongside US2 after that point.

### Within Each User Story

- Shared harness tasks come before story-specific tests.
- Runtime and wrapper behavior changes come before docs/types/stubs that describe them.
- Packaged-artifact smoke and CI wiring come after the relevant runtime behavior exists.
- A story is complete only when its independent test criteria pass using the supported public package boundaries.

### Parallel Opportunities

- `T003` can run in parallel with `T001` and `T002`.
- `T005` and `T006` can run in parallel after `T001`-`T003`.
- In US1, `T008`, `T010`, and `T011` can run in parallel; `T012` and `T013` follow those changes.
- In US2, `T014`, `T015`, and `T016` can run in parallel; `T018` and `T019` follow.
- In US3, `T020` and `T021` can run in parallel; `T022` and `T023` can then proceed in parallel before `T024`.
- In Polish, `T025` and `T026` can run in parallel.

---

## Parallel Example: User Story 1

```bash
# Launch runtime and wrapper work in parallel:
Task: "T008 [US1] Fix top-level TypeScript export resolution and missing public aliases in typescript/@relateby/pattern/src/index.ts"
Task: "T010 [US1] Fix Python wrapper pattern reconstruction and public graph workflows in python/relateby/relateby/pattern/__init__.py"
Task: "T011 [US1] Fix native Python public workflow behavior in crates/pattern-core/src/python.rs"

# Then validate each language-specific workflow layer:
Task: "T012 [US1] Expand TypeScript runtime workflow coverage in typescript/@relateby/pattern/tests/pattern.test.ts"
Task: "T013 [US1] Expand Python public workflow coverage in python/relateby/tests/test_public_api.py"
```

## Parallel Example: User Story 2

```bash
# Launch declaration and stub work in parallel:
Task: "T014 [US2] Align the package-level TypeScript declarations with the curated runtime surface in typescript/@relateby/pattern/src/wasm-types.d.ts"
Task: "T015 [US2] Align shared generated-facing TypeScript declarations in crates/pattern-core/typescript/pattern_core.d.ts and crates/pattern-wasm/typescript/gram.d.ts"
Task: "T016 [US2] Create shipped public Python stubs in python/relateby/relateby/pattern/__init__.pyi and python/relateby/relateby/gram/__init__.pyi"
```

## Parallel Example: User Story 3

```bash
# Launch error-path normalization in parallel:
Task: "T020 [US3] Normalize TypeScript public initialization and parse failure behavior in typescript/@relateby/pattern/src/index.ts and typescript/@relateby/pattern/src/gram.ts"
Task: "T021 [US3] Normalize Python public exception mapping and wrapper failure behavior in crates/pattern-core/src/python.rs, python/relateby/relateby/pattern/__init__.py, and python/relateby/relateby/gram/__init__.py"

# Then expand packaged-artifact smoke coverage in parallel:
Task: "T022 [US3] Expand npm packed-artifact smoke coverage in scripts/release/npm-smoke/smoke.mjs"
Task: "T023 [US3] Expand Python wheel smoke coverage in scripts/release/python-smoke.py"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup.
2. Complete Phase 2: Foundational harnesses.
3. Complete Phase 3: User Story 1.
4. Validate the documented TypeScript and Python quick-start workflows using only public imports.
5. Stop after US1 if an MVP fix release is needed quickly.

### Incremental Delivery

1. Deliver US1 to restore reliable public workflows.
2. Deliver US2 to align docs, examples, and shipped type surfaces with the repaired runtime.
3. Deliver US3 to harden error behavior and make packaged-artifact verification release-blocking.
4. Finish with cross-cutting release and example cleanup.

### Suggested MVP Scope

- **MVP**: Phase 1, Phase 2, and Phase 3 (US1) only.
- This delivers the highest-value outcome: the documented public package entry points work for the primary TypeScript and Python workflows.

---

## Notes

- All tasks follow the required checklist format with IDs, optional `[P]` markers, required `[US#]` labels for story tasks, and exact file paths.
- Public package boundaries are the source of truth for this feature: `@relateby/pattern`, `relateby.pattern`, and `relateby.gram`.
- Packaged-artifact validation is mandatory for this feature because source-tree tests alone do not protect the published developer experience.
