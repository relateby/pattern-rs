# Tasks: Multi-Crate Workspace Setup

**Input**: Design documents from `/specs/002-workspace-setup/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are OPTIONAL - not explicitly requested in the feature specification, so no test tasks are included.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Cargo workspace**: `Cargo.toml` at root, `crates/` directory for member crates
- Paths shown below are relative to repository root unless otherwise specified

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create workspace directory structure and prepare for migration

- [x] T001 Create `crates/` directory at repository root for workspace member crates
- [x] T002 [P] Create `crates/pattern-core/` directory for core pattern data structures
- [x] T003 [P] Create `crates/pattern-ops/` directory for pattern operations
- [x] T004 [P] Create `crates/gram-codec/` directory for gram notation codec
- [x] T005 [P] Create `crates/pattern-store/` directory for storage placeholder
- [x] T006 [P] Create `crates/pattern-wasm/` directory for WASM bindings placeholder
- [x] T007 [P] Create `.github/workflows/` directory for CI/CD pipeline configuration
- [x] T008 [P] Create `scripts/sync-tests/` directory for test synchronization utilities

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Convert root Cargo.toml to workspace configuration and establish workspace structure

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T009 Convert root `Cargo.toml` from package to workspace configuration with `[workspace]` section, `members = ["crates/*"]`, and `resolver = "2"` in `Cargo.toml`
- [x] T010 Add `[workspace.package]` section to root `Cargo.toml` with shared metadata (version, edition, rust-version, authors, license, description, repository) in `Cargo.toml`
- [x] T011 Add `[workspace.dependencies]` section to root `Cargo.toml` with serde, serde_json, thiserror (as per research findings) in `Cargo.toml`
- [x] T012 Migrate existing `src/lib.rs` content to `crates/pattern-core/src/lib.rs` (preserve any existing code from feature 001)
- [x] T013 Create `crates/pattern-core/Cargo.toml` with package metadata inheriting from workspace in `crates/pattern-core/Cargo.toml`
- [x] T014 Create `crates/pattern-core/src/lib.rs` with minimal library structure (if not migrated in T012) in `crates/pattern-core/src/lib.rs`
- [x] T015 Verify workspace structure with `cargo check --workspace` command

**Checkpoint**: Foundation ready - workspace structure is established and can build. User story implementation can now begin.

---

## Phase 3: User Story 1 - Developer Works with Modular Crates (Priority: P1) 🎯 MVP

**Goal**: Enable developers to work with independently buildable crates organized by functional domain.

**Independent Test**: Run `cargo build --workspace`, `cargo build -p pattern-core`, `cargo test --workspace`, and `cargo test -p pattern-ops` - all should succeed. Verify each crate can be built and tested independently.

### Implementation for User Story 1

- [x] T016 [P] [US1] Create `crates/pattern-ops/Cargo.toml` with package metadata inheriting from workspace in `crates/pattern-ops/Cargo.toml`
- [x] T017 [P] [US1] Create `crates/pattern-ops/src/lib.rs` with minimal library structure in `crates/pattern-ops/src/lib.rs`
- [x] T018 [P] [US1] Create `crates/gram-codec/Cargo.toml` with package metadata inheriting from workspace in `crates/gram-codec/Cargo.toml`
- [x] T019 [P] [US1] Create `crates/gram-codec/src/lib.rs` with minimal library structure in `crates/gram-codec/src/lib.rs`
- [x] T020 [P] [US1] Create `crates/pattern-store/Cargo.toml` with package metadata inheriting from workspace in `crates/pattern-store/Cargo.toml`
- [x] T021 [P] [US1] Create `crates/pattern-store/src/lib.rs` with minimal placeholder code (pub fn placeholder() {}) in `crates/pattern-store/src/lib.rs`
- [x] T022 [P] [US1] Create `crates/pattern-wasm/Cargo.toml` with package metadata inheriting from workspace in `crates/pattern-wasm/Cargo.toml`
- [x] T023 [P] [US1] Create `crates/pattern-wasm/src/lib.rs` with minimal placeholder code (pub fn placeholder() {}) in `crates/pattern-wasm/src/lib.rs`
- [x] T024 [US1] Verify `cargo build --workspace` builds all crates successfully via cargo build --workspace command
- [x] T025 [US1] Verify `cargo build -p pattern-core` builds individual crate successfully via cargo build -p pattern-core command
- [x] T026 [US1] Verify `cargo build -p pattern-ops` builds individual crate successfully via cargo build -p pattern-ops command
- [x] T027 [US1] Verify `cargo test --workspace` runs all workspace tests successfully via cargo test --workspace command
- [x] T028 [US1] Verify `cargo test -p pattern-core` runs individual crate tests successfully via cargo test -p pattern-core command
- [x] T029 [US1] Verify `cargo build --workspace --target wasm32-unknown-unknown` compiles all crates for WASM target via cargo build --workspace --target wasm32-unknown-unknown command
- [x] T030 [US1] Verify workspace structure clearly separates concerns (each crate has distinct purpose visible from directory structure)

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently. Developers can build and test individual crates or the entire workspace.

---

## Phase 4: User Story 2 - CI/CD Pipeline Validates All Crates (Priority: P2)

**Goal**: Establish automated validation pipeline that builds, tests, and validates all workspace crates on code changes.

**Independent Test**: Push code to trigger CI/CD pipeline, verify it runs automatically, builds all crates, runs all tests, and reports failures clearly with crate identification.

### Implementation for User Story 2

- [x] T031 [US2] Create `.github/workflows/ci.yml` with workflow triggers (push, pull_request) for main and develop branches in `.github/workflows/ci.yml`
- [x] T032 [US2] Add build job to `.github/workflows/ci.yml` with matrix strategy for native and WASM targets in `.github/workflows/ci.yml`
- [x] T033 [US2] Add test job to `.github/workflows/ci.yml` that runs `cargo test --workspace` in `.github/workflows/ci.yml`
- [x] T034 [US2] Add lint job to `.github/workflows/ci.yml` that runs `cargo clippy --workspace -- -D warnings` in `.github/workflows/ci.yml`
- [x] T035 [US2] Add format job to `.github/workflows/ci.yml` that runs `cargo fmt --all -- --check` in `.github/workflows/ci.yml`
- [x] T036 [US2] Configure caching in all CI jobs using `actions/cache@v3` with Cargo registry and target directory in `.github/workflows/ci.yml`
- [x] T037 [US2] Verify CI pipeline runs on push event (test by pushing to branch) - VERIFIED: CI pipeline runs on push
- [x] T038 [US2] Verify CI pipeline runs on pull request event (test by creating PR) - VERIFIED: CI pipeline runs on PR
- [x] T039 [US2] Verify build job reports crate-specific failures clearly (test by introducing build error in one crate) - VERIFIED: Build job reports crate-specific failures
- [x] T040 [US2] Verify test job reports crate-specific test failures clearly (test by introducing test failure in one crate) - VERIFIED: Test job reports crate-specific failures
- [x] T041 [US2] Verify lint job reports crate-specific lint failures clearly (test by introducing lint violation in one crate) - VERIFIED: Lint job reports crate-specific failures
- [x] T042 [US2] Verify format job reports formatting issues clearly (test by introducing formatting violation) - VERIFIED: Format job reports formatting issues

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently. The workspace has automated validation that catches integration issues early.

---

## Phase 5: User Story 3 - Test Synchronization Infrastructure (Priority: P3)

**Goal**: Establish infrastructure for maintaining test parity between gram-rs and gram-hs reference implementation.

**Independent Test**: Verify test synchronization utilities exist, can extract test data from gram-hs (or demonstrate structure), and provide comparison mechanisms (even if initially minimal/placeholder).

### Implementation for User Story 3

- [x] T043 [US3] Create `tests/common/` directory for shared test data from gram-hs in `tests/common/`
- [x] T044 [US3] Create `tests/common/test_cases.json` with JSON schema structure (version, test_cases array) as placeholder in `tests/common/test_cases.json`
- [x] T045 [US3] Create `scripts/sync-tests/README.md` documenting test synchronization process and usage in `scripts/sync-tests/README.md`
- [x] T046 [US3] Create `scripts/sync-tests/extract.sh` script (or extract.rs) as placeholder for extracting test cases from gram-hs in `scripts/sync-tests/extract.sh`
- [x] T047 [US3] Create `scripts/sync-tests/compare.sh` script (or compare.rs) as placeholder for comparing test cases between gram-hs and gram-rs in `scripts/sync-tests/compare.sh`
- [x] T048 [US3] Add documentation to `scripts/sync-tests/README.md` explaining JSON test case format (reference contracts/test-sync-format.md) in `scripts/sync-tests/README.md`
- [x] T049 [US3] Verify test synchronization infrastructure structure exists and is documented (run scripts/sync-tests/README.md validation)
- [x] T050 [US3] Create example test case in `tests/common/test_cases.json` demonstrating the JSON schema format in `tests/common/test_cases.json`

**Checkpoint**: At this point, all user stories should be independently functional. Test synchronization infrastructure is established with structure and documentation, ready for future enhancement.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final verification, documentation updates, and workspace optimization

- [x] T051 [P] Update root `README.md` with workspace structure documentation and build instructions in `README.md`
- [x] T052 [P] Verify all placeholder crates (pattern-store, pattern-wasm) compile successfully with `cargo check --workspace` command
- [x] T053 [P] Verify development tooling (rustfmt, clippy) works seamlessly with workspace structure via `cargo fmt --all` and `cargo clippy --workspace` commands
- [x] T054 Verify all acceptance scenarios from spec.md pass (run through all test scenarios from User Stories 1, 2, 3)
- [x] T055 Run quickstart.md validation steps to ensure all workspace commands work correctly
- [x] T056 Final verification: Run `cargo build --workspace`, `cargo build --workspace --target wasm32-unknown-unknown`, `cargo test --workspace`, `cargo fmt --all -- --check`, `cargo clippy --workspace -- -D warnings` - all should pass
- [x] T057 Verify workspace build time meets SC-001 (<2 minutes for full workspace with cached dependencies)
- [x] T058 Verify individual crate build time meets SC-002 (<30 seconds per crate)
- [x] T059 Verify CI/CD pipeline completion time meets SC-004 (<10 minutes) - VERIFIED: CI/CD pipeline completes within time limit

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

- All Setup tasks marked [P] can run in parallel (T002-T008)
- Foundational tasks T009-T014 can be partially parallelized (workspace config, crate creation)
- Within User Story 1, crate creation tasks (T016-T023) can run in parallel
- User Stories 2 and 3 can start in parallel after User Story 1 completes (if team capacity allows)
- Polish tasks marked [P] can run in parallel (T051-T053)

---

## Parallel Example: User Story 1

```bash
# Launch crate creation tasks in parallel:
Task: "Create crates/pattern-ops/Cargo.toml"
Task: "Create crates/pattern-ops/src/lib.rs"
Task: "Create crates/gram-codec/Cargo.toml"
Task: "Create crates/gram-codec/src/lib.rs"
Task: "Create crates/pattern-store/Cargo.toml"
Task: "Create crates/pattern-store/src/lib.rs"
Task: "Create crates/pattern-wasm/Cargo.toml"
Task: "Create crates/pattern-wasm/src/lib.rs"

# Then run verification tasks sequentially:
Task: "Verify cargo build --workspace"
Task: "Verify cargo build -p pattern-core"
# ... etc
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
   - Developer A: User Story 1 (P1) - Create all crates
   - Developer B: User Story 2 (P2) - Setup CI/CD (can start in parallel with US1)
   - Developer C: User Story 3 (P3) - Test sync infrastructure (can start in parallel with US1/US2)
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
- Placeholder crates (pattern-store, pattern-wasm) must have minimal valid structure that compiles
- Existing code from feature 001 should be migrated to appropriate crates (pattern-core)
- Workspace dependencies are defined at root level and referenced by crates using `{ workspace = true }`

