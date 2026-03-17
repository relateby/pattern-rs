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

For a local source checkout of the combined package, use a normal install:

```bash
cd python/relateby && python -m pip install '.[dev]'
```

`python/relateby` does not currently support editable installs (`pip install -e .`) because the custom build backend only implements wheel and sdist builds.

After installation, run import checks and examples from outside `python/relateby` so the source tree does not shadow the installed package.

## Use

```python
import relateby.pattern
import relateby.gram
```

There are no top-level `pattern_core` or `gram_codec` imports; use `relateby.pattern` and `relateby.gram` only.

Representative public imports:

```python
from relateby.pattern import Pattern, StandardGraph, Subject, ValidationRules, Value
from relateby.gram import parse_gram, round_trip, validate_gram
```

Example public workflow:

```python
alice = Subject("alice", {"Person"}, {"name": Value.string("Alice")})
graph = StandardGraph.from_patterns([Pattern.point(alice)])

assert graph.node_count == 1
assert parse_gram("(alice:Person)").pattern_count == 1
assert validate_gram("(alice:Person)") is True
assert round_trip("(alice:Person)") == "(alice:Person)"
```

## Building from source

From the repository root, build the wheel from the combined package directory (requires maturin and Rust; Python 3.8–3.13 for the extension build):

```bash
cd python/relateby && pip wheel . -w dist
```
