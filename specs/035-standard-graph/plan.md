# Implementation Plan: StandardGraph

**Branch**: `035-standard-graph` | **Date**: 2026-03-15 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/035-standard-graph/spec.md`

## Summary

Implement `StandardGraph`, a concrete ergonomic wrapper around `PatternGraph<(), Subject>` that provides zero-configuration graph construction and querying. Also implement `SubjectBuilder` for fluent `Subject` value construction. StandardGraph composes on existing `PatternGraph`, `GraphClassifier`, `GraphQuery`, and reconciliation infrastructure — no reimplementation of core logic.

## Technical Context

**Language/Version**: Rust 1.70.0 (workspace MSRV), Edition 2021
**Primary Dependencies**: pattern-core (PatternGraph, GraphClassifier, GraphQuery, GraphValue, Subject, Symbol, reconcile), gram-codec (parse_gram) — no new external crates
**Storage**: N/A (in-memory data structures only)
**Testing**: cargo test (unit + integration), proptest (property-based)
**Target Platform**: Native Rust + wasm32-unknown-unknown (WASM)
**Project Type**: Library (Rust crate within workspace)
**Performance Goals**: Correctness first; no specific performance targets for this feature
**Constraints**: WASM-compatible (no blocking I/O, no file system access), no new external crates
**Scale/Scope**: In-memory, bounded by available memory; test correctness verified at 1,000 nodes / 5,000 relationships

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Reference Implementation Fidelity | PASS (N/A) | StandardGraph has no gram-hs equivalent. It is a Rust-native ergonomic layer wrapping faithfully-ported types (PatternGraph, GraphClassifier, reconciliation). No behavioral deviation from reference. |
| II. Correctness & Compatibility | PASS | Composes on existing correct implementations. LastWriteWins reconciliation, canonical classifier, and Pattern structures are already verified against gram-hs. |
| III. Rust Native Idioms | PASS | Builder pattern (SubjectBuilder), direct methods on concrete type, idiomatic Result/Option returns, snake_case naming. |
| IV. Multi-Target Library Design | PASS | In-memory only, no I/O, no platform-specific code. WASM-compatible by construction. |
| V. External Language Bindings | DEFERRED | Python/WASM bindings for StandardGraph are explicitly out of scope per proposal. Will be addressed in a future feature. |

**Gate result: PASS** — no violations require justification.

## Project Structure

### Documentation (this feature)

```text
specs/035-standard-graph/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
crates/pattern-core/
├── src/
│   ├── lib.rs                    # Add re-exports for StandardGraph, SubjectBuilder
│   ├── subject.rs                # Add SubjectBuilder impl
│   └── graph/
│       ├── mod.rs                # Add standard module, re-export StandardGraph
│       └── standard.rs           # NEW: StandardGraph implementation
└── tests/
    └── standard_graph_tests.rs   # NEW: Integration tests for StandardGraph
```

**Structure Decision**: StandardGraph lives in the existing `graph/` module as `standard.rs`, consistent with how `graph_classifier.rs`, `graph_query.rs`, and `graph_view.rs` are organized. SubjectBuilder lives in `subject.rs` alongside the Subject type it builds (no separate file needed). Tests go in the existing `tests/` directory.
