# Release Process Contract

**Feature**: 029-relateby-pypi-release  
**Type**: Process contract (inputs, outputs, steps). Not an HTTP API.

This document defines the agreed release process for the **unified** package so that implementations (docs, scripts, CI) can be validated against it.

---

## Preconditions

| Input / prerequisite | Required | Description |
|----------------------|----------|-------------|
| Repository at release version | Yes | Branch/tag and working tree match the version to release. |
| Unified package `pyproject.toml` | Yes | Single pyproject.toml for project name `relateby`, with version and all PyPI-required metadata. |
| maturin (and/or unified build) | Yes | Available on PATH; one build command produces one set of artifacts. |
| PyPI credentials | Yes for production | API token (e.g. in `~/.pypirc` or `MATURIN_PYPI_TOKEN`) or Trusted Publishing. TestPyPI credentials for dry-run. |
| Version not already on target index | Yes for PyPI | PyPI rejects re-upload of same version; TestPyPI may allow. |

---

## Process Steps (Ordered)

1. **Set version**  
   Ensure `version` in the **unified** package’s `pyproject.toml` (the one for `relateby`) matches the intended release. Single version for the whole project.

2. **Build**  
   Run from the **unified** package directory (e.g. the directory containing the single `pyproject.toml` for `relateby`):  
   One build command (e.g. `maturin build --release` or as documented) that produces wheel(s) and optionally sdist.  
   Success: one set of artifacts (e.g. under `target/wheels/` or documented output dir) for the project `relateby`.

3. **Validate (optional)**  
   Option A: Run `maturin publish --repository testpypi` (or equivalent) to TestPyPI.  
   Option B: If supported, run maturin’s dry-run or metadata check.  
   Success: No upload errors; optional install test: `pip install --index-url https://test.pypi.org/simple/ relateby` then `import relateby.pattern`, `import relateby.gram`.

4. **Publish**  
   For production: `maturin publish` (or `twine upload`) from the unified package context.  
   For dry-run: `maturin publish --repository testpypi`.  
   Success: Package and version visible on target index; installable via `pip install relateby`.

5. **Verify**  
   Optional: Install from target index (`pip install relateby` or from TestPyPI URL) and run minimal import/usage check for both `relateby.pattern` and `relateby.gram`.

---

## Outputs

| Output | When | Description |
|--------|------|-------------|
| Wheel(s) and sdist | After step 2 | Local artifacts for project `relateby`; may be discarded after upload. |
| Published release | After step 4 | One project, one version, visible on PyPI or TestPyPI; one install provides both subpackages. |

---

## Error Handling

| Condition | Expected behavior |
|-----------|-------------------|
| Build failure | No upload; fix build and re-run from step 1. |
| Duplicate version (PyPI) | Upload fails with clear error; do not retry same version. Bump version and re-run. |
| Credential error | Upload fails; no partial upload. Fix credentials and re-run step 4. |
| Network failure | Retry step 4; PyPI accepts idempotent uploads for new version. |

---

## Out of Scope for This Contract

- Version bump strategy (e.g. semver, calver).
- Changelog or release notes generation.
- Signing of artifacts (can be added later).
- Notifications (e.g. release webhooks).
