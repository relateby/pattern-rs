# Data Model: Release Branch Workflow

This feature does not introduce a persistent application datastore. The model below describes the release lifecycle concepts that the scripts and workflows must coordinate.

## Entities

### Release Branch

- **Purpose**: Represents the branch used to prepare a single versioned release.
- **Key attributes**:
  - Version identifier
  - Branch name
  - Source branch
  - Current lifecycle state
  - Last validation result
- **Relationships**:
  - Created from `main`
  - Can become a finalized release candidate
  - May be merged back into `main`

### Release Candidate

- **Purpose**: Represents a release branch that has passed validation and is eligible for finalization.
- **Key attributes**:
  - Version identifier
  - Validation timestamp
  - Validation outcome
  - Merge status
- **Relationships**:
  - Derived from a release branch
  - Precedes stable tag creation

### Stable Tag

- **Purpose**: Represents the immutable public release marker `vX.Y.Z`.
- **Key attributes**:
  - Tag name
  - Version
  - Commit SHA
  - Creation timestamp
- **Relationships**:
  - Created from a finalized release candidate
  - Triggers publish workflows
  - Must not exist before successful validation

### Published Release

- **Purpose**: Represents the externally published artifacts for a version.
- **Key attributes**:
  - Version
  - Registry acceptance state
  - Publish completion status
  - Artifact families published
- **Relationships**:
  - Created after the stable tag exists
  - Makes the version immutable for future release attempts

## State Transitions

### Release Branch

1. `created`
2. `version_bumped`
3. `validated`
4. `merged`
5. `finalized`

### Stable Tag

1. `absent`
2. `created`
3. `immutable`

### Published Release

1. `not_published`
2. `partially_published`
3. `fully_published`
4. `immutable`

## Validation Rules

- A stable tag may only be created from a release branch that has passed validation.
- A release branch may be revalidated after a fix as long as no registry has accepted the version.
- Once any registry accepts the version, the release becomes immutable and must move to a new patch version.
- A finalized release must have a clear merge path back to `main`.
