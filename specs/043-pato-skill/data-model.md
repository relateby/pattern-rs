# Data Model: `pato skill`

## Canonical Skill Package

**Purpose**: Represents the single source-of-truth skill artifact stored in the
repository and bundled by the CLI for installation.

**Fields**:
- `name`: skill identifier; must be `pato`
- `source_root`: repository path to the canonical package; must be `.agents/skills/pato/`
- `entry_file`: required skill entry point; must be `SKILL.md`
- `supporting_files`: zero or more files under approved subdirectories such as
  `references/` and `assets/`
- `bundle_version`: version of the package materialized into the CLI build

**Validation Rules**:
- `SKILL.md` must exist
- `name` in frontmatter must match the directory name
- package contents must be installable as a complete directory tree
- the repository must not contain a second authoritative checked-in copy of the same
  skill package

**Relationships**:
- One `Canonical Skill Package` can produce many `Installed Skill` instances

## Installation Request

**Purpose**: Captures a single invocation of `pato skill`.

**Fields**:
- `scope`: `project` or `user`
- `target_convention`: `interoperable` or `client-native`
- `allow_replace`: whether an existing install may be replaced
- `print_path_only`: whether the caller asked to print the resolved destination path

**Validation Rules**:
- `scope=project` requires a Vercel-discoverable install target
- `scope=project` must reject unsupported client-native project locations
- `allow_replace=false` must not overwrite an existing installed skill

**Relationships**:
- One `Installation Request` resolves to exactly one `Install Target`
- One `Installation Request` may create or replace one `Installed Skill`

## Install Target

**Purpose**: Represents the resolved filesystem destination for a requested install.

**Fields**:
- `scope`: `project` or `user`
- `target_convention`: `interoperable` or `client-native`
- `resolved_path`: absolute destination path on disk
- `is_vercel_discoverable`: whether the destination is valid for Vercel project
  discovery requirements

**Validation Rules**:
- project-scope targets must resolve under `.agents/skills/`
- user-scope interoperable targets must resolve to the configured interoperable user
  location
- user-scope client-native targets must resolve to the configured client-native user
  location

**Relationships**:
- One `Install Target` may contain zero or one `Installed Skill` for the `pato` name

## Installed Skill

**Purpose**: Represents a local materialization of the canonical skill package.

**Fields**:
- `name`: installed skill name; must be `pato`
- `destination_path`: path of the installed skill root
- `installed_from`: canonical source identity used for the install
- `replaced_existing`: whether the install replaced a prior copy
- `status`: `created`, `replaced`, or `failed`

**Validation Rules**:
- installed contents must be functionally equivalent to the canonical skill package
- successful installs must include `SKILL.md`
- failed installs must not be reported as usable installs

**State Transitions**:
- `absent -> created`
- `existing -> replaced` when explicit replacement is allowed
- `absent|existing -> failed` when validation, path resolution, or filesystem write
  checks fail
