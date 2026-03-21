# Implementation Plan: Release Branch Workflow

**Branch**: `044-release-branch-flow` | **Date**: 2026-03-21 | **Spec**: [`spec.md`](./spec.md)
**Input**: Feature specification from `/specs/044-release-branch-flow/spec.md`

## Summary

Move release preparation off `main` and onto dedicated release branches so that version bumps and release-only fixes can be validated before a stable tag is created. The implementation will add a branch-first release flow, split validation from tagging/publishing, and update the release documentation so failed validation never leaves a dangling stable tag behind.

## Technical Context

**Language/Version**: Bash scripts, GitHub Actions YAML, repo documentation; existing Rust/Node/Python toolchain remains unchanged  
**Primary Dependencies**: `git`, `cargo`, `npm`, `uv`, `maturin`, `twine`, `wasm-pack`, GitHub Actions, `gh` for maintainer-facing release operations  
**Storage**: Git refs, annotated tags, and GitHub repository metadata; no new application datastore  
**Testing**: `scripts/ci-local.sh --release`, shell script execution, GitHub Actions workflow validation, existing Rust/npm/Python validation paths  
**Target Platform**: Local maintainer shell environment and GitHub Actions runners  
**Project Type**: Release automation / repository workflow  
**Performance Goals**: Release preparation should complete in one pass without requiring a new version bump for pre-publish validation failures  
**Constraints**: Stable tags remain immutable once published; branch-based release changes must preserve the existing multi-language publish order and registry behavior  
**Scale/Scope**: Repository-level workflow for a single release stream at a time

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Reference implementation fidelity: pass. This feature changes release orchestration only and does not alter library behavior.
- Correctness & compatibility: pass. The release version set and published artifacts remain semver-based and registry-compatible.
- Rust native idioms: pass. No Rust implementation changes are required.
- Multi-target library design: pass. The feature affects repository workflow, not runtime target support.
- External language bindings & examples: pass. No API or example surface changes are required.

## Project Structure

### Documentation (this feature)

```text
specs/044-release-branch-flow/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/
    └── release-workflow.md
```

### Source Code and Workflow Touchpoints

```text
scripts/
├── new-release.sh
└── release/
    ├── common.sh
    ├── prerelease.sh
    ├── verify-tag.sh
    └── smoke-python.sh

.github/workflows/
├── ci.yml
└── publish.yml

docs/
└── release.md
```

**Structure Decision**: This feature updates release automation scripts, GitHub Actions workflows, and release documentation only. No application source tree changes are expected.

## Complexity Tracking

No constitution violations require justification for this feature.
