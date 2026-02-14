# Research: Relateby PyPI Release

**Feature**: 029-relateby-pypi-release  
**Purpose**: Resolve technical unknowns for publishing the **unified** Python package to PyPI (one project `relateby`, one install, both `relateby.pattern` and `relateby.gram`).

---

## 1. PyPI Project Name and Import Layout

**Decision**: Single PyPI project name **`relateby`**. Users run `pip install relateby`; they import `relateby.pattern` and `relateby.gram` only (no legacy `pattern_core` or `gram_codec` at top level). The package installs a top-level namespace `relateby` with subpackages `pattern` and `gram`.

**Rationale**: Spec requires one cohesive library (like a single JS object with properties). One project name and one install keep the mental model simple. Import paths are defined by package layout (e.g. `relateby/pattern/`, `relateby/gram/`); internal extension modules (from the two Rust crates) can be private (e.g. `relateby._pattern`, `relateby._gram`) so only the intended public API is exposed.

**Alternatives considered**:
- **Two PyPI projects** (e.g. relateby-pattern, relateby-gram): Rejected; spec chose one project.
- **Hyphenated name** (e.g. relateby-pattern-gram): PyPI normalizes to same; short name `relateby` is preferred for `pip install relateby`.

---

## 2. Unified Build: One Wheel from Two Rust Crates

**Decision**: Implement a **unified Python package** (single pyproject.toml, project name `relateby`) that produces one sdist and one set of wheels. The package builds or depends on both pattern-core and gram-codec (Rust crates with PyO3) and assembles the wheel so that the installed layout provides `relateby.pattern` and `relateby.gram`. Preferred approach: a **wrapper package** (e.g. new directory `crates/relateby` or `python/relateby`) with a build backend that (1) builds both crates’ Python extensions (via maturin or cargo), (2) places the resulting extension modules under `relateby/pattern` and `relateby/gram` (or re-exports from internal `_pattern` / `_gram` so only those subpackages are public). Exact mechanism (maturin mixed project, setuptools-rust with multiple extensions, or custom build script) is an implementation detail; the contract is one build command, one version, one wheel containing both subpackages.

**Rationale**: Maturin typically builds one crate per pyproject.toml. To ship one wheel with two Rust-based subpackages, we need either a meta-package that builds both and assembles the wheel, or a single Cargo workspace package that compiles both extensions. A wrapper keeps existing crates (pattern-core, gram-codec) as the source of truth and composes them into the `relateby` layout.

**Alternatives considered**:
- **Two separate wheels, one PyPI project**: PyPI allows multiple dists per project (e.g. wheel + sdist) but not “two logical packages in one project” that install as one; one install must provide both. So one wheel (or one sdist that builds one wheel) is required.
- **Single crate that re-exports both**: Would merge pattern-core and gram-codec into one Rust crate; larger refactor and duplicates code; rejected in favor of wrapper.

---

## 3. Build and Publish Tooling

**Decision**: Use **maturin** for build and publish where applicable. The unified package may use maturin as the build backend (if it can drive a multi-crate build) or another backend (e.g. setuptools with a build script that invokes maturin/cargo for each crate); publish step uses `maturin publish` or `twine upload` for the built artifacts. Prefer a single-tool flow (maturin build + maturin publish) from the unified package directory; document any extra steps (e.g. building dependencies first).

**Rationale**: Project already uses maturin; it supports TestPyPI and PyPI. If the unified package is maturin-built, same tool for build and publish; otherwise build then twine/maturin upload.

**Alternatives considered**:
- **twine only**: Use if the unified build is not maturin-native (e.g. setuptools wrapper); document in release steps.
- **flit/poetry**: Not used; maturin/setuptools align with PyO3.

---

## 4. Credentials and Secrets

**Decision**: Unchanged from prior research. (1) **Local**: PyPI API token in `~/.pypirc` or `MATURIN_PYPI_TOKEN`, never committed. (2) **CI**: Prefer **Trusted Publishing**; else scoped API token in repository secrets. Document both in release docs.

**Rationale**: PyPI 2FA and token-based uploads are standard; Trusted Publishing avoids long-lived tokens in CI.

---

## 5. TestPyPI and Dry-Run

**Decision**: Document **TestPyPI** as the dry-run target: build the unified package, then `maturin publish --repository testpypi` (or equivalent). Verify with `pip install --index-url https://test.pypi.org/simple/ relateby` and `import relateby.pattern`, `import relateby.gram`. TestPyPI allows re-uploading the same version for testing.

**Rationale**: Same as before; validates release without affecting production.

---

## 6. Version and Re-upload

**Decision**: **Single version** for the unified package only (in the one pyproject.toml for `relateby`). PyPI **rejects re-upload** of an existing file (same project, version, filename); document that maintainers must bump version for a new release. No overwrite or delete in the release process.

**Rationale**: Spec requires one version for the whole project; PyPI immutability is standard.

---

## 7. Package Metadata and Validation

**Decision**: **One pyproject.toml** for the PyPI project `relateby` holds all metadata (name, version, description, readme, license, classifiers, requires-python). Validate with a successful build and, before first production publish, a TestPyPI upload. Description/readme must render on PyPI (e.g. valid Markdown).

**Rationale**: Single source of truth; avoids version or name drift between subpackages.

---

## Summary Table

| Topic              | Decision                               | Key point                                        |
|--------------------|----------------------------------------|--------------------------------------------------|
| PyPI project name  | `relateby`                             | One project; one install                         |
| Import layout      | `relateby.pattern`, `relateby.gram` only | No legacy pattern_core / gram_codec              |
| Unified build      | Wrapper package, one wheel             | Builds pattern-core + gram-codec; one version    |
| Publish tool       | maturin (or build + maturin/twine)     | One tool flow from unified package dir           |
| Credentials        | API token + Trusted Publishing         | No secrets in repo                               |
| Dry-run            | Publish to TestPyPI                     | Verify install and imports                       |
| Re-upload same ver| Not allowed; document                  | Bump version for new release                      |
| Metadata           | Single pyproject.toml for `relateby`   | One version, one name                            |
