# Implementation Plan: Multi-Language Repository Restructure

**Branch**: `040-restructure-multilang-layout` | **Date**: 2026-03-18 | **Spec**: `/Users/akollegger/Developer/gram-data/pattern-rs/specs/040-restructure-multilang-layout/spec.md`
**Input**: Feature specification from `/specs/040-restructure-multilang-layout/spec.md`

## Summary

Restructure the repository into a clearer medium-churn multi-language layout that makes peer implementations, adapter layers, active examples, and archived material obvious from the tree. The implementation keeps public package identities stable, promotes all three TypeScript packages as supported public surfaces, preserves `pattern-wasm` as a discoverable adapter package, archives legacy examples in-repo, and removes the stale root `src/` once all active references are gone.

## Scope Guard

- Deliver only the medium-churn layout captured in this plan.
- Do not introduce a fully symmetric `implementations/`-style repository redesign in this feature.
- Preserve public package identities and behavior while changing only repository structure, packaging metadata, and contributor guidance.

## Technical Context

**Language/Version**: Rust 1.70.0 workspace, TypeScript 5.x workspace packages, Python `>=3.8,<3.14` packaging, Bash automation, Markdown documentation  
**Primary Dependencies**: Cargo workspaces, npm workspaces, wasm-pack, maturin, `uv`, pytest, vitest, GitHub Actions, release scripts, repository documentation  
**Storage**: N/A  
**Testing**: `cargo build --workspace`, `cargo test --workspace`, `cargo build --workspace --target wasm32-unknown-unknown`, `npm run build --workspace=@relateby/pattern`, `npm run test --workspace=@relateby/pattern`, `npm run build --workspace=@relateby/graph`, `npm run test --workspace=@relateby/graph`, `npm run build --workspace=@relateby/gram`, `npm run test --workspace=@relateby/gram`, Python package tests and wheel validation from `python/packages/relateby`, `./scripts/check-workflows.sh`, `./scripts/ci-local.sh`  
**Target Platform**: Contributor workstations and CI for native Rust, WASM, npm packaging, Python packaging, and documentation/example validation  
**Project Type**: Multi-language monorepo for library packages, bindings, and adapter surfaces  
**Performance Goals**: Contributors can classify the primary repository areas within 2 minutes from the root; active docs and examples reference canonical current paths only; at least 90% of sampled onboarding tasks succeed on first attempt during review  
**Acceptance Review Sample Set**: Use 10 onboarding tasks drawn from the repository root: locate the Rust peer libraries, locate the Python distribution root, locate all three public TypeScript package roots, locate the `pattern-wasm` adapter, locate the active examples root, locate the archive roots, find the release guide, find the Python usage guide, and confirm root `src/` is absent  
**Constraints**: Preserve published package names and import surfaces, avoid behavior changes, keep all supported build/test workflows working, limit scope to the medium-churn layout, keep `pattern-wasm` discoverable as an adapter, archive legacy examples in-repo, and remove root `src/` only after reference cleanup  
**Scale/Scope**: Root workspace manifests, 2 peer Rust crates, 1 WASM adapter crate, 3 public TypeScript packages, 1 Python distribution root, root-facing docs, example collections, CI/release scripts, and stale top-level paths such as root `src/`

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Pre-Phase 0 Gate Review

- **I. Reference Implementation Fidelity**: PASS. This feature changes repository layout, guidance, and packaging boundaries rather than gram-hs behavior. The plan keeps public identities stable and validates representative flows so reference-aligned behavior is not accidentally changed.
- **II. Correctness & Compatibility**: PASS. Public package identities and imports remain stable while filesystem locations and guidance are reorganized. Any packaging metadata changes are compatibility-preserving.
- **III. Rust Native Idioms**: PASS. Rust changes are limited to workspace membership, crate path relocation, and documentation updates. No non-idiomatic Rust abstractions are introduced.
- **IV. Multi-Target Library Design**: PASS with validation required. The plan explicitly revalidates native Rust, WASM, TypeScript, and Python flows after path changes.
- **V. External Language Bindings & Examples**: PASS. Active examples remain required, legacy examples are archived rather than lost, and guidance is updated to reflect current public surfaces.

**Gate Result**: PASS. No unjustified constitution violations exist before research.

### Post-Design Re-Check

- The design preserves compatibility of the Rust, TypeScript, Python, and WASM public surfaces while changing only repository structure and packaging metadata.
- The design includes migration guidance, active/archived documentation separation, and representative multi-target validation steps.
- The design keeps the feature scoped to the medium-churn layout and avoids unnecessary re-foundation of workspace entrypoints.

**Post-Design Result**: PASS.

## Project Structure

### Documentation (this feature)

```text
specs/040-restructure-multilang-layout/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── repository-structure.openapi.yaml
│   └── repository-manifest.schema.json
└── tasks.md
```

### Source Code (repository root)

```text
Cargo.toml
package.json
crates/
├── pattern-core/
└── gram-codec/
adapters/
└── wasm/
    └── pattern-wasm/
python/
└── packages/
    └── relateby/
typescript/
└── packages/
    ├── pattern/
    ├── graph/
    └── gram/
examples/
├── rust/
├── python/
├── typescript/
└── archive/
docs/
├── ...
└── archive/
scripts/
specs/
external/
.github/
└── workflows/
tests/
```

**Structure Decision**: Use the medium-churn mixed-root layout. Keep the repo-root Cargo and npm entrypoints in place, but move package roots into role-signaling directories: `crates/` for peer Rust libraries, `adapters/` for the WASM bridge, `typescript/packages/` for all three public TypeScript packages, `python/packages/` for the Python distribution root, and language-bucketed active examples with explicit archive areas. Remove the stale root `src/` only after all active references have been eliminated.

## Complexity Tracking

No constitution violations or justified exceptions were identified for this feature.

## Phase 6 Validation Evidence

- `./scripts/check-workflows.sh` passed on 2026-03-18 after the workflow and release-path updates.
- `./scripts/ci-local.sh` passed on 2026-03-18 and re-ran the representative quickstart compatibility checks for format/lint, native Rust build/test, docs, `wasm32-unknown-unknown`, all three npm packages, and the combined Python wheel/public API validation.
- Standalone Python package verification also passed on 2026-03-18 with `cd python/packages/relateby && .venv/bin/python -m pytest tests` (`27 passed, 6 skipped`).
- The final stale-path audit for `typescript/@relateby/`, `python/relateby`, `crates/pattern-wasm`, and root `src/` references reported no active matches in `README.md`, `docs/`, `examples/`, `scripts/`, or `.github/workflows/`; remaining legacy path mentions are confined to archived historical examples.
