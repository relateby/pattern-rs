# Relateby

Unified Python package for the pattern-rs library. One install provides both subpackages:

- **relateby.pattern** — Pattern data structures (from pattern-core)
- **relateby.gram** — Gram notation parser and serializer (from gram-codec)

Optional: you can instead install **relateby-pattern** and/or **relateby-gram** separately (same namespace; useful for minimal or combined installs). See the `python/relateby-pattern` and `python/relateby-gram` directories and `docs/python-packaging.md` in the repository.

## Install

```bash
# Core (pattern + gram)
pip install relateby

# With optional dependencies (e.g. dev tools)
pip install relateby[dev]      # maturin, pytest, pytest-cov
pip install relateby[all]      # all optional extras
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
