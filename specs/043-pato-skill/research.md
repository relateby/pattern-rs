# Research: `pato skill`

## Decision 1: Canonical skill source lives at `.agents/skills/pato/`

**Decision**: Keep `.agents/skills/pato/` as the single checked-in source-of-truth for
the bundled `pato` skill package.

**Rationale**:
- The feature spec explicitly requires `.agents/skills/pato/` as the canonical
  repository location.
- Vercel's `skills` tooling discovers project skills from `.agents/skills/`, so using
  that path directly keeps the repository immediately compatible with project-level
  auto-discovery.
- One canonical tree avoids drift between a repo-facing copy and a crate-private copy.

**Alternatives considered**:
- `crates/pato/skills/pato/` as the checked-in source: rejected because it would
  separate the canonical tree from the Vercel-discoverable project location and would
  conflict with the spec clarification.
- Keeping both `.agents/skills/pato/` and `crates/pato/skills/pato/` checked in:
  rejected because it creates two authoritative in-repo copies that must be kept in
  sync.

## Decision 2: Project installs must always target `.agents/skills/`

**Decision**: For project-scope installs, `pato skill` will always install to a
Vercel-discoverable `.agents/skills/` path and will not support project-level
destinations that Vercel's tooling would skip.

**Rationale**:
- The feature spec requires project installs to be automatically discoverable by
  Vercel skills tooling.
- Vercel's published discovery list includes `.agents/skills/` but not project-level
  `.cursor/skills/`.
- This removes ambiguity in acceptance tests and makes project installs interoperable
  by default.

**Alternatives considered**:
- Allowing project-level `.cursor/skills/`: rejected because it would violate the
  clarified compatibility requirement.
- Adding a third special Vercel target: rejected as unnecessary because the
  interoperable project path already satisfies the requirement.

## Decision 3: User-scope client-native installs remain supported

**Decision**: Support user-scope installs to both interoperable and client-native
locations, while keeping project-scope installs limited to the interoperable path.

**Rationale**:
- The spec still requires both project and user scope installs.
- User-level client-native locations are valuable for tools that expect a per-user
  directory and are not constrained by the Vercel project-discovery requirement.
- This preserves flexibility without reintroducing ambiguity at project scope.

**Alternatives considered**:
- Supporting only interoperable installs everywhere: rejected because the spec retains
  user-scope client-native support.
- Supporting client-native installs at both scopes: rejected because project-level
  auto-discovery would no longer be guaranteed.

## Decision 4: The crate should consume the canonical skill tree instead of owning a second editable copy

**Decision**: The `relateby-pato` crate should bundle or otherwise consume the canonical
`.agents/skills/pato/` tree for installation behavior, rather than maintaining its own
separately edited skill source under `crates/pato/`.

**Rationale**:
- The spec forbids two separately maintained authoritative copies.
- The CLI must still work when built or installed outside the repository root, so
  purely runtime-relative file lookups are not sufficient for the shipped command.
- A bundling strategy preserves self-contained CLI behavior while keeping only one
  editable source in the repository.

**Alternatives considered**:
- Runtime-only reads from repository-relative paths: rejected because installed binaries
  and packaged crates would not reliably have access to the repository tree.
- A second crate-local checked-in copy: rejected because it recreates the sync problem
  the spec explicitly avoids.

## Decision 5: Packaging verification must be part of the feature's validation strategy

**Decision**: Include packaging-oriented verification in the implementation and test
plan so the bundled skill assets are not accidentally omitted from packaged builds.

**Rationale**:
- The feature depends on non-code assets that must remain available after packaging or
  installation.
- Asset-handling bugs can be masked in repository-local development if tests only read
  from the working tree.
- Verifying package/bundle behavior reduces the risk of a `pato skill` command that
  works in the repo but fails when distributed.

**Alternatives considered**:
- Repository-only tests: rejected because they would not cover packaged/distributed
  behavior.
- Deferring packaging verification to a later release: rejected because bundling is
  central to the feature, not an optional enhancement.
