# Quickstart: Multi-Language Repository Restructure

**Feature**: `040-restructure-multilang-layout`

## Goal

Apply the repository restructure in a sequence that preserves public package identities and keeps the repository coherent at every step.

## Suggested Implementation Order

1. Rewrite root-facing guidance first.
   - Update `README.md` and example index documents so the repository is described as a multi-language monorepo.
   - Introduce archive destinations for non-normative docs and examples.

2. Prepare migration inventories before moving paths.
   - Audit active references to the current TypeScript, Python, adapter, example, and root `src/` paths.
   - Separate active references from historical references so only active guidance must be updated in the same phase.

3. Move the WASM adapter crate.
   - Update the root Cargo workspace membership to include the adapter path explicitly.
   - Update any build, docs, or release references that still point at `crates/pattern-wasm`.
   - Move the crate into `adapters/wasm/pattern-wasm`.

4. Move the TypeScript and Python package roots.
   - Update the root npm workspace globs to the new TypeScript package layout.
   - Move the TypeScript packages into `typescript/packages/{pattern,graph,gram}`.
   - Move the Python distribution root into `python/packages/relateby`.
   - Preserve package names, import names, and public package identities.
   - Promote `@relateby/graph` and `@relateby/gram` as supported public package surfaces alongside `@relateby/pattern`.

5. Reorganize examples.
   - Move active examples into `examples/rust`, `examples/python`, and `examples/typescript`.
   - Archive obsolete or superseded examples into `examples/archive`.
   - Remove a legacy example only if it is explicitly confirmed to have no remaining reference value.

6. Remove stale active paths last.
   - Delete the root `src/` only after all active references have been removed or updated.
   - Remove any other confirmed stale active paths after the same check.

## Validation Commands

Run these representative checks after the related moves land:

```bash
cargo build --workspace
cargo test --workspace
cargo build --workspace --target wasm32-unknown-unknown
npm run build --workspace=@relateby/pattern
npm run test --workspace=@relateby/pattern
npm run build --workspace=@relateby/graph
npm run test --workspace=@relateby/graph
npm run build --workspace=@relateby/gram
npm run test --workspace=@relateby/gram
cd python/packages/relateby && pytest tests
./scripts/check-workflows.sh
./scripts/ci-local.sh
```

## Acceptance Review

Confirm the following before marking the feature ready for task breakdown:

- A reviewer can identify the peer implementation, adapter, example, support, and archive areas from the repository root.
- Active docs and active examples reference canonical current paths only.
- The three supported TypeScript package surfaces are visible in the tree and in active guidance.
- The Python distribution root is discoverable as a first-class implementation area.
- The WASM adapter remains discoverable without being presented as a peer Rust library.
- Archived material is still available when needed but is no longer treated as active guidance.
- The root `src/` has been removed only after active references were audited and cleared.
