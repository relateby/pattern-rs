# Quickstart: Release Branch Workflow

## Goal

Prepare a release on a dedicated branch, validate it before tagging, and only then publish the stable version.

## Typical Flow

1. Start from an up-to-date `main` branch.
2. Create a release branch for the target version, such as `release/v0.2.0`.
3. Apply the version bump and any release-only fixes on the release branch.
4. Run release validation for the branch.
5. Push the branch and open a PR to `main`.
6. Merge the release branch back to `main` once validation passes.
7. Run `./scripts/release/finalize-release.sh 0.2.0 --push` from `main` to create the stable tag.
8. Let the tag-triggered publish workflow publish the release artifacts.

## Expected Outcomes

- Validation failures do not create stable tags.
- Release fixes can be made on the same release branch before anything is published.
- Once the stable tag exists and publishing begins, the version is treated as immutable.

## Maintainer Checks

- Confirm the release branch name matches the target version.
- Confirm the release branch is based on `main`.
- Confirm the release validation run completes successfully before tagging.
- Confirm no public registry has already accepted the version before reusing it.
