# Data Model: Consolidated Stable Publishing Workflow

**Feature**: 037-publishing-workflow  
**Date**: 2026-03-16

This feature does not add runtime application data structures. The model below describes the release domain that the script, workflows, and documentation must coordinate.

## Entities

### Release Version

- **Description**: The canonical semantic version for one release across all public artifacts.
- **Fields**:
  - `value`: semantic version string, e.g. `0.2.0`
  - `tag`: annotated git tag derived from the version, e.g. `v0.2.0`
  - `is_stable`: boolean, true only for `MAJOR.MINOR.PATCH` versions without prerelease suffixes
- **Validation rules**:
  - Must match all release-managed manifest versions.
  - Must produce a stable tag before npm publishing is allowed.

### Release Artifact

- **Description**: A package published for a given `Release Version`.
- **Fields**:
  - `ecosystem`: one of `cargo`, `npm`, `pypi`
  - `name`: public artifact name
  - `source_path`: repo path that defines/builds the artifact
  - `version_source`: manifest field updated by `scripts/new-release.sh`
  - `publish_order`: integer ordering within a release
  - `validation_commands`: artifact-specific checks that must pass before publish
- **Instances**:
  - `relateby-pattern` crate from `crates/pattern-core`
  - `relateby-gram` crate from `crates/gram-codec`
  - `@relateby/pattern` npm package from `typescript/@relateby/pattern`
  - `relateby-pattern` Python distribution from `python/relateby`
- **Relationships**:
  - `relateby-gram` depends on `relateby-pattern` crate at the same release version.
  - npm and Python artifacts both expose combined pattern + gram functionality.

### Release Preparation Run

- **Description**: One local execution of `scripts/new-release.sh`.
- **Fields**:
  - `branch`: expected to be `main`
  - `working_tree_clean`: boolean
  - `remote_sync_ok`: boolean indicating local `main` matches `origin/main`
  - `target_version`: `Release Version`
  - `updated_manifests`: list of manifest paths changed during preparation
  - `validation_result`: pass/fail summary for the full release matrix
  - `release_commit`: commit SHA created by the script on success
  - `release_tag`: annotated tag created by the script on success
  - `pushed`: boolean, true only when maintainer opts to push
- **Validation rules**:
  - Must abort before editing files if branch/cleanliness/sync checks fail.
  - Must not create a tag when validation fails.

### Validation Matrix

- **Description**: The set of checks required before a tag is created or any artifact is published.
- **Fields**:
  - `rust_checks`: fmt, clippy, build, tests, wasm build, crate dry-runs
  - `npm_checks`: install, build, tests, pack, clean install smoke tests
  - `python_checks`: build combined artifact, wheel metadata checks, clean install smoke tests
  - `workflow_checks`: tag format, version alignment, commit-on-main verification
- **Validation rules**:
  - All required checks must pass locally before tag creation.
  - The publish workflow reruns the same or equivalent checks before publishing.

### Publish Workflow Run

- **Description**: A GitHub Actions run triggered by pushing a release tag.
- **Fields**:
  - `tag`: `Release Version.tag`
  - `commit_sha`: commit referenced by the tag
  - `is_on_main`: boolean
  - `validation_passed`: boolean
  - `published_artifacts`: list of `Release Artifact` names successfully published
  - `failed_step`: optional identifier of the first failing step
- **State transitions**:
  - `triggered` -> `validated` when tag and artifact checks pass
  - `validated` -> `published` when all artifact publish jobs succeed
  - `triggered` -> `failed` on tag mismatch or validation failure
  - `published_partially` is possible only after immutable registry side effects begin and must be documented for recovery

## Relationships

- One `Release Version` maps to many `Release Artifact` instances.
- One `Release Preparation Run` produces at most one `Release Version`.
- One `Publish Workflow Run` consumes one prepared `Release Version`.
- The `Validation Matrix` is shared by both local preparation and remote publishing.

## Release-managed Manifest Set

The implementation should treat these paths as the initial authoritative version set:

- `Cargo.toml`
- `crates/gram-codec/Cargo.toml` for the `relateby-pattern` dependency pin
- `typescript/@relateby/pattern/package.json`
- `python/relateby/pyproject.toml`

Additional manifests may remain in the repo temporarily during migration, but they must either be synchronized explicitly or excluded from the supported release path.

## State Transitions

1. `main clean + synced` -> start `Release Preparation Run`
2. `version update complete` -> run `Validation Matrix`
3. `validation passed` -> create release commit and annotated tag
4. `tag pushed` -> start `Publish Workflow Run`
5. `workflow validation passed` -> publish artifacts in dependency order / parallel where safe
6. `all publish jobs succeeded` -> release complete

## Failure Semantics

- **Local precondition failure**: no files updated, no tag created.
- **Local validation failure**: updated manifests may exist in the working tree, but no tag is created.
- **Remote validation failure**: no registry publish occurs.
- **Partial remote publish**: only artifacts already accepted by immutable registries remain published; recovery requires documented follow-up steps and possibly a new patch release.
