# Feature 018: Comonad Instance - Port Recommendation

**Date**: 2026-01-05 (Updated after conceptual discussion)  
**Implementation Date**: 2026-01-05  
**Reviewer**: AI Assistant  
**Status**: ✅ **COMPLETE - Implemented and Tested**

## TL;DR

**✅ COMPLETE**: Minimal Comonad (`extract` + `extend`) has been successfully implemented. Comonad is conceptually the right abstraction for Pattern's "decorated sequence" semantics where the value decorates the elements.

## Quick Facts

| Aspect | Finding |
|--------|---------|
| **Practical usage in gram-hs** | Helper functions only (depthAt, sizeAt, indicesAt) |
| **Production usage** | None (only in tests) |
| **Conceptual fit** | ✅ Excellent - matches Pattern's semantics |
| **Rust implementation complexity** | Low (straightforward, no Clone needed) |
| **User value added** | Medium (enables position-aware decorations) |
| **Testing burden** | Moderate (3 comonad laws + helper tests) |
| **Recommendation** | 🎯 IMPLEMENT minimal version (extract + extend) |

## Conceptual Insight: Why Comonad is Right

### Pattern's Semantics

```rust
Pattern { 
    value: "sonata",           // Information ABOUT the pattern
    elements: ["A", "B", "A"]  // The pattern itself
}
```

**Key insight**: The elements ARE the pattern. The value DECORATES those elements with information.

This is fundamentally different from:
- **List/Vec**: Values are the content
- **Tree**: Values and structure are both content
- **Pattern**: Structure is the content, value is decoration/metadata

### Why Comonad is the Right Fit

**Comonad is the only typeclass that treats both components (value + elements) as information:**

1. **`extract`**: Access the decorative information
   ```rust
   pattern.extract() -> "sonata"
   ```

2. **`extend`**: Compute new decorative information based on context
   ```rust
   pattern.extend(|p| p.depth()) -> Pattern with depth at each position
   ```

3. **Position-aware decorations**: Natural way to compute metadata at every position
   ```rust
   depth_at: Pattern<String> -> Pattern<usize>  // depth decorates each position
   size_at: Pattern<String> -> Pattern<usize>   // size decorates each position
   ```

### Comparison with Other Typeclasses

| Typeclass | How it treats Pattern | Fits Pattern Semantics? |
|-----------|----------------------|------------------------|
| **Functor** | Transform values (decorations) | ✅ Yes - but only decorations |
| **Foldable** | Aggregate values (decorations) | ✅ Yes - but only decorations |
| **Traversable** | Effectful value transformations | ✅ Yes - but only decorations |
| **Comonad** | **Compute new decorations based on context (the subpattern)** | ✅✅ **Perfect fit** |

**Only Comonad gives functions access to the full subpattern** at each position, enabling context-aware decorations.

## Implementation Recommendation

### 🎯 IMPLEMENT - Minimal Comonad

**Implement `extract` and `extend` now** as they are conceptually correct for Pattern's semantics.

**Skip `duplicate` for now** (no clear use case for `Pattern<Pattern<V>>`).

### Implementation Plan

```rust
impl<V> Pattern<V> {
    /// Extracts the decorative value (the information about the pattern).
    /// 
    /// Pattern semantics: The value provides information about the elements.
    /// This operation accesses that information.
    pub fn extract(&self) -> &V {
        &self.value
    }
    
    /// Computes new decorative information at each position.
    ///
    /// Takes a function that computes information about a subpattern,
    /// and applies it at every position to create new decorations.
    ///
    /// This is the key Comonad operation: it gives the function access
    /// to the full subpattern at each position, enabling context-aware
    /// computation of new decorative information.
    pub fn extend<W, F>(&self, f: &F) -> Pattern<W>
    where
        F: Fn(&Pattern<V>) -> W,
    {
        Pattern {
            value: f(self),
            elements: self.elements.iter().map(|e| e.extend(f)).collect(),
        }
    }
}
```

### Helper Functions Using `extend`

```rust
impl<V> Pattern<V> {
    /// Decorates each position with its depth (maximum nesting level).
    pub fn depth_at(&self) -> Pattern<usize> {
        self.extend(&|p| p.depth())
    }
    
    /// Decorates each position with its subtree size (total nodes).
    pub fn size_at(&self) -> Pattern<usize> {
        self.extend(&|p| p.size())
    }
    
    /// Decorates each position with its path from root.
    /// Note: Requires path tracking, cannot use extend directly.
    pub fn indices_at(&self) -> Pattern<Vec<usize>> {
        fn go<V>(path: Vec<usize>, pattern: &Pattern<V>) -> Pattern<Vec<usize>> {
            Pattern {
                value: path.clone(),
                elements: pattern.elements
                    .iter()
                    .enumerate()
                    .map(|(i, e)| {
                        let mut new_path = path.clone();
                        new_path.push(i);
                        go(new_path, e)
                    })
                    .collect(),
            }
        }
        go(vec![], self)
    }
}
```

## Why This is Better Than Direct Implementations

**Conceptual consistency**: Using `extend` makes it clear that these operations are computing decorative information based on context.

```rust
// With extend - conceptually clear
pub fn depth_at(&self) -> Pattern<usize> {
    self.extend(&|p| p.depth())  // "Decorate with depth"
}

// Direct implementation - obscures the concept
pub fn depth_at(&self) -> Pattern<usize> {
    Pattern {
        value: self.depth(),
        elements: self.elements.iter().map(|e| e.depth_at()).collect(),
    }
}
```

Both produce the same result, but `extend` makes the "decorative computation" semantics explicit.

## Original Analysis vs Updated Recommendation

### What Changed

**Original focus**: Practical usage patterns
- ❌ Zero production usage → defer
- ❌ Can implement directly → defer
- ⚠️ Limited use cases → defer

**Updated focus**: Conceptual fit
- ✅ Conceptually correct for Pattern's semantics → implement
- ✅ Makes decorative computation explicit → implement
- ✅ Not complex (extract trivial, extend straightforward) → implement
- ✅ Enables natural helper implementations → implement

### The Key Insight

**Pattern is fundamentally a "decorated sequence" where value decorates elements.**

Comonad is the typeclass that naturally expresses "compute decorations based on context."

Even with limited production usage, implementing Comonad:
1. **Documents the semantics** (value decorates elements)
2. **Provides the right abstraction** (context-aware decoration)
3. **Enables elegant helpers** (depthAt, sizeAt via extend)
4. **Is not complex** (straightforward implementation)

## Implementation Summary

- [x] Implement `extract` method (trivial - return reference to value) ✅
- [x] Implement `extend` method (straightforward recursive application) ✅
- [x] Implement `depth_at` using `extend` ✅
- [x] Implement `size_at` using `extend` ✅
- [x] Implement `indices_at` (direct implementation - needs path tracking) ✅
- [x] Port comonad law tests (extract-extend, extend-extract, associativity) ✅
- [x] Port helper function tests ✅
- [x] Document Pattern's "decorated sequence" semantics ✅
- [x] Add examples showing position-aware decorations ✅

**Implementation locations**:
- Core operations: `crates/pattern-core/src/pattern/comonad.rs`
- Helper functions: `crates/pattern-core/src/pattern/comonad_helpers.rs`
- Law tests: `crates/pattern-core/tests/comonad_laws.rs`
- Example: `crates/pattern-core/examples/comonad_usage.rs`
- Documentation: Module-level docs in all source files

## Implementation Notes

### Why Only 1 of 3 Helpers Uses `extend` in gram-hs

In gram-hs:
- `depthAt = extend depth` ✅ Uses Comonad
- `sizeAt` ❌ Direct recursive implementation
- `indicesAt` ❌ Direct recursive implementation

**For pattern-rs, we'll use `extend` where conceptually appropriate:**
- `depth_at()` - Use `extend` (like gram-hs)
- `size_at()` - Use `extend` (for conceptual consistency)
- `indices_at()` - Direct implementation (needs path tracking)

This makes the decorative computation pattern explicit while still being pragmatic where needed.

## Comparison: Comonad vs Applicative

| Aspect | Applicative | Comonad |
|--------|-------------|---------|
| **Production usage** | ❌ Zero | ❌ Zero |
| **Concrete use cases** | ❌ None | ✅ 3 helpers |
| **Conceptual fit** | ❌ Poor | ✅✅ **Excellent** |
| **Implementation complexity** | High (function storage) | Low (straightforward) |
| **Recommendation** | ⏸️ Defer indefinitely | 🎯 **Implement now** |

**Verdict**: Comonad is conceptually correct for Pattern's semantics, making it worth implementing despite limited production usage.

## Completion Status

1. ✅ Update TODO.md (complete)
2. ✅ Implement `extract` and `extend` (complete)
3. ✅ Implement helper functions using `extend` (complete)
4. ✅ Port and verify tests (complete - all tests passing)
5. ✅ Document Pattern's decorated sequence semantics (complete)
6. ⏸️ Skip `duplicate` (deferred - no concrete use cases)

**All tests passing**: Property-based tests verify all three Comonad laws hold for arbitrary patterns.  
**Code quality**: Passes `cargo clippy` with no warnings, formatted with `cargo fmt`.  
**Documentation**: Comprehensive module-level docs, doc examples, and practical usage examples.

## See Also

- **Detailed Analysis**: `specs/018-comonad-instance/ANALYSIS.md` (original analysis)
- **Haskell Implementation**: `../pattern-hs/libs/pattern/src/Pattern/Core.hs:720-728, 1104-1138`
- **TODO**: `../../TODO.md` (updated with new recommendation)
- **README**: `specs/018-comonad-instance/README.md` (overview)
