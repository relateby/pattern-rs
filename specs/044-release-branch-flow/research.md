# Research: Release Branch Workflow

## Decision 1: Use dedicated `release/vX.Y.Z` branches for release preparation

- **Decision**: Prepare each release on a branch named for the target version, such as `release/v0.2.0`.
- **Rationale**: This keeps release-only changes off `main` until they are reviewed and validated, which matches the repository policy that changes should happen in a branch.
- **Alternatives considered**:
  - Continue preparing releases directly on `main`. Rejected because it creates release-side effects on the integration branch and makes recovery noisy.
  - Use a temporary validation branch with an unrelated name. Rejected because version-linked branch names are easier to identify and audit.

## Decision 2: Validate before creating the stable tag

- **Decision**: Run release validation on the release branch before the stable `vX.Y.Z` tag is created.
- **Rationale**: The reported failure mode comes from validation failing after the release version has effectively been reserved. Delaying the tag avoids dangling stable tags for pre-publish failures.
- **Alternatives considered**:
  - Create the tag first and validate afterward. Rejected because it keeps the current failure mode intact.
  - Create lightweight candidate tags for validation. Rejected because it introduces extra tag lifecycle complexity without solving the main problem as cleanly as branch-based validation.

## Decision 3: Keep the stable tag as the publish trigger, but only after release readiness is confirmed

- **Decision**: Retain `vX.Y.Z` as the publish trigger, but create it only after the release branch has passed validation and been finalized.
- **Rationale**: This preserves the existing publish pipeline and registry ordering while moving the risky validation step earlier in the lifecycle.
- **Alternatives considered**:
  - Move publishing into the branch workflow and tag afterward. Rejected because it would require a broader publish orchestration change and complicate the existing GitHub Actions flow.
  - Publish from a manually triggered workflow dispatch only. Rejected because it weakens the repository’s simple tag-driven release signal.

## Decision 4: Treat any published version as immutable

- **Decision**: Once any registry has accepted a version, the release process will require a new patch version for further changes.
- **Rationale**: Registry immutability is the safest recovery rule and matches the existing release documentation policy.
- **Alternatives considered**:
  - Allow republishing the same version after a partial failure. Rejected because external registries do not guarantee safe overwrite semantics.

## Decision 5: Model the feature as workflow orchestration, not application code

- **Decision**: Limit implementation work to shell scripts, GitHub Actions workflows, and release documentation.
- **Rationale**: The feature changes how a release is prepared and finalized, not how the library behaves at runtime.
- **Alternatives considered**:
  - Add runtime code to manage release state. Rejected because the repo already uses shell and CI workflows for release management.

## Consolidated Outcome

The release process should become:

1. Cut a version-specific release branch from `main`.
2. Apply the version bump and any release-only fixes on that branch.
3. Run release validation on the branch.
4. Merge the release branch back to `main`.
5. Create the stable `vX.Y.Z` tag from the finalized release commit.
6. Let the existing tag-triggered publish workflow perform registry publication.
