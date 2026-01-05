# Type Signatures: Predicate-Based Pattern Matching

**Feature**: 016-predicate-matching  
**Date**: 2025-01-05  
**Status**: Design Complete

## Public API Contracts

This document defines the public API contracts for the three new pattern matching methods added to `Pattern<V>`.

### find_first

**Location**: `crates/pattern-core/src/pattern.rs`

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn find_first<F>(&self, predicate: F) -> Option<&Pattern<V>>
    where
        F: Fn(&Pattern<V>) -> bool,
    {
        // Implementation
    }
}
```

**Contract**:
- **Preconditions**: None (works with any pattern, any predicate)
- **Postconditions**:
  - If result is `Some(p)`, then `predicate(p)` is true
  - If result is `Some(p)`, then `p` appears in self's structure (self or descendant)
  - If result is `Some(p)`, then `p` is the first matching pattern in depth-first pre-order traversal
  - If result is `None`, then no pattern in self's structure satisfies predicate
- **Traversal**: Depth-first pre-order (root first, elements left-to-right, recursive)
- **Short-Circuit**: Returns immediately on first match
- **Complexity**: Time O(k) where k is position of first match, Space O(d) where d is depth
- **Lifetime**: Returned reference borrows from self

**Test Requirements**:
- ✅ Returns Some for root pattern when root matches
- ✅ Returns Some for element pattern when element matches
- ✅ Returns Some for deeply nested pattern when nested pattern matches
- ✅ Returns None when no patterns match
- ✅ Returns first match in pre-order traversal when multiple match
- ✅ Works with atomic patterns (no elements)
- ✅ Works with empty elements
- ✅ Predicate receives correct pattern reference
- ✅ Predicate can examine value and structure
- ✅ Handles deeply nested structures (100+ levels)

### matches

**Location**: `crates/pattern-core/src/pattern.rs`

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn matches(&self, other: &Pattern<V>) -> bool
    where
        V: PartialEq,
    {
        // Implementation
    }
}
```

**Contract**:
- **Preconditions**: V must implement PartialEq (values must be comparable)
- **Postconditions**:
  - Returns true iff:
    - self.value == other.value
    - self.elements.len() == other.elements.len()
    - For all i: self.elements[i].matches(&other.elements[i])
  - Reflexive: `p.matches(&p)` is always true
  - Symmetric: `p.matches(&q) == q.matches(&p)`
  - For identical patterns: `p.matches(&q)` iff `p == q` (when Eq defined)
- **Distinguishes Structure**: Patterns with same flattened values but different structures return false
- **Short-Circuit**: Returns false on first mismatch
- **Complexity**: Time O(min(n, m)), Space O(min(d1, d2))
- **No Mutation**: Both self and other unchanged

**Test Requirements**:
- ✅ Returns true for identical patterns
- ✅ Returns true for self-comparison (reflexive)
- ✅ Returns false for different values
- ✅ Returns false for different element counts
- ✅ Returns false for different element structures
- ✅ Distinguishes same values, different structures
- ✅ Symmetric: p.matches(&q) == q.matches(&p)
- ✅ Works with atomic patterns
- ✅ Works with empty elements
- ✅ Works with deeply nested structures

### contains

**Location**: `crates/pattern-core/src/pattern.rs`

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn contains(&self, subpattern: &Pattern<V>) -> bool
    where
        V: PartialEq,
    {
        // Implementation
    }
}
```

**Contract**:
- **Preconditions**: V must implement PartialEq (uses matches internally)
- **Postconditions**:
  - Returns true iff:
    - self.matches(subpattern) OR
    - Any element in self.elements contains subpattern (recursive)
  - Reflexive: `p.contains(&p)` is always true
  - Transitive: if `a.contains(&b)` and `b.contains(&c)` then `a.contains(&c)`
  - Weaker than matches: `p.matches(&q)` implies `p.contains(&q)`
- **Search Scope**: Checks self and all descendants (all nesting levels)
- **Short-Circuit**: Returns true on first match
- **Complexity**: Time O(n*m) worst case, Space O(d)
- **No Mutation**: Both self and subpattern unchanged

**Test Requirements**:
- ✅ Returns true for self-containment
- ✅ Returns true when subpattern is direct element
- ✅ Returns true when subpattern is nested descendant
- ✅ Returns false when subpattern not found
- ✅ Transitive: a.contains(&b) && b.contains(&c) => a.contains(&c)
- ✅ Weaker than matches: p.matches(&q) => p.contains(&q)
- ✅ Works with atomic patterns
- ✅ Works with empty elements
- ✅ Works with deeply nested structures
- ✅ Handles multiple occurrences (returns true if any match)

## Type Constraints Summary

| Method | Self Bound | Other/Predicate Bound | Return Type |
|--------|------------|----------------------|-------------|
| find_first | None | F: Fn(&Pattern<V>) -> bool | Option<&Pattern<V>> |
| matches | V: PartialEq | None | bool |
| contains | V: PartialEq | None | bool |

## Behavioral Properties

### Consistency Properties

**Traversal Consistency**:
- find_first uses same traversal order as filter, any_value, all_values, fold, map

**Predicate Consistency**:
- find_first and filter use same predicate type: F: Fn(&Pattern<V>) -> bool

**Return Type Consistency**:
- find_first returns Option (not Result) - no match is not an error
- matches and contains return bool (binary condition)

### Mathematical Properties

**find_first**:
- `find_first(p).is_some()` implies `filter(p).len() > 0`
- `find_first(p).is_some()` implies `any_value(q)` where q tests if value appears in filter(p)
- `find_first(p) == Some(x)` implies `p(x)` is true
- `find_first(p) == Some(x)` implies x is first in pre-order traversal

**matches**:
- Reflexive: `p.matches(&p)` for all p
- Symmetric: `p.matches(&q)` iff `q.matches(&p)`
- For Eq types: `p.matches(&q)` iff `p == q`

**contains**:
- Reflexive: `p.contains(&p)` for all p
- Transitive: `a.contains(&b)` and `b.contains(&c)` implies `a.contains(&c)`
- Weaker than matches: `p.matches(&q)` implies `p.contains(&q)`
- Not symmetric: `p.contains(&q)` does not imply `q.contains(&p)`

### Relationship Properties

**find_first vs filter**:
- `find_first(p) == Some(x)` implies `x == filter(p)[0]`
- `find_first(p) == None` iff `filter(p).is_empty()`

**matches vs Eq**:
- When V: Eq, `p.matches(&q)` iff `p == q`
- matches may diverge from Eq in future (wildcards, partial matching)

**contains vs matches**:
- `p.matches(&q)` implies `p.contains(&q)` (equality implies containment)
- `p.contains(&q)` does not imply `p.matches(&q)` (containment weaker)

## Performance Contracts

### Time Complexity Guarantees

| Method | Best Case | Average Case | Worst Case |
|--------|-----------|--------------|------------|
| find_first | O(1) | O(k) | O(n) |
| matches | O(1) | O(min(n,m)/2) | O(min(n,m)) |
| contains | O(1) | O(n*m/2) | O(n*m) |

Where:
- n = number of nodes in container
- m = number of nodes in subpattern/comparison
- k = position of first match

### Space Complexity Guarantees

All methods use O(d) stack space where d is maximum nesting depth.

For patterns with depth ≤ 100, stack usage ≤ 10KB (well within limits).

### Performance Targets (from spec SC-005, SC-006)

- find_first on 1000-node pattern, match in first 10 nodes: < 10ms
- All methods on 1000-node pattern, depth 100: < 100ms

## Error Handling Contracts

**No Panics**: All methods handle all inputs gracefully:
- Atomic patterns (no elements)
- Empty elements (empty Vec)
- Deep nesting (100+ levels)
- No matches (returns false/None)
- Null patterns not possible (Rust type system prevents)

**No Errors**: No Result return types, no error conditions:
- No match is expected outcome (false/None)
- Invalid input prevented by type system (PartialEq bound)

## Thread Safety

All methods are thread-safe:
- Immutable borrows only (&self, &Pattern<V>)
- No interior mutability
- Can be called from multiple threads (if V: Sync)
- No data races possible

## Lifetime Contracts

**find_first**:
- Returned Option<&Pattern<V>> borrows from self
- Lifetime: `'a` where self: `'a`
- Cannot move/drop self while reference exists

**matches and contains**:
- No references returned
- No lifetime constraints

## WASM Compatibility

All methods compatible with WASM:
- No blocking I/O
- No file system access
- No platform-specific code
- Pure computation only
- Stack usage within WASM limits

## Verification Requirements

### Unit Tests (per method)

Each method requires tests for:
1. Basic functionality (simple cases)
2. Edge cases (atomic, empty, deep, no match)
3. Traversal order (find_first)
4. Structural properties (matches/contains)
5. Integration with other methods

### Property Tests

Required property verifications:
- find_first: consistency with filter
- matches: reflexive, symmetric
- contains: reflexive, transitive, weaker than matches

### Equivalence Tests

Compare outputs with gram-hs reference implementation:
- Extract test cases from `../gram-hs/libs/pattern/tests/`
- Verify identical behavior for identical inputs
- Document any intentional deviations

### Performance Tests

Verify performance targets:
- find_first: < 10ms for 1000 nodes, match in first 10
- All methods: < 100ms for 1000 nodes, depth 100
- No stack overflow for depth 100

## Documentation Requirements

Each method requires:
- **Purpose**: One-sentence summary
- **Parameters**: Description and constraints
- **Returns**: Description and semantics
- **Examples**: Basic usage, edge cases, combinations
- **Complexity**: Time and space complexity
- **Panics**: None (document this explicitly)
- **Relationship**: How it relates to other methods

Documentation style matches existing methods (any_value, all_values, filter).

## Breaking Changes

None. All additions are new methods that don't affect existing API:
- Existing methods unchanged
- No signature changes
- No behavioral changes to existing functionality
- Backward compatible with all existing code

