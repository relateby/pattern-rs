# Proposal: Restructure the Monorepo Around Peer Implementations

**Date**: 2026-03-18
**Status**: Draft
**Scope**: Repository layout, package boundaries, examples, documentation, and legacy cleanup

---

## Summary

This repository has evolved from a Rust-first workspace with language bindings into a multi-language monorepo containing peer implementations and thin interoperability layers. The current layout still reflects the earlier architecture, which makes the repository harder to navigate and increases the chance of stale directories, outdated examples, and misleading documentation.

This proposal recommends restructuring the repository around three explicit concepts:

1. **Peer implementations** that are first-class developer surfaces in Rust, TypeScript, and Python
2. **Adapters** that exist only to bridge implementations, such as the WASM codec boundary
3. **Legacy or historical material** that should be archived, renamed, or removed once superseded

The goal is to make the repository layout match the actual architecture, reduce ambiguity about which packages are public vs internal, and remove leftover structure from the old single-crate era.

---

## Problems with the Current Layout

### 1. Root-level structure still implies a Rust-only project

The repository root still presents itself primarily as a Cargo workspace, even though the repo now contains first-class Python and TypeScript implementations. This creates a mismatch between the actual architecture and the directory layout contributors see first.

### 2. Some paths are clearly stale

There are directories and files that no longer correspond to active package roots or supported entry points:

- `src/` at the repo root appears to be an orphan from an earlier root-crate layout
- `examples/wasm-js/` still assumes the repo root is a Rust package
- some example and support docs still describe old crate-era boundaries

These stale paths make the repository harder to trust and harder to onboard into.

### 3. Public and internal TypeScript packages look equally first-class

The TypeScript workspace currently contains:

- `typescript/@relateby/pattern`
- `typescript/@relateby/graph`
- `typescript/@relateby/gram`

But the release and public API docs treat `@relateby/pattern` as the supported npm surface. The current layout makes it unclear whether the others are intended public packages, private implementation modules, or future publishable artifacts.

### 4. Adapter code is mixed with peer implementation crates

`crates/pattern-wasm/` is important, but it is conceptually different from `relateby-pattern` and `relateby-gram`. It is not a peer domain library; it is an adapter layer. Keeping it alongside the main Rust crates makes the Rust crate set look more symmetrical than it actually is.

### 5. Examples are organized around historical package boundaries

The `examples/` tree mixes:

- active Rust examples
- active Python examples
- active TypeScript examples
- WASM bridge examples
- clearly legacy examples

Several folder names still reflect older crate names or earlier packaging assumptions. The result is more of an archaeological record than a clean set of examples for current users.

### 6. Historical notes are still mixed with living documentation

Some tracked markdown files read more like migration notes, branch summaries, or temporary analyses than active repository guidance. They add noise during navigation and make it harder to tell which docs are normative.

---

## Design Principles

### 1. Make architecture visible in the directory tree

The top-level layout should make it obvious what is:

- a language implementation
- a bridge or adapter
- a runnable example
- repo support material
- historical/archive content

### 2. Separate public package boundaries from internal modules

Paths should communicate whether a package is part of the supported public surface or an internal implementation detail.

### 3. Minimize churn in the first pass

The first restructuring pass should avoid unnecessary disruption to Cargo, CI, releases, and packaging workflows. The goal is to improve clarity without forcing a complete re-foundation of every toolchain at once.

### 4. Remove obviously stale structure early

Confirmed orphaned or structurally invalid paths should not be preserved just because they are old. A repository becomes easier to maintain when dead layout is removed promptly.

### 5. Keep archival history available, but out of the way

If historical notes are still worth keeping, they should move to an archive-oriented area rather than living beside active docs.

---

## Proposed Target Layout

This proposal recommends a medium-churn target layout like this:

```text
pattern-rs/
├── Cargo.toml
├── crates/                     # Rust publishable crates
│   ├── pattern-core/
│   └── gram-codec/
├── adapters/                   # Cross-language bridge layers
│   └── wasm/
│       └── pattern-wasm/
├── python/
│   └── packages/
│       └── relateby/
├── typescript/
│   ├── package.json
│   └── packages/
│       ├── pattern/            # supported public package
│       └── internal/
│           ├── graph/
│           └── gram/
├── examples/
│   ├── rust/
│   ├── python/
│   ├── typescript/
│   └── archive/
├── docs/
├── proposals/
├── scripts/
├── specs/
└── external/
```

This keeps the repository root stable while making package roles much clearer.

---

## Concrete Recommendations

### A. Remove stale top-level code layout

- Remove the root `src/` directory after confirming it is unused
- Treat any remaining root-crate assumptions as migration leftovers to eliminate

### B. Separate adapters from peer Rust libraries

- Move `crates/pattern-wasm/` into an adapter-focused area such as `adapters/wasm/pattern-wasm/`
- Keep `crates/` reserved for peer Rust libraries that are part of the core Rust product surface

### C. Make the TypeScript workspace reflect package intent

- Keep the TypeScript workspace root at `typescript/`
- Move workspace packages under `typescript/packages/`
- Put supported public packages in a clearly public area
- Put internal or non-published packages in `typescript/packages/internal/`

If `@relateby/graph` and `@relateby/gram` are meant to stay internal, their paths should communicate that directly. If they are intended to become public packages later, that decision should be made explicitly and documented.

### D. Give Python the same structural clarity as TypeScript

- Keep the published package under a package-oriented subtree such as `python/packages/relateby/`
- Treat Python as a first-class implementation area, not as an add-on beside Rust crates

### E. Reorganize examples by language or public surface

- Move active Rust examples under `examples/rust/`
- Move active Python examples under `examples/python/`
- Move active TypeScript examples under `examples/typescript/`
- Move obsolete examples into `examples/archive/` or remove them if they are no longer useful

In particular:

- remove or archive `examples/wasm-js/`
- rename crate-era example folders whose names no longer match supported package boundaries

### F. Clean up living docs vs historical notes

- Keep active user and contributor docs under `docs/`
- Move historical notes, review memos, and migration summaries into an archive area if they must be retained
- Rewrite root-facing docs to describe the monorepo as multi-language, not Rust-only

---

## Paths Likely to Be Cleaned Up First

The following are strong early candidates because they appear stale or misleading:

- `src/`
- `examples/wasm-js/`
- crate-era example naming in `examples/`
- historical review/status markdown files that are no longer normative

These changes would improve clarity immediately with relatively low risk.

---

## Phased Migration Plan

### Phase 1: Classification and cleanup

Classify each top-level subtree as one of:

- public implementation
- internal adapter
- example
- repo support
- archive/legacy

Then remove or archive obvious dead structure:

- root `src/`
- `examples/wasm-js/`
- other confirmed stale directories and docs

### Phase 2: Adapter separation

- Move `pattern-wasm` out of `crates/` into an adapter-specific location
- Update build, CI, and release references accordingly

This phase makes the Rust side easier to read without changing the public Rust crate model.

### Phase 3: TypeScript workspace normalization

- Introduce `typescript/packages/`
- Move public and internal packages into explicit subtrees
- Update root workspace config, scripts, and docs

### Phase 4: Python workspace normalization

- Move `python/relateby/` into a package-oriented subtree
- Update docs and release scripts to match

### Phase 5: Example consolidation

- Reorganize examples around current supported public surfaces
- Archive or remove legacy examples
- Update example READMEs and links

### Phase 6: Documentation rewrite

- Update `README.md`
- Update `examples/README.md`
- Update language-specific usage docs
- Clarify which packages are public, which are internal, and which are adapters

---

## Expected Benefits

This restructuring should produce several benefits:

- Faster onboarding for contributors
- Clearer mental model of the monorepo
- Less confusion about public vs internal package boundaries
- Fewer stale paths and legacy assumptions in examples
- Better alignment between architecture, packaging, and documentation
- Easier future decisions about publishing additional language packages

---

## Risks and Trade-offs

### 1. Tooling churn

Moving package roots will require updates to:

- workspace configuration
- build scripts
- CI
- release scripts
- documentation

This is manageable, but should be phased rather than done opportunistically.

### 2. Path instability during migration

Contributors may have local scripts or habits tied to current paths. The migration should therefore be documented clearly and grouped into coherent commits.

### 3. Over-correcting too early

A fully symmetrical `implementations/<lang>/...` layout may be attractive, but it would create more churn than necessary right now. This proposal intentionally recommends a lower-churn first target that improves clarity without rebuilding every workflow.

---

## Open Questions

The following should be decided before implementation begins:

1. Are `@relateby/graph` and `@relateby/gram` intended to remain internal, or become public packages later?
2. Should legacy examples be archived in-repo, or removed once superseded?
3. Should `pattern-wasm` remain discoverable as a contributor-facing adapter package, or be de-emphasized as an implementation detail?
4. Is a medium-churn layout sufficient, or is there appetite for a later second-phase move to a fully symmetric multi-language `implementations/` structure?

---

## Recommendation

Adopt the restructuring in phases, starting with the lowest-risk cleanup:

1. remove obvious orphaned/stale paths
2. separate adapter code from peer implementation packages
3. make TypeScript and Python package roots reflect their actual roles
4. rewrite examples and docs around the current public surfaces

This yields a repository that better reflects the architecture it now has: a multi-language monorepo of peer implementations, with explicit bridge layers and much less leftover structure from the original Rust-only era.
