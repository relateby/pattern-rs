# Tasks: Rust Project Initialization

**Input**: Design documents from `/specs/001-rust-init/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are OPTIONAL - not explicitly requested in the feature specification, so no test tasks are included.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- Paths shown below assume single project structure per plan.md

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create standard Rust directory structure (src/, tests/, examples/, benches/) at repository root
- [x] T002 [P] Create Cargo.toml with package metadata (name, version, edition, authors, license, description, repository, rust-version) in Cargo.toml
- [x] T003 [P] Create .gitignore file with Rust-specific ignores (target/, **/*.rs.bk, IDE files, OS files) in .gitignore
- [x] T004 [P] Create rustfmt.toml configuration file with edition = "2021" in rustfmt.toml
- [x] T005 [P] Create clippy.toml configuration file for pedantic lints in clippy.toml
- [x] T006 [P] Create LICENSE file with BSD-3-Clause license text in LICENSE

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T007 Create minimal src/lib.rs library root file with test module structure in src/lib.rs
- [x] T008 Configure Cargo.toml [lib] section with name and path in Cargo.toml
- [x] T009 Add [features] section to Cargo.toml for conditional compilation (default = []) in Cargo.toml
- [x] T010 Verify project compiles with `cargo build` command
- [x] T011 Verify project passes `cargo check` command

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Rust Developer Sets Up Project (Priority: P1) 🎯 MVP

**Goal**: Enable Rust developers to clone the repository and immediately start working with a properly configured project that builds, tests, and validates successfully.

**Independent Test**: Clone repository, run `cargo build`, `cargo test`, `cargo check`, `cargo clippy`, and `cargo fmt --check` - all should succeed without errors.

### Implementation for User Story 1

- [x] T012 [US1] Create README.md with project description, build instructions, and link to gram-hs reference in README.md
- [x] T013 [US1] Add [dev-dependencies] section to Cargo.toml (empty initially, ready for future test utilities) in Cargo.toml
- [x] T014 [US1] Verify `cargo build` succeeds and produces library artifact in target/debug/
- [x] T015 [US1] Verify `cargo test` runs successfully (even with empty test suite) via cargo test command
- [x] T016 [US1] Verify `cargo check` validates project without errors via cargo check command
- [x] T017 [US1] Verify `cargo fmt --check` executes without errors via cargo fmt --check command
- [x] T018 [US1] Verify `cargo clippy` executes without errors via cargo clippy command

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently. Developers can clone, build, test, and validate the project.

---

## Phase 4: User Story 2 - WASM Target Compilation (Priority: P2)

**Goal**: Enable compilation of the library for WebAssembly to support browser and Node.js integration.

**Independent Test**: Run `cargo build --target wasm32-unknown-unknown` and verify successful compilation without errors, even if library is initially empty.

### Implementation for User Story 2

- [x] T019 [US2] Verify WASM target is available (provide instructions if missing) via rustup target list command
- [x] T020 [US2] Add conditional compilation feature flag for WASM-specific code in Cargo.toml [features] section
- [x] T021 [US2] Verify `cargo build --target wasm32-unknown-unknown` compiles successfully via cargo build --target wasm32-unknown-unknown
- [x] T022 [US2] Verify WASM artifact is created in target/wasm32-unknown-unknown/debug/ directory
- [x] T023 [US2] Update README.md with WASM target installation instructions in README.md
- [x] T024 [US2] Add note about WASM compatibility constraints (no blocking I/O, no file system) in README.md

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently. The project compiles for both native Rust and WASM targets.

---

## Phase 5: User Story 3 - External Language Binding Examples (Priority: P3)

**Goal**: Provide minimal working examples demonstrating library usage from external language targets (JavaScript/TypeScript for WASM, with placeholders for Python and C).

**Independent Test**: Verify that example directories exist with README files explaining how to build and run examples, and that the WASM example compiles successfully.

### Implementation for User Story 3

- [x] T025 [US3] Create examples/ directory structure at repository root
- [x] T026 [US3] Create examples/README.md with overview of available examples in examples/README.md
- [x] T027 [US3] Create examples/wasm-js/ directory for WASM/JavaScript example
- [x] T028 [US3] Create examples/wasm-js/Cargo.toml with package metadata, lib crate-type, and wasm-bindgen dependency in examples/wasm-js/Cargo.toml
- [x] T029 [US3] Create examples/wasm-js/src/lib.rs with minimal wasm-bindgen example demonstrating library usage in examples/wasm-js/src/lib.rs
- [x] T030 [US3] Create examples/wasm-js/README.md with build and run instructions for WASM example in examples/wasm-js/README.md
- [x] T031 [US3] Create examples/wasm-js/www/ directory for web assets (placeholder for future HTML/JS files)
- [x] T032 [US3] Verify examples/wasm-js compiles with `cargo build --target wasm32-unknown-unknown` in examples/wasm-js/ directory
- [x] T033 [US3] Add placeholder documentation for future Python binding example in examples/README.md
- [x] T034 [US3] Add placeholder documentation for future C binding example in examples/README.md

**Checkpoint**: At this point, all user stories should be independently functional. The project has a working WASM example and placeholders for other language bindings.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T035 [P] Update README.md with complete quickstart guide based on quickstart.md from contracts
- [x] T036 [P] Add troubleshooting section to README.md covering common issues (missing Rust version, missing WASM target, etc.) in README.md
- [x] T037 Verify all acceptance scenarios from spec.md pass (run through all test scenarios)
- [x] T038 Run quickstart.md validation steps to ensure all instructions work correctly
- [x] T039 Final verification: Run `cargo build`, `cargo build --target wasm32-unknown-unknown`, `cargo test`, `cargo fmt --check`, `cargo clippy` - all should pass

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
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Independent of US1, can run in parallel
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Independent of US1/US2, can run in parallel

### Within Each User Story

- Core implementation tasks before verification tasks
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel (T002-T006)
- All Foundational tasks can run sequentially (they build on each other)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- Within User Story 3, tasks T025-T031 can be partially parallelized (directory creation, file creation)

---

## Parallel Example: User Story 3

```bash
# Launch directory and file creation tasks in parallel:
Task: "Create examples/ directory structure"
Task: "Create examples/README.md with overview"
Task: "Create examples/wasm-js/ directory"
Task: "Create examples/wasm-js/www/ directory"

# Then create files (some can be parallel):
Task: "Create examples/wasm-js/Cargo.toml"
Task: "Create examples/wasm-js/src/lib.rs"
Task: "Create examples/wasm-js/README.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Verify all acceptance scenarios pass
6. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (P1)
   - Developer B: User Story 2 (P2) - can start in parallel with US1
   - Developer C: User Story 3 (P3) - can start in parallel with US1/US2
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- All file paths are relative to repository root unless otherwise specified
- Verification tasks use `cargo` commands and should be run from repository root

