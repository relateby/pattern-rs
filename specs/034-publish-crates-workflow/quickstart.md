# Quickstart: Publishing a Release

**Feature**: 034-publish-crates-workflow  
**Audience**: Maintainers performing a release

## Prerequisites

- Push access to the repository.
- crates.io account with permission to publish the crates (relateby-pattern, relateby-gram).
- A crates.io API token (create at [crates.io/settings/tokens](https://crates.io/settings/tokens)).
- GitHub secret `CARGO_REGISTRY_TOKEN` set to that token (Settings → Secrets and variables → Actions).

## Release steps

1. **Ensure the tree is releasable**
   - Run locally: `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, `cargo fmt --all -- --check`.
   - Optionally: `cargo publish -p relateby-pattern --dry-run` and `cargo publish -p relateby-gram --dry-run` (after relateby-pattern has a version in Cargo.toml for gram-codec dependency).

2. **Bump versions** (if not already bumped)
   - Update `version` in workspace root `Cargo.toml` and/or in each publishable crate’s `Cargo.toml` so they match the release version (e.g. `0.1.0`).
   - Ensure gram-codec’s dependency on relateby-pattern uses that same version for publish.

3. **Commit and push**
   - Commit version and any release-related changes; push to the default branch.

4. **Create and push the version tag**
   - Tag format: `v<major>.<minor>.<patch>` (e.g. `v0.1.0`).
   - Example: `git tag v0.1.0 && git push origin v0.1.0`.

5. **Trigger the workflow**
   - Pushing the tag starts the publish workflow.
   - Monitor the run under Actions; it will build, test, lint, then publish relateby-pattern, then relateby-gram.

6. **Verify**
   - Check crates.io for the new versions.
   - Check docs.rs for updated API docs (may take a few minutes).

## If something fails

- **Build/test/lint fails**: Fix the failure, commit, and push a new tag (you must use a new version; you cannot re-push the same tag to re-run for the same version).
- **relateby-pattern publishes, relateby-gram fails**: Either fix the issue and re-run (e.g. trigger workflow again if the job supports re-run) or publish relateby-gram manually with `cargo publish -p relateby-gram --token <token>` from a checkout at the same tag. Do not re-publish relateby-pattern at the same version (crates.io will reject it).
- **Duplicate version**: crates.io does not allow re-publishing the same version. Bump to a new patch version, commit, and push a new tag.

## Dry-run (local)

Without publishing, from repo root:

```bash
cargo publish -p relateby-pattern --dry-run
# Fix any path dependency in gram-codec Cargo.toml to use version = "0.1.0" for relateby-pattern, then:
cargo publish -p relateby-gram --dry-run
```

Revert the gram-codec dependency change if you are not publishing yet.

## Full details

See `docs/release.md` (or equivalent) for full publishing flow, tag format, secrets, and recovery.
