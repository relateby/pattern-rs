# Data Model: Python Pattern-Core Bindings

**Feature**: 024-python-pattern-core  
**Date**: 2026-01-27

## Overview

This document describes the Python data model for pattern-core bindings. The Python API exposes Rust Pattern and Subject types as Python classes with Pythonic interfaces.

## Core Types

### Pattern

A recursive, nested structure (s-expression-like) that can hold any value type.

**Python Class**: `pattern_core.Pattern`

**Attributes**:
- `value` (Any): The value component of the pattern
- `elements` (List[Pattern]): List of child patterns (empty for atomic patterns)

**Construction**:
- `Pattern.of(value)` → `Pattern`: Create atomic pattern (alias for point)
- `Pattern.point(value)` → `Pattern`: Create atomic pattern (no elements)
- `Pattern.pattern(value, elements)` → `Pattern`: Create pattern with elements
- `Pattern.from_values(values)` → `List[Pattern]`: Convert list of values to list of patterns (lift each via point)

**Inspection Methods**:
- `length()` → `int`: Number of direct elements
- `size()` → `int`: Total number of nodes in pattern
- `depth()` → `int`: Maximum nesting depth
- `is_atomic()` → `bool`: True if pattern has no elements
- `values()` → `List[Any]`: All values as flat list (pre-order traversal)

**Query Methods**:
- `any_value(predicate: Callable[[Any], bool])` → `bool`: Check if any value satisfies predicate
- `all_values(predicate: Callable[[Any], bool])` → `bool`: Check if all values satisfy predicate
- `filter(predicate: Callable[[Pattern], bool])` → `Pattern`: Extract matching subpatterns
- `find_first(predicate: Callable[[Pattern], bool])` → `Optional[Pattern]`: Find first matching subpattern
- `matches(other: Pattern)` → `bool`: Check if patterns have identical structure
- `contains(other: Pattern)` → `bool`: Check if pattern contains other as subpattern

**Transformation Methods**:
- `map(func: Callable[[Any], Any])` → `Pattern`: Transform values while preserving structure
- `fold(init: Any, func: Callable[[Any, Any], Any])` → `Any`: Fold over all values

**Combination Methods**:
- `combine(other: Pattern)` → `Pattern`: Combine two patterns associatively

**Comonad Methods**:
- `extract()` → `Any`: Extract value at current position
- `extend(func: Callable[[Pattern], Any])` → `Pattern`: Apply function to all contexts
- `depth_at()` → `Pattern[int]`: Decorate each position with depth
- `size_at()` → `Pattern[int]`: Decorate each position with subtree size
- `indices_at()` → `Pattern[List[int]]`: Decorate each position with path from root

**Validation**:
- `validate(rules: ValidationRules)` → `Result[None, ValidationError]`: Validate pattern structure

**Analysis**:
- `analyze_structure()` → `StructureAnalysis`: Analyze pattern structure

### PatternSubject

Specialized Pattern class for Pattern<Subject> with Subject-specific operations.

**Python Class**: `pattern_core.PatternSubject`

**Inherits**: All Pattern methods and attributes

**Additional Methods**:
- Subject-specific query methods (if any)
- Direct access to Subject properties via pattern.value

### Subject

Self-descriptive value type with identity, labels, and properties.

**Python Class**: `pattern_core.Subject`

**Attributes**:
- `identity` (str): Symbol identifier (string)
- `labels` (Set[str]): Set of label strings
- `properties` (Dict[str, Value]): Map of property names to Value instances

**Construction**:
- `Subject(identity: str, labels: Set[str] = None, properties: Dict[str, Value] = None)` → `Subject`

**Methods**:
- `add_label(label: str)` → `None`: Add a label
- `remove_label(label: str)` → `None`: Remove a label
- `has_label(label: str)` → `bool`: Check if label exists
- `get_property(name: str)` → `Optional[Value]`: Get property value
- `set_property(name: str, value: Value)` → `None`: Set property value
- `remove_property(name: str)` → `None`: Remove property

### Value

Enum representing property value types.

**Python Class**: `pattern_core.Value`

**Variants**:
- `Value.string(s: str)` → `Value`: String value
- `Value.int(i: int)` → `Value`: Integer value
- `Value.decimal(f: float)` → `Value`: Decimal/float value
- `Value.boolean(b: bool)` → `Value`: Boolean value
- `Value.symbol(s: str)` → `Value`: Symbol value
- `Value.array(items: List[Value])` → `Value`: Array of values
- `Value.map(items: Dict[str, Value])` → `Value`: Map of string to value
- `Value.range(lower: Optional[float], upper: Optional[float])` → `Value`: Numeric range
- `Value.measurement(value: float, unit: str)` → `Value`: Measurement with unit

**Automatic Conversion**:
Python native types automatically convert to Value:
- `str` → `Value.string()`
- `int` → `Value.int()`
- `float` → `Value.decimal()`
- `bool` → `Value.boolean()`
- `list` → `Value.array()` (recursive)
- `dict` → `Value.map()` (recursive)

**Methods**:
- `as_string()` → `str`: Extract string value (raises if not string)
- `as_int()` → `int`: Extract integer value (raises if not int)
- `as_decimal()` → `float`: Extract decimal value (raises if not decimal)
- `as_boolean()` → `bool`: Extract boolean value (raises if not boolean)
- `as_array()` → `List[Value]`: Extract array value (raises if not array)
- `as_map()` → `Dict[str, Value]`: Extract map value (raises if not map)

### Symbol

Wrapper around string representing an identifier.

**Python Type**: `str` (simplified - Symbol is just a string in Python)

**Note**: In Python, Symbol is represented as a plain string. The Symbol wrapper exists in Rust but is transparent in Python.

### ValidationRules

Configuration for pattern validation.

**Python Class**: `pattern_core.ValidationRules`

**Attributes**:
- `max_depth` (Optional[int]): Maximum allowed nesting depth
- `max_elements` (Optional[int]): Maximum allowed elements per pattern

**Construction**:
- `ValidationRules(max_depth: Optional[int] = None, max_elements: Optional[int] = None)` → `ValidationRules`

### ValidationError

Error raised when pattern validation fails.

**Python Exception**: `pattern_core.ValidationError` (extends `ValueError`)

**Attributes**:
- `message` (str): Error message
- `rule` (str): Name of violated rule
- `location` (Optional[str]): Location in pattern where violation occurred

### StructureAnalysis

Result of pattern structure analysis.

**Python Class**: `pattern_core.StructureAnalysis`

**Attributes**:
- `summary` (str): Human-readable summary
- `depth_distribution` (List[int]): Count of nodes at each depth (index = depth)
- `element_counts` (List[int]): Element counts at each level
- `nesting_patterns` (List[str]): Description of nesting patterns

## Type Relationships

```
Pattern
├── value: Any
└── elements: List[Pattern]  (recursive)

PatternSubject extends Pattern
├── value: Subject
└── elements: List[PatternSubject]

Subject
├── identity: str
├── labels: Set[str]
└── properties: Dict[str, Value]

Value (enum)
├── string(str)
├── int(int)
├── decimal(float)
├── boolean(bool)
├── symbol(str)
├── array(List[Value])  (recursive)
├── map(Dict[str, Value])  (recursive)
├── range(Optional[float], Optional[float])
└── measurement(float, str)
```

## Type Conversions

### Python → Rust

- `str` → `String`
- `int` → `i64` (or appropriate integer type)
- `float` → `f64`
- `bool` → `bool`
- `list` → `Vec<T>` (with element conversion)
- `dict` → `HashMap<String, T>` (with value conversion)
- `set` → `HashSet<String>` (for labels)

### Rust → Python

- `String` → `str`
- Integer types → `int`
- `f64` → `float`
- `bool` → `bool`
- `Vec<T>` → `list` (with element conversion)
- `HashMap<String, T>` → `dict` (with value conversion)
- `HashSet<String>` → `set` (for labels)

## Validation Rules

1. **Pattern Structure**:
   - Pattern must maintain recursive structure (elements are Patterns)
   - Value-element pairing must be preserved
   - No cycles allowed (patterns form a tree, not a graph)

2. **Subject Properties**:
   - Property names must be non-empty strings
   - Property values must be valid Value instances
   - Labels must be non-empty strings

3. **Value Types**:
   - Arrays can contain any Value types (including nested arrays/maps)
   - Maps can contain any Value types (including nested arrays/maps)
   - Ranges must have valid bounds (lower <= upper if both present)

4. **Python Callbacks**:
   - Map/filter functions must accept correct argument types
   - Functions must return correct return types
   - Functions must not raise exceptions (handled by PyO3)

## Edge Cases

1. **Empty Patterns**: `Pattern.point(value)` creates atomic pattern with empty elements list
2. **Deep Nesting**: Patterns with 100+ levels require stack overflow protection
3. **Large Patterns**: Patterns with 1000+ nodes must maintain performance
4. **None Values**: Python None converts to appropriate Rust Option::None
5. **Type Mismatches**: Type conversion errors raise TypeError with clear messages
6. **Circular References**: Not possible (Pattern is tree structure, not graph)

## Performance Considerations

- Pattern operations maintain O(n) complexity where n is number of nodes
- Python-Rust boundary crossing has minimal overhead (<2x native Rust)
- Large patterns (1000+ nodes) handled efficiently
- Deep nesting (100+ levels) requires careful stack management
