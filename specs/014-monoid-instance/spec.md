# Feature Specification: Pattern Identity Element via Default Trait

**Feature Branch**: `014-monoid-instance`  
**Created**: 2026-01-05  
**Status**: Draft  
**Input**: User description: "Support feature 014-monoid-instance in TODO.md by implementing `Default` trait and documenting the monoid laws in tests/docs"

## Context

This feature adds identity element support to Pattern combination operations by implementing Rust's standard `Default` trait. In mathematical terms, this completes the Monoid structure (Semigroup + identity) for patterns, enabling patterns to have a well-defined "empty" or "neutral" value that acts as an identity under combination.

In Haskell, this is expressed as a Monoid instance with `mempty` as the identity element. In Rust, the idiomatic approach is to use the standard library's `Default` trait, which provides a canonical "default" or "zero" value for types. While `Default` doesn't explicitly encode monoid laws, it serves the same practical purpose and integrates seamlessly with Rust's ecosystem.

**Foundation**: This feature builds on:
- Pattern data structure (features 004-006)
- Pattern combination operations via `Combinable` trait (feature 013-semigroup-instance)

**Reference**: The authoritative implementation is in `../gram-hs/libs/pattern/` (Haskell source code). Historical notes may exist in `../gram-hs/specs/011-monoid-instance/` but should be verified against actual source.

**Implementation Guidance**: Follow idiomatic Rust patterns by implementing `std::default::Default` rather than creating a custom Monoid trait. Document monoid laws in test comments and documentation, not in trait constraints.

**Monoid Laws**: The implementation must satisfy:
1. **Left Identity**: `empty.combine(x) == x` for all patterns x
2. **Right Identity**: `x.combine(empty) == x` for all patterns x

Where `empty` is `Pattern::default()` for value types that implement `Default`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Create Identity Pattern (Priority: P1)

Developers need a standard way to create an "empty" or "neutral" pattern that serves as an identity element for combination operations. This enables using patterns with Rust's standard iterator methods (`fold`) and provides a starting point for building patterns incrementally.

**Why this priority**: Identity elements are essential for many common patterns in functional programming and iterator operations. Without a defined identity, operations like `fold` require arbitrary starting values, and the meaning of "empty pattern" is unclear.

**Independent Test**: Can be fully tested by creating a default pattern and verifying it behaves as an identity under combination (combining with any pattern yields that pattern unchanged).

**Acceptance Scenarios**:

1. **Given** a pattern type with value type that implements `Default`, **When** `Pattern::default()` is called, **Then** a valid pattern with default value and empty elements is created
2. **Given** any pattern `p` and the default pattern `empty`, **When** `empty.combine(p)` is computed, **Then** the result equals `p`
3. **Given** any pattern `p` and the default pattern `empty`, **When** `p.combine(empty)` is computed, **Then** the result equals `p`
4. **Given** a default pattern, **When** combined with itself, **Then** the result is still a default pattern

---

### User Story 2 - Verify Identity Laws (Priority: P1)

Developers and the type system need assurance that the default pattern behaves as a proper identity element for combination, satisfying the monoid identity laws. This mathematical property ensures predictable behavior in iterator operations and functional composition.

**Why this priority**: Identity laws are the defining properties of a monoid. Without them, the default pattern would just be an arbitrary pattern value rather than a meaningful identity element. Property-based testing must verify these laws hold.

**Independent Test**: Can be tested using property-based testing with randomly generated patterns, verifying that combining with the default pattern (on either side) produces the original pattern unchanged.

**Acceptance Scenarios**:

1. **Given** arbitrary patterns and the default pattern, **When** left identity is tested (`empty.combine(x) == x`), **Then** the law holds for all test cases
2. **Given** arbitrary patterns and the default pattern, **When** right identity is tested (`x.combine(empty) == x`), **Then** the law holds for all test cases
3. **Given** patterns of varying depths and element counts, **When** identity laws are tested, **Then** the laws hold for all structures
4. **Given** patterns with different value types (String, Vec, unit), **When** identity laws are tested, **Then** the laws hold for all value types that implement `Default`

---

### User Story 3 - Use with Iterator Methods (Priority: P2)

Developers need to use patterns with Rust's standard iterator methods like `fold` in a natural way, using the default pattern as the initial value. This enables idiomatic Rust code for accumulating patterns from collections.

**Why this priority**: The primary practical benefit of having an identity element is enabling clean, idiomatic usage with iterator methods. This demonstrates the real-world utility of the feature.

**Independent Test**: Can be tested by using `fold` with `Pattern::default()` as the initial value and verifying correct accumulation of patterns.

**Acceptance Scenarios**:

1. **Given** a collection of patterns, **When** folded using `Pattern::default()` as initial value, **Then** the result equals combining all patterns in sequence
2. **Given** an empty collection of patterns, **When** folded with `Pattern::default()`, **Then** the result is the default pattern
3. **Given** a single-element collection, **When** folded with `Pattern::default()`, **Then** the result equals that single pattern
4. **Given** patterns being accumulated incrementally, **When** starting from `Pattern::default()`, **Then** the accumulation behaves identically to sequential combination

---

### Edge Cases

**Default Pattern Creation**:
- What is the default pattern for `Pattern<String>`? (Pattern with empty string value and no elements)
- What is the default pattern for `Pattern<Vec<T>>`? (Pattern with empty vector value and no elements)
- What is the default pattern for `Pattern<()>`? (Pattern with unit value and no elements)
- Can default patterns be created for types without `Default`? (No, compile-time error via trait bounds)

**Identity Laws**:
- Do identity laws hold for atomic patterns? (Must hold for all patterns)
- Do identity laws hold for deeply nested patterns? (Must hold for all patterns)
- Do identity laws hold when the value type has non-trivial `Default`? (Must hold if value's combination respects identity)

**Integration**:
- Does `Default` implementation work with standard library functions like `mem::take`? (Yes, standard behavior)
- Can `Pattern::default()` be used as a sentinel or placeholder value? (Yes, but verify identity laws still apply)

## Requirements *(mandatory)*

### Functional Requirements

**Default Implementation**:
- **FR-001**: Pattern type MUST implement `std::default::Default` trait for value types that implement `Default`
- **FR-002**: `Pattern::default()` MUST create a pattern with the default value and empty elements list
- **FR-003**: The default pattern MUST be a valid, well-formed pattern
- **FR-004**: Default implementation MUST be available for common value types (String, Vec<T>, (), integers, etc.)

**Identity Element Behavior**:
- **FR-005**: The default pattern MUST satisfy left identity: `Pattern::default().combine(x)` equals `x` for all patterns `x`
- **FR-006**: The default pattern MUST satisfy right identity: `x.combine(Pattern::default())` equals `x` for all patterns `x`
- **FR-007**: Identity laws MUST hold for atomic patterns (no elements)
- **FR-008**: Identity laws MUST hold for patterns with elements
- **FR-009**: Identity laws MUST hold for nested/recursive pattern structures

**Documentation & Testing**:
- **FR-010**: Monoid laws MUST be documented in the `Default` implementation's doc comments
- **FR-011**: Property-based tests MUST verify left identity law for randomly generated patterns
- **FR-012**: Property-based tests MUST verify right identity law for randomly generated patterns
- **FR-013**: Unit tests MUST demonstrate usage with iterator methods (`fold`, etc.)
- **FR-014**: Documentation MUST explain the relationship between `Default` and monoid identity
- **FR-015**: Documentation MUST include examples of using default patterns with combination

**Integration**:
- **FR-016**: Default patterns MUST work correctly with existing pattern operations (map, fold, traverse)
- **FR-017**: Default implementation MUST follow Rust's standard library conventions
- **FR-018**: Tests MUST verify behavioral equivalence with gram-hs monoid identity semantics

### Key Entities

- **Pattern<V>**: The recursive pattern structure with value of type `V` and elements
- **Default Trait**: Rust's standard library trait for types with a default/zero value
- **Identity Element**: The default pattern that acts as a neutral element under combination
- **Monoid Laws**: Mathematical properties that the identity element must satisfy (left identity, right identity)
- **Combinable Trait**: The existing trait that provides the combination operation (from feature 013)

## Success Criteria *(mandatory)*

### Measurable Outcomes

**Core Functionality**:
- **SC-001**: Developers can create a default pattern using `Pattern::default()` for any value type that implements `Default`
- **SC-002**: Property-based tests verify left identity law for 10,000+ randomly generated patterns with 100% success rate
- **SC-003**: Property-based tests verify right identity law for 10,000+ randomly generated patterns with 100% success rate
- **SC-004**: Unit tests achieve 100% code coverage for default implementation including all edge cases

**Behavioral Equivalence**:
- **SC-005**: Default pattern behavior matches the gram-hs Haskell reference implementation's `mempty` semantics
- **SC-006**: Identity laws hold for all value types that implement both `Default` and `Combinable` (verified through testing)

**Integration & Usability**:
- **SC-007**: Developers can use `Pattern::default()` as the initial value in `fold` operations without manual intervention
- **SC-008**: Code examples in documentation demonstrate at least 3 practical use cases for the default pattern
- **SC-009**: API documentation clearly explains the monoid laws and their significance

**Quality**:
- **SC-010**: All tests pass on stable Rust without warnings
- **SC-011**: Documentation includes mathematical notation and plain English explanations of monoid laws
- **SC-012**: Feature integrates seamlessly with existing pattern operations without breaking changes

## Assumptions

**Implementation Approach**:
- Using Rust's standard `Default` trait is more idiomatic than creating a custom `Monoid` trait
- The Rust ecosystem doesn't use algebraic typeclass abstractions, so `Default` is the appropriate choice
- Documentation and tests can adequately convey monoid laws without encoding them in traits
- The default pattern is defined as having the default value and empty elements (no special constructor needed)

**Value Type Requirements**:
- The value type `V` must implement both `Default` and `Combinable` for patterns to form a complete monoid
- Common types like `String`, `Vec<T>`, and `()` already implement `Default` with appropriate semantics
- The default value for the value type serves as an identity for the value type's combination operation

**Monoid Properties**:
- The combination operation from feature 013 is associative (already verified)
- The default pattern as defined (default value, empty elements) satisfies identity laws
- Identity laws automatically hold if the value type's default is an identity for its combination operation
- Element concatenation with empty list is always an identity operation

**Integration**:
- No breaking changes to existing code that uses patterns
- Default implementation is purely additive and doesn't affect existing functionality
- Tests for identity laws can reuse the pattern generation infrastructure from feature 013

## Dependencies

- **Pattern data structure** (features 004-006): Core Pattern<V> type must be implemented
- **Combinable trait** (feature 013): Combination operation must be available for identity laws to be meaningful
- **Test infrastructure** (feature 003): Property-based testing framework (proptest) must be available
- **Existing pattern operations**: Integration with map, fold, and other operations

## Out of Scope

**Not Included in This Feature**:
- Custom `Monoid` trait (non-idiomatic in Rust; use standard `Default` instead)
- Multiple identity elements or custom identity strategies (single standard identity via `Default`)
- Specialized identity implementations for specific value types beyond `Default`
- Compile-time enforcement of monoid laws via traits (laws documented and tested, not encoded)
- Alternative identity elements for different combination strategies (future work if needed)
- Identity-aware optimization of combination operations (future performance enhancement)
- Identity element for pattern types where value doesn't implement `Default` (use explicit initial values)

**Future Enhancements** (separate features):
- Optimized combination when one operand is the default pattern (structural sharing)
- Builder pattern that uses default as starting point with method chaining
- Specialized default patterns for graph or property graph interpretations

## Notes

**Why Default Instead of Custom Monoid Trait**:

Rust's ecosystem doesn't follow Haskell's typeclass-based approach to abstract algebra. Creating a custom `Monoid` trait would be:
1. **Non-idiomatic**: Very few Rust libraries define or use Semigroup/Monoid traits
2. **Low adoption**: Users wouldn't expect or look for such abstractions
3. **Redundant**: `Default` trait already serves the practical purpose of providing a zero/identity value
4. **Ecosystem mismatch**: Standard library and popular crates use `Default`, not `Monoid`

Instead, this feature:
- Uses the standard `Default` trait that all Rust developers understand
- Documents monoid laws clearly in comments and documentation
- Verifies laws through comprehensive property-based testing
- Provides practical benefits (iterator methods, clean APIs) without custom abstractions

**Relationship to Iterator Methods**:

The primary practical benefit is enabling clean code with iterators:

```rust
// With Default implementation:
let result = patterns.into_iter()
    .fold(Pattern::default(), |acc, p| acc.combine(p));

// Equivalent to reduce but handles empty collections:
let result = patterns.into_iter()
    .reduce(|acc, p| acc.combine(p))
    .unwrap_or_else(Pattern::default);
```

This is the idiomatic Rust way to handle accumulation with an identity element.

