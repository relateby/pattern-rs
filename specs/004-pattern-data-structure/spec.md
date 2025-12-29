# Feature Specification: Core Pattern Data Structure

**Feature Branch**: `004-pattern-data-structure`  
**Created**: 2025-01-27  
**Status**: Draft  
**Input**: User description: "Implement the core pattern type as detailed in \"004-pattern-data-structure: Core Pattern Type\" of TODO.md"

## Clarifications

### Session 2025-01-27

- Q: What is the distinction between Subject types as structural elements versus value elements? → A: Subjects are VALUE elements, not structural elements. Pattern<V> is a generic recursive nested structure that works with any value type V. Pattern<Subject> is useful for replacing object-graphs with nested patterns, which may be VIEWED as graphs through interpretation operations, but patterns themselves are not graphs. The "subject notation" in gram notation is the generic notation for patterns, not a special structural element.

- Q: Are patterns trees or s-expressions? What is the relationship between pattern value and elements? → A: Patterns are NOT trees - they are recursive, nested structures more like s-expressions. They may appear tree-like and accept tree-like operations, but they're fundamentally s-expression-like structures. The V value of a pattern is "information about the elements" - they form an intimate pairing. The elements happen to also be patterns, creating the recursive structure.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Developer Creates Pattern Instances (Priority: P1)

A developer needs to create pattern instances as recursive, nested structures (s-expression-like) in their application. They need a type system that allows them to construct patterns where the value provides information about the elements, and the elements are themselves patterns, enabling representation of complex nested data structures that may later be interpreted as graphs through separate operations.

**Why this priority**: Pattern creation is the foundational operation that enables all other pattern functionality. Without the ability to create patterns, developers cannot use the library for any purpose. This is the most basic requirement.

**Independent Test**: Can be fully tested by verifying that developers can create pattern instances with different values and element structures, and that these patterns can be inspected (via Debug/Display) to confirm their structure matches expectations.

**Acceptance Scenarios**:

1. **Given** the Pattern type is defined, **When** a developer creates a pattern with a value and no elements, **Then** they can successfully instantiate the pattern and inspect its structure
2. **Given** the Pattern type supports recursion, **When** a developer creates a pattern with nested elements, **Then** they can create patterns of arbitrary depth and structure
3. **Given** patterns are generic over value type, **When** a developer creates patterns with different value types (strings, numbers, custom types), **Then** the type system correctly handles each value type
4. **Given** patterns are created, **When** a developer inspects a pattern using Debug or Display, **Then** they see a clear representation of the pattern's structure

---

### User Story 2 - Developer Works with Subject Type (Priority: P1)

A developer needs to work with the Subject type as a value type when building applications that use patterns to replace object-graphs. They need to be able to use Subject as a pattern value in `Pattern<Subject>`, enabling nested patterns that may be viewed as graphs through interpretation operations.

**Why this priority**: Subject provides a semantic value type that is commonly used with patterns. While `Pattern<V>` works with any value type, `Pattern<Subject>` is a common use case for replacing object-graphs with nested patterns. Subject is a value element, not a structural element - the pattern structure comes from the recursive nested structure (s-expression-like), while Subject is one possible value type.

**Independent Test**: Can be fully tested by verifying that Subject can be used as a pattern value in `Pattern<Subject>`, and that its structure can be inspected. Developers can create patterns with Subject values and verify the Subject information is preserved in the pattern value.

**Acceptance Scenarios**:

1. **Given** the Subject type is defined, **When** a developer creates a Subject with identity, labels, and properties, **Then** they can use it as a pattern value in `Pattern<Subject>` and the Subject is stored as the pattern's value
2. **Given** Subject supports labels, **When** a developer creates a Subject with labels, **Then** those labels are preserved in the Subject value stored in the pattern
3. **Given** Subject supports properties, **When** a developer creates a Subject with properties, **Then** those properties are preserved in the Subject value stored in the pattern
4. **Given** a pattern contains a Subject value, **When** a developer inspects the pattern, **Then** they can see the Subject information (identity, labels, properties) in the pattern's value component

---

### User Story 3 - Developer Inspects Pattern Structure (Priority: P2)

A developer needs to inspect and debug pattern structures during development. They need human-readable representations of patterns that show the structure, values, and relationships between elements.

**Why this priority**: Debugging and inspection capabilities are essential for development workflow. While not as critical as pattern creation, the ability to see what patterns look like is necessary for verifying correctness and understanding behavior.

**Independent Test**: Can be fully tested by verifying that patterns implement Debug and Display traits, and that the output is readable and accurately represents the pattern structure. Developers can print patterns and see meaningful output.

**Acceptance Scenarios**:

1. **Given** patterns implement Debug trait, **When** a developer uses `{:?}` format specifier, **Then** they see a structured representation suitable for debugging
2. **Given** patterns implement Display trait, **When** a developer uses `{}` format specifier, **Then** they see a human-readable representation of the pattern
3. **Given** patterns have nested structures, **When** a developer prints a pattern, **Then** the output clearly shows the nesting and hierarchy
4. **Given** patterns contain Subject values, **When** a developer prints a pattern, **Then** the Subject information is clearly visible in the output

---

### User Story 4 - Developer Verifies Behavioral Equivalence with gram-hs (Priority: P2)

A developer needs to verify that pattern instances created in gram-rs behave identically to patterns created in the gram-hs reference implementation. They need confidence that the port maintains correctness and that patterns can be used interchangeably between implementations.

**Why this priority**: Behavioral equivalence is critical for maintaining correctness during the port. While not required for basic functionality, verification ensures the port is faithful and prevents divergence from the reference implementation.

**Independent Test**: Can be fully tested by creating equivalent patterns in both gram-rs and gram-hs, comparing their structure and behavior, and verifying they match. Test cases from gram-hs can be ported and executed in gram-rs with identical results.

**Acceptance Scenarios**:

1. **Given** test cases from gram-hs are available, **When** a developer runs equivalence tests, **Then** patterns created in gram-rs match the structure and behavior of patterns in gram-hs
2. **Given** equivalence checking utilities exist, **When** a developer creates a pattern in gram-rs, **Then** they can verify it matches the expected gram-hs pattern structure
3. **Given** test data is extracted from gram-hs, **When** a developer runs tests, **Then** gram-rs patterns produce the same results as gram-hs for identical inputs
4. **Given** behavioral equivalence is verified, **When** a developer uses patterns, **Then** they can trust that behavior matches the reference implementation

---

### User Story 5 - Developer Compiles Patterns for WASM (Priority: P3)

A developer needs to use patterns in web applications via WebAssembly. They need confidence that pattern types compile successfully for WASM targets and can be used in browser or Node.js environments.

**Why this priority**: WASM support is a long-term goal but not required for initial pattern functionality. However, establishing WASM compatibility early ensures the design doesn't preclude WASM usage and validates the architecture.

**Independent Test**: Can be fully tested by compiling the pattern-core crate for `wasm32-unknown-unknown` target and verifying successful compilation without errors. The patterns don't need to be usable from JavaScript yet, just compilable.

**Acceptance Scenarios**:

1. **Given** pattern types are defined, **When** a developer compiles for `wasm32-unknown-unknown`, **Then** compilation succeeds without errors
2. **Given** patterns compile for WASM, **When** a developer reviews the compilation output, **Then** they see that pattern types are included in the WASM module
3. **Given** WASM compilation works, **When** a developer uses patterns, **Then** they know the types are compatible with WASM targets

---

### Edge Cases

- What happens when a pattern has deeply nested elements (very large depth)? (Should handle reasonable depths without stack overflow)
- How does the system handle patterns with many elements (very wide structures)? (Should handle reasonable widths efficiently)
- What happens when a pattern value is empty or null? (Should support empty/optional values appropriately)
- How are patterns with circular references handled? (Should prevent or detect cycles if they're not allowed)
- What happens when Subject types have invalid or malformed data? (Should validate or handle gracefully)
- How does Debug/Display handle patterns that are too large to print? (Should truncate or summarize very large patterns)
- What happens when patterns are compared for equality with different value types? (Should handle type mismatches appropriately)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a `Pattern<V>` type that is generic over value type `V` and supports recursive element structures
- **FR-002**: System MUST allow patterns to be constructed with a value (which provides information about the elements) and a collection of nested pattern elements, where the value and elements form an intimate pairing
- **FR-003**: System MUST provide a `Subject` type that can be used as a pattern value in `Pattern<Subject>`. Subject contains identity (Symbol), labels (Set<String>), and properties (Map<String, Value>).
- **FR-004**: System MUST implement `Debug` trait for patterns to enable debugging and inspection
- **FR-005**: System MUST implement `Display` trait for patterns to enable human-readable output
- **FR-006**: System MUST support patterns with empty element lists (atomic patterns)
- **FR-007**: System MUST support patterns with nested elements of arbitrary depth (within reasonable limits)
- **FR-008**: System MUST compile successfully for `wasm32-unknown-unknown` target
- **FR-009**: System MUST maintain behavioral equivalence with gram-hs reference implementation for pattern structure
- **FR-010**: System MUST support pattern equality comparison (via `PartialEq` and `Eq` traits)
- **FR-011**: System MUST allow patterns to be cloned (via `Clone` trait) for value semantics
- **FR-012**: System MUST place pattern types in the `pattern-core` crate as the foundational data structure
- **FR-013**: System MUST support Subject type with identity, labels, and properties when used as pattern values in `Pattern<Subject>`.
- **FR-014**: System MUST enable porting of test cases from gram-hs to verify correctness

### Key Entities *(include if feature involves data)*

- **Pattern<V>**: A recursive, nested structure (s-expression-like) that is generic over value type `V`. Contains a value of type `V` (which provides information about the elements) and a collection of nested pattern elements. The value and elements form an intimate pairing - the value is information about the elements, and the elements are themselves patterns. This is the core data structure - a recursive nested structure that may appear tree-like and accept tree-like operations, but is fundamentally s-expression-like. Patterns may be interpreted as graphs through separate operations, but patterns themselves are not graphs.

- **Subject**: A self-descriptive value type that can be used as a pattern value. Subject contains identity (Symbol), labels (Set<String>), and properties (Map<String, Value>). Subject is a VALUE element, not a structural element. When used in `Pattern<Subject>`, it provides semantic value types for patterns that may be used to replace object-graphs with nested patterns. This feature defines the Subject type.

- **Pattern Value**: The value component of a pattern, which can be any type `V`. The value provides "information about the elements" - it forms an intimate pairing with the elements. The value does not determine the pattern's structure - structure comes from the recursive nested structure of elements. When `V` is a Subject type, the pattern contains a Subject value that provides information about the pattern's elements.

- **Pattern Elements**: The nested collection of patterns that form the recursive nested structure of a pattern. Elements are themselves patterns, creating the recursive structure. The value and elements form an intimate pairing where the value provides information about the elements. Patterns are s-expression-like structures, not trees, though they may appear tree-like and accept tree-like operations.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can create pattern instances with values and nested elements, and successfully compile code using patterns (verified by compilation success and basic instantiation tests)
- **SC-002**: Developers can create patterns using the Subject type and use it as a pattern value (verified by successful pattern creation with Subject values)
- **SC-003**: Developers can print patterns using Debug (`{:?}`) and Display (`{}`) format specifiers and see readable output (verified by format output tests)
- **SC-004**: Pattern types compile successfully for `wasm32-unknown-unknown` target without errors (verified by WASM compilation test)
- **SC-005**: Patterns created in gram-rs match the structure and behavior of equivalent patterns in gram-hs for at least 95% of test cases from gram-hs (verified by equivalence test suite)
- **SC-006**: Developers can clone patterns and compare them for equality, with equality working correctly for patterns with identical structure and values (verified by Clone and Eq trait tests)
- **SC-007**: Pattern types are available in the `pattern-core` crate and can be imported and used by other crates in the workspace (verified by cross-crate usage tests)

## Assumptions

- Pattern structure follows the gram-hs reference implementation: `Pattern { value: V, elements: Vec<Pattern<V>> }`
- Patterns are recursive, nested structures (s-expression-like), NOT trees - they may appear tree-like and accept tree-like operations, but are fundamentally s-expression-like structures
- The pattern value V provides "information about the elements" - the value and elements form an intimate pairing
- Patterns are not graphs - graph interpretation is a separate operation on patterns
- Subject is a VALUE type that can be used as a pattern value, not a structural element - it is one possible value type for `Pattern<V>`. This feature defines the Subject type.
- `Pattern<Subject>` is a common use case for replacing object-graphs with nested patterns, but patterns themselves are not graphs
- Patterns support reasonable nesting depths (at least 100 levels) without performance issues
- Patterns support reasonable element counts (at least 10,000 elements) without performance issues
- Debug and Display implementations should be readable but don't need to match gram-hs output format exactly (structure should be clear)
- Behavioral equivalence means structural equivalence and equality behavior, not necessarily identical memory layout
- WASM compilation means successful compilation, not necessarily working JavaScript bindings (those come in later features)
- Test cases can be extracted from gram-hs using existing test synchronization infrastructure
- Pattern types will be extended with additional functionality (operations, traits) in subsequent features
- The `pattern-core` crate already exists as a placeholder and just needs the type definitions added

## Dependencies

- **Feature 001 (Rust Init)**: Provides Rust project structure and build configuration
- **Feature 002 (Workspace Setup)**: Provides multi-crate workspace with `pattern-core` crate
- **Feature 003 (Test Infrastructure)**: Provides testing framework, equivalence checking utilities, and test synchronization infrastructure for verifying behavioral equivalence with gram-hs
- **gram-hs Reference Implementation**: Provides the reference specification, type signatures, and test cases at `../gram-hs/specs/001-pattern-data-structure/`

## References

- **Primary Source (Authoritative)**: **gram-hs Implementation**: `../gram-hs/libs/` - Haskell library source code
  - Pattern: `../gram-hs/libs/pattern/src/Pattern.hs`
  - Subject: `../gram-hs/libs/subject/src/Subject/Core.hs`
  - Tests: `../gram-hs/libs/*/tests/`
- **Secondary Source (Context Only)**: gram-hs Design Documents: `../gram-hs/specs/001-pattern-data-structure/`
  - Feature Specification: `../gram-hs/specs/001-pattern-data-structure/spec.md` (for context, may be outdated)
  - Type Signatures: `../gram-hs/specs/001-pattern-data-structure/contracts/type-signatures.md` (for context, may be outdated)
- **Porting Guide**: `PORTING_GUIDE.md` - Systematic approach for porting from gram-hs
- **Project Plan**: `docs/gram-rs-project-plan.md` - Overall architecture and design decisions
- **TODO**: `TODO.md` - Feature tracking and porting checklist

**Important**: The Haskell implementation in `../gram-hs/libs/` is the authoritative source of truth. Design documents in `../gram-hs/specs/` are useful for context but may contain outdated information or design mistakes that were corrected in the actual implementation.
