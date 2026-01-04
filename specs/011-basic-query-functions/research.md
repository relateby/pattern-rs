# Implementation Research: Pattern Query Operations

**Date**: 2025-01-04  
**Researcher**: AI Agent  
**Phase**: 0 - Research & Analysis

## Overview

This document consolidates research findings for implementing the three missing predicate/search functions from the gram-hs reference implementation.

## Reference Implementation Analysis

### Source Location

**Haskell Implementation**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` (lines 945-1028)  
**Test Suite**: `../gram-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs` (lines 4023-4238)  
**Property Tests**: `../gram-hs/libs/pattern/tests/Spec/Pattern/Properties.hs` (lines 1210-1252)

### Function Specifications

#### 1. `anyValue` Function

**Haskell Signature**: `anyValue :: (v -> Bool) -> Pattern v -> Bool`

**Implementation** (lines 958-959):
```haskell
anyValue :: (v -> Bool) -> Pattern v -> Bool
anyValue p = foldr (\v acc -> p v || acc) False
```

**Semantics**:
- Uses `foldr` over pattern values with logical OR (`||`)
- Initial accumulator: `False`
- Short-circuits on first `True` (due to Haskell's lazy evaluation and `||` semantics)
- Returns `True` if at least one value satisfies the predicate
- Returns `False` for empty patterns (no values to test)
- Time complexity: O(n) worst case, O(1) to O(n) average (depends on match position)

**Test Coverage** (from CoreSpec.hs):
- T001: Atomic pattern with matching/non-matching values
- T002: Nested pattern with matching values
- T003: Pattern with no matching values  
- T008: Deeply nested patterns
- T063: Works correctly with mapped patterns
- T064: Consistent with `any` over `toList` results
- T066: Large patterns (1000+ elements)

**Property Tests**:
- T009: `anyValue p = not (allValues (not . p))` (complementary relationship)
- T010: `anyValue (const True)` always returns `True`

**Rust Translation Strategy**:
- Use existing `fold()` method with closure
- Return type: `bool`
- Signature: `pub fn any_value<F>(&self, predicate: F) -> bool where F: Fn(&V) -> bool`
- Implementation: `self.fold(false, |acc, v| acc || predicate(v))`
- Note: Rust's `||` short-circuits like Haskell's

#### 2. `allValues` Function

**Haskell Signature**: `allValues :: (v -> Bool) -> Pattern v -> Bool`

**Implementation** (lines 974-975):
```haskell
allValues :: (v -> Bool) -> Pattern v -> Bool
allValues p = foldr (\v acc -> p v && acc) True
```

**Semantics**:
- Uses `foldr` over pattern values with logical AND (`&&`)
- Initial accumulator: `True`
- Short-circuits on first `False` (due to Haskell's lazy evaluation and `&&` semantics)
- Returns `True` if all values satisfy the predicate
- Returns `True` for empty patterns (vacuous truth)
- Time complexity: O(n) worst case, O(1) to O(n) average (depends on first failure position)

**Test Coverage**:
- T004: Atomic pattern with all/not all values matching
- T007: Empty pattern (vacuous truth - returns `True`)
- T008: Deeply nested patterns
- T063: Works correctly with mapped patterns
- T064: Consistent with `all` over `toList` results
- T066: Large patterns

**Property Tests**:
- T009: `allValues p = not (anyValue (not . p))` (complementary relationship)
- T011: `allValues (const False)` returns `False` for non-empty patterns

**Rust Translation Strategy**:
- Use existing `fold()` method with closure
- Return type: `bool`
- Signature: `pub fn all_values<F>(&self, predicate: F) -> bool where F: Fn(&V) -> bool`
- Implementation: `self.fold(true, |acc, v| acc && predicate(v))`
- Note: Rust's `&&` short-circuits like Haskell's

#### 3. `filterPatterns` Function

**Haskell Signature**: `filterPatterns :: (Pattern v -> Bool) -> Pattern v -> [Pattern v]`

**Implementation** (lines 991-993):
```haskell
filterPatterns :: (Pattern v -> Bool) -> Pattern v -> [Pattern v]
filterPatterns p pat@(Pattern _ es) =
  (if p pat then [pat] else []) ++ concatMap (filterPatterns p) es
```

**Semantics**:
- Recursively traverses pattern structure
- Checks predicate on current pattern (root)
- If predicate matches, includes current pattern in results
- Recursively applies to all element patterns
- Returns list of all matching patterns in pre-order traversal
- Time complexity: O(n) where n is total number of nodes

**Test Coverage**:
- T019: Predicate matching some subpatterns
- T020: Predicate matching root pattern
- T021: Predicate matching no subpatterns
- T025: Deeply nested patterns (finds leaf)
- T026: Single-node pattern
- T027: Complex structural predicates

**Property Tests**:
- T029: `filterPatterns (const True)` returns all subpatterns
- T030: `filterPatterns (const False)` returns empty list
- T031: Relationship with `findPattern` (first element of filter results)

**Rust Translation Strategy**:
- Cannot use fold (needs access to entire Pattern structure, not just values)
- Needs custom recursive implementation
- Return type: `Vec<&Pattern<V>>` (references to avoid cloning)
- Signature: `pub fn filter<F>(&self, predicate: F) -> Vec<&Pattern<V>> where F: Fn(&Pattern<V>) -> bool`
- Implementation:
  ```rust
  pub fn filter<F>(&self, predicate: F) -> Vec<&Pattern<V>>
  where
      F: Fn(&Pattern<V>) -> bool,
  {
      let mut result = Vec::new();
      if predicate(self) {
          result.push(self);
      }
      for element in &self.elements {
          result.extend(element.filter(&predicate));
      }
      result
  }
  ```

### Additional Functions Found (Not in Spec)

**`findPattern`** (lines 1009-1015): Returns first matching pattern (returns `Option<&Pattern<V>>`)  
**`findAllPatterns`** (lines 1027-1028): Alias for `filterPatterns`

**Decision**: Not implementing these in this feature. `findPattern` could be added in a future feature if needed. `findAllPatterns` is just an alias and not necessary in Rust.

## Performance Considerations

### Short-Circuit Evaluation

**Haskell Approach**: Relies on lazy evaluation - `foldr` with `||` or `&&` naturally short-circuits

**Rust Approach**: 
- Rust's `||` and `&&` operators short-circuit when used in boolean expressions
- Using `fold(initial, |acc, v| acc || predicate(v))` will short-circuit in Rust
- Verified: Rust evaluates left-to-right and stops as soon as result is determined

**Verification Strategy**: 
- Create performance benchmarks comparing early vs late matches
- Verify through instrumentation (counter in predicate) that evaluation stops early

### Memory Considerations

**`filter` Function**:
- Haskell version creates new list with references (lazy evaluation)
- Rust version returns `Vec<&Pattern<V>>` (references, not clones)
- Memory efficient: O(m) where m is number of matches, not O(n)

## Testing Strategy

### Unit Tests (Port from Haskell)

**anyValue Tests**:
1. Atomic pattern with matching value
2. Atomic pattern with non-matching value
3. Nested pattern with matching values at different levels
4. Pattern with no matching values
5. Empty pattern (should return false)

**allValues Tests**:
1. Atomic pattern where all values match
2. Atomic pattern where not all values match
3. Empty pattern (should return true - vacuous truth)
4. Nested pattern with all values matching
5. Nested pattern with one value failing

**filter Tests**:
1. Predicate matching atomic patterns only
2. Predicate matching root pattern
3. Predicate matching no patterns
4. Complex structural predicates (e.g., `length(elements) > 0`)
5. Predicates combining structural and value properties

### Property Tests

**Complementarity Properties**:
- `anyValue(p) == !allValues(!p)` for all predicates p
- `allValues(p) == !anyValue(!p)` for all predicates p

**Identity Properties**:
- `anyValue(const true)` always returns true
- `allValues(const true)` always returns true  
- `anyValue(const false)` always returns false
- `allValues(const false)` returns false for non-empty patterns

**Filter Properties**:
- `filter(const true)` returns all subpatterns
- `filter(const false)` returns empty vec
- `filter(predicate).len() <= size()` (matches are subset of all nodes)

### Cross-Implementation Tests

Use gram-hs CLI to generate expected outputs:
1. Create test patterns in Haskell
2. Apply predicates and capture results
3. Compare with Rust implementation results

## API Design Decisions

### Naming Conventions

| Haskell | Rust | Rationale |
|---------|------|-----------|
| `anyValue` | `any_value` | Rust snake_case convention |
| `allValues` | `all_values` | Rust snake_case convention |
| `filterPatterns` | `filter` | Simpler name, context is clear from Pattern type |

### Predicate Types

**Decision**: Use generic `F: Fn(&V) -> bool` for value predicates, `F: Fn(&Pattern<V>) -> bool` for pattern predicates

**Rationale**:
- Idiomatic Rust (closures instead of function pointers)
- Allows inline closures and closure captures
- Zero-cost abstraction (monomorphization)
- Flexible (works with function pointers, closures, method references)

### Return Types

| Function | Haskell Return | Rust Return | Rationale |
|----------|----------------|-------------|-----------|
| `any_value` | `Bool` | `bool` | Direct mapping |
| `all_values` | `Bool` | `bool` | Direct mapping |
| `filter` | `[Pattern v]` | `Vec<&Pattern<V>>` | References avoid cloning |

**Note**: Using references in `filter` return value prevents unnecessary cloning of potentially large patterns.

## Integration with Existing Code

### Leveraging Existing Infrastructure

- `any_value` and `all_values` can use existing `fold()` method (feature 009)
- No new dependencies required
- Fits naturally into existing Pattern API

### WASM Compatibility

- All functions are pure computation
- No I/O, no platform-specific code
- Compatible with WASM compilation constraints
- Will be automatically available through pattern-wasm bindings

## Documentation Requirements

### Doc Comments

Each function needs:
1. High-level description
2. Complexity analysis (time/space)
3. Examples with various pattern structures
4. Edge case behavior (empty patterns, short-circuit behavior)
5. Reference to Haskell equivalent

### Module Documentation

Update `pattern.rs` module docs to list new query operations:
- Add to "Query Functions" section
- Note: complement existing structural queries (length/size/depth/values)

## Risk Assessment

### Low Risk ✅

- **Implementation complexity**: Low (straightforward ports from well-defined Haskell)
- **API surface**: Small (3 new methods)
- **Behavioral equivalence**: High confidence (clear reference implementation)
- **Breaking changes**: None (additive only)

### Mitigation Strategies

- **Comprehensive tests**: Port all Haskell tests + add property tests
- **Performance benchmarks**: Verify short-circuit behavior
- **Cross-implementation validation**: Compare with gram-hs outputs
- **Early review**: Get feedback on signatures before full implementation

## Open Questions

✅ **Q1**: Should `filter` return owned Patterns or references?  
**A1**: References (`Vec<&Pattern<V>>`) to avoid unnecessary cloning. Matches Rust idioms.

✅ **Q2**: Should we implement `findPattern` in this feature?  
**A2**: No. Focus on the three functions specified. Can add in future feature if needed.

✅ **Q3**: How to verify short-circuit evaluation in tests?  
**A3**: Performance benchmarks + instrumentation (counter in test predicates)

✅ **Q4**: Should predicates take references or values?  
**A4**: References (`&V`, `&Pattern<V>`) - no unnecessary copies, follows Rust conventions

## Conclusion

All research complete. No NEEDS CLARIFICATION items remain. Ready to proceed to Phase 1 (Design & Contracts).

**Key Findings**:
1. Reference implementations are straightforward to port
2. Existing fold infrastructure supports two of three functions
3. `filter` needs custom recursive implementation
4. Comprehensive test suite available in gram-hs
5. All functions maintain O(n) or better complexity with short-circuit evaluation
6. No platform-specific concerns or dependency requirements

