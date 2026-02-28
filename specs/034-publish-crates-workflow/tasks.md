# Tasks: Publish Rust Artifacts to Crates with Docs, Examples, and Tag-Based Release Workflow

**Input**: Design documents from `/specs/034-publish-crates-workflow/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: Not explicitly requested in spec; validation via `cargo publish --dry-run` and workflow run.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: User story (US1–US5)
- Include exact file paths in descriptions

## Path Conventions

- Workspace root: `Cargo.toml`, `crates/pattern-core/`, `crates/gram-codec/`, `.github/workflows/`, `docs/`
- **Published package names** use the `relateby-` prefix: `relateby-pattern`, `relateby-gram` (set in each crate’s `Cargo.toml` `[package] name`). Directory names stay `pattern-core` and `gram-codec`.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Verify workspace and prepare for publish-related changes

- [ ] T001 Verify workspace builds and CI passes at repo root: run `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, `cargo fmt --all -- --check` (or `./scripts/ci-local.sh`)
- [ ] T002 [P] Ensure `docs/` directory exists at repo root (create if missing for release instructions)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Package metadata and dependency setup so publish dry-run can succeed; minimal release doc structure.

**Critical**: No user story implementation until this phase is complete.

- [ ] T003 [P] Set `name = "relateby-pattern"` and add `readme`, `documentation`, `repository`, and `homepage` to `crates/pattern-core/Cargo.toml` per plan and research.md (documentation = `https://docs.rs/relateby-pattern`)
- [ ] T004 Set `name = "relateby-gram"` and add `readme`, `documentation`, `repository`, and `relateby-pattern` versioned dependency (`path` + `version = "0.1.0"`) to `crates/gram-codec/Cargo.toml`
- [ ] T005 Create `docs/release.md` with minimal section headings: Prerequisites, Publish order, Tag format and workflow, Recovery

**Checkpoint**: Metadata and dependency ready; release doc placeholder exists.

---

## Phase 3: User Story 1 – Publish Library Packages to Public Registry (Priority: P1) – MVP

**Goal**: Both crates are publishable; dry-run succeeds; manual publish order is documented.

**Independent Test**: Run `cargo publish -p relateby-pattern --dry-run` and `cargo publish -p relateby-gram --dry-run`; both succeed; docs/release.md states order (relateby-pattern then relateby-gram).

### Implementation for User Story 1

- [ ] T006 [US1] Run `cargo publish -p relateby-pattern --dry-run` and fix any packaging or metadata errors in `crates/pattern-core/` (e.g. missing files, invalid metadata)
- [ ] T007 [US1] Run `cargo publish -p relateby-gram --dry-run` and fix any packaging or metadata errors in `crates/gram-codec/`
- [ ] T008 [US1] Add publish order (relateby-pattern then relateby-gram) to `docs/release.md`

**Checkpoint**: User Story 1 complete; dry-run passes for both crates.

---

## Phase 4: User Story 4 – Tag-Triggered Release Workflow (Priority: P1)

**Goal**: Pushing a version tag triggers a workflow that builds, validates, and publishes both crates; secrets documented.

**Independent Test**: Push a tag `v*` (e.g. in a fork or test repo); workflow runs; build/test/lint run before publish; publish steps use `CARGO_REGISTRY_TOKEN`; docs/release.md describes the secret.

### Implementation for User Story 4

- [ ] T009 [US4] Create `.github/workflows/publish.yml` with `on: push: tags: ['v*']` per `specs/034-publish-crates-workflow/contracts/publish-workflow.md`
- [ ] T010 [US4] Add checkout (with submodules if needed), Rust toolchain, and Cargo cache steps to `.github/workflows/publish.yml`
- [ ] T011 [US4] Add build, test, clippy, and fmt-check steps to `.github/workflows/publish.yml` (fail job on failure so no publish runs)
- [ ] T012 [US4] Add `cargo publish -p relateby-pattern --token ${{ secrets.CARGO_REGISTRY_TOKEN }}` step to `.github/workflows/publish.yml`
- [ ] T013 [US4] Add optional delay (e.g. 30s) and `cargo publish -p relateby-gram --token ${{ secrets.CARGO_REGISTRY_TOKEN }}` step to `.github/workflows/publish.yml`
- [ ] T014 [US4] Document `CARGO_REGISTRY_TOKEN` (create at crates.io, add under GitHub Settings → Secrets) in `docs/release.md` Prerequisites

**Checkpoint**: User Story 4 complete; workflow file and secret documentation in place.

---

## Phase 5: User Story 2 – Published Documentation (Priority: P2)

**Goal**: Each crate has a documentation URL; docs build cleanly; docs.rs will host after publish.

**Independent Test**: `cargo doc --no-deps -p relateby-pattern` and `cargo doc --no-deps -p relateby-gram` succeed; Cargo.toml `documentation` points to docs.rs URL.

### Implementation for User Story 2

- [ ] T015 [P] [US2] Set `documentation = "https://docs.rs/relateby-pattern"` in `crates/pattern-core/Cargo.toml` if not already set
- [ ] T016 [P] [US2] Set `documentation = "https://docs.rs/relateby-gram"` in `crates/gram-codec/Cargo.toml` if not already set
- [ ] T017 [US2] Add `cargo doc --no-deps` step for publishable crates to `.github/workflows/publish.yml` before publish (optional per research; or document that docs.rs builds docs)
- [ ] T018 [US2] Fix doc warnings in `crates/pattern-core/src/` and `crates/gram-codec/src/` so `cargo doc --no-deps` builds without warnings (or document in `docs/release.md`)

**Checkpoint**: User Story 2 complete; documentation URLs set and doc build clean or documented.

---

## Phase 6: User Story 3 – Examples Available to Users (Priority: P2)

**Goal**: Examples are included in the package or documented; users can run them after installing the crate.

**Independent Test**: After packaging, examples are present in the tarball or run instructions are in README; `cargo run --example <name>` from crate directory succeeds.

### Implementation for User Story 3

- [ ] T019 [US3] Fix or remove `[[example]]` entries in `crates/pattern-core/Cargo.toml` that reference paths outside the crate (e.g. `../../examples/`); move examples into `crates/pattern-core/examples/` or remove
- [ ] T020 [US3] Fix or remove `[[example]]` entries in `crates/gram-codec/Cargo.toml` that reference paths outside the crate; move into `crates/gram-codec/examples/` or remove
- [ ] T021 [US3] Document how to run examples (e.g. `cargo run --example <name>`) in `crates/pattern-core/README.md` and `crates/gram-codec/README.md` or in `docs/release.md`
- [ ] T022 [US3] Verify `cargo run --example` for each declared example from `crates/pattern-core/` and `crates/gram-codec/` succeeds

**Checkpoint**: User Story 3 complete; examples included or documented and runnable.

---

## Phase 7: User Story 5 – Publishing Instructions (Priority: P2)

**Goal**: Maintainers have full written instructions for prerequisites, tag format, triggering the workflow, and recovery.

**Independent Test**: A maintainer unfamiliar with the project can follow `docs/release.md` to set up token, run dry-run, and understand tag format and recovery.

### Implementation for User Story 5

- [ ] T023 [US5] Fill Prerequisites section in `docs/release.md` (crates.io account, token creation, `CARGO_REGISTRY_TOKEN` in GitHub Secrets)
- [ ] T024 [US5] Fill Tag format and workflow trigger section in `docs/release.md` (e.g. `v0.1.0`, push tag to run workflow)
- [ ] T025 [US5] Fill Recovery section in `docs/release.md` (partial publish, duplicate version, build/test/lint failure)
- [ ] T026 [US5] Link to or embed quickstart steps from `specs/034-publish-crates-workflow/quickstart.md` in `docs/release.md`

**Checkpoint**: User Story 5 complete; full publishing instructions in `docs/release.md`.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Validate release flow and update project docs.

- [ ] T027 [P] Validate release flow per `specs/034-publish-crates-workflow/quickstart.md` (dry-run both crates; optionally document tag-push test)
- [ ] T028 Run `./scripts/ci-local.sh` (or `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, `cargo fmt --all -- --check`) and fix any failures
- [ ] T029 [P] Update `CLAUDE.md` or root `README.md` with pointer to `docs/release.md` for publishing

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies; start immediately.
- **Phase 2 (Foundational)**: Depends on Phase 1; blocks all user stories.
- **Phase 3 (US1)**: Depends on Phase 2; MVP for “publishable packages.”
- **Phase 4 (US4)**: Depends on Phase 2 (workflow can reference existing metadata); ideally after US1 so dry-run already passes.
- **Phase 5 (US2)**: Depends on Phase 2; can follow US1/US4.
- **Phase 6 (US3)**: Depends on Phase 2; example paths can be fixed after metadata is stable.
- **Phase 7 (US5)**: Depends on Phase 2 (release.md exists); can be filled after US4 so workflow and secret are documented.
- **Phase 8 (Polish)**: Depends on completion of desired user stories.

### User Story Completion Order

- **US1 (P1)**: After Foundational → dry-run and publish order (MVP).
- **US4 (P1)**: After Foundational (and preferably US1) → workflow and secret docs.
- **US2, US3, US5 (P2)**: After Foundational; can be done in parallel or in any order.

### Within Each User Story

- US1: T006 then T007 (relateby-gram dry-run depends on relateby-pattern being publishable); T008 can follow T007.
- US4: T009 → T010 → T011 → T012 → T013 (sequential); T014 with T023/T024/T025.
- US2: T015, T016 parallel; T017, T018 after.
- US3: T019, T020 parallel; T021, T022 after.
- US5: T023–T026 sequential or T023–T025 then T026.

### Parallel Opportunities

- Phase 1: T002 [P] with T001.
- Phase 2: T003 [P] with T004 (different crates); T005 after T002.
- Phase 5: T015 [P], T016 [P] in parallel.
- Phase 6: T019 [P], T020 [P] in parallel.
- Phase 8: T027 [P], T029 [P] in parallel.

---

## Parallel Example: User Story 2

```text
# Documentation URLs in parallel:
T015: Set documentation in crates/pattern-core/Cargo.toml (relateby-pattern)
T016: Set documentation in crates/gram-codec/Cargo.toml (relateby-gram)
```

## Parallel Example: User Story 3

```text
# Example path fixes in parallel:
T019: Fix [[example]] in crates/pattern-core/Cargo.toml
T020: Fix [[example]] in crates/gram-codec/Cargo.toml
```

---

## Implementation Strategy

### MVP First (User Stories 1 and 4)

1. Complete Phase 1: Setup  
2. Complete Phase 2: Foundational  
3. Complete Phase 3: User Story 1 (dry-run passes, publish order documented)  
4. Complete Phase 4: User Story 4 (workflow and secret docs)  
5. **Stop and validate**: Dry-run locally; push a tag in a test repo and confirm workflow runs (and optionally publish)  
6. Deploy/demo: First tag-driven publish

### Incremental Delivery

1. Setup + Foundational → metadata and release.md skeleton  
2. US1 → dry-run passes, publish order documented (MVP: “can publish”)  
3. US4 → tag-triggered workflow (MVP: “automated publish”)  
4. US2 → documentation URLs and doc build  
5. US3 → examples in package and runnable  
6. US5 → full instructions in docs/release.md  
7. Polish → quickstart validation and project doc pointer  

### Parallel Team Strategy

- After Foundational:  
  - Dev A: US1 (dry-run + order)  
  - Dev B: US4 (workflow)  
  - Dev C: US2 (docs) and US3 (examples)  
- Then US5 (instructions) and Polish together or sequentially.

---

## Notes

- [P] = different files or commands, no task dependencies.
- [USn] maps task to user story for traceability.
- No separate test tasks; validation is `cargo publish --dry-run` and workflow run.
- Commit after each task or logical group.
- Use absolute or repo-root-relative paths when implementing (e.g. `crates/pattern-core/Cargo.toml`).
