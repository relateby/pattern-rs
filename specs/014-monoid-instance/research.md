# Research: Pattern Identity Element via Default Trait

**Feature**: 014-monoid-instance  
**Date**: 2026-01-05

## Research Tasks

### 1. Haskell Monoid Instance Analysis

**Task**: Understand the Pattern Monoid instance from gram-hs reference implementation

**Source**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` or similar location

**Expected Haskell Implementation Pattern**:
```haskell
instance Monoid v => Monoid (Pattern v) where
  mempty = Pattern mempty []
  
-- Combining with mempty (identity laws):
-- mempty <> p = p
-- p <> mempty = p
```

**Key Questions**:
1. What is the definition of the identity pattern in gram-hs?
2. How does gram-hs handle the value component of the identity pattern?
3. Are there any special cases or optimizations for combining with identity?
4. How are the monoid laws tested in gram-hs?

**Expected Answer** (to be verified):
- Identity pattern has `mempty` value (from value type's Monoid instance)
- Identity pattern has empty elements list `[]`
- Combination with identity returns the non-identity pattern unchanged
- Laws verified through property-based testing (QuickCheck)

---

### 2. Idiomatic Rust Approach Decision

**Task**: Determine the most idiomatic way to express monoid identity in Rust

**Options Considered**:

#### Option A: Custom Monoid Trait
```rust
pub trait Monoid: Combinable {
    fn empty() -> Self;
}

impl<V: Monoid> Monoid for Pattern<V> {
    fn empty() -> Self {
        Pattern::point(V::empty())
    }
}
```

**Pros**: 
- Direct translation from Haskell
- Explicit monoid laws in trait documentation
- Clear semantic intent

**Cons**:
- Non-idiomatic in Rust ecosystem
- Rust doesn't have standard algebraic typeclass traits
- Would need to implement for all value types
- Low adoption likelihood in broader ecosystem

#### Option B: Use std::default::Default ✅ SELECTED
```rust
impl<V: Default> Default for Pattern<V> {
    fn default() -> Self {
        Pattern::point(V::default())
    }
}
```

**Pros**:
- Idiomatic Rust using standard library trait
- Works with existing ecosystem (mem::take, etc.)
- Familiar to all Rust developers
- Already implemented by common types (String, Vec, etc.)
- Integrates seamlessly with iterator methods

**Cons**:
- Doesn't explicitly encode monoid laws in type system
- `Default` is a general-purpose trait, not algebra-specific
- Laws must be documented and tested separately

**Decision**: **Option B - Use std::default::Default**

**Rationale**:
1. Rust ecosystem strongly favors standard library traits over custom abstractions
2. `Default` trait serves the practical purpose of providing a zero/identity value
3. Monoid laws can be documented clearly and verified through comprehensive testing
4. This approach enables idiomatic Rust patterns (fold with default initial value)
5. Precedent: Other Rust libraries use `Default` for identity-like values rather than custom traits

---

### 3. Monoid Laws Testing Strategy

**Task**: Determine how to verify monoid laws in Rust's testing framework

**Approach**: Property-based testing with `proptest`

**Laws to Verify**:
1. **Left Identity**: `Pattern::default().combine(x) == x`
2. **Right Identity**: `x.combine(Pattern::default()) == x`
3. **Associativity** (already tested in feature 013): `(a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)`

**Testing Strategy**:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_left_identity(p: Pattern<String>) {
        let empty = Pattern::default();
        prop_assert_eq!(empty.combine(p.clone()), p);
    }
    
    #[test]
    fn test_right_identity(p: Pattern<String>) {
        let empty = Pattern::default();
        prop_assert_eq!(p.clone().combine(empty), p);
    }
}
```

**Test Coverage**:
- Atomic patterns (no elements)
- Patterns with elements
- Nested patterns
- Multiple value types (String, Vec<T>, (), i32)
- Edge cases: very deep nesting, many elements

---

### 4. Integration with Existing Pattern Operations

**Task**: Ensure Default implementation works seamlessly with existing pattern operations

**Existing Operations** (from previous features):
- `map()` - Functor (feature 008)
- `fold()` - Foldable (feature 009)
- `traverse_*()` - Traversable (feature 010)
- `any_value()`, `all_values()`, `filter()` - Query functions (feature 011)
- `combine()` - Semigroup (feature 013)

**Integration Points**:

1. **With combine()**: Default must act as identity
   ```rust
   let p = Pattern::point("test".to_string());
   assert_eq!(Pattern::default().combine(p.clone()), p);
   assert_eq!(p.clone().combine(Pattern::default()), p);
   ```

2. **With fold()**: Can use as initial value
   ```rust
   let result = patterns.into_iter()
       .fold(Pattern::default(), |acc, p| acc.combine(p));
   ```

3. **With map()**: Mapping over default preserves identity
   ```rust
   let empty = Pattern::<String>::default();
   let mapped = empty.map(|s| s.to_uppercase());
   assert_eq!(mapped, Pattern::default());
   ```

4. **With values()**: Default has single value (the default value)
   ```rust
   let empty = Pattern::<String>::default();
   let vals = empty.values();
   assert_eq!(vals, vec![""]);
   ```

---

### 5. Documentation Strategy

**Task**: Document monoid laws and usage patterns clearly

**Documentation Locations**:

1. **Trait Implementation Doc Comment**:
   ```rust
   /// Provides a default (identity) pattern for value types that implement `Default`.
   ///
   /// The default pattern has the default value and no elements, acting as an
   /// identity element for pattern combination:
   ///
   /// # Monoid Laws
   ///
   /// - **Left Identity**: `Pattern::default().combine(p) == p`
   /// - **Right Identity**: `p.combine(Pattern::default()) == p`
   ```

2. **Module Documentation**: Explain relationship to monoid algebra

3. **Usage Examples**: Show practical patterns with iterators

4. **Test Documentation**: Reference monoid laws in property test descriptions

---

### 6. Behavioral Equivalence Verification

**Task**: Ensure implementation matches gram-hs monoid semantics

**Verification Strategy**:
1. Create equivalent patterns in gram-hs and gram-rs
2. Verify identity laws hold in both implementations
3. Cross-check test cases from gram-hs test suite
4. Document any intentional deviations (e.g., using Default instead of custom trait)

**Key Equivalence Points**:
- Identity pattern structure (default value + empty elements)
- Identity laws hold identically
- Behavior with iterators/folds matches Haskell's fold with mempty

---

## Research Conclusions

1. **Use `std::default::Default` trait** - Most idiomatic approach for Rust
2. **Document monoid laws** - In doc comments and test descriptions
3. **Verify via property tests** - Comprehensive testing of identity laws
4. **Integrate with iterators** - Primary practical benefit
5. **Maintain behavioral equivalence** - Match gram-hs semantics while using Rust idioms

The implementation will provide the same functionality as Haskell's Monoid instance but expressed through Rust's standard library conventions rather than custom algebraic traits.

