# Feature 017: Applicative Instance - Port Evaluation

**Evaluation Date**: 2026-01-05  
**Status**: ⏸️ **DEFERRED - Not Recommended for Port**

## Quick Summary

After comprehensive analysis of the Haskell implementation and usage patterns, **I recommend DEFERRING this feature indefinitely**. The Applicative instance has zero practical usage in gram-hs and all potential use cases are better served by already-implemented features.

## Key Findings

### 1. No Practical Usage in gram-hs ❌

- ✅ Defined in `Pattern.Core` (70 lines of implementation + documentation)
- ✅ Tested for mathematical laws (115 lines of property tests)
- ❌ **Zero** production usage in gram-hs codebase
- ❌ **Zero** examples of practical applications
- ❌ **Zero** documentation beyond "it satisfies the laws"

**Conclusion**: Applicative is a theoretical convenience, not a practical necessity.

### 2. Complex and Counterintuitive Semantics ⚠️

The implementation uses **Cartesian product** semantics, not "zip-like" as the spec claims:

```
Given:  Pattern f [f1, f2] <*> Pattern x [x1, x2]
Result: Pattern (f x) [4 elements]  -- NOT 2!

The 4 elements are:
  1. f1 <*> Pattern x [x1, x2]
  2. f2 <*> Pattern x [x1, x2]
  3. Pattern f [f1, f2] <*> x1
  4. Pattern f [f1, f2] <*> x2
```

This can cause **exponential pattern growth** and is not what users expect.

### 3. All Use Cases Covered by Existing Features ✅

| What You Want | Applicative Way | Better Rust Way (Already Exists) |
|---------------|----------------|----------------------------------|
| Wrap value in pattern | `pure value` | ✅ `Pattern::point(value)` |
| Transform values | `pure f <*> p` | ✅ `p.map(f)` |
| Validation with effects | Pattern of validators | ✅ `p.traverse_result(validate)` |
| Combine patterns | `pure combine <*> p1 <*> p2` | ✅ `p1.combine(&p2)` |
| Aggregate values | `pure fold <*> p` | ✅ `p.fold(init, f)` |

### 4. Awkward in Rust 🦀❌

Problems porting to Rust:
- Functions aren't easily stored in data structures (not `Clone`, unique closure types)
- Would require `Box<dyn Fn>` → heap allocation + vtable indirection
- Extensive cloning needed for Cartesian product semantics
- API is confusing: `pattern_of_functions.apply(pattern_of_values)` is not intuitive

### 5. High Cost, No Benefit 💸

**Cost**:
- Complex implementation (function storage, cloning)
- Extensive testing (5 applicative laws + edge cases + functor consistency)
- Documentation of non-obvious semantics
- Ongoing maintenance

**Benefit**:
- None identified

## Detailed Documentation

For complete analysis, see:

1. **`ANALYSIS.md`** - Comprehensive technical analysis
   - Haskell implementation details
   - Usage patterns in gram-hs
   - Rust port challenges
   - Comparison with existing features

2. **`RECOMMENDATION.md`** - Executive summary and decision guide
   - Quick facts table
   - What Rust users need to know
   - Specific recommendation
   - Next steps

## Recommendation

### ⏸️ DEFER Indefinitely

**Do not implement Applicative at this time.**

Instead:
- ✅ Document that existing methods cover applicative-like use cases
- ✅ Keep the door open for future reconsideration
- ✅ Add specific operations (like `zip_with`) if concrete needs emerge

### If You Disagree

If you believe Applicative should be implemented despite this analysis, please provide:

1. **Concrete use case**: Code example showing what you want to do
2. **Why alternatives don't work**: Explanation of why map/traverse/fold/combine are insufficient
3. **User story**: Who needs this and why

Without these, proceeding with implementation would be premature optimization.

## Alternative: Minimal Implementation

If you absolutely need _something_ Applicative-like:

```rust
impl<V> Pattern<V> {
    /// Wraps a value in an atomic pattern.
    /// Alias for `Pattern::point` provided for consistency with Haskell's `pure`.
    pub fn pure(value: V) -> Self {
        Self::point(value)
    }
}
```

This provides the `pure` operation (though `point` already exists). Skip implementing `<*>` (apply) until concrete use cases emerge.

## Evaluation Checklist

Analysis completed:

- [x] Reviewed Haskell implementation (`Pattern/Core.hs:670-676`)
- [x] Reviewed test coverage (law tests only, no practical usage)
- [x] Searched for practical usage in gram-hs codebase (found none)
- [x] Compared with existing Rust features (all use cases covered)
- [x] Evaluated Rust implementation challenges (significant)
- [x] Assessed cost/benefit ratio (high cost, no benefit)
- [x] Documented findings and recommendation
- [x] Updated TODO.md with decision

## Next Steps

1. ✅ Review this analysis
2. ⏸️ Proceed to feature 018-comonad (needs similar evaluation)
3. 📋 Consider Phase 4 features (gram notation serialization) as next priority
4. 📝 Document applicative-like patterns using existing methods in user guide

## References

- Haskell Implementation: `../pattern-hs/libs/pattern/src/Pattern/Core.hs:670-676`
- Haskell Tests: `../pattern-hs/libs/pattern/tests/Spec/Pattern/Properties.hs:1075-1189`
- Feature Spec (outdated): `../pattern-hs/specs/013-applicative-instance/spec.md`
- Porting Guide: `../../docs/porting-guide.md`
- TODO: `../../TODO.md` (updated with DEFER status)
