# Feature Specification: End-user documentation

**Feature Branch**: `022-end-user-docs`  
**Created**: 2026-01-19  
**Status**: Draft  
**Input**: User description: "End-user documentation in docs/ that covers pattern concepts, gram notation and basic usage in rust. The documentation can take inspiration from ../pattern-hs/docs/"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Understand Pattern Concepts (Priority: P1)

As a developer new to the project, I want to understand what a "Pattern" is and why it's useful, so that I can decide if this library meets my needs.

**Why this priority**: Core value proposition. Users must understand the conceptual model before they can use the tool effectively.

**Independent Test**: A user unfamiliar with the project can read the introduction and explain the "decorated sequence" concept and the difference between explicit patterns and implicit traversals.

**Acceptance Scenarios**:

1. **Given** no prior knowledge of Gram, **When** I read `docs/introduction.md`, **Then** I understand that a Pattern is a decorated sequence where elements form the concept and the value provides decoration.
2. **Given** knowledge of knowledge graphs, **When** I read the introduction, **Then** I understand how Patterns make implicit traversals explicit.

---

### User Story 2 - Learn Gram Notation (Priority: P1)

As a data engineer or architect, I want to learn how to represent my data using Gram notation, so that I can create and share patterns easily.

**Why this priority**: Gram notation is the primary way patterns are expressed and shared.

**Independent Test**: A user can write gram notation for a simple node, a relationship between two nodes, and an annotated pattern, and these match the reference implementation's expectations.

**Acceptance Scenarios**:

1. **Given** a need to represent a "Person" node, **When** I check `docs/gram-notation.md`, **Then** I find that `(n:Person)` or `[n:Person]` are valid representations.
2. **Given** a need to represent a relationship, **When** I check the notation reference, **Then** I find how to represent directed and undirected relationships.

---

### User Story 3 - Use pattern-rs in Rust (Priority: P2)

As a Rust developer, I want to know how to include the library in my project and perform basic operations, so that I can start building applications.

**Why this priority**: Essential for library adoption.

**Independent Test**: A developer can copy a code example from `docs/rust-usage.md` into a Rust project and it performs the expected operation (e.g., creating a pattern).

**Acceptance Scenarios**:

1. **Given** a new Rust project, **When** I read `docs/rust-usage.md`, **Then** I know which crates to include and how to use `Pattern::point` and `Pattern::pattern`.
2. **Given** a constructed Pattern, **When** I check the usage guide, **Then** I find how to access its value and elements.

---

### Edge Cases

- **What happens when a Gram notation example is complex or nested?** The notation reference should show how nesting works and how to read it.
- **How does the system handle features not yet implemented in Rust?** The documentation should clearly mark features as "coming soon" or provide the equivalent Rust construction if a high-level one is missing (mirroring the `gram-hs` approach).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Documentation MUST be provided in Markdown format in the `docs/` directory.
- **FR-002**: `docs/introduction.md` MUST cover the "decorated sequence" concept and the "explicit vs implicit" distinction, inspired by the `gram-hs` guide.
- **FR-003**: `docs/gram-notation.md` MUST provide a reference for nodes, annotations, relationships, and paths, mapping them to the underlying Pattern structure.
- **FR-004**: `docs/rust-usage.md` MUST demonstrate basic usage of the `pattern-rs` crates (specifically `pattern-core` and `gram-codec`).
- **FR-005**: The root `README.md` SHOULD be updated to link to the new documentation sections.
- **FR-006**: Documentation MUST use consistent terminology (e.g., "Pattern", "Value", "Elements", "Atomic Pattern").

### Key Entities *(include if feature involves data)*

- **Pattern**: The core data structure, consisting of a Value and a sequence of Elements (which are themselves Patterns).
- **Gram Notation**: The textual representation format for Patterns.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Documentation consists of at least three new files: `introduction.md`, `gram-notation.md`, and `rust-usage.md`.
- **SC-002**: All examples in `rust-usage.md` are compatible with the current implementation in `crates/pattern-core` and `crates/gram-codec`.
- **SC-003**: The documentation covers all major concepts present in the `gram-hs` introductory guide.
- **SC-004**: A reader can successfully construct a "relationship" pattern in Rust after reading the documentation.
