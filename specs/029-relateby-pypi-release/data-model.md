# Data Model: Relateby PyPI Release

**Feature**: 029-relateby-pypi-release  
**Scope**: Entities for the **unified** Python package, its metadata, release process, and the relateby import namespace. No application database—metadata in version-controlled config and on PyPI.

---

## Entities

### 1. Python Package (Unified)

The single distributable published to PyPI (project name `relateby`). One install delivers both subpackages.

| Attribute    | Description |
|-------------|-------------|
| Identity    | PyPI project name `relateby`. Unique on the index; follows PyPI naming rules. |
| Version     | Single version string for the whole project (e.g. `0.1.0`). Not per-subpackage. |
| Artifacts   | Built outputs: wheel(s) per platform, optional sdist. Produced by unified build (one build command from unified package dir). |
| Import layout | Public entry points: `relateby.pattern` and `relateby.gram` only. No legacy `pattern_core` or `gram_codec` at top level. |
| Subpackages | (1) pattern-core → `relateby.pattern`; (2) gram-codec → `relateby.gram`. Both in one wheel. |

**Validation (from FR-001, FR-002, FR-003)**:
- Project name must be available on PyPI.
- Version must not already be published for this project (PyPI rejects duplicate uploads).
- One build produces one set of artifacts with consistent metadata (name, version).
- Only `relateby.pattern` and `relateby.gram` are public; no compatibility aliases.

**Relations**: One package has one **Package metadata** (one version). Package embodies the **Relateby namespace** (top-level `relateby`, subpackages `pattern` and `gram`).

---

### 2. Package Metadata

Information required by PyPI and installers, stored in the **single** `pyproject.toml` for the unified package.

| Attribute       | Description |
|-----------------|-------------|
| Name            | PyPI project name `relateby`. |
| Version         | Single release version for the unified package. |
| Description     | Short summary; used on PyPI project page. |
| Readme          | Long description (e.g. README.md); must render on PyPI. |
| License         | SPDX identifier (e.g. Apache-2.0). |
| Requires-Python | Version specifier (e.g. `>=3.8`). |
| Dependencies    | Optional runtime dependencies (unified package may have none or aggregate from crates). |
| Classifiers     | Trove classifiers (license, Python versions, status, etc.). |
| Keywords        | Optional list for discovery. |

**Validation (from FR-003)**:
- All fields required by PyPI present and valid.
- Description/readme must render (e.g. valid Markdown).
- No secrets or environment-specific paths in published metadata.
- Version is defined once (not per subpackage).

**Relations**: Describes exactly one **Python package (unified)** (one-to-one per release).

---

### 3. Relateby Namespace

The top-level import namespace and subpackages. Matches the PyPI project name.

| Attribute   | Description |
|-------------|-------------|
| Top-level   | `relateby` (import namespace and PyPI project name). |
| Subpackages | `relateby.pattern` (from pattern-core), `relateby.gram` (from gram-codec). |
| Install     | Users run `pip install relateby`; one install provides both subpackages. |

**Validation (from FR-006)**:
- PyPI project name and import paths are clearly defined and consistent.
- No conflict with existing PyPI names that would block or confuse installs.

**Relations**: The unified **Python package** provides this namespace; one-to-one.

---

### 4. Release Process

The set of steps and prerequisites to build, validate, and publish the **single** package.

| Attribute     | Description |
|---------------|-------------|
| Prerequisites | Tooling (maturin and/or unified build, Python), PyPI account with 2FA, API token or Trusted Publishing. |
| Inputs        | Source tree, single version (in unified package pyproject.toml), credentials (not in repo). |
| Steps        | Build unified package (one command), optional metadata check, upload (maturin publish or twine) to PyPI or TestPyPI. |
| Outputs      | Published release on PyPI (or TestPyPI); one project, one version; installable via `pip install relateby`. |
| Idempotency  | Same version cannot be re-uploaded to PyPI; TestPyPI can be used for repeated dry-runs. |

**Validation (from FR-004, FR-005)**:
- Process documented so a new maintainer can run it without reverse-engineering.
- Same inputs (version, source) produce the same publishable artifacts.
- Failed upload does not leave partial state; document retry.

**Relations**: Produces one **Python package (unified)** release per run. Consumes **Package metadata** from the unified package config.

---

## State Transitions

- **Package (per version)**:
  - Not built → Built (artifacts on disk from unified build).
  - Built → Published (uploaded to PyPI or TestPyPI).
  - Published (PyPI) → no further upload of same version (terminal state for that version).
- **Release process**: Single run; no persistent state in repo except version in the unified package’s pyproject.toml. Success = package visible on index; failure = no change or clear error.

---

## Out of Scope

- Source code or Rust crates (pattern-core, gram-codec) beyond packaging layout.
- PyPI user/account model.
- Download or install analytics (PyPI-side).
