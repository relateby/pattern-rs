# Research: Predicate-Based Pattern Matching

**Feature**: 016-predicate-matching  
**Date**: 2025-01-05  
**Status**: Complete

## Research Objectives

1. Analyze gram-hs implementation of missing predicate functions (findPattern, matches, contains)
2. Design Rust idiomatic equivalents using Option, borrowed references, and Fn traits
3. Determine optimal implementation strategies for structural matching operations
4. Review existing gram-rs implementation (any_value, all_values, filter) for consistency
5. Identify performance considerations and edge case handling

## Implementation Status Analysis

### Already Implemented in gram-rs

**✅ any_value** (`crates/pattern-core/src/pattern.rs` lines 636-661)
- **Signature**: `pub fn any_value<F>(&self, predicate: F) -> bool where F: Fn(&V) -> bool`
- **Implementation**: Recursive with early termination (short-circuit on first match)
- **Traversal**: Depth-first pre-order (root first, then elements)
- **Status**: Complete with 66 passing tests in `tests/query_any_value.rs`
- **Equivalence**: Verified against gram-hs `anyValue` function

**✅ all_values** (`crates/pattern-core/src/pattern.rs` lines 738-763)
- **Signature**: `pub fn all_values<F>(&self, predicate: F) -> bool where F: Fn(&V) -> bool`
- **Implementation**: Recursive with early termination (short-circuit on first failure)
- **Traversal**: Depth-first pre-order (root first, then elements)
- **Status**: Complete with 66 passing tests in `tests/query_all_values.rs`
- **Equivalence**: Verified against gram-hs `allValues` function

**✅ filter** (`crates/pattern-core/src/pattern.rs` lines 876-902)
- **Signature**: `pub fn filter<F>(&self, predicate: F) -> Vec<&Pattern<V>> where F: Fn(&Pattern<V>) -> bool`
- **Implementation**: Recursive accumulation into Vec
- **Traversal**: Depth-first pre-order (root first, then elements)
- **Return Type**: `Vec<&Pattern<V>>` (owned collection of borrowed references)
- **Status**: Complete with 66 passing tests in `tests/query_filter.rs`
- **Equivalence**: Verified against gram-hs `filterPatterns` function
- **Note**: Returns Vec not Iterator - existing API decision, maintained for consistency

### Missing Implementations

**❌ find_first** (gram-hs: `findPattern`)
- **gram-hs Signature**: `findPattern :: (Pattern v -> Bool) -> Pattern v -> Maybe (Pattern v)`
- **Rust Signature**: `pub fn find_first<F>(&self, predicate: F) -> Option<&Pattern<V>> where F: Fn(&Pattern<V>) -> bool`
- **Need**: Return first matching subpattern using Option for no-match case
- **Traversal**: Depth-first pre-order with early termination
- **Design Decision**: NEEDS CLARIFICATION

**❌ matches** (gram-hs: `matches`)
- **gram-hs Signature**: `matches :: (Eq v) => Pattern v -> Pattern v -> Bool`
- **Rust Signature**: `pub fn matches(&self, other: &Pattern<V>) -> bool where V: PartialEq`
- **Need**: Structural equality check beyond Eq trait
- **Design Decision**: NEEDS CLARIFICATION

**❌ contains** (gram-hs: `contains`)
- **gram-hs Signature**: `contains :: (Eq v) => Pattern v -> Pattern v -> Bool`
- **Rust Signature**: `pub fn contains(&self, subpattern: &Pattern<V>) -> bool where V: PartialEq`
- **Need**: Check if pattern contains another as subpattern
- **Design Decision**: NEEDS CLARIFICATION

## Decision 1: find_first Implementation Strategy

**Chosen Approach**: Recursive depth-first pre-order traversal with early termination using Option

**Implementation Pattern**:
```rust
pub fn find_first<F>(&self, predicate: F) -> Option<&Pattern<V>>
where
    F: Fn(&Pattern<V>) -> bool,
{
    self.find_first_recursive(&predicate)
}

fn find_first_recursive<F>(&self, predicate: &F) -> Option<&Pattern<V>>
where
    F: Fn(&Pattern<V>) -> bool,
{
    // Check current pattern first (pre-order)
    if predicate(self) {
        return Some(self);
    }
    
    // Recursively search elements, return first match
    for element in &self.elements {
        if let Some(found) = element.find_first_recursive(predicate) {
            return Some(found);
        }
    }
    
    None
}
```

**Rationale**:
- Consistent with existing any_value and filter traversal order
- Early termination optimizes performance (O(1) best case, O(n) worst case)
- Option<&Pattern<V>> is idiomatic Rust for "value or absence" (not an error)
- Borrowed reference avoids unnecessary cloning
- Helper method pattern (recursive with &F) matches existing implementations
- Traversal order matches gram-hs reference implementation

**Alternatives Considered**:
- **Breadth-first traversal**: Rejected - inconsistent with existing operations, more complex, higher memory usage
- **Return Result<&Pattern<V>, NotFound>**: Rejected - no match is not an error, Option is semantically correct
- **Return Vec with max 1 element**: Rejected - less ergonomic, doesn't convey "find first" semantics

**Verification Against gram-hs**:
```haskell
-- gram-hs implementation (../gram-hs/libs/pattern/src/Pattern/Core.hs)
findPattern :: (Pattern v -> Bool) -> Pattern v -> Maybe (Pattern v)
findPattern p pat
  | p pat = Just pat
  | otherwise = foldr (\e acc -> case acc of
      Nothing -> findPattern p e
      Just _ -> acc) Nothing (elements pat)
```
- Same semantics: pre-order traversal, early termination
- Same return type: Maybe in Haskell ≡ Option in Rust
- Rust version uses explicit recursion instead of foldr for clarity

## Decision 2: matches Implementation Strategy

**Chosen Approach**: Recursive structural comparison with short-circuit evaluation

**Implementation Pattern**:
```rust
pub fn matches(&self, other: &Pattern<V>) -> bool
where
    V: PartialEq,
{
    // Check values are equal
    if self.value != other.value {
        return false;
    }
    
    // Check element counts match
    if self.elements.len() != other.elements.len() {
        return false;
    }
    
    // Recursively check elements pairwise
    for (self_elem, other_elem) in self.elements.iter().zip(other.elements.iter()) {
        if !self_elem.matches(other_elem) {
            return false;
        }
    }
    
    true
}
```

**Rationale**:
- Structural matching checks value AND element structure recursively
- Short-circuits on first mismatch (value, length, or element)
- Distinguishes patterns with same flattened values but different structures
- PartialEq bound enables value comparison without requiring full Eq implementation
- Method signature avoids function vs trait ambiguity (method is clearer than free function)
- O(min(n, m)) complexity where n, m are pattern sizes

**Alternatives Considered**:
- **Use existing Eq instance**: Rejected - spec requires distinction from exact equality, matches may have different future semantics (e.g., partial matching with wildcards)
- **Free function instead of method**: Rejected - method is more ergonomic and discoverable
- **Use Eq trait bound**: Rejected - PartialEq is sufficient and less restrictive

**Relationship to Eq Trait**:
- Pattern<V> already implements Eq when V: Eq
- `matches` provides structural matching that may diverge from Eq in future
- For now: `p1.matches(&p2) == (p1 == p2)` when both defined
- Future: matches could support wildcards, partial matching, etc.

**Verification Against gram-hs**:
```haskell
-- gram-hs implementation (../gram-hs/libs/pattern/src/Pattern/Core.hs)
matches :: (Eq v) => Pattern v -> Pattern v -> Bool
matches (Pattern v1 els1) (Pattern v2 els2) =
  v1 == v2 && length els1 == length els2 && 
  all (uncurry matches) (zip els1 els2)
```
- Same semantics: value comparison, length check, recursive element comparison
- Same short-circuit behavior: stops on first mismatch
- Rust version uses explicit loop instead of all/zip for clarity

## Decision 3: contains Implementation Strategy

**Chosen Approach**: Recursive search using matches for structural comparison

**Implementation Pattern**:
```rust
pub fn contains(&self, subpattern: &Pattern<V>) -> bool
where
    V: PartialEq,
{
    // Check if current pattern matches subpattern
    if self.matches(subpattern) {
        return true;
    }
    
    // Recursively search elements
    for element in &self.elements {
        if element.contains(subpattern) {
            return true;
        }
    }
    
    false
}
```

**Rationale**:
- Checks self-containment first (pattern contains itself)
- Uses matches for structural comparison (not just value equality)
- Recursively searches all elements
- Early termination on first match
- O(n*m) worst case where n = size of container, m = size of subpattern
- Simple and correct implementation following gram-hs semantics

**Alternatives Considered**:
- **Use Eq instead of matches**: Rejected - should use structural matching for consistency
- **Breadth-first search**: Rejected - depth-first is simpler and sufficient
- **Memoization**: Rejected - premature optimization, complexity not justified

**Verification Against gram-hs**:
```haskell
-- gram-hs implementation (../gram-hs/libs/pattern/src/Pattern/Core.hs)
contains :: (Eq v) => Pattern v -> Pattern v -> Bool
contains pat subpat =
  pat `matches` subpat || any (`contains` subpat) (elements pat)
```
- Same semantics: self-match check, recursive element search
- Same early termination behavior
- Rust version uses explicit loop instead of any for consistency

## Consistency Analysis

### Traversal Order Consistency

All pattern query operations use depth-first pre-order traversal:
- ✅ any_value: pre-order (root first, then elements)
- ✅ all_values: pre-order (root first, then elements)
- ✅ filter: pre-order (root first, then elements)
- ✅ find_first: pre-order (root first, then elements) - NEW
- ✅ fold: pre-order (root first, then elements) - existing
- ✅ map: pre-order (root first, then elements) - existing

This consistency ensures predictable behavior across all operations.

### Predicate Trait Bounds

All predicate functions use `Fn` trait (not `FnMut` or `FnOnce`):
- ✅ any_value: `F: Fn(&V) -> bool`
- ✅ all_values: `F: Fn(&V) -> bool`
- ✅ filter: `F: Fn(&Pattern<V>) -> bool`
- ✅ find_first: `F: Fn(&Pattern<V>) -> bool` - NEW

This enables predicate reuse (callable multiple times) and matches std library conventions (Iterator::filter uses Fn).

### Return Type Patterns

- `any_value`: returns `bool` (short-circuits to true)
- `all_values`: returns `bool` (short-circuits to false)
- `filter`: returns `Vec<&Pattern<V>>` (collects all matches)
- `find_first`: returns `Option<&Pattern<V>>` (short-circuits to Some) - NEW
- `matches`: returns `bool` (short-circuits to false) - NEW
- `contains`: returns `bool` (short-circuits to true) - NEW

All return types use borrowed references where applicable, avoiding unnecessary cloning.

## Performance Considerations

### Time Complexity

- **find_first**: O(k) where k is position of first match (best case O(1), worst case O(n))
- **matches**: O(min(n, m)) where n, m are pattern sizes (short-circuits on first difference)
- **contains**: O(n*m) worst case where n = container size, m = subpattern size

### Space Complexity

All operations use O(d) stack space where d = maximum nesting depth (recursion overhead).

### Performance Targets

Per spec (SC-005, SC-006):
- find_first on 1000 nodes, match in first 10: < 10ms ✅ (expected ~μs range)
- All operations on 1000 nodes, 100 depth: < 100ms ✅ (expected ~ms range)

Rust's efficient stack management and zero-cost abstractions make these targets easily achievable.

### Stack Overflow Prevention

Maximum nesting depth of 100 (per spec) requires ~100 stack frames:
- Rust default stack size: 2MB (Linux), 8MB (macOS)
- Each frame: ~100 bytes estimate
- Total: ~10KB for 100 frames
- Margin: >1000x safety factor

No risk of stack overflow for target workloads.

## Edge Case Handling

All functions handle edge cases consistently:

**Atomic patterns** (no elements):
- find_first: checks pattern value, returns Some(self) if matches
- matches: compares values, both must have length 0
- contains: checks if patterns match

**Empty elements** (pattern with empty Vec):
- find_first: checks pattern value, no elements to search
- matches: requires both to have length 0
- contains: only self-containment possible

**Deeply nested structures** (100+ levels):
- find_first: recursive traversal handles arbitrary depth
- matches: recursive comparison handles arbitrary depth
- contains: recursive search handles arbitrary depth

**No matches**:
- find_first: returns None
- matches: returns false
- contains: returns false

**Multiple matches** (find_first only):
- Returns first match in pre-order traversal
- Consistent with short-circuit semantics

## Integration with Existing Features

### Compatibility with Other Pattern Operations

New functions integrate seamlessly:

```rust
// Combine find_first with structural properties
pattern.find_first(|p| p.length() > 2 && p.depth() < 5)

// Use matches with filter
pattern.filter(|p| p.matches(&target_pattern))

// Use contains with value predicates
pattern.filter(|p| p.contains(&subpattern) && p.all_values(|v| *v > 0))
```

### Comparison with Query Functions

| Function | Scope | Return | Short-Circuit |
|----------|-------|--------|---------------|
| any_value | Values only | bool | Yes (first true) |
| all_values | Values only | bool | Yes (first false) |
| filter | Patterns | Vec<&Pattern> | No |
| find_first | Patterns | Option<&Pattern> | Yes (first Some) |
| matches | Two patterns | bool | Yes (first false) |
| contains | Two patterns | bool | Yes (first true) |

## Documentation Requirements

Each function needs:

1. **Purpose**: Clear statement of what it does
2. **Semantics**: Traversal order, matching criteria, return conditions
3. **Examples**: Atomic patterns, nested patterns, edge cases, combinations
4. **Complexity**: Time and space complexity
5. **Relationship**: How it relates to other Pattern operations
6. **Edge Cases**: Behavior on atomic patterns, empty elements, no matches

Documentation style will match existing any_value, all_values, filter documentation.

## Testing Strategy

### Unit Tests (per function)

- Atomic patterns (single value, no elements)
- Nested patterns (2-3 levels)
- Deep patterns (10+ levels)
- Wide patterns (10+ elements)
- Empty elements
- No matches
- Multiple matches (find_first: first, filter: all)
- Edge cases from spec

### Property-Based Tests

- `find_first(p)` returns Some implies `filter(p)` is non-empty
- `find_first(p)` returns Some(x) implies `p(x)` is true
- `matches` is reflexive: `p.matches(&p)` is true
- `matches` is symmetric: `p.matches(&q) == q.matches(&p)`
- `contains` is reflexive: `p.contains(&p)` is true
- `contains` is transitive: if `a.contains(&b)` and `b.contains(&c)` then `a.contains(&c)`
- Consistency between functions: `p.matches(&q)` implies `p.contains(&q)`

### Integration Tests

- Combining with other Pattern operations (map, fold, etc.)
- Performance benchmarks for target workloads
- WASM compilation verification

### Equivalence Tests (gram-hs)

Extract test cases from gram-hs test suite:
- `../gram-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs`
- Compare outputs for identical inputs
- Document any intentional deviations

## Implementation Order

**Phase 1**: find_first
- Simplest of the three new functions
- Builds on existing filter infrastructure
- Tests verify traversal order consistency

**Phase 2**: matches
- Required by contains
- Standalone functionality
- Tests verify structural comparison semantics

**Phase 3**: contains
- Depends on matches
- Most complex (recursive search with structural matching)
- Tests verify containment semantics

## Conclusion

**Decision**: ✅ Proceed with predicate matching implementation

**Rationale**:
1. Clear implementation strategies for all three missing functions
2. Consistency with existing gram-rs operations (traversal, traits, return types)
3. Behavioral equivalence with gram-hs reference implementation
4. Idiomatic Rust patterns (Option, borrowed references, Fn traits)
5. Performance targets easily achievable
6. Edge case handling well-defined
7. Testing strategy comprehensive

**Next Steps**: Proceed to Phase 1 design (data-model.md, contracts, quickstart.md)

