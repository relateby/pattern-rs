# Feature Specification: Consolidated Stable Publishing Workflow

**Feature Branch**: `037-publishing-workflow`  
**Created**: 2026-03-16  
**Status**: Draft  
**Input**: User description: "Update publishing workflow as described above"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Prepare A Release From Main (Priority: P1)

As a maintainer, I want a single `scripts/new-release.sh` flow that prepares a release from the current `main` branch so that version bumps, validation, and tagging happen consistently before any remote publishing occurs.

**Why this priority**: This is the entry point for the entire release process. Without a reliable preparation flow, automated publishing increases the chance of version drift and broken releases.

**Independent Test**: Can be fully tested by running `scripts/new-release.sh <version>` on a clean checkout of `main` and verifying that it updates all release-managed versions, runs the release validation suite, and creates the expected release commit and annotated tag without publishing locally.

**Acceptance Scenarios**:

1. **Given** a clean checkout on an up-to-date `main`, **When** a maintainer runs `scripts/new-release.sh 0.2.0`, **Then** all release-managed manifests are version-aligned, the release validation suite passes, and an annotated `v0.2.0` tag is created.
2. **Given** the working tree is dirty or the current branch is not `main`, **When** the maintainer runs the release script, **Then** the script aborts before changing versioned files or creating a tag.
3. **Given** one release validation step fails, **When** the script runs, **Then** it exits with a clear error and does not create or push a release tag.

---

### User Story 2 - Publish Automatically From A Stable Tag (Priority: P1)

As a maintainer, I want pushing a stable `vX.Y.Z` tag on `main` to automatically validate and publish all release artifacts so that releases are auditable, repeatable, and do not require manual registry publishing from a developer machine.

**Why this priority**: Automatic tag-triggered publishing is the core release automation outcome and the main improvement over the current crate-only workflow.

**Independent Test**: Can be fully tested by pushing a stable tag from a prepared release commit and verifying that CI reruns the release-grade validation matrix and then publishes the Rust crates, npm package, and PyPI package in the required order.

**Acceptance Scenarios**:

1. **Given** a release commit on `main` with aligned versions and tag `v0.2.0`, **When** the tag is pushed, **Then** GitHub Actions validates the release artifacts and publishes `relateby-pattern`, `relateby-gram`, `@relateby/pattern`, and the combined Python package automatically.
2. **Given** a stable tag whose validation fails, **When** the publish workflow runs, **Then** no registry publish step is executed and the failure is reported in workflow logs.
3. **Given** a tag that is not a stable semantic version or does not point to a commit on `main`, **When** the workflow is triggered, **Then** it refuses to publish any artifact.

---

### User Story 3 - Consume Single Combined npm And Python Artifacts (Priority: P2)

As a downstream user, I want npm and PyPI to each provide one combined package that exposes both pattern and gram functionality so that installation is simple and version alignment across language bindings is guaranteed.

**Why this priority**: Package consolidation reduces user confusion, removes cross-package version drift, and matches the desired long-term release model.

**Independent Test**: Can be fully tested by installing the packed npm tarball and the built Python wheel in clean environments and confirming that pattern and gram functionality are available from the single public artifact with the expected import paths.

**Acceptance Scenarios**:

1. **Given** a published npm release, **When** a user installs `@relateby/pattern`, **Then** they can access both the existing pattern/graph APIs and the gram codec APIs from that single package.
2. **Given** a published Python release, **When** a user installs the combined Python distribution, **Then** they can import both `relateby.pattern` and `relateby.gram` without needing a separate `relateby` distribution artifact.
3. **Given** documentation and examples for JS or Python, **When** a user follows them after the packaging change, **Then** the instructions reference only the supported public artifacts and still work.

---

### User Story 4 - Release Documentation Matches Automation (Priority: P2)

As a maintainer, I want release documentation and CI validation steps to match the real automated workflow so that maintainers can reason about a release without reverse-engineering the pipeline.

**Why this priority**: The current repo already has release docs, packaging docs, and CI scripts that no longer line up cleanly. Aligning them is necessary for maintainability.

**Independent Test**: Can be fully tested by following the documented release quickstart from a fresh checkout and confirming that the described commands, artifacts, and workflow behavior match reality.

**Acceptance Scenarios**:

1. **Given** the updated docs, **When** a maintainer follows the quickstart, **Then** the documented preparation steps, tag trigger, and published artifact list match the actual implementation.
2. **Given** a registry or packaging failure, **When** a maintainer consults the release documentation, **Then** they find the correct recovery path for the consolidated artifact model.

### Edge Cases

- A release tag matches `v*` but includes a prerelease suffix such as `-rc.1`; npm stable publishing must not run for that tag.
- The release script bumps one manifest but misses another publishable manifest; validation must catch version mismatch before tag creation or publish.
- The packed npm artifact builds successfully in the workspace but is missing generated files when installed from `npm pack`; release validation must test the packed tarball, not only workspace builds.
- The combined Python wheel builds locally but the wheel or sdist cannot be installed in a clean environment; release validation must smoke-test the built artifact.
- `relateby-pattern` publishes to crates.io successfully but a later publish step fails; the workflow and docs must describe recovery without re-publishing immutable crate versions.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The repository MUST provide a `scripts/new-release.sh` script that prepares releases only from `main`.
- **FR-002**: The release script MUST refuse to run when the working tree is dirty, the current branch is not `main`, or local `main` is not synchronized with `origin/main`.
- **FR-003**: The release script MUST update the shared release version in every release-managed Cargo, npm, and Python manifest before creating the release tag.
- **FR-004**: The release script MUST run a release-grade validation suite that covers Rust, WASM, the packed npm artifact, and the combined Python artifact before creating a release commit and tag.
- **FR-005**: The repository MUST publish exactly one public npm package, `@relateby/pattern`, containing both pattern and gram functionality.
- **FR-006**: The repository MUST publish exactly one combined Python distribution artifact; `relateby` MUST remain an import namespace only and MUST NOT be published as a distribution artifact.
- **FR-007**: The combined Python artifact MUST preserve the import paths `relateby.pattern` and `relateby.gram`.
- **FR-008**: Pushing a stable tag `v<major>.<minor>.<patch>` from a release commit on `main` MUST trigger a workflow that reruns release validation before any registry publish step.
- **FR-009**: The publish workflow MUST automatically publish the Rust crates in dependency order: `relateby-pattern` first, then `relateby-gram`.
- **FR-010**: The publish workflow MUST automatically publish the npm and Python artifacts after validation succeeds.
- **FR-011**: The publish workflow MUST fail without publishing when tag validation, version alignment, or release validation fails.
- **FR-012**: npm publishing MUST support stable releases only; prerelease tag publishing is out of scope.
- **FR-013**: Registry credentials MUST be provided through secure CI configuration and MUST NOT be stored in the repository.
- **FR-014**: Repository documentation and local validation scripts MUST be updated to describe and validate only the supported consolidated release artifacts.

### Key Entities *(include if feature involves data)*

- **Release Version**: A semantic version shared by all public release artifacts for one release.
- **Release Preparation Run**: A local execution of `scripts/new-release.sh` that validates releasability, syncs versions, and creates the release commit and tag.
- **Release Artifact**: A public package published for a release version: a Rust crate, the single npm package, or the single combined Python package.
- **Publish Workflow Run**: A tag-triggered CI execution that validates the release commit and publishes release artifacts.
- **Validation Matrix**: The set of required checks that must pass before a release tag is created or any artifact is published.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Running `scripts/new-release.sh <version>` on a clean checkout of `main` completes without manual file edits and produces one release commit plus one annotated release tag.
- **SC-002**: Pushing a valid stable tag triggers one automated workflow that either publishes all supported artifacts successfully or publishes none outside documented immutable-registry recovery cases.
- **SC-003**: npm consumers install exactly one public package, `@relateby/pattern`, to access both pattern and gram functionality.
- **SC-004**: Python consumers install exactly one published distribution artifact and can import both `relateby.pattern` and `relateby.gram`.
- **SC-005**: Release documentation and quickstart instructions allow a maintainer unfamiliar with the old workflow to prepare and trigger a release without tribal knowledge.

## Assumptions

- The combined Python distribution will be published as `relateby-pattern`, while Python imports remain under the `relateby` namespace.
- `@relateby/graph` and `@relateby/gram` will be removed from the set of published npm artifacts, even if some internal source structure remains.
- Existing Rust crate names (`relateby-pattern`, `relateby-gram`) remain unchanged.
- GitHub Actions is the authoritative remote publishing mechanism for stable releases.
