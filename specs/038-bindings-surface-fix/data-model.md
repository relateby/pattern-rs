# Data Model: TypeScript and Python Surface Improvements

**Feature**: 038-bindings-surface-fix | **Date**: 2026-03-17

## Overview

This feature does not add new end-user domain entities. It defines and aligns the developer-facing artifacts that together make up the supported TypeScript and Python experience.

## Core Entities

### Public API Surface

| Field | Type | Notes |
|-------|------|-------|
| language | `TypeScript \| Python` | Indicates which external language surface is being described |
| package_entrypoint | `string` | Supported import boundary for external developers |
| public_symbols | `list[string]` | Classes, functions, constants, helpers, and error types expected at that boundary |
| alias_map | `list[Public Alias]` | Maps curated public names to underlying generated/native symbols where applicable |
| unsupported_symbols | `list[string]` | Internal or generated-only names that must not be documented as stable public API |

**Validation rules**:
- Every documented public symbol must be importable from the supported package entry point.
- Every imported symbol must have matching runtime behavior and developer guidance.
- Alias mappings may curate or rename underlying symbols, but may not materially change documented behavior without explicit documentation.

### Developer Workflow

| Field | Type | Notes |
|-------|------|-------|
| workflow_name | `string` | Human-readable task such as create, parse, query, validate, or inspect |
| language | `TypeScript \| Python` | Workflow is validated per language |
| required_symbols | `list[string]` | Public symbols needed to complete the workflow |
| expected_outcome | `string` | User-visible result promised by docs and examples |
| failure_mode | `Public Failure Mode` | Documented error or recovery behavior for invalid input/setup |

**Validation rules**:
- Shared workflows must have equivalent feature-level outcomes in both languages.
- Common workflows must be completable without importing internal modules or generated artifacts.
- Invalid-input behavior must be categorized and documented at the public package layer.

### Developer Guidance Asset

| Field | Type | Notes |
|-------|------|-------|
| asset_type | `Type definition \| Stub \| Guide \| Example \| Quickstart \| Smoke test` | Kind of developer-facing artifact |
| scope | `TypeScript \| Python \| Shared` | Language or audience scope |
| referenced_symbols | `list[string]` | Public symbols and workflows the asset describes |
| validation_mode | `Executed \| Type-checked \| Reviewed via contract test` | How the asset is verified |

**Validation rules**:
- Every official guide or example must reference only supported public imports.
- Type definitions and stubs must describe the shipped public package surface, not a different internal layer.
- Assets that teach a shared workflow must agree on the feature-level behavior across languages.

### Public Failure Mode

| Field | Type | Notes |
|-------|------|-------|
| category | `Missing export \| Invalid input \| Missing setup \| Unsupported usage \| Documentation drift` | Normalized public-facing failure class |
| surfaced_by | `TypeScript facade \| Python wrapper \| Generated binding \| Native binding` | Layer through which the user encounters the error |
| recovery_guidance | `string` | What the user should do next |
| expected_consistency | `string` | Cross-language or cross-artifact consistency requirement |

**Validation rules**:
- Failures must surface through the documented public workflow rather than requiring internal-layer debugging.
- Error categories and recovery guidance must match the docs and quickstart material.

## Supporting Entities

### Public Alias

| Field | Type | Notes |
|-------|------|-------|
| public_name | `string` | Stable name exposed from the supported package boundary |
| underlying_name | `string` | Generated or native symbol name used internally |
| intent | `string` | Why the alias exists (stability, naming consistency, packaging abstraction) |

### Verification Artifact

| Field | Type | Notes |
|-------|------|-------|
| artifact_name | `string` | Named validation asset or test suite |
| package_scope | `npm package \| Python wheel \| source tree` | Where the verification runs |
| coverage_target | `Exports \| Types/stubs \| Docs/examples \| Workflow behavior` | What it protects |
| release_blocking | `bool` | Whether failure blocks release readiness |

## Relationships

- A **Public API Surface** contains many **Public Aliases** and supports many **Developer Workflows**.
- A **Developer Workflow** is taught by one or more **Developer Guidance Assets**.
- A **Developer Workflow** may fail with one or more **Public Failure Modes**.
- A **Verification Artifact** validates one or more combinations of **Public API Surface**, **Developer Workflow**, and **Developer Guidance Asset**.

## Lifecycle

### Public Surface Alignment Lifecycle

1. Underlying generated or native bindings define the available behavior.
2. The package facade curates that behavior into the supported public entry point.
3. Type definitions or stubs describe the supported public entry point.
4. Docs and examples teach the supported workflows.
5. Verification artifacts confirm that runtime behavior, public imports, type/stub surfaces, and docs/examples still agree.

### Regression Lifecycle

1. A wrapper, generated artifact, or guide changes.
2. Export inventory, consumer checks, or executable examples detect a mismatch.
3. The mismatch is either resolved by realigning the public surface or by removing unsupported guidance.
4. The public contract returns to a consistent state before release.
