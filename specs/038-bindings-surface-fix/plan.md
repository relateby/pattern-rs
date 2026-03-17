# Implementation Plan: TypeScript and Python Surface Improvements

**Branch**: `038-bindings-surface-fix` | **Date**: 2026-03-17 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/038-bindings-surface-fix/spec.md`

## Summary

Align the real public TypeScript and Python package surfaces with their documented developer experience by making the package-level facades authoritative, deriving public exports from the underlying generated/native bindings instead of shadowing them, and adding release-blocking verification for runtime exports, type or stub surfaces, wrappers, and docs/examples. The implementation will keep `@relateby/pattern`, `relateby.pattern`, and `relateby.gram` as the supported public boundaries while tightening wrappers, fixing missing or misleading exports, and validating the packaged artifacts instead of only crate-local modules.

## Technical Context

**Language/Version**: Rust 1.70.0 (workspace MSRV), Edition 2021; TypeScript 5.x package layer and generated declarations; Python 3.8+ unified packaging with PyO3  
**Primary Dependencies**: wasm-bindgen 0.2, js-sys 0.3, PyO3, wasm-pack, TypeScript, Vitest, maturin, pytest, existing workspace crates `relateby-pattern`, `relateby-gram`, and `pattern-wasm`  
**Storage**: N/A (in-memory library bindings and generated package artifacts)  
**Testing**: `cargo test --workspace`, TypeScript package tests via Vitest and `tsc`, Python tests via pytest, packed-artifact smoke tests for npm and Python wheel installs  
**Target Platform**: Native Rust, WASM (`wasm32-unknown-unknown`, bundler and Node.js package targets), and Python wheel consumers  
**Project Type**: Multi-target library workspace with TypeScript/WASM and Python packaging layers  
**Performance Goals**: N/A for throughput; developer-facing workflows must remain thin wrappers over existing behavior and keep setup and example execution straightforward  
**Constraints**: Preserve current public package boundaries (`@relateby/pattern`, `relateby.pattern`, `relateby.gram`); avoid introducing new crate dependency cycles; keep WASM-compatible public APIs; validate packaged artifacts rather than relying solely on source-tree tests  
**Scale/Scope**: TypeScript package facade and docs, generated declaration alignment, Python wrapper package and stubs/docs, release smoke checks, and public-workflow regression coverage across a small number of shared workflows

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Reference Implementation Fidelity | PASS | The feature does not change core pattern semantics; it aligns external language surfaces with existing behavior. Verification must ensure wrapper changes do not drift from the established Rust behavior that mirrors `../pattern-hs`. |
| II. Correctness & Compatibility | PASS | The plan preserves existing public package names and focuses on removing mismatches, missing exports, and misleading guidance rather than introducing new behavioral divergence. |
| III. Rust Native Idioms | PASS | Rust remains the source of truth for underlying behavior; wrapper changes stay in existing binding/package layers without imposing non-idiomatic patterns on core Rust APIs. |
| IV. Multi-Target Library Design | PASS | The plan explicitly covers native Rust-backed bindings for WASM and Python while preserving current multi-target constraints and package boundaries. |
| V. External Language Bindings & Examples | PASS | This feature directly improves external language bindings, examples, stubs, and quick-start guidance, and adds stronger verification that those artifacts stay in sync. |

**Gate result (pre-research)**: Pass.

**Gate result (post-design)**: Pass. Phase 1 design preserves current public package boundaries, keeps cross-crate dependency constraints intact, and adds verification and documentation alignment across TypeScript and Python without changing core library semantics.

## Project Structure

### Documentation (this feature)

```text
specs/038-bindings-surface-fix/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── typescript-api.md
│   ├── python-api.md
│   └── verification.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── pattern-core/
│   ├── src/
│   │   └── python.rs
│   ├── pattern_core/
│   │   └── __init__.pyi
│   └── tests/
│       └── python/
├── gram-codec/
│   └── src/
│       └── python.rs
└── pattern-wasm/
    └── src/
        ├── lib.rs
        ├── gram.rs
        └── standard_graph.rs

typescript/
└── @relateby/
    └── pattern/
        ├── package.json
        ├── src/
        │   ├── index.ts
        │   ├── gram.ts
        │   ├── wasm-types.d.ts
        │   └── graph/
        ├── tests/
        ├── wasm/
        └── wasm-node/

python/
└── relateby/
    ├── pyproject.toml
    ├── README.md
    ├── relateby/
    │   ├── __init__.py
    │   ├── gram/__init__.py
    │   ├── pattern/__init__.py
    │   └── _native/__init__.py
    └── relateby_build/

docs/
├── python-usage.md
├── wasm-usage.md
├── typescript-graph.md
└── release.md

scripts/
└── release/
    ├── npm-smoke/
    └── python-smoke.py
```

**Structure Decision**: Keep the existing workspace and package layout. Implement TypeScript fixes inside the `typescript/@relateby/pattern` facade, generated-binding alignment, docs, and package verification; implement Python fixes across the native bindings, the unified `python/relateby/relateby` wrapper, shipped stubs, docs, and wheel-based verification. No new top-level package or crate is needed.

## Complexity Tracking

No constitution violations require justification.
