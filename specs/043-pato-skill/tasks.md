# Tasks: `pato skill`

**Input**: Design documents from `specs/043-pato-skill/`  
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓, quickstart.md ✓

**Organization**: Tasks grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (US1–US3)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the canonical skill package and prepare `relateby-pato` to bundle and test it.

- [ ] T001 Create the canonical skill package entry file in `.agents/skills/pato/SKILL.md`
- [ ] T002 [P] Create bundled skill reference content in `.agents/skills/pato/references/workflows.md` and `.agents/skills/pato/references/output-contracts.md`
- [ ] T003 [P] Create bundled skill example content in `.agents/skills/pato/assets/examples.md`
- [ ] T004 Update `crates/pato/Cargo.toml` to support bundling and packaging verification for the canonical skill package
- [ ] T005 Create skill-specific test scaffolding in `crates/pato/tests/skill_tests.rs` and `crates/pato/tests/fixtures/skill/.gitkeep`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add shared CLI and install infrastructure that blocks all user story work.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [ ] T006 Add `Skill` command definitions and argument enums to `crates/pato/src/cli.rs`
- [ ] T007 [P] Export the new skill command surface from `crates/pato/src/commands/mod.rs` and `crates/pato/src/lib.rs`
- [ ] T008 Create install module structure and shared result/error types in `crates/pato/src/skill_install/mod.rs`
- [ ] T009 [P] Implement canonical package enumeration and metadata validation in `crates/pato/src/skill_install/package.rs`
- [ ] T010 [P] Implement install target resolution rules in `crates/pato/src/skill_install/target.rs`
- [ ] T011 Implement shared filesystem install helpers, replace guards, and copy logic in `crates/pato/src/skill_install/mod.rs`

**Checkpoint**: Foundation ready — all user story phases can now begin.

---

## Phase 3: User Story 1 — Install the bundled skill locally (Priority: P1) 🎯 MVP

**Goal**: `pato skill` installs the bundled `pato` skill to the default project-level interoperable path and reports the resolved destination.

**Independent Test**: Run `cargo run -p relateby-pato -- skill` in a clean environment and verify that a complete `pato` skill appears at the default project-level location and the command reports the installed path.

### Tests for User Story 1

- [ ] T012 [P] [US1] Add default project install and reported-path integration tests in `crates/pato/tests/skill_tests.rs`

### Implementation for User Story 1

- [ ] T013 [US1] Implement default skill install flow in `crates/pato/src/commands/skill.rs`
- [ ] T014 [US1] Wire `Commands::Skill` dispatch in `crates/pato/src/main.rs`
- [ ] T015 [US1] Implement `--print-path` handling in `crates/pato/src/commands/skill.rs` and extend assertions in `crates/pato/tests/skill_tests.rs`

**Checkpoint**: `pato skill` performs the default project install and is independently testable.

---

## Phase 4: User Story 2 — Choose install scope and target convention (Priority: P2)

**Goal**: Users can choose supported project/user scope and interoperable/client-native destinations, while project installs remain Vercel-discoverable.

**Independent Test**: Run the command for each supported install combination and verify that the skill appears only in the selected destination, with all project-scope installs resolving to a Vercel-discoverable `.agents/skills/` path.

### Tests for User Story 2

- [ ] T016 [P] [US2] Add install-target resolution coverage for supported scope/target combinations in `crates/pato/tests/skill_tests.rs`

### Implementation for User Story 2

- [ ] T017 [US2] Implement `--scope` and `--target` argument handling in `crates/pato/src/cli.rs` and `crates/pato/src/commands/skill.rs`
- [ ] T018 [US2] Implement user-scope interoperable and client-native destination resolution in `crates/pato/src/skill_install/target.rs`
- [ ] T019 [US2] Reject unsupported project-level client-native installs and surface clear errors in `crates/pato/src/commands/skill.rs` and `crates/pato/tests/skill_tests.rs`

**Checkpoint**: Supported install combinations are selectable and independently testable.

---

## Phase 5: User Story 3 — Protect existing installs and support explicit replacement (Priority: P3)

**Goal**: Existing installs are left unchanged unless the user explicitly requests replacement.

**Independent Test**: Install once, run the same command again without replacement and verify the existing install is unchanged, then rerun with explicit replacement enabled and verify the install is refreshed.

### Tests for User Story 3

- [ ] T020 [P] [US3] Add existing-install, no-replace, and explicit-replace integration tests in `crates/pato/tests/skill_tests.rs`

### Implementation for User Story 3

- [ ] T021 [US3] Add explicit replacement flag parsing to `crates/pato/src/cli.rs`
- [ ] T022 [US3] Implement existing-install detection and guarded replacement behavior in `crates/pato/src/skill_install/mod.rs`
- [ ] T023 [US3] Surface replace outcomes and conflict errors from `crates/pato/src/commands/skill.rs`

**Checkpoint**: Overwrite protection and explicit replacement are independently testable.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Packaging verification, documentation alignment, and code-quality validation across all stories.

- [ ] T024 [P] Add bundled asset packaging verification coverage in `crates/pato/tests/skill_tests.rs` and finalize package include rules in `crates/pato/Cargo.toml`
- [ ] T025 Update `specs/043-pato-skill/quickstart.md` with the final `pato skill` flags and validation steps
- [ ] T026 [P] Run `cargo fmt --all` and fix formatting issues across `.agents/skills/pato/` and `crates/pato/`
- [ ] T027 [P] Run `cargo clippy --workspace -- -D warnings` and fix warnings affecting `crates/pato/`
- [ ] T028 Run `cargo test -p relateby-pato` and validate the end-to-end quickstart flow in `specs/043-pato-skill/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 — **BLOCKS all user stories**
- **US1 (Phase 3)**: Depends on Phase 2 — MVP slice
- **US2 (Phase 4)**: Depends on US1 command wiring and Phase 2 target-resolution infrastructure
- **US3 (Phase 5)**: Depends on US1 install flow and Phase 2 filesystem install helpers
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **US1 (P1)**: Can start after Phase 2. No dependency on later stories.
- **US2 (P2)**: Builds on the `pato skill` command introduced in US1, but remains independently testable once target selection is added.
- **US3 (P3)**: Builds on the install flow from US1 and can be validated independently once replacement logic is added.

### Parallel Opportunities Within Each Story

- **Phase 1**: T002 and T003 can run in parallel after T001 defines the canonical skill root.
- **Phase 2**: T009 and T010 can run in parallel after T008 defines shared install types.
- **US1**: T012 can be authored while T013 begins the command implementation; T014 and T015 follow once the command module exists.
- **US2**: T016 can run in parallel with T017 before final target-resolution behavior is completed in T018.
- **US3**: T020 can be prepared in parallel with T021 before T022 and T023 finalize behavior.
- **Polish**: T024 and T025 can run in parallel before code-quality and end-to-end validation steps.

---

## Parallel Example: User Story 2

```text
T016 [US2] Add install-target resolution coverage in crates/pato/tests/skill_tests.rs
T017 [US2] Implement --scope and --target argument handling in crates/pato/src/cli.rs and crates/pato/src/commands/skill.rs
```

After those complete, finish:

```text
T018 [US2] Implement user-scope destination resolution in crates/pato/src/skill_install/target.rs
T019 [US2] Reject unsupported project-level client-native installs in crates/pato/src/commands/skill.rs and crates/pato/tests/skill_tests.rs
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Validate `pato skill` default project install before expanding scope

### Incremental Delivery

1. Deliver the default project install path first (US1)
2. Add explicit scope/target selection (US2)
3. Add overwrite protection and explicit replacement (US3)
4. Finish with packaging verification and code-quality validation

### Parallel Team Strategy

1. One contributor prepares the canonical skill package while another readies test scaffolding in Phase 1
2. During Phase 2, package validation and target resolution can proceed in parallel once shared types exist
3. After US1 lands, one contributor can expand destination selection while another drafts replacement-path tests

---

## Notes

- All tasks use exact file paths and follow the required checklist format.
- Test tasks are included because the plan and quickstart explicitly require targeted integration and packaging verification.
- US1 is the recommended MVP scope.
- Project-level installs must remain Vercel-discoverable throughout implementation.
