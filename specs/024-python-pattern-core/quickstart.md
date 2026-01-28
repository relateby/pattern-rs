# Quickstart: Python Pattern-Core Bindings

**Feature**: 024-python-pattern-core  
**Date**: 2026-01-27

## Installation

### From Wheel (Local Development)

```bash
# Build the wheel
cd crates/pattern-core
maturin build --release --features python

# Install the wheel
pip install ../../target/wheels/pattern_core-*.whl
```

### From Source

```bash
# Install maturin
pip install maturin

# Build and install
cd crates/pattern-core
maturin develop --uv --features python
```

## Quick Examples

### 1. Create Atomic Pattern

```python
import pattern_core

# Create atomic pattern (no elements)
atomic = pattern_core.Pattern.point("hello")
print(atomic.value)  # "hello"
print(atomic.elements)  # []
print(atomic.is_atomic())  # True
```

### 2. Create Nested Pattern

```python
import pattern_core

# Create nested pattern
child1 = pattern_core.Pattern.point("child1")
child2 = pattern_core.Pattern.point("child2")
parent = pattern_core.Pattern.pattern("parent", [child1, child2])

print(parent.value)  # "parent"
print(parent.length())  # 2
print(parent.depth())  # 1
print(parent.elements[0].value)  # "child1"
```

### 3. Create Pattern with Subject

```python
import pattern_core

# Create Subject
subject = pattern_core.Subject(
    identity="alice",
    labels={"Person", "Employee"},
    properties={
        "name": pattern_core.Value.string("Alice"),
        "age": pattern_core.Value.int(30)
    }
)

# Create Pattern with Subject
pattern = pattern_core.PatternSubject.point(subject)

print(pattern.value.identity)  # "alice"
print(pattern.value.labels)  # {"Person", "Employee"}
print(pattern.value.properties["name"].as_string())  # "Alice"
```

### 4. Transform Patterns (Map)

```python
import pattern_core

# Create pattern
pattern = pattern_core.Pattern.pattern("hello", [
    pattern_core.Pattern.point("world"),
    pattern_core.Pattern.point("python")
])

# Transform values to uppercase
upper = pattern.map(str.upper)

print(upper.value)  # "HELLO"
print(upper.elements[0].value)  # "WORLD"
```

### 5. Filter Patterns

```python
import pattern_core

# Create nested pattern
pattern = pattern_core.Pattern.pattern("root", [
    pattern_core.Pattern.point("a"),
    pattern_core.Pattern.point("b"),
    pattern_core.Pattern.point("c")
])

# Filter patterns with value starting with "a"
filtered = pattern.filter(lambda p: p.value.startswith("a"))

print(filtered.length())  # 1
print(filtered.elements[0].value)  # "a"
```

### 6. Combine Patterns

```python
import pattern_core

# Create two patterns
p1 = pattern_core.Pattern.point("hello")
p2 = pattern_core.Pattern.point(" world")

# Combine them
combined = p1.combine(p2)

print(combined.value)  # "hello world"
```

### 7. Structural Analysis

```python
import pattern_core

# Create nested pattern
pattern = pattern_core.Pattern.pattern("root", [
    pattern_core.Pattern.pattern("a", [
        pattern_core.Pattern.point("b")
    ]),
    pattern_core.Pattern.point("c")
])

# Analyze structure
analysis = pattern.analyze_structure()

print(analysis.summary)  # "Pattern with 4 nodes, depth 2"
print(analysis.depth)  # 2
print(analysis.size)  # 4
```

### 8. Comonad Operations

```python
import pattern_core

# Create pattern
pattern = pattern_core.Pattern.pattern("root", [
    pattern_core.Pattern.point("a"),
    pattern_core.Pattern.point("b")
])

# Decorate with depth
depths = pattern.depth_at()

print(depths.value)  # 0 (root depth)
print(depths.elements[0].value)  # 1 (child depth)
```

### 9. Value Types

```python
import pattern_core

# Create various Value types
str_val = pattern_core.Value.string("hello")
int_val = pattern_core.Value.int(42)
float_val = pattern_core.Value.decimal(3.14)
bool_val = pattern_core.Value.boolean(True)
array_val = pattern_core.Value.array([
    pattern_core.Value.string("a"),
    pattern_core.Value.int(1)
])
map_val = pattern_core.Value.map({
    "name": pattern_core.Value.string("Alice"),
    "age": pattern_core.Value.int(30)
})

# Automatic conversion from Python types
auto_str = pattern_core.Value.string("hello")  # or just use "hello" in properties
auto_int = pattern_core.Value.int(42)  # or just use 42
auto_list = pattern_core.Value.array([1, 2, 3])  # converts automatically
```

### 10. Type Safety

```python
import pattern_core
from typing import List

# Type hints work with type checkers
def process_pattern(p: pattern_core.Pattern) -> List[str]:
    return [str(v) for v in p.values()]

# Type checker validates this
pattern = pattern_core.Pattern.point("hello")
result: List[str] = process_pattern(pattern)  # Type checker validates types
```

## Common Patterns

### Create Pattern from List

```python
import pattern_core

# Create pattern from list of values
pattern = pattern_core.Pattern.pattern("root", pattern_core.Pattern.from_values(["a", "b", "c"]))

print(pattern.value)  # "root"
print(pattern.length())  # 3
```

### Validate Pattern

```python
import pattern_core

# Create pattern
pattern = pattern_core.Pattern.pattern("root", [
    pattern_core.Pattern.point("a")
])

# Validate with rules
rules = pattern_core.ValidationRules(max_depth=10, max_elements=100)
try:
    pattern.validate(rules)
    print("Pattern is valid")
except pattern_core.ValidationError as e:
    print(f"Validation failed: {e.message}")
```

### Query Patterns

```python
import pattern_core

# Create pattern
pattern = pattern_core.Pattern.pattern("root", [
    pattern_core.Pattern.point("hello"),
    pattern_core.Pattern.point("world")
])

# Check if any value matches predicate
has_hello = pattern.any_value(lambda v: v == "hello")  # True

# Check if all values are strings
all_strings = pattern.all_values(lambda v: isinstance(v, str))  # True

# Find first matching subpattern
found = pattern.find_first(lambda p: p.value == "hello")
if found:
    print(found.value)  # "hello"
```

## Next Steps

- See `examples/pattern-core-python/` for comprehensive examples
- Read `docs/python-usage.md` for complete API reference
- Check type stubs in `pattern_core/__init__.pyi` for IDE support
- Run `mypy` or `pyright` for type checking validation
