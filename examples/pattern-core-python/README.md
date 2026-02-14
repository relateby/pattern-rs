# Pattern-Core Python Examples

Quickstart guide and working examples for pattern-core Python bindings.

## Installation

### From source (development)

See [Release process](../../docs/release.md). Build from `python/relateby/` with `pip wheel . -w dist`, then install the wheel.

### From PyPI or TestPyPI

```bash
pip install relateby
# Or from TestPyPI: pip install --index-url https://test.pypi.org/simple/ relateby
```

### From wheel (local build)

```bash
# From repo: cd python/relateby && pip wheel . -w dist
pip install python/relateby/dist/relateby-*.whl
```

## Quick Start

### 1. Create an Atomic Pattern

```python
import relateby.pattern

# Create atomic pattern (no elements)
atomic = relateby.pattern.Pattern.point("hello")
print(f"Value: {atomic.value}")  # "hello"
print(f"Is atomic: {atomic.is_atomic()}")  # True
```

### 2. Create a Nested Pattern

```python
# Create nested pattern with elements
# The value decorates/describes the pattern represented by the elements
elem1 = relateby.pattern.Pattern.point("elem1")
elem2 = relateby.pattern.Pattern.point("elem2")
decorated = relateby.pattern.Pattern.pattern("decoration", [elem1, elem2])

print(f"Length: {decorated.length()}")  # 2
print(f"Depth: {decorated.depth()}")    # 1
print(f"Size: {decorated.size()}")      # 3 (root + 2 elements)
```

### 3. Work with Subjects

```python
# Create Subject with properties
subject = relateby.pattern.Subject(
    identity="alice",
    labels={"Person", "Employee"},
    properties={
        "name": relateby.pattern.Value.string("Alice"),
        "age": relateby.pattern.Value.int(30)
    }
)

# Create Pattern with Subject
pattern = relateby.pattern.Pattern.point(subject)
print(f"Identity: {pattern.value.identity}")  # "alice"
```

### 4. Transform Patterns

```python
# Map: Transform all values to uppercase
pattern = relateby.pattern.Pattern.pattern("hello", relateby.pattern.Pattern.from_values(["world", "python"]))
upper = pattern.map(str.upper)
print(upper.values())  # ["HELLO", "WORLD", "PYTHON"]
```

### 5. Query Patterns

```python
# Filter: Find patterns matching predicate
pattern = relateby.pattern.Pattern.pattern("root", relateby.pattern.Pattern.from_values(["a", "b", "c"]))
filtered = pattern.filter(lambda p: p.value in ["a", "c"])
print(f"Filtered count: {len(filtered)}")  # 2
```

### 6. Paramorphism - Structure-Aware Fold

```python
# Para: Structure-aware aggregation with access to pattern structure
pattern = relateby.pattern.Pattern.pattern(10, [
    relateby.pattern.Pattern.pattern(5, [
        relateby.pattern.Pattern.point(2),
        relateby.pattern.Pattern.point(1)
    ]),
    relateby.pattern.Pattern.point(3)
])

# Compute multiple statistics in one pass
def compute_stats(pattern, element_results):
    if not element_results:  # Atomic
        return (pattern.value, 1, 0)  # (sum, count, depth)
    
    # Aggregate from children
    child_sum = sum(r[0] for r in element_results)
    child_count = sum(r[1] for r in element_results)
    child_max_depth = max(r[2] for r in element_results)
    
    return (pattern.value + child_sum, 1 + child_count, 1 + child_max_depth)

stats = pattern.para(compute_stats)
print(f"Sum: {stats[0]}, Count: {stats[1]}, Depth: {stats[2]}")
```

## Examples

This directory contains comprehensive examples:

1. **[basic_usage.py](./basic_usage.py)** - Pattern and Subject construction
2. **[operations.py](./operations.py)** - Pattern operations (map, filter, fold, para, combine)
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
tree = relateby.pattern.Pattern.pattern(".", [
    relateby.pattern.Pattern.pattern("src", [
        relateby.pattern.Pattern.point("main.py"),
        relateby.pattern.Pattern.point("utils.py")
    ]),
    relateby.pattern.Pattern.point("README.md")
])

print(f"Total files: {tree.size()}")
```

### Graph Relationships

```python
# Create relationships from lists (bulk import)
people = [relateby.pattern.Pattern.point("Alice"), relateby.pattern.Pattern.point("Bob")]
companies = [relateby.pattern.Pattern.point("TechCorp"), relateby.pattern.Pattern.point("Startup")]
rel_types = ["WORKS_FOR", "FOUNDED"]

relationships = relateby.pattern.Pattern.zip3(people, companies, rel_types)
# Creates: (Alice)-[:WORKS_FOR]->(TechCorp), (Bob)-[:FOUNDED]->(Startup)

# Create relationships with logic (computed values)
relationships = relateby.pattern.Pattern.zip_with(
    people,
    companies,
    lambda p, c: "FOUNDED" if "Startup" in c.value else "WORKS_AT"
)
```

### Graph Structure

```python
# Build a social graph
alice = relateby.pattern.Subject(identity="alice", labels={"Person"})
bob = relateby.pattern.Subject(identity="bob", labels={"Person"})

alice_pattern = relateby.pattern.Pattern.point(alice)
bob_pattern = relateby.pattern.Pattern.point(bob)

# Alice knows Bob
graph = relateby.pattern.Pattern.pattern(alice, [bob_pattern])
```

### Validation

```python
# Validate pattern structure
rules = relateby.pattern.ValidationRules(max_depth=10, max_elements=100)
try:
    pattern.validate(rules)
    print("Pattern is valid")
except relateby.pattern.ValidationError as e:
    print(f"Validation failed: {e.message}")
```

## API Reference

See [docs/python-usage.md](../../docs/python-usage.md) for complete API documentation.

## Type Safety

Type hints: use `relateby.pattern` with mypy/pyright. See `docs/python-usage.md`.

- **mypy**: `mypy your_script.py`
- **pyright**: `pyright your_script.py`
- **IDE**: VS Code with Pylance, PyCharm

## Next Steps

- Read the [comprehensive API guide](../../docs/python-usage.md)
- Explore [type safety features](../../crates/pattern-core/PYTHON-TYPE-CHECKING.md)
- Check the [Rust documentation](https://docs.rs/pattern-core)
