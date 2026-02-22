# Feature 017: Applicative Instance - Analysis for Rust Port

**Date**: 2026-01-05  
**Status**: Under Review  
**Decision**: Pending

## Executive Summary

After reviewing the Haskell implementation and test suite, **I recommend DEFERRING the Applicative instance port** for the following reasons:

1. **No practical usage found** in the gram-hs codebase beyond law verification tests
2. **Complex Cartesian product semantics** that don't match the "zip-like" description in the spec
3. **Limited utility for Rust users** compared to already-implemented features (Functor, Traversable, Foldable)
4. **Better alternatives exist** in Rust's standard library and ecosystem for similar use cases

## What Applicative Provides in Haskell

### Haskell Implementation

The Haskell Applicative instance for Pattern is defined as:

```haskell
instance Applicative Pattern where
  pure :: a -> Pattern a
  pure x = Pattern x []

  (<*>) :: Pattern (a -> b) -> Pattern a -> Pattern b
  (Pattern f fs) <*> (Pattern x xs) = 
    Pattern (f x) (map (<*> Pattern x xs) fs ++ map (Pattern f fs <*>) xs)
```

### Semantics

1. **`pure x`**: Creates an atomic pattern `Pattern x []` (pattern with value `x` and no elements)

2. **`<*>` (apply)**: Takes a pattern of functions and a pattern of values:
   - Applies root function `f` to root value `x` to produce the result root: `f x`
   - For elements, creates a **Cartesian product-like structure**:
     - Each function element in `fs` is applied to the entire value pattern `Pattern x xs`
     - The entire function pattern `Pattern f fs` is applied to each value element in `xs`
     - These are concatenated together

### Example

Given:
- Function pattern: `Pattern f [f1, f2]`
- Value pattern: `Pattern x [x1, x2]`

Result:
```
Pattern (f x) [
  f1 <*> Pattern x [x1, x2],    -- from fs
  f2 <*> Pattern x [x1, x2],    -- from fs
  Pattern f [f1, f2] <*> x1,    -- from xs
  Pattern f [f1, f2] <*> x2     -- from xs
]
```

**This produces 4 elements (2 + 2), NOT a zip operation!**

### Discrepancy with Spec

The feature spec (`specs/013-applicative-instance/spec.md` in gram-hs) describes the behavior as:

> **FR-003**: System MUST provide `<*>` operator that applies a pattern of functions to a pattern of values using **structure-preserving/zip-like semantics**

**This is incorrect.** The actual implementation uses **Cartesian product semantics**, not zip-like semantics. The spec appears to be outdated or was never fully aligned with the implementation.

## Actual Usage in gram-hs

### Test Coverage

The gram-hs codebase includes property-based tests for Applicative laws:
- Identity law: `pure id <*> v = v`
- Composition law: `pure (.) <*> u <*> v <*> w = u <*> (v <*> w)`
- Homomorphism law: `pure f <*> pure x = pure (f x)`
- Interchange law: `u <*> pure y = pure ($ y) <*> u`
- Functor consistency: `fmap f x = pure f <*> x`

### Practical Usage

**Zero practical usage found.** The Applicative instance is:
- ✅ Defined in `Pattern.Core`
- ✅ Tested for mathematical laws
- ❌ **NOT used** in any production code in gram-hs
- ❌ **NO examples** of practical applications
- ❌ **NO documentation** of use cases beyond law verification

This suggests Applicative is a **theoretical convenience** rather than a practical necessity.

## Utility for Rust Users

### What Rust Users Gain

If ported, Rust users would get:
1. `Pattern::pure(value)` - wraps a value in an atomic pattern (already available via `Pattern::point`)
2. `Pattern::apply(func_pattern, value_pattern)` - applies pattern of functions to pattern of values

### What Rust Users Already Have

More useful operations already implemented:
1. **`Pattern::map`** (Functor) - Transform all values while preserving structure
2. **`Pattern::traverse_option`** / **`Pattern::traverse_result`** (Traversable) - Effectful transformations with short-circuiting
3. **`Pattern::fold`** (Foldable) - Aggregate values into a single result
4. **`Pattern::any_value`** / **`Pattern::all_values`** - Query operations with short-circuiting
5. **`Pattern::filter`** - Extract subpatterns matching predicates
6. **`Pattern::combine`** (Semigroup) - Merge patterns associatively

### Use Case Analysis

Common scenarios where Applicative might seem useful:

| Scenario | Applicative Approach | Better Rust Alternative |
|----------|---------------------|------------------------|
| Transform values | `pure f <*> pattern` | ✅ `pattern.map(f)` (already exists) |
| Validation with effects | Pattern of validators | ✅ `pattern.traverse_result(validate)` (already exists) |
| Combining patterns | `pure combine <*> p1 <*> p2` | ✅ `p1.combine(&p2)` (already exists) |
| Parallel operations | Pattern of operations | ✅ Use `rayon` with `par_iter()` |
| Zip two patterns | `pure tuple <*> p1 <*> p2` | ❌ Applicative doesn't zip! Use custom method |

**Conclusion**: There are no compelling use cases where Applicative provides value beyond existing methods.

## Idiomatic Rust Port Options

If we were to port this feature, here are the options:

### Option 1: Direct Methods (Recommended if porting)

```rust
impl<V> Pattern<V> {
    /// Wraps a value in an atomic pattern (equivalent to `pure` in Haskell).
    /// 
    /// Note: This is identical to `Pattern::point` and provided for conceptual completeness.
    pub fn pure(value: V) -> Self {
        Pattern::point(value)
    }
}

impl<V, W> Pattern<Fn(&V) -> W> {
    /// Applies a pattern of functions to a pattern of values using Cartesian product semantics.
    ///
    /// Warning: This operation creates a Cartesian product of elements, which can lead to
    /// exponential growth in pattern size. Use with caution.
    pub fn apply<F>(self, values: Pattern<V>) -> Pattern<W>
    where
        F: Fn(&V) -> W,
    {
        // Implementation following Haskell semantics
        let result_value = (self.value)(&values.value);
        
        let mut result_elements = Vec::new();
        
        // Apply each function element to the entire value pattern
        for func_elem in self.elements {
            result_elements.push(func_elem.apply(values.clone()));
        }
        
        // Apply the entire function pattern to each value element
        for val_elem in values.elements {
            result_elements.push(self.clone().apply(val_elem));
        }
        
        Pattern {
            value: result_value,
            elements: result_elements,
        }
    }
}
```

**Issues with this approach:**
- ❌ Requires `Pattern<Fn(&V) -> W>` which is awkward in Rust (functions aren't typically stored in data structures)
- ❌ Requires extensive cloning due to Cartesian product semantics
- ❌ Can cause exponential pattern growth (not obvious from API)
- ❌ No clear use cases where this is preferable to `map`, `traverse`, or custom logic

### Option 2: Trait-Based (Not Recommended)

```rust
trait Applicative<A> {
    type Output<B>;
    fn pure(value: A) -> Self;
    fn apply<B, F>(self, values: Self::Output<A>) -> Self::Output<B>
    where
        F: Fn(&A) -> B;
}

impl<V> Applicative<V> for Pattern<V> {
    type Output<B> = Pattern<B>;
    // ... implementation
}
```

**Issues:**
- ❌ Requires Higher-Kinded Types (HKTs), which Rust doesn't support
- ❌ Would need GATs (Generic Associated Types) which add complexity
- ❌ Not idiomatic Rust - trait hierarchies should be avoided unless truly necessary

### Option 3: Custom Applicative-Like Operations (Alternative)

Instead of porting Applicative directly, implement specific operations that users actually need:

```rust
impl<V> Pattern<V> {
    /// Zips two patterns element-wise, applying a function to matching pairs.
    /// Uses truncation semantics - stops at the shorter pattern's length.
    pub fn zip_with<W, R, F>(self, other: Pattern<W>, f: F) -> Pattern<R>
    where
        F: Fn(&V, &W) -> R + Clone,
    {
        let result_value = f(&self.value, &other.value);
        
        let result_elements = self.elements
            .iter()
            .zip(other.elements.iter())
            .map(|(e1, e2)| e1.clone().zip_with(e2.clone(), f.clone()))
            .collect();
        
        Pattern {
            value: result_value,
            elements: result_elements,
        }
    }
    
    /// Applies a transformation to all values, using context from a matching pattern.
    pub fn map_with_context<W, R, F>(self, context: &Pattern<W>, f: F) -> Pattern<R>
    where
        F: Fn(&V, &W) -> R + Clone,
    {
        // Similar to zip_with but keeps full structure of `self`
        // ... implementation
    }
}
```

**Advantages:**
- ✅ Provides concrete, understandable operations
- ✅ Clear semantics (zip vs Cartesian product)
- ✅ Idiomatic Rust (no function-storing patterns needed)
- ✅ Can be added later if use cases emerge

## Challenges for Rust Port

1. **Function Storage**: Rust doesn't handle storing functions in data structures well
   - Functions aren't `Clone` by default
   - Closures have unique types
   - Would need `Box<dyn Fn>` or similar, adding runtime overhead

2. **Cartesian Product Semantics**: The actual semantics are complex and non-obvious
   - Can cause exponential growth in pattern size
   - Not what users expect from "applying functions to values"
   - Different from spec description

3. **Limited Utility**: All practical use cases are better served by existing methods
   - `map` for transformations
   - `traverse` for effectful operations
   - `fold` for aggregations
   - Custom methods for specific needs

4. **Testing Burden**: Would require extensive property-based testing for laws
   - Identity, composition, homomorphism, interchange laws
   - Functor consistency
   - Edge cases with nested patterns
   - No practical benefit to justify this effort

## Recommendations

### Primary Recommendation: DEFER

**Do not implement Applicative at this time.**

**Rationale:**
1. Zero practical usage in gram-hs beyond law verification
2. Complex semantics that don't match common expectations
3. All use cases better served by existing methods
4. High implementation and testing burden for no clear benefit
5. Can be added later if concrete use cases emerge

### If Future Need Arises

If concrete use cases emerge that require Applicative-like operations:

1. **First, try existing methods**: Can `map`, `traverse`, `fold`, or `combine` solve it?
2. **Then, add specific operations**: Implement `zip_with` or similar for specific needs
3. **Only then, consider Applicative**: If multiple specific operations follow Applicative pattern

### Alternative: Document Equivalence

Add a documentation section explaining how to achieve Applicative-like operations:

```rust
/// # Applicative-Like Operations
///
/// While Pattern doesn't implement a general Applicative trait, you can achieve
/// similar results using existing methods:
///
/// - `pure` → Use `Pattern::point(value)`
/// - `fmap` → Use `pattern.map(f)`  
/// - `<*>` with atomic patterns → Use `pattern.map(f)` where f captures context
/// - Validation workflows → Use `pattern.traverse_result(validator)`
///
/// For use cases not covered by these methods, consider opening an issue with
/// your specific requirements.
```

## Comparison with Other Features

| Feature | Status | Practical Usage | Rust Idiomaticity | Priority |
|---------|--------|----------------|-------------------|----------|
| Functor (map) | ✅ Implemented | High - transformations everywhere | ✅ Idiomatic | Critical |
| Foldable | ✅ Implemented | High - aggregations, queries | ✅ Idiomatic | Critical |
| Traversable | ✅ Implemented | High - validation, effects | ✅ Idiomatic | High |
| Semigroup | ✅ Implemented | Medium - pattern merging | ✅ Idiomatic | Medium |
| Monoid | ✅ Implemented | Medium - identity patterns | ✅ Idiomatic | Medium |
| **Applicative** | ❌ Not implemented | **None found** | ⚠️ **Awkward** | **Low** |
| Comonad | ❌ Not implemented | Unknown | ⚠️ Theoretical | Unknown |

## Decision Required

**Should we implement Applicative instance for Pattern?**

- [ ] **Yes, implement now** - Despite limited utility, for completeness
- [x] **No, defer indefinitely** - Recommend this option
- [ ] **No, but add specific operations** - Add `zip_with` etc. if use cases emerge
- [ ] **Document equivalence only** - Show how to achieve Applicative-like behavior with existing methods

## References

- **Haskell Implementation**: `../pattern-hs/libs/pattern/src/Pattern/Core.hs` (lines 670-676)
- **Haskell Tests**: `../pattern-hs/libs/pattern/tests/Spec/Pattern/Properties.hs` (lines 1075-1189)
- **Feature Spec**: `../pattern-hs/specs/013-applicative-instance/spec.md` (Note: spec semantics don't match implementation)
- **Porting Guide**: `docs/porting-guide.md` - Idiomatic Rust principles
