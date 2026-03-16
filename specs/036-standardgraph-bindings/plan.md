# Implementation Plan: StandardGraph TypeScript/WASM and Python Bindings

**Branch**: `036-standardgraph-bindings` | **Date**: 2026-03-15 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/036-standardgraph-bindings/spec.md`

## Summary

Expose the Rust `StandardGraph` type through the existing WASM/TypeScript and Python binding layers. StandardGraph is monomorphic (`PatternGraph<(), Subject>` with fixed classifier), making it the most binding-friendly type in the crate. All boundary types (`Subject`, `Pattern<Subject>`, `Symbol`, `Value`) already have wrappers in both targets. This is incremental wrapper work following established patterns.

## Technical Context

**Language/Version**: Rust 1.70.0 (MSRV), Edition 2021; TypeScript (type definitions); Python 3.8+ (PyO3)
**Primary Dependencies**: wasm-bindgen 0.2, js-sys 0.3, PyO3 (existing); pattern-core, gram-codec (workspace crates)
**Storage**: N/A (in-memory graph structures)
**Testing**: cargo test (Rust), Node.js integration tests (WASM), pytest (Python)
**Target Platform**: WASM (wasm32-unknown-unknown), native Rust, Python (via maturin/PyO3)
**Project Type**: Library (multi-target)
**Performance Goals**: N/A — thin wrapper layer; performance governed by underlying Rust implementation
**Constraints**: wasm-bindgen does not support consuming `self` in chained methods; SubjectBuilder must use `&mut self` internally
**Scale/Scope**: ~400-500 lines WASM bindings, ~300-350 lines Python bindings, ~150 lines TypeScript definitions

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Reference Implementation Fidelity | PASS | StandardGraph is a Rust-native convenience type (no Haskell equivalent). Bindings faithfully expose the Rust API. |
| II. Correctness & Compatibility | PASS | Bindings delegate to existing, tested Rust implementation. No behavioral changes. |
| III. Rust Native Idioms | PASS | Follows existing binding patterns (WasmPatternGraph, PySubject). snake_case for Python, camelCase for JS. |
| IV. Multi-Target Library Design | PASS | Feature-gated behind `wasm` and `python` features. No platform-specific code paths. |
| V. External Language Bindings & Examples | PASS | This feature directly advances this principle — adding examples for both targets. |

No violations. No complexity tracking needed.

## Project Structure

### Documentation (this feature)

```text
specs/036-standardgraph-bindings/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   ├── wasm-api.md      # WASM/TypeScript API contract
│   └── python-api.md    # Python API contract
└── tasks.md             # Phase 2 output (via /speckit.tasks)
```

### Source Code (repository root)

```text
crates/
├── pattern-core/
│   ├── src/
│   │   ├── wasm.rs                          # + WasmStandardGraph, WasmSubjectBuilder (~400-500 lines)
│   │   └── python.rs                        # + PyStandardGraph, PySubjectBuilder (~300-350 lines)
│   ├── typescript/
│   │   └── pattern_core.d.ts                # + StandardGraph, SubjectBuilder type defs (~150 lines)
│   ├── pattern_core/
│   │   └── __init__.pyi                     # + StandardGraph, SubjectBuilder type stubs (~100 lines)
│   └── tests/
│       └── python/
│           └── test_standard_graph.py       # Python integration tests (~200 lines)
├── pattern-wasm/
│   └── src/
│       └── lib.rs                           # + re-exports, WasmStandardGraph::fromGram (needs gram-codec)
└── gram-codec/                              # No changes needed

python/
└── relateby/
    └── pattern/
        └── __init__.py                      # + StandardGraph.from_gram (bridges pattern-core + gram-codec)

examples/
├── pattern-core-wasm/
│   ├── standard_graph.mjs                   # WASM/Node.js example (~50 lines)
│   └── test_standard_graph.mjs              # WASM/Node.js integration tests (~150 lines)
└── pattern-core-python/
    └── standard_graph.py                    # Python example (~50 lines)
```

**Structure Decision**: Most new code lives in existing files (`wasm.rs`, `python.rs`, `pattern_core.d.ts`, `__init__.pyi`). Key exception: `fromGram`/`from_gram` must be implemented outside `pattern-core` because `pattern-core` cannot depend on `gram-codec` (circular dependency — gram-codec depends on pattern-core for types). WASM `fromGram` goes in `pattern-wasm/src/lib.rs` (which already depends on both crates). Python `from_gram` goes in the unified `python/relateby/pattern/` package layer.
