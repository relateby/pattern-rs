# Tasks: Multi-Language Repository Restructure

**Input**: Design documents from `/specs/040-restructure-multilang-layout/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`

**Tests**: No test-first tasks are generated because the spec does not require TDD. Validation tasks are included where they materially prove each story independently.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., `US1`, `US2`, `US3`)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare migration scaffolding and audit points before changing package roots.

- [x] T001 Audit active path references in `README.md`, `docs/`, `examples/`, `scripts/`, and `.github/workflows/` against the target layout in `specs/040-restructure-multilang-layout/research.md`
- [x] T002 Create target layout entrypoints in `adapters/wasm/`, `python/packages/`, `typescript/packages/`, `examples/archive/`, and `docs/archive/`
- [x] T003 Create archive index documents in `examples/archive/README.md` and `docs/archive/README.md`
- [x] T004 Record the medium-churn scope guard in `specs/040-restructure-multilang-layout/plan.md` and `specs/040-restructure-multilang-layout/research.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared workspace and automation changes that MUST be complete before story-specific moves.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 Update Cargo workspace membership in `Cargo.toml` for the adapter path `adapters/wasm/pattern-wasm`
- [x] T006 Update npm workspace globs and root release commands in `package.json` and `package-lock.json` for `typescript/packages/{pattern,graph,gram}`
- [x] T007 [P] Update workflow path assumptions in `.github/workflows/ci.yml` and `.github/workflows/publish.yml`
- [x] T008 [P] Update local validation and release path assumptions in `scripts/ci-local.sh`, `scripts/release/common.sh`, and `scripts/README.md`

**Checkpoint**: Workspace and automation are ready for package and documentation moves

---

## Phase 3: User Story 1 - Navigate the repository confidently (Priority: P1) 🎯 MVP

**Goal**: Make the peer implementations, public package surfaces, and adapter layer obvious from the repository tree and active guidance.

**Independent Test**: Review the repository root plus active guidance and confirm contributors can identify peer Rust libraries, three public TypeScript packages, the Python distribution root, and the `pattern-wasm` adapter without guesswork.

### Implementation for User Story 1

- [X] T009 [US1] Move the TypeScript package roots from `typescript/@relateby/` to `typescript/packages/pattern/`, `typescript/packages/graph/`, and `typescript/packages/gram/`
- [X] T010 [US1] Promote `typescript/packages/graph/package.json` and `typescript/packages/gram/package.json` to public package metadata and add any missing dependency declarations
- [X] T011 [US1] Move the Python distribution root from `python/relateby/` to `python/packages/relateby/` and update `python/packages/relateby/pyproject.toml`
- [X] T012 [US1] Move `crates/pattern-wasm/` to `adapters/wasm/pattern-wasm/` and update `adapters/wasm/pattern-wasm/Cargo.toml`
- [X] T013 [US1] Update package and build entrypoints in `typescript/packages/pattern/package.json`, `python/packages/relateby/relateby_build/__init__.py`, and `docs/wasm-usage.md` for the moved package roots
- [X] T014 [P] [US1] Add or refresh public package documentation in `typescript/packages/pattern/README.md`, `typescript/packages/graph/README.md`, `typescript/packages/gram/README.md`, and `python/packages/relateby/README.md`
- [X] T015 [US1] Update root-facing package guidance in `README.md`, `docs/release.md`, and `docs/python-usage.md` to present the final public surfaces and the discoverable adapter role
- [X] T016 [US1] Run canonical-layout validation for the public package surfaces and adapter from `specs/040-restructure-multilang-layout/quickstart.md`

**Checkpoint**: Contributors can find the canonical implementation and package surfaces from the repository root

---

## Phase 4: User Story 2 - Remove stale and misleading structure safely (Priority: P2)

**Goal**: Remove or archive stale paths without losing historical material that still has reference value.

**Independent Test**: Compare the repository before and after the migration and confirm stale paths are either archived or removed, while active guidance points only to the canonical current locations.

### Implementation for User Story 2

- [X] T017 [US2] Classify legacy docs and examples in `docs/`, `examples/`, and `README.md` references and record archive vs active decisions in `specs/040-restructure-multilang-layout/research.md`
- [X] T018 [P] [US2] Move historical review and migration notes into `docs/archive/` from active locations such as `docs/TOP-LEVEL-MD-REVIEW.md`
- [X] T019 [P] [US2] Archive approved legacy example paths into `examples/archive/` from `examples/wasm-js/`, `examples/pattern-core-wasm/`, `examples/gram-codec-wasm-web/`, and `examples/gram-codec-wasm-node/`
- [X] T020 [US2] Update active references to archived material in `README.md`, `examples/README.md`, `docs/`, and `scripts/` so archived paths are no longer presented as current
- [X] T021 [US2] Remove the stale root `src/` and clear any remaining active references in `README.md`, `docs/`, `scripts/`, and `.github/workflows/`
- [X] T022 [US2] Run stale-path and removal-readiness validation for archived paths and root `src/` using `specs/040-restructure-multilang-layout/quickstart.md`

**Checkpoint**: Stale paths are safely archived or removed, and active guidance contains no contradictory legacy references

---

## Phase 5: User Story 3 - Follow examples and docs that match the current product surface (Priority: P3)

**Goal**: Reorganize active examples and living docs so they match the current multi-language public surfaces and archive boundaries.

**Independent Test**: Open active examples and active docs only, then confirm they point to canonical current paths and reflect the current public package boundaries without relying on archived material.

### Implementation for User Story 3

- [X] T023 [US3] Reorganize active example directories into `examples/rust/`, `examples/python/`, and `examples/typescript/`
- [X] T024 [P] [US3] Update active example guidance in `examples/README.md`, `examples/relateby-graph/README.md`, `examples/pattern-core-python/README.md`, and `examples/gram-codec-python/README.md`
- [X] T025 [P] [US3] Update language and packaging guidance in `docs/python-packaging.md`, `docs/typescript-graph.md`, `.github/workflows/README.md`, and `scripts/README.md`
- [X] T026 [US3] Align publishing and smoke-test guidance with the new public surfaces in `docs/release.md`, `.github/workflows/publish.yml`, and `scripts/release/npm-smoke/package.json`
- [X] T027 [US3] Run active example and documentation validation for Rust, Python, and TypeScript surfaces using `specs/040-restructure-multilang-layout/quickstart.md`

**Checkpoint**: Active examples and docs reflect only the current multi-language product surface

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Finish repo-wide consistency, contributor guidance, and full validation.

- [X] T028 [P] Refresh contributor guidance in `AGENTS.md`, `CLAUDE.md`, and `.github/copilot-instructions.md`
- [X] T029 [P] Run full multi-target validation via `scripts/check-workflows.sh` and `scripts/ci-local.sh`
- [X] T030 Perform a final stale-path sweep for `typescript/@relateby/`, `python/relateby`, `crates/pattern-wasm`, and root `src/` references across `README.md`, `docs/`, `examples/`, `scripts/`, and `.github/workflows/`
- [X] T031 Review `../pattern-hs/specs/` and `../pattern-hs/libs/` and document in `specs/040-restructure-multilang-layout/research.md` that this feature changes repository structure only and introduces no intentional behavioral deviation
- [X] T032 [P] Re-run representative compatibility checks from `specs/040-restructure-multilang-layout/quickstart.md` and record completion evidence in `specs/040-restructure-multilang-layout/plan.md`
- [X] T033 Execute the 10-task onboarding review sample from `specs/040-restructure-multilang-layout/plan.md` and record first-attempt success results in `specs/040-restructure-multilang-layout/research.md`
- [X] T034 [P] Update Python workflow scripts and contributor guidance to prefer `uv` with a local `.venv` in `scripts/ci-local.sh`, `scripts/release/smoke-python.sh`, and related docs/comments while keeping lightweight metadata helpers on plain `python3`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1: Setup**: No dependencies, starts immediately
- **Phase 2: Foundational**: Depends on Phase 1, blocks all story work
- **Phase 3: US1**: Depends on Phase 2, defines the canonical package and adapter locations
- **Phase 4: US2**: Depends on US1, because stale paths can only be removed or archived after canonical replacements exist
- **Phase 5: US3**: Depends on US1, because active docs and examples must point at the final canonical package locations
- **Phase 6: Polish**: Depends on all desired user stories being complete

### User Story Dependencies

- **US1 (P1)**: Starts after Foundational and establishes the MVP repository shape
- **US2 (P2)**: Starts after US1 canonical locations exist; may overlap with late US3 doc cleanup if ownership is clear
- **US3 (P3)**: Starts after US1 canonical locations exist; should finish before final polish validation

### Within Each User Story

- Canonical package and path moves before documentation that references them
- Archive destination setup before moving legacy material
- Reference cleanup before deleting stale paths such as root `src/`
- Story validation before starting the next dependent phase

### Parallel Opportunities

- `T007` and `T008` can run in parallel after `T005` and `T006`
- `T014` can run in parallel with late US1 doc updates after package moves complete
- `T018` and `T019` can run in parallel because docs and examples archive moves affect different paths
- `T024` and `T025` can run in parallel after active example directories are in place
- `T028`, `T029`, and `T032` can run in parallel during polish once story work is complete

---

## Parallel Example: User Story 1

```bash
# After T009-T013 complete, run public surface documentation tasks in parallel:
Task: "Add or refresh public package documentation in typescript/packages/pattern/README.md, typescript/packages/graph/README.md, typescript/packages/gram/README.md, and python/packages/relateby/README.md"
Task: "Update root-facing package guidance in README.md, docs/release.md, and docs/python-usage.md to present the final public surfaces and the discoverable adapter role"
```

---

## Parallel Example: User Story 2

```bash
# Archive docs and examples in parallel once archive directories exist:
Task: "Move historical review and migration notes into docs/archive/ from active locations such as docs/TOP-LEVEL-MD-REVIEW.md"
Task: "Archive approved legacy example paths into examples/archive/ from examples/wasm-js/, examples/pattern-core-wasm/, examples/gram-codec-wasm-web/, and examples/gram-codec-wasm-node/"
```

---

## Parallel Example: User Story 3

```bash
# After active example directories are reorganized, update docs in parallel:
Task: "Update active example guidance in examples/README.md, examples/relateby-graph/README.md, examples/pattern-core-python/README.md, and examples/gram-codec-python/README.md"
Task: "Update language and packaging guidance in docs/python-packaging.md, docs/typescript-graph.md, .github/workflows/README.md, and scripts/README.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Confirm contributors can identify the final package surfaces and adapter from the repository root using `specs/040-restructure-multilang-layout/quickstart.md`

### Incremental Delivery

1. Finish Setup + Foundational to stabilize workspace and automation
2. Deliver US1 to establish the canonical repository layout and public surfaces
3. Deliver US2 to archive/remove stale structure safely
4. Deliver US3 to align all active docs and examples with the new structure
5. Finish with Polish for repo-wide consistency and validation

### Parallel Team Strategy

1. One developer handles workspace and automation updates in Phases 1-2
2. After US1 canonical paths land:
   - Developer A: stale-path archival and removal work in US2
   - Developer B: active examples and docs alignment in US3
3. Team reconverges for final validation and contributor guidance updates in Phase 6

---

## Notes

- `[P]` tasks touch different files and can proceed in parallel once dependencies are met
- Each user story has a concrete independent validation checkpoint
- User Story 1 is the suggested MVP because it establishes the canonical repository structure
- Validation tasks use `specs/040-restructure-multilang-layout/quickstart.md` as the acceptance source of truth
- Polish includes explicit constitution verification and onboarding review coverage
