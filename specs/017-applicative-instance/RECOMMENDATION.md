# Feature 017: Applicative Instance - Port Recommendation

**Date**: 2026-01-05  
**Reviewer**: AI Assistant  
**Status**: ⏸️ **RECOMMEND DEFER**

## TL;DR

**Do not port the Applicative instance at this time.** It has zero practical usage in gram-hs, complex Cartesian product semantics, and all use cases are better served by already-implemented features (map, traverse, fold, combine).

## Quick Facts

| Aspect | Finding |
|--------|---------|
| **Practical usage in gram-hs** | None (only law verification tests) |
| **Rust implementation complexity** | High (function storage, cloning issues) |
| **User value added** | None (all use cases covered by existing methods) |
| **Testing burden** | High (5 applicative laws + edge cases) |
| **Idiomatic Rust fit** | Poor (requires awkward patterns) |
| **Recommendation** | ⏸️ DEFER indefinitely |

## What You Need to Know

### The Actual Implementation (Not What the Spec Says)

The Haskell implementation uses **Cartesian product semantics**, NOT the "zip-like" semantics described in the spec:

```haskell
-- Given:
-- Function pattern: Pattern f [f1, f2]
-- Value pattern: Pattern x [x1, x2]

-- Result has 4 elements (2 + 2), not 2!
Pattern (f x) [
  f1 <*> Pattern x [x1, x2],    -- each function elem applied to full value pattern
  f2 <*> Pattern x [x1, x2],
  Pattern f [f1, f2] <*> x1,    -- full function pattern applied to each value elem
  Pattern f [f1, f2] <*> x2
]
```

This can cause **exponential growth** in pattern size and is **not intuitive**.

### What Rust Users Already Have (Better Alternatives)

| Need | Applicative Way | Better Rust Way |
|------|----------------|-----------------|
| Transform values | `pure f <*> pattern` | `pattern.map(f)` ✅ |
| Validation | Pattern of validators | `pattern.traverse_result(validate)` ✅ |
| Combine patterns | `pure combine <*> p1 <*> p2` | `p1.combine(&p2)` ✅ |
| Aggregate | `pure fold <*> pattern` | `pattern.fold(init, f)` ✅ |

### Problems with Porting to Rust

1. **Function Storage**: Rust doesn't like storing functions in data structures
   - Need `Box<dyn Fn>` → heap allocation, vtable indirection
   - Functions aren't `Clone` → extensive workarounds needed
   - Closures have unique types → generic complexity

2. **Confusing API**: `pattern.apply(other_pattern)` is not intuitive
   - Users expect zip-like behavior, get Cartesian product
   - Can cause unexpected pattern size explosion
   - No obvious use cases where this is the best solution

3. **Maintenance Burden**: Need to test 5 applicative laws + edge cases
   - No practical benefit to justify this effort
   - gram-hs only tests laws, never uses it practically

## Recommendation

### ⏸️ DEFER - Do Not Implement

**Add this note to TODO.md:**

```markdown
### ⏸️ 017-applicative-instance: Applicative Trait (DEFERRED - Not Recommended)

**Status**: DEFERRED - Analysis shows no practical value for Rust users.

**Rationale**: 
- Zero practical usage in gram-hs (only law verification tests)
- All use cases better served by existing features (map, traverse, fold, combine)
- Complex Cartesian product semantics that don't match expectations
- Awkward in Rust (requires storing functions in patterns)
- High testing burden for no clear benefit

**Alternative**: Users can achieve applicative-like operations using existing methods:
- `Pattern::point(value)` for `pure`
- `pattern.map(f)` for single function application
- `pattern.traverse_result(f)` for validation workflows
- `pattern.fold(...)` for aggregations
- Custom methods for specific needs

**Reconsider if**: Concrete use cases emerge that cannot be solved with existing methods.

**See**: `specs/017-applicative-instance/ANALYSIS.md` for detailed analysis
```

### If You Still Want to Implement

If there's a strong reason to have Applicative despite this analysis:

1. **Start with `pure`**: Just add `Pattern::pure` as an alias to `Pattern::point`
2. **Wait for use cases**: Don't implement `apply` until concrete needs emerge
3. **Consider alternatives**: Implement `zip_with` or other specific operations instead
4. **Document clearly**: Warn about Cartesian product semantics and size explosion

## Next Steps

1. ✅ Read the detailed analysis: `specs/017-applicative-instance/ANALYSIS.md`
2. ✅ Update TODO.md to mark feature as DEFERRED with rationale
3. ⏸️ Move to next priority feature (018-comonad needs similar evaluation)
4. 📝 Consider documenting applicative-like patterns in user guide using existing methods

## Questions to Consider

Before proceeding with implementation, ask:

1. **Is there a concrete use case?** Can you write example code that benefits from this?
2. **Are existing methods insufficient?** Have you tried `map`, `traverse`, `fold`, `combine`?
3. **Is the complexity justified?** Function storage, cloning, law testing - worth it?
4. **Will Rust users understand it?** Cartesian product semantics are not obvious.

If you can't answer "yes" to all of these, defer the feature.

## See Also

- **Detailed Analysis**: `specs/017-applicative-instance/ANALYSIS.md`
- **Haskell Implementation**: `../gram-hs/libs/pattern/src/Pattern/Core.hs:670-676`
- **Porting Guide**: `docs/porting-guide.md` - Idiomatic Rust vs. literal translation
- **Existing Features**: Phase 3 features (008-016) provide better alternatives

