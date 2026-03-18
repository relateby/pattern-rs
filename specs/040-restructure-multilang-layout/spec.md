# Feature Specification: Multi-Language Repository Restructure

**Feature Branch**: `040-restructure-multilang-layout`  
**Created**: 2026-03-18  
**Status**: Draft  
**Input**: User description: "restructure the project for a cleaner multi-language setup as described in @proposals/project-restructure-proposal.md"

## Clarifications

### Session 2026-03-18

- Q: Should `@relateby/graph` and `@relateby/gram` remain internal, or become public package surfaces in this restructure? → A: Make both public package surfaces now.
- Q: Should legacy examples be archived in-repo or removed once replaced? → A: Archive legacy examples in-repo.
- Q: Should `pattern-wasm` remain discoverable as a contributor-facing adapter package, or be de-emphasized as an implementation detail? → A: Keep it discoverable as a contributor-facing adapter package.
- Q: Should this feature deliver only the medium-churn target layout, or also include a later fully symmetric `implementations/`-style layout? → A: Deliver only the medium-churn target layout in this feature.
- Q: Should the root `src/` directory be removed in this feature once confirmed unused, or deferred? → A: Remove `src/` in this feature once confirmed unused.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Navigate the repository confidently (Priority: P1)

As a contributor, I can look at the repository root and quickly understand which areas are peer implementations, which areas are adapters, which package surfaces are public, and which material is historical, so I can start work in the right place without guesswork.

**Why this priority**: This is the core value of the restructuring. If repository roles remain ambiguous, the cleanup does not solve the primary onboarding and navigation problem.

**Independent Test**: Can be fully tested by reviewing the repository tree and active top-level guidance to confirm that a contributor can correctly classify each major area and find the intended starting point for common tasks.

**Acceptance Scenarios**:

1. **Given** a contributor starts at the repository root, **When** they review the top-level structure, **Then** they can distinguish peer implementation areas, adapter areas, examples, support material, and archived material without relying on tribal knowledge.
2. **Given** a contributor needs the supported package surface for an implementation area, **When** they inspect the repository paths and active guidance, **Then** the supported surface is clearly distinguished from internal-only packages, with `@relateby/pattern`, `@relateby/graph`, and `@relateby/gram` presented as supported public TypeScript package surfaces and `pattern-wasm` presented as a discoverable adapter package rather than a peer implementation surface.

---

### User Story 2 - Remove stale and misleading structure safely (Priority: P2)

As a maintainer, I can eliminate or relocate stale directories, misleading entry points, and superseded examples or notes so the repository reflects the current product surface instead of historical leftovers.

**Why this priority**: Stale structure is a direct source of confusion and mistrust. Removing it is necessary to make the new layout credible and maintainable.

**Independent Test**: Can be fully tested by comparing the pre-change and post-change repository structure and confirming that previously stale or misleading areas have either been removed, archived, or replaced with a single canonical location.

**Acceptance Scenarios**:

1. **Given** a path no longer represents an active package root, entry point, or supported example, **When** the restructuring is applied, **Then** that path is removed or moved out of active guidance.
2. **Given** material still has historical value but is no longer normative, **When** the restructuring is applied, **Then** that material is retained only in an archive-oriented location and is not presented as current guidance.
3. **Given** the root `src/` directory is confirmed unused, **When** the restructuring is applied, **Then** `src/` is removed in this feature rather than deferred.

---

### User Story 3 - Follow examples and docs that match the current product surface (Priority: P3)

As a user or contributor, I can rely on examples and documentation to reflect the current supported package boundaries and repository roles, so I do not follow outdated instructions or unsupported paths.

**Why this priority**: Clear structure must be reinforced by clear examples and documentation; otherwise the repository remains confusing even after directories are renamed.

**Independent Test**: Can be fully tested by sampling active examples and active docs to verify that they point to canonical paths, describe the repository as multi-language, and avoid outdated boundaries.

**Acceptance Scenarios**:

1. **Given** a user opens an active example or usage guide, **When** they follow its references, **Then** the referenced paths and package boundaries match the current supported layout.
2. **Given** a contributor reads active repository guidance, **When** they look for current package roles, **Then** the guidance describes the repository as a multi-language monorepo with explicit public, internal, adapter, example, and archive roles where relevant.

### Edge Cases

- What happens when a historical note is still useful for context but conflicts with active guidance? It must be preserved only as archived material and clearly separated from normative documentation.
- What happens when a moved area remains referenced from active docs or examples? The old reference must be removed or updated so only one canonical path is presented as current.
- How does the system handle any remaining internal support areas that contributors still need to discover? They must remain discoverable without being presented as peer public surfaces.
- How does the system handle adapter areas that contributors still need to discover? Adapter packages such as `pattern-wasm` must remain easy to find in the repository and active guidance without being presented as peer public implementation surfaces.
- How does the system handle examples that mix active and legacy usage patterns? Active examples must move under current implementation-oriented groupings, and legacy examples must be archived in-repo rather than removed by default.
- What happens when the root `src/` directory is still referenced by active guidance or scripts? Those references must be updated or removed before `src/` is deleted in this feature.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The repository MUST present a top-level structure that makes the roles of peer implementations, adapters, examples, support material, and archived material clearly distinguishable.
- **FR-002**: The repository MUST separate peer implementation areas from adapter-only areas so they are not presented as the same kind of package surface.
- **FR-003**: The repository MUST distinguish supported public package surfaces from internal-only packages through path naming and active guidance.
- **FR-003a**: The repository MUST present `@relateby/pattern`, `@relateby/graph`, and `@relateby/gram` as supported public TypeScript package surfaces in the restructured layout and active guidance.
- **FR-004**: The repository MUST remove, relocate, or archive stale paths that no longer represent active package roots, supported entry points, or current examples.
- **FR-004a**: The root `src/` directory MUST be removed in this feature once it is confirmed unused and all active references to it have been eliminated.
- **FR-005**: The repository MUST organize active examples by current implementation area or current supported surface rather than by superseded package boundaries.
- **FR-005a**: Legacy or superseded examples MUST be moved to an in-repository archive area rather than deleted as part of this restructure, unless they are confirmed to have no remaining reference value.
- **FR-006**: The repository MUST isolate historical notes, review memos, migration summaries, and similar superseded material from active documentation whenever they are retained.
- **FR-007**: The repository MUST ensure that active root-facing guidance describes the repository as a multi-language monorepo rather than implying a single-language project.
- **FR-008**: The repository MUST provide a single canonical location for each active package surface, example set, and guidance document after restructuring.
- **FR-009**: The repository MUST preserve contributor discoverability for adapter areas and any remaining internal support areas while clearly signaling that they are not peer public surfaces.
- **FR-009a**: The repository MUST present `pattern-wasm` as a discoverable contributor-facing adapter package in the restructured layout and active guidance, without classifying it as a peer public implementation surface.
- **FR-010**: The restructuring MUST include migration guidance for contributors wherever renamed or moved paths are likely to disrupt routine workflows.
- **FR-011**: This feature MUST deliver the medium-churn target repository layout only and MUST NOT expand scope to a fully symmetric `implementations/`-style repository redesign.

### Key Entities *(include if feature involves data)*

- **Repository Area**: A top-level or nested area with a clear role such as peer implementation, adapter, example, support material, or archive.
- **Public Package Surface**: A package location explicitly presented as supported for external use.
- **Internal Package**: A package location retained for implementation support or future use but not presented as part of the current supported public surface.
- **Adapter Area**: A package or directory whose purpose is to bridge between implementation areas rather than serve as a peer implementation surface.
- **Example Collection**: A grouped set of runnable or instructional examples associated with a current supported surface or archived as legacy material.
- **Archived Material**: Historical notes, examples, or guidance retained for reference but not treated as current documentation.
- **Migration Guidance**: Contributor-facing instructions that explain renamed, moved, archived, or removed locations affected by the restructure.

### Assumptions

- `@relateby/pattern`, `@relateby/graph`, and `@relateby/gram` are all treated as supported public TypeScript package surfaces in this feature.
- Legacy examples are archived in-repository by default during this restructure so historical usage context remains available.
- `pattern-wasm` remains discoverable as a contributor-facing adapter package in this feature.
- Material that is still historically useful is archived in-repository; material with no ongoing value may be removed.
- The restructuring may be delivered in phases as long as each completed phase leaves active paths and guidance in a coherent state.
- A later fully symmetric repository layout may be considered separately, but it is not part of this feature.
- The root `src/` directory is expected to be removed in this feature once it is confirmed unused.

### Out of Scope

- Changing the behavior or public capabilities of the underlying products.
- Defining new publishable package surfaces beyond clarifying the current intended ones.
- Redesigning implementation internals except where path or guidance updates are needed to support the repository restructure.
- Delivering a fully symmetric `implementations/`-style repository layout in this feature.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In acceptance review, contributors can correctly identify the repository location for a peer implementation area, an adapter area, an active example area, and archived material within 2 minutes from the repository root.
- **SC-002**: 100% of sampled active root-facing guidance and active examples reference canonical current paths only, with no references to stale active entry points.
- **SC-003**: Reviewers can classify every top-level repository area into a defined role with no unresolved ambiguity at sign-off.
- **SC-004**: At least 90% of sampled onboarding tasks related to finding the correct package surface or example location are completed on the first attempt during review.
