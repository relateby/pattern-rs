# Tasks: pato help and self-documentation

**Input**: Design documents from `/specs/045-pato-help/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/cli-contract.md, quickstart.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the topic corpus directory that all subsequent phases depend on.

- [X] T001 Create `crates/pato/skill-package/pato/reference/` directory. If `.agents/skills/pato/` already exists on this machine, also create `.agents/skills/pato/reference/` so that `build.rs` (which prefers the installed location) picks up the new topic files during compilation.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Author the topic markdown files and embed them in the binary. All user story phases depend on this content existing.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [X] T002 [P] Author topic file `crates/pato/skill-package/pato/reference/gram-notation.md` — include: one-sentence definition, syntax rules, semantic rules, 2–4 examples, related topics section
- [X] T003 [P] Author topic file `crates/pato/skill-package/pato/reference/stdout-stderr-contracts.md` — adapt from `crates/pato/skill-package/pato/references/output-contracts.md`; include: rule summary, rationale, what goes to stdout vs stderr, examples
- [X] T004 Create `crates/pato/src/topic_catalog.rs` with `TopicEntry` struct, `TOPICS: &[TopicEntry]` static using `include_str!` for each topic file, `find_topic(name: &str) -> Option<&'static TopicEntry>`, and `topic_names() -> impl Iterator<Item = &'static str>` (depends on T002, T003)
- [X] T005 Register `pub mod topic_catalog;` in `crates/pato/src/lib.rs` and verify `cargo build -p relateby-pato` compiles cleanly (proves embed paths resolve)

**Checkpoint**: `cargo build -p relateby-pato` succeeds — topic content is embedded in the binary.

---

## Phase 3: User Story 1 — Fast help discovery (Priority: P1) 🎯 MVP

**Goal**: `pato -h` presents a concise scan-friendly command list; `pato --help` provides full usage. The new `help` subcommand appears in the command listing.

**Independent Test**: Run `pato -h` and confirm output is short (fits one screen), lists commands including `help`, and does not include long descriptions. Run `pato --help` and confirm full description and examples are present.

### Implementation for User Story 1

- [X] T006 [US1] Update `crates/pato/src/cli.rs`: set concise `about` strings on all `Commands` enum variants so `-h` output is compact; set `long_about` on variants that need extended description for `--help`
- [X] T007 [US1] Add `Commands::Help(HelpArgs)` variant to the `Commands` enum in `crates/pato/src/cli.rs` with `about = "Show help for a topic"` so the help subcommand appears in `-h` output (depends on T006)

**Checkpoint**: `pato -h` shows a compact command list including `help`; `pato --help` shows full usage. User Story 1 is independently testable.

---

## Phase 4: User Story 2 — Topic-based guidance (Priority: P1)

**Goal**: `pato help <topic>` prints topic content to stdout and exits 0. `pato help` with no topic or an unknown topic prints an error and the topic list to stderr and exits 1.

**Independent Test**: Run `pato help gram-notation` and confirm output contains the topic content. Run `pato help no-such-topic` and confirm exit code 1, stderr contains an error message and the list of available topics.

### Implementation for User Story 2

- [X] T008 [US2] Add `HelpArgs` struct to `crates/pato/src/cli.rs` with `topic: Option<String>` field and doc comment (depends on T007)
- [X] T009 [US2] Create `crates/pato/src/commands/help.rs` implementing `pub fn run(args: &HelpArgs) -> ExitCode` — known topic prints content to stdout and returns `ExitCode::SUCCESS`; no topic or unknown topic prints error and topic list to stderr and returns `ExitCode::FAILURE` (depends on T004, T008)
- [X] T010 [US2] Register `pub mod help;` in `crates/pato/src/commands/mod.rs` (depends on T009)
- [X] T011 [US2] Add `Commands::Help(args) => commands::help::run(&args)` dispatch arm in `crates/pato/src/main.rs` (depends on T010)
- [X] T012 [US2] Write integration tests in `crates/pato/tests/help_tests.rs` covering: known topic exits 0 and stdout contains content; unknown topic exits 1 and stderr contains error + topic list; no topic exits 1 and stderr contains topic list; all catalog entries resolve to non-empty content; stdout for a topic containing a fenced code block includes the opening fence (` ``` `) intact and untruncated (depends on T011)

**Checkpoint**: `pato help gram-notation` works end-to-end. `pato help` and `pato help bad-topic` fail clearly with topic list. User Story 2 is independently testable.

---

## Phase 5: User Story 3 — Packaged and installed reference docs (Priority: P2)

**Goal**: `pato skill` installs the `reference/` directory and its topic files alongside the rest of the skill tree.

**Independent Test**: Run `pato skill --scope project`, then confirm `.agents/skills/pato/reference/gram-notation.md` and `.agents/skills/pato/reference/stdout-stderr-contracts.md` exist and are non-empty.

### Implementation for User Story 3

- [X] T013 [US3] Wire the generated bundle into `crates/pato/src/lib.rs`: add `include!(concat!(env!("OUT_DIR"), "/skill_bundle.rs"));` to expose the `SKILL_BUNDLE: &[(&str, &[u8])]` constant generated by `build.rs`
- [X] T014 [US3] Rewrite `crates/pato/src/skill_install/package.rs`: replace `locate_canonical_bundle()` and the filesystem directory-copy logic with a function that iterates `SKILL_BUNDLE` and writes each embedded `(&str path, &[u8] content)` entry to the resolved install target directory, creating subdirectories as needed (depends on T013)
- [X] T015 [US3] Remove or guard behind a dev-only feature flag the `locate_canonical_bundle()` fallback paths that search for `skill-package/pato/` on disk, so the production binary is fully self-contained (depends on T014). Also verify the `ExistingInstallPresent` error message in `crates/pato/src/skill_install/mod.rs` explicitly tells the user to re-run with `--force` (FR-010); update the message if it does not.
- [X] T016 [US3] Update `crates/pato/tests/skill_tests.rs`: assert that after a simulated install the installed files are present AND their content matches the corresponding `SKILL_BUNDLE` entries byte-for-byte; include an assertion for `reference/gram-notation.md` and `reference/stdout-stderr-contracts.md` specifically (depends on T015)

**Checkpoint**: Skill install includes reference topic files. User Story 3 is independently testable.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Code quality, CI compliance, and final validation across all stories.

- [X] T017 Update `crates/pato/skill-package/pato/SKILL.md` to note that re-running `pato skill --force` after a binary upgrade re-syncs the installed skill tree with the new binary's embedded content
- [X] T018 Run `cargo test -p relateby-pato` and fix any failures
- [X] T019 [P] Run `cargo clippy --workspace -- -D warnings` and fix all warnings
- [X] T020 [P] Run `cargo fmt --all` and commit formatting fixes
- [X] T021 Run `scripts/ci-local.sh` and confirm full CI pass

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Phase 2 (topic_catalog.rs must exist so `Commands::Help` can be added)
- **US2 (Phase 4)**: Depends on Phase 2 and Phase 3
- **US3 (Phase 5)**: Depends on Phase 2 (topic files must exist); can run in parallel with Phase 4
- **Polish (Phase 6)**: Depends on all desired stories being complete

### User Story Dependencies

- **US1 (P1)**: Depends on Foundational; no dependency on US2 or US3
- **US2 (P1)**: Depends on Foundational and US1 (T007 adds `Commands::Help` variant that T008 builds on)
- **US3 (P2)**: Depends on Foundational only; independent of US1 and US2

### Within Each User Story

- Models (`topic_catalog.rs`) before services/commands
- CLI struct additions before command handler
- Command handler before dispatch wiring
- Dispatch wiring before integration tests

### Parallel Opportunities

- T002 and T003 (topic file authoring) can run in parallel — different files
- T019 and T020 (clippy and fmt) can run in parallel — different tools
- US3 (Phase 5) can proceed in parallel with US2 (Phase 4) — no shared files

---

## Parallel Example: Foundational Phase

```bash
# Author both topic files concurrently:
Task: T002 — gram-notation.md
Task: T003 — stdout-stderr-contracts.md
```

## Parallel Example: Polish Phase

```bash
# Run quality checks concurrently:
Task: T019 — cargo clippy
Task: T020 — cargo fmt
```

---

## Implementation Strategy

### MVP First (US1 + US2 only)

1. Complete Phase 1: Setup (T001)
2. Complete Phase 2: Foundational (T002–T005) — blocks everything
3. Complete Phase 3: US1 — fast discovery (T006–T007)
4. Complete Phase 4: US2 — topic help (T008–T012)
5. **STOP and VALIDATE**: `pato help gram-notation` works; `pato help` and bad topic fail clearly
6. Ship if ready

### Full Delivery

1. Complete MVP above
2. Add Phase 5: US3 — embedded skill install (T013–T016)
3. Polish: Phase 6 (T017–T021)

---

## Notes

- [P] tasks = different files, no blocking dependencies
- [Story] label maps each task to its user story for traceability
- T013 is a verification task — if `pato skill` already copies subdirectories correctly, no code change is needed; the task confirms and adds a test
- Topic file content (T002, T003) is part of the public contract once shipped — write carefully
- Commit after completing each phase checkpoint
