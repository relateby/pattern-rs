# Feature Specification: Pattern Query Operations

**Feature Branch**: `011-basic-query-functions`  
**Created**: 2025-01-04  
**Status**: Draft (Refocused on missing functionality)  
**Input**: User description: "Pattern query operations as described in \"011-basic-query-functions\" of TODO.md"

## Context

**Already Implemented**: The basic structural query operations (`length`, `size`, `depth`, `values`) are already implemented in `crates/pattern-core/src/pattern.rs` from earlier features. These methods work correctly but need comprehensive test coverage.

**This Feature Focuses On**: Porting the missing predicate and search functions from the Haskell reference implementation (`anyValue`, `allValues`, `filter`) that enable pattern querying based on value predicates and pattern predicates.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Check if Any Value Satisfies Predicate (Priority: P1)

Developers need to check if at least one value in a pattern structure satisfies a given condition without extracting all values. This enables efficient value-based queries and validation without materializing intermediate collections.

**Why this priority**: The `any_value` operation is essential for conditional logic and validation. It enables short-circuit evaluation (stops as soon as a match is found) and is a fundamental query primitive used in pattern filtering, validation, and search operations.

**Independent Test**: Can be fully tested by calling `any_value` with various predicates on patterns with different value distributions and verifying it correctly identifies when at least one value matches. This delivers immediate value for pattern queries.

**Acceptance Scenarios**:

1. **Given** a pattern with values [1, 2, 3], **When** `any_value` is called with predicate "greater than 2", **Then** it returns true
2. **Given** a pattern with values [1, 2, 3], **When** `any_value` is called with predicate "greater than 5", **Then** it returns false
3. **Given** a nested pattern with values across multiple levels, **When** `any_value` is called, **Then** it checks all values at all nesting levels
4. **Given** an atomic pattern with a single value, **When** `any_value` is called, **Then** it evaluates the predicate on that value

---

### User Story 2 - Check if All Values Satisfy Predicate (Priority: P1)

Developers need to verify that all values in a pattern structure satisfy a given condition. This is critical for validation, invariant checking, and ensuring pattern consistency.

**Why this priority**: The `all_values` operation is essential for validation and constraint checking. It enables developers to verify that patterns meet specific requirements (e.g., all nodes are labeled, all values are non-negative) before processing.

**Independent Test**: Can be fully tested by calling `all_values` with various predicates on patterns with different value distributions and verifying it correctly identifies when all values match. This delivers immediate value for pattern validation.

**Acceptance Scenarios**:

1. **Given** a pattern with values [2, 4, 6], **When** `all_values` is called with predicate "is even", **Then** it returns true
2. **Given** a pattern with values [2, 3, 6], **When** `all_values` is called with predicate "is even", **Then** it returns false
3. **Given** a nested pattern with values across multiple levels, **When** `all_values` is called, **Then** it checks all values at all nesting levels
4. **Given** an empty pattern structure, **When** `all_values` is called, **Then** it returns true (vacuous truth)

---

### User Story 3 - Filter Patterns by Predicate (Priority: P2)

Developers need to extract subpatterns that satisfy specific pattern-level conditions. This enables pattern selection, extraction of graph components, and structural queries.

**Why this priority**: The `filter` operation enables structural queries that go beyond value-based filtering. While less frequently used than value predicates, it's essential for advanced pattern operations like extracting specific subgraphs or finding patterns with specific structural properties.

**Independent Test**: Can be fully tested by calling `filter` with various pattern predicates and verifying it returns only matching subpatterns. This delivers value for structural queries independently.

**Acceptance Scenarios**:

1. **Given** a nested pattern structure, **When** `filter` is called with a pattern predicate, **Then** it returns all subpatterns (including root) that satisfy the predicate
2. **Given** a pattern where no subpatterns match, **When** `filter` is called, **Then** it returns an empty collection
3. **Given** a pattern where all subpatterns match, **When** `filter` is called, **Then** it returns all subpatterns
4. **Given** patterns with varying nesting depths, **When** `filter` is called, **Then** it checks all subpatterns at all levels

---

### User Story 4 - Verify Existing Query Operations (Priority: P3)

Developers need comprehensive test coverage for the existing structural query operations (`length`, `size`, `depth`, `values`) to ensure they work correctly and match the reference implementation behavior.

**Why this priority**: These operations already exist and work correctly, but need comprehensive test coverage to ensure behavioral equivalence with the Haskell reference implementation and prevent regressions.

**Independent Test**: Can be fully tested by creating comprehensive test suites covering edge cases, large patterns, and comparing results with the Haskell reference implementation output.

**Acceptance Scenarios**:

1. **Given** existing `length`, `size`, `depth`, `values` operations, **When** comprehensive tests are run, **Then** all tests pass and cover edge cases
2. **Given** test cases from Haskell reference implementation, **When** equivalent tests are run in Rust, **Then** results match
3. **Given** patterns with various structures, **When** query operations are called, **Then** performance meets expected targets
4. **Given** the query API documentation, **When** developers read it, **Then** they can use query operations effectively without consulting implementation code

---

### Edge Cases

**Predicate Functions (New)**:
- How does `any_value` behave on an empty pattern? (Should return false - no values to satisfy predicate)
- How does `all_values` behave on an empty pattern? (Should return true - vacuous truth)
- How does `any_value` handle patterns where the first value matches? (Should short-circuit and return true immediately)
- How does `all_values` handle patterns where the first value fails? (Should short-circuit and return false immediately)
- How does `filter` behave when no patterns match? (Should return empty collection)
- How does `filter` handle predicates that match the root pattern? (Should include root in results)
- How do predicate functions handle very deeply nested patterns? (Should work correctly without stack overflow)
- How do predicate functions handle patterns with many direct elements? (Should work efficiently)

**Existing Query Operations (Verification)**:
- Do `length`, `size`, `depth`, `values` handle atomic patterns correctly? (Already implemented, needs test verification)
- Do existing operations handle deeply nested patterns without stack overflow? (Already implemented, needs test verification)
- Do existing operations match Haskell reference implementation behavior? (Needs equivalence testing)

## Requirements *(mandatory)*

### Functional Requirements

**New Predicate/Search Functions**:
- **FR-001**: System MUST provide an `any_value` operation that checks if at least one value in a pattern satisfies a given predicate
- **FR-002**: System MUST provide an `all_values` operation that checks if all values in a pattern satisfy a given predicate
- **FR-003**: System MUST provide a `filter` operation that extracts subpatterns (including root) that satisfy a given pattern predicate
- **FR-004**: The `any_value` operation MUST traverse all values at all nesting levels until a match is found or all values are checked
- **FR-005**: The `any_value` operation MUST short-circuit and return true as soon as a matching value is found
- **FR-006**: The `all_values` operation MUST traverse all values at all nesting levels until a non-match is found or all values are checked
- **FR-007**: The `all_values` operation MUST short-circuit and return false as soon as a non-matching value is found
- **FR-008**: The `all_values` operation MUST return true for empty patterns (vacuous truth)
- **FR-009**: The `filter` operation MUST check all subpatterns (including root) at all nesting levels
- **FR-010**: The `filter` operation MUST return subpatterns in a consistent order (pre-order traversal: root first, then elements)
- **FR-011**: All new operations MUST handle edge cases correctly (empty patterns, single nodes, deep nesting, many elements)

**Existing Query Operations (Verification)**:
- **FR-012**: Comprehensive test coverage MUST be added for existing `length`, `size`, `depth`, `values` operations
- **FR-013**: Tests MUST verify equivalence with Haskell reference implementation for existing operations
- **FR-014**: All query operations MUST have appropriate documentation explaining their behavior, complexity, and usage
- **FR-015**: Performance tests MUST verify that operations meet expected performance targets

### Key Entities

- **Pattern**: The recursive data structure being queried, containing a value and a list of element patterns
- **Value Predicate**: A function that takes a value and returns a boolean (used by `any_value` and `all_values`)
- **Pattern Predicate**: A function that takes a pattern and returns a boolean (used by `filter`)
- **Query Result**: The result returned by query operations (boolean for predicate checks, collection of patterns for filter, numeric/collection for existing operations)

## Success Criteria *(mandatory)*

### Measurable Outcomes

**New Predicate/Search Functions**:
- **SC-001**: Developers can use `any_value` to check if any value matches a predicate and receive accurate boolean results for patterns with up to 10,000 nodes in under 100 milliseconds
- **SC-002**: The `any_value` operation demonstrates short-circuit behavior by stopping as soon as a match is found (verified through performance tests)
- **SC-003**: Developers can use `all_values` to verify all values match a predicate and receive accurate boolean results for patterns with up to 10,000 nodes in under 100 milliseconds
- **SC-004**: The `all_values` operation demonstrates short-circuit behavior by stopping as soon as a non-match is found (verified through performance tests)
- **SC-005**: Developers can use `filter` to extract matching subpatterns and receive complete collections of all matching subpatterns in under 200 milliseconds for patterns with up to 10,000 nodes
- **SC-006**: All new operations handle edge cases correctly with 100% test coverage for empty patterns, single nodes, deep nesting (100+ levels), and many elements (1000+ direct elements)

**Existing Operations Verification**:
- **SC-007**: Comprehensive test suites exist for `length`, `size`, `depth`, `values` operations covering all edge cases and matching Haskell reference implementation behavior
- **SC-008**: All query operations are documented with clear examples and usage patterns, enabling developers to use them effectively without consulting implementation code
- **SC-009**: Query operation behavior matches the Haskell reference implementation for equivalent inputs (verified through cross-implementation testing)

## Assumptions

**New Predicate/Search Functions**:
- Predicate functions (`any_value`, `all_values`) should support short-circuit evaluation for performance
- The `all_values` operation returns true for empty patterns (vacuous truth - standard functional programming convention)
- The `filter` operation returns subpatterns in pre-order traversal order (root first, then elements in order)
- Predicate functions are pure (no side effects) and work with patterns of any value type
- Pattern predicates can access the entire pattern structure (value + elements) when filtering

**Existing Query Operations**:
- Basic query operations (`length`, `size`, `depth`, `values`) are already implemented and functional
- Existing operations follow Haskell reference implementation semantics (depth of 0 for atomic patterns, pre-order traversal for values, etc.)
- Performance targets assume typical pattern structures; very large or deeply nested patterns may take longer but should still complete successfully without stack overflow

## Dependencies

- Pattern data type must be fully implemented (already complete from features 004-006)
- Basic query operations (`length`, `size`, `depth`, `values`) already implemented in pattern-core
- Pattern traversal capability (already complete from feature 009) provides foundational functionality that predicate operations leverage
- Test infrastructure must be available (already complete from feature 003)

## Out of Scope

- Advanced path-based queries (XPath-like queries, JSONPath-like queries)
- Query optimization or caching mechanisms
- Query operations for specific pattern interpretations (graph views, property graph queries, RDF queries)
- Modification of pattern structure through queries (this would be a transformation, not a query)
- Query result streaming or pagination
- Custom query combinators or query builders (beyond the three basic predicate functions)
- Complex predicate composition helpers (users can compose predicates using standard functional programming techniques)
