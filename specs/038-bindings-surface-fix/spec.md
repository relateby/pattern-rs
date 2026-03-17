# Feature Specification: TypeScript and Python Surface Improvements

**Feature Branch**: `038-bindings-surface-fix`  
**Created**: 2026-03-17  
**Status**: Draft  
**Input**: User description: "improved surface for Typscript and Python, closing gaps, fixing mismatches and resulting in a nicer developer experience"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Consistent Public Workflows (Priority: P1)

A TypeScript or Python developer wants to complete common tasks using only the documented public packages and names, without discovering that examples, type information, and runtime behavior disagree.

**Why this priority**: This is the primary value of the feature. If the public surface is inconsistent, basic onboarding and everyday usage break down even when the underlying functionality exists.

**Independent Test**: Can be fully tested by following the documented quick-start workflows for TypeScript and Python using only public imports and verifying that the documented names, return values, and behavior all work as described.

**Acceptance Scenarios**:

1. **Given** a developer using the documented TypeScript package entry point, **When** they follow a published example for a common workflow, **Then** every referenced public symbol is importable, callable, and behaves as documented.
2. **Given** a developer using the documented Python package entry point, **When** they follow a published example for a common workflow, **Then** every referenced public symbol is importable, callable, and behaves as documented.
3. **Given** a workflow that is available in both TypeScript and Python, **When** a developer performs the same task in each language, **Then** the public behavior and expected outcomes are equivalent at the feature level.

---

### User Story 2 - Clear and Trustworthy Guidance (Priority: P2)

A developer wants the official docs, examples, and type information to match the real public API so they can learn the product quickly and avoid trial-and-error debugging.

**Why this priority**: Discoverability and trust are critical for adoption. Even small mismatches in names, signatures, or examples create disproportionate friction for new users.

**Independent Test**: Can be fully tested by reviewing the published docs, examples, and type surfaces for TypeScript and Python and verifying that they describe the same public workflows and outcomes as the actual package behavior.

**Acceptance Scenarios**:

1. **Given** an official example or guide, **When** a developer follows it without consulting internal modules, **Then** the example runs successfully or the documented outcome is achieved without requiring undocumented adjustments.
2. **Given** a public method, class, or helper, **When** a developer reads its documentation or type information, **Then** the documented inputs, outputs, and usage constraints accurately reflect real behavior.

---

### User Story 3 - Predictable Errors and Package Boundaries (Priority: P3)

A developer wants unsupported usage, missing setup, and invalid inputs to fail in a predictable way, with errors and package boundaries that make it obvious how to recover.

**Why this priority**: Once the primary workflows work, the next biggest source of friction is confusing failure behavior, especially when wrappers, generated bindings, and package-level surfaces diverge.

**Independent Test**: Can be fully tested by attempting documented invalid or incomplete flows in both languages and verifying that failures are surfaced through the documented public package boundary with clear, actionable outcomes.

**Acceptance Scenarios**:

1. **Given** invalid input to a documented public workflow, **When** the operation fails, **Then** the developer receives a consistent and clearly categorized failure through the public package surface.
2. **Given** a symbol or workflow that is not part of the supported public API, **When** a developer uses the documented package entry points, **Then** the package makes the supported path clear and does not require internal module knowledge to recover.

### Edge Cases

- What happens when a capability exists in the underlying bindings but is missing from the public package entry point in only one language?
- How does the system handle public examples that depend on helper names, chaining behavior, or return shapes that differ from runtime behavior?
- What happens when a workflow is partially available, such as being present in generated bindings or stubs but absent from the documented public import surface?
- How does the system handle invalid input or missing setup when wrapper layers are involved, so that developers see a consistent failure rather than wrapper-specific surprises?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST provide a documented public package surface for TypeScript that is internally consistent across imports, runtime behavior, type information, and examples.
- **FR-002**: The system MUST provide a documented public package surface for Python that is internally consistent across imports, runtime behavior, type information, and examples.
- **FR-003**: The system MUST ensure that every publicly documented TypeScript symbol for core developer workflows is available from the supported public package entry point.
- **FR-004**: The system MUST ensure that every publicly documented Python symbol for core developer workflows is available from the supported public package entry point.
- **FR-005**: The system MUST align the documented method names, argument expectations, return values, and chaining behavior with the actual public behavior in both languages.
- **FR-006**: The system MUST expose equivalent public workflows for shared capabilities across TypeScript and Python, including creation, parsing, graph-oriented operations, and developer guidance for common tasks.
- **FR-007**: The system MUST ensure that generated or handwritten type surfaces do not advertise public behavior that is unavailable or materially different at runtime.
- **FR-008**: The system MUST ensure that wrapper layers and package-level facades do not remove or distort documented public capabilities provided by the underlying bindings.
- **FR-009**: The system MUST provide developer-facing error behavior that is predictable, documented, and consistent with the public workflow being used.
- **FR-010**: The system MUST update official examples and guides so that they use only supported public imports and valid public usage patterns.
- **FR-011**: The system MUST define and preserve clear package boundaries so developers can distinguish supported public APIs from internal or generated implementation details.
- **FR-012**: The system MUST include verification coverage for the supported TypeScript and Python public workflows most likely to regress when wrapper, binding, or documentation surfaces change.

### Key Entities *(include if feature involves data)*

- **Public API Surface**: The set of supported classes, functions, constants, helpers, and error types that developers are expected to import and use from the TypeScript and Python packages.
- **Developer Workflow**: A user-facing task such as creating data structures, parsing notation, building graphs, querying results, or validating input through the public API.
- **Developer Guidance Asset**: Any official type definition, stub, example, quick-start, or usage guide that teaches or constrains how the public API should be used.
- **Public Failure Mode**: A documented error outcome exposed through the public package boundary when developers provide invalid input, incomplete setup, or unsupported usage.

## Assumptions

- The supported public entry points for TypeScript and Python will remain the primary integration paths for external developers.
- Existing core capabilities should remain available where they already exist; this feature is focused on surfacing, aligning, and clarifying them rather than removing them.
- Internal modules, generated artifacts, and native binding layers are not considered supported public APIs unless explicitly re-exposed and documented as such.
- Developer experience improvements should favor predictable, low-surprise behavior over preserving mismatched legacy naming or wrapper behavior.

## Dependencies

- Availability of the existing TypeScript and Python package surfaces that are already distributed to developers.
- Availability of current documentation, examples, and type surfaces so they can be reconciled with the intended public experience.
- Existing shared feature concepts across languages, so equivalent workflows can be described and validated consistently.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of officially documented TypeScript quick-start and common-task examples succeed using only supported public imports and names.
- **SC-002**: 100% of officially documented Python quick-start and common-task examples succeed using only supported public imports and names.
- **SC-003**: For 100% of shared core workflows covered by the feature, developers can complete the task in both TypeScript and Python using only documented public imports, names, and examples.
- **SC-004**: All publicly documented symbols covered by the feature have matching runtime availability, documented behavior, and type or stub information, with zero known mismatches remaining at release time.
- **SC-005**: Public workflow regressions identified in this feature area are detectable by automated verification before release.
