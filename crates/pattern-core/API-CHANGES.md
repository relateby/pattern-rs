# Python API Changes - Pattern Construction

**Date**: 2026-01-27  
**Version**: 0.2.0 (Breaking Changes)

## Summary

This document describes breaking changes made to the Python pattern-core API to improve clarity and follow functional programming conventions.

## Changes

### 1. Removed: `Pattern.from_list(value, values)`

**Old API (Removed):**
```python
# Confusing: takes both a root value and a list of values
pattern = Pattern.from_list("root", [1, 2, 3])
```

**Why Removed:**
- Confusing signature: name implies it takes only a list
- Hidden nesting: the `value` parameter wasn't obvious
- Not intuitive for users expecting a simple "list → pattern" conversion

### 2. Added: `Pattern.from_values(values) → List[Pattern]`

**New API:**
```python
# Clear: converts values to a list of atomic patterns
patterns = Pattern.from_values([1, 2, 3])
# Returns: [Pattern.point(1), Pattern.point(2), Pattern.point(3)]

# For nested structures, be explicit:
root = Pattern.pattern("root", Pattern.from_values([1, 2, 3]))
```

**Benefits:**
- ✅ Does ONE thing: converts values → patterns
- ✅ Name matches behavior: "from values" returns patterns
- ✅ Explicit nesting via `Pattern.pattern()`
- ✅ No hidden parameters
- ✅ Return type is clear (list of patterns)

**Type Signature:**
```python
@staticmethod
def from_values(values: List[Any]) -> List[Pattern]:
    """Convert a list of values into a list of atomic patterns."""
```

### 3. Added: `Pattern.of(value) → Pattern`

**New API:**
```python
# Alias for Pattern.point()
p1 = Pattern.point(42)
p2 = Pattern.of(42)
# Both create identical atomic patterns
```

**Benefits:**
- ✅ Follows functional programming convention
- ✅ Familiar to developers from other FP languages
- ✅ `of` typically lifts a value into a functor/applicative
- ✅ Zero runtime overhead (inlined alias)

**Type Signature:**
```python
@staticmethod
def of(value: Any) -> Pattern:
    """Alias for point(). Lift a value into a Pattern."""
```

## Migration Guide

### Migrating from `from_list`

**Before:**
```python
# Old API
pattern = Pattern.from_list("root", [1, 2, 3, 4, 5])
```

**After (Option 1 - Explicit structure):**
```python
# New API: explicit decoration
elements = Pattern.from_values([1, 2, 3, 4, 5])
pattern = Pattern.pattern("root", elements)
```

**After (Option 2 - Manual construction):**
```python
# Alternative: manual construction
pattern = Pattern.pattern("root", [
    Pattern.point(1),
    Pattern.point(2),
    Pattern.point(3),
    Pattern.point(4),
    Pattern.point(5)
])
```

**After (Option 3 - Just the list):**
```python
# If you don't need a root, just use the list
patterns = Pattern.from_values([1, 2, 3, 4, 5])
# Use patterns directly
```

### Using `Pattern.of()`

**New (preferred for FP style):**
```python
# Functional programming style
pattern = Pattern.of(42)
```

**Still works (original API):**
```python
# Original style still available
pattern = Pattern.point(42)
```

## Examples

### Creating a decorated pattern

```python
import pattern_core as pc

# Old way (removed):
# root = pc.Pattern.from_list("root", ["a", "b", "c"])

# New way (explicit decoration):
# The value decorates/describes the pattern represented by the elements
elements = pc.Pattern.from_values(["a", "b", "c"])
decorated = pc.Pattern.pattern("decoration", elements)

print(f"Decoration: {decorated.value}")
print(f"Elements: {decorated.length()}")
```

### Functional style

```python
import pattern_core as pc

# Using Pattern.of() for functor/applicative style
data = [1, 2, 3, 4, 5]

# Create patterns functionally
patterns = [pc.Pattern.of(x) for x in data]

# Or use the convenience method
patterns = pc.Pattern.from_values(data)

# Build tree
tree = pc.Pattern.pattern("numbers", patterns)
```

### Converting existing code

```python
import pattern_core as pc

# Before (BROKEN):
# def build_graph(decoration, values):
#     return pc.Pattern.from_list(decoration, values)

# After (FIXED):
def build_graph(decoration, values):
    elements = pc.Pattern.from_values(values)
    return pc.Pattern.pattern(decoration, elements)

# Usage remains similar:
graph = build_graph("users", [1, 2, 3, 4, 5])
```

## Rationale

### Why break compatibility?

1. **Clarity**: The old API was confusing to users
2. **Single Responsibility**: `from_values` does one thing well
3. **Explicit > Implicit**: Nesting is now explicit via `Pattern.pattern()`
4. **Standards**: `Pattern.of()` follows FP conventions
5. **Clean**: Better to break once than carry confusion forever

### Design Principles

The new API follows these principles:

1. **Names match behavior**: `from_values` returns patterns (plural)
2. **One function, one job**: No hidden structure creation
3. **Explicit decoration**: Use `Pattern.pattern()` to decorate elements with a value
4. **FP conventions**: `of` for lifting values
5. **Type clarity**: Return types match names

## Testing

All existing tests have been updated. Key test changes:

```python
# Before:
p = Pattern.from_list("root", [1, 2, 3])

# After:
p = Pattern.pattern("root", Pattern.from_values([1, 2, 3]))
```

## Documentation Updates

- ✅ Type stubs updated (`__init__.pyi`)
- ✅ Examples updated (`basic_usage.py`, `operations.py`)
- ✅ API documentation updated
- ✅ This migration guide created

## Questions?

For questions or issues with the migration:
1. Check examples in `examples/pattern-core-python/`
2. Review type stubs in `pattern_core/__init__.pyi`
3. See tests in `crates/pattern-core/tests/python/`

## Version History

- **v0.1.0**: Original API with `from_list(value, values)`
- **v0.2.0**: Breaking change - removed `from_list`, added `from_values` and `of`
