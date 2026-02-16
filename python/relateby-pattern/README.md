# relateby-pattern

Pattern-only distribution for the **relateby** namespace. Installs `relateby.pattern` (Pattern data structures from pattern-core). For both pattern and Gram notation, install the unified package instead:

```bash
pip install relateby
```

To install only this component (e.g. to combine with `relateby-gram` or to keep installs minimal):

```bash
pip install relateby-pattern
```

Then use:

```python
import relateby.pattern
```

## Building from source

From the repository root (requires maturin and Rust):

```bash
cd python/relateby-pattern && pip wheel . -w dist
```
