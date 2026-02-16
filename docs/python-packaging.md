# Python Packaging (relateby)

This document describes how the **relateby** packages are packaged and how to extend them with optional dependencies using [extras](https://peps.python.org/pep-0508/#extras) (square-bracket notation).

## Package layout (three publishable artifacts)

| Package | PyPI name | Installs | Use case |
|---------|-----------|----------|----------|
| **relateby** | `relateby` | `relateby.pattern` + `relateby.gram` | Default: one install for both. |
| **relateby-pattern** | `relateby-pattern` | `relateby.pattern` only | Minimal install; combine with `relateby-gram` if needed. |
| **relateby-gram** | `relateby-gram` | `relateby.gram` only | Minimal install; combine with `relateby-pattern` if needed. |

All three install into the **same** `relateby` namespace. Installing `relateby-pattern` and `relateby-gram` (e.g. `pip install relateby-pattern relateby-gram`) merges into one `relateby` with both subpackages, equivalent to `pip install relateby`.

- **Unified wheel** (`relateby`): Built from `python/relateby/`. One wheel with both native extensions under `relateby._native`.
- **Single-crate wheels** (`relateby-pattern`, `relateby-gram`): Built from `python/relateby-pattern/` and `python/relateby-gram/`. Each wheel contains one native extension and the corresponding subpackage. Same namespace so they can be combined.

## Extras (optional dependencies)

Extras let users install optional dependencies with square-bracket notation:

```bash
# Base install (pattern + gram)
pip install relateby

# With development tools (testing, building from source)
pip install relateby[dev]

# All optional dependencies (currently same as [dev]; extend as you add extras)
pip install relateby[all]
```

Defined in `python/relateby/pyproject.toml` under `[project.optional-dependencies]`. The build backend copies these into the wheel METADATA as `Provides-Extra` and `Requires-Dist`, so pip resolves them when a user requests an extra.

## Adding new optional libraries

When you add an optional library (e.g. a visualization or export helper), add a new extra and optionally include it in `all`:

1. **Edit** `python/relateby/pyproject.toml`:

   ```toml
   [project.optional-dependencies]
   dev = [ ... ]
   # New optional feature
   viz = [
       "matplotlib>=3.5",
       "networkx>=3.0",
   ]
   export = [
       "pyyaml>=6.0",
   ]
   all = [
       "relateby[dev]",
       "relateby[viz]",
       "relateby[export]",
   ]
   ```

2. **No backend changes**: The build backend reads `[project.optional-dependencies]` from `pyproject.toml` and embeds them in the wheel METADATA. New extras are picked up automatically.

3. **Document** the extra in `docs/python-usage.md` or this file so users know to install e.g. `relateby[viz]`.

### Naming and grouping

- Use short, lowercase names: `dev`, `viz`, `export`, `docs`.
- `dev`: development and testing (maturin, pytest, etc.).
- `all`: include every other extra (list `relateby[extra1]`, `relateby[extra2]`, …).
- You can combine extras: `pip install relateby[dev,viz]`.

## Building and publishing the three packages

- **Unified** (`relateby`): From repo root, `cd python/relateby && pip wheel . -w dist`. See [Release process](release.md).
- **Single-crate** (`relateby-pattern`, `relateby-gram`): From repo root:
  ```bash
  cd python/relateby-pattern && pip wheel . -w dist
  cd python/relateby-gram   && pip wheel . -w dist
  ```
  Each package has its own version in its `pyproject.toml`. Publish with `twine upload dist/*` from each directory. Keep versions in sync (e.g. all 0.1.0). When users install both single-crate packages, both must be built for the same Python and platform.

## Alternative: optional native subpackages (historical)

If you ever need **optional native components** (e.g. install only `relateby.pattern` and not `relateby.gram`), the current single-wheel design does not support that. Options would be:

1. **Separate PyPI packages**: Publish `relateby-pattern` and `relateby-gram` (or similar), and make `relateby` a meta-package that depends on both. Then extras could add optional packages (e.g. `relateby[gram]` = add `relateby-gram`). This increases release and versioning complexity.
2. **Conditional build**: One package but build multiple wheels (e.g. default wheel with both extensions, and a “minimal” wheel with only pattern-core). Pip does not support “choose one of these wheels by extra”; you’d need separate package names or install paths.

For most use cases, one wheel with both pattern and gram and extras for **optional Python dependencies** is sufficient and easier to maintain.

## Summary

| Goal | Approach |
|------|----------|
| Optional **dependencies** (dev, viz, export, etc.) | Add entries under `[project.optional-dependencies]` in `pyproject.toml`. Use `pip install relateby[extra]`. |
| Publish **unified** package | Build from `python/relateby/`, upload with twine. |
| Publish **single-crate** packages | Build from `python/relateby-pattern/` and `python/relateby-gram/`, upload each with twine. |
| Adding a new extra | Edit that package's `pyproject.toml`; rebuild; document. |
