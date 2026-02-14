# Implementation Plan: Relateby PyPI Release

**Branch**: `029-relateby-pypi-release` | **Date**: 2025-02-14 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/029-relateby-pypi-release/spec.md`

## Summary

Publish a **single** Python package to PyPI under the project name `relateby`. One install (`pip install relateby`) delivers both `relateby.pattern` (from pattern-core) and `relateby.gram` (from gram-codec); no legacy import names (e.g. `pattern_core`). Single version for the whole project; build produces one wheel/sdist from a unified package that wraps or assembles the two existing Rust crates’ Python extensions. Documentation and repeatable release process for maintainers; optional TestPyPI dry-run and CI.

## Technical Context

**Language/Version**: Rust 1.70+ (workspace MSRV), Python 3.8+ (requires-python)  
**Primary Dependencies**: maturin (build/publish), PyO3 (pattern-core and gram-codec `python` features); unified package build may use a wrapper with build script that invokes cargo/maturin for both crates  
**Storage**: N/A (release artifacts built then uploaded)  
**Testing**: Existing cargo test, pytest in crates; add TestPyPI dry-run and `pip install relateby` + import smoke test  
**Target Platform**: PyPI (and TestPyPI); wheels for platforms supported by maturin (manylinux, macOS, Windows)  
**Project Type**: Library with Python bindings; one publishable PyPI project, two source crates (pattern-core, gram-codec)  
**Performance Goals**: Build and publish complete in reasonable time (minutes)  
**Constraints**: One PyPI project name (`relateby`); credentials not in repo; same version not re-uploadable to PyPI; only `relateby.pattern` and `relateby.gram` as public imports  
**Scale/Scope**: One unified package; two subpackages; single version; no migration doc required

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|--------|
| I. Reference Implementation Fidelity | Pass (N/A) | Packaging/release only; no change to library behavior or gram-hs port. |
| II. Correctness & Compatibility | Pass | PyPI and spec define behavior; no API break inside Rust crates beyond Python import names (clean break per spec). |
| III. Rust Native Idioms | Pass (N/A) | No new Rust code; existing crates unchanged. |
| IV. Multi-Target Library Design | Pass | Python wheel is one target; maturin multi-platform; native/WASM unchanged. |
| V. External Language Bindings & Examples | Pass | Unified package improves distribution; docs/examples use `relateby.pattern` and `relateby.gram` only. |

No violations. Complexity Tracking not used.

**Note**: When porting features from gram-hs, reference `../gram-hs`. This feature does not port from gram-hs.

## Project Structure

### Documentation (this feature)

```text
specs/029-relateby-pypi-release/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
└── tasks.md             # /speckit.tasks
```

### Source Code (repository root)

```text
crates/pattern-core/      # Existing; may remain as lib + Python extension source
crates/gram-codec/         # Existing; may remain as lib + Python extension source

# Unified Python package (location TBD by tasks: e.g. crates/relateby or python/relateby)
# Single pyproject.toml, name = "relateby", version in one place
# Build produces one wheel with relateby.pattern and relateby.gram
# No legacy pattern_core / gram_codec at top level

docs/                     # Optional: release.md or extend python-usage.md
scripts/                  # Optional: release helper
.github/workflows/        # Optional: publish workflow (secrets in CI)
```

**Structure Decision**: The unified package may be a new top-level package (e.g. `crates/relateby` or `python/relateby`) that depends on or builds pattern-core and gram-codec and assembles one wheel with the `relateby` namespace and `pattern`/`gram` subpackages. Exact layout is determined in tasks; plan assumes one pyproject.toml for the PyPI project `relateby`.

## Complexity Tracking

Not applicable.
