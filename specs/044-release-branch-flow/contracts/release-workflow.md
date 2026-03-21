# Release Workflow Contract

This contract describes the maintainer-facing release operations that must remain stable after the branch-based release workflow lands.

## Command Contract: Release Preparation

### `scripts/new-release.sh <version>`

- **Purpose**: Create and prepare a version-specific release branch.
- **Inputs**:
  - A stable semantic version such as `0.2.0`
- **Expected behavior**:
  - Creates a release branch tied to the version
  - Applies the release-managed version bump
  - Leaves validation and final tagging to the dedicated release flow
- **Success result**:
  - The release branch is ready for review and finalization
- **Failure result**:
  - No stable tag is created
  - The version can be corrected and retried before any publish begins

## Command Contract: Release Validation

### `scripts/ci-local.sh --release`

- **Purpose**: Validate the release branch before tagging.
- **Expected behavior**:
  - Runs repository format, lint, build, test, documentation, WASM, npm, and Python checks as appropriate
  - Verifies that release-specific tooling is available when required
- **Success result**:
  - The release branch is eligible for finalization
- **Failure result**:
  - The stable tag must not be created

## Command Contract: Release Finalization

### Stable tag creation

- **Purpose**: Mark the finalized release commit with `vX.Y.Z`.
- **Expected behavior**:
  - Occurs only after validation passes, the release branch has been merged, and the release is finalized
  - Triggers the existing tag-based publish workflow
- **Failure result**:
  - If the stable tag cannot be created, publishing must not begin

## Workflow Contract: Publish

### `.github/workflows/publish.yml`

- **Trigger**: Stable version tags only
- **Expected behavior**:
  - Validate the tag and version mapping
  - Publish registry artifacts in the existing order
  - Treat a published version as immutable once any registry accepts it
- **Recovery rule**:
  - If publishing fails after a registry accepts the version, the next attempt must use a new patch version
