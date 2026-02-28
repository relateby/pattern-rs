# Implementation Plan: Publish Rust Artifacts to Crates with Docs, Examples, and Tag-Based Release Workflow

**Branch**: `034-publish-crates-workflow` | **Date**: 2025-02-28 | **Spec**: [spec.md](spec.md)  
**Input**: Feature specification from `/specs/034-publish-crates-workflow/spec.md`

## Summary

Publish workspace library crates under the **relateby-** prefix (relateby-pattern, relateby-gram) to crates.io with required metadata, dependency-order publish, API docs on docs.rs, and runnable examples. Add a GitHub Actions workflow that runs on version tags to build, validate, and publish. Document the publishing flow (prerequisites, tag format, secrets, recovery) so maintainers can release or hand off without tribal knowledge.

## Technical Context

**Language/Version**: Rust 1.70.0 (workspace MSRV), Edition 2021  
**Primary Dependencies**: cargo, crates.io API; GitHub Actions for CI; no new runtime dependencies  
**Storage**: N/A (publish pushes to crates.io; secrets in GitHub Secrets)  
**Testing**: cargo test, cargo publish --dry-run; existing CI (cargo test, clippy, fmt)  
**Target Platform**: Same as workspace (native, WASM); publish and docs run on GitHub-hosted Linux  
**Project Type**: Rust workspace (library crates); release automation is CI/workflow and docs  
**Performance Goals**: Publish flow completes in under 15 minutes; tag push triggers workflow within ~1 minute  
**Constraints**: No credentials in repo; workflow must not publish on validation failure; dependency order for publish  
**Scale/Scope**: 2 publishable crates (relateby-pattern, relateby-gram); one workflow; one set of publishing instructions

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|--------|
| I. Reference Implementation Fidelity | N/A | This feature is release/orchestration and docs, not a port of gram-hs behavior. No reference impl for “publish to crates.io” in pattern-hs. |
| II. Correctness & Compatibility | Pass | Publishing and docs must not change library API or behavior; only tooling and metadata. |
| III. Rust Native Idioms | Pass | Workflow and docs are configuration and prose; no new Rust code required for “publish flow” beyond existing Cargo.toml metadata. |
| IV. Multi-Target Library Design | Pass | No change to library targets; publish and docs apply to existing crates. |
| V. External Language Bindings & Examples | Pass | Spec requires examples be available to users; existing examples in workspace will be included or referenced per Phase 1. |

**Gate result**: Pass. No violations; N/A for reference fidelity is documented.

## Project Structure

### Documentation (this feature)

```text
specs/034-publish-crates-workflow/
├── plan.md              # This file
├── research.md          # Phase 0
├── data-model.md        # Phase 1
├── quickstart.md        # Phase 1
├── contracts/           # Phase 1 (workflow contract)
└── tasks.md             # Phase 2 (/speckit.tasks)
```

### Source Code (repository root)

```text
# Existing layout – this feature adds/updates:
.github/workflows/
├── ci.yml               # Existing
└── publish.yml          # NEW: tag-triggered publish

crates/
├── pattern-core/        # Publishable as relateby-pattern; fix metadata, examples inclusion
│   ├── Cargo.toml       # name = "relateby-pattern"; add readme, documentation, repository/homepage
│   └── ...
└── gram-codec/          # Publishable as relateby-gram; depends on relateby-pattern
    ├── Cargo.toml       # name = "relateby-gram"; relateby-pattern version for publish; readme, documentation
    └── ...

docs/                    # Existing; add or update publishing instructions
└── release.md           # NEW or existing: publishing flow, tag format, secrets, recovery
```

**Structure Decision**: No new crate or app; only workflow file, Cargo.toml metadata, and docs. Repository remains a Rust workspace with existing crates/ and .github/workflows/.

## Complexity Tracking

No constitution violations. Table left empty.
