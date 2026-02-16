# relateby-gram

Gram-only distribution for the **relateby** namespace. Installs `relateby.gram` (Gram notation parser/serializer from gram-codec). For both pattern and Gram notation, install the unified package instead:

```bash
pip install relateby
```

To install only this component (e.g. to combine with `relateby-pattern` or to keep installs minimal):

```bash
pip install relateby-gram
```

Then use:

```python
import relateby.gram
```

## Building from source

From the repository root (requires maturin and Rust):

```bash
cd python/relateby-gram && pip wheel . -w dist
```
