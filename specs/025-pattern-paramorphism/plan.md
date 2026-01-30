# Implementation Plan: Pattern Paramorphism

**Branch**: `025-pattern-paramorphism` | **Date**: 2026-01-30 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/025-pattern-paramorphism/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Add a structure-aware folding operation `para` to `Pattern<V>` that gives the folding function access to both the current pattern and recursively computed results from its elements. This enables pattern-of-elements analysis (e.g., detecting A, B, A sequences), depth-weighted aggregations, and element-count-aware computations. The implementation ports the gram-hs `para` from `../gram-hs/libs/pattern/src/Pattern/Core.hs` (lines 1188–1190) with behavioral equivalence verified by property and unit tests.

## Technical Context

**Language/Version**: Rust (existing workspace; use same toolchain as pattern-core)  
**Primary Dependencies**: None new (pattern-core crate existing)  
**Storage**: N/A (in-memory pattern structure only)  
**Testing**: cargo test (unit + property-based; port tests from gram-hs Pattern/Properties.hs and CoreSpec.hs paramorphism sections)  
**Target Platform**: Native Rust, WASM (pattern-core already multi-target)  
**Project Type**: Library (multi-crate workspace; changes in crates/pattern-core only)  
**Performance Goals**: O(n) time where n = total nodes; O(d) stack where d = max depth (same as existing fold)  
**Constraints**: WASM-compatible (no blocking I/O); behavioral equivalence with gram-hs para  
**Scale/Scope**: Same as Pattern (100+ nesting levels, 10k+ elements per existing docs)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|--------|
| **I. Reference Implementation Fidelity** | PASS | Para will be ported from `../gram-hs/libs/pattern/src/Pattern/Core.hs` (para at lines 1188–1190). Equivalence tests will be ported from `../gram-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs` (paramorphism describe block) and `../gram-hs/libs/pattern/tests/Spec/Pattern/Properties.hs` (T025–T030). |
| **II. Correctness & Compatibility** | PASS | API and semantics will match gram-hs para; no breaking changes to existing Pattern API. |
| **III. Rust Native Idioms** | PASS | Method on `Pattern<V>`, `Fn(&Pattern<V>, &[R]) -> R` for the folding function; references to avoid unnecessary cloning. |
| **IV. Multi-Target Library Design** | PASS | No new platform-specific code; para is pure and fits existing pattern-core design. |
| **V. External Language Bindings & Examples** | PASS | Doc examples in quickstart and inline; Python bindings can expose para in a follow-up if needed (out of scope for this feature). |

**Note**: When porting features from gram-hs, reference the local implementation at `../gram-hs` and corresponding feature specifications in `../gram-hs/specs/`. See [porting guide](../../docs/porting-guide.md) for detailed porting instructions.

## Project Structure

### Documentation (this feature)

```text
specs/025-pattern-paramorphism/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/pattern-core/
├── src/
│   ├── lib.rs           # Re-export Pattern (no change)
│   ├── pattern.rs       # Add para() method to impl Pattern<V>
│   ├── pattern/
│   │   ├── comonad.rs
│   │   └── comonad_helpers.rs
│   ├── subject.rs
│   ├── python.rs        # Optional: expose para to Python in future
│   └── test_utils/
│       ├── mod.rs
│       ├── equivalence.rs
│       ├── generators.rs
│       └── helpers.rs
└── tests/               # Integration tests if any (para covered in unit/property tests in pattern.rs or dedicated module)
```

**Structure Decision**: Single crate change within the existing gram-rs workspace. All implementation lives in `crates/pattern-core/src/pattern.rs` (new `para` method and any private helpers). Tests can live in the same file (`#[cfg(test)] mod tests`) or in `tests/` for integration-style tests; property tests will use existing test_utils generators.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations. Table left empty.
