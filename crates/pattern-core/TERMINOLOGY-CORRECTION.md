# Terminology Correction - Pattern Structure

**Date**: 2026-01-27  
**Issue**: Incorrect use of tree terminology (parent/child) for pattern structures

## Problem

The codebase was using tree-like terminology (parent, child, children) to describe pattern structures. This is conceptually incorrect because:

**Patterns are NOT trees:**
- A pattern is a **tuple** of `(value, elements)`
- The **value** is a **decoration** that describes or says something about the pattern
- The **elements ARE the pattern** (not children of it)

## Correct Conceptual Model

```
Pattern = (decoration, elements)

Where:
- decoration: A value that describes or says something about the pattern
- elements: List of patterns that form this pattern
```

**Example:**
```python
# Create atomic patterns (the elements)
elem1 = Pattern.point("a")
elem2 = Pattern.point("b")

# Decorate them with a value
# The value "list" DESCRIBES the pattern [elem1, elem2]
decorated = Pattern.pattern("list", [elem1, elem2])
```

The value "list" is not a parent - it's a **decoration** that says something about the pattern formed by the elements.

## Terminology Changes

### Removed Terms
- ❌ "parent" - implies hierarchical ownership
- ❌ "child"/"children" - implies parent-child relationship  
- ❌ "tree" - implies tree structure

### Correct Terms
- ✅ "decoration" or "value" - what describes the pattern
- ✅ "elements" - the patterns that form this pattern
- ✅ "atomic" - pattern with no elements
- ✅ "nested" - pattern with elements

## Files Updated

### Source Code
- `src/python.rs` - Docstrings updated
  - Changed "child Pattern instances" → "Pattern instances that form the pattern"
  - Changed "direct child patterns" → "elements"
  - Emphasized decoration concept

### Type Stubs
- `pattern_core/__init__.pyi` - All docstrings updated
  - Changed class docstrings to emphasize decoration
  - Updated method signatures and descriptions
  - Clarified that value "decorates or describes" the pattern

### Examples
- `examples/pattern-core-python/basic_usage.py`
  - `parent`/`child` → `decorated`/`elem`
  - Updated comments to explain decoration
  
- `examples/pattern-core-python/README.md`
  - Updated quickstart examples
  - Added comments about decoration
  
- `examples/pattern-core-python/operations.py`
  - Variable names updated
  - Comments clarified
  
- `examples/pattern-core-python/advanced.py`
  - Variable names updated (`child0` → `elem0`, etc.)

### Documentation
- `API-CHANGES.md`
  - Updated all examples
  - Changed "parent-child" → "value decoration"
  - Clarified "explicit decoration" principle

## Key Conceptual Points

### 1. Value as Decoration
The value doesn't "contain" or "own" the elements. It **decorates** them:

```python
# The value "user" DESCRIBES this pattern
user = Pattern.pattern("user", [
    Pattern.point("name: Alice"),
    Pattern.point("age: 30")
])
```

### 2. Elements ARE the Pattern
The elements form the pattern. They are not subordinate to the value:

```python
# These elements form a pattern
elements = [Pattern.point(1), Pattern.point(2), Pattern.point(3)]

# We can decorate this pattern with a value
numbered = Pattern.pattern("sequence", elements)

# The value "sequence" describes/decorates the pattern [1, 2, 3]
```

### 3. Not a Hierarchy
Patterns are not hierarchical structures with parents and children. They are decorated, nested structures where values describe patterns formed by elements.

## Examples of Correct Usage

### Before (Incorrect - Tree Language)
```python
# Create child patterns
child1 = Pattern.point("a")
child2 = Pattern.point("b")

# Create parent with children
parent = Pattern.pattern("root", [child1, child2])

# Access children
for child in parent.elements:
    print(child.value)
```

### After (Correct - Decoration Language)
```python
# Create atomic patterns (elements)
elem1 = Pattern.point("a")
elem2 = Pattern.point("b")

# Decorate elements with a value
# The value "root" describes this pattern
decorated = Pattern.pattern("root", [elem1, elem2])

# Access elements
for elem in decorated.elements:
    print(elem.value)
```

## Impact

### User-Facing Changes
- Variable names in examples use better terminology
- Documentation explains decoration concept
- Comments clarify that value describes/decorates

### API (No Changes)
- Method names unchanged (`Pattern.pattern`, etc.)
- Function signatures unchanged
- Behavior unchanged

### Understanding
- Clearer conceptual model
- Better alignment with s-expression semantics
- Avoids tree-thinking misconceptions

## Benefits

1. **Conceptual Accuracy**: Reflects actual semantics of patterns
2. **S-Expression Alignment**: Matches s-expression interpretation
3. **Clearer Intent**: "decoration" better describes the role of the value
4. **Avoids Misconceptions**: Tree-thinking leads to wrong assumptions
5. **Better Documentation**: Comments and docs now explain true structure

## Verification

All terminology has been systematically updated across:
- ✅ Source code docstrings
- ✅ Type stubs
- ✅ Examples (basic, operations, advanced)
- ✅ README and documentation
- ✅ API migration guides

The code compiles and all examples run correctly with the new terminology.

## References

This terminology correction aligns with the conceptual model where:
- Patterns are tuples `(value, elements)`
- Values are decorations that describe patterns
- Elements form the pattern (they ARE the pattern)
- Not a tree: no parent-child relationship

**Status**: Terminology correction complete ✅
