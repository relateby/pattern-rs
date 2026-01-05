# Feature Specification: Predicate-Based Pattern Matching

**Feature Branch**: `016-predicate-matching`  
**Created**: 2025-01-05  
**Status**: Draft  
**Input**: User description: "Pattern matching as described in 016-predicate-matching: Pattern Matching of TODO.md"

## Clarifications

### Session 2025-01-05

- Q: How should "find first pattern" represent "no match found" for idiomatic Rust (Option vs Result vs panic vs empty collection)? → A: Return Option<T> where None means no match
- Q: Should functions that return "all matching subpatterns" return owned Vec, Iterator, slice references, or both variants? → A: Return Iterator (lazy evaluation)
- Q: Should filter and find functions return references to patterns, owned/cloned patterns, smart pointers, or both variants? → A: Return references (borrowed)
- Q: Should predicate closures use Fn (immutable/reusable), FnMut (mutable state), FnOnce (consumes), or accept all via generic bounds? → A: Fn trait (immutable, reusable)
- Q: What traversal order should determine which pattern is "first" (depth-first pre-order, breadth-first, depth-first post-order, unspecified)? → A: Depth-first (pre-order traversal)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Query Patterns by Value Properties (Priority: P1)

As a developer using the Pattern library, I need to check whether patterns contain values that satisfy specific conditions, so that I can filter and query patterns based on value properties without examining exact values or navigating the structure manually.

**Why this priority**: Value-based predicates are the most fundamental query capability. They enable developers to ask questions like "does this pattern contain any negative numbers?" or "are all values in this pattern valid?" This is essential for validation, filtering, and conditional logic based on value properties. Without this, developers must manually traverse structures and extract values, leading to error-prone code.

**Independent Test**: Can be fully tested by providing patterns with various values and predicates, then verifying that the results correctly indicate whether any or all values satisfy the predicate. Testing includes atomic patterns (single value), nested patterns (multiple levels), empty patterns, and edge cases. Delivers immediate value by enabling value-based pattern queries.

**Acceptance Scenarios**:

1. **Given** a pattern containing values `[5, 10, 3, 7]`, **When** checking if any value is greater than 8, **Then** the function returns true (10 satisfies the predicate)
2. **Given** a pattern containing values `[5, 10, 3, 7]`, **When** checking if all values are positive, **Then** the function returns true (all values satisfy the predicate)
3. **Given** an atomic pattern with value 5, **When** checking if any value is greater than 3, **Then** the function returns true
4. **Given** a nested pattern with values at multiple levels, **When** applying value predicates, **Then** all values at all nesting levels are considered
5. **Given** a pattern where no values match the predicate, **When** checking if any value matches, **Then** it returns false
6. **Given** a pattern where some but not all values match, **When** checking if all values match, **Then** it returns false

---

### User Story 2 - Find and Filter Patterns by Structure (Priority: P2)

As a developer using the Pattern library, I need to find specific subpatterns within a pattern structure that match certain criteria, so that I can extract relevant pattern components based on structural or value-based conditions without writing complex traversal code.

**Why this priority**: Pattern-based predicates enable finding and filtering subpatterns based on their structure, values, or both. This is essential for pattern analysis, extraction, and transformation workflows. For example, finding all leaf patterns, finding patterns with specific depths, or extracting patterns with certain element sequences. This significantly simplifies complex pattern manipulation tasks.

**Independent Test**: Can be fully tested by providing patterns with various structures and predicates, then verifying that all matching subpatterns are found correctly. Testing includes patterns with different nesting levels, atomic patterns, patterns with repeating structures, and edge cases. Delivers immediate value by enabling structural pattern queries without custom traversal logic.

**Acceptance Scenarios**:

1. **Given** a pattern structure with nested subpatterns, **When** filtering for patterns that have no elements (atomic patterns), **Then** an iterator over all leaf patterns is returned
2. **Given** a pattern structure, **When** finding the first pattern that matches a predicate (e.g., value equals "target"), **Then** the first matching subpattern in depth-first pre-order traversal is returned, or no result if none match
3. **Given** a pattern structure, **When** filtering for patterns that match a predicate, **Then** the root pattern is included if it matches the predicate
4. **Given** a deeply nested pattern structure, **When** filtering or finding, **Then** all nested subpatterns at all depths are considered
5. **Given** a pattern where multiple subpatterns match the predicate, **When** filtering, **Then** an iterator over all matching subpatterns is returned
6. **Given** a pattern where no subpatterns match the predicate, **When** filtering, **Then** an empty iterator is returned

---

### User Story 3 - Match Patterns by Structure (Priority: P3)

As a developer using the Pattern library, I need to check if one pattern matches or contains another pattern structurally, so that I can perform structural pattern matching and containment checks that go beyond simple equality comparison.

**Why this priority**: Structural pattern matching enables checking if patterns have matching structures (same values and element arrangement recursively) or if one pattern contains another as a subpattern. This extends beyond exact equality to support structural comparison and containment queries, which are essential for pattern analysis, validation, and comparison tasks.

**Independent Test**: Can be fully tested by providing pairs of patterns and verifying that structural matching and containment checks return correct results. Testing includes identical patterns, patterns with different structures, patterns with same values but different arrangements, and patterns with subpattern relationships. Delivers value by enabling structural comparisons without manual recursive checks.

**Acceptance Scenarios**:

1. **Given** two patterns with identical values and structure, **When** checking if one matches the other structurally, **Then** the function returns true
2. **Given** two patterns with same values but different element arrangements, **When** checking structural matching, **Then** the function returns false (structure matters)
3. **Given** a pattern and one of its subpatterns, **When** checking if the pattern contains the subpattern, **Then** the function returns true
4. **Given** a pattern and itself, **When** checking structural matching, **Then** it returns true (self-matching)
5. **Given** a pattern and itself, **When** checking containment, **Then** it returns true (pattern contains itself)
6. **Given** two patterns with no structural relationship, **When** checking containment, **Then** it returns false

---

### Edge Cases

- What happens when applying value predicates to an atomic pattern with a single value?
- What happens when applying value predicates to a pattern where all values match the predicate?
- What happens when applying value predicates to a pattern where no values match the predicate?
- What happens when filtering patterns with a predicate that matches the root pattern?
- What happens when filtering patterns with a predicate that matches no subpatterns (empty iterator)?
- What happens when finding first pattern with a predicate that matches no subpatterns (no result/absence)?
- What happens when filtering patterns with a predicate that matches all subpatterns (iterator over all)?
- What happens when finding patterns in deeply nested structures (100+ levels)?
- What happens when checking structural matching between patterns with identical values but different structures?
- What happens when checking containment where the subpattern appears multiple times in the structure?
- What happens when checking structural matching or containment with atomic patterns?
- What happens when predicates examine both structure and values versus only one aspect?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a function that checks if any value in a pattern satisfies a given predicate function that can be called multiple times without consuming state
- **FR-002**: System MUST provide a function that checks if all values in a pattern satisfy a given predicate function that can be called multiple times without consuming state
- **FR-003**: Value predicate functions MUST consider all values at all nesting levels in the pattern structure (flattened values)
- **FR-004**: Value predicate functions MUST handle atomic patterns correctly by evaluating the predicate on the pattern's single value
- **FR-005**: Value predicate functions MUST short-circuit when possible (e.g., "any" stops on first match)
- **FR-006**: System MUST provide a function that filters all subpatterns (including root) that match a pattern predicate that can be called multiple times, using depth-first pre-order traversal, returning an iterator over borrowed references for lazy evaluation and zero-cost abstraction
- **FR-007**: System MUST provide a function that finds the first subpattern (including root) that matches a pattern predicate that can be called multiple times, using depth-first pre-order traversal, returning an optional borrowed reference where absence indicates no match found
- **FR-008**: System MUST provide a function that finds all subpatterns (including root) that match a pattern predicate that can be called multiple times, using depth-first pre-order traversal, returning an iterator over borrowed references for lazy evaluation and zero-cost abstraction
- **FR-009**: Pattern predicate functions MUST consider the root pattern and all nested subpatterns at all depths
- **FR-010**: Pattern predicate functions MUST allow predicates to examine structure (element count, depth) and values
- **FR-011**: Pattern predicate functions MUST enable matching on element sequences and structural patterns (e.g., repetition, palindromes)
- **FR-012**: System MUST provide a function that checks if one pattern matches another structurally (value and elements recursively)
- **FR-013**: System MUST provide a function that checks if one pattern contains another as a subpattern anywhere in its structure
- **FR-014**: Structural matching functions MUST distinguish patterns based on structure, not just flattened values
- **FR-015**: Structural matching functions MUST handle self-matching and self-containment correctly
- **FR-016**: All predicate and matching functions MUST handle edge cases: atomic patterns, empty elements, deeply nested structures, no matches, all matches
- **FR-017**: System MUST maintain equivalence with the reference implementation's behavior (gram-hs) for all predicate and matching operations

### Key Entities

- **Pattern**: A recursive structure representing a decorated sequence, where elements form the pattern and value provides decoration. Patterns can be atomic (no elements) or contain nested patterns as elements.
- **Value Predicate**: A reusable function that takes a value and returns a boolean, used to test individual values within patterns (operates on flattened values from all nesting levels). Must be callable multiple times without consuming state.
- **Pattern Predicate**: A reusable function that takes a pattern and returns a boolean, used to test entire pattern structures including structural properties (element count, depth, element sequences). Must be callable multiple times without consuming state.
- **Subpattern**: Any pattern that appears within another pattern's structure, including the root pattern itself and all nested patterns at any depth. Functions that return subpatterns provide borrowed references rather than owned values.
- **Structural Matching**: Checking if two patterns have identical structure (same values and element arrangement recursively), distinguishing patterns with same values but different structures.
- **Subpattern Containment**: Checking if one pattern appears anywhere in another pattern's structure, including as the root pattern itself.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can query patterns by value properties, with value predicate functions correctly identifying matches in 100% of test cases across atomic patterns, nested patterns with up to 100 nesting levels, and edge cases
- **SC-002**: Developers can find and filter subpatterns by structure, with pattern predicate functions correctly identifying all matching subpatterns in 100% of test cases, including root and nested patterns up to 100 levels deep
- **SC-003**: Developers can perform structural pattern matching, with matching functions correctly distinguishing patterns based on structure (not just values) in 100% of test cases
- **SC-004**: All predicate and matching functions handle edge cases correctly (atomic patterns, empty elements, deeply nested structures with 100+ levels, no matches, all matches) with 100% test coverage
- **SC-005**: Value predicate operations that can short-circuit (any value checks, first pattern finds) complete within 10 milliseconds for patterns with up to 1000 nodes when a match is found in the first 10 nodes
- **SC-006**: All predicate and matching operations complete within 100 milliseconds for patterns with up to 1000 nodes and nesting depth up to 100 levels
- **SC-007**: Predicate and matching functions produce identical results to the reference implementation (gram-hs) in 100% of behavioral equivalence test cases

## Assumptions

- Value predicates operate on individual values extracted from all nesting levels (flattened values), consistent with fold semantics
- Pattern predicates operate on full pattern structures, enabling examination of structural properties (element sequences, depth, size) and values
- Structural matching requires exact structural correspondence (value and elements recursively), not just equivalent flattened values
- Subpattern containment checks if a pattern appears anywhere in another pattern's structure, including as the root pattern itself
- All predicate functions traverse the entire pattern structure (all nesting levels) to ensure comprehensive matching
- Pattern predicate functions include the root pattern in their search scope, not just nested subpatterns
- The reference implementation (gram-hs) provides authoritative behavior that must be matched for behavioral equivalence
- Standard industry practice for predicate functions includes short-circuit evaluation where applicable (e.g., "any" stops on first match)
- Performance targets assume reasonable patterns (up to 1000 nodes, 100 levels) that cover typical use cases
- Functions returning multiple results use iterators for lazy evaluation, enabling zero-cost abstraction and allowing users to collect into desired collection types
- Filter and find functions return borrowed references to patterns rather than owned values, avoiding unnecessary cloning and following Rust's principle of "don't pay for what you don't use"
- Predicate closures must be reusable (callable multiple times without consuming state), consistent with functional programming semantics where predicates are pure functions
- Pattern traversal uses depth-first pre-order (root first, then elements recursively), consistent with the reference implementation and existing fold/map operations, providing O(depth) memory efficiency
