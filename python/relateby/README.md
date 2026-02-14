# Relateby

Unified Python package for the pattern-rs library. One install provides both subpackages:

- **relateby.pattern** — Pattern data structures (from pattern-core)
- **relateby.gram** — Gram notation parser and serializer (from gram-codec)

## Install

```bash
pip install relateby
```

## Use

```python
import relateby.pattern
import relateby.gram
```

There are no top-level `pattern_core` or `gram_codec` imports; use `relateby.pattern` and `relateby.gram` only.

## Building from source

From the repository root, build the wheel from the unified package directory (requires maturin and Rust; Python 3.8–3.13 for the extension build):

```bash
cd python/relateby && pip wheel . -w dist
```
