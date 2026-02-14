# Tasks: Relateby PyPI Release

**Input**: Design documents from `/specs/029-relateby-pypi-release/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: No separate test phases requested in the feature spec. Verification steps (build produces wheel, install and import work) are included as implementation tasks.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Unified Python package**: `python/relateby/` (single pyproject.toml, project name `relateby`; layout `relateby/pattern` and `relateby/gram`)
- **Existing crates**: `crates/pattern-core/`, `crates/gram-codec/`
- **Docs**: `docs/` at repository root; release process in `docs/release.md` or linked from specs

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the unified package directory and minimal configuration so the single PyPI project `relateby` has one version and metadata source.

- [ ] T001 Create unified package directory `python/relateby/` and minimal structure: `python/relateby/pyproject.toml` placeholder and `python/relateby/relateby/` package directory for layout (relateby/pattern, relateby/gram)
- [ ] T002 Create single `python/relateby/pyproject.toml` with project name `relateby`, version (e.g. 0.1.0), description, readme, license (Apache-2.0), requires-python (>=3.8), classifiers and keywords per research.md and data-model.md
- [ ] T003 [P] Add `python/relateby/README.md` describing the unified package, that one install provides `relateby.pattern` and `relateby.gram`, and install command `pip install relateby`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Implement the unified build so one build command produces one wheel (and sdist) for project `relateby` with only `relateby.pattern` and `relateby.gram` as public imports. No user story can be completed until this phase is done.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T004 Implement build that produces pattern-core and gram-codec Python extensions (invoke maturin or cargo for `crates/pattern-core` and `crates/gram-codec` with python feature) from or triggered by `python/relateby/`
- [ ] T005 Assemble built extensions into package layout so installed package exposes only `relateby.pattern` and `relateby.gram` (no top-level `pattern_core` or `gram_codec`) under `python/relateby/relateby/pattern/` and `python/relateby/relateby/gram/` (or equivalent re-exports)
- [ ] T006 Configure unified package build backend in `python/relateby/pyproject.toml` (maturin or setuptools with build script) so one build command from `python/relateby/` produces wheel(s) and sdist for project name `relateby`
- [ ] T007 Verify one build command from `python/relateby/` produces wheel(s) and sdist; run local `pip install` from built wheel and confirm only `import relateby.pattern` and `import relateby.gram` succeed (no `pattern_core` at top level)

**Checkpoint**: Foundation ready — unified package builds and installs with correct public API; user story implementation can begin

---

## Phase 3: User Story 1 - Publish Package to PyPI (Priority: P1) 🎯 MVP

**Goal**: A maintainer can publish the single `relateby` package to PyPI (or TestPyPI) so the project is discoverable and one install delivers both `relateby.pattern` and `relateby.gram`.

**Independent Test**: Run a single publish (or dry-run to TestPyPI) and confirm the unified package appears on the index and one install provides both subpackages.

### Implementation for User Story 1

- [ ] T008 [US1] Document release prerequisites (maturin, Python, PyPI account with 2FA, API token or Trusted Publishing) in `docs/release.md` without embedding secrets
- [ ] T009 [US1] Document build and publish commands in `docs/release.md`: from `python/relateby/` run `maturin build --release` then `maturin publish` (or equivalent); include exact directory and command
- [ ] T010 [US1] Document TestPyPI dry-run in `docs/release.md`: `maturin publish --repository testpypi` and optional verification `pip install --index-url https://test.pypi.org/simple/ relateby` then `import relateby.pattern`, `import relateby.gram`
- [ ] T011 [US1] Document in `docs/release.md` that PyPI rejects re-upload of same version (bump version for new release) and document credentials handling (~/.pypirc, MATURIN_PYPI_TOKEN; CI: Trusted Publishing or repository secret) without embedding secrets

**Checkpoint**: User Story 1 complete — maintainers have documented steps to publish the single package to PyPI

---

## Phase 4: User Story 2 - Install Package via Relateby Namespace (Priority: P2)

**Goal**: An end user can run `pip install relateby` and get both `relateby.pattern` and `relateby.gram` in one install; docs and examples use only the new import names.

**Independent Test**: Install the package from PyPI (or TestPyPI) in a clean environment and run a basic usage scenario (import and one documented operation) within 5 minutes.

### Implementation for User Story 2

- [ ] T012 [US2] Ensure `python/relateby/pyproject.toml` has all PyPI-required metadata (name, version, description, readme, license, requires-python, classifiers) so upload and `pip install relateby` succeed
- [ ] T013 [US2] Document end-user install and usage in `docs/python-usage.md` (or project README): `pip install relateby` and `import relateby.pattern`, `import relateby.gram` with a minimal example
- [ ] T014 [P] [US2] Update existing Python examples and docs to use `relateby.pattern` and `relateby.gram` only (remove or replace `pattern_core` and `gram_codec` references) in `examples/pattern-core-python/`, `examples/gram-codec-python/`, `examples/README.md`, and `docs/python-usage.md`

**Checkpoint**: User Story 2 complete — end users can install and use the package; docs and examples use only the new imports

---

## Phase 5: User Story 3 - Repeatable and Documented Release (Priority: P3)

**Goal**: A maintainer has clear documentation and a repeatable process for releasing the unified `relateby` package so future releases are consistent and new maintainers can publish without guessing.

**Independent Test**: A maintainer follows the documentation to perform a release (or dry-run) and confirms all steps are documented and repeatable.

### Implementation for User Story 3

- [ ] T015 [US3] Add or update release process document in `docs/release.md` with ordered steps per `specs/029-relateby-pypi-release/contracts/release-process.md`: set version in `python/relateby/pyproject.toml`, build, optional TestPyPI validate, publish, verify
- [ ] T016 [US3] Link or reference maintainer quickstart (e.g. `specs/029-relateby-pypi-release/quickstart.md`) from `docs/release.md` or docs README so new maintainers can find it
- [ ] T017 [US3] Document in `docs/release.md` that version is defined in one place (`python/relateby/pyproject.toml`), that same source and version produce consistent artifacts, and error handling (build failure, duplicate version, credential error, retry) per release-process contract

**Checkpoint**: User Story 3 complete — release process is documented and repeatable

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Code quality, CI, and final verification

### Documentation & Consistency

- [ ] T018 [P] Update `CLAUDE.md` with `pip install relateby` and `relateby.pattern` / `relateby.gram` in Python bindings section if not already covered by US2 docs

### Code Quality Checks (REQUIRED)

- [ ] T019 Run `cargo fmt --all` and fix any formatting
- [ ] T020 Run `cargo clippy --workspace -- -D warnings` and fix any issues
- [ ] T021 Run `scripts/ci-local.sh` (or equivalent) and fix any failures
- [ ] T022 Run `cargo test --workspace` and any Python tests (e.g. in `crates/pattern-core/tests/python/`) and fix failures; verify unified package build and local install still work

### Final Verification

- [ ] T023 Confirm all acceptance criteria from `specs/029-relateby-pypi-release/spec.md` are met (publish steps documented, install works, release repeatable, only relateby.pattern and relateby.gram public)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 completion — BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Phase 2 — publish docs require a build that works
- **User Story 2 (Phase 4)**: Depends on Phase 2 — install docs and examples assume unified package exists
- **User Story 3 (Phase 5)**: Depends on Phase 2; benefits from Phase 3 (release doc can consolidate US1 content)
- **Polish (Phase 6)**: Depends on completion of Phases 1–5

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2). No dependency on US2 or US3.
- **User Story 2 (P2)**: Can start after Foundational (Phase 2). Depends on unified package existing; doc updates can reference build from Phase 2.
- **User Story 3 (P3)**: Can start after Foundational (Phase 2). Release process doc can be written once build exists; may reference or merge US1 publish steps.

### Within Each User Story

- US1: Prerequisites → build/publish commands → TestPyPI → version and credentials (logical order in one doc)
- US2: Metadata completeness → end-user doc → examples update (T014 [P] can run in parallel with T012/T013)
- US3: Release steps → quickstart link → version and error handling

### Parallel Opportunities

- Phase 1: T003 [P] can run in parallel with T002 after T001
- Phase 2: T004 and T005 are sequential (build then assemble); T006 config may overlap with T005
- Phase 3: T008–T011 can be done in sequence as one doc or split; no [P] needed if single file
- Phase 4: T014 [P] can run in parallel with T012 and T013 (different files)
- Phase 6: T018 [P] can run in parallel with T019–T022

---

## Parallel Example: User Story 2

```text
# After Phase 2 complete, US2 tasks that touch different files:
T012: Ensure python/relateby/pyproject.toml has full metadata
T013: Document end-user install in docs/python-usage.md
T014 [P]: Update examples/pattern-core-python/, examples/gram-codec-python/, examples/README.md, docs/python-usage.md for relateby.pattern / relateby.gram
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (unified package dir and pyproject.toml)
2. Complete Phase 2: Foundational (unified build producing one wheel with relateby.pattern and relateby.gram)
3. Complete Phase 3: User Story 1 (publish documentation)
4. **STOP and VALIDATE**: Perform a TestPyPI dry-run using the documented steps; confirm package appears and one install provides both subpackages
5. Optionally publish to production PyPI or hand off to maintainer

### Incremental Delivery

1. Setup + Foundational → unified package builds and installs correctly (MVP foundation)
2. Add User Story 1 → Maintainers can publish (MVP!)
3. Add User Story 2 → End users can install and docs/examples use new names
4. Add User Story 3 → Release process is repeatable and documented
5. Polish → CI and acceptance criteria verified

### Single-Developer Strategy

1. Execute phases in order (1 → 2 → 3 → 4 → 5 → 6)
2. After Phase 2, run local install verification before writing US1/US2/US3 docs
3. Use T014 [P] to batch-update all example and doc files in one pass if preferred

---

## Notes

- [P] tasks = different files or independent edits; no ordering requirement
- [Story] label maps task to spec user story for traceability
- Each user story is independently testable per spec (publish, install, repeatable release)
- No separate test phases; verification is part of implementation (T007, T023)
- Commit after each task or logical group
- Unified package location is `python/relateby/`; if the team chooses `crates/relateby/` instead, update paths in tasks and docs accordingly
