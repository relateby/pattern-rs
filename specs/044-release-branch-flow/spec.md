# Feature Specification: Release Branch Workflow

**Feature Branch**: `044-release-branch-flow`  
**Created**: 2026-03-21  
**Status**: Draft  
**Input**: User description: "release branches would be a cleaner approach. Incorporate the discussion above into a feature plan to improve the release process"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Prepare Releases on a Dedicated Branch (Priority: P1)

As a release maintainer, I want each release to be prepared on a dedicated release branch so that version bumps and release-only fixes can be reviewed before the release is finalized.

**Why this priority**: This removes the current dependency on a direct release workflow from `main` and gives maintainers a controlled place to make release-related changes without disrupting ongoing development.

**Independent Test**: A maintainer can cut a release branch for a chosen version, apply the version bump and any release-only fixes there, and verify that the release candidate is isolated from `main`.

**Acceptance Scenarios**:

1. **Given** an up-to-date `main` branch, **When** a maintainer starts a release for a specific version, **Then** a dedicated release branch is created for that version.
2. **Given** a release branch, **When** release-managed files are updated, **Then** the changes remain confined to the release branch until the branch is merged.

---

### User Story 2 - Validate Before Tagging (Priority: P1)

As a release maintainer, I want validation to complete before a stable tag is created so that failed release attempts do not leave dangling tags behind.

**Why this priority**: The current failure mode creates stable tags too early, which forces unnecessary version bumps even when the only issue is a release script or build fix.

**Independent Test**: A maintainer can run the release validation flow on a release branch and confirm that no stable tag exists unless validation succeeds.

**Acceptance Scenarios**:

1. **Given** a release branch with a failing validation step, **When** validation runs, **Then** no stable tag is created.
2. **Given** a release branch with passing validation, **When** the release is finalized, **Then** the stable tag is created only after validation completes successfully.

---

### User Story 3 - Recover Without Burning a Version (Priority: P2)

As a release maintainer, I want to fix release preparation issues on the release branch without incrementing the version number so that build-script changes do not consume a new patch release.

**Why this priority**: Release tooling changes should not require a new product version unless a publish has already started or completed.

**Independent Test**: A maintainer can correct a validation failure on the release branch, rerun validation, and complete the same version release without creating a new version number.

**Acceptance Scenarios**:

1. **Given** a release branch that has not published any artifacts, **When** a validation issue is fixed on that branch, **Then** the same release version can be validated again.
2. **Given** a release version that has already been published to any registry, **When** a maintainer attempts to reuse that version, **Then** the process rejects the reuse and requires a new version.

### Edge Cases

- A release branch already exists for the target version and must be updated rather than duplicated.
- Validation fails after the version bump but before any publish step begins.
- A publish step begins and one registry accepts the version while a later step fails.
- The release branch diverges from `main` after it is cut and must be refreshed before finalization.
- A maintainer needs to make a release-script-only fix that should not change product-facing behavior.

### Assumptions

- Release branches use a predictable naming pattern tied to the target version, such as `release/vX.Y.Z`.
- The stable release tag remains `vX.Y.Z` and is created only after the release is ready to publish.
- Release validation can be repeated on the same branch without changing the version number as long as no public registry has accepted that version.
- If any public registry has already accepted the version, the release is considered immutable and a new patch version is required.
- Release branches are merged back to `main` after the release has been finalized.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The release process MUST prepare each release on a dedicated branch associated with the target stable version.
- **FR-002**: The release branch MUST be created from the current `main` branch state.
- **FR-003**: Release-managed version updates MUST occur on the release branch rather than directly on `main`.
- **FR-004**: The stable release tag MUST NOT be created until release validation has completed successfully.
- **FR-005**: If release validation fails, the release process MUST leave the stable tag absent.
- **FR-006**: If a release version has not yet been published, the maintainer MUST be able to fix release-preparation issues on the same release branch without changing the version number.
- **FR-007**: If any public artifact for a version has already been published, the release process MUST treat that version as immutable and require a new patch version for further changes.
- **FR-008**: The release process MUST provide a clear path to merge the finished release branch back into `main`.
- **FR-009**: Release documentation MUST describe the release-branch lifecycle, final tagging step, and recovery rules for failed validation or partial publishing.

### Key Entities *(include if feature involves data)*

- **Release Branch**: The temporary branch that carries release-managed changes for a single target version.
- **Release Candidate**: The release branch state that is being validated before final publication.
- **Stable Tag**: The immutable version tag that marks a completed release.
- **Published Release**: The versioned artifacts that have been accepted by one or more public registries.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of successful releases create the stable tag only after validation has passed.
- **SC-002**: 0 release attempts that fail validation leave behind a stable release tag.
- **SC-003**: Release maintainers can correct a pre-publish release tooling issue and revalidate the same version without incrementing the version number in at least 95% of such cases.
- **SC-004**: 100% of release documentation examples describe a branch-based release flow and the rules for recovery after failure.
