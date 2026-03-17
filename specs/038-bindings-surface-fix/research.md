# Research: TypeScript and Python Surface Improvements

**Feature**: 038-bindings-surface-fix | **Date**: 2026-03-17

## R1: TypeScript public boundary ownership

**Decision**: Keep `@relateby/pattern` as the only supported public TypeScript package boundary and make its top-level facade a thin, explicit layer over the generated WASM outputs rather than a separately maintained shadow API.

**Rationale**: The existing package already publishes only the top-level entry point while bundling `wasm/` and `wasm-node/` as implementation assets. Preserving that boundary avoids leaking target-specific details, while narrowing the facade to stable aliases and curated exports reduces drift between runtime behavior, generated declarations, handwritten types, and docs.

**Alternatives considered**:
- Re-export the raw generated `wasm/` or `wasm-node/` modules directly: rejected because it exposes target-specific naming and weakens the package boundary.
- Keep the current handwritten facade as an independent public model: rejected because it has already drifted from runtime behavior and generated bindings.

## R2: TypeScript source of truth for exported behavior

**Decision**: Treat the generated WASM declarations and generated runtime export inventory as the source of truth for what the underlying TypeScript/WASM layer actually provides, and limit handwritten TypeScript declarations to deliberate package-level aliases and adapters.

**Rationale**: The generated `pattern_wasm.d.ts` and runtime glue files already reflect the Rust-exported API. Handwritten TypeScript should only define the stable package presentation, not restate the full generated surface from scratch, because restatement is where mismatches appear.

**Alternatives considered**:
- Handwrite the full TypeScript API contract independently of the generated outputs: rejected because it creates a second authoritative surface to maintain.
- Rename the public package surface to mirror raw `Wasm*` generated names exactly: rejected because it would be a needless public-facing disruption when stable aliases can provide a better developer experience.

## R3: Python public boundary ownership

**Decision**: Make the unified `python/relateby/relateby` wrapper package the authoritative public Python surface, with `relateby.pattern` and `relateby.gram` as the only supported imports, and colocate any public stubs with that wrapper layer.

**Rationale**: The repo’s docs and packaging already define those imports as the public API, and the wrapper layer legitimately owns cross-module behavior such as `StandardGraph.from_gram` because the native crates cannot depend on each other freely. Public typing and examples therefore need to describe the wrapper package, not only the crate-local native modules.

**Alternatives considered**:
- Expose `pattern_core` and `gram_codec` directly as public APIs: rejected because it contradicts the documented package boundary.
- Move all wrapper-only public helpers into native modules: rejected because existing crate dependency constraints make that impractical in some cases.

## R4: Verification must target packaged artifacts

**Decision**: Require release-blocking verification against the packaged npm tarball and combined Python wheel, not only source-tree or crate-local tests.

**Rationale**: The failures under review arise in package entry points, wrapper layers, shipped docs, and generated assets, all of which can differ from crate-local behavior. Verifying the packed artifacts is the most reliable way to catch missing exports, wrapper-only bugs, stale stubs, and documentation drift before release.

**Alternatives considered**:
- Rely on existing source-tree tests alone: rejected because they can miss packaging and facade regressions.
- Rely on minimal smoke tests only: rejected because narrow smoke coverage does not catch export-family or type-surface mismatches.

## R5: Verification needs multiple alignment gates

**Decision**: Add four explicit verification layers for this feature area: runtime export inventory checks, top-level package import checks, type/stub consumer validation, and executable documentation/example validation.

**Rationale**: Surface mismatches can exist independently at runtime, in type declarations, in public wrappers, or in docs/examples. A layered verification strategy catches these separately and prevents one successful test layer from masking another broken user-facing artifact.

**Alternatives considered**:
- One broad integration suite only: rejected because failures become harder to localize and some artifact classes still remain untested.
- Manual documentation review: rejected because it is too easy for subtle mismatches to survive.

## R6: Repository-specific constraints to preserve

**Decision**: Preserve the existing public package identities, keep `pattern-wasm` as the cross-crate aggregation point for WASM-facing behavior, and preserve the single-distribution Python packaging model with `relateby-pattern` providing `relateby.pattern` and `relateby.gram`.

**Rationale**: These are already established repository constraints in packaging, release docs, and prior feature work. The feature should improve developer experience inside those boundaries rather than expanding scope into package reshaping or new crate relationships.

**Alternatives considered**:
- Split or rename the public npm or Python package boundaries: rejected because it creates migration work unrelated to the immediate user pain.
- Rework cross-crate ownership of parsing and graph helpers: rejected because it would introduce broader architectural change than needed for the surface-alignment goal.
