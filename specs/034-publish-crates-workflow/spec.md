# Feature Specification: Publish Rust Artifacts to Crates with Docs, Examples, and Tag-Based Release Workflow

**Feature Branch**: `034-publish-crates-workflow`  
**Created**: 2025-02-28  
**Status**: Draft  
**Input**: User description: "Publish rust artifacts to crates, along with docs, examples, and a github workflow that operates on tagged releases. Include instructions for publishing flow"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Publish Library Packages to Public Registry (Priority: P1)

As a maintainer, I want to publish the workspace library packages to the public package registry so that downstream users can depend on them in their own projects without using path or git dependencies.

**Why this priority**: Publishing is the core outcome; without it, docs and automation add no value.

**Independent Test**: Can be fully tested by performing a dry-run publish for each package and confirming the package metadata and contents are accepted; delivers installable packages.

**Acceptance Scenarios**:

1. **Given** the repository is in a releasable state, **When** a maintainer runs the publish flow for a package, **Then** the package is published to the public registry and becomes installable by version.
2. **Given** multiple packages with dependency order, **When** the publish flow is executed, **Then** packages are published in the correct order so that dependents resolve from the registry.
3. **Given** a package that fails validation (e.g., missing metadata or broken build), **When** publish is attempted, **Then** the publish fails with clear feedback and no broken version appears on the registry.

---

### User Story 2 - Published Documentation (Priority: P2)

As a user, I want to view up-to-date API documentation for the published packages so that I can discover and use the library without reading source code.

**Why this priority**: Documentation is the primary way users learn the API after deciding to use the package.

**Independent Test**: Can be tested by publishing a version and confirming that the documentation site shows the expected modules, types, and examples for that version.

**Acceptance Scenarios**:

1. **Given** a published package version, **When** a user opens the documentation for that version, **Then** they see structured API docs (modules, types, functions) consistent with the package contents.
2. **Given** the package includes doc comments and examples in code, **When** the documentation is built, **Then** those comments and runnable examples appear in the published docs.
3. **Given** a new release is published, **When** the documentation pipeline runs, **Then** the documentation for the new version is published and linked from the package page.

---

### User Story 3 - Examples Available to Users (Priority: P2)

As a user, I want to run or inspect examples that demonstrate how to use the library so that I can get started quickly and verify behavior.

**Why this priority**: Examples reduce time-to-first-success and support self-service adoption.

**Independent Test**: Can be tested by fetching the published package and running the documented examples; they complete successfully and demonstrate intended usage.

**Acceptance Scenarios**:

1. **Given** the package declares one or more examples, **When** a user installs the package and runs the documented example commands, **Then** the examples execute successfully.
2. **Given** examples are part of the published artifact or linked from the package page, **When** a user follows the instructions, **Then** they can run examples without cloning the repository.
3. **Given** the package metadata, **When** a user looks at the package listing, **Then** they can discover that examples exist and how to run them.

---

### User Story 4 - Tag-Triggered Release Workflow (Priority: P1)

As a maintainer, I want releases to be triggered by creating a version tag so that publishing is consistent, auditable, and does not rely on manual steps from a local machine.

**Why this priority**: Automation reduces human error and makes releases repeatable; tags provide a clear audit trail of what was released.

**Independent Test**: Can be tested by pushing a version tag in a test environment and verifying that the automated workflow runs and publishes the expected packages (or fails with clear errors).

**Acceptance Scenarios**:

1. **Given** a new version tag is pushed to the repository, **When** the tag is created, **Then** an automated workflow runs that builds, validates, and publishes the packages.
2. **Given** the workflow runs, **When** a step fails (e.g., tests or lint), **Then** the workflow stops and no package is published; the maintainer receives clear failure information.
3. **Given** the same tag is pushed again, **When** the workflow runs, **Then** the system handles the duplicate appropriately (e.g., idempotent skip or clear error) and does not leave the registry in an inconsistent state.

---

### User Story 5 - Publishing Instructions (Priority: P2)

As a maintainer (or someone handing off release duties), I want clear, step-by-step instructions for the publishing flow so that I can perform a release or onboard others without relying on tribal knowledge.

**Why this priority**: Instructions ensure the feature is maintainable and transferable.

**Independent Test**: Can be tested by having someone unfamiliar with the project follow the instructions and successfully perform a dry-run or real release.

**Acceptance Scenarios**:

1. **Given** a maintainer who has not published before, **When** they follow the documented publishing instructions, **Then** they can complete prerequisites (e.g., credentials, tooling) and run the publish flow.
2. **Given** the instructions, **When** a maintainer reads them, **Then** they understand the order of operations (e.g., tag format, which packages to publish first, how to trigger the workflow).
3. **Given** a failed publish or workflow run, **When** the maintainer consults the instructions, **Then** they find guidance on common failures and how to fix or retry.

---

### Edge Cases

- What happens when a tag is pushed that does not match the expected version format? The workflow should not publish; it should fail or skip with a clear message.
- How does the system handle a partial publish (e.g., first package succeeds, second fails)? Instructions and workflow should describe recovery (e.g., whether to re-tag, fix and re-run, or publish the remaining package manually) so the registry stays consistent.
- What happens when documentation build has warnings or non-fatal errors? The desired behavior (e.g., fail the doc step vs. publish anyway with warnings) should be documented so maintainers know what to expect.
- How are secrets (e.g., registry token) managed? Instructions must describe where to store them and that they are not committed to the repository.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST support publishing each publishable workspace package to the public package registry so that each can be installed by version by downstream users.
- **FR-002**: The system MUST ensure package metadata required by the registry (e.g., description, license, repository, documentation URL) is present and valid before publish.
- **FR-003**: The system MUST support publishing packages in dependency order so that dependents can resolve their dependencies from the registry after publish.
- **FR-004**: The system MUST produce and publish API documentation for each published package so that users can view docs for the released version.
- **FR-005**: The system MUST include or expose examples so that users can run or inspect them after installing the package.
- **FR-006**: An automated workflow MUST run when a version tag is pushed, performing build, validation, and publish steps without requiring manual publish from a local machine.
- **FR-007**: The workflow MUST fail clearly and not publish any package when validation (e.g., tests, lint, build) fails.
- **FR-008**: Maintainers MUST have access to written instructions that cover prerequisites, tag format, how to trigger the workflow, and how to handle common failures or retries.
- **FR-009**: Registry credentials MUST be configurable via secure mechanism (e.g., secrets) and MUST NOT be stored in the repository.

### Key Entities

- **Package**: A distributable unit of the workspace that can be published to the registry; has a name, version, and metadata; may depend on other packages in the same workspace.
- **Version tag**: A tag in the repository that denotes a release and triggers the automated publish workflow; format and semantics must be documented.
- **Published documentation**: The generated API docs for a package version, viewable by users from the package’s documentation URL.
- **Publishing instructions**: The documented steps and prerequisites for performing a release, including tag creation, workflow behavior, and troubleshooting.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A maintainer can publish all intended workspace packages to the public registry in a single release flow (manual or tag-triggered) in under 15 minutes under normal conditions.
- **SC-002**: After a release, users can install the package by version from the registry and run documented examples without cloning the repository.
- **SC-003**: API documentation for each published version is available at the documented URL and reflects the code for that version.
- **SC-004**: Pushing a version tag triggers the release workflow within one minute; the workflow either completes successfully and publishes or fails with identifiable cause without publishing.
- **SC-005**: A new maintainer can perform a dry-run or real release by following the written instructions alone, without prior experience with the project’s release process.

## Assumptions

- The public package registry and the chosen CI platform support the required operations (publish, secrets, tag triggers).
- **Published crate names use the relateby- prefix** (e.g. relateby-pattern, relateby-gram) on crates.io; source directories may remain pattern-core and gram-codec.
- Package versioning follows semantic versioning or the project’s stated scheme; version tags and package versions are aligned.
- Only the packages intended for public use are published; other workspace members (e.g., binaries, internal tools) are excluded from the publish flow.
- Documentation is generated from the same source as the published package (e.g., from the tag commit) so that docs and code stay in sync for a given version.
