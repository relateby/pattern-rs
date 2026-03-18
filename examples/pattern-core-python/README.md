# Native Python Pattern Examples

These examples use the supported public `relateby.pattern` API from the combined wheel:

```bash
pip install relateby-pattern
```

## Quick Start

```python
from relateby.pattern import Pattern, StringVal, Subject

alice = Subject.from_id("alice").with_label("Person").with_property("name", StringVal("Alice"))
tree = Pattern(value=alice, elements=[Pattern.point(Subject.from_id("bob"))])

assert tree.size == 2
assert [subject.identity for subject in tree.values()] == ["alice", "bob"]
```

## Examples

This directory includes:

- `basic_usage.py`: native `Pattern`, `Subject`, and `Value` construction
- `operations.py`: `map`, `fold`, `find_first`, and structural traversal examples
- `standard_graph.py`: native `StandardGraph` classification and lookup examples

## Running

```bash
python examples/pattern-core-python/basic_usage.py
python examples/pattern-core-python/operations.py
python examples/pattern-core-python/standard_graph.py
```
