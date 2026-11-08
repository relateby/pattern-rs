# Python Packaging

Python release packaging is now intentionally simple:

- **Published distribution**: `relateby-pattern`
- **Import namespace**: `relateby.pattern`, `relateby.gram`
- **Build source**: `python/packages/relateby/`
- **Supported Python range**: `>=3.8,<3.14`

`relateby` itself is not a PyPI artifact.

For local development, prefer `uv` with a project-local `.venv`:

```bash
cd python/packages/relateby
uv venv --python 3.13 .venv
source .venv/bin/activate
uv pip install '.[dev]'
```

This is guidance rather than a hard packaging requirement, but it keeps the interpreter version aligned with the currently supported PyO3 build path.

## Extras (optional dependencies)

Extras let users install optional dependencies with square-bracket notation:

```bash
# Base install (pattern + gram)
pip install relateby-pattern

# With development tools (testing, building from source)
pip install relateby-pattern[dev]

# All optional dependencies (currently same as [dev]; extend as you add extras)
pip install relateby-pattern[all]
```

Defined in `python/packages/relateby/pyproject.toml` under `[project.optional-dependencies]`. The build backend copies these into the wheel METADATA as `Provides-Extra` and `Requires-Dist`, so pip resolves them when a user requests an extra.

## Adding new optional libraries

When you add an optional library (e.g. a visualization or export helper), add a new extra and optionally include it in `all`:

1. **Edit** `python/packages/relateby/pyproject.toml`:

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
       "relateby-pattern[dev]",
       "relateby-pattern[viz]",
       "relateby-pattern[export]",
   ]
   ```

2. **No backend changes**: The build backend reads `[project.optional-dependencies]` from `pyproject.toml` and embeds them in the wheel METADATA. New extras are picked up automatically.

3. **Document** the extra in `docs/python-usage.md` or this file so users know to install e.g. `relateby[viz]`.

### Naming and grouping

- Use short, lowercase names: `dev`, `viz`, `export`, `docs`.
- `dev`: development and testing (maturin, pytest, etc.).
- `all`: include every other extra (list `relateby-pattern[extra1]`, `relateby-pattern[extra2]`, …).
- You can combine extras: `pip install relateby-pattern[dev,viz]`.

## Building and publishing

For local development, build a wheel for the current platform:

```bash
cd python/packages/relateby
uv build --wheel --python 3.13 --out-dir dist
```

Published artifacts are built in CI by `.github/workflows/python-wheels.yml`, which `publish.yml` calls before uploading to PyPI:

- one wheel per platform, built with [cibuildwheel](https://cibuildwheel.pypa.io/) (configuration in `[tool.cibuildwheel]` in `pyproject.toml`): Linux x86_64 and aarch64 (`manylinux_2_17`), macOS arm64 and x86_64 (macOS 11+), and Windows 64-bit
- an sdist, so other platforms can build from source (needs a Rust toolchain)

The extension modules use the stable ABI (`pyo3/abi3-py38`), so each wheel is tagged `cp38-abi3` and works on every supported Python (3.8–3.13) rather than one wheel per Python version. musllinux (Alpine), Windows ARM64 and 32-bit builds are not published.

The workflow smoke-tests every wheel on Python 3.9 and 3.13 (and 3.8 on Linux and Windows) and installs the sdist before anything is uploaded. It can also be run on its own: use "Run workflow" on the *Python wheels* workflow, or open a pull request that touches Python packaging.

The sdist rewrites two manifests (`Cargo.toml`, `crates/gram-codec/Cargo.toml`) so that cargo can build the crates without the rest of the repository; see `relateby_build`.

The legacy split package directories remain in the repository for migration/reference only and are not part of the supported publish path.

## Public Surface Verification

The combined wheel is expected to ship:

- `relateby.pattern`
- `relateby.gram`
- wrapper `.pyi` files for both public subpackages
- `relateby/py.typed`

Before release, verify all of the following:

1. `python -m pytest python/packages/relateby/tests/test_public_api.py`
2. `python -m pip wheel python/packages/relateby -w python/packages/relateby/dist --no-deps`
3. `bash ./scripts/release/smoke-python.sh --wheel ./python/packages/relateby/dist/*.whl`

The supported public imports are:

```python
import relateby.pattern
import relateby.gram

from relateby.pattern import Pattern, StandardGraph, StringVal, Subject
from relateby.gram import gram_validate, parse_gram, round_trip
```

`pattern_core` and `gram_codec` remain internal build artifacts, not supported public imports.

## Alternative: optional native subpackages (historical)

If you ever need **optional native components** (e.g. install only `relateby.pattern` and not `relateby.gram`), the current single-wheel design does not support that. Options would be:

1. **Separate PyPI packages**: Publish `relateby-pattern` and `relateby-gram` (or similar), and make `relateby` a meta-package that depends on both. Then extras could add optional packages (e.g. `relateby[gram]` = add `relateby-gram`). This increases release and versioning complexity.
2. **Conditional build**: One package but build multiple wheels (e.g. default wheel with both extensions, and a “minimal” wheel with only pattern-core). Pip does not support “choose one of these wheels by extra”; you’d need separate package names or install paths.

For most use cases, one wheel with both pattern and gram and extras for **optional Python dependencies** is sufficient and easier to maintain.

## Summary

| Goal | Approach |
|------|----------|
| Optional **dependencies** (dev, viz, export, etc.) | Add entries under `[project.optional-dependencies]` in `pyproject.toml`. Use `pip install relateby-pattern[extra]`. |
| Publish combined package | Build from `python/packages/relateby/`, upload with twine. |
| Legacy split package directories | Keep out of the supported release flow. |
| Adding a new extra | Edit that package's `pyproject.toml`; rebuild; document. |
