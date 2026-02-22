# Implementation Plan: Graph Classifier Port

**Branch**: `030-graph-classifier` | **Date**: 2026-02-22 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/030-graph-classifier/spec.md`

## Summary

Port `Pattern.Graph.GraphClassifier` and `Pattern.PatternGraph` from the Haskell reference implementation (`../pattern-hs`) to idiomatic Rust in `pattern-core`. The port adds a shape-based pattern classifier, an injectable classification abstraction, and an eagerly materialized graph container that routes `Pattern<V>` values into six typed collections. A prerequisite sub-task is porting `Pattern.Reconcile`, which does not yet exist in pattern-rs and is required by `PatternGraph`'s merge-on-insert semantics.

## Technical Context

**Language/Version**: Rust 1.70.0 (MSRV), Edition 2021
**Primary Dependencies**: std (HashMap, Vec, HashSet) — no new external crates required
**Storage**: N/A (in-memory data structures only)
**Testing**: `cargo test` (unit + integration), `proptest` (property-based)
**Target Platform**: Native Rust (x86_64, ARM), WASM (wasm32-unknown-unknown), Python via PyO3
**Project Type**: Library (pattern-core crate)
**Performance Goals**: Correctness is primary; no specific performance targets for this feature
**Constraints**: WASM-compatible (no blocking I/O, no filesystem); no new `unsafe` code
**Scale/Scope**: Adds 3 new modules (~400 lines of impl), ~200 lines of tests; modifies 2 existing files (subject.rs, lib.rs)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Reference Implementation Fidelity | ✅ PASS | Direct port of `GraphClassifier.hs` + `PatternGraph.hs`; all Haskell tests ported; behavioral equivalence is the primary correctness measure |
| II. Correctness & Compatibility | ✅ PASS | API matches reference semantics; no existing APIs changed; `Symbol` Ord addition is additive only |
| III. Rust Native Idioms | ✅ PASS | `Box<dyn Fn>` for classifier; `HashMap` instead of `Map`; traits instead of typeclasses; no literal Haskell syntax |
| IV. Multi-Target Library Design | ✅ PASS | All new code is pure std; no platform-specific paths; WASM build verified during validation |
| V. External Language Bindings | ⚠️ DEFERRED | Python and WASM bindings for new types not in this feature scope; documented as follow-up. Existing Python bindings unaffected. |

**Complexity Tracking**: No violations. The `Principle V` deferral is intentional and documented — the core types are internal library additions; bindings are follow-up work.

## Project Structure

### Documentation (this feature)

```text
specs/030-graph-classifier/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/
│   └── public-api.md    # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/pattern-core/src/
├── lib.rs                     (MODIFIED: add module declarations + re-exports)
├── subject.rs                 (MODIFIED: add Ord + PartialOrd to Symbol)
├── reconcile.rs               (NEW: port Pattern.Reconcile)
├── graph/
│   ├── mod.rs                 (NEW: re-exports from graph_classifier)
│   └── graph_classifier.rs    (NEW: GraphClass, GraphClassifier, GraphValue,
│                                    classify_by_shape, canonical_classifier, from_test_node)
└── pattern_graph.rs           (NEW: PatternGraph, merge, from_patterns, etc.)

crates/pattern-core/tests/
├── graph_classifier.rs        (NEW: ported from GraphClassifierSpec.hs)
└── pattern_graph.rs           (NEW: ported from PatternGraphSpec.hs)
```

**Structure Decision**: Single project (Option 1). All new code lives within the `pattern-core` crate. The `graph/` subdirectory mirrors the Haskell module hierarchy (`Pattern.Graph.GraphClassifier`). The `reconcile.rs` module is a peer of `pattern.rs` and `subject.rs` at the `src/` root. `pattern_graph.rs` imports from both `graph::graph_classifier` and `reconcile`, matching the Haskell module dependency graph.

## Implementation Sequence

### Phase A: Prerequisites

1. **Port `Pattern.Reconcile`** → `src/reconcile.rs`
   - Traits: `HasIdentity`, `Mergeable`, `Refinable`
   - Types: `ReconciliationPolicy<S>`, `ElementMergeStrategy`, `SubjectMergeStrategy`, `LabelMerge`, `PropertyMerge`
   - Instances: all three traits for `Subject`
   - Function: `reconcile()` returning `Result<Pattern<V>, ReconcileError>`
   - Reference: `../pattern-hs/libs/pattern/src/Pattern/Reconcile.hs`

2. **Add `Ord` to `Symbol`** → `src/subject.rs`
   - Add `#[derive(PartialOrd, Ord)]` to `Symbol`
   - Run `cargo test` to confirm no regressions

### Phase B: Classifier Module

3. **Create `src/graph/mod.rs`** — module stub, re-exports from `graph_classifier`
4. **Create `src/graph/graph_classifier.rs`**
   - `GraphClass<Extra>` enum + `map_other` impl
   - `GraphValue` trait + `Subject` impl
   - `GraphClassifier<Extra, V>` struct + `new()` constructor
   - Private: `is_node_like`, `is_relationship_like`, `is_valid_walk`
   - Public: `classify_by_shape`, `canonical_classifier`, `from_test_node`

### Phase C: Container Module

5. **Create `src/pattern_graph.rs`**
   - `PatternGraph<Extra, V>` struct
   - `empty()` constructor
   - Private insert functions: `insert_node`, `insert_relationship`, `insert_walk`, `insert_annotation`, `insert_other`
   - Public: `merge`, `merge_with_policy`, `from_patterns`, `from_patterns_with_policy`

### Phase D: Tests and Wiring

6. **Port tests** → `tests/graph_classifier.rs`, `tests/pattern_graph.rs`
7. **Update `lib.rs`** — add module declarations and re-exports
8. **Run full CI** — format, clippy, tests, WASM build

## Key Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| `ReconciliationPolicy` source | Port from Haskell `Pattern.Reconcile` | Required by `PatternGraph`; doesn't exist in pattern-rs |
| `Symbol` ordering | Add `Ord` derive | `GraphValue::Id` requires `Ord`; String has natural lexicographic order |
| `GraphValue::Id` for Subject | `Symbol` (not `String`) | Matches Haskell type; `Symbol` is already the canonical identity type |
| Classifier closure lifetime | `'static` | Enables storage in structs and `Send` contexts |
| `pg_other` storage | `(Extra, Pattern<V>)` tuple | Preserves classifier tag alongside pattern per Haskell spec |
| Walk decomposition | Recursive (walk → rels → nodes) | Matches Haskell `insertWalk` behavior; validated by test |
| Python/WASM bindings | Deferred to follow-up | Out of scope; new types are internal library additions |
| `from_test_node` | Implement as `pub fn` | Needed for GraphLens bridge; not in Haskell exports but in porting guide |

## Reference Files

| Haskell Source | Rust Target |
|---------------|-------------|
| `../pattern-hs/libs/pattern/src/Pattern/Graph/GraphClassifier.hs` | `src/graph/graph_classifier.rs` |
| `../pattern-hs/libs/pattern/src/Pattern/PatternGraph.hs` | `src/pattern_graph.rs` |
| `../pattern-hs/libs/pattern/src/Pattern/Reconcile.hs` | `src/reconcile.rs` |
| `../pattern-hs/libs/pattern/tests/Spec/Pattern/Graph/GraphClassifierSpec.hs` | `tests/graph_classifier.rs` |
| `../pattern-hs/libs/pattern/tests/Spec/Pattern/PatternGraphSpec.hs` | `tests/pattern_graph.rs` |
