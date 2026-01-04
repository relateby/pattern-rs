# Feature Specification: Pattern Combination Operations

**Feature Branch**: `013-semigroup-instance`  
**Created**: 2026-01-04  
**Status**: Draft  
**Input**: User description: "Semigroup trait for pattern combination operations"

## Context

This feature ports the pattern combination functionality from the gram-hs Haskell reference implementation to gram-rs. Pattern combination is a binary associative operation that enables two patterns to be combined into a single pattern while preserving the associativity law: `(a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)`.

In Haskell, this is expressed as a Semigroup instance. In Rust, the implementation should follow idiomatic patterns—this may be a concrete method (like `combine()` or `append()`), use of `std::ops::Add` if semantics align, or another approach that fits Rust conventions. The key requirement is the mathematical property of associativity, not a specific trait implementation.

**Foundation**: This feature builds on the Pattern data structure (features 004-006) and requires that the value type `V` also supports some form of combination operation.

**Reference**: The authoritative implementation is in `../gram-hs/libs/pattern/` (Haskell source code). Historical notes may exist in `../gram-hs/specs/010-semigroup-instance/` but should be verified against actual source.

**Implementation Guidance**: The implementation should prioritize Rust idioms over direct Haskell translation. Since Rust doesn't have a standard Semigroup trait and custom algebraic traits are non-idiomatic, prefer concrete methods or standard library traits where appropriate.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Combine Two Patterns (Priority: P1)

Developers need to combine two pattern structures into a single pattern. This enables building larger patterns incrementally, merging pattern fragments, and implementing compositional pattern construction APIs.

**Why this priority**: Pattern combination is a fundamental operation for compositional pattern building. It enables constructing complex pattern structures from simpler components and is a building block for higher-level operations like concatenating pattern sequences or merging pattern collections.

**Independent Test**: Can be fully tested by combining two patterns and verifying the resulting pattern structure matches expected semantics (element concatenation or value-level combination). This delivers immediate value for compositional pattern construction regardless of the specific API used.

**Acceptance Scenarios**:

1. **Given** pattern `p1` with value `v1` and elements `[e1, e2]` and pattern `p2` with value `v2` and elements `[e3, e4]`, **When** `p1` is combined with `p2`, **Then** the result is a pattern with combined value and concatenated elements
2. **Given** two atomic patterns (no elements), **When** combined, **Then** the result reflects the combination semantics appropriate for the value type
3. **Given** patterns with nested structures, **When** combined, **Then** the recursive structure is preserved and combination is applied correctly
4. **Given** a pattern combined with itself, **When** the operation completes, **Then** the result is well-formed and valid

---

### User Story 2 - Verify Associativity Law (Priority: P1)

Developers and the type system need assurance that pattern combination is associative, meaning `(a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)`. This mathematical property ensures that pattern combination behaves predictably regardless of grouping.

**Why this priority**: Associativity is the defining property of this combination operation. Without it, higher-level abstractions (like identity elements, fold operations) that depend on associativity will fail or produce unexpected results. Property-based testing must verify this law holds for all pattern structures.

**Independent Test**: Can be tested using property-based testing with randomly generated patterns, verifying that combining patterns in different groupings produces identical results. This is testable through the proptest framework.

**Acceptance Scenarios**:

1. **Given** three arbitrary patterns `a`, `b`, `c`, **When** computed as `(a <> b) <> c` and `a <> (b <> c)`, **Then** both results are structurally equal
2. **Given** patterns of varying depths and element counts, **When** associativity is tested, **Then** the law holds for all test cases
3. **Given** edge cases (empty elements, deeply nested patterns), **When** associativity is tested, **Then** the law holds
4. **Given** patterns with different value types (strings, integers, subjects), **When** associativity is tested, **Then** the law holds for all value types where combination is defined

---

### User Story 3 - Combine Multiple Patterns in Sequence (Priority: P2)

Developers need to combine multiple patterns in sequence (fold/reduce operation). This enables building patterns from collections of pattern fragments, implementing pattern concatenation operations, and constructing patterns from streams or iterators.

**Why this priority**: The ability to combine multiple patterns efficiently is a natural extension that demonstrates the practical utility of the binary operation. It's essential for real-world usage patterns where patterns are built incrementally from multiple sources.

**Independent Test**: Can be tested by folding/reducing a collection of patterns using the combination operation and verifying the result matches sequential pairwise combination. This tests that the operation works correctly when applied repeatedly.

**Acceptance Scenarios**:

1. **Given** a collection of patterns `[p1, p2, p3, p4]`, **When** combined using fold/reduce, **Then** the result equals `((p1 <> p2) <> p3) <> p4`
2. **Given** a collection with a single pattern, **When** combined, **Then** the result is that pattern unchanged
3. **Given** a collection of patterns with varying structures, **When** combined, **Then** all elements are incorporated correctly
4. **Given** an empty collection, **When** attempted to combine, **Then** the behavior is well-defined (requires a starting value or returns None/Option)

---

### Edge Cases

**Combination Operation**:
- How does combination work when both patterns are atomic (no elements)? (Combine values, result has no elements)
- How does combination work when one pattern is atomic and the other has elements? (Combine values, preserve elements from the non-atomic pattern)
- How does combination handle patterns with many elements (1000+ elements)? (Should work efficiently, O(n) time where n is total element count)
- How does combination handle deeply nested patterns (100+ levels)? (Should work without stack overflow)
- What happens when combining patterns with values that cannot be combined? (Type system should prevent this, or return Result/Option)

**Associativity Law**:
- Does associativity hold for patterns with mixed depths and structures? (Must hold for all valid patterns)
- Does associativity hold when values have their own complex combination semantics? (Must hold if value type's combination is associative)

**Performance**:
- What is the time complexity of combining two patterns? (Should be O(1) or O(n) depending on semantics)
- Does repeated combination cause excessive memory allocation? (Should reuse structure where possible)

## Requirements *(mandatory)*

### Functional Requirements

**Core Combination Operation**:
- **FR-001**: Pattern type MUST provide a binary combination operation that merges two patterns into one
- **FR-002**: The combination operation MUST be associative: `(a ⊕ b) ⊕ c` equals `a ⊕ (b ⊕ c)` for all valid patterns
- **FR-003**: The combination operation MUST work for patterns with any value type `V` that itself supports combination
- **FR-004**: The combination operation MUST handle atomic patterns (no elements) correctly
- **FR-005**: The combination operation MUST handle patterns with elements correctly
- **FR-006**: The combination operation MUST handle nested/recursive pattern structures correctly
- **FR-007**: The combination operation MUST preserve pattern validity (result is a well-formed pattern)

**Combination Semantics**:
- **FR-008**: When combining patterns with values of type `V`, the values MUST be combined using `V`'s combination operation (if `V` is a semigroup)
- **FR-009**: When combining patterns with element lists, the elements MUST be combined according to the defined semantics (element concatenation or other specified behavior)
- **FR-010**: The combination result MUST have a deterministic structure based on the inputs

**Testing & Verification**:
- **FR-011**: Property-based tests MUST verify the associativity law for randomly generated patterns
- **FR-012**: Unit tests MUST cover edge cases: atomic patterns, deeply nested patterns, patterns with many elements
- **FR-013**: Tests MUST verify behavioral equivalence with the gram-hs Haskell reference implementation
- **FR-014**: Benchmarks MUST measure combination performance for patterns of varying sizes

### Key Entities

- **Pattern<V>**: The recursive pattern structure with value of type `V` and elements (list of patterns)
- **Value Type V**: The type of values in patterns, which may or may not support its own combination operation
- **Combination Operation**: The binary associative operation that combines two patterns into one (may be exposed as a method, operator, or other API)
- **Associativity Law**: The mathematical property that `(a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)`

## Success Criteria *(mandatory)*

### Measurable Outcomes

**Core Functionality**:
- **SC-001**: Developers can combine two patterns and receive a well-formed result pattern with combined structure
- **SC-002**: Property-based tests verify associativity law for 10,000+ randomly generated pattern triples with 100% success rate
- **SC-003**: Unit tests achieve 100% code coverage for combination operation including all edge cases (atomic patterns, empty elements, deep nesting)

**Behavioral Equivalence**:
- **SC-004**: Combination behavior matches the gram-hs Haskell reference implementation for equivalent inputs (verified through cross-implementation testing)
- **SC-005**: All tests ported from gram-hs test suite pass without modification (except for syntax/API differences)

**Performance**:
- **SC-006**: Combining two patterns with up to 1000 elements each completes in under 1 millisecond
- **SC-007**: Combining patterns with nesting depth of 100 levels completes without stack overflow
- **SC-008**: Memory usage during combination is proportional to result size (no excessive temporary allocations)

**Integration**:
- **SC-009**: Pattern combination integrates cleanly with existing pattern operations (map, fold, traverse)
- **SC-010**: API documentation clearly explains combination semantics and provides usage examples

## Assumptions

**Combination Semantics**:
- The combination operation for patterns follows the semantics defined in the gram-hs reference implementation (to be verified during implementation research)
- If the value type `V` is also a semigroup, pattern combination uses `V`'s combination operation on values
- Element lists are combined through concatenation or another operation that preserves associativity
- The combination operation is purely functional (no side effects, no mutation)

**Type System & Rust Idioms**:
- The implementation should follow idiomatic Rust patterns rather than directly translating Haskell's typeclass approach
- Possible idiomatic approaches include: concrete methods (e.g., `combine()`, `append()`), `std::ops::Add` if semantics align, or other standard library traits
- Creating a custom `Semigroup` trait is discouraged as it's non-idiomatic in Rust (the ecosystem doesn't use algebraic typeclasses)
- The implementation may provide different combination strategies for different value types
- Type safety ensures that combination is only available when the value type supports it

**Performance**:
- Combination operation should be efficient enough for common use cases (patterns with hundreds of elements)
- The implementation may clone pattern structures as needed (Rust ownership model)
- Performance targets assume typical pattern structures; extreme cases (millions of elements) may be slower but must still complete successfully

## Dependencies

- **Pattern data structure** (features 004-006): Core Pattern<V> type must be implemented
- **Test infrastructure** (feature 003): Property-based testing framework (proptest) must be available
- **Existing pattern operations**: Integration with map, fold, and other operations

## Out of Scope

**Not Included in This Feature**:
- Identity element for combination (empty pattern) - separate feature (014-monoid-instance)
- Non-associative combination operations (violates the associativity requirement)
- In-place mutation or optimization of pattern structures during combination
- Parallel or concurrent combination of multiple patterns
- Lazy or streaming combination (all combination is eager)
- Custom combination strategies beyond the standard associative semantics
- Combination operations for specific graph or property graph interpretations (this is generic pattern combination)
- Prescriptive trait implementation details (the spec defines behavior, not API structure)

**Future Enhancements** (separate features):
- Identity element support for forming a monoid
- Optimized combination for specific value types
- Incremental combination with structural sharing
