# Research: Consolidated Stable Publishing Workflow

**Feature**: 037-publishing-workflow  
**Date**: 2026-03-16

## 1. Release orchestration model

**Decision**: Use a two-stage tag-triggered GitHub Actions workflow with an explicit `validate` stage followed by publish jobs that run only after validation passes.

**Rationale**: The current `publish.yml` mixes build, test, and publish in one job and only covers crates.io. A separate validation stage prevents side effects before release-grade checks pass and makes it safe to add npm and PyPI publishing after the same gate.

**Alternatives considered**:
- Keep the current monolithic publish job: rejected because registry writes would begin before the full multi-ecosystem validation matrix is complete.
- Publish locally from `scripts/new-release.sh`: rejected because the user wants tag-triggered CI/CD to be the authoritative publish path.

## 2. Local release preparation responsibilities

**Decision**: Make `scripts/new-release.sh` the only supported release-preparation entry point and limit it to local, reversible work: precondition checks, version alignment, release validation, release commit creation, and annotated tag creation.

**Rationale**: Version state is duplicated across Cargo, npm, and Python metadata. A single script minimizes drift and ensures the tag is created only after the repo is proven releasable.

**Alternatives considered**:
- Manual version editing plus manual tag creation: rejected because it is error-prone and does not scale across four public artifacts.
- Let the publish workflow mutate versions on the release tag: rejected because release tags should point to an already-reviewed release commit.

## 3. npm packaging direction

**Decision**: Publish only `@relateby/pattern` to npm and fold the current `@relateby/gram` and `@relateby/graph` public package surfaces into that package.

**Rationale**: `@relateby/pattern` is already the umbrella package and currently depends on `@relateby/graph`, while `@relateby/gram` adds packaging overhead without representing an independent release artifact. A single npm package reduces version drift, simplifies docs, and matches the user’s desired release model.

**Alternatives considered**:
- Continue publishing `@relateby/graph`, `@relateby/pattern`, and `@relateby/gram`: rejected because the user explicitly wants one npm artifact.
- Publish `@relateby/pattern` plus keep `@relateby/graph` public: rejected because it would still leave multiple public packages and complicate release/version management.

## 4. Python packaging direction

**Decision**: Publish exactly one combined Python distribution, named `relateby-pattern`, while keeping `relateby.pattern` and `relateby.gram` as the import surface. Do not publish a distribution named `relateby`.

**Rationale**: The repo already has a combined package layer in `python/relateby/` that assembles both Rust extensions and exposes the full namespace-based Python API. Reusing that packaging path with renamed distribution metadata is the lowest-risk route that matches the requested "namespace-only" role for `relateby`.

**Alternatives considered**:
- Keep publishing `relateby` as the combined distribution: rejected because the user explicitly said `relateby` should be an organizational namespace, never an artifact.
- Publish two separate Python distributions (`relateby-pattern` and `relateby-gram`): rejected because the user wants one Python artifact and because the split-namespace layout is fragile.

## 5. Release validation strategy

**Decision**: Validate actual release artifacts, not only workspace builds. The release matrix should include Rust fmt/clippy/build/test, WASM build, `cargo publish --dry-run` for both crates, npm build/test plus `npm pack` and clean install smoke tests, and combined Python build plus wheel/smoke validation.

**Rationale**: Current CI treats WASM and Python as optional and does not validate the packed npm artifact or the combined Python distribution path. Automatic stable publishing requires artifact-level confidence, not just compile confidence.

**Alternatives considered**:
- Reuse current CI unchanged: rejected because it does not validate the real npm/PyPI release artifacts.
- Skip clean-environment smoke tests: rejected because packaging failures often appear only after pack/build output is installed outside the workspace.

## 6. Stable-only publishing policy

**Decision**: Restrict automated npm publishing to stable tags of the form `v<major>.<minor>.<patch>` and treat prerelease suffixes as non-publishable for npm.

**Rationale**: The user explicitly wants stable npm releases only. This also simplifies the publish workflow by removing dist-tag management and prerelease branching from the design.

**Alternatives considered**:
- Support npm prerelease tags such as `next` or `beta`: rejected because it adds policy and workflow complexity outside the requested scope.

## 7. Repo-specific gaps to address

**Decision**: Treat the following current-state mismatches as first-class implementation work:

1. `@relateby/pattern` depends on `@relateby/graph`, so npm consolidation requires source/package restructuring.
2. Python docs and metadata center `relateby` as the combined published artifact, which conflicts with the desired namespace-only role.
3. The current publish workflow only handles Rust crates.
4. CI treats WASM and Python validation as optional and does not test packed release artifacts.

**Rationale**: These gaps are structural, not cosmetic. The implementation plan must account for them to avoid designing a release process that cannot actually publish the intended artifacts.

**Alternatives considered**:
- Leave repo structure unchanged and update docs only: rejected because the current structure cannot support a single public npm package or a namespace-only Python distribution cleanly.
