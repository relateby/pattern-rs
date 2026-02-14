# Quickstart: Releasing the Unified Relateby Package to PyPI

**Feature**: 029-relateby-pypi-release  
**Audience**: Maintainers who want to publish the single `relateby` package to PyPI (one install provides `relateby.pattern` and `relateby.gram`).

---

## Prerequisites

- **Repository**: Checked out at the branch/tag you want to release.
- **Tooling**: [maturin](https://www.maturin.rs/) (and any unified-build tooling) installed. One build command produces the unified package artifacts.
- **PyPI**: Account with 2FA; create an API token (scope to this project if possible). For CI, prefer [Trusted Publishing](https://docs.pypi.org/trusted-publishers/).
- **Config**: Unified package has a single `pyproject.toml` with project name `relateby`, one version, and valid PyPI metadata. Public imports are only `relateby.pattern` and `relateby.gram`.

---

## Release Steps (Minimal)

1. **Set version**  
   Edit the **unified** package’s `pyproject.toml` (the one for project `relateby`): set `version` to the release version (e.g. `0.1.0`). Commit/tag as needed.

2. **Build**  
   From the **unified** package directory (the one that builds the single `relateby` wheel):
   ```bash
   # Example; exact command depends on unified package location (see tasks/docs)
   maturin build --release
   ```
   Artifacts appear in the configured output (e.g. `target/wheels/`).

3. **Dry-run (recommended)**  
   Upload to TestPyPI. Configure TestPyPI in `~/.pypirc` or use env vars; then:
   ```bash
   maturin publish --repository testpypi
   ```
   Optionally verify install:
   ```bash
   pip install --index-url https://test.pypi.org/simple/ relateby
   python -c "import relateby.pattern; import relateby.gram; print('OK')"
   ```

4. **Publish to PyPI**  
   When ready for production:
   ```bash
   maturin publish
   ```
   Use your PyPI API token when prompted (or Trusted Publishing in CI). Do not re-upload the same version—PyPI will reject it.

5. **Verify**  
   ```bash
   pip install relateby
   python -c "import relateby.pattern; import relateby.gram; print('OK')"
   ```

---

## Credentials (Do Not Commit)

- **Local**: Store PyPI token in `~/.pypirc` under `[pypi]` with username `__token__` and password `pypi-...`, or set `MATURIN_PYPI_TOKEN`.
- **CI**: Prefer Trusted Publishing; otherwise put a scoped API token in repository secrets and pass it into the publish step (e.g. `MATURIN_PYPI_TOKEN`).

---

## More Detail

- **Process contract**: [contracts/release-process.md](./contracts/release-process.md)  
- **Decisions and rationale**: [research.md](./research.md)  
- **Data model**: [data-model.md](./data-model.md)
