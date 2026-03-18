# Relateby

Combined Python distribution for the pattern-rs library. One install provides both subpackages:

- **relateby.pattern** — Pattern data structures (from pattern-core)
- **relateby.gram** — Gram notation parser and serializer (from gram-codec)

The published distribution name is **relateby-pattern** while the import namespace remains `relateby.*`.

## Install

```bash
# Combined distribution (pattern + gram)
pip install relateby-pattern

# With optional dependencies (e.g. dev tools)
pip install relateby-pattern[dev]      # maturin, pytest, pytest-cov
pip install relateby-pattern[all]      # all optional extras
```

For a local source checkout, prefer `uv` with a project-local virtual environment:

```bash
cd python/packages/relateby
uv venv --python 3.13 .venv
source .venv/bin/activate
uv pip install '.[dev]'
```

`python/packages/relateby` does not currently support editable installs (`pip install -e .`) because the custom build backend only implements wheel and sdist builds.
The supported Python range is `>=3.8,<3.14`; `3.13` is the recommended local development target.

After installation, run import checks and examples from outside `python/packages/relateby` so the source tree does not shadow the installed package.

## Use

```python
import relateby.pattern
import relateby.gram
```

There are no top-level `pattern_core` or `gram_codec` imports; use `relateby.pattern` and `relateby.gram` only.

Representative public imports:

```python
from relateby.pattern import Pattern, StandardGraph, StringVal, Subject
from relateby.gram import gram_validate, parse_gram, round_trip
```

Example public workflow:

```python
alice = Subject.from_id("alice").with_label("Person").with_property("name", StringVal("Alice"))
graph = StandardGraph.from_patterns([Pattern.point(alice)])

assert graph.node_count == 1
assert len(parse_gram("(alice:Person)")) == 1
assert gram_validate("(alice:Person)") == []
assert round_trip("(alice:Person)") == "(alice:Person)"
```

## Building from source

From the repository root, build the wheel from the combined package directory (requires maturin and Rust; Python 3.8–3.13 for the extension build):

```bash
cd python/packages/relateby && uv build --wheel --python 3.13 --out-dir dist
```
