# Feature Specification: pato help and self-documentation

**Feature Branch**: `045-pato-help`  
**Created**: 2026-03-22  
**Status**: Draft  
**Input**: User description: "superb pato help as described in @proposals/pato-help-proposal.md"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Fast help discovery (Priority: P1)

As a user or agent, I want `pato` to provide concise top-level help so I can quickly discover the available commands and choose the right next action.

**Why this priority**: This is the entry point for every help flow and must stay compact and reliable.

**Independent Test**: Run the top-level help surface and confirm it shows a short command list and basic usage guidance without overwhelming detail.

**Acceptance Scenarios**:

1. **Given** `pato` is installed, **When** a user runs `pato -h`, **Then** the output is short, scan-friendly, and lists the main commands.
2. **Given** `pato` is installed, **When** a user runs `pato --help` or `pato <command> --help`, **Then** the output provides normal command usage, arguments, flags, and examples.

---

### User Story 2 - Topic-based guidance (Priority: P1)

As a user or agent, I want to ask `pato` for help on a specific topic so I can get focused guidance on gram concepts and workflows.

**Why this priority**: Topic help is the main value of the new documentation surface and should be directly usable from prompts.

**Independent Test**: Run `pato help <topic>` for a known topic and confirm the topic content is shown clearly and consistently.

**Acceptance Scenarios**:

1. **Given** a known topic exists, **When** a user runs `pato help gram`, **Then** the command prints the matching topic content in a readable terminal form.
2. **Given** the topic content contains definitions, caveats, and examples, **When** the user views the help output, **Then** those sections are preserved and easy to copy into another prompt or terminal session.
3. **Given** a topic name maps to a file name, **When** the user requests that topic, **Then** the same topic name always resolves to the same document.

---

### User Story 3 - Packaged and installed reference docs (Priority: P2)

As a maintainer, I want the reference help corpus to ship with `pato` and be installed with the skill tree so the CLI help and the installed docs stay aligned.

**Why this priority**: The help surface is only trustworthy if the installed skill content matches the shipped documentation corpus.

**Independent Test**: Install the skill tree and verify that the reference topic files are present and match the packaged corpus.

**Version alignment note**: The installed skill tree is authoritative for the `pato` version that installed it. `pato help <topic>` always reads from the binary's embedded content (FR-007), so help output is never stale. However, agents and tools that read the installed skill tree directly will see the version installed by the last `pato skill` run. After upgrading the `pato` binary, users should run `pato skill --force` to bring the installed tree into alignment with the new binary.

**Acceptance Scenarios**:

1. **Given** a release includes topic reference documents, **When** `pato skill` installs the skill tree, **Then** the installed tree includes the same topic files.
2. **Given** a topic exists in the canonical corpus, **When** the package is built and installed, **Then** the topic file path and topic name remain aligned.
3. **Given** the installed skill tree is inspected, **When** a topic file is compared to the shipped corpus, **Then** the content matches and there is no separate drifted copy.

### Edge Cases

- A user asks for a topic that does not exist.
- A topic file is missing from the installed skill tree.
- A topic name uses the wrong spelling, casing, or separator.
- The canonical reference corpus contains a topic that is not exposed by the installed skill package.
- A topic contains long examples or code blocks that must remain readable in terminal output.

## Assumptions

- Topic names are exact markdown basenames and may include hyphens or underscores.
- The topic corpus is intentionally small at first and can grow over time without changing the topic contract.
- Topic help is written for terminal use and prompt reuse, not as a full manual.
- The installed skill tree is derived from the canonical repository content rather than maintained separately.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `pato -h` MUST present a concise command-discovery view that favors quick scanning over detailed reference text.
- **FR-002**: `pato --help` and `pato <command> --help` MUST provide standard command usage, arguments, options, and examples.
- **FR-003**: `pato help <topic>` MUST resolve the topic name to a single canonical markdown topic document embedded in the binary.
- **FR-004**: `pato help <topic>` MUST display the topic content in a readable terminal format that preserves the meaning of headings, lists, code blocks, and examples. Raw markdown output is acceptable for v1.
- **FR-005**: `pato help` with no topic or an unknown topic MUST fail clearly and MUST enumerate the valid topic names so the user can choose the correct one.
- **FR-006**: Each supported topic MUST have a stable public name that maps exactly to one markdown file name, with the published name matching the file basename exactly.
- **FR-007**: The topic corpus MUST be embedded in the `pato` binary and is the single source of truth. `pato help <topic>` reads exclusively from this embedded corpus, not from the installed skill tree.
- **FR-008**: `pato skill` MUST install the skill tree (including reference topic files) by extracting content embedded in the binary. The installed files MUST match the binary's embedded content exactly. The binary MUST be self-sufficient: `pato skill` MUST work correctly after `cargo install relateby-pato` with no source tree present on disk.
- **FR-010**: If a skill tree is already present at the install target (e.g., installed by a separate skills management system), `pato skill` MUST fail clearly with an actionable error message and MUST tell the user to re-run with `--force` to replace it. `pato skill --force` MUST always succeed and bring the installed tree into exact alignment with the binary's embedded content, regardless of what was previously installed.
- **FR-009**: The help corpus MUST avoid duplicated copies of the same topic content that can drift independently.

### Key Entities *(include if feature involves data)*

- **Topic Document**: A single markdown help topic identified by a stable topic name, embedded in the `pato` binary, intended for both human reading and prompt reuse.
- **Topic Catalog**: The complete set of topic names available from the embedded corpus, exposed by `pato help` with no arguments.
- **Skill Tree**: The installed documentation bundle produced by `pato skill`. Its reference topic files are exported from the embedded corpus and are derived artifacts — authoritative only insofar as they match the binary that installed them.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In usability checks, at least 90% of users can identify the correct help command to use from the top-level help output within 15 seconds.
- **SC-002**: In test coverage, 100% of supported topics resolve to the intended topic document on the first attempt.
- **SC-003**: In installation verification, 100% of released topic files are present in the installed skill tree and match the canonical corpus.
- **SC-004**: In error-path testing, every unknown topic request produces a clear failure message that points users toward valid topic discovery.
- **SC-005**: In reviewer feedback, topic help is consistently described as short enough for terminal use and specific enough to reuse in prompts.
