# Releasing the Relateby Python Package

This document describes how to build and publish the unified **relateby** package to PyPI (or TestPyPI for a dry-run). One install delivers `relateby.pattern` and `relateby.gram`; there are no top-level `pattern_core` or `gram_codec` imports.

**Dry-run approach**: Always validate with a TestPyPI upload and install test before publishing to production PyPI.

**For new maintainers**: A short maintainer-oriented quickstart is in the feature spec: [Maintainer quickstart](../specs/029-relateby-pypi-release/quickstart.md). This document is the canonical release process and matches the [release process contract](../specs/029-relateby-pypi-release/contracts/release-process.md).

---

## Ordered release steps (per release process contract)

Follow these steps in order for a repeatable release:

1. **Set version** — In `python/relateby/pyproject.toml`, set `version` to the release version (e.g. `0.1.0`). Commit/tag as needed.
2. **Build** — From `python/relateby/`, run `pip wheel . -w dist` (or `python -m build --wheel`). Artifacts appear in `python/relateby/dist/`.
3. **Validate (optional)** — Upload to TestPyPI: `twine upload --repository testpypi dist/*`. Optionally verify: `pip install --index-url https://test.pypi.org/simple/ relateby` then `import relateby.pattern`, `import relateby.gram`.
4. **Publish** — For production: `twine upload dist/*`. For dry-run only: use step 3.
5. **Verify** — Optionally install from the target index and run the same import check.

---

## 1. Prerequisites

Ensure the following are in place **before** building or publishing. Do not embed secrets in this repo or in docs.

| Prerequisite | Description |
|--------------|-------------|
| **Repository** | Checkout at the branch/tag you intend to release. Working tree clean or committed. |
| **Python** | 3.8–3.13 recommended for building the extensions (PyO3/maturin support). Use a venv with maturin installed for the build. |
| **Rust** | Rust toolchain (e.g. `rustup`) so that maturin can build the pattern-core and gram-codec extensions. |
| **maturin** | Installed in the environment used to build (e.g. `pip install maturin`). The unified package build backend invokes maturin for both crates. |
| **twine** | For uploading wheels to PyPI or TestPyPI (e.g. `pip install twine`). |
| **PyPI account** | For production: PyPI account with 2FA enabled. Create an API token (project-scoped if desired) at [pypi.org/manage/account/token/](https://pypi.org/manage/account/token/). |
| **TestPyPI account** | For dry-run: optional but recommended; create at [test.pypi.org](https://test.pypi.org/). |
| **Credentials** | Stored locally in `~/.pypirc` or via environment variables (see [Credentials](#credentials)); never commit tokens or passwords. |
| **CI (optional)** | Prefer [Trusted Publishing](https://docs.pypi.org/trusted-publishers/); otherwise use a repository secret (e.g. `TWINE_PASSWORD` or PyPI token) for the upload step. |

---

## 2. Build and Publish Commands

The unified package lives in **`python/relateby/`**. All build and publish steps are run from that directory (or from repo root with `python/relateby` as the project path).

### 2.1 Set version

Edit **`python/relateby/pyproject.toml`** and set the `version` field to the release version (e.g. `0.1.0`). This is the single source of version for the whole project. Commit (and tag) as needed.

### 2.2 Build

From the **repository root**:

```bash
cd python/relateby
pip wheel . -w dist
```

Or, using the `build` package:

```bash
cd python/relateby
python -m build --wheel
```

- **Directory**: `python/relateby/` (the directory containing the single `pyproject.toml` for project name `relateby`).
- **Output**: Wheel(s) and optionally sdist under `python/relateby/dist/` (e.g. `relateby-0.1.0-cp312-cp312-macosx_11_0_arm64.whl`). The build backend invokes maturin for `crates/pattern-core` and `crates/gram-codec` and assembles one wheel per platform.

### 2.3 Publish to PyPI (production)

After a successful dry-run to TestPyPI (recommended), upload to production PyPI:

```bash
cd python/relateby
twine upload dist/*
```

Use your PyPI API token when prompted (or configure credentials as in [Credentials](#credentials)).

### 2.4 TestPyPI dry-run (recommended before production)

Upload to TestPyPI first to validate metadata and install without affecting production:

```bash
cd python/relateby
twine upload --repository testpypi dist/*
```

Optional verification after upload:

```bash
pip install --index-url https://test.pypi.org/simple/ relateby
python -c "import relateby.pattern; import relateby.gram; print('OK')"
```

- TestPyPI allows re-uploading the same version for repeated testing (unlike production PyPI).
- Use a separate TestPyPI account/token if you do not want to use your production credentials.

---

## 3. Version (one place) and reproducibility

- **Single source of version**: Version is defined in **one place only**: `python/relateby/pyproject.toml`. There is no separate version for each subpackage; the whole project has one version.
- **Consistent artifacts**: The same source tree and version produce the same build artifacts. Re-running the build from the same commit and version yields identical wheel(s) (and sdist if built). This makes releases reproducible and auditable.
- **PyPI re-upload**: PyPI **rejects** re-upload of an existing file (same project name, version, and filename). If upload fails with a duplicate-version error, do **not** retry the same version. Bump the version in `python/relateby/pyproject.toml`, rebuild, then upload again.

---

## 4. Credentials

**Never commit API tokens or passwords.** Use one of the following.

| Method | Use case |
|--------|----------|
| **`~/.pypirc`** | Local uploads. Create `[pypi]` and/or `[testpypi]` with `username = __token__` and `password = pypi-...` (your token). |
| **`TWINE_USERNAME` / `TWINE_PASSWORD`** | Environment variables for twine (e.g. in CI). For token, use `__token__` and the token value. |
| **`MATURIN_PYPI_TOKEN`** | Used by maturin when it is the upload tool; our unified package uses twine, but if you use maturin elsewhere, set this instead of embedding in config. |
| **CI: Trusted Publishing** | Prefer [Trusted Publishing](https://docs.pypi.org/trusted-publishers/) so CI can upload without long-lived tokens. |
| **CI: Repository secret** | Store a PyPI API token in a repository secret and pass it into the workflow as `TWINE_PASSWORD` (or equivalent). |

---

## 5. Error handling (per release process contract)

| Condition | Expected behavior |
|-----------|-------------------|
| **Build failure** | No upload. Fix the build (Rust/Python/maturin). Re-run from step 1 (set version) and step 2 (build). Do not retry upload until build succeeds. |
| **Duplicate version (PyPI)** | Upload fails with a clear error. Do not retry the same version. Bump version in `python/relateby/pyproject.toml`, rebuild, then re-run the publish step. |
| **Credential error** | Upload fails; no partial upload. Fix credentials (`~/.pypirc` or env vars) and re-run the publish step (step 4). |
| **Network failure** | Retry the publish step. PyPI accepts idempotent uploads for a new version; retrying is safe once the version is not already on the index. |

---

## 6. Summary (dry-run flow)

1. Set version in `python/relateby/pyproject.toml`.
2. From `python/relateby/`: `pip wheel . -w dist`.
3. **Dry-run**: `twine upload --repository testpypi dist/*` then `pip install --index-url https://test.pypi.org/simple/ relateby` and `import relateby.pattern`, `import relateby.gram`.
4. **Production**: `twine upload dist/*`.
5. Optionally verify with `pip install relateby` and the same import check.
