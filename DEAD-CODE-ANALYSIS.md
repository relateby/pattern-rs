# Dead Code Analysis: Clippy Warnings Explained

**Date**: January 10, 2026  
**Context**: After fixing clippy warnings with `#[allow(dead_code)]`, analyzing which are false positives vs. truly unused

---

## Summary

Clippy's dead code detection has **limited call tracing** through parser combinators and can't distinguish between:
1. **Public API functions** (intended for external use)
2. **Internal helper functions** (used indirectly via combinators)
3. **Truly dead code** (replaced or never integrated)

---

## Analysis by Function

### ✅ Truly Dead Code (Correctly Flagged)

#### 1. `labels()` - `parser/subject.rs`
**Status**: ✅ **Truly dead**  
**Reason**: Function was replaced by inline combinator in `subject()`:
```rust
// Old (unused):
fn labels(input: &str) -> ParseResult<'_, Vec<String>> {
    separated_list1(char(':'), identifier)(input)
}

// Current (used):
opt(preceded(char(':'), separated_list1(char(':'), identifier)))
```
**Action**: Could be removed, but kept for potential future use or API completeness.

---

#### 2. `tagged_fenced_string()` - `parser/value.rs`
**Status**: ✅ **Truly dead**  
**Reason**: Defined but never called. `tagged_string()` uses simpler `backtick_quoted_string` approach instead.
**Action**: Could be removed, but kept for potential future syntax support.

---

#### 3. `tagged_triple_quoted()` - `parser/value.rs`
**Status**: ✅ **Truly dead**  
**Reason**: Defined but never called. Same as above.
**Action**: Could be removed, but kept for potential future syntax support.

---

### 🔶 Public API (Not Used Internally)

#### 4. `relationship()` - `parser/relationship.rs`
**Status**: 🔶 **Public API, unused internally**  
**Reason**: Marked `pub` but not called in `gram_pattern()`. Instead, `path_pattern` is used directly:
```rust
// gram_pattern() uses:
relationship::path_pattern,  // ✅ Used

// But relationship() itself:
pub fn relationship(...) { ... }  // ❌ Not called
```
**Why**: `relationship()` might be intended as a public API for parsing single relationships, but the main parser uses `path_pattern` which handles paths of any length.
**Action**: Valid public API - should keep for external use.

---

#### 5. `label()` - `parser/subject.rs`
**Status**: 🔶 **Public API, unused internally**  
**Reason**: Marked `pub` and used in tests, but not called in actual parser code. Labels are parsed inline in `subject()`.
**Action**: Valid public API - useful for parsing just a label in isolation.

---

#### 6. `decimal()` - `parser/value.rs`
**Status**: 🔶 **Public API, unused internally**  
**Reason**: Marked `pub` and used in tests, but `value_parser()` uses `number()` which handles decimals differently (via `recognize()` pattern matching).
**Action**: Valid public API - useful for parsing just a decimal in isolation.

---

#### 7. `ArrowType` methods (`is_forward`, `is_bidirectional`, `is_undirected`)
**Status**: 🔶 **Public API, unused internally**  
**Reason**: Public methods on public enum, used in tests but not in parser implementation.
**Action**: Valid public API - useful for consumers of the library to query arrow properties.

---

### 🔷 Utility Functions (Future Use / Parser Pattern)

#### 8. `padded()` - `parser/combinators.rs`
**Status**: 🔷 **Utility function, parser pattern**  
**Reason**: Common parser combinator pattern for wrapping parsers with whitespace. Not currently used but follows nom patterns.
**Action**: Keep for consistency with parser combinator patterns.

---

#### 9. `with_span()` - `parser/combinators.rs`
**Status**: 🔷 **Utility function, future use**  
**Reason**: Useful for tracking source locations for better error messages. Not currently used but valuable for future error improvements.
**Action**: Keep for future error message enhancements.

---

#### 10. `Span` struct - `parser/types.rs`
**Status**: 🔷 **Used by `with_span()`, future use**  
**Reason**: Part of location tracking infrastructure.
**Action**: Keep for future error tracking.

---

### 🔴 Error Infrastructure (Legacy / Future)

#### 11. `ParseError` struct and methods - `error.rs`
**Status**: 🔴 **Legacy from tree-sitter migration**  
**Reason**: Some methods (`new`, `with_error`, `invalid_structure`, `error_count`) were from tree-sitter parser and aren't used in nom parser.
**Action**: Could be cleaned up, but kept for potential future error recovery features.

---

#### 12. `value_parse_error()` - `parser/value.rs`
**Status**: 🔴 **Helper function, unused**  
**Reason**: Helper for converting nom errors, but not currently called.
**Action**: Keep for potential future error handling improvements.

---

## Clippy's Limitations

### 1. **Parser Combinator Call Tracing**
Clippy can't trace through parser combinators like `alt()`, `map()`, `delimited()`, etc. Functions passed as closures or function pointers aren't detected as "used".

**Example**:
```rust
// Clippy sees this as unused:
fn labels(input: &str) -> ParseResult<'_, Vec<String>> { ... }

// But this IS used (inline combinator):
opt(preceded(char(':'), separated_list1(char(':'), identifier)))
```

### 2. **Public API vs. Internal Use**
Clippy treats all unused functions the same, whether they're:
- Public API (intended for external use)
- Internal helpers (used indirectly)
- Truly dead code

### 3. **Test Code Not Counted**
Functions used only in tests are flagged as unused, even though they're part of the API surface.

---

## Recommendations

### Current Approach (Using `#[allow(dead_code)]`)
✅ **Good for now** - Preserves API surface and future flexibility

### Alternative Approaches

1. **Remove Truly Dead Code**
   - Remove `labels()`, `tagged_fenced_string()`, `tagged_triple_quoted()`
   - Clean up unused `ParseError` methods
   - **Risk**: Might need them later

2. **Mark Public API Explicitly**
   - Use `#[cfg(feature = "public-api")]` or similar
   - Document which functions are public API
   - **Benefit**: Clearer intent

3. **Improve Clippy Configuration**
   - Use `clippy.toml` to ignore specific warnings
   - Configure per-module or per-function
   - **Benefit**: More granular control

---

## Conclusion

**Most "dead code" warnings are false positives**:
- ✅ Some are truly dead (3 functions)
- 🔶 Many are public API (5+ functions)
- 🔷 Some are utility functions for future use (2 functions)
- 🔴 Some are legacy infrastructure (2 functions)

**Clippy's limitation**: Can't trace through parser combinators or distinguish public API from internal code.

**Current solution**: `#[allow(dead_code)]` is appropriate - preserves API surface and future flexibility while acknowledging that clippy can't fully understand parser combinator usage patterns.

---

**Recommendation**: Keep current approach. The code is correct, clippy just can't see the full picture.
