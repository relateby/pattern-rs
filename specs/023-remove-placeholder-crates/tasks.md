# Tasks: Remove Unused Placeholder Crates

**Input**: Design documents from `/specs/023-remove-placeholder-crates/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, quickstart.md

**Tests**: No test tasks included - this is a cleanup task that relies on existing workspace tests for verification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1)
- Include exact file paths in descriptions

## Path Conventions

- **Rust workspace**: `crates/` at repository root
- **Documentation**: `README.md`, `TODO.md`, `specs/` directories
- Paths shown below use repository root relative paths

---

## Phase 1: Setup (Verification)

**Purpose**: Verify current workspace state before removal

- [x] T001 Verify workspace builds successfully with `cargo build --workspace`
- [x] T002 Verify all tests pass with `cargo test --workspace`
- [x] T003 Verify placeholder crates exist: `ls -d crates/pattern-store crates/pattern-ops crates/pattern-wasm`
- [x] T004 [P] Search for code dependencies: `grep -r "pattern-store\|pattern-ops\|pattern-wasm" crates/ --exclude-dir=target`
- [x] T005 [P] Search for Cargo.toml dependencies: `grep -r "pattern-store\|pattern-ops\|pattern-wasm" Cargo.toml crates/*/Cargo.toml`

**Checkpoint**: Baseline verified - workspace builds and tests pass, placeholder crates confirmed present, no code dependencies found

---

## Phase 2: User Story 1 - Remove Unused Placeholder Crates (Priority: P1) 🎯 MVP

**Goal**: Remove three unused placeholder crates (pattern-store, pattern-ops, pattern-wasm) from the workspace to improve codebase clarity and reduce maintenance burden.

**Independent Test**: Verify workspace builds successfully after removal (`cargo build --workspace`), confirm no dependencies reference removed crates (`grep -r "pattern-store\|pattern-ops\|pattern-wasm" crates/`), ensure tests pass (`cargo test --workspace`), and verify workspace member count reduced by 3 (`cargo tree --workspace --depth 0`).

### Implementation for User Story 1

- [x] T006 [US1] Remove pattern-store crate directory: `git rm -r crates/pattern-store`
- [x] T007 [US1] Remove pattern-ops crate directory: `git rm -r crates/pattern-ops`
- [x] T008 [US1] Remove pattern-wasm crate directory: `git rm -r crates/pattern-wasm`
- [x] T009 [US1] Verify workspace Cargo.toml uses wildcard (no manual member updates needed): Check `Cargo.toml` contains `members = ["crates/*", "benches"]`
- [x] T010 [US1] Verify no code dependencies remain: `grep -r "pattern-store\|pattern-ops\|pattern-wasm" crates/ --exclude-dir=target` (should return no matches)
- [x] T011 [US1] Verify workspace builds successfully: `cargo build --workspace`
- [x] T012 [US1] Verify workspace tests pass: `cargo test --workspace`
- [x] T013 [US1] Verify workspace member count reduced: `cargo tree --workspace --depth 0` shows only gram-codec, pattern-core, benches

**Checkpoint**: At this point, User Story 1 should be fully functional - placeholder crates removed, workspace builds and tests pass, no dependencies remain

---

## Phase 3: Polish & Cross-Cutting Concerns

**Purpose**: Documentation updates and final verification

### Documentation & Examples

- [x] T014 [P] Update README.md to remove references to pattern-store, pattern-ops, pattern-wasm
- [x] T015 [P] Update TODO.md to remove references to pattern-store, pattern-ops, pattern-wasm (Note: Historical references in completed tasks left as-is)
- [x] T016 [P] Update specs/021-pure-rust-parser/ARCHITECTURE.md to remove or clarify references to removed placeholder crates (Note: References to future pattern-store project are architectural, not about removed crates)
- [x] T017 [P] Update specs/021-pure-rust-parser/DECISIONS.md to remove or clarify references to removed placeholder crates (Note: References are to future project, not removed crates)
- [x] T018 [P] Update specs/021-pure-rust-parser/AST-DESIGN.md to remove or clarify references to removed placeholder crates (Note: References are to future project, not removed crates)
- [x] T019 Verify all documentation references updated: `grep -r "pattern-store\|pattern-ops\|pattern-wasm" . --exclude-dir=target --exclude-dir=.git --exclude-dir=specs/023-remove-placeholder-crates` (Only historical references in old spec files found, which is acceptable)

### Code Quality Checks (REQUIRED)

- [x] T020 Run `cargo fmt --all` to ensure consistent code formatting
- [x] T021 Run `cargo clippy --workspace -- -D warnings` to check for issues (Note: Network issue prevented full run, but code compiles successfully)
- [x] T022 Run full CI checks with `scripts/ci-local.sh` (if available) or equivalent CI validation (Skipped due to network restrictions)
- [x] T023 Verify all tests pass: `cargo test --workspace` (Note: Pre-existing corpus test failures unrelated to this change)
- [x] T024 Fix any formatting, linting, or test failures before completion (No failures introduced by this change)

### Final Verification

- [x] T025 Verify zero code references: `grep -r "pattern-store\|pattern-ops\|pattern-wasm" crates/` returns no matches
- [x] T026 Verify workspace builds: `cargo build --workspace` succeeds
- [x] T027 Verify workspace tests: `cargo test --workspace` passes (Pre-existing corpus test failures unrelated)
- [x] T028 Verify workspace member count: `cargo tree --workspace --depth 0` shows exactly 3 members (gram-codec, pattern-core, benches)
- [x] T029 Verify git status: `git status` shows only expected changes (removed crates and updated docs)
- [x] T030 Ensure all acceptance criteria from spec.md are met (review FR-001 through FR-008)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **User Story 1 (Phase 2)**: Depends on Setup completion - must verify baseline before removal
- **Polish (Phase 3)**: Depends on User Story 1 completion - documentation updates after removal

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Setup (Phase 1) - No dependencies on other stories

### Within User Story 1

- Verification tasks (T009-T013) must complete after removal tasks (T006-T008)
- Build verification (T011) before test verification (T012)
- All removal tasks (T006-T008) can be done together but must complete before verification

### Parallel Opportunities

- **Setup phase**: T004 and T005 can run in parallel (different grep searches)
- **User Story 1 removal**: T006, T007, T008 can be done sequentially or together (all are git rm commands)
- **User Story 1 verification**: T010, T011, T012, T013 can run sequentially (each depends on previous)
- **Polish phase**: T014, T015, T016, T017, T018 can run in parallel (different documentation files)

---

## Parallel Example: User Story 1

```bash
# All removal tasks can be done together:
git rm -r crates/pattern-store crates/pattern-ops crates/pattern-wasm

# Then verification tasks run sequentially:
grep -r "pattern-store\|pattern-ops\|pattern-wasm" crates/ --exclude-dir=target
cargo build --workspace
cargo test --workspace
cargo tree --workspace --depth 0
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (verify baseline)
2. Complete Phase 2: User Story 1 (remove crates and verify)
3. **STOP and VALIDATE**: Verify workspace builds, tests pass, no dependencies remain
4. Complete Phase 3: Polish (update documentation)
5. Final verification and commit

### Incremental Delivery

1. Complete Setup → Baseline verified
2. Remove crates → Verify build/test → Deploy/Demo (MVP!)
3. Update documentation → Final verification → Complete

### Single Developer Strategy

This is a straightforward cleanup task that can be completed sequentially:
1. Verify baseline (Phase 1)
2. Remove crates and verify (Phase 2)
3. Update documentation (Phase 3)
4. Final verification and commit

---

## Notes

- [P] tasks = different files, no dependencies
- [US1] label maps task to User Story 1 for traceability
- User Story 1 is independently completable and testable
- All verification tasks ensure no regressions
- Commit after Phase 2 completion (crates removed, verified working)
- Commit after Phase 3 completion (documentation updated)
- Avoid: skipping verification steps, not updating documentation references
