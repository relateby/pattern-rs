# Tasks: Release Branch Workflow

**Input**: Design documents from `/specs/044-release-branch-flow/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Not included. The feature spec did not explicitly request TDD, so this plan focuses on implementation tasks and doc updates.

**Organization**: Tasks are grouped by user story so each story can be implemented and reviewed independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish shared release helpers that the branch-based workflow will reuse.

- [X] T001 Add shared release branch, branch-state, and published-version helper functions in `scripts/release/common.sh`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Create the release finalization entrypoint that later stories will complete and extend.

**⚠️ CRITICAL**: No release finalization work should begin until the shared helpers exist.

- [X] T002 Add the release finalization scaffold in `scripts/release/finalize-release.sh` so later steps can verify a finalized release target before tagging

**Checkpoint**: Shared release helpers and the finalization entrypoint are ready for branch-first release work.

---

## Phase 3: User Story 1 - Prepare Releases on a Dedicated Branch (Priority: P1) 🎯 MVP

**Goal**: Move release preparation off `main` and onto a dedicated release branch.

**Independent Test**: A maintainer can start a release from `main`, produce a branch-specific version bump, and review the branch without creating the stable tag yet.

- [X] T003 [US1] Refactor `scripts/new-release.sh` to create `release/vX.Y.Z` from `main`, run `scripts/release/prerelease.sh` on that branch, and stop before creating `vX.Y.Z`
- [X] T004 [P] [US1] Update `docs/release.md` and `specs/044-release-branch-flow/quickstart.md` with the branch-first preparation flow and PR-review handoff

**Checkpoint**: Release preparation now happens on a dedicated branch instead of directly on `main`.

---

## Phase 4: User Story 2 - Validate Before Tagging (Priority: P1)

**Goal**: Create the stable release tag only after validation has succeeded and the release branch is finalized.

**Independent Test**: A maintainer can finalize a validated release branch, create the stable tag from the merged commit, and trigger the existing publish flow only after validation has passed.

- [X] T005 [US2] Complete `scripts/release/finalize-release.sh` so it can create and push the stable tag only after the release branch has been merged to `main` and validation has passed
- [X] T006 [P] [US2] Update `scripts/release/verify-tag.sh`, `docs/release.md`, and `specs/044-release-branch-flow/contracts/release-workflow.md` to describe the finalized-release-tag contract and the publish trigger

**Checkpoint**: Stable tagging is separated from branch preparation and only happens at finalization time.

---

## Phase 5: User Story 3 - Recover Without Burning a Version (Priority: P2)

**Goal**: Allow release-only fixes to be retried on the same branch before any publish occurs, while rejecting reuse once a version is public.

**Independent Test**: A maintainer can fix a release-preparation issue on the same branch before publish and reuse the version, but the release tooling rejects reuse once any registry already contains that version.

- [X] T007 [US3] Extend `scripts/release/finalize-release.sh` to reject stable-tag reuse when the shared publication helper reports that any registry already contains the version
- [X] T008 [P] [US3] Update `docs/release.md` and `specs/044-release-branch-flow/quickstart.md` with the pre-publish retry versus post-publish patch-bump recovery rule

**Checkpoint**: Pre-publish fixes can be retried on the same release branch, but published versions remain immutable.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Align release guidance and messaging across the workflow after the branch-based flow is in place.

- [X] T009 [P] Review `scripts/new-release.sh`, `scripts/release/finalize-release.sh`, and `docs/release.md` for help text, examples, and error messages that still imply direct tagging from `main`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - blocks the user story implementation steps that rely on shared release helpers
- **User Stories (Phase 3+)**: Depend on the shared helpers and finalization scaffold
- **Polish (Final Phase)**: Depends on the user story phases being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Phase 2 - no dependency on other user stories
- **User Story 2 (P1)**: Can start after Phase 2 - uses the shared finalization flow but remains independently testable
- **User Story 3 (P2)**: Can start after Phase 2, but in practice extends the same finalization path and should be completed after the tag-finalization flow is in place

### Within Each User Story

- Shared helpers before story-specific script changes
- Script behavior before documentation updates
- Story complete before moving to the next priority

### Parallel Opportunities

- `T004` can run in parallel with `T003` once the script shape is stable because it only updates docs
- `T006` can run in parallel with `T005` because it only updates docs and the contract
- `T008` can run in parallel with `T007` because it only updates docs
- `T009` can run alongside final doc cleanup and message tuning

---

## Parallel Example: User Story 1

```bash
Task: "Refactor `scripts/new-release.sh` to create `release/vX.Y.Z` from `main`, run `scripts/release/prerelease.sh` on that branch, and stop before creating `vX.Y.Z`"
Task: "Update `docs/release.md` and `specs/044-release-branch-flow/quickstart.md` with the branch-first preparation flow and PR-review handoff"
```

---

## Parallel Example: User Story 2

```bash
Task: "Complete `scripts/release/finalize-release.sh` so it can create and push the stable tag only after the release branch has been merged to `main` and validation has passed"
Task: "Update `scripts/release/verify-tag.sh`, `docs/release.md`, and `specs/044-release-branch-flow/contracts/release-workflow.md` to describe the finalized-release-tag contract and the publish trigger"
```

---

## Parallel Example: User Story 3

```bash
Task: "Extend `scripts/release/finalize-release.sh` to reject stable-tag reuse when the shared publication helper reports that any registry already contains the version"
Task: "Update `docs/release.md` and `specs/044-release-branch-flow/quickstart.md` with the pre-publish retry versus post-publish patch-bump recovery rule"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Validate that release preparation now happens on a branch and no stable tag is created yet
5. Stop and review before starting the finalization flow

### Incremental Delivery

1. Ship the branch-first preparation flow in User Story 1
2. Add the finalized-tag creation flow in User Story 2
3. Add the immutability/retry rules in User Story 3
4. Finish with polish and messaging cleanup

### Parallel Team Strategy

With multiple contributors:

1. One contributor can work on `scripts/new-release.sh` for User Story 1
2. Another can prepare the finalization flow in `scripts/release/finalize-release.sh`
3. A third can update the release docs and quickstart examples in parallel

---

## Notes

- `[P]` tasks touch different files and have no direct dependency on unfinished tasks.
- User story phases should remain independently reviewable even if they share release script helpers.
- Keep the stable tag as the publish trigger, but only create it after the release branch is validated and finalized.
