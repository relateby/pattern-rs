# Research: Pattern Construction & Access

**Feature**: 005-basic-pattern-type  
**Date**: 2025-01-27

## Research Tasks

### 1. Pattern Construction Functions from gram-hs

**Task**: Understand construction function signatures and patterns from gram-hs reference implementation

**Findings**:
- **Decision**: Construction functions from gram-hs:
  1. `point :: v -> Pattern v` - Creates atomic pattern (pattern with no elements) - special case constructor
  2. `pattern :: v -> [Pattern v] -> Pattern v` - Creates pattern with explicit elements (primary constructor)
  3. `fromList :: v -> [v] -> Pattern v` - Creates pattern from list of values (converts values to atomic patterns)
- **Rationale**: 
  - `point` provides convenience for atomic patterns (common case)
  - `pattern` provides primary constructor with explicit elements
  - `fromList` provides convenience for creating patterns from value lists
- **Alternatives considered**: 
  - Only full constructor - rejected (lacks convenience for atomic patterns)
  - Builder pattern - deferred (not in gram-hs, may add later if needed)
- **Source**: Verified from `../gram-hs/libs/pattern/src/Pattern/Core.hs` (lines 862-889)
- **Implementation Notes**:
  - Functions are generic over `V`
  - No validation needed (Pattern structure itself is valid)
  - Should be efficient (no unnecessary cloning)

**Rust Translation**:
```rust
impl<V> Pattern<V> {
    // Equivalent to Haskell's `point :: v -> Pattern v`
    pub fn point(value: V) -> Self {
        Pattern { value, elements: vec![] }
    }
    
    // Equivalent to Haskell's `pattern :: v -> [Pattern v] -> Pattern v`
    pub fn pattern(value: V, elements: Vec<Pattern<V>>) -> Self {
        Pattern { value, elements }
    }
    
    // Equivalent to Haskell's `fromList :: v -> [v] -> Pattern v`
    pub fn from_list(value: V, values: Vec<V>) -> Self {
        Pattern {
            value,
            elements: values.into_iter().map(Pattern::point).collect(),
        }
    }
}
```

**Note**: Verified from actual gram-hs implementation. Function names follow Rust conventions (snake_case).

### 2. Pattern Accessor Functions from gram-hs

**Task**: Understand accessor function patterns (methods vs functions, return types)

**Findings**:
- **Decision**: In gram-hs, accessors are field accessors from the data type definition:
  1. `value :: Pattern v -> v` - Field accessor for value (from data type)
  2. `elements :: Pattern v -> [Pattern v]` - Field accessor for elements (from data type)
- **Rationale**: 
  - In Haskell, record fields automatically become accessor functions
  - In Rust, we should provide methods for accessing fields
  - Returning references avoids unnecessary cloning
- **Alternatives considered**: 
  - Standalone functions - rejected (methods are more idiomatic in Rust)
  - Returning owned values - rejected (inefficient, unnecessary)
- **Source**: Verified from `../gram-hs/libs/pattern/src/Pattern/Core.hs` (lines 267-270, data type definition)
- **Implementation Notes**:
  - Accessors should be simple field access (no computation)
  - Type information is preserved through generics
  - Should be O(1) operations
  - In Rust, fields are already public, but methods provide consistent API

**Rust Translation**:
```rust
impl<V> Pattern<V> {
    // Equivalent to Haskell's `value :: Pattern v -> v` (field accessor)
    pub fn value(&self) -> &V {
        &self.value
    }
    
    // Equivalent to Haskell's `elements :: Pattern v -> [Pattern v]` (field accessor)
    pub fn elements(&self) -> &[Pattern<V>] {
        &self.elements
    }
}
```

**Note**: Verified from actual gram-hs implementation. In Haskell, these are record field accessors; in Rust, we provide methods for consistency with construction functions.

### 3. Pattern Inspection Utilities from gram-hs

**Task**: Understand inspection utility functions (atomic check, depth, element count)

**Findings**:
- **Decision**: Inspection utilities from gram-hs:
  1. `length :: Pattern v -> Int` - Returns number of direct elements (O(1))
  2. `size :: Pattern v -> Int` - Returns total number of nodes in pattern structure (O(n))
  3. `depth :: Pattern v -> Int` - Returns maximum nesting depth (O(n))
- **Rationale**: 
  - `length` provides direct element count (simple and fast)
  - `size` provides total node count (useful for understanding structure size)
  - `depth` provides nesting depth information (important for structural analysis)
- **Alternatives considered**: 
  - `is_atomic` helper - can be derived as `length p == 0` but not in gram-hs
  - Additional structural analysis - deferred (can add later if needed)
- **Source**: Verified from `../gram-hs/libs/pattern/src/Pattern/Core.hs` (lines 904-943)
- **Implementation Notes**:
  - `length` is O(1) - just returns length of elements list
  - `size` is O(n) - recursively counts all nodes
  - `depth` is O(n) - recursively calculates maximum depth
  - Atomic patterns have depth 0 (corrected from previous inconsistency in gram-hs)
  - Depth calculation must handle recursion safely (avoid stack overflow)

**Rust Translation**:
```rust
impl<V> Pattern<V> {
    // Equivalent to Haskell's `length :: Pattern v -> Int`
    pub fn length(&self) -> usize {
        self.elements.len()
    }
    
    // Equivalent to Haskell's `size :: Pattern v -> Int`
    pub fn size(&self) -> usize {
        1 + self.elements.iter().map(|e| e.size()).sum::<usize>()
    }
    
    // Equivalent to Haskell's `depth :: Pattern v -> Int`
    pub fn depth(&self) -> usize {
        if self.elements.is_empty() {
            0  // Atomic patterns have depth 0
        } else {
            1 + self.elements.iter().map(|e| e.depth()).max().unwrap_or(0)
        }
    }
    
    // Convenience helper (not in gram-hs but useful)
    pub fn is_atomic(&self) -> bool {
        self.elements.is_empty()
    }
}
```

**Note**: Verified from actual gram-hs implementation. Atomic patterns have depth 0 (corrected from previous inconsistency).

### 4. Rust Implementation Patterns (Methods vs Associated Functions)

**Task**: Determine idiomatic Rust patterns for construction, access, and inspection

**Findings**:
- **Decision**: Use methods for accessors and inspection, associated functions for construction
- **Rationale**: 
  - Methods (`self` parameter) for operations on existing instances (access, inspection)
  - Associated functions (no `self`) for creating new instances (construction)
  - This follows Rust conventions (e.g., `Vec::new()`, `vec.len()`)
- **Alternatives considered**: 
  - All as methods - rejected (construction doesn't have `self`)
  - All as standalone functions - rejected (less idiomatic)
- **Implementation Notes**:
  - Construction: Associated functions (`Pattern::new()`, `Pattern::atomic()`)
  - Access: Methods (`pattern.value()`, `pattern.elements()`)
  - Inspection: Methods (`pattern.is_atomic()`, `pattern.depth()`)

### 5. Behavioral Equivalence Testing Strategy

**Task**: Determine how to verify behavioral equivalence with gram-hs

**Findings**:
- **Decision**: Use existing test utilities in `crates/pattern-core/src/test_utils/` for equivalence checking
- **Rationale**: 
  - Test infrastructure already exists (feature 003)
  - Can extract test cases from gram-hs using test synchronization infrastructure
  - Equivalence checking utilities are ready for use
- **Alternatives considered**: 
  - Manual comparison - rejected (too error-prone)
  - External testing tools - rejected (use existing infrastructure)
- **Implementation Notes**:
  - Port test cases from `../gram-hs/libs/pattern/tests/`
  - Test construction functions with various inputs
  - Test accessors return correct values
  - Test inspection utilities with various pattern structures
  - Use `gram-hs` CLI tool for test case generation if needed (per `docs/gram-hs-cli-testing-guide.md`)

### 6. Performance Considerations

**Task**: Ensure functions meet performance requirements

**Findings**:
- **Decision**: 
  - Construction: O(1) for atomic, O(n) for nested (where n is element count)
  - Accessors: O(1) for both value and elements
  - Inspection: O(1) for is_atomic and element_count, O(n) for depth (where n is total nodes)
- **Rationale**: 
  - Construction and access should be efficient
  - Depth calculation requires traversal but should handle 100+ levels safely
  - Use iterative approach for depth to avoid stack overflow
- **Alternatives considered**: 
  - Caching depth - deferred (optimization, not needed initially)
  - Lazy depth calculation - deferred (complexity not justified)
- **Implementation Notes**:
  - Depth calculation should use iterative approach or ensure tail recursion
  - Test with patterns up to 100 levels deep
  - Test with patterns with 10,000+ elements

## Resolved Clarifications

All NEEDS CLARIFICATION items from Technical Context have been resolved by verifying against actual gram-hs implementation:
- ✅ Construction function signatures: `point()`, `pattern()`, and `from_list()` matching gram-hs `point`, `pattern`, and `fromList`
- ✅ Accessor implementation: Methods `value()` and `elements()` matching gram-hs field accessors
- ✅ Inspection utilities: `length()`, `size()`, and `depth()` matching gram-hs functions
- ✅ Input validation: No validation needed (Pattern structure is always valid)
- ✅ Special case constructor: `point()` for atomic patterns, primary constructor `pattern()` for patterns with elements, `from_list()` for value lists

**Note**: All decisions have been verified against the actual gram-hs implementation in `../gram-hs/libs/pattern/src/Pattern/Core.hs`. Function signatures match gram-hs with Rust naming conventions (snake_case).

## Open Questions (Deferred to Implementation)

1. **Mutable Accessors**: Should we provide `value_mut()` and `elements_mut()`? (Decision: Add if needed, start with immutable)
2. **Additional Inspection Utilities**: Are there other structural analysis functions in gram-hs? (Decision: Verify during implementation, add if needed)
3. **Builder Pattern**: Is a builder pattern useful for complex nested construction? (Decision: Defer, start with simple constructors)
4. **Error Handling**: Do construction functions need to handle errors? (Decision: No, Pattern structure is always valid)

## References

- **Primary Source (Authoritative)**: gram-hs Implementation: `../gram-hs/libs/`
  - Pattern Construction/Access/Inspection: `../gram-hs/libs/pattern/src/Pattern.hs`
  - Tests: `../gram-hs/libs/pattern/tests/`
- **Secondary Source (Context Only)**: gram-hs Design Documents: `../gram-hs/specs/002-basic-pattern-type/`
  - Type Signatures: `../gram-hs/specs/002-basic-pattern-type/contracts/type-signatures.md` (may be outdated)
  - Feature Spec: `../gram-hs/specs/002-basic-pattern-type/spec.md` (for context)
- Porting Guide: `PORTING_GUIDE.md`
- Test Infrastructure: `specs/003-test-infrastructure/`
- gram-hs CLI Testing Guide: `docs/gram-hs-cli-testing-guide.md`
- Existing Pattern Implementation: `crates/pattern-core/src/pattern.rs`

