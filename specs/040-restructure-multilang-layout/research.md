# Research: Multi-Language Repository Restructure

**Feature**: `040-restructure-multilang-layout`  
**Date**: 2026-03-18

## Decision 1: Normalize TypeScript and Python package roots without changing published identities

- **Decision**: Move the TypeScript packages to `typescript/packages/{pattern,graph,gram}` and move the Python distribution root to `python/packages/relateby`. Keep the published package identities unchanged: npm remains `@relateby/pattern`, `@relateby/graph`, and `@relateby/gram`; the Python distribution remains `relateby-pattern`; and Python imports remain `relateby.pattern` and `relateby.gram`. Keep the repo-root `package.json` as the npm workspace entrypoint in this feature and update its workspace globs to the new package paths.
- **Rationale**: The clarified feature scope promotes all three TypeScript packages to supported public surfaces, so an `internal/` split would now contradict the spec. Preserving the root npm workspace entrypoint minimizes churn in CI, release automation, and local workflows while still making the package layout visibly multi-language and package-oriented.
- **Alternatives considered**:
  - Keep the current `typescript/@relateby/*` and `python/relateby` layout. Rejected because it does not deliver the requested medium-churn restructure.
  - Move the npm workspace root into `typescript/package.json` immediately. Rejected for this feature because it adds unnecessary lockfile and workflow churn.
  - Use `typescript/packages/public/{pattern,graph,gram}`. Rejected because the extra nesting adds churn without improving clarity once all three packages are public.

## Decision 2: Move the WASM adapter crate out of `crates/` and keep workspace inclusion explicit

- **Decision**: Move `pattern-wasm` from `crates/pattern-wasm` to `adapters/wasm/pattern-wasm` in a dedicated adapter-focused phase. Keep the crate/package identity unchanged, switch the root Cargo workspace from wildcard-only membership to an explicit mixed member list, and update all active path-based callers before the move lands.
- **Rationale**: `pattern-wasm` must remain discoverable, but as an adapter rather than a peer implementation crate. Handling the workspace and path callers first keeps the move low-risk while making the adapter boundary visible in both the tree and contributor guidance.
- **Alternatives considered**:
  - Leave `pattern-wasm` under `crates/` and only clarify it in docs. Rejected because the tree would still imply false symmetry among Rust crates.
  - Split adapters into a separate Cargo workspace. Rejected because it would break current `cargo --workspace` assumptions and add unnecessary migration risk.
  - Combine the WASM move with the TypeScript package-root move. Rejected because it would stack two rounds of relative-path churn into one harder-to-debug change.

## Decision 3: Use a docs-first phased cleanup for examples and historical material

- **Decision**: Use a docs-first, archive-first migration. Rewrite `README.md` and example indexes first, establish `examples/archive` and `docs/archive` as the canonical homes for legacy material, classify current examples into active versus legacy buckets, and archive superseded examples in-repo rather than deleting them by default.
- **Rationale**: The feature now explicitly preserves legacy examples in-repo, so the main requirement is to separate active guidance from historical material without losing migration context. Updating the root-facing docs first reduces confusion while path moves are phased in later.
- **Alternatives considered**:
  - Perform a full package-root migration before rewriting docs. Rejected because active documentation would remain misleading during the move.
  - Clean docs only and leave old example paths in place. Rejected because stale example names would continue to encode superseded package boundaries.
  - Delete historical notes and examples immediately. Rejected because the feature now explicitly prefers in-repo archival over deletion by default.

## Decision 4: Validate the restructure as a no-behavior-change migration

- **Decision**: Treat the feature as a structural and packaging migration with zero intended public behavior change. Acceptance requires representative verification across Rust, WASM, all three public TypeScript packages, Python packaging, workflow validation, example discoverability, and stale-path auditing in active docs/examples before deleting root `src/`.
- **Rationale**: The constitution emphasizes correctness, compatibility, and multi-target support. Because this feature is not a behavior port, the safest way to comply is to prove that public package identities and representative workflows still work after the path changes.
- **Alternatives considered**:
  - Validate only the moved paths and skip representative build/test flows. Rejected because it would not protect multi-target compatibility.
  - Couple the restructure to public API or packaging changes. Rejected because it expands scope and increases the risk of accidental behavioral regressions.

## Phase Sequencing Outcome

1. Update root-facing documentation, example indexes, and archive boundaries.
2. Update the Cargo workspace definition and move the WASM adapter crate.
3. Update npm workspace globs and move the three public TypeScript packages.
4. Move the Python distribution root and update Python release/build references.
5. Reorganize active examples by language and archive obsolete ones in-repo.
6. Remove the stale root `src/` only after all active references are gone.

## Active Reference Audit (Phase 1)

- **README.md**: Still presents the repository primarily as a Rust/Cargo workspace, lists only `@relateby/pattern` as the npm artifact, and points users to the legacy `examples/wasm-js/` path.
- **examples/README.md**: Still organizes examples by historical package boundaries, keeps `wasm-js/` as an active legacy example, and includes build flows rooted in old crate/package paths.
- **docs/release.md**: Still treats `typescript/@relateby/pattern/package.json` and `python/relateby/pyproject.toml` as release-managed roots and only names `@relateby/pattern` as the npm release artifact.
- **docs/python-usage.md**: Still instructs source builds from `python/relateby`.
- **.github/workflows/ci.yml** and **.github/workflows/publish.yml**: Still validate and publish only `@relateby/pattern` and still use `python/relateby` working directories.

## Phase 4 Archive Classification

- **Archive** `docs/TOP-LEVEL-MD-REVIEW.md` into `docs/archive/`. It is a historical repository cleanup review, not current contributor guidance.
- **Archive** `examples/wasm-js/`, `examples/pattern-core-wasm/`, `examples/gram-codec-wasm-web/`, and `examples/gram-codec-wasm-node/` into `examples/archive/`. These examples preserve useful migration and historical context, but they no longer represent the active public package layout for this feature.
- **Keep active** the Rust and Python example guides under `examples/` and route current WASM/package usage guidance through root-facing docs such as `docs/wasm-usage.md`.
- **Remove** the root `src/` directory in this phase. It contains only a placeholder `src/lib.rs`, and active-reference validation for `README.md`, `docs/`, `scripts/`, and `.github/workflows/` must be clean before deletion.

## Scope Guard

- This feature is limited to the medium-churn repository layout defined in `plan.md`.
- The implementation must not introduce a fully symmetric `implementations/`-style top-level redesign.
- Package identity, public imports, and behavior remain stable while filesystem layout and contributor guidance are reorganized.

## Phase 6 Reference Fidelity Review

- Reviewed authoritative Haskell implementation sources in `../pattern-hs/libs/`, including `pattern/src/Pattern/Core.hs` and `pattern/src/Pattern/Graph/GraphClassifier.hs`.
- Reviewed `../pattern-hs/specs/README.md`, which explicitly describes `../pattern-hs/specs/` as historical development context rather than the current behavioral source of truth.
- Conclusion: this feature changes repository structure, packaging roots, examples, and contributor guidance only. It introduces no intentional behavioral deviation from the `pattern-hs` reference because the authoritative behavior remains defined by `../pattern-hs/libs/` and was not altered by this migration.

## Phase 6 Onboarding Review Sample

- First-attempt onboarding review result: `10/10` tasks passed on 2026-03-18.
- Completed checks:
  - Locate `crates/pattern-core`
  - Locate `crates/gram-codec`
  - Locate `python/packages/relateby`
  - Locate all three public TypeScript package roots in `typescript/packages/`
  - Locate `adapters/wasm/pattern-wasm`
  - Locate active examples under `examples/rust`, `examples/python`, and `examples/typescript`
  - Locate archive roots in `examples/archive` and `docs/archive`
  - Find `docs/release.md`
  - Find `docs/python-usage.md`
  - Confirm the root `src/` directory is absent
