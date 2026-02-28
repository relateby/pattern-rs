# Data Model: Publish Crates Workflow

**Feature**: 034-publish-crates-workflow  
**Date**: 2025-02-28

This feature does not introduce new application data structures. The following entities describe the release/publish domain and workflow state.

---

## Entities

### Package (publishable crate)

- **Name**: Crate name (e.g. `relateby-pattern`, `relateby-gram`). Published package names use the relateby- prefix; source directories remain `pattern-core` and `gram-codec`.
- **Version**: From `Cargo.toml`; must match workspace/package version at publish time.
- **Metadata** (required for publish): `description`, `license`, `repository`, `documentation` (URL), `readme` (path or false), `homepage` (optional).
- **Relationship**: relateby-gram depends on relateby-pattern; publish order is relateby-pattern then relateby-gram.
- **Validation**: `cargo publish --dry-run` must succeed; build and tests must pass.

### Version tag

- **Format**: `v<major>.<minor>.<patch>` (e.g. `v0.1.0`).
- **Semantics**: Denotes a release; triggers the publish workflow when pushed.
- **Constraint**: Tag must match the version being published (e.g. tag `v0.1.0` → crates at `0.1.0`).

### Workflow run

- **Trigger**: Push of a tag matching `v*`.
- **Inputs**: Ref (tag), commit SHA, repository.
- **Secrets**: `CARGO_REGISTRY_TOKEN` (crates.io API token).
- **Steps**: Checkout → build → test → lint → (optional) doc → publish relateby-pattern → publish relateby-gram.
- **Outputs**: Success (all steps pass, crates published) or failure (no publish; logs show first failure).
- **State**: No persistent state in repo; crates.io holds published versions.

### Publishing instructions (documentation)

- **Content**: Prerequisites (token, tooling), tag format, how to trigger the workflow, recovery from partial publish and duplicate tag, where secrets are stored.
- **Location**: `docs/release.md` (or equivalent) in the repository.

---

## Validation Rules (from spec)

- Package metadata MUST be present and valid before publish (FR-002).
- Packages MUST be published in dependency order (FR-003).
- Workflow MUST NOT publish any package when validation fails (FR-007).
- Registry credentials MUST be stored in a secure mechanism, not in the repo (FR-009).

---

## State Transitions

- **Tag pushed** → Workflow starts.
- **Workflow: build/test/lint** → Pass → continue; Fail → exit, no publish.
- **Workflow: publish relateby-pattern** → Success → publish relateby-gram; Fail → exit, no gram-codec publish (recovery documented).
- **Workflow: publish relateby-gram** → Success → workflow complete; Fail → relateby-pattern may already be published (recovery documented).
- **Duplicate version publish** → crates.io rejects; workflow fails with clear error.

No persistent state machine in code; the above describes the intended behavior for implementation and docs.
