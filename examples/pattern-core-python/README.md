# Pattern-Core Python Examples

Quickstart guide and working examples for pattern-core Python bindings.

## Installation

### From Source (Development)

```bash
cd crates/pattern-core

# Install uv if not already installed
curl -LsSf https://astral.sh/uv/install.sh | sh  # Or: brew install uv

# Create virtual environment and install
uv venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate
uv pip install -e ".[dev]"
maturin develop --uv --features python
```

### From Wheel

```bash
uv pip install pattern_core-*.whl
```

## Quick Start

### 1. Create an Atomic Pattern

```python
import pattern_core

# Create atomic pattern (no elements)
atomic = pattern_core.Pattern.point("hello")
print(f"Value: {atomic.value}")  # "hello"
print(f"Is atomic: {atomic.is_atomic()}")  # True
```

### 2. Create a Nested Pattern

```python
# Create nested pattern with elements
# The value decorates/describes the pattern represented by the elements
elem1 = pattern_core.Pattern.point("elem1")
elem2 = pattern_core.Pattern.point("elem2")
decorated = pattern_core.Pattern.pattern("decoration", [elem1, elem2])

print(f"Length: {decorated.length()}")  # 2
print(f"Depth: {decorated.depth()}")    # 1
print(f"Size: {decorated.size()}")      # 3 (root + 2 elements)
```

### 3. Work with Subjects

```python
# Create Subject with properties
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
print(f"Identity: {pattern.get_value().identity}")  # "alice"
```

### 4. Transform Patterns

```python
# Map: Transform all values to uppercase
pattern = pattern_core.Pattern.pattern("hello", pattern_core.Pattern.from_values(["world", "python"]))
upper = pattern.map(str.upper)
print(upper.values())  # ["HELLO", "WORLD", "PYTHON"]
```

### 5. Query Patterns

```python
# Filter: Find patterns matching predicate
pattern = pattern_core.Pattern.pattern("root", pattern_core.Pattern.from_values(["a", "b", "c"]))
filtered = pattern.filter(lambda p: p.value in ["a", "c"])
print(f"Filtered count: {len(filtered)}")  # 2
```

## Examples

This directory contains comprehensive examples:

1. **[basic_usage.py](./basic_usage.py)** - Pattern and Subject construction
2. **[operations.py](./operations.py)** - Pattern operations (map, filter, fold, combine)
3. **[zip_relationships.py](./zip_relationships.py)** - Relationship creation (zip3, zip_with)
4. **[type_safety.py](./type_safety.py)** - Type hints and static type checking
5. **[advanced.py](./advanced.py)** - Advanced patterns (comonad, complex subjects)

## Running Examples

```bash
# Run all examples
python basic_usage.py
python operations.py
python zip_relationships.py
python type_safety.py
python advanced.py

# Run with type checking
mypy type_safety.py
pyright type_safety.py
```

## Common Patterns

### Tree Structure

```python
# Build a directory tree
tree = pattern_core.Pattern.pattern(".", [
    pattern_core.Pattern.pattern("src", [
        pattern_core.Pattern.point("main.py"),
        pattern_core.Pattern.point("utils.py")
    ]),
    pattern_core.Pattern.point("README.md")
])

print(f"Total files: {tree.size()}")
```

### Graph Relationships

```python
# Create relationships from lists (bulk import)
people = [pattern_core.Pattern.point("Alice"), pattern_core.Pattern.point("Bob")]
companies = [pattern_core.Pattern.point("TechCorp"), pattern_core.Pattern.point("Startup")]
rel_types = ["WORKS_FOR", "FOUNDED"]

relationships = pattern_core.Pattern.zip3(people, companies, rel_types)
# Creates: (Alice)-[:WORKS_FOR]->(TechCorp), (Bob)-[:FOUNDED]->(Startup)

# Create relationships with logic (computed values)
relationships = pattern_core.Pattern.zip_with(
    people,
    companies,
    lambda p, c: "FOUNDED" if "Startup" in c.value else "WORKS_AT"
)
```

### Graph Structure

```python
# Build a social graph
alice = pattern_core.Subject(identity="alice", labels={"Person"})
bob = pattern_core.Subject(identity="bob", labels={"Person"})

alice_pattern = pattern_core.PatternSubject.point(alice)
bob_pattern = pattern_core.PatternSubject.point(bob)

# Alice knows Bob
graph = pattern_core.PatternSubject.pattern(alice, [bob_pattern])
```

### Validation

```python
# Validate pattern structure
rules = pattern_core.ValidationRules(max_depth=10, max_elements=100)
try:
    pattern.validate(rules)
    print("Pattern is valid")
except pattern_core.ValidationError as e:
    print(f"Validation failed: {e.message}")
```

## API Reference

See [docs/python-usage.md](../../docs/python-usage.md) for complete API documentation.

## Type Safety

All classes and methods have type hints in `pattern_core/__init__.pyi`. Use with:

- **mypy**: `mypy your_script.py`
- **pyright**: `pyright your_script.py`
- **IDE**: VS Code with Pylance, PyCharm

## Next Steps

- Read the [comprehensive API guide](../../docs/python-usage.md)
- Explore [type safety features](../../crates/pattern-core/PYTHON-TYPE-CHECKING.md)
- Check the [Rust documentation](https://docs.rs/pattern-core)
