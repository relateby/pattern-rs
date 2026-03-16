# Tasks: Consolidated Stable Publishing Workflow

**Input**: Design documents from `/specs/037-publishing-workflow/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/release-api.yaml, quickstart.md

**Tests**: Release validation and smoke-test automation are required because the feature specification defines artifact-level validation as part of the release flow.

**Organization**: Tasks are grouped by user story so each story can be implemented and validated independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (e.g. `US1`, `US2`)
- Include exact file paths in every task description

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create shared release-support files used by local preparation, CI validation, and publish automation.

- [X] T001 Create shared release helper library in `scripts/release/common.sh`
- [X] T002 [P] Create npm packed-artifact smoke fixture in `scripts/release/npm-smoke/package.json`
- [X] T003 [P] Create Python artifact smoke helpers in `scripts/release/python-smoke.py` and `scripts/release/smoke-python.sh`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish the consolidated artifact topology and shared validation surface that all user stories rely on.

**⚠️ CRITICAL**: No user story work should begin until this phase is complete.

- [X] T004 Define the release-managed version set in `Cargo.toml`, `crates/gram-codec/Cargo.toml`, `typescript/@relateby/pattern/package.json`, and `python/relateby/pyproject.toml`
- [X] T005 [P] Reconfigure npm workspace publishing metadata for one public package in `package.json`, `typescript/@relateby/pattern/package.json`, `typescript/@relateby/gram/package.json`, and `typescript/@relateby/graph/package.json`
- [X] T006 [P] Reconfigure the combined Python package metadata for namespace-only `relateby` imports in `python/relateby/pyproject.toml` and `python/relateby/relateby_build/__init__.py`
- [X] T007 Wire shared release smoke helpers into local validation entrypoints in `scripts/ci-local.sh`
- [X] T008 Wire shared release smoke helpers into CI validation entrypoints in `.github/workflows/ci.yml`

**Checkpoint**: Consolidated artifact metadata and shared validation entrypoints are ready for story implementation.

---

## Phase 3: User Story 1 - Prepare A Release From Main (Priority: P1) 🎯 MVP

**Goal**: Add a single `scripts/new-release.sh` flow that verifies `main`, aligns versions, runs release validation, and creates the release commit and annotated tag locally.

**Independent Test**: Run `./scripts/new-release.sh 0.2.0` from a clean, up-to-date `main` checkout and verify that it updates all release-managed manifests, runs the full release validation suite, creates one release commit, and creates annotated tag `v0.2.0` without publishing locally.

- [X] T009 [US1] Implement `main` branch, clean-worktree, and `origin/main` sync preflight checks in `scripts/new-release.sh`
- [X] T010 [US1] Implement shared version bumping and version-alignment verification in `scripts/new-release.sh` and `scripts/release/common.sh`
- [X] T011 [US1] Implement the release validation runner for Rust, WASM, npm pack smoke tests, and Python artifact smoke tests in `scripts/new-release.sh`, `scripts/release/common.sh`, `scripts/release/npm-smoke/package.json`, and `scripts/release/smoke-python.sh`
- [X] T012 [US1] Implement release commit creation, annotated tag creation, and optional `--push` handling in `scripts/new-release.sh`
- [X] T013 [US1] Add maintainer-facing usage output and failure diagnostics in `scripts/new-release.sh`

**Checkpoint**: `scripts/new-release.sh` can prepare a stable release from `main` without performing any remote publishing.

---

## Phase 4: User Story 2 - Publish Automatically From A Stable Tag (Priority: P1)

**Goal**: Replace the current crates-only tag workflow with a validation-first publish workflow that automatically publishes all supported release artifacts from a stable tag on `main`.

**Independent Test**: Push a stable `vX.Y.Z` tag from a prepared release commit on `main` and verify that GitHub Actions reruns the release validation matrix, then publishes the two Rust crates, the single npm package, and the combined Python package only after validation succeeds.

- [X] T014 [US2] Add stable-tag and commit-on-`main` verification helpers in `scripts/release/verify-tag.sh`
- [X] T015 [US2] Refactor `.github/workflows/publish.yml` into a validation-first job graph that calls `scripts/release/verify-tag.sh` and blocks publish steps on validation success
- [X] T016 [US2] Add ordered crates.io publishing and verification steps for `relateby-pattern` and `relateby-gram` in `.github/workflows/publish.yml`
- [X] T017 [US2] Add stable npm publishing and verification steps for `@relateby/pattern` in `.github/workflows/publish.yml`
- [X] T018 [US2] Add combined PyPI publishing and verification steps for `relateby-pattern` in `.github/workflows/publish.yml`
- [X] T019 [US2] Align `.github/workflows/ci.yml` with the publish workflow’s release-grade validation matrix in `.github/workflows/ci.yml`

**Checkpoint**: A stable release tag on `main` triggers one automated validation-and-publish workflow for all supported artifacts.

---

## Phase 5: User Story 3 - Consume Single Combined npm And Python Artifacts (Priority: P2)

**Goal**: Consolidate npm and Python packaging so that each ecosystem exposes one supported public artifact while preserving the existing import/API surface.

**Independent Test**: Build the packed npm tarball and combined Python wheel, install each in a clean environment, and confirm that pattern and gram functionality are available from `@relateby/pattern` and `relateby.pattern` / `relateby.gram` respectively.

- [X] T020 [P] [US3] Fold `@relateby/graph` and `@relateby/gram` exports into `typescript/@relateby/pattern/src/index.ts` and supporting modules under `typescript/@relateby/pattern/src/`
- [X] T021 [US3] Update single-package npm publish metadata, generated files, and smoke coverage in `typescript/@relateby/pattern/package.json`, `typescript/@relateby/pattern/tests/pattern.test.ts`, and `scripts/release/npm-smoke/package.json`
- [X] T022 [US3] Mark `typescript/@relateby/gram/package.json` and `typescript/@relateby/graph/package.json` as internal-only/non-published workspace packages
- [X] T023 [P] [US3] Rename the combined Python distribution artifact to `relateby-pattern` while preserving namespace imports in `python/relateby/pyproject.toml`, `python/relateby/README.md`, and `python/relateby/relateby_build/__init__.py`
- [X] T024 [US3] Retire split Python release metadata from the supported release path in `python/relateby-pattern/pyproject.toml` and `python/relateby-gram/pyproject.toml`
- [X] T025 [US3] Finalize clean-install artifact smoke checks for combined npm and Python packaging in `scripts/release/python-smoke.py`, `scripts/release/smoke-python.sh`, and `scripts/release/npm-smoke/package.json`

**Checkpoint**: npm and Python each have one supported public artifact, and both can be verified from built release artifacts outside the workspace.

---

## Phase 6: User Story 4 - Release Documentation Matches Automation (Priority: P2)

**Goal**: Update maintainer and consumer documentation so it describes the consolidated stable publishing workflow and the supported artifact set accurately.

**Independent Test**: Follow the updated release and packaging docs from a fresh checkout and verify that the commands, package names, import paths, and workflow behavior all match the implementation.

- [X] T026 [P] [US4] Rewrite the consolidated maintainer release flow in `docs/release.md` and `docs/python-packaging.md`
- [X] T027 [P] [US4] Update npm and WASM consumer documentation for a single public package in `docs/wasm-usage.md`, `docs/typescript-graph.md`, and `README.md`
- [X] T028 [US4] Update Python consumer and maintainer documentation for namespace-only `relateby` imports in `docs/python-usage.md`, `python/relateby/README.md`, and `CLAUDE.md`
- [X] T029 [US4] Refresh local CI and workflow documentation in `.github/workflows/README.md` and `scripts/README.md`

**Checkpoint**: Maintainers and consumers can follow the docs without encountering obsolete package names or release steps.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final cleanup, end-to-end validation, and drift removal across the repo.

- [X] T030 [P] Run the quickstart walkthrough against the implemented workflow in `specs/037-publishing-workflow/quickstart.md`
- [X] T031 Remove obsolete artifact references from example and support docs in `examples/README.md`, `examples/pattern-core-python/README.md`, and `examples/gram-codec-python/README.md`
- [X] T032 Run the final release-grade validation pass and record any maintainer follow-up notes in `docs/release.md` and `scripts/ci-local.sh`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies; can start immediately.
- **Foundational (Phase 2)**: Depends on Setup completion; blocks all user stories.
- **User Story 1 (Phase 3)**: Depends on Foundational completion.
- **User Story 2 (Phase 4)**: Depends on User Story 1 because the publish workflow must match the local release-preparation contract.
- **User Story 3 (Phase 5)**: Depends on Foundational completion; can proceed after the consolidated artifact metadata is in place.
- **User Story 4 (Phase 6)**: Depends on User Stories 1, 2, and 3 so documentation reflects the implemented release behavior.
- **Polish (Phase 7)**: Depends on all user stories being complete.

### User Story Dependencies

- **US1**: Starts after Foundational; establishes the MVP local release-prep flow.
- **US2**: Depends on US1’s release contract and validation commands.
- **US3**: Starts after Foundational; provides the final single-artifact npm/Python surfaces consumed by US1 and US2 validation.
- **US4**: Depends on US1-US3 implementation details to document the final workflow accurately.

### Within Each User Story

- Shared helpers before orchestration.
- Version alignment before validation.
- Validation before commit/tag or publish wiring.
- Package metadata changes before smoke-install checks.
- Implementation before documentation.

### Parallel Opportunities

- **Setup**: T002 and T003 can run in parallel after T001.
- **Foundational**: T005 and T006 can run in parallel after T004; T007 and T008 can run in parallel once release helpers are in place.
- **US3**: T020 and T023 can run in parallel; T021 depends on T020, and T024 depends on T023.
- **US4**: T026 and T027 can run in parallel; T028 and T029 can follow once package behavior is finalized.

---

## Parallel Example: User Story 3

```bash
# Start the npm and Python consolidation work in parallel:
Task: "Fold @relateby/graph and @relateby/gram exports into typescript/@relateby/pattern/src/index.ts and supporting modules under typescript/@relateby/pattern/src/"
Task: "Rename the combined Python distribution artifact to relateby-pattern while preserving namespace imports in python/relateby/pyproject.toml, python/relateby/README.md, and python/relateby/relateby_build/__init__.py"

# Then finish ecosystem-specific packaging tasks:
Task: "Update single-package npm publish metadata, generated files, and smoke coverage in typescript/@relateby/pattern/package.json, typescript/@relateby/pattern/tests/pattern.test.ts, and scripts/release/npm-smoke/package.json"
Task: "Retire split Python release metadata from the supported release path in python/relateby-pattern/pyproject.toml and python/relateby-gram/pyproject.toml"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup.
2. Complete Phase 2: Foundational.
3. Complete Phase 3: User Story 1.
4. **Stop and validate**: Run `./scripts/new-release.sh <version>` on a clean `main` checkout and confirm it produces the release commit and tag without publishing.

### Incremental Delivery

1. Finish Setup + Foundational to establish consolidated artifact metadata and shared validation helpers.
2. Deliver US1 to make local release preparation reliable.
3. Deliver US2 to automate stable publishing from release tags.
4. Deliver US3 to finalize the single-artifact npm/Python packaging model.
5. Deliver US4 to align docs with the implemented workflow.
6. Finish with Polish for end-to-end validation and repo cleanup.

### Parallel Team Strategy

1. One developer completes Setup + Foundational with review from release stakeholders.
2. After Foundational:
   - Developer A: US1 local release script
   - Developer B: US3 npm/Python packaging consolidation
3. Once US1 is stable:
   - Developer A: US2 publish workflow
   - Developer B: US4 docs refresh

---

## Notes

- All tasks follow the required checklist format with IDs, optional `[P]`, required `[US#]` labels for story tasks, and exact file paths.
- Story phases are designed so each user story can be validated independently at its checkpoint.
- The suggested MVP scope is **User Story 1 only**.
