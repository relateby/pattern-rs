# Feature Specification: Unified Gram WASM Package

**Feature Branch**: `028-unified-gram-wasm`  
**Created**: 2026-01-31  
**Status**: Draft  
**Input**: User description: "unified wasm that combines pattern-core and gram-codec as described by unified-wasm-spec.md"

## Clarifications

### Session 2026-01-31

- **Terminology (user correction):** The project focuses on Pattern&lt;V&gt;. This feature unifies WASM for pattern and gram-codec. Gram notation is about Pattern&lt;Subject&gt;. Graph-like notation is syntactic sugar; graph lens support is planned. The spec has been reframed around patterns and Subject; all "graph" language was replaced with Pattern&lt;Subject&gt; / Pattern&lt;V&gt; / Subject as appropriate.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Single package for pattern and gram (Priority: P1)

A developer building a browser or Node application needs one dependency that provides both the Pattern data structure and the ability to serialize and parse gram notation. They do not want to wire together multiple packages or understand internal parser representations.

**Why this priority**: Without a single entry point, adoption is blocked; developers must assemble and maintain integration between pattern types and serialization themselves.

**Independent Test**: Can be fully tested by importing from one package and constructing a pattern, then serializing it to text and parsing it back; delivers a working round-trip with one dependency.

**Acceptance Scenarios**:

1. **Given** a project that depends on the unified package, **When** the developer imports the public types and the gram serialization interface from that single package, **Then** they can build patterns and convert them to/from gram notation without additional imports.
2. **Given** a Pattern&lt;Subject&gt; (pattern whose values are Subjects with identity, labels, properties), **When** the developer serializes it to text and parses that text back, **Then** the result is equivalent to the original for structure and data.

---

### User Story 2 - Serialize and parse feel like JSON (Priority: P2)

A developer familiar with `JSON.stringify` and `JSON.parse` expects to serialize patterns to a string and parse a string into patterns with similarly named, predictable operations. The mental model is "stringify turns my data into text; parse turns text into my data."

**Why this priority**: Familiarity reduces learning time and mistakes; the feature succeeds when the API feels natural for the target environment.

**Independent Test**: Can be tested by verifying that one method turns a pattern into a string and another turns a valid string into a pattern (or list of patterns), with behavior and naming consistent with common serialization APIs.

**Acceptance Scenarios**:

1. **Given** a valid in-memory Pattern&lt;Subject&gt;, **When** the developer calls the stringify operation, **Then** they receive a single string representation suitable for storage or transport.
2. **Given** a valid gram-notation string, **When** the developer calls the parse operation, **Then** they receive one or more patterns that match the notation, with the option to obtain only the first pattern when that is all they need.

---

### User Story 3 - Convert generic patterns to serializable form (Priority: P3)

A developer has a Pattern&lt;V&gt; where V is not Subject (e.g. numbers or strings). They need a supported way to turn that pattern into Pattern&lt;Subject&gt; so it can be serialized to gram notation, using sensible defaults for identity, labels, and properties.

**Why this priority**: Enables serialization of arbitrary pattern-shaped data without forcing manual mapping for every type; improves completeness of the offering.

**Independent Test**: Can be tested by building a pattern of primitive or custom values, applying the conversion with default or minimal options, then stringifying and parsing to confirm round-trip.

**Acceptance Scenarios**:

1. **Given** a Pattern&lt;V&gt; over primitives (e.g. strings, numbers, booleans), **When** the developer uses the conventional conversion with default options, **Then** each value is represented as a Subject with a generated identity, a type-appropriate label, and the value stored in a property.
2. **Given** the same pattern, **When** the developer supplies optional overrides (e.g. label name, property name for the value), **Then** the converted Pattern&lt;Subject&gt; reflects those choices and still serializes and parses correctly.

---

### Edge Cases

- What happens when the input string to parse is empty or only whitespace? The system should return an empty list (or a defined "no pattern" result for the single-pattern variant) rather than failing or returning undefined behavior.
- What happens when the input string is invalid gram notation? The system should report a clear error (e.g. parse failure with position or reason) and not expose internal parser structures.
- What happens when the developer tries to serialize a Pattern&lt;V&gt; where V is not Subject? The system should accept only Pattern&lt;Subject&gt; for serialization and require the developer to convert other patterns first (e.g. via conventional conversion), or document that stringify accepts only Pattern&lt;Subject&gt;.
- How does the system behave when the same identity appears in multiple places in a pattern? Behavior should be consistent and predictable (e.g. structure preserved); duplicate or conflicting definitions should be handled in a documented way.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST provide one consumable artifact (e.g. one package or one entry point) from which users obtain both Pattern types and the gram serialization and parsing operations.
- **FR-002**: The system MUST allow users to serialize a single in-memory Pattern&lt;Subject&gt; to a string representation in gram notation.
- **FR-003**: The system MUST allow users to parse a string in gram notation into one or more in-memory patterns, with a documented way to request only the first pattern when applicable.
- **FR-004**: The system MUST support round-trip: parsing the string produced by serializing a Pattern&lt;Subject&gt; must yield an equivalent pattern (same structure and Subject data).
- **FR-005**: The system MUST accept for serialization only Pattern&lt;Subject&gt;; patterns over other value types (Pattern&lt;V&gt;) must be converted to Pattern&lt;Subject&gt; before serialization.
- **FR-006**: The system MUST provide a conventional conversion from Pattern&lt;V&gt; to Pattern&lt;Subject&gt;, with sensible defaults for identity, labels, and value property, and optional overrides.
- **FR-007**: The system MUST NOT expose internal parser or AST types to the public API; users work only with Pattern and Subject types.
- **FR-008**: The system MUST report parse failures for invalid input with a clear error (e.g. message or code) and must not expose internal structures in error output.
- **FR-009**: The system MUST define behavior for empty or whitespace-only parse input (e.g. return empty list or defined "no pattern" result).

### Key Entities

- **Pattern&lt;V&gt;**: The core data structure: a nested structure of values of type V. Users build and transform patterns. Gram notation serializes Pattern&lt;Subject&gt; specifically.
- **Subject**: A value type with identity, labels, and properties; the value type that gram notation represents. Pattern&lt;Subject&gt; is the serializable form.
- **Value**: Typed property values (e.g. string, number, boolean) attached to a Subject; used when constructing or reading Subjects.
- **Gram (namespace)**: The set of operations for serializing Pattern&lt;Subject&gt; to gram-notation strings and parsing strings into patterns; the developer-facing name for "stringify" and "parse" (and related operations).

*Note: Gram notation has syntactic sugar for graph-like notation, and graph-lens support is planned for the project; the unified WASM feature is about Pattern and gram-codec unification.*

## Assumptions

- The target consumer environment is JavaScript/TypeScript (browser and Node); the deliverable is a single consumable artifact (e.g. one package) for that environment.
- Gram notation is the only serialization format in scope; other formats are out of scope.
- Existing pattern and gram-codec behavior is the source of truth; the unified package composes them and does not change their semantics.
- Conventional conversion defaults (e.g. label names, property names for primitives) are sufficient for most use cases; advanced customization is optional.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A developer can complete "install dependency, create a small Pattern&lt;Subject&gt;, serialize it, parse it back, and assert equivalence" in under 10 minutes using only the product documentation.
- **SC-002**: Serialize and parse operations are available from a single import; no additional setup or wiring is required to perform a full round-trip.
- **SC-003**: Round-trip equivalence holds for all supported Pattern&lt;Subject&gt;: parsing the output of stringify yields structurally and semantically equivalent pattern(s).
- **SC-004**: Users never need to reference or depend on internal parser or AST types to accomplish serialize/parse workflows.
- **SC-005**: Invalid parse input produces a clear, actionable error (e.g. message or code) without exposing internal representation; empty or whitespace input has defined, documented behavior.
