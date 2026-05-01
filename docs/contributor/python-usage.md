# Python Usage Guide

Install the combined distribution:

```bash
pip install relateby-pattern
```

Use only the public package boundaries:

```python
import relateby.pattern
import relateby.gram
```

`pattern_core` and `gram_codec` are implementation details, not supported imports.

## Native Pattern API

`relateby.pattern` exposes native dataclasses for `Pattern`, `Subject`, and `Value`:

```python
from relateby.pattern import Pattern, Subject, StringVal

alice = Subject.from_id("alice").with_label("Person").with_property("name", StringVal("Alice"))
tree = Pattern(
    value=alice,
    elements=[
        Pattern.point(Subject.from_id("bob").with_label("Person")),
        Pattern.point(Subject.from_id("carol").with_label("Person")),
    ],
)

assert tree.size == 3
assert [subject.identity for subject in tree.values()] == ["alice", "bob", "carol"]
```

The native `Pattern` API is pure Python, so structural operations run without a PyO3 round-trip:

```python
weights = Pattern(value=1, elements=[Pattern.point(2), Pattern.point(3)])

total = weights.fold(0, lambda acc, value: acc + value)
first_large = weights.find_first(lambda value: value > 2)
depths = weights.extend(lambda subtree: subtree.depth)

assert total == 6
assert first_large == 3
assert depths.values() == [1, 0, 0]
```

## Gram Helpers

`relateby.gram` parses Gram notation into native `Pattern[Subject]` values and raises `GramParseError` on failure:

```python
from relateby.gram import GramParseError, gram_stringify, gram_validate, parse_gram

patterns = parse_gram("(alice:Person)-->(bob:Person)")
assert patterns[0].value.identity == "alice"
assert gram_validate("(alice:Person)") == []
assert "alice" in gram_stringify(patterns)

try:
    parse_gram("(alice")
except GramParseError as exc:
    assert exc.input == "(alice"
```

## StandardGraph

`StandardGraph` is the graph-oriented wrapper over native `Pattern[Subject]` values:

```python
from relateby.pattern import Pattern, StandardGraph, Subject

relationship = Pattern(
    value=Subject.from_id("r1").with_label("KNOWS"),
    elements=[
        Pattern.point(Subject.from_id("alice").with_label("Person")),
        Pattern.point(Subject.from_id("bob").with_label("Person")),
    ],
)

graph = StandardGraph.from_patterns([relationship])

assert graph.node_count == 2
assert graph.relationship_count == 1
assert graph.source("r1").value.identity == "alice"
assert graph.target("r1").value.identity == "bob"
```

You can also compose parsing and classification:

```python
graph = StandardGraph.from_gram("(alice:Person)-->(bob:Person)")
assert graph.node("alice") is not None
```

## Building From Source

For local development, prefer `uv` with a project-local virtual environment:

```bash
cd python/packages/relateby
uv venv --python 3.13 .venv
source .venv/bin/activate
uv pip install '.[dev]'
```

The supported Python range for this package is `>=3.8,<3.14`. Using `3.13` is the safest default for local development because the current PyO3 build stack does not yet support `3.14`.

Build the combined wheel from the repo:

```bash
cd python/packages/relateby
uv build --wheel --python 3.13 --out-dir dist
```

Then install the wheel and run examples from outside `python/packages/relateby` so the source tree does not shadow the installed package.
