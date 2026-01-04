# Feature Specification: Pattern Ordering and Comparison

**Feature Branch**: `012-ord-trait`  
**Created**: 2025-01-04  
**Status**: Draft  
**Input**: User description: "Implement ordering and comparison for Pattern types, porting PartialOrd and Ord traits from Haskell Ord typeclass"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Compare Patterns for Sorting (Priority: P1)

Developers working with collections of patterns need to sort them in a deterministic, consistent order. This enables operations like finding minimum/maximum patterns, maintaining sorted pattern collections, and implementing pattern-based data structures that require ordering (e.g., binary search trees, priority queues).

**Why this priority**: Core functionality that unlocks pattern-based algorithms and data structures. Without ordering, developers cannot reliably sort patterns or use ordered collections, limiting the utility of the pattern system.

**Independent Test**: Can be fully tested by creating multiple patterns, comparing them pairwise using comparison operators, and verifying that the ordering is consistent, transitive, and matches the reference Haskell implementation. Delivers the ability to sort pattern collections deterministically.

**Acceptance Scenarios**:

1. **Given** two atomic patterns with comparable values, **When** developer compares them using comparison operators, **Then** the comparison result matches the ordering of their values
2. **Given** two patterns with different structures, **When** developer compares them, **Then** the comparison follows a consistent, well-defined ordering rule (e.g., comparing values first, then structure)
3. **Given** a collection of patterns, **When** developer sorts them using standard sorting functions, **Then** the patterns are ordered consistently and deterministically
4. **Given** three patterns A, B, and C where A < B and B < C, **When** developer compares A and C, **Then** A < C (transitivity holds)

---

### User Story 2 - Find Extrema in Pattern Collections (Priority: P2)

Developers need to find the minimum or maximum pattern in a collection based on the defined ordering. This supports use cases like finding the "smallest" subpattern that matches certain criteria, or the "largest" pattern in a search result set.

**Why this priority**: Important for practical pattern querying and analysis operations. Builds on P1 ordering capability to provide higher-level operations.

**Independent Test**: Can be tested by creating pattern collections with known ordering, using min/max operations, and verifying correct identification of extrema. Delivers practical utility for pattern analysis workflows.

**Acceptance Scenarios**:

1. **Given** a non-empty collection of patterns, **When** developer requests the minimum pattern, **Then** the returned pattern is less than or equal to all other patterns in the collection
2. **Given** a non-empty collection of patterns, **When** developer requests the maximum pattern, **Then** the returned pattern is greater than or equal to all other patterns in the collection
3. **Given** a collection with a single pattern, **When** developer requests min or max, **Then** that pattern is returned

---

### User Story 3 - Use Patterns in Ordered Data Structures (Priority: P3)

Developers want to use patterns as keys in ordered data structures (like BTreeMap, BTreeSet) or as elements in data structures that require ordering (like binary heaps). This enables pattern-based indexing and efficient pattern lookups.

**Why this priority**: Enables advanced use cases and integration with the ecosystem's ordered collections. While valuable, it's not as fundamental as basic comparison and extrema operations.

**Independent Test**: Can be tested by inserting patterns into ordered data structures, performing lookups, and verifying that the structure maintains ordering invariants. Delivers the ability to build pattern-based indices and caches.

**Acceptance Scenarios**:

1. **Given** patterns are used as keys in an ordered map, **When** developer inserts and retrieves patterns, **Then** the map maintains ordering and allows efficient lookups
2. **Given** patterns are stored in a binary heap, **When** developer pops elements, **Then** they are retrieved in the correct order based on the defined comparison
3. **Given** patterns in an ordered set, **When** developer performs set operations, **Then** the set maintains ordering and prevents duplicates correctly

---

### Edge Cases

- What happens when comparing patterns with deeply nested structures (100+ levels)?
- How does comparison handle patterns with very wide structures (1000+ elements at one level)?
- What happens when comparing patterns with the same structure but different value types (generic type V)?
- How does comparison behave when patterns have equivalent values but different structural arrangements?
- What happens when comparing patterns where values are equal but one has more elements than the other?
- How does comparison perform with patterns containing non-comparable value types (should fail at compile time)?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide total ordering for patterns (any two patterns can be compared, and comparison is consistent)
- **FR-002**: System MUST implement comparison that is transitive (if A < B and B < C, then A < C)
- **FR-003**: System MUST implement comparison that is asymmetric (if A < B, then !(B < A))
- **FR-004**: System MUST support all standard comparison operations: less than (<), less than or equal (<=), greater than (>), greater than or equal (>=)
- **FR-005**: System MUST maintain behavioral equivalence with the Haskell reference implementation's Ord typeclass
- **FR-006**: System MUST use a consistent ordering strategy: compare values first, then recurse into structural comparison if values are equal
- **FR-007**: System MUST support partial ordering (PartialOrd trait) for patterns where value type V implements PartialOrd
- **FR-008**: System MUST support total ordering (Ord trait) for patterns where value type V implements Ord
- **FR-009**: Comparison operations MUST work correctly with atomic patterns (no elements)
- **FR-010**: Comparison operations MUST work correctly with nested patterns at arbitrary depth
- **FR-011**: System MUST provide comparison that is consistent with equality (if A == B, then !(A < B) and !(A > B))

### Key Entities

- **Pattern**: The recursive structure being compared, consisting of a value and a collection of element patterns
- **Ordering**: The result of comparison operations (Less, Equal, Greater), determining the relative position of patterns

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can sort collections of 10,000 patterns in under 200ms
- **SC-002**: All comparison operations maintain mathematical properties (transitivity, asymmetry, consistency with equality) verified through property-based testing
- **SC-003**: Comparison behavior matches Haskell reference implementation for 100% of ported test cases
- **SC-004**: Developers can use patterns as keys in ordered data structures without compilation errors or runtime panics
- **SC-005**: Deep patterns (200+ levels) can be compared without stack overflow
- **SC-006**: Wide patterns (5,000+ elements) can be compared and the operation completes in under 500ms

## Scope *(mandatory)*

### In Scope

- Implementing PartialOrd trait for Pattern<V> where V: PartialOrd
- Implementing Ord trait for Pattern<V> where V: Ord
- Defining consistent ordering rules for pattern comparison (value-first, then structural)
- Porting test cases from Haskell reference implementation
- Property-based tests for ordering invariants (transitivity, asymmetry, etc.)
- Performance testing for comparison operations on large patterns
- Documentation of ordering semantics and usage examples

### Out of Scope

- Custom comparison strategies or user-defined ordering (this feature defines one canonical ordering)
- Comparison optimization through memoization or caching (can be added later if needed)
- Parallel comparison for very large patterns (linear comparison is sufficient for MVP)
- Specialized comparison for specific value types (generic implementation only)

## Dependencies & Assumptions

### Dependencies

- **Pattern Type**: Depends on the existing Pattern<V> type definition (feature 004)
- **Equality Traits**: Depends on existing PartialEq and Eq implementations for Pattern (feature 004)
- **Haskell Reference**: Requires access to `../gram-hs/libs/` for Ord typeclass implementation
- **Test Infrastructure**: Depends on property-based testing framework (proptest) from feature 003

### Assumptions

- Pattern's value type V already implements PartialOrd or Ord (comparison not possible otherwise)
- Patterns are finite structures (no infinite recursion)
- Standard Rust ordering semantics apply (follows std::cmp::Ordering conventions)
- Performance targets assume patterns of "typical" size (depths up to 200, widths up to 5,000)
- Haskell's Ord typeclass implementation provides the authoritative ordering semantics

### Reference Implementation Notes

- **Primary Source**: `../gram-hs/libs/` - Haskell Ord typeclass instance for Pattern
- **Documentation**: `../gram-hs/docs/` - Ordering semantics and usage examples
- **Test Cases**: `../gram-hs/libs/*/tests/` - Property tests and concrete comparison examples
- **Historical Context**: `../gram-hs/specs/009-ord-instance/` - Development notes (verify against actual code)
