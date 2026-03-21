# Feature Specification: `pato skill`

**Feature Branch**: `043-pato-skill`  
**Created**: 2026-03-20  
**Status**: Draft  
**Input**: User description: "pato skill as described in @proposals/pato-skills-proposal.md"

## Clarifications

### Session 2026-03-20

- Q: Should project installs be automatically discoverable by Vercel skills tooling? → A: Yes. Project installs must use a Vercel-discoverable path, with `.agents/skills/` as the required project-level location.
- Q: Should the canonical package location be `crates/pato/skills/pato/` or `.agents/skills/pato/`? → A: `.agents/skills/pato/` is the single canonical source in the repository, and the crate bundles from it rather than maintaining a second authoritative copy.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Install the bundled skill locally (Priority: P1)

As a developer or agent user, I want to run `pato skill` and get a ready-to-use
`pato` skill installed into a well-known local location so I can use it without
manually copying files or assembling the package myself.

**Why this priority**: This is the core user value of the feature. Without local
installation, the bundled skill is not practically usable.

**Independent Test**: Can be fully tested by invoking `pato skill` with default
options in a clean environment and verifying that a complete `pato` skill appears in
the default project-level location and is ready for discovery by skills-compatible
tools.

**Acceptance Scenarios**:

1. **Given** a project that has no installed `pato` skill, **When** the user runs
   `pato skill` with no additional options, **Then** the system installs the bundled
   `pato` skill into the default project-level interoperable location in a path that
   is automatically discoverable by Vercel skills tooling.
2. **Given** a successful installation, **When** the command completes, **Then** the
   user is told where the skill was installed.

---

### User Story 2 - Choose install scope and target convention (Priority: P2)

As a developer managing multiple environments, I want to choose whether the skill is
installed for the current project or for my user account, and choose among supported
destination conventions that preserve the intended discovery behavior, so I can place
it where my tools will discover it.

**Why this priority**: Different users and agent products discover skills from
different well-known locations. Supporting explicit destination selection makes the
feature broadly useful.

**Independent Test**: Can be fully tested by running the command separately for each
supported install combination and verifying that the skill appears in the selected
destination only, with project installs always landing in a Vercel-discoverable path.

**Acceptance Scenarios**:

1. **Given** a user selects project scope and the interoperable target convention,
   **When** the command runs successfully, **Then** the skill is installed into the
   project-level interoperable skills directory.
2. **Given** a user selects user scope and the client-native target convention,
   **When** the command runs successfully, **Then** the skill is installed into the
   user-level client-native skills directory.
3. **Given** a user selects project scope, **When** installation completes, **Then**
   the command uses a Vercel-discoverable project location and does not install into a
   project-level location that Vercel skills tooling would skip.
4. **Given** the user selects one supported destination, **When** installation
   completes, **Then** no other supported destination is modified as part of that
   command.

---

### User Story 3 - Protect existing installs and support explicit replacement (Priority: P3)

As a user who may already have a `pato` skill installed, I want the command to avoid
accidentally overwriting an existing install unless I explicitly request replacement,
so that rerunning the command is predictable and safe.

**Why this priority**: Protecting existing skill directories prevents accidental loss
of local changes or confusion about which version of the skill is active.

**Independent Test**: Can be fully tested by installing once, attempting a second
install without replacement enabled, and confirming that the existing install remains
unchanged; then repeating with explicit replacement enabled and confirming the install
is refreshed.

**Acceptance Scenarios**:

1. **Given** the destination already contains a `pato` skill, **When** the user runs
   the install command without explicit replacement, **Then** the command fails with a
   clear message and leaves the existing install unchanged.
2. **Given** the destination already contains a `pato` skill, **When** the user runs
   the install command with explicit replacement, **Then** the destination is updated
   with the bundled skill and the user is informed of the result.

---

### Edge Cases

- What happens when the selected destination path does not yet exist?
- What happens when the selected destination cannot be created or written to?
- How does the system behave if the bundled skill package is incomplete or missing a
  required entry file?
- How does the system behave when the user provides an unsupported scope or target
  value?
- How does the system behave when a requested project-level destination would not be
  automatically discoverable by Vercel skills tooling?
- What happens when the user requests the install path without changing the default
  installation behavior?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST include one bundled canonical `pato` skill package that
  is suitable for local installation and publication as a reusable skill artifact.
- **FR-001a**: The canonical repository location of the bundled `pato` skill package
  MUST be `.agents/skills/pato/`.
- **FR-002**: The bundled canonical skill package MUST include a `SKILL.md` entry file
  with the required metadata for skill discovery.
- **FR-003**: The bundled canonical skill package MUST be structured so supporting
  documentation and examples can be installed alongside the main skill instructions.
- **FR-004**: Users MUST be able to install the bundled `pato` skill by invoking a
  built-in `pato skill` command.
- **FR-005**: The system MUST support installation at both project scope and user
  scope.
- **FR-006**: The system MUST support installation to the interoperable skills
  directory convention at both project scope and user scope.
- **FR-007**: When project scope is selected, the system MUST install the skill into a
  project-level path that is automatically discoverable by Vercel skills tooling.
- **FR-008**: The system MUST NOT install a project-scoped skill into a project-level
  destination that Vercel skills tooling would not auto-discover.
- **FR-009**: The system MUST support installation to a client-native skills directory
  convention for user-scope installs.
- **FR-010**: When no scope is specified, the system MUST install to the project-level
  destination by default.
- **FR-011**: When no target convention is specified, the system MUST install to the
  interoperable skills directory by default.
- **FR-012**: The system MUST place the installed skill in the well-known destination
  that corresponds to the selected scope and supported destination convention.
- **FR-013**: The system MUST create any missing directories needed for a successful
  installation within the selected destination path.
- **FR-014**: The installed skill content MUST remain functionally equivalent to the
  bundled canonical skill package across all supported destinations.
- **FR-015**: The system MUST refuse to overwrite an existing installed `pato` skill
  unless the user explicitly requests replacement.
- **FR-016**: When explicit replacement is requested, the system MUST replace the
  existing installed `pato` skill with the bundled canonical package.
- **FR-017**: After a successful installation, the system MUST report the resolved
  destination path to the user.
- **FR-018**: If installation cannot be completed, the system MUST return a clear
  failure result that explains why no usable install was produced.
- **FR-019**: The initial feature scope MUST remain local-only and MUST NOT require a
  remote registry, network fetch, publication workflow, or account setup in order to
  install the bundled skill.
- **FR-020**: The system MUST avoid requiring two separately maintained authoritative
  copies of the same skill package within the repository.

### Key Entities *(include if feature involves data)*

- **Canonical Skill Package**: The source-of-truth `pato` skill artifact stored at
  `.agents/skills/pato/`, including its required metadata, instructions, and any
  bundled support files.
- **Installation Request**: The user's chosen install action, including scope,
  destination convention, and whether replacement of an existing install is allowed.
- **Install Target**: A well-known skill directory determined by the selected scope and
  destination convention.
- **Installed Skill**: A locally materialized copy of the canonical skill package at a
  selected install target.

### Assumptions

- The initial published skill name is `pato`.
- The feature supports exactly one bundled `pato` skill package in its initial scope.
- The repository keeps exactly one authoritative checked-in copy of the skill package,
  located at `.agents/skills/pato/`.
- The initial scope includes the interoperable convention at both scopes and a
  client-native convention for user-scope installs.
- Project-scope installs prioritize automatic discovery by Vercel skills tooling and
  therefore use `.agents/skills/` as the required project-level location.
- The crate consumes the canonical repository skill package for bundling and
  installation rather than owning a second source-of-truth copy under `crates/pato/`.
- The initial scope is limited to local installation of the bundled package and does
  not include packaging, validation, submission, or publication commands.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In a clean project, a user can install the bundled `pato` skill to the
  default destination with a single command and no manual file-copy steps.
- **SC-002**: In acceptance testing, 100% of successful installs create a complete and
  discoverable `pato` skill directory in the selected destination.
- **SC-003**: In acceptance testing, every supported install combination results in
  installation to the correct destination, with all project-scope installs landing in
  a Vercel-discoverable path and without modifying unselected destinations.
- **SC-004**: In acceptance testing, 100% of attempts to install over an existing skill
  without explicit replacement leave the existing install unchanged.
- **SC-005**: After every successful installation, the destination path is shown to the
  user in the command result.
