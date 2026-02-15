# Python Usage Guide (relateby)

Comprehensive API reference and usage guide for the **relateby** Python package. One install provides `relateby.pattern` (Pattern data structures) and `relateby.gram` (Gram notation). This guide focuses on the pattern API; use `relateby.gram` for parsing and serializing gram notation.

## Table of Contents

- [Installation](#installation)
- [Core Concepts](#core-concepts)
- [Value Types](#value-types)
- [Subject API](#subject-api)
- [Pattern API](#pattern-api)
- [PatternSubject API](#patternsubject-api)
- [Pattern Operations](#pattern-operations)
- [Comonad Operations](#comonad-operations)
- [Validation](#validation)
- [Structure Analysis](#structure-analysis)
- [Type Safety](#type-safety)
- [Best Practices](#best-practices)
- [Examples](#examples)

## Installation

### From PyPI (recommended)

```bash
pip install relateby
# Or with optional dependencies: pip install relateby[dev] or relateby[all]
```

One install provides both subpackages. See [Python packaging](python-packaging.md) for extras and adding optional libraries. Use them as:

```python
import relateby.pattern
import relateby.gram

# Minimal example
p = relateby.pattern.Pattern.point(42)
print(p.value)  # 42

# Gram notation
result = relateby.gram.parse_gram("(alice)-[:KNOWS]->(bob)")
```

### From TestPyPI (pre-release testing)

To try a version published to TestPyPI before it is on production PyPI:

```bash
pip install --index-url https://test.pypi.org/simple/ relateby
```

Then use `import relateby.pattern` and `import relateby.gram` as above. Note: TestPyPI may have older or pre-release versions; for stable use, install from PyPI.

### From source (development)

To build and install the unified package from the repository:

```bash
cd python/relateby
pip wheel . -w dist
pip install dist/relateby-*.whl
```

See [Release process](release.md) for build prerequisites (Rust, maturin).

## Core Concepts

relateby.pattern provides three main types:

1. **Value**: Property value types (string, int, array, map, etc.)
2. **Subject**: Self-descriptive value with identity, labels, and properties
3. **Pattern**: Recursive tree structure that can hold any value

## Value Types

### Creating Values

```python
import relateby.pattern

# Standard types
str_val = relateby.pattern.Value.string("hello")
int_val = relateby.pattern.Value.int(42)
decimal_val = relateby.pattern.Value.decimal(3.14)
bool_val = relateby.pattern.Value.boolean(True)
symbol_val = relateby.pattern.Value.symbol("alice")

# Extended types
array_val = relateby.pattern.Value.array([
    relateby.pattern.Value.int(1),
    relateby.pattern.Value.int(2),
    relateby.pattern.Value.int(3)
])

map_val = relateby.pattern.Value.map({
    "name": relateby.pattern.Value.string("Alice"),
    "age": relateby.pattern.Value.int(30),
    "active": relateby.pattern.Value.boolean(True)
})

range_val = relateby.pattern.Value.range(lower=0.0, upper=100.0)
measurement_val = relateby.pattern.Value.measurement(42.5, "meters")
```

### Extracting Values

```python
# Extract typed values
string = str_val.as_string()  # Returns str
integer = int_val.as_int()    # Returns int
decimal = decimal_val.as_decimal()  # Returns float
boolean = bool_val.as_boolean()  # Returns bool
array = array_val.as_array()  # Returns List[Value]
map_dict = map_val.as_map()   # Returns Dict[str, Value]
```

### Automatic Conversion

Python native types automatically convert to Value:

```python
subject = relateby.pattern.Subject(
    identity="alice",
    properties={
        "name": "Alice",  # Auto-converts to Value.string
        "age": 30,        # Auto-converts to Value.int
        "scores": [95, 87, 92],  # Auto-converts to Value.array
        "metadata": {"key": "value"}  # Auto-converts to Value.map
    }
)
```

## Subject API

### Creating Subjects

```python
# Basic subject with identity only
subject = relateby.pattern.Subject(identity="alice")

# Subject with labels
subject = relateby.pattern.Subject(
    identity="alice",
    labels={"Person", "Employee", "Developer"}
)

# Subject with properties
subject = relateby.pattern.Subject(
    identity="alice",
    properties={
        "name": relateby.pattern.Value.string("Alice"),
        "age": relateby.pattern.Value.int(30),
        "email": relateby.pattern.Value.string("alice@example.com")
    }
)

# Subject with everything
subject = relateby.pattern.Subject(
    identity="alice",
    labels={"Person", "Employee"},
    properties={
        "name": relateby.pattern.Value.string("Alice"),
        "department": relateby.pattern.Value.string("Engineering")
    }
)
```

### Working with Labels

```python
# Add labels
subject.add_label("Manager")
subject.add_label("TeamLead")

# Check labels
if subject.has_label("Manager"):
    print("Subject is a manager")

# Remove labels
subject.remove_label("Manager")

# Get all labels
labels = subject.get_labels()  # Returns Set[str]
print(f"Labels: {labels}")
```

### Working with Properties

```python
# Set properties
subject.set_property("name", relateby.pattern.Value.string("Alice"))
subject.set_property("age", 30)  # Auto-converts to Value.int

# Get properties
name_value = subject.get_property("name")  # Returns Optional[Value]
if name_value:
    name = name_value.as_string()
    print(f"Name: {name}")

# Remove properties
subject.remove_property("age")

# Get all properties
properties = subject.get_properties()  # Returns Dict[str, Value]
```

## Pattern API

### Creating Patterns

```python
# Atomic pattern (no elements)
atomic = relateby.pattern.Pattern.point("hello")

# Pattern with elements
child1 = relateby.pattern.Pattern.point("a")
child2 = relateby.pattern.Pattern.point("b")
parent = relateby.pattern.Pattern.pattern("root", [child1, child2])

# Pattern from list (convenience method)
pattern = relateby.pattern.Pattern.from_list("root", ["a", "b", "c", "d"])
```

### Accessing Pattern Properties

```python
# Get value
value = pattern.value

# Get elements
elements = pattern.elements  # Returns List[Pattern]

# Check if atomic
is_atomic = pattern.is_atomic()  # True if no elements

# Get structural properties
length = pattern.length()  # Number of direct children
size = pattern.size()      # Total number of nodes
depth = pattern.depth()    # Maximum nesting depth
```

### Pattern Inspection

```python
# Get all values (pre-order traversal)
values = pattern.values()  # Returns List[str]

# Check if pattern matches structure
pattern1 = relateby.pattern.Pattern.point("a")
pattern2 = relateby.pattern.Pattern.point("a")
matches = pattern1.matches(pattern2)  # True

# Check if pattern contains subpattern
parent = relateby.pattern.Pattern.pattern("root", [
    relateby.pattern.Pattern.point("a"),
    relateby.pattern.Pattern.point("b")
])
child = relateby.pattern.Pattern.point("a")
contains = parent.contains(child)  # True
```

## PatternSubject API

### Creating PatternSubject

```python
# Create Subject
subject = relateby.pattern.Subject(
    identity="alice",
    labels={"Person"},
    properties={"name": relateby.pattern.Value.string("Alice")}
)

# Create PatternSubject
pattern = relateby.pattern.PatternSubject.point(subject)

# PatternSubject with elements
child_subject = relateby.pattern.Subject(identity="child")
child_pattern = relateby.pattern.PatternSubject.point(child_subject)

parent_subject = relateby.pattern.Subject(identity="parent")
parent_pattern = relateby.pattern.PatternSubject.pattern(
    parent_subject,
    [child_pattern]
)
```

### Accessing PatternSubject Properties

```python
# Get Subject value
subject = pattern.get_value()  # Returns Subject

# Get elements
elements = pattern.get_elements()  # Returns List[PatternSubject]

# Get all Subject values
subjects = pattern.values()  # Returns List[Subject]
```

## Pattern Operations

### Query Operations

```python
# Check if any value satisfies predicate
has_hello = pattern.any_value(lambda v: v == "hello")

# Check if all values satisfy predicate
all_strings = pattern.all_values(lambda v: isinstance(v, str))

# Filter patterns by predicate
filtered = pattern.filter(lambda p: p.is_atomic())

# Find first matching pattern
found = pattern.find_first(lambda p: p.value == "target")
if found:
    print(f"Found: {found.value}")
```

### Transformation Operations

```python
# Transform values (preserves structure)
upper = pattern.map(str.upper)

# Fold over values
total = pattern.fold(0, lambda acc, val: acc + len(str(val)))

# Combine patterns
pattern1 = relateby.pattern.Pattern.point("hello")
pattern2 = relateby.pattern.Pattern.point(" world")
combined = pattern1.combine(pattern2)
```

### Relationship Creation

#### zip3 - From Three Lists

```python
# Create relationships from pre-computed lists
sources = [relateby.pattern.Pattern.point("Alice"), relateby.pattern.Pattern.point("Bob")]
targets = [relateby.pattern.Pattern.point("TechCorp"), relateby.pattern.Pattern.point("Startup")]
rel_types = ["WORKS_FOR", "FOUNDED"]

relationships = relateby.pattern.Pattern.zip3(sources, targets, rel_types)

# Each relationship: Pattern(rel_type, [source, target])
for rel in relationships:
    print(f"({rel.elements[0].value}) -[:{rel.value}]-> ({rel.elements[1].value})")
```

#### zip_with - With Computed Values

```python
# Create relationships with derived values
people = [relateby.pattern.Pattern.point("Alice"), relateby.pattern.Pattern.point("Bob")]
companies = [relateby.pattern.Pattern.point("TechCorp"), relateby.pattern.Pattern.point("Startup")]

relationships = relateby.pattern.Pattern.zip_with(
    people,
    companies,
    lambda p, c: "FOUNDED" if "Startup" in c.value else "WORKS_AT"
)
```

## Comonad Operations

Comonad operations allow you to work with patterns in their context.

### Extract

```python
# Extract value at current position
pattern = relateby.pattern.Pattern.point("hello")
value = pattern.extract()  # Returns "hello"
```

### Extend

```python
# Apply function to all contexts
pattern = relateby.pattern.Pattern.pattern("root", [
    relateby.pattern.Pattern.point("a"),
    relateby.pattern.Pattern.point("b")
])

# Replace each value with its depth
depths = pattern.extend(lambda p: p.depth())
```

### Structural Decorations

```python
# Decorate with depth
depths = pattern.depth_at()

# Decorate with subtree size
sizes = pattern.size_at()

# Decorate with path indices from root
indices = pattern.indices_at()
```

## Validation

### Validation Rules

```python
# Create validation rules
rules = relateby.pattern.ValidationRules(
    max_depth=10,      # Maximum nesting depth
    max_elements=100   # Maximum elements per pattern
)

# Validate pattern
try:
    pattern.validate(rules)
    print("Pattern is valid")
except relateby.pattern.ValidationError as e:
    print(f"Validation failed: {e.message}")
    print(f"Rule violated: {e.rule}")
    print(f"Location: {e.location}")
```

### Validation Error Handling

```python
try:
    deep_pattern = create_very_deep_pattern()  # 100+ levels
    rules = relateby.pattern.ValidationRules(max_depth=50)
    deep_pattern.validate(rules)
except relateby.pattern.ValidationError as e:
    if "depth" in e.rule.lower():
        print(f"Pattern too deep: {e.message}")
    elif "elements" in e.rule.lower():
        print(f"Too many elements: {e.message}")
```

## Structure Analysis

```python
# Analyze pattern structure
analysis = pattern.analyze_structure()

# Get human-readable summary
print(analysis.summary)  # "Pattern with 10 nodes, depth 3"

# Get depth distribution (nodes per depth level)
depth_dist = analysis.depth_distribution  # [1, 3, 4, 2]

# Get element counts per level
elem_counts = analysis.element_counts  # [3, 2, 1]

# Get nesting pattern descriptions
nesting = analysis.nesting_patterns  # ["uniform", "sparse"]
```

## Type Safety

### Type Hints

```python
from typing import List
import relateby.pattern

def process_pattern(p: relateby.pattern.Pattern) -> List[str]:
    """Type hints work with type checkers (mypy, pyright)."""
    return p.values()

def create_subject(name: str, age: int) -> relateby.pattern.Subject:
    """Type checkers validate parameter and return types."""
    return relateby.pattern.Subject(
        identity=name.lower(),
        properties={
            "name": relateby.pattern.Value.string(name),
            "age": relateby.pattern.Value.int(age)
        }
    )
```

### Type Checking

```bash
# Run mypy
mypy your_script.py

# Run pyright
pyright your_script.py
```

## Creating Graph Relationships

### Overview

Pattern-core provides two methods for creating relationship patterns between nodes:

1. **`Pattern.zip3(left, right, values)`** - Combine three pre-computed lists
2. **`Pattern.zip_with(left, right, value_fn)`** - Compute values from node pairs

Both create patterns with structure: `Pattern(relationship_value, [source_pattern, target_pattern])`

### When to Use Which

| Method | Use When | Example Use Cases |
|--------|----------|-------------------|
| `zip3` | Values are pre-computed | CSV import, database queries, API responses |
| `zip_with` | Values are derived | Business rules, conditional logic, computed types |

### Graph Import Example (zip3)

```python
# Importing relationship triples from external data
import csv

def import_relationships_from_csv(filepath):
    """Load relationships from CSV file."""
    sources = []
    targets = []
    rel_types = []
    
    with open(filepath) as f:
        reader = csv.DictReader(f)
        for row in reader:
            sources.append(relateby.pattern.Pattern.point(row['source']))
            targets.append(relateby.pattern.Pattern.point(row['target']))
            rel_types.append(row['relationship_type'])
    
    # Create all relationships in one operation
    return relateby.pattern.Pattern.zip3(sources, targets, rel_types)

# Usage
relationships = import_relationships_from_csv('graph_data.csv')
print(f"Imported {len(relationships)} relationships")
```

### Dynamic Relationship Creation (zip_with)

```python
# Derive relationship types from node properties
def create_smart_relationships(users, resources):
    """Create access relationships based on user roles."""
    
    def determine_access(user, resource):
        # Extract info from patterns
        user_role = user.value.split('_')[0]  # e.g., "admin_user" -> "admin"
        resource_type = resource.value
        
        # Business logic
        if user_role == "admin":
            return "FULL_ACCESS"
        elif resource_type == "public":
            return "READ_ONLY"
        else:
            return "NO_ACCESS"
    
    return relateby.pattern.Pattern.zip_with(users, resources, determine_access)

# Usage
users = [relateby.pattern.Pattern.point("admin_user"), relateby.pattern.Pattern.point("guest_user")]
resources = [relateby.pattern.Pattern.point("database"), relateby.pattern.Pattern.point("public_api")]
access_graph = create_smart_relationships(users, resources)
```

### Building Knowledge Graphs

```python
# Combine entity extraction with relationship creation
entities = extract_entities_from_text(document)  # Returns List[Pattern]
relationships_data = nlp_model.predict_relationships(document)  # Returns List[tuple]

# Method 1: If NLP returns types directly
sources = [entities[i] for i, _, _ in relationships_data]
targets = [entities[j] for _, j, _ in relationships_data]
types = [rel_type for _, _, rel_type in relationships_data]
kg = relateby.pattern.Pattern.zip3(sources, targets, types)

# Method 2: If types need computation
def classify_relationship(src, tgt):
    # Use heuristics or another model
    if src.value.endswith("Person") and tgt.value.endswith("Org"):
        return "WORKS_FOR"
    else:
        return "RELATED_TO"

kg = relateby.pattern.Pattern.zip_with(entities, entities, classify_relationship)
```

### Performance Considerations

- Both methods have O(n) time complexity where n = min(len(left), len(right))
- `zip3` is slightly faster (no function calls)
- `zip_with` allows lazy evaluation of relationship types
- Both stop at shortest list length (safe default)

## Best Practices

### 1. Use Atomic Patterns for Leaf Nodes

```python
# Good: Atomic pattern for leaf
leaf = relateby.pattern.Pattern.point("value")

# Avoid: Empty elements list
leaf = relateby.pattern.Pattern.pattern("value", [])  # Unnecessary
```

### 2. Use PatternSubject for Graph-Like Structures

```python
# Good: PatternSubject for subjects
subject = relateby.pattern.Subject(identity="node1", labels={"Person"})
pattern = relateby.pattern.PatternSubject.point(subject)

# Avoid: Plain Pattern for complex subjects
pattern = relateby.pattern.Pattern.point("node1")  # Loses structure
```

### 3. Validate Input Patterns

```python
# Good: Validate untrusted patterns
rules = relateby.pattern.ValidationRules(max_depth=100, max_elements=1000)
try:
    user_pattern.validate(rules)
    process_pattern(user_pattern)
except relateby.pattern.ValidationError as e:
    log_error(f"Invalid pattern: {e.message}")
```

### 4. Use Type Hints

```python
# Good: Type hints for clarity
def transform(p: relateby.pattern.Pattern) -> relateby.pattern.Pattern:
    return p.map(str.upper)

# Avoid: No type hints
def transform(p):
    return p.map(str.upper)
```

### 5. Handle Optional Return Values

```python
# Good: Handle None case
found = pattern.find_first(lambda p: p.value == "target")
if found:
    print(found.value)
else:
    print("Not found")

# Avoid: Assume non-None
found = pattern.find_first(lambda p: p.value == "target")
print(found.value)  # May raise AttributeError if None!
```

## Examples

### Example 1: Tree Structure

```python
# Build a file system tree
root = relateby.pattern.Pattern.pattern(".", [
    relateby.pattern.Pattern.pattern("src", [
        relateby.pattern.Pattern.point("main.py"),
        relateby.pattern.Pattern.point("utils.py")
    ]),
    relateby.pattern.Pattern.pattern("tests", [
        relateby.pattern.Pattern.point("test_main.py")
    ]),
    relateby.pattern.Pattern.point("README.md")
])

# Count total files
file_count = root.size()
print(f"Total files: {file_count}")

# Find all Python files
python_files = root.filter(lambda p: p.value.endswith(".py"))
print(f"Python files: {len(python_files)}")

# Get depth distribution
analysis = root.analyze_structure()
print(f"Structure: {analysis.summary}")
```

### Example 2: Social Graph

```python
# Create people as Subjects
alice = relateby.pattern.Subject(
    identity="alice",
    labels={"Person", "Employee"},
    properties={
        "name": relateby.pattern.Value.string("Alice"),
        "age": relateby.pattern.Value.int(30)
    }
)

bob = relateby.pattern.Subject(
    identity="bob",
    labels={"Person", "Employee"},
    properties={
        "name": relateby.pattern.Value.string("Bob"),
        "age": relateby.pattern.Value.int(35)
    }
)

# Build social graph
alice_pattern = relateby.pattern.PatternSubject.point(alice)
bob_pattern = relateby.pattern.PatternSubject.point(bob)

# Alice knows Bob
alice_with_friends = relateby.pattern.PatternSubject.pattern(
    alice,
    [bob_pattern]
)

# Query the graph
friends = alice_with_friends.get_elements()
print(f"Alice has {len(friends)} friends")

# Find employees
employees = alice_with_friends.filter(
    lambda p: p.get_value().has_label("Employee")
)
print(f"Found {len(employees)} employees")
```

### Example 3: Data Transformation Pipeline

```python
# Create data pattern
data = relateby.pattern.Pattern.from_list("data", [1, 2, 3, 4, 5])

# Transform: multiply by 2
doubled = data.map(lambda x: int(x) * 2)

# Filter: keep only values > 5
filtered = doubled.filter(lambda p: int(p.value) > 5)

# Fold: sum all values
total = filtered.fold(0, lambda acc, val: acc + int(val))

print(f"Total: {total}")
```

### Example 4: Pattern Validation

```python
def create_safe_pattern(data: List[str], max_depth: int = 10) -> relateby.pattern.Pattern:
    """Create pattern with validation."""
    pattern = relateby.pattern.Pattern.from_list("root", data)
    
    rules = relateby.pattern.ValidationRules(
        max_depth=max_depth,
        max_elements=1000
    )
    
    try:
        pattern.validate(rules)
        return pattern
    except relateby.pattern.ValidationError as e:
        raise ValueError(f"Pattern validation failed: {e.message}")

# Usage
try:
    pattern = create_safe_pattern(["a", "b", "c"])
    print("Pattern created successfully")
except ValueError as e:
    print(f"Error: {e}")
```

## API Reference Summary

### Value Class

| Method | Description | Returns |
|--------|-------------|---------|
| `Value.string(s)` | Create string value | `Value` |
| `Value.int(i)` | Create integer value | `Value` |
| `Value.decimal(f)` | Create decimal value | `Value` |
| `Value.boolean(b)` | Create boolean value | `Value` |
| `Value.symbol(s)` | Create symbol value | `Value` |
| `Value.array(items)` | Create array value | `Value` |
| `Value.map(items)` | Create map value | `Value` |
| `Value.range(lower, upper)` | Create range value | `Value` |
| `Value.measurement(value, unit)` | Create measurement value | `Value` |
| `as_string()` | Extract string | `str` |
| `as_int()` | Extract integer | `int` |
| `as_decimal()` | Extract decimal | `float` |
| `as_boolean()` | Extract boolean | `bool` |
| `as_array()` | Extract array | `List[Value]` |
| `as_map()` | Extract map | `Dict[str, Value]` |

### Subject Class

| Method | Description | Returns |
|--------|-------------|---------|
| `Subject(identity, labels, properties)` | Create subject | `Subject` |
| `identity` | Get identity | `str` |
| `get_labels()` | Get all labels | `Set[str]` |
| `get_properties()` | Get all properties | `Dict[str, Value]` |
| `add_label(label)` | Add label | `None` |
| `remove_label(label)` | Remove label | `None` |
| `has_label(label)` | Check label | `bool` |
| `get_property(name)` | Get property | `Optional[Value]` |
| `set_property(name, value)` | Set property | `None` |
| `remove_property(name)` | Remove property | `None` |

### Pattern Class

| Method | Description | Returns |
|--------|-------------|---------|
| `Pattern.point(value)` | Create atomic pattern | `Pattern` |
| `Pattern.pattern(value, elements)` | Create pattern with elements | `Pattern` |
| `Pattern.from_list(value, values)` | Create from list | `Pattern` |
| `value` | Get value | `str` |
| `elements` | Get elements | `List[Pattern]` |
| `is_atomic()` | Check if atomic | `bool` |
| `length()` | Number of children | `int` |
| `size()` | Total nodes | `int` |
| `depth()` | Max depth | `int` |
| `values()` | All values | `List[str]` |
| `any_value(pred)` | Check any value | `bool` |
| `all_values(pred)` | Check all values | `bool` |
| `filter(pred)` | Filter patterns | `List[Pattern]` |
| `find_first(pred)` | Find first match | `Optional[Pattern]` |
| `matches(other)` | Check structure match | `bool` |
| `contains(other)` | Check contains | `bool` |
| `map(func)` | Transform values | `Pattern` |
| `fold(init, func)` | Fold over values | `Any` |
| `combine(other)` | Combine patterns | `Pattern` |
| `zip3(left, right, values)` | Create from three lists | `List[Pattern]` |
| `zip_with(left, right, fn)` | Create with computed values | `List[Pattern]` |
| `extract()` | Extract value | `str` |
| `extend(func)` | Apply to contexts | `Pattern` |
| `depth_at()` | Decorate with depth | `Pattern` |
| `size_at()` | Decorate with size | `Pattern` |
| `indices_at()` | Decorate with indices | `Pattern` |
| `validate(rules)` | Validate pattern | `None` |
| `analyze_structure()` | Analyze structure | `StructureAnalysis` |

## Further Reading

- [Quickstart Guide](../examples/pattern-core-python/README.md)
- [Examples Directory](../examples/pattern-core-python/)
- [Type Safety Guide](../crates/pattern-core/PYTHON-TYPE-CHECKING.md)
- [Rust API Documentation](https://docs.rs/pattern-core)
