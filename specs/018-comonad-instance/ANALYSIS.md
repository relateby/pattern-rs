# Feature 018: Comonad Instance - Analysis for Rust Port

**Date**: 2026-01-05  
**Status**: Under Review  
**Decision**: Pending

## Executive Summary

After reviewing the Haskell implementation and usage patterns, **I recommend DEFERRING the Comonad instance port** for the following reasons:

1. **Limited practical usage** - Only `depthAt` uses `extend`, other helpers use direct implementations
2. **All functionality already available** - pattern-rs already has `depth()`, `size()`, and `analyze_structure()`
3. **Helper functions don't require Comonad** - Can be implemented directly (as `sizeAt` and `indicesAt` demonstrate)
4. **Complex abstraction with unclear benefit** - Pattern as Comonad is theoretically interesting but practically unnecessary

However, there is a **stronger case for Comonad than Applicative**:
- ✅ Has concrete use cases (depth-at-position, size-at-position, indices-at-position)
- ✅ Used in tests and examples (though not in production code)
- ✅ Could enable interesting context-aware transformations

**Verdict**: Defer for now, but **reconsider if users request position-aware operations**.

## What Comonad Provides in Haskell

### Haskell Implementation

```haskell
instance Comonad Pattern where
  extract :: Pattern a -> a
  extract (Pattern v _) = v

  duplicate :: Pattern a -> Pattern (Pattern a)
  duplicate p@(Pattern _ es) = Pattern p (map duplicate es)

  extend :: (Pattern a -> b) -> Pattern a -> Pattern b
  extend f p@(Pattern _ es) = Pattern (f p) (map (extend f) es)
```

### Semantics

1. **`extract`**: Returns the root value of a pattern
   - `extract (Pattern v es) = v`
   - Simply accesses the `value` field

2. **`duplicate`**: Creates a pattern where each position contains the full subpattern rooted at that position
   - `duplicate p@(Pattern _ es) = Pattern p (map duplicate es)`
   - Root contains the full pattern
   - Each element position contains the subpattern at that position

3. **`extend`**: Applies a context-aware function to every position in the pattern
   - Takes a function `Pattern a -> b` that can "see" the entire subpattern at each position
   - Returns a pattern where each position's value is the result of applying the function to that subpattern

### Comonad Laws

Must satisfy:
1. **Left identity**: `extract . extend f = f`
2. **Right identity**: `extend extract = id`
3. **Associativity**: `extend f . extend g = extend (f . extend g)`

### Example

```haskell
-- Pattern structure
p = pattern "root" [pattern "a" [point "x"], point "b"]

-- depthAt uses extend to compute depth at each position
depthAt p = extend depth p
-- Result: Pattern 2 [Pattern 1 [Pattern 0 []], Pattern 0 []]
--   Root has depth 2, "a" has depth 1, "x" and "b" have depth 0
```

## Helper Functions Built on Comonad

gram-hs provides three helper functions:

### 1. `depthAt :: Pattern v -> Pattern Int`

```haskell
depthAt = extend depth
```

Returns a pattern where each position contains its maximum nesting depth.

**Uses Comonad**: ✅ Uses `extend`

### 2. `sizeAt :: Pattern v -> Pattern Int`

```haskell
sizeAt (Pattern _ es) =
  let subResults = map sizeAt es
      mySize = 1 + sum (map value subResults)
  in Pattern mySize subResults
```

Returns a pattern where each position contains the size of its subtree.

**Uses Comonad**: ❌ Direct recursive implementation, does NOT use `extend`

### 3. `indicesAt :: Pattern v -> Pattern [Int]`

```haskell
indicesAt = go []
  where
    go path (Pattern _ es) =
      Pattern path (zipWith (\i e -> go (path ++ [i]) e) [0..] es)
```

Returns a pattern where each position contains its path indices from root.

**Uses Comonad**: ❌ Direct recursive implementation, does NOT use `extend`

### Key Observation

**Only 1 of 3 helper functions uses Comonad operations!**

This suggests that:
- `extend` is theoretically elegant but not practically necessary
- Direct recursive implementations are often simpler and clearer
- The main value is enabling "compute X at every position" patterns

## Actual Usage in gram-hs

### Where is it Used?

**Test Coverage**: Extensive property-based tests for Comonad laws:
- Extract-extend law: `extract . extend f = f`
- Extend-extract law: `extend extract = id`
- Extend composition law: `extend f . extend g = extend (f . extend g)`
- Tests for `depthAt`, `sizeAt`, `indicesAt` with various pattern structures

**Production Usage**: ❌ **ZERO** production usage found
- Comonad operations only used in:
  1. Pattern/Core.hs - instance definition and helper functions
  2. Tests - law verification and helper function tests
- NOT used in:
  - Pattern/Graph.hs (graph operations)
  - Any other library code
  - Any application code

**Helper Function Usage**:
- `depthAt` - Used in 6 test cases
- `sizeAt` - Used in 3 test cases
- `indicesAt` - Used in 3 test cases
- `extend` directly - Used only in law tests
- `duplicate` directly - Used only in law tests

### Comparison with Existing pattern-rs Features

pattern-rs already has equivalent or better functionality:

| Haskell Comonad Operation | pattern-rs Equivalent | Status |
|---------------------------|-------------------|--------|
| `extract p` (get root value) | `p.value()` or `p.value` field | ✅ Already exists |
| `depth p` (max depth) | `p.depth()` | ✅ Already exists |
| `size p` (total nodes) | `p.size()` | ✅ Already exists |
| `depthAt p` (depth at each position) | ❌ Not implemented | Could add if needed |
| `sizeAt p` (size at each position) | ❌ Not implemented | Could add if needed |
| `indicesAt p` (path at each position) | ❌ Not implemented | Could add if needed |
| `extend f p` (context-aware map) | ❌ Not implemented | Theoretical use case |
| `duplicate p` (pattern of subpatterns) | ❌ Not implemented | Theoretical use case |

**Key Insight**: pattern-rs has all the **inspection** operations (depth, size, value access) but lacks **position-aware transformation** operations (depthAt, sizeAt, indicesAt).

## Use Case Analysis

### What Comonad Enables

**Position-aware transformations**: Compute a value at every position based on the subpattern at that position.

Examples:
1. **`depthAt`**: Show depth at every position
2. **`sizeAt`**: Show subtree size at every position
3. **`indicesAt`**: Show path from root at every position
4. **Custom**: Any function `Pattern<V> -> W` applied at every position

### When Would Users Need This?

**Potential use cases**:
1. **Visualization**: Annotate every node with metadata (depth, size, path)
2. **Analysis**: Compute structural properties at every position
3. **Debugging**: Show context information at every position
4. **Transformation**: Apply position-aware transformations

**Reality check**:
- ❌ No production usage in gram-hs
- ❌ Only used in tests to verify laws
- ❌ Helper functions can be implemented without Comonad (as `sizeAt` and `indicesAt` show)
- ⚠️ If users need these operations, they can be added as direct methods

## Idiomatic Rust Port Options

### Option 1: Skip Comonad, Add Helper Methods (Recommended)

Don't implement Comonad trait. Instead, add specific helper methods if/when needed:

```rust
impl<V> Pattern<V> {
    /// Returns the value at the root of the pattern.
    /// 
    /// Note: This is identical to accessing the `value` field directly.
    /// Provided for API completeness.
    pub fn extract(&self) -> &V {
        &self.value
    }
    
    /// Returns a pattern where each position contains its depth.
    ///
    /// The depth at a position is the maximum nesting depth of the
    /// subpattern rooted at that position.
    pub fn depth_at(&self) -> Pattern<usize> {
        Pattern {
            value: self.depth(),
            elements: self.elements.iter().map(|e| e.depth_at()).collect(),
        }
    }
    
    /// Returns a pattern where each position contains its subtree size.
    ///
    /// The size at a position is the total number of nodes in the
    /// subpattern rooted at that position.
    pub fn size_at(&self) -> Pattern<usize> {
        let sub_results: Vec<Pattern<usize>> = 
            self.elements.iter().map(|e| e.size_at()).collect();
        let my_size = 1 + sub_results.iter().map(|r| r.value).sum::<usize>();
        Pattern {
            value: my_size,
            elements: sub_results,
        }
    }
    
    /// Returns a pattern where each position contains its path from root.
    ///
    /// The path is represented as a vector of indices.
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

**Advantages**:
- ✅ Clear, specific operations that users can discover
- ✅ No complex trait abstractions
- ✅ Idiomatic Rust (direct methods)
- ✅ Easy to test and document
- ✅ Can add more helpers as needed

**Disadvantages**:
- ❌ No general `extend` operation for custom context-aware functions
- ❌ Each operation needs its own implementation

### Option 2: Implement Comonad Operations (Not Recommended)

```rust
impl<V: Clone> Pattern<V> {
    /// Extracts the root value (Comonad extract operation).
    pub fn extract(&self) -> &V {
        &self.value
    }
    
    /// Creates a pattern where each position contains the subpattern at that position.
    pub fn duplicate(&self) -> Pattern<Pattern<V>> {
        Pattern {
            value: self.clone(),
            elements: self.elements.iter().map(|e| e.duplicate()).collect(),
        }
    }
    
    /// Applies a context-aware function to every position.
    pub fn extend<W, F>(&self, f: F) -> Pattern<W>
    where
        F: Fn(&Pattern<V>) -> W + Clone,
    {
        Pattern {
            value: f(self),
            elements: self.elements.iter().map(|e| e.extend(f.clone())).collect(),
        }
    }
}

// Then implement helpers using extend
impl<V: Clone> Pattern<V> {
    pub fn depth_at(&self) -> Pattern<usize> {
        self.extend(|p| p.depth())
    }
}
```

**Advantages**:
- ✅ General `extend` operation for any context-aware function
- ✅ Matches Haskell implementation
- ✅ Enables custom position-aware transformations

**Disadvantages**:
- ❌ Requires `Clone` bound (for function cloning in recursion)
- ❌ More complex API that users must understand
- ❌ `duplicate` creates nested Pattern<Pattern<V>> which is awkward
- ❌ No clear use cases for general `extend` beyond the specific helpers
- ❌ Testing burden (3 comonad laws + edge cases)

### Option 3: Minimal Implementation (Middle Ground)

Add only `extract` as an alias, skip `duplicate` and `extend`, add specific helpers:

```rust
impl<V> Pattern<V> {
    /// Returns a reference to the root value.
    /// 
    /// This is equivalent to accessing the `value` field directly,
    /// provided for API completeness and consistency with Haskell's `extract`.
    pub fn extract(&self) -> &V {
        &self.value
    }
    
    // Add depth_at, size_at, indices_at as direct implementations
    // (as shown in Option 1)
}
```

**Advantages**:
- ✅ Provides `extract` for Haskell compatibility
- ✅ Adds useful helpers without complex abstractions
- ✅ Clear, discoverable API

**Disadvantages**:
- ❌ No general `extend` operation (but no clear need for it either)

## Challenges for Rust Port

1. **`Clone` requirement**: `extend` needs to clone the function for recursion
   - Could use `&F` and pass by reference, but complicates API

2. **`Pattern<Pattern<V>>`**: `duplicate` creates nested patterns
   - Type is awkward: `Pattern<Pattern<String>>` 
   - Unclear when users would want this

3. **Limited utility**: Only 1 of 3 helpers uses `extend`
   - Suggests direct implementations are clearer

4. **No production usage**: Zero usage beyond tests
   - Hard to justify implementation effort

5. **Testing burden**: 3 comonad laws + edge cases
   - Significant effort for unclear benefit

## Recommendations

### Primary Recommendation: DEFER, Add Helpers If Needed

**Do not implement Comonad at this time.**

Instead:
1. ✅ Keep existing `depth()`, `size()`, `value()` methods
2. ⏸️ Wait for user requests for position-aware operations
3. ✅ If requested, add specific helpers (`depth_at`, `size_at`, `indices_at`) as direct implementations
4. ❌ Skip general `extend` and `duplicate` unless concrete use cases emerge

**Rationale**:
- All inspection operations already exist
- Position-aware transformations have no proven use cases
- Direct implementations are clearer than Comonad abstraction
- Can add helpers later if users request them

### If Users Request Position-Aware Operations

If users specifically ask for "compute X at every position" operations:

1. **First, add specific helpers** (Option 1):
   - `depth_at()` - depth at each position
   - `size_at()` - size at each position
   - `indices_at()` - path at each position

2. **Then, evaluate if general `extend` is needed**:
   - Do users need custom position-aware functions?
   - Are there multiple use cases beyond the three helpers?
   - Would `extend` simplify the implementations?

3. **Only then, consider full Comonad**:
   - If `extend` proves useful, implement it
   - Skip `duplicate` unless users specifically need it
   - Document clearly with examples

### Alternative: Document Pattern

Add documentation showing how to implement position-aware operations:

```rust
/// # Position-Aware Operations
///
/// While Pattern doesn't implement a general Comonad trait, you can implement
/// position-aware operations using recursive methods:
///
/// ```rust
/// impl<V> Pattern<V> {
///     fn depth_at(&self) -> Pattern<usize> {
///         Pattern {
///             value: self.depth(),
///             elements: self.elements.iter().map(|e| e.depth_at()).collect(),
///         }
///     }
/// }
/// ```
///
/// This pattern can be adapted for any position-aware computation.
```

## Comparison: Comonad vs Applicative

| Aspect | Applicative | Comonad |
|--------|-------------|---------|
| **Production usage** | Zero | Zero |
| **Test usage** | Law tests only | Law tests + helper tests |
| **Concrete use cases** | None found | 3 helpers (depthAt, sizeAt, indicesAt) |
| **Helpers use abstraction** | N/A | Only 1 of 3 uses `extend` |
| **Rust implementation** | Awkward (function storage) | Moderate (Clone requirement) |
| **User value** | None (all covered by existing methods) | Low (helpers can be direct methods) |
| **Recommendation** | ⏸️ DEFER indefinitely | ⏸️ DEFER, add helpers if requested |

**Verdict**: Comonad has a **slightly stronger case** than Applicative because:
- ✅ Has concrete helper functions (even if not widely used)
- ✅ Position-aware operations could be useful for visualization/debugging
- ✅ Helpers are tested and documented in gram-hs

But still **not strong enough to justify immediate implementation**.

## Decision Required

**Should we implement Comonad instance for Pattern?**

- [ ] **Yes, implement full Comonad** - `extract`, `duplicate`, `extend` + helpers
- [ ] **Yes, but minimal** - Add `extract` and specific helpers only
- [x] **No, defer indefinitely** - Recommend this option
- [ ] **No, but add helpers later** - Add `depth_at`, `size_at`, `indices_at` if users request

## References

- **Haskell Implementation**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` (lines 720-728, 1104-1138)
- **Haskell Tests**: `../gram-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs` (lines 4242-4400)
- **Haskell Property Tests**: `../gram-hs/libs/pattern/tests/Spec/Pattern/Properties.hs` (lines 1287-1332)
- **Feature Spec**: `../gram-hs/specs/014-comonad-instance/spec.md`
- **Porting Guide**: `docs/porting-guide.md` - Idiomatic Rust principles
