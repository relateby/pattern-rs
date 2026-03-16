# Implementation Plan: Consolidated Stable Publishing Workflow

**Branch**: `037-publishing-workflow` | **Date**: 2026-03-16 | **Spec**: [spec.md](spec.md)  
**Input**: Feature specification from `/specs/037-publishing-workflow/spec.md`

## Summary

Replace the current crates-only release flow with a consolidated stable publishing workflow. A new `scripts/new-release.sh` script will prepare releases from `main` by aligning versions and running release-grade validation, while a tag-triggered GitHub Actions workflow will automatically validate and publish four public artifacts: `relateby-pattern`, `relateby-gram`, `@relateby/pattern`, and the combined Python distribution `relateby-pattern`. npm and Python packaging will each move to a single public artifact while preserving the existing import/API surfaces.

## Technical Context

**Language/Version**: Rust 1.70.0 (workspace MSRV), Bash for release scripting, Node.js 20/npm workspaces, Python 3.8+ packaging with maturin/twine  
**Primary Dependencies**: Cargo, GitHub Actions, wasm-pack, npm, vitest, maturin, twine, PyPI trusted publishing or token auth  
**Storage**: N/A; release state is encoded in versioned manifests, annotated tags, workflow runs, and external registries  
**Testing**: `cargo fmt`, `cargo clippy`, `cargo build`, `cargo test`, wasm32 build, `cargo publish --dry-run`, npm build/test/pack smoke checks, combined Python wheel build plus smoke install/import  
**Target Platform**: GitHub-hosted Linux for CI/CD; release artifacts target crates.io, npm, PyPI, and existing WASM/Python consumers  
**Project Type**: Multi-target Rust library workspace with TypeScript/WASM and Python packaging plus CI/CD automation  
**Performance Goals**: Release preparation completes in one maintainer flow; tag-triggered workflow publishes automatically after validation under normal CI conditions  
**Constraints**: `scripts/new-release.sh` must run only from clean `main`; no credentials in repo; npm stable releases only; `relateby` is namespace-only for Python and never a published distribution artifact; no publish on validation failure  
**Scale/Scope**: 2 Rust crates, 1 npm package, 1 combined Python distribution, 1 release-prep script, 2 GitHub workflows, and aligned release documentation

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Reference Implementation Fidelity | N/A | This feature changes release tooling, packaging, and documentation rather than gram-hs behavioral parity. No reference implementation exists in `../pattern-hs` for CI/CD or package registry orchestration. |
| II. Correctness & Compatibility | Pass | The design preserves the public Rust crate names and preserves Python imports `relateby.pattern` and `relateby.gram`. npm consolidation is an intentional public packaging change and will be documented. |
| III. Rust Native Idioms | Pass | Rust code behavior is unchanged; script/workflow changes remain outside core library logic. |
| IV. Multi-Target Library Design | Pass | The plan explicitly strengthens release validation for native Rust, WASM, npm/WASM, and Python artifacts rather than weakening support. |
| V. External Language Bindings & Examples | Pass | The feature requires updating npm/Python docs and validating the packaged artifacts so examples remain accurate for external language users. |

**Gate result (pre-research)**: Pass.

**Gate result (post-design)**: Pass. Phase 1 design preserves existing multi-target import/API surfaces while narrowing the set of published artifacts intentionally and documenting the migration.

## Project Structure

### Documentation (this feature)

```text
specs/037-publishing-workflow/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── release-api.yaml
└── tasks.md
```

### Source Code (repository root)

```text
.github/workflows/
├── ci.yml
└── publish.yml

scripts/
├── ci-local.sh
└── new-release.sh

Cargo.toml
crates/
├── pattern-core/Cargo.toml
└── gram-codec/Cargo.toml

package.json
package-lock.json
typescript/
└── @relateby/
    ├── pattern/
    │   ├── package.json
    │   ├── src/
    │   └── tests/
    ├── gram/
    │   ├── package.json
    │   └── src/
    └── graph/
        ├── package.json
        ├── src/
        └── tests/

python/
├── relateby/
│   ├── pyproject.toml
│   ├── relateby/
│   └── relateby_build/
├── relateby-pattern/
│   └── pyproject.toml
└── relateby-gram/
    └── pyproject.toml

docs/
├── release.md
├── python-packaging.md
├── python-usage.md
├── wasm-usage.md
└── typescript-graph.md
```

**Structure Decision**: Keep the existing workspace layout and implement the feature by consolidating public packaging and release automation within current directories. No new crate or top-level app is introduced; the main additions are `scripts/new-release.sh`, stronger workflow orchestration, and packaging/doc updates in the existing TypeScript and Python trees.

## Complexity Tracking

No constitution violations require justification.
