# Research: Pattern Combination Operations

**Feature**: 013-semigroup-instance  
**Date**: 2026-01-04

## Research Tasks

### 1. Haskell Semigroup Instance Analysis

**Task**: Understand the Pattern Semigroup instance from gram-hs reference implementation

**Source**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` (Semigroup instance location needs verification)

**Findings**:

**Decision**: Implement pattern combination using value combination + element concatenation

**Expected Haskell Implementation Pattern**:
```haskell
instance Semigroup v => Semigroup (Pattern v) where
  Pattern v1 es1 <> Pattern v2 es2 = Pattern (v1 <> v2) (es1 <> es2)
```

**Algorithm** (to be verified against actual code):
1. Combine pattern values (v1 <> v2) using V's Semigroup instance
2. Combine element lists (es1 <> es2) by concatenation
3. Create new pattern with combined value and combined elements

**Rationale**: This provides compositional combination where:
- Values combine using their type's combination semantics
- Elements concatenate, preserving order (left pattern elements first, then right pattern elements)
- Associativity follows from associativity of value combination and list concatenation

**Key Properties**:
- Requires V to implement Semigroup (or equivalent combination operation)
- Element concatenation is always associative: (a ++ b) ++ c = a ++ (b ++ c)
- If V's combination is associative, Pattern<V>'s combination is also associative
- No special handling needed for atomic patterns (empty elements concatenate to empty)

**Verification Needed**: 
- Confirm exact implementation in `../gram-hs/libs/pattern/src/Pattern/Core.hs`
- Verify any special cases or alternative combination strategies
- Check if there are multiple Semigroup instances for different use cases

**Alternatives considered**: None yet - will be based on actual gram-hs implementation

### 2. Idiomatic Rust Approach Decision

**Task**: Determine the most idiomatic way to express pattern combination in Rust

**Source**: Rust standard library patterns, existing Pattern methods, Rust API guidelines

**Findings**:

**Decision**: Use concrete method `combine()` rather than custom Semigroup trait or std::ops::Add

**Rationale**:

1. **Custom Semigroup Trait (REJECTED)**:
   - Rust ecosystem doesn't use abstract algebraic typeclasses
   - Would be non-idiomatic and unfamiliar to most Rust developers
   - No other Rust libraries use this pattern successfully
   - Creates API that doesn't match ecosystem conventions

2. **std::ops::Add (EVALUATED)**:
   - Pros:
     - Standard library trait
     - Enables `p1 + p2` syntax
     - Familiar to Rust developers
   - Cons:
     - Semantic mismatch: "addition" implies commutativity (a + b = b + a)
     - Pattern combination is likely NOT commutative (element order matters)
     - Would be misleading to users expecting addition semantics
   - **Decision**: REJECT unless combination is proven commutative (unlikely)

3. **Concrete Method `combine()` (SELECTED)**:
   - Pros:
     - Matches existing Pattern API style (`map()`, `fold()`, `traverse()`)
     - Clear, explicit naming
     - No misleading operator semantics
     - Easy to discover and understand
     - Follows Rust API guidelines for explicit methods
   - Cons:
     - No operator syntax (must write `p1.combine(p2)`)
   - **Decision**: ACCEPT - best fit for Rust idioms

**API Signature**:
```rust
impl<V> Pattern<V> 
where
    V: /* combination trait - see Task 3 */
{
    /// Combines two patterns associatively.
    ///
    /// Creates a new pattern by combining the values and concatenating the elements.
    /// The operation is associative: `(a.combine(b)).combine(c)` equals `a.combine(b.combine(c))`.
    ///
    /// # Examples
    ///
    /// ```
    /// let p1 = Pattern::point("hello");
    /// let p2 = Pattern::point("world");
    /// let combined = p1.combine(p2);
    /// ```
    pub fn combine(self, other: Self) -> Self {
        Pattern {
            value: self.value.combine(other.value),
            elements: [self.elements, other.elements].concat(),
        }
    }
}
```

**Alternatives for element combination**:
- `[self.elements, other.elements].concat()` - creates new Vec by concatenation
- `self.elements.extend(other.elements); self.elements` - mutate and return (won't work with self ownership)
- Proper approach: `let mut elements = self.elements; elements.extend(other.elements); elements`

**Naming Alternatives Considered**:
- `append()` - considered but less clear about associativity property
- `merge()` - implies merging might deduplicate or reorder (doesn't capture concatenation)
- `concat()` - too specific to concatenation, doesn't capture value combination
- `combine()` - SELECTED - clear, matches mathematical property, explicit

### 3. Value Type Requirements

**Task**: Determine what trait bounds V must satisfy for combination

**Findings**:

**Decision**: Define a trait bound that expresses "types that can be combined associatively"

**Options**:

**Option A: Custom Trait (Simple)**:
```rust
/// Types that support associative combination
pub trait Combinable {
    fn combine(self, other: Self) -> Self;
}

impl<V: Combinable> Pattern<V> {
    pub fn combine(self, other: Self) -> Self {
        Pattern {
            value: self.value.combine(other.value),
            elements: /* concatenation */
        }
    }
}
```

**Option B: Use std::ops::Add (If Appropriate)**:
```rust
impl<V: Add<Output = V>> Pattern<V> {
    pub fn combine(self, other: Self) -> Self {
        Pattern {
            value: self.value + other.value,
            elements: /* concatenation */
        }
    }
}
```

**Option C: Generic with Closure (Most Flexible)**:
```rust
impl<V> Pattern<V> {
    pub fn combine_with<F>(self, other: Self, f: F) -> Self 
    where
        F: Fn(V, V) -> V
    {
        Pattern {
            value: f(self.value, other.value),
            elements: /* concatenation */
        }
    }
}

// Convenience method for types that implement Add
impl<V: Add<Output = V>> Pattern<V> {
    pub fn combine(self, other: Self) -> Self {
        self.combine_with(other, |a, b| a + b)
    }
}
```

**Decision**: Start with **Option A (Custom Trait)** for MVP, can add Option C later

**Rationale**:
- Option A is simplest and most explicit about requirements
- Makes it clear that combination must be associative (document in trait)
- Allows different types to implement combination in their own way
- Can be extended later with more sophisticated approaches
- Aligns with making combination behavior explicit

**Trait Design**:
```rust
/// Types that support associative combination.
///
/// Implementors must ensure that combination is associative:
/// `(a.combine(b)).combine(c)` must equal `a.combine(b.combine(c))` for all values.
///
/// # Examples
///
/// ```
/// impl Combinable for String {
///     fn combine(self, other: Self) -> Self {
///         self + &other  // String concatenation is associative
///     }
/// }
/// ```
pub trait Combinable {
    fn combine(self, other: Self) -> Self;
}
```

**Standard Type Implementations** (to provide out of the box):
- `String`: concatenation (`s1 + s2`)
- `Vec<T>`: concatenation (`v1.extend(v2)`)
- Numeric types: addition (if semantically appropriate - may defer)
- `()`: trivial combination (returns `()`)

### 4. Property-Based Testing Strategy

**Task**: Identify properties to test with proptest

**Source**: Rust proptest documentation, Semigroup laws, existing pattern tests

**Findings**:

**Decision**: Implement comprehensive property tests for associativity law

**Properties to Test**:

1. **Associativity** (P001 - CRITICAL):
   ```rust
   for all patterns a, b, c:
     (a.combine(b)).combine(c) == a.combine(b.combine(c))
   ```
   
   This is THE defining property of the semigroup operation.

2. **Structural Preservation** (P002):
   ```rust
   for all patterns p1, p2:
     let result = p1.combine(p2)
     result is a valid Pattern (well-formed)
   ```

3. **Element Concatenation** (P003):
   ```rust
   for all patterns p1, p2:
     let result = p1.combine(p2)
     result.elements.len() == p1.elements.len() + p2.elements.len()
     result.elements[0..p1.len()] == p1.elements
     result.elements[p1.len()..] == p2.elements
   ```

4. **Value Combination** (P004):
   ```rust
   for all patterns p1, p2:
     let result = p1.combine(p2)
     result.value == p1.value.combine(p2.value)
   ```

5. **Atomic Pattern Handling** (P005):
   ```rust
   for all atomic patterns p1, p2:
     let result = p1.combine(p2)
     result.elements.is_empty()
   ```

**Test Pattern Generators**:
- Atomic patterns (no elements)
- Shallow patterns (1-2 levels, 1-5 elements)
- Deep patterns (50-100 levels)
- Wide patterns (100-1000 elements)
- Mixed structures (various combinations)

**Test Data Types**:
- `String` (concatenation combination)
- `i32` or `i64` (addition combination - if implemented)
- `()` (unit type - trivial combination)
- Custom test type with known combination semantics

**Existing Infrastructure**: proptest is already set up (feature 003), can reuse pattern generators and extend with Combinable instances.

### 5. Performance Considerations

**Task**: Identify performance targets and optimization strategies

**Findings**:

**Decision**: Direct implementation should meet targets; optimize if needed

**Performance Targets** (from spec):
- Combine two patterns with 1000 elements each: <1ms
- Combine deep patterns (100+ levels): no stack overflow, <10ms
- Fold 100 patterns in sequence: <100ms

**Expected Performance**:
- **Value combination**: Depends on V's combine implementation
  - For String: O(n + m) where n, m are string lengths
  - For numeric types: O(1)
- **Element concatenation**: O(n) where n is total element count
  - Allocates new Vec
  - Copies elements from both input vectors
- **Overall**: O(value_combine + elements1 + elements2)

**Implementation Strategy**:

```rust
pub fn combine(self, other: Self) -> Self {
    // 1. Combine values (depends on V's implementation)
    let combined_value = self.value.combine(other.value);
    
    // 2. Concatenate elements efficiently
    let mut combined_elements = self.elements;
    combined_elements.extend(other.elements);
    
    // 3. Return new pattern
    Pattern {
        value: combined_value,
        elements: combined_elements,
    }
}
```

**Optimization Strategies**:

1. **Efficient element concatenation**:
   - Use `Vec::extend()` instead of `[a, b].concat()`
   - Reuses first vector's allocation when possible
   - Single allocation + copy instead of two

2. **Ownership consumption**:
   - Method takes `self` (not `&self`)
   - Moves values instead of cloning
   - No unnecessary copies

3. **If needed (profile first)**:
   - Pre-allocate vector with correct capacity: `Vec::with_capacity(len1 + len2)`
   - Consider Rc/Arc for structural sharing (advanced optimization)
   - Benchmark against gram-hs performance for equivalent patterns

**Memory Usage**:
- Creates new Pattern (1 allocation)
- Combines values (depends on V)
- Extends element vector (may reallocate if capacity insufficient)
- Best case: 1-2 allocations
- Worst case: 1 + reallocation per extend

**Rationale**: Start with straightforward implementation using Vec::extend for efficiency. Profile if performance targets aren't met. The gram-hs reference doesn't specify special optimizations, so correctness is priority over premature optimization.

## Summary

### Implementation Approach

1. **API Design**:
   - Concrete method `Pattern::combine(self, other: Self) -> Self`
   - Custom `Combinable` trait for value types
   - No custom Semigroup trait (non-idiomatic in Rust)

2. **Combination Algorithm**:
   - Combine values using V's Combinable implementation
   - Concatenate elements (left elements first, then right elements)
   - Return new pattern with combined value and elements

3. **Testing Strategy**:
   - 5 property-based tests focusing on associativity
   - 20-30 concrete tests for edge cases
   - Equivalence tests comparing with gram-hs behavior
   - Performance benchmarks for large patterns

4. **Performance**:
   - Use Vec::extend for efficient element concatenation
   - Consume self to avoid unnecessary clones
   - Profile if targets not met (expected to be fine)

### Key Decisions Rationale

1. **Concrete method over trait**: Most idiomatic for Rust, matches existing Pattern API
2. **Custom Combinable trait**: Explicit about requirements, flexible for different types
3. **Element concatenation**: Simple, associative, preserves order
4. **Associativity testing**: Critical property, comprehensive property-based verification

### Value Type Implementations

Provide Combinable implementations for common types:
- `String`: concatenation
- `Vec<T>`: concatenation
- `()`: trivial (returns `()`)
- Additional types as needed

### Risk Mitigation

- **Associativity verification**: Comprehensive property tests with 10,000+ cases
- **Performance**: Benchmark with large patterns, optimize if needed
- **Semantic drift**: Port all gram-hs test cases, verify behavior exactly
- **API usability**: Follow existing Pattern method conventions (map, fold, traverse)

## References

- **Haskell Semigroup Instance**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` (to be verified)
- **Haskell Spec**: `../gram-hs/specs/010-semigroup-instance/` (for context)
- **Rust API Guidelines**: https://rust-lang.github.io/api-guidelines/
- **Vec extend**: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.extend
- **Porting Guide**: `../../../PORTING_GUIDE.md`

