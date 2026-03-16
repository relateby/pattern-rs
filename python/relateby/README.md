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

## Use

```python
import relateby.pattern
import relateby.gram
```

There are no top-level `pattern_core` or `gram_codec` imports; use `relateby.pattern` and `relateby.gram` only.

## Building from source

From the repository root, build the wheel from the combined package directory (requires maturin and Rust; Python 3.8–3.13 for the extension build):

```bash
cd python/relateby && pip wheel . -w dist
```
