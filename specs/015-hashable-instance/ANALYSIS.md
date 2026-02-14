# Feature 015: Hash Trait Implementation - Analysis

## Executive Summary

**Recommendation**: Implement Hash trait for `Pattern<V>` - **HIGH VALUE** with straightforward implementation.

**Implementation Approach**: Idiomatic Rust using conditional trait implementation (`impl<V: Hash> Hash for Pattern<V>`).

**Complexity**: LOW - Simple recursive implementation, ~20 lines of code.

**Use Cases**: Enables efficient pattern deduplication, caching, and set-based operations.

## Haskell Reference Implementation

The gram-hs implementation provides a Hashable instance for Pattern:

```haskell
instance Hashable v => Hashable (Pattern v) where
  hashWithSalt salt (Pattern v es) = 
    salt `hashWithSalt` v `hashWithSalt` es
```

**Key Properties** (from gram-hs documentation):
- Structure-preserving: Hashing based on value and elements recursively
- Equality consistency: If `p1 == p2`, then `hash p1 == hash p2`
- Structure distinguishes hash: Different structures with same values produce different hashes
- Enables HashMap/HashSet usage for efficient lookups and deduplication

## Idiomatic Rust Approach

### 1. Trait Implementation Strategy

Rust provides `std::hash::Hash` trait for hashing. For `Pattern<V>`, the idiomatic approach is:

```rust
impl<V: Hash> Hash for Pattern<V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.elements.hash(state);
    }
}
```

**Why this is idiomatic**:
- Uses conditional trait bounds (`V: Hash`) - only patterns with hashable values are hashable
- Leverages automatic `Vec<T>` Hash implementation (already recursive)
- Simple, maintainable, follows Rust std library conventions
- No custom derive needed - manual implementation is more explicit for recursive types

### 2. Type Compatibility Analysis

**Fully Compatible Types** (Can use Hash):
- ✅ `Pattern<String>` - Most common case
- ✅ `Pattern<i32>`, `Pattern<u64>`, etc. - Numeric types
- ✅ `Pattern<Symbol>` - Already implements Eq, can add Hash
- ✅ `Pattern<&str>`, `Pattern<bool>` - Standard types

**Incompatible Types** (Cannot use Hash):
- ❌ `Pattern<Subject>` - Subject contains `f64` in `Value` enum
- ❌ `Pattern<f64>` - Floats don't implement Hash (NaN issues)
- ❌ Any `Pattern<V>` where V contains floats

**This is correct behavior**: Rust's type system prevents nonsensical hashing of types that shouldn't be hashed.

### 3. Subject Type Considerations

`Subject` contains `Value`, which includes `VDecimal(f64)` variant:

```rust
pub enum Value {
    VInteger(i64),
    VDecimal(f64),  // ❌ Cannot implement Hash
    VBoolean(bool),
    VString(String),
    // ... other variants
}
```

**Options**:

**Option A: Don't implement Hash for Subject** (RECOMMENDED)
- Subject already only implements `PartialEq`, not `Eq` (documented reason: f64)
- Consistent with existing design decisions
- Pattern<Subject> won't be hashable, which is semantically correct
- Users can still hash Pattern<String>, Pattern<Symbol>, etc.

**Option B: Implement Hash for Subject with NaN handling**
- Would require special handling of f64 (e.g., convert NaN to constant)
- Violates Hash/Eq consistency requirement in Rust docs
- Not recommended - adds complexity and potential bugs

**Option C: Make Value hashable with bit-pattern hashing**
- Could use `f64.to_bits()` for hashing
- But still inconsistent with lack of Eq implementation
- Not semantically sound (equal values must hash equally)

**Recommendation**: Accept that `Subject` and `Value` should not implement Hash, which is correct for types containing floats.

## Use Cases and Value Assessment

### High-Value Use Cases

1. **Pattern Deduplication**
   ```rust
   use std::collections::HashSet;
   
   let patterns: Vec<Pattern<String>> = load_patterns();
   let unique: HashSet<Pattern<String>> = patterns.into_iter().collect();
   ```
   **Value**: Essential for processing large pattern sets, removing duplicates efficiently.

2. **Pattern Caching/Memoization**
   ```rust
   use std::collections::HashMap;
   
   let mut cache: HashMap<Pattern<String>, ComputedResult> = HashMap::new();
   
   fn expensive_computation(p: &Pattern<String>) -> ComputedResult {
       if let Some(result) = cache.get(p) {
           return result.clone();
       }
       // ... expensive work
   }
   ```
   **Value**: Critical for performance optimization in parsers, analyzers, code generators.

3. **Set-Based Pattern Operations**
   ```rust
   let set_a: HashSet<Pattern<String>> = /* ... */;
   let set_b: HashSet<Pattern<String>> = /* ... */;
   
   let intersection = set_a.intersection(&set_b);
   let difference = set_a.difference(&set_b);
   ```
   **Value**: Enables efficient set-theoretic operations on pattern collections.

4. **Pattern Indexing**
   ```rust
   let mut index: HashMap<Pattern<String>, Vec<Location>> = HashMap::new();
   // Build reverse index for pattern search
   ```
   **Value**: Essential for pattern search engines, code navigation, refactoring tools.

### Performance Benefits

- **O(1) average-case** lookups vs O(n) with Vec
- **O(1) average-case** deduplication vs O(n²) with naive approaches
- **Memory efficiency**: HashSet avoids duplicates automatically
- **Cache-friendly**: Hash-based structures enable efficient memoization

### Real-World Usage Patterns

Based on the project context (pattern-rs as a graph pattern library):

1. **Pattern Storage**: Store unique patterns efficiently
2. **Graph Serialization**: Deduplicate patterns during codec operations
3. **Pattern Query**: Fast lookups for pattern matching operations
4. **WASM Bindings**: JavaScript Maps/Sets expect hashable keys

## Implementation Plan

### Phase 1: Core Implementation

```rust
// In crates/pattern-core/src/pattern.rs

impl<V: Hash> Hash for Pattern<V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.elements.hash(state);  // Vec's Hash impl handles recursion
    }
}
```

### Phase 2: Symbol Type Enhancement

```rust
// In crates/pattern-core/src/subject.rs

#[derive(Clone, PartialEq, Eq, Hash)]  // Add Hash derive
pub struct Symbol(pub String);
```

### Phase 3: Documentation

Update Pattern documentation to note Hash availability:

```rust
/// # Trait Implementations
///
/// - `Hash`: Patterns can be hashed when `V: Hash` for use in HashMap/HashSet
///   - Enables pattern deduplication and caching
///   - Note: Pattern<Subject> is NOT hashable (Subject contains f64)
```

### Phase 4: Test Suite

Comprehensive tests covering:
- Hash consistency with equality (equal patterns have equal hashes)
- Structure distinguishes hash (different structures → different hashes)
- HashMap operations
- HashSet operations
- Property-based testing (Hash laws)

## Comparison with Other Features

| Feature | Complexity | Value | Priority |
|---------|-----------|-------|----------|
| 012-ord-instance | Medium | High | ✅ Complete |
| 013-semigroup-instance | Medium | High | ✅ Complete |
| 014-monoid-instance | Low | High | ✅ Complete |
| **015-hashable-instance** | **Low** | **High** | **Recommended** |
| 016-predicate-matching | High | High | Pending |
| 017-applicative-instance | Medium | Medium | Pending |

**Hash trait stands out as**:
- Lower complexity than most remaining features
- High practical value (HashMap/HashSet usage is ubiquitous)
- Natural complement to existing Eq/Ord implementations
- Required for efficient pattern storage/query systems

## Risks and Mitigations

### Risk 1: Hash/Eq Consistency
**Risk**: Hash implementation might not be consistent with Eq.  
**Mitigation**: 
- Leverage Vec's built-in Hash implementation (already consistent)
- Property-based tests to verify consistency
- Simple implementation reduces chance of bugs

### Risk 2: Performance on Deep Nesting
**Risk**: Hashing deeply nested patterns might be slow.  
**Mitigation**:
- Hash computation is O(n) in pattern size (acceptable)
- Results are cached in HashSet/HashMap (computed once)
- No slower than Eq comparison (which is also O(n))

### Risk 3: Breaking Changes
**Risk**: None - this is purely additive (new trait implementation).  
**Mitigation**: Not applicable.

## Decision Criteria

**Implement if**:
- ✅ Pattern deduplication/caching is needed (yes - essential for pattern storage)
- ✅ Implementation is straightforward (yes - ~20 lines of code)
- ✅ Consistent with Rust idioms (yes - standard conditional trait impl)
- ✅ Has clear use cases (yes - HashMap/HashSet are fundamental collections)
- ✅ Compatible with existing design (yes - works with Eq/Ord implementations)

**Defer if**:
- ❌ Would require breaking changes (no)
- ❌ Complex implementation (no)
- ❌ Unclear use cases (no)
- ❌ Conflicts with other features (no)

## Recommendation: IMPLEMENT

**Priority**: HIGH (next after 014-monoid-instance)

**Rationale**:
1. **High practical value**: HashMap/HashSet usage is fundamental in Rust
2. **Low implementation complexity**: Simple recursive hashing, ~20 lines
3. **Complements existing traits**: Natural extension of Eq/Ord/Clone
4. **Enables key features**: Pattern caching, deduplication, indexing
5. **WASM-friendly**: Essential for efficient JavaScript interop
6. **Consistent with gram-hs**: Direct port of existing Haskell feature

**Implementation Estimate**: 2-3 hours
- 30 min: Core implementation
- 30 min: Symbol Hash derive
- 1 hour: Test suite
- 30 min: Documentation updates

**Testing Strategy**:
- Unit tests: Hash consistency with Eq
- Property tests: Hash laws (proptest)
- Integration tests: HashMap/HashSet usage
- Equivalence tests: Compare with gram-hs behavior

## Alternatives Considered

### Alternative 1: Use BTreeMap/BTreeSet Instead
**Status**: Not recommended
- Requires Ord (already implemented)
- But hash-based collections are faster for lookups (O(1) vs O(log n))
- Both should be supported (user's choice)

### Alternative 2: Defer Until Pattern Store Phase
**Status**: Not recommended  
- Hash is useful now (testing, examples, early adoption)
- Simple enough to implement immediately
- Blocking on future features adds unnecessary delay

### Alternative 3: Implement Custom Hash Algorithm
**Status**: Not necessary
- Rust's standard Hash trait is sufficient
- No special requirements justify custom algorithm
- Would add complexity without benefit

## Conclusion

Implementing Hash for Pattern<V> is a **high-value, low-complexity feature** that should be implemented soon. It enables critical use cases (deduplication, caching, indexing) while maintaining design consistency and Rust idioms.

The implementation is straightforward, follows the Haskell reference closely, and integrates naturally with existing Eq/Ord implementations. The only limitation (Pattern<Subject> not hashable) is correct and expected due to Subject containing floats.

**Recommended Action**: Proceed with implementation as feature 015, following the implementation plan above.
