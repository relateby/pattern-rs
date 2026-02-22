# Feature Specification: Pattern Hashing via Hash Trait

**Feature Branch**: `015-hashable-instance`  
**Created**: 2026-01-05  
**Status**: Draft  
**Input**: User description: "An idiomatic Rust port of Hashable as described in '015-hashable-instance: Hash Trait' of TODO.md"

## Context

This feature adds hashing support to Pattern types by implementing Rust's standard `std::hash::Hash` trait. This enables patterns to be used as keys in HashMap and elements in HashSet, enabling efficient pattern deduplication, caching, and set-based operations.

In Haskell (gram-hs), this is expressed as a Hashable instance that combines hashes of value and elements recursively. In Rust, the idiomatic approach is to implement the standard library's `Hash` trait with conditional trait bounds, leveraging Rust's built-in hashing infrastructure.

**Foundation**: This feature builds on:
- Pattern data structure (features 004-006)
- Eq/PartialEq traits (feature 004)
- Ord/PartialOrd traits (feature 012)

**Reference**: The authoritative implementation is in `../pattern-hs/libs/pattern/src/Pattern/Core.hs` (Haskell source code, lines 477-535).

**Implementation Guidance**: Follow idiomatic Rust patterns by implementing `std::hash::Hash` with conditional trait bounds (`impl<V: Hash> Hash for Pattern<V>`). This approach only allows hashing for patterns where the value type is hashable, which is semantically correct.

**Hash/Eq Consistency**: The implementation must satisfy:
1. **Consistency with Equality**: If `p1 == p2`, then `hash(p1) == hash(p2)` for all patterns
2. **Structure Distinguishes Hash**: Different structures with same values produce different hashes

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Pattern Deduplication (Priority: P1)

Developers need to efficiently remove duplicate patterns from collections using HashSet. This enables processing large pattern sets without O(n²) duplicate checking.

**Why this priority**: Deduplication is a fundamental operation when working with pattern collections, essential for parsers, analyzers, and code generators.

**Independent Test**: Can be fully tested by adding patterns to a HashSet and verifying duplicates are automatically removed.

**Acceptance Scenarios**:

1. **Given** a collection of patterns with some duplicates, **When** added to HashSet, **Then** only unique patterns remain
2. **Given** two equal patterns, **When** hashed, **Then** they produce the same hash value
3. **Given** two different patterns, **When** hashed, **Then** they likely produce different hash values (collision rare but possible)
4. **Given** patterns with identical values but different structures, **When** hashed, **Then** they produce different hash values

---

### User Story 2 - Pattern Caching (Priority: P1)

Developers need to cache expensive pattern computations using HashMap with patterns as keys. This enables memoization for performance optimization.

**Why this priority**: Caching is critical for performance in parsers, compilers, and analyzers that repeatedly process the same patterns.

**Independent Test**: Can be tested by using a HashMap<Pattern, Result> to cache computations and verifying lookups work correctly.

**Acceptance Scenarios**:

1. **Given** a HashMap with pattern keys, **When** a pattern is looked up, **Then** the corresponding value is returned
2. **Given** cached results for patterns, **When** the same pattern is processed again, **Then** the cached result is used
3. **Given** patterns as keys, **When** equal patterns are used, **Then** they access the same cache entry
4. **Given** hash-based caching, **When** many patterns are cached, **Then** lookups remain O(1) average case

---

### User Story 3 - Set-Based Operations (Priority: P2)

Developers need to perform set-theoretic operations (intersection, difference, union) on pattern collections efficiently using HashSet.

**Why this priority**: Set operations are useful for pattern analysis, but less critical than basic deduplication and caching.

**Independent Test**: Can be tested by creating multiple HashSets and performing standard set operations, verifying correctness.

**Acceptance Scenarios**:

1. **Given** two HashSets of patterns, **When** intersection is computed, **Then** result contains only common patterns
2. **Given** two HashSets of patterns, **When** difference is computed, **Then** result contains patterns in first but not second
3. **Given** two HashSets of patterns, **When** union is computed, **Then** result contains all unique patterns
4. **Given** a pattern and a HashSet, **When** checking membership, **Then** lookup is O(1) average case

---

### Edge Cases

**Hash Consistency**:
- Do equal atomic patterns hash the same? (Must hash the same)
- Do equal nested patterns hash the same? (Must hash the same)  
- Do patterns with same values but different structures hash differently? (Yes, structure distinguishes)
- Can patterns be hashed when value type doesn't implement Hash? (No, compile-time error via trait bounds)

**Type Constraints**:
- Can Pattern<String> be hashed? (Yes, String implements Hash)
- Can Pattern<Subject> be hashed? (No, Subject contains f64 which doesn't implement Hash - correct behavior)
- Can Pattern<f64> be hashed? (No, f64 doesn't implement Hash - correct behavior)
- Do trait bounds prevent nonsensical hashing? (Yes, type system enforces correctness)

**Integration**:
- Does Hash work with HashMap/HashSet? (Yes, standard usage)
- Is hash computation efficient for deeply nested patterns? (O(n) in pattern size, acceptable)
- Does hashing respect structural equality? (Yes, matches Eq implementation)

## Requirements *(mandatory)*

### Functional Requirements

**Hash Implementation**:
- **FR-001**: Pattern type MUST implement `std::hash::Hash` trait for value types that implement Hash
- **FR-002**: Hash implementation MUST hash both value and elements recursively
- **FR-003**: Hash MUST be consistent with Eq: if `p1 == p2` then `hash(p1) == hash(p2)`
- **FR-004**: Hash implementation MUST leverage Vec's built-in Hash for elements
- **FR-005**: Hash MUST distinguish different structures with same value content

**Symbol Enhancement**:
- **FR-006**: Symbol type MUST implement Hash (add to derive macro)
- **FR-007**: Symbol Hash MUST be consistent with its Eq implementation

**Type Safety**:
- **FR-008**: Hash implementation MUST use conditional trait bounds (`V: Hash`)
- **FR-009**: Pattern<Subject> MUST NOT compile with Hash operations (Subject contains f64)
- **FR-010**: Compilation errors for non-hashable types MUST be clear and helpful

**Documentation & Testing**:
- **FR-011**: Hash trait documentation MUST explain usage with HashMap/HashSet
- **FR-012**: Property-based tests MUST verify hash/eq consistency for randomly generated patterns
- **FR-013**: Unit tests MUST demonstrate HashMap and HashSet usage
- **FR-014**: Tests MUST verify structure distinguishes hash values
- **FR-015**: Documentation MUST note that Pattern<Subject> is not hashable

**Integration**:
- **FR-016**: Hash implementation MUST work with standard library HashMap
- **FR-017**: Hash implementation MUST work with standard library HashSet
- **FR-018**: Tests MUST verify behavioral equivalence with gram-hs Hashable instance

### Key Entities

- **Pattern<V>**: The recursive pattern structure with value of type `V` and elements
- **Hash Trait**: Rust's standard library trait for types that can be hashed
- **Hasher**: The hasher state that accumulates hash computations
- **HashMap<Pattern<V>, T>**: Hash-based map using patterns as keys
- **HashSet<Pattern<V>>**: Hash-based set for pattern deduplication
- **Symbol**: The symbol type that needs Hash added to its derives

## Success Criteria *(mandatory)*

### Measurable Outcomes

**Core Functionality**:
- **SC-001**: Developers can use Pattern<String> as HashMap keys
- **SC-002**: Developers can add Pattern<String> to HashSet for deduplication
- **SC-003**: Property-based tests verify hash/eq consistency for 10,000+ patterns with 100% success rate
- **SC-004**: Unit tests achieve 100% code coverage for hash implementation

**Behavioral Equivalence**:
- **SC-005**: Hash behavior matches the gram-hs Haskell reference implementation's Hashable semantics
- **SC-006**: Structure-preserving hashing verified through testing (different structures → different hashes)

**Integration & Usability**:
- **SC-007**: Developers can deduplicate patterns using HashSet with O(n) complexity
- **SC-008**: Developers can cache pattern results using HashMap with O(1) lookup
- **SC-009**: Documentation includes at least 3 practical use cases for hashing patterns
- **SC-010**: API documentation clearly explains hash/eq consistency requirements

**Quality**:
- **SC-011**: All tests pass on stable Rust without warnings
- **SC-012**: Hash implementation compiles for WASM target
- **SC-013**: Feature integrates seamlessly with existing Eq/Ord implementations
- **SC-014**: Type errors for non-hashable patterns provide clear guidance

## Assumptions

**Implementation Approach**:
- Using Rust's standard `Hash` trait is more idiomatic than creating a custom hashable trait
- Conditional trait bounds (`V: Hash`) correctly express type constraints
- Vec<T>'s Hash implementation is structurally sound and efficient
- The default hasher (SipHash or similar) provides good distribution for patterns

**Type Constraints**:
- Some types (like Subject with f64) cannot and should not implement Hash
- This limitation is correct and expected behavior, not a bug
- Developers will use Pattern<String> or Pattern<Symbol> for hashable patterns
- The type system prevents incorrect Hash usage at compile time

**Hash Properties**:
- Hash computation is O(n) in pattern size (acceptable for practical use)
- Hash collisions are rare enough that HashMap/HashSet performance is good
- Hashing both value and elements is sufficient for structural distinction
- The hasher implementation handles recursive structures efficiently

**Integration**:
- No breaking changes to existing code that uses patterns
- Hash implementation is purely additive and doesn't affect existing functionality
- Works naturally with existing Eq and Ord implementations
- Symbol type can have Hash added without breaking changes

## Dependencies

- **Pattern data structure** (features 004-006): Core Pattern<V> type must be implemented
- **Eq/PartialEq** (feature 004): Equality is prerequisite for Hash consistency
- **Test infrastructure** (feature 003): Property-based testing framework (proptest) must be available
- **Existing trait implementations**: Ord, Clone, Debug already implemented

## Out of Scope

**Not Included in This Feature**:
- Custom hash algorithms (use Rust's standard hasher)
- Hash implementation for Subject type (contains f64, semantically incorrect)
- Optimized hashing for specific pattern structures (future performance work)
- Hash-aware pattern operations (future enhancement if needed)
- Custom Hasher implementations (use standard library hashers)
- Hash caching or memoization (patterns are immutable, no benefit)

**Future Enhancements** (separate features):
- Optimized HashMap/HashSet for patterns (if profiling shows need)
- Specialized hashing strategies for specific value types
- Hash-based pattern indexing structures

## Notes

**Why Hash for Pattern but not Subject**:

Subject contains Value enum which has VDecimal(f64) variant. Since f64 doesn't implement Hash (or Eq) due to NaN semantics, Subject cannot implement Hash. This is **correct behavior** - not all types should be hashable.

Pattern<String>, Pattern<Symbol>, and other patterns with hashable value types can still use Hash effectively.

**Relationship to HashMap/HashSet**:

The primary benefit is enabling Rust's standard hash-based collections:

```rust
// Deduplication
let unique: HashSet<Pattern<String>> = patterns.into_iter().collect();

// Caching
let mut cache: HashMap<Pattern<String>, Result> = HashMap::new();
if let Some(result) = cache.get(&pattern) {
    return result.clone();
}
```

This is idiomatic Rust for efficient collection operations.

**Performance Characteristics**:

- Hash computation: O(n) where n is total nodes in pattern
- HashMap lookup: O(1) average case
- HashSet deduplication: O(n) for n patterns vs O(n²) naive
- Deep nesting: Acceptable performance (hashing is typically fast relative to other operations)
