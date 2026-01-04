# Research: Pattern Ordering and Comparison

**Feature**: 012-ord-trait  
**Date**: 2025-01-04

## Research Tasks

### 1. Haskell Ord Instance Analysis

**Task**: Understand the Pattern Ord instance from gram-hs reference implementation

**Source**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` lines 335-339

**Findings**:

**Decision**: Use simple lexicographic comparison: value first, then elements

**Haskell Implementation**:
```haskell
instance Ord v => Ord (Pattern v) where
  compare (Pattern v1 es1) (Pattern v2 es2) =
    case compare v1 v2 of
      EQ -> compare es1 es2
      other -> other
```

**Algorithm**:
1. Compare pattern values (v1 vs v2) using their Ord instance
2. If values are equal (EQ), compare element lists lexicographically
3. Otherwise, return the result of value comparison

**Rationale**: This provides a simple, predictable ordering that prioritizes the value (which represents "information about the elements"). List comparison in Haskell is automatic and lexicographic, comparing element-by-element from left to right.

**Alternatives considered**: None needed - this is the reference implementation

**Key Properties**:
- Value comparison takes precedence over structure
- Element lists compared lexicographically (element-by-element, left-to-right)
- Shorter lists compare as less than longer lists if all compared elements are equal
- Deeply nested patterns are compared recursively following the same rules

### 2. Rust Ord Trait Requirements

**Task**: Understand Rust's Ord trait hierarchy and requirements

**Source**: Rust standard library documentation (std::cmp)

**Findings**:

**Decision**: Implement both PartialOrd and Ord traits following Rust conventions

**Trait Hierarchy**:
```
PartialEq (already implemented ✓)
    ↓
    Eq (already implemented ✓)
    ↓
    PartialOrd (to implement)
    ↓
    Ord (to implement)
```

**Required Implementations**:

1. **PartialOrd trait**:
   - `fn partial_cmp(&self, other: &Self) -> Option<Ordering>`
   - For Pattern<V> where V: PartialOrd
   - Returns Some(ordering) or None if values can't be compared

2. **Ord trait**:
   - `fn cmp(&self, other: &Self) -> Ordering`
   - For Pattern<V> where V: Ord
   - Always returns a definitive ordering (Less, Equal, or Greater)
   - Requires Eq + PartialOrd to be implemented

**Ord Laws** (must hold for all x, y, z):

1. **Reflexivity**: x == x implies x.cmp(x) == Equal
2. **Antisymmetry**: if x < y then !(y < x)
3. **Transitivity**: if x < y and y < z then x < z
4. **Totality**: exactly one of x < y, x == y, or x > y holds
5. **Consistency with Eq**: x == y implies x.cmp(y) == Equal

**Rationale**: Rust's type system enforces the trait hierarchy, ensuring that patterns can only implement Ord if the value type V also implements Ord. This prevents nonsensical comparisons at compile time.

**Note**: Pattern already implements PartialEq and Eq (derived), so we can build on that foundation.

### 3. Comparison Strategy for Recursive Structures

**Task**: Determine implementation approach for comparing nested patterns

**Findings**:

**Decision**: Use direct recursive comparison (not iterative)

**Implementation Strategy**:

```rust
impl<V: Ord> Ord for Pattern<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        // 1. Compare values first
        match self.value.cmp(&other.value) {
            Ordering::Equal => {
                // 2. If equal, compare element vectors
                self.elements.cmp(&other.elements)
            }
            non_equal => non_equal
        }
    }
}
```

**Rationale**: 
- Simple and matches Haskell's case-of pattern
- Rust's Vec<T> already implements Ord with lexicographic comparison
- Recursive comparison is handled automatically by Vec's Ord instance
- No manual element-by-element iteration needed
- Leverages Rust's automatic recursive trait derivation

**Stack Overflow Consideration**:
- Pattern depth target: 200+ levels
- Rust's default stack is ~2MB (Linux), ~8MB (macOS)
- Each comparison frame is small (just comparing references)
- Testing at 200 levels should reveal any issues
- If needed, can use iterative comparison with explicit stack (unlikely to be necessary)

**Short-Circuit Optimization**:
- Value comparison short-circuits (if values differ, done)
- Vec comparison short-circuits (stops at first differing element)
- No manual optimization needed, automatic from Rust's Ord on Vec

**Alternatives considered**:
- **Iterative with explicit stack**: Rejected - adds complexity without clear benefit, recursive approach is simpler
- **Manual element iteration**: Rejected - Vec::cmp already does this correctly
- **Parallel comparison**: Rejected - overkill for MVP, can add later if needed

### 4. Property-Based Testing for Ordering

**Task**: Identify properties to test with proptest

**Source**: Rust proptest documentation, Ord trait laws, existing pattern tests

**Findings**:

**Decision**: Implement comprehensive property tests for all Ord laws

**Properties to Test**:

1. **Reflexivity** (T001):
   ```rust
   for all patterns p: p.cmp(&p) == Ordering::Equal
   ```

2. **Antisymmetry** (T002):
   ```rust
   for all patterns p1, p2: 
     if p1.cmp(&p2) == Less then p2.cmp(&p1) == Greater
   ```

3. **Transitivity** (T003):
   ```rust
   for all patterns p1, p2, p3:
     if p1 < p2 and p2 < p3 then p1 < p3
   ```

4. **Totality** (T004):
   ```rust
   for all patterns p1, p2:
     exactly one of (p1 < p2), (p1 == p2), (p1 > p2) is true
   ```

5. **Consistency with Eq** (T005):
   ```rust
   for all patterns p1, p2:
     if p1 == p2 then p1.cmp(&p2) == Ordering::Equal
   ```

6. **Value Precedence** (T006):
   ```rust
   for all patterns p1, p2 where p1.value != p2.value:
     p1.cmp(&p2) == p1.value.cmp(&p2.value)
   ```

7. **Lexicographic Element Comparison** (T007):
   ```rust
   for all patterns with equal values but different elements:
     comparison follows lexicographic order of element vectors
   ```

**Test Pattern Generators**:
- Atomic patterns (no elements)
- Shallow patterns (1-2 levels)
- Deep patterns (50-200 levels)
- Wide patterns (100-5000 elements)
- Mixed structures (various combinations)

**Rationale**: Property-based testing ensures the implementation satisfies mathematical properties for all possible inputs, not just hand-picked examples. This provides much stronger correctness guarantees.

**Existing Infrastructure**: proptest is already set up (feature 003), can reuse pattern generators from other tests.

### 5. Performance Considerations

**Task**: Identify performance targets and optimization strategies

**Findings**:

**Decision**: Direct implementation should meet targets; profile if issues arise

**Performance Targets** (from spec):
- Sort 10,000 patterns: <200ms
- Compare deep patterns (200+ levels): no stack overflow, <100ms
- Compare wide patterns (5,000+ elements): <500ms

**Expected Performance**:
- **Atomic patterns**: O(1) - single value comparison
- **Nested patterns**: O(min(n1, n2)) where n is number of nodes
  - Best case: O(1) if values differ
  - Average case: O(log n) if early difference in elements
  - Worst case: O(n) if patterns are very similar
- **Sorting N patterns**: O(N log N × comparison_cost)

**Optimization Strategies**:

1. **Short-circuit on value difference** (automatic):
   - Most comparisons will differ at value level
   - Element comparison only happens when values equal

2. **Leverage Vec::cmp optimization** (automatic):
   - Rust's Vec comparison is optimized
   - Stops at first differing element
   - Uses efficient slice comparison

3. **No allocation during comparison** (automatic):
   - Comparison uses references (&self, &other)
   - No temporary allocations
   - Cache-friendly sequential access

4. **If needed (unlikely)**:
   - Profile hot paths with cargo-flamegraph
   - Consider caching comparison results (for repeated comparisons)
   - Consider parallel sorting for very large collections

**Rationale**: Start simple with direct implementation. The Haskell reference doesn't have special optimizations, and Rust's Vec comparison is already well-optimized. Profile and optimize only if targets aren't met.

## Summary

### Implementation Approach

1. **Trait Structure**:
   - Implement PartialOrd for Pattern<V> where V: PartialOrd
   - Implement Ord for Pattern<V> where V: Ord
   - Leverage existing PartialEq and Eq implementations

2. **Comparison Algorithm**:
   - Value-first lexicographic comparison
   - Delegate to Vec::cmp for element comparison
   - Automatic recursive comparison via Rust's trait system

3. **Testing Strategy**:
   - 7 property-based tests for Ord laws
   - 20-30 concrete tests for edge cases
   - Equivalence tests comparing with gram-hs behavior
   - Performance benchmarks for large patterns

4. **Performance**:
   - Direct implementation (no manual optimization)
   - Profile if targets not met (unlikely)
   - Leverage Rust's optimized Vec comparison

### Key Decisions Rationale

1. **Direct recursive comparison**: Simplest approach, matches Haskell, leverages Rust's automatic trait derivation
2. **No manual optimizations**: Start simple, optimize only if needed (YAGNI principle)
3. **Comprehensive property testing**: Ensures mathematical correctness for all inputs
4. **Behavioral equivalence focus**: Reference implementation is authoritative for comparison semantics

### Risk Mitigation

- **Stack overflow**: Test with 200+ level patterns, will reveal issues early
- **Performance**: Benchmark with large patterns (10k, 5k elements), profile if needed
- **Semantic drift**: Port all gram-hs test cases, verify outputs match exactly

## References

- **Haskell Ord Instance**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` lines 272-339
- **Rust Ord Documentation**: https://doc.rust-lang.org/std/cmp/trait.Ord.html
- **Rust PartialOrd Documentation**: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
- **Vec Ord Implementation**: https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-Ord

