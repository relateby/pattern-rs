# Data Model: Multi-Language Repository Restructure

**Feature**: `040-restructure-multilang-layout`  
**Date**: 2026-03-18

## Overview

This feature does not add runtime domain data. Its design model is a repository-governance model used to classify paths, define canonical package surfaces, preserve public identities during moves, archive legacy examples in-repo, and validate migration outcomes.

## Entity: RepositoryArea

**Purpose**: Represents a top-level or nested directory that has a clearly defined repository role.

**Fields**

- `path`: Repository-relative path.
- `role`: One of `peer-implementation`, `adapter`, `example`, `support`, `archive`.
- `language_scope`: One of `rust`, `python`, `typescript`, `cross-language`, `repo-wide`.
- `status`: One of `active`, `archived`, `removed`, `planned`.
- `canonical`: Boolean indicating whether this is the single active path for its purpose.
- `notes`: Short migration or discoverability notes for contributors.
- `public_surface`: Boolean indicating whether the area contains user-facing package surfaces.

**Relationships**

- A `RepositoryArea` may contain zero or more `PackageSurface` records.
- A `RepositoryArea` may contain zero or more `ExampleCollection` records.
- A `RepositoryArea` may be referenced by zero or more `DocumentationEntry` records.

**Validation Rules**

- Every active repository area must have exactly one role.
- No active purpose may have more than one canonical path.
- Archived areas must not be presented as active entry points in root-facing guidance.
- Areas containing public package surfaces must be discoverable from root-facing guidance.

**State Transitions**

- `planned -> active`
- `active -> archived`
- `active -> removed`

## Entity: PackageSurface

**Purpose**: Represents a package or crate root that contributors or users may interact with directly.

**Fields**

- `name`: Public or internal package identity.
- `path`: Canonical repository-relative package root.
- `ecosystem`: One of `cargo`, `npm`, `python`.
- `visibility`: One of `public`, `internal`, `adapter`.
- `published_identity`: External package/crate/distribution name that must remain stable for this feature.
- `import_surface`: Canonical import or package surface exposed to users.
- `status`: One of `active`, `archived`, `planned`.
- `depends_on`: Optional list of related adapter or peer surfaces.

**Relationships**

- A `PackageSurface` belongs to one `RepositoryArea`.
- A public `PackageSurface` may depend on one or more adapter or peer `PackageSurface` records.
- A `PackageSurface` may have one or more `ExampleCollection` records that demonstrate it.

**Validation Rules**

- Public package identities must remain unchanged across the restructure.
- All three TypeScript package surfaces in this feature are public and must be described as such in active docs.
- Internal packages must not be advertised as public package surfaces in active docs.
- Adapter package surfaces must remain discoverable but must not be described as peer public implementations.
- `pattern-wasm` must remain discoverable as an adapter package rather than a peer implementation surface.

**State Transitions**

- `planned -> active`
- `active -> archived`

## Entity: ExampleCollection

**Purpose**: Represents a grouped set of examples aligned to a current language surface or retained as legacy material.

**Fields**

- `path`: Canonical repository-relative path.
- `language`: One of `rust`, `python`, `typescript`, `cross-language`.
- `status`: One of `active`, `archived`, `planned`, `removed`.
- `surface_focus`: The package surface or contributor flow the example demonstrates.
- `entry_commands`: Representative commands used for validation.
- `legacy_source`: Optional prior path if the collection was moved or archived.
- `archive_reason`: Optional explanation for why a legacy example was archived instead of removed.

**Relationships**

- An `ExampleCollection` belongs to one `RepositoryArea`.
- An `ExampleCollection` may support one or more `PackageSurface` records.
- An `ExampleCollection` may be referenced by one or more `DocumentationEntry` records.

**Validation Rules**

- Active examples must live under language-oriented active buckets.
- Archived examples must not be linked as current examples from root-facing guidance.
- Every supported public package surface should have at least one discoverable active example or usage guide.
- Legacy examples are archived in-repo by default unless explicitly classified as safe to remove.

**State Transitions**

- `planned -> active`
- `active -> archived`
- `active -> removed`

## Entity: DocumentationEntry

**Purpose**: Represents a root-facing, language-facing, or archival document that influences contributor navigation.

**Fields**

- `path`: Repository-relative path.
- `audience`: One of `user`, `contributor`, `release`, `historical`.
- `status`: One of `active`, `archived`, `planned`.
- `topics`: Set of repository topics covered by the document.
- `references`: List of package, example, or area paths mentioned as current.
- `canonical`: Boolean indicating whether the document is the preferred active guide for its topic.

**Relationships**

- A `DocumentationEntry` may reference multiple `RepositoryArea`, `PackageSurface`, and `ExampleCollection` records.

**Validation Rules**

- Active documentation must reference canonical current paths only.
- Historical documentation retained in the repository must be archived and not treated as normative guidance.
- Root-facing guidance must describe Rust, Python, and TypeScript as peer implementation areas where applicable.

## Entity: MigrationMapping

**Purpose**: Represents the movement, archival, or removal of an existing path.

**Fields**

- `source_path`: Existing repository-relative path.
- `target_path`: New repository-relative path, if retained.
- `disposition`: One of `move`, `archive`, `remove`, `retain`.
- `reason`: Short explanation for the decision.
- `user_impact`: One of `none`, `low`, `medium`, `high`.
- `validation_status`: One of `planned`, `updated`, `verified`.
- `requires_archive`: Boolean indicating whether the source must remain available in an archive location.

**Relationships**

- A `MigrationMapping` may update one `RepositoryArea`, `PackageSurface`, `ExampleCollection`, or `DocumentationEntry`.

**Validation Rules**

- Every moved or archived active path must have its current references updated in active documentation and scripts.
- Paths marked `remove` must have no remaining active references at the time of deletion.
- High-impact migrations require explicit contributor guidance.
- The root `src/` path may transition only from `planned -> removed` after active-reference validation is complete.

**State Transitions**

- `planned -> updated -> verified`
