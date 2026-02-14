# Feature Specification: Relateby PyPI Release

**Feature Branch**: `029-relateby-pypi-release`  
**Created**: 2025-02-14  
**Status**: Draft  
**Input**: User description: "prepare the python packages for release to pypi under a namespace called relateby"

## Clarifications

### Session 2025-02-14

- Q: Which packages are in scope and how are they exposed to users? → A: Two packages, published together under the relateby namespace: pattern-core available as `relateby.pattern`, gram-codec available as `relateby.gram`.
- Q: Two separate PyPI projects or one PyPI project? → A: One PyPI project (e.g. `relateby`); single pip install provides both `relateby.pattern` and `relateby.gram`. Packages are intimately related; downstream users should think of them as one cohesive library (like a single JS object with properties).
- Q: Backward compatibility for existing import names (e.g. pattern_core)? → A: Clean break. Only `relateby.pattern` and `relateby.gram` are supported; no legacy import aliases; existing code must change to the new names.
- Q: How is versioning handled for the unified package? → A: Single version for the unified package only. The PyPI project has one version; the two subpackages are not separately versioned for Python users.
- Q: Migration documentation requirement for the clean break? → A: No explicit migration-doc requirement. General docs and examples use the new names only; no mandatory migration note or release note for the break.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Publish Package to PyPI (Priority: P1)

As a maintainer, I can publish the project's unified Python package to the Python Package Index (PyPI) as a single project (e.g. `relateby`) so that one install delivers both `relateby.pattern` and `relateby.gram` to end users.

**Why this priority**: Publishing is the prerequisite for all downstream consumption; without it, the feature delivers no user value.

**Independent Test**: Can be fully tested by performing a single publish (or dry-run) and confirming the unified package appears on PyPI (or test index) and delivers both `relateby.pattern` and `relateby.gram` in one install.

**Acceptance Scenarios**:

1. **Given** the unified package (pattern-core + gram-codec) is built and metadata is valid, **When** a maintainer runs the release process, **Then** the single project is successfully uploaded to PyPI (e.g. as `relateby`).
2. **Given** a maintainer has appropriate credentials, **When** they follow the documented release steps, **Then** they can complete the single publish without undocumented manual steps.
3. **Given** a new release version, **When** the publish completes, **Then** the project is discoverable on PyPI and one install provides both `relateby.pattern` and `relateby.gram`.

---

### User Story 2 - Install Package via Relateby Namespace (Priority: P2)

As an end user, I can install the official Python package (e.g. `pip install relateby`) from PyPI so that I get both `relateby.pattern` and `relateby.gram` in one install, without relying on local or development builds.

**Why this priority**: Installability under the chosen namespace is the primary outcome users expect after a PyPI release.

**Independent Test**: Can be tested by installing the package(s) from PyPI using the relateby namespace on a clean environment and verifying the installed package works for basic use.

**Acceptance Scenarios**:

1. **Given** the unified package is published to PyPI (e.g. as `relateby`), **When** a user runs `pip install relateby`, **Then** both `relateby.pattern` and `relateby.gram` are installed without errors.
2. **Given** a user with a supported Python version, **When** they install the package from PyPI, **Then** they can import and use both subpackages as documented (`import relateby.pattern`, `import relateby.gram`).
3. **Given** the single PyPI project, **When** a user installs it, **Then** the two subpackages are consistent (same release version) and do not conflict.

---

### User Story 3 - Repeatable and Documented Release (Priority: P3)

As a maintainer, I have clear documentation and a repeatable process for releasing the unified `relateby` package to PyPI so that future releases are consistent and onboarding of new maintainers is straightforward.

**Why this priority**: Ensures long-term sustainability and reduces release errors.

**Independent Test**: Can be tested by a maintainer following the documentation to perform a release (or dry-run) and confirming all steps are documented and repeatable.

**Acceptance Scenarios**:

1. **Given** a maintainer new to the project, **When** they read the release documentation, **Then** they can understand how to publish the single `relateby` package to PyPI without guessing.
2. **Given** the same source and version, **When** the release process is run twice (e.g., dry-run then real), **Then** the resulting package metadata and behavior are consistent.
3. **Given** a failed publish (e.g., network or credential issue), **When** the maintainer retries, **Then** the process can be safely retried without leaving partial or conflicting state on PyPI.

---

### Edge Cases

- What happens when the PyPI project name (e.g. `relateby`) is already claimed? The feature must account for name reservation or an alternative name so that publish does not fail unexpectedly.
- How does the release process handle version conflicts (e.g., re-publishing the same version)? Behavior should be documented and either prevented or explicitly allowed with clear semantics.
- How are credentials and secrets for PyPI handled? The process must not require embedding secrets in code or docs; recommended approach (e.g., tokens, CI secrets) should be documented.
- Existing code using legacy import names (e.g. `import pattern_core`) will break; there are no compatibility aliases. General documentation and examples use only the new imports (`relateby.pattern`, `relateby.gram`); no explicit migration note is required.
- What happens when the package metadata (version, dependencies, import layout) is invalid for PyPI? The process should validate before upload and give clear, actionable errors.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST produce a single Python package artifact (one PyPI project, e.g. `relateby`) that is accepted by PyPI and that provides both `relateby.pattern` and `relateby.gram` in one install.
- **FR-002**: The package MUST be installable by end users via the standard package install mechanism (e.g. `pip install relateby`) and MUST expose only `relateby.pattern` and `relateby.gram` as public import entry points (no legacy aliases such as `pattern_core`).
- **FR-003**: Package metadata (name, single version for the unified package, dependencies, license, supported Python versions) MUST satisfy PyPI requirements so that upload and install succeed.
- **FR-004**: Maintainers MUST have documented steps to publish the single package to PyPI, including prerequisites (e.g., credentials, tooling).
- **FR-005**: The release process MUST be repeatable: the same inputs must produce consistent, publishable output so that releases can be automated or re-run safely.
- **FR-006**: The PyPI project name (e.g. `relateby`) and import paths (`relateby.pattern`, `relateby.gram`) MUST be clearly defined and MUST NOT conflict with existing PyPI names in a way that would block or confuse installs.

### Key Entities

- **Python package (unified)**: The single distributable published to PyPI (e.g. project name `relateby`). It has one version for the whole project; the two subpackages are not separately versioned for Python users. It contains two subpackages: pattern-core (importable as `relateby.pattern`) and gram-codec (importable as `relateby.gram`). One install delivers both; users think of them as one cohesive library (like a single JS object with properties). Legacy import names (e.g. `pattern_core`) are not supported; only `relateby.pattern` and `relateby.gram` are public.
- **Relateby namespace**: The top-level import namespace `relateby`; subpackages are `relateby.pattern` and `relateby.gram`. The PyPI project name (e.g. `relateby`) matches this naming.
- **Release process**: The set of steps and artifacts (documentation, scripts, or tooling) that maintainers use to build, validate, and upload the single package to PyPI.
- **Package metadata**: Information required by PyPI for the single project (name, single version, dependencies, license, supported Python versions); must be valid so that upload and install succeed. Version is defined once for the unified package, not per subpackage.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A maintainer can publish the single package to PyPI on the first attempt when following the documented process and having valid credentials.
- **SC-002**: An end user can run `pip install relateby` and within 5 minutes run a basic usage scenario (e.g. import relateby.pattern or relateby.gram and one documented operation) on a supported environment.
- **SC-003**: The published package is discoverable on PyPI (or designated test index) and displays correct metadata (name, version, summary); one install provides both subpackages.
- **SC-004**: Release documentation exists and allows a new maintainer to perform a publish (or a dry-run) without reverse-engineering the build or upload steps.
- **SC-005**: Re-running the release process for the same version does not leave users or tooling in an inconsistent state (e.g. duplicate uploads are disallowed by PyPI; behavior is documented).

## Assumptions

- The PyPI project name (e.g. `relateby`) is available or can be registered on PyPI without conflict.
- One unified PyPI project delivers both pattern-core (as `relateby.pattern`) and gram-codec (as `relateby.gram`); they are intimately related and presented as one cohesive library (like a single JS object with properties).
- Maintainers have or can obtain PyPI credentials (e.g., API token) with permission to publish under the target namespace.
- The existing package build and metadata (e.g., version, dependencies, supported Python versions) are sufficient as a base; changes are limited to what is needed for PyPI and namespace compliance.
