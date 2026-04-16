# Feature Specification: TypeScript Polish for Downstream Projects

**Feature Branch**: `046-ts-downstream-polish`  
**Created**: 2026-04-16  
**Status**: Draft  
**Input**: User description: "Typescript polish for downstream projects as described in conversation above"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Accurate WASM Type Declarations (Priority: P1)

A downstream TypeScript developer reads the source of `@relateby/pattern` to understand what the WASM layer provides. They encounter `wasm-types.d.ts` and use it to understand the WASM adapter's capabilities. Currently the file describes a large, outdated surface (NativePatternGraph, WasmGraphQuery, centrality functions, BFS/DFS, etc.) that does not match the actual adapter, which exposes only three methods on a `Gram` class.

**Why this priority**: A developer reading stale declarations will either be confused about what's actually available, attempt to use non-existent APIs, or distrust the package entirely. This is a mandatory fix before encouraging downstream adoption.

**Independent Test**: A developer can read `wasm-types.d.ts` and find that the declared surface exactly matches what the built WASM artifact exports — no more, no less.

**Acceptance Scenarios**:

1. **Given** `wasm-types.d.ts` has been updated, **When** a developer reads it, **Then** they see only the `Gram` class with `parseToJson`, `stringifyFromJson`, and `validate` methods — no legacy classes or free functions.
2. **Given** a TypeScript project importing `@relateby/pattern`, **When** it compiles, **Then** no type errors arise from the WASM module declarations.
3. **Given** the updated declarations, **When** the existing test suite runs, **Then** all tests pass without modification.

---

### User Story 2 - Meaningful Gram Package Tests (Priority: P2)

A developer evaluating `@relateby/gram` wants confidence that gram parse/stringify/validate operations behave correctly before adopting the package. Currently the package has a single smoke test (3 assertions) that only verifies the exports are defined.

**Why this priority**: Test coverage is a signal of maturity and reliability to downstream consumers. A near-empty test suite for a package's primary API surface undermines trust. Tests also lock the API surface against regressions.

**Independent Test**: Run the `@relateby/gram` test suite alone and see substantive coverage of parse, stringify, and validate operations across common gram notation patterns.

**Acceptance Scenarios**:

1. **Given** valid gram notation, **When** `Gram.parse` is called, **Then** it returns a structured representation of the parsed patterns.
2. **Given** a parsed pattern, **When** `Gram.stringify` is called, **Then** it produces valid gram notation that round-trips back to an equivalent structure.
3. **Given** syntactically invalid gram notation, **When** `Gram.parse` is called, **Then** it returns a `GramParseError` with a descriptive message.
4. **Given** a gram string, **When** `Gram.validate` is called, **Then** it returns an empty array for valid input and an array of error objects for invalid input.
5. **Given** the `init()` export, **When** called, **Then** it resolves without error and subsequent gram operations succeed.

---

### User Story 3 - Documented Annotation Serialization Limitation (Priority: P3)

A developer working with annotated patterns discovers that annotation content is not fully preserved during a parse-then-serialize round-trip. Without documentation, they spend time debugging what appears to be a data loss bug. The limitation — that annotation body content is stored as properties on the wrapping subject rather than in a dedicated field — should be clearly described.

**Why this priority**: This is a known limitation that will affect any project using annotations. Documenting it converts a confusing bug report into a known constraint. The serialization choice between `@@a:L @k(37) (x)` notation and `[a:L {k:37} | x]` notation is future work; for now, documenting the current behaviour is sufficient.

**Independent Test**: A developer reading `docs/gram-notation.md` finds a clear note in the Annotations section describing the current behavior, its impact on round-trips, and the planned future improvement.

**Acceptance Scenarios**:

1. **Given** the updated gram-notation.md, **When** a developer reads the Annotations section, **Then** they find a note explaining that annotation body content is currently stored as properties on the wrapping subject.
2. **Given** the documentation, **When** a developer reads it, **Then** they understand that a parse-then-serialize round-trip may not preserve the original annotation syntax.
3. **Given** the documentation, **When** a developer reads it, **Then** they see that configurable annotation serialization (unary pattern format vs. property-map format) is planned but not yet available.

---

### Edge Cases

- What if a downstream project already imports types from the legacy `wasm-types.d.ts` declarations? The updated file must not introduce new exports that would cause naming conflicts in the generated `.d.ts` files.
- What if the WASM `validate` method returns an empty array vs. an array with error objects — the test suite must cover both the valid and invalid cases explicitly.
- What if `init()` is called multiple times — the test should verify idempotent behavior.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The `wasm-types.d.ts` file MUST declare only the types and methods that the actual WASM adapter exposes: a `Gram` class with `parseToJson(gram: string): string`, `stringifyFromJson(json: string): string`, and `validate(gram: string): Array<unknown>` methods, plus a default `init(): Promise<void>` export.
- **FR-002**: The `wasm-types.d.ts` file MUST NOT declare any legacy types (`NativePatternGraph`, `WasmGraphQuery`, `NativeReconciliationPolicy`, `WasmPattern`, `WasmSubject`, free traversal functions, centrality functions, etc.) that are not exposed by the current WASM adapter.
- **FR-003**: Both module declarations in `wasm-types.d.ts` (`../wasm/pattern_wasm.js` and `../wasm-node/pattern_wasm.js`) MUST be updated consistently.
- **FR-004**: The `@relateby/gram` package MUST have tests covering: successful parse of nodes, relationships, and property-bearing patterns; stringify of a parsed result; validation returning no errors for valid input; validation returning error objects for invalid input; and idempotent `init()` behavior.
- **FR-005**: The `docs/gram-notation.md` Annotations section MUST include a note describing the current limitation: annotation body content is stored as properties on the wrapping subject, not in a dedicated annotation field.
- **FR-006**: The documentation note MUST mention that configurable annotation serialization format (inline unary notation vs. property-map notation) is planned as future work.
- **FR-007**: All existing tests MUST continue to pass after these changes.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A developer reading `wasm-types.d.ts` finds zero references to types or methods not present in the actual WASM adapter output.
- **SC-002**: The `@relateby/gram` test suite contains at minimum 5 distinct behavioral test cases (up from the current 3 surface-existence assertions).
- **SC-003**: All three npm package test suites pass without modification after the changes.
- **SC-004**: The gram-notation.md Annotations section contains a clearly marked limitation note visible within the section (not buried in an appendix).
- **SC-005**: A downstream developer evaluating the package can determine the actual WASM boundary from source alone, without needing to read the compiled WASM artifact.

## Assumptions

- The two module declarations in `wasm-types.d.ts` (for `../wasm/pattern_wasm.js` and `../wasm-node/pattern_wasm.js`) expose the same `Gram`-only surface; the wasm-node variant is the Node.js-compatible build of the same adapter.
- The gram-notation.md documentation update is prose-only; no code changes to the Rust or TypeScript implementation are required for this story.
- Future annotation serialization format configuration (the `@@a:L @k(37) (x)` vs. `[a:L {k:37} | x]` choice) is out of scope for this feature and will be tracked separately.
- No new npm dependencies are needed for the additional tests; the existing vitest + `@relateby/pattern` setup is sufficient.
