# Feature Specification: Basic Gram Codec

**Feature Branch**: `019-gram-codec`  
**Created**: 2026-01-06  
**Status**: Draft  
**Input**: User description: "Basic gram Codec as described in 019 of TODO.md and using `../tree-sitter-gram` as the standard grammar reference and the `gram-lint` CLI tool for validating _all_ gram snippets. Unlike most other features, `../pattern-hs` is not authoritative but `../tree-sitter-gram` is. Also, when reviewing a parser library, remember that this will be used in WASM and Python as well, so make sure the choice of parser library supports that."

## Clarifications

### Session 2026-01-06

- Q: How should the tree-sitter-gram test corpus (`../tree-sitter-gram/test/corpus/`) be incorporated into the project for testing? → A: To be determined during planning (options: copy files, git submodule, symbolic links, or programmatic access)
- Q: When the parser encounters invalid gram notation, how should it handle the error? → A: Error recovery - attempt to recover from errors and report all syntax errors found in the input
- Q: What should the parser return for empty/whitespace-only input? → A: Empty collection - return success with an empty collection/list of patterns (no patterns found, but valid input)
- Q: For 2-element patterns, when should the serializer use relationship notation `(a)-->(b)` versus subject pattern notation `[root | e1, e2]`? → A: Use relationship notation when both elements are atomic nodes (0 elements); otherwise use subject pattern notation
- Q: How should heterogeneous property values (strings, numbers, booleans, arrays, ranges) be represented in the Subject's record? → A: Value enum - define a `Value` enum with variants for each type and store properties as `HashMap<String, Value>` (may require refactoring Subject to avoid redundant value implementation)
- Q: Where and how should arrow type information (→, ←, ↔, ~~) be stored in the Pattern structure? → A: Infer from element ordering and context - arrow types are syntactic sugar, ordering matters (needs further clarification for handling each variation during parsing/serialization)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Parse Gram Notation to Pattern (Priority: P1)

As a developer using the Pattern library, I need to parse gram notation text into Pattern data structures, so that I can load patterns from gram files, accept gram notation as input, and enable users to work with the familiar gram syntax for creating patterns.

**Why this priority**: Parsing gram notation is the entry point for all gram-based workflows. Without this capability, users cannot load patterns from gram files, cannot accept gram notation as input, and cannot leverage the gram syntax that makes patterns human-readable and writable. This is the most fundamental codec operation and must work correctly before serialization can be implemented.

**Independent Test**: Can be fully tested by providing valid gram notation strings and verifying that they parse into correct Pattern structures. Testing includes all gram syntax forms (nodes, relationships, subject patterns, annotations), nested structures, properties, labels, identifiers, comments, and edge cases. The `gram-lint` CLI tool must validate all test gram snippets. Delivers immediate value by enabling gram notation as the input format for patterns.

**Acceptance Scenarios**:

1. **Given** gram notation for a simple node `(hello)`, **When** parsing the notation, **Then** a Pattern with identifier "hello" and 0 elements is created
2. **Given** gram notation for a relationship `(a)-->(b)`, **When** parsing the notation, **Then** a Pattern with 2 elements (nodes a and b) is created
3. **Given** gram notation for a subject pattern `[team | alice, bob]`, **When** parsing the notation, **Then** a Pattern with identifier "team" and elements for alice and bob is created
4. **Given** gram notation with labels `(a:Person)`, **When** parsing the notation, **Then** the Pattern includes the Person label in its value
5. **Given** gram notation with properties `(a {name: "Alice"})`, **When** parsing the notation, **Then** the Pattern includes the properties record in its value
6. **Given** gram notation with comments `// comment\n(hello)`, **When** parsing the notation, **Then** comments are ignored and the pattern is parsed correctly
7. **Given** gram notation with annotations `@key(value) (node)`, **When** parsing the notation, **Then** an annotated Pattern with 1 element is created
8. **Given** gram notation with nested structures `[outer | [inner | leaf]]`, **When** parsing the notation, **Then** a nested Pattern structure with multiple levels is created
9. **Given** invalid gram notation, **When** attempting to parse, **Then** a descriptive error message is returned indicating the syntax error location and nature
10. **Given** empty gram notation or whitespace only, **When** parsing, **Then** success is returned with an empty collection of patterns (valid input with no patterns)

---

### User Story 2 - Serialize Pattern to Gram Notation (Priority: P2)

As a developer using the Pattern library, I need to serialize Pattern data structures into gram notation text, so that I can save patterns to gram files, output patterns in human-readable form, and enable interoperability with other gram tools.

**Why this priority**: Serialization enables patterns to be saved, shared, and viewed in gram notation format. This is essential for persistence, debugging, human inspection, and interoperability with other tools in the gram ecosystem. While less critical than parsing (you need to read patterns before you can write them), serialization is necessary for any workflow that produces or modifies patterns.

**Independent Test**: Can be fully tested by providing Pattern structures and verifying that they serialize into valid gram notation that, when re-parsed, produces equivalent patterns (round-trip testing). Testing includes all pattern types (atomic, nested, with labels, with properties), proper escaping, and formatting. All serialized output must pass `gram-lint` validation. Delivers immediate value by enabling gram notation as the output format for patterns.

**Acceptance Scenarios**:

1. **Given** a Pattern with identifier "hello" and 0 elements, **When** serializing to gram notation, **Then** the output is `(hello)` or equivalent valid gram notation
2. **Given** a Pattern representing a relationship with 2 elements, **When** serializing to gram notation, **Then** the output is valid relationship notation like `(a)-->(b)`
3. **Given** a Pattern with multiple elements, **When** serializing to gram notation, **Then** the output uses subject pattern notation like `[root | element1, element2]`
4. **Given** a Pattern with labels in its value, **When** serializing to gram notation, **Then** the output includes label syntax like `(a:Label)`
5. **Given** a Pattern with properties in its value, **When** serializing to gram notation, **Then** the output includes property record syntax like `(a {key: "value"})`
6. **Given** a Pattern with special characters in identifiers or property values, **When** serializing to gram notation, **Then** appropriate escaping or quoting is used to produce valid gram notation
7. **Given** a deeply nested Pattern structure, **When** serializing to gram notation, **Then** the output correctly represents the nesting with proper bracket matching
8. **Given** an annotated Pattern with 1 element, **When** serializing to gram notation, **Then** the output uses annotation syntax like `@key(value) (element)`
9. **Given** a Pattern that was parsed from gram notation, **When** serializing and re-parsing (round-trip), **Then** the resulting Pattern is structurally equivalent to the original
10. **Given** any serialized gram notation output, **When** validating with `gram-lint`, **Then** the output passes validation with no errors

---

### User Story 3 - Handle All Gram Syntax Forms (Priority: P3)

As a developer using the Pattern library, I need the codec to support all gram syntax forms defined in the tree-sitter-gram grammar, so that I can work with the full expressiveness of gram notation including identifiers, labels, properties, arrays, ranges, relationships, annotations, and comments.

**Why this priority**: Complete grammar support ensures the codec can handle any valid gram notation, not just a subset. This includes advanced features like property arrays, numeric ranges, boolean values, tagged strings, bidirectional relationships, and various identifier formats. While basic patterns are covered by P1 and P2, full syntax support is needed for real-world use cases.

**Independent Test**: Can be fully tested by providing gram notation using each syntax form and verifying correct parsing and serialization. Testing includes all value types (strings, numbers, booleans, symbols, arrays, ranges), all relationship types (left arrow, right arrow, bidirectional, squiggle), all identifier formats, and all subject components (identifier, labels, record, combinations). All test cases must be validated with `gram-lint`. Delivers value by ensuring the codec works with any valid gram notation from the tree-sitter-gram grammar.

**Acceptance Scenarios**:

1. **Given** gram notation with numeric values `(a {count: 42, ratio: 3.14})`, **When** parsing, **Then** numeric properties are correctly represented in the Pattern value
2. **Given** gram notation with boolean values `(a {active: true})`, **When** parsing, **Then** boolean properties are correctly represented in the Pattern value
3. **Given** gram notation with array properties `(a {tags: ["rust", "wasm", "python"]})`, **When** parsing, **Then** array properties are correctly represented in the Pattern value
4. **Given** gram notation with range values `(a {score: 1..10})`, **When** parsing, **Then** range values are correctly represented in the Pattern value
5. **Given** gram notation with tagged strings `(a {doc: """markdown content"""})`, **When** parsing, **Then** tagged strings are correctly represented in the Pattern value
6. **Given** gram notation with left arrow relationships `(a)<--(b)`, **When** parsing, **Then** a relationship Pattern with correct directionality is created
7. **Given** gram notation with bidirectional relationships `(a)<-->(b)`, **When** parsing, **Then** a relationship Pattern representing bidirectional connection is created
8. **Given** gram notation with squiggle relationships `(a)~>(b)` or `(a)~~(b)`, **When** parsing, **Then** a relationship Pattern with squiggle arrow type is created
9. **Given** gram notation with multiple labels `(a:Label1:Label2)`, **When** parsing, **Then** the Pattern includes both labels in its value
10. **Given** gram notation with integer identifiers `(42)` or string literal identifiers `("node-id")`, **When** parsing, **Then** the Pattern uses the identifier as specified
11. **Given** gram notation with relationship labels and properties `(a)-[:KNOWS {since: 2020}]->(b)`, **When** parsing, **Then** the relationship Pattern includes the label and properties in its value
12. **Given** gram notation with a root record `{graph: "social"} (a)-->(b)`, **When** parsing, **Then** the root Pattern includes the record as metadata

---

### Edge Cases

- What happens when parsing gram notation with syntax errors (missing brackets, unclosed strings, invalid tokens)?
- What happens when parsing empty input, whitespace-only input, or input with only comments? (Answer: returns success with empty collection - valid but contains no patterns)
- What happens when parsing gram notation with deeply nested structures (100+ levels)?
- What happens when serializing a Pattern that cannot be represented in gram notation (e.g., arbitrary value types not in gram grammar)?
- What happens when round-trip testing (parse then serialize then parse) with complex nested patterns?
- What happens when gram notation contains Unicode characters, emoji, or special characters in identifiers and properties?
- What happens when serializing patterns with very long property values or large arrays?
- What happens when parsing gram notation with all forms of whitespace (spaces, tabs, newlines, mixed)?
- What happens when parsing gram notation with comments at various positions (beginning, middle, end, between elements)?
- What happens when serializing patterns and the output must be formatted for human readability versus compact machine output?
- What happens when parsing gram notation that uses pattern references (identifiers that reference previously defined patterns)?
- What happens when validating serialized output with `gram-lint` for edge cases and boundary conditions?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a function that parses gram notation text into Pattern data structures following the tree-sitter-gram grammar specification
- **FR-002**: System MUST provide a function that serializes Pattern data structures into valid gram notation text that conforms to the tree-sitter-gram grammar
- **FR-003**: Parser MUST support all gram syntax forms defined in tree-sitter-gram grammar: node patterns, relationship patterns, subject patterns, annotated patterns
- **FR-004**: Parser MUST support all subject components: identifiers (symbols, strings, integers), labels (single and multiple), property records
- **FR-005**: Parser MUST support all value types: strings (quoted and unquoted symbols), numbers (integers and decimals), booleans, arrays, ranges, tagged strings
- **FR-006**: Parser MUST support all relationship arrow types: right arrow `-->`, left arrow `<--`, bidirectional `<-->`, squiggle `~~` and `~>`. Arrow types are syntactic sugar for relationship direction; element ordering in the Pattern structure captures the semantic relationship (planning phase must specify exact element ordering rules for each arrow type)
- **FR-007**: Parser MUST support comments using `//` syntax and ignore them during parsing (preserving only the semantic content)
- **FR-008**: Parser MUST support annotations with `@key(value)` syntax on subject patterns and path patterns
- **FR-009**: Parser MUST support nested subject patterns at arbitrary depths
- **FR-010**: Parser MUST support root records at the beginning of gram_pattern (optional metadata for the pattern collection)
- **FR-011**: Parser MUST provide descriptive error messages indicating syntax error location (line, column) and nature (expected vs found), and SHOULD attempt error recovery to report multiple syntax errors in a single parse attempt
- **FR-012**: Parser MUST handle all forms of whitespace (spaces, tabs, newlines) between tokens
- **FR-013**: Serializer MUST produce valid gram notation that passes `gram-lint` validation
- **FR-014**: Serializer MUST properly escape or quote identifiers and property values that contain special characters or whitespace
- **FR-015**: Serializer MUST choose appropriate gram syntax forms based on Pattern structure: node notation `()` for 0 elements, relationship notation `()-->()` for 2 elements when both are atomic (0 elements each), subject pattern notation `[ | ]` for all other cases (2 elements with non-atomic children, or N elements where N ≠ 0, 2)
- **FR-016**: Serializer MUST correctly serialize all Pattern value types that have gram notation equivalents (Subject with identifier, labels, properties)
- **FR-017**: Serializer MUST correctly serialize nested Pattern structures with proper bracket matching and syntax
- **FR-018**: System MUST support round-trip correctness: parsing gram notation, serializing the result, and re-parsing must produce a structurally equivalent Pattern
- **FR-019**: System MUST validate all gram notation examples and test cases using the `gram-lint` CLI tool
- **FR-020**: Parser library chosen MUST support compilation to WASM target for browser and Node.js usage
- **FR-021**: Parser library chosen MUST support Python bindings or have a clear path to Python integration
- **FR-022**: Parser MUST handle Unicode characters, emoji, and international characters in identifiers and property values
- **FR-023**: System MUST provide clear documentation mapping between gram notation syntax and Pattern data structure representation
- **FR-024**: System MUST represent property values using a `Value` enum with variants for all gram notation value types: String, Integer, Decimal, Boolean, Array (of scalar values), Range (lower and upper bounds), TaggedString (format tag and content)

### Key Entities

- **Gram Notation**: Human-readable text format for patterns using the syntax defined by tree-sitter-gram grammar. Includes node notation `()`, relationship notation `()-->()`, subject pattern notation `[ | ]`, annotations `@key(value)`, comments `//`, and various value types.
- **Pattern**: The core data structure being serialized/deserialized, consisting of a value (Subject) and elements (nested patterns).
- **Subject**: The value type for gram patterns, containing optional identifier, labels, and/or property record. Properties are stored as `HashMap<String, Value>` where `Value` is an enum supporting all gram notation value types (strings, integers, decimals, booleans, arrays, ranges, tagged strings). May require refactoring of existing Subject implementation to avoid redundant value representation.
- **Parser**: Component that transforms gram notation text into Pattern data structures, handling lexing, parsing, and error reporting.
- **Serializer**: Component that transforms Pattern data structures into gram notation text, handling syntax selection, escaping, and formatting.
- **Codec**: The combined parser and serializer that provides bidirectional transformation between gram notation and Pattern structures.
- **tree-sitter-gram**: The authoritative grammar specification for gram notation, implemented as a tree-sitter grammar with bindings for multiple languages.
- **gram-lint**: CLI tool for validating gram notation syntax, used to verify all parser output and test cases.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can parse valid gram notation into Pattern structures, with 100% of test cases covering all gram syntax forms successfully parsing when validated by `gram-lint`
- **SC-002**: Developers can serialize Pattern structures into valid gram notation, with 100% of serialized output passing `gram-lint` validation
- **SC-003**: Round-trip correctness is verified for all test cases: patterns parsed from gram notation, serialized, and re-parsed produce structurally equivalent results in 100% of cases
- **SC-004**: All gram syntax forms from tree-sitter-gram grammar are supported: node patterns, relationship patterns (all arrow types), subject patterns, annotations, comments, all value types, all identifier formats, all subject components
- **SC-005**: Parser provides helpful error messages for invalid gram notation, with error location (line and column) and error description (expected vs found) in 100% of error cases, and reports all syntax errors found (not just the first error) when error recovery is feasible
- **SC-006**: Parser and serializer complete operations within 100 milliseconds for typical gram notation (up to 1000 nodes, 100 levels of nesting)
- **SC-007**: Codec compiles successfully to WASM target with size under 500KB (compressed) and works in browser and Node.js environments
- **SC-008**: Codec supports Python integration through the chosen parser library or through clear integration path
- **SC-009**: All gram notation test cases from tree-sitter-gram test corpus are successfully parsed and round-tripped
- **SC-010**: Parser handles Unicode characters, emoji, and special characters correctly in identifiers and property values in 100% of test cases

## Assumptions

- tree-sitter-gram is the authoritative and complete grammar specification for gram notation, superseding any gram-hs specifications for parsing/serialization
- The `gram-lint` CLI tool correctly validates gram notation according to the tree-sitter-gram grammar and is used as the reference validator for all codec output
- Pattern data structures use Subject as the value type, where Subject contains optional identifier, labels, and record (properties)
- Round-trip correctness means structural equivalence: the pattern structure (value and elements recursively) is preserved, though formatting details like whitespace and comment placement may differ
- Parser library evaluation must prioritize: (1) tree-sitter-gram grammar compatibility, (2) WASM compilation support, (3) Python binding support, (4) performance, (5) error reporting quality
- Standard industry practice for parsers includes descriptive error messages with location information (line, column) and error nature (expected tokens vs found tokens)
- Serializer may use canonical formatting (e.g., consistent spacing, no comments) rather than preserving original formatting from parsed input
- Property types in gram notation (strings, numbers, booleans, arrays, ranges, tagged strings) map to a `Value` enum in the Pattern's Subject record, with enum variants for each type: String, Integer, Decimal, Boolean, Array (of scalar values), Range (lower and upper bounds), TaggedString (format tag and content)
- Relationship patterns in gram notation (2-element patterns with arrows) map to Patterns with 2 elements. Arrow types (→, ←, ↔, ~~) are syntactic sugar - the semantic information is in element ordering. Right arrow `(a)-->(b)` preserves order (left → right). Handling of left arrow (reverse elements or flag?), bidirectional, and undirected arrows needs further specification during planning phase.
- Subject pattern notation `[subject | elements]` is the general form that can represent any pattern, while node `()` and relationship `()-->()` notations are syntactic sugar for specific element counts
- The codec focuses on correctness and completeness first, with performance optimization (e.g., streaming, incremental parsing) as potential future enhancements
- Path flattening (converting linear relationship chains into nested structures) is handled by the parser according to tree-sitter-gram semantics
- Pattern references (identifiers that reference other patterns) are parsed as identifiers, with reference resolution being a separate concern outside the codec's scope
