# Implementation Plan: TypeScript and Python Pattern API Parity

**Branch**: `047-ts-py-parity` | **Date**: 2026-04-16 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/047-ts-py-parity/spec.md`

---

## Summary

Add 12 missing Pattern operations to the TypeScript (`@relateby/pattern`) and Python (`relateby.pattern`) libraries, matching the Haskell reference implementation at `../pattern-hs/libs/pattern/src/Pattern/Core.hs`. Additionally add 6 graph transform functions to Python that TypeScript already provides. All operations are pure in-memory, no new dependencies required.

The Haskell reference defines the authoritative semantics; the Rust implementation (`pattern-core`) serves as the verified intermediate reference. All new operations are already present and tested in Rust.

---

## Technical Context

**Language/Version**:
- TypeScript 5.x (existing `@relateby/pattern` package)
- Python 3.8+ (existing `relateby.pattern` pure-Python package)

**Primary Dependencies**:
- TypeScript: `effect` (Data.Class structural equality, pipe composition) — no new deps
- Python: Python stdlib only (`dataclasses`, `typing`) — no new deps

**Storage**: N/A (in-memory only)

**Testing**:
- TypeScript: `vitest` (existing test runner)
- Python: `pytest` (existing test runner)

**Target Platform**:
- TypeScript: Node.js + browser (pure JS, no WASM for Pattern ops)
- Python: CPython 3.8+ (pure Python)

**Project Type**: Library (multi-language bindings over a Haskell reference)

**Performance Goals**: Consistent with existing recursive operations; correctness over optimization.

**Constraints**:
- No new external dependencies in either TypeScript or Python
- TypeScript ops must be composable via Effect `pipe()`
- Python must maintain 3.8 compatibility (`from __future__ import annotations`)
- No breaking changes to existing API surface

**Scale/Scope**: Adding 12 core Pattern ops + 6 graph transform ops across 2 language packages.

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Principle | Status | Notes |
|---|---|---|
| I. Reference Fidelity | ✅ PASS | All 12 ops verified against Haskell source in `../pattern-hs/libs/pattern/src/Pattern/Core.hs`. Semantics documented in `research.md`. |
| II. Correctness & Compatibility | ✅ PASS | Semantics are derived from authoritative Haskell source and cross-checked against Rust implementation. Behavioral equivalence tests required. |
| III. Rust Native Idioms | N/A | This feature adds TypeScript and Python, not Rust. The Rust implementation already has all these operations. |
| IV. Multi-Target Library Design | ✅ PASS | All new ops are pure in-memory; no I/O or WASM constraints. TypeScript ops are pure JS with no WASM dependency. |
| V. External Language Bindings | ✅ PASS | This feature IS the language binding work. Examples are included in `quickstart.md`. |

**Post-design re-check**: No design decisions introduced violations. The functional approach to `combine` (passing combiner function rather than requiring a `Combinable` protocol) is an intentional deviation from the Haskell Semigroup constraint, justified because TypeScript and Python lack type classes. This must be documented in the implementation.

---

## Project Structure

### Documentation (this feature)

```text
specs/047-ts-py-parity/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0: API decisions and Haskell semantics
├── data-model.md        # Phase 1: entity and operation contracts
├── quickstart.md        # Phase 1: usage examples
├── contracts/
│   ├── typescript-api.md    # TypeScript public API signatures
│   └── python-api.md        # Python public API signatures
└── checklists/
    └── requirements.md
```

### Source Code (repository paths)

```text
# TypeScript
typescript/packages/pattern/src/
├── pattern.ts           # MODIFY: add Pattern.pattern(), Pattern.fromList() static constructors
├── ops.ts               # MODIFY: add anyValue, allValues, matches, contains, para, unfold,
│                        #         combine, depthAt, sizeAt, indicesAt
└── index.ts             # MODIFY: export new ops

typescript/packages/pattern/src/__tests__/ (or tests/)
└── ops.test.ts          # MODIFY/CREATE: tests for new operations

# Python
python/packages/relateby/relateby/pattern/
├── _pattern.py          # MODIFY: add any_value, all_values, matches, contains, para,
│                        #         Pattern.pattern(), Pattern.from_list(), Pattern.unfold(),
│                        #         combine, depth_at, size_at, indices_at
├── _graph_transforms.py # CREATE: map_graph, map_all_graph, filter_graph, fold_graph,
│                        #         map_with_context, para_graph
└── __init__.py          # MODIFY: export new methods and graph transforms

python/packages/relateby/tests/
├── test_pattern_parity.py    # CREATE: tests for new Pattern operations
└── test_graph_transforms.py  # CREATE: tests for Python graph transforms
```

**Structure Decision**: Follows existing conventions. TypeScript ops go in `ops.ts` (standalone curried functions) and constructors go on the `Pattern` class in `pattern.ts`. Python operations go as class methods on `Pattern` in `_pattern.py`; graph transforms go in a new standalone module `_graph_transforms.py` to maintain separation of concerns.

---

## Implementation Order

Operations should be implemented in dependency order within each language:

### TypeScript implementation order

1. `Pattern.pattern()` and `Pattern.fromList()` — foundation for tests
2. `anyValue`, `allValues` — simple predicates, test immediately
3. `matches`, `contains` — use Effect `Equal.equals`
4. `para` — required before `depthAt`/`sizeAt` (those can use `extend` instead, but `para` is needed independently)
5. `unfold` — independent
6. `depthAt`, `sizeAt`, `indicesAt` — use `extend` under the hood
7. `combine` — independent
8. Export all from `index.ts`
9. Tests for each operation

### Python implementation order

1. `Pattern.pattern()`, `Pattern.from_list()`, `Pattern.unfold()` classmethods
2. `any_value`, `all_values`
3. `matches`, `contains`
4. `para`
5. `depth_at`, `size_at`, `indices_at`
6. `combine`
7. Update `__init__.py` exports
8. Tests for new Pattern operations
9. `_graph_transforms.py` with all 6 transform functions
10. Update `__init__.py` to export transforms
11. Tests for graph transforms

---

## Key Implementation Notes

### `matches` in TypeScript

`Pattern` extends `Data.Class` from Effect, giving structural equality via `Equal.equals(a, b)`. This directly implements `matches`.

```typescript
export const matches = <V>(a: Pattern<V>, b: Pattern<V>): boolean => Equal.equals(a, b)
```

### `indicesAt` cannot use `extend`

Unlike `depthAt` and `sizeAt`, `indicesAt` requires knowing the position of each node within its parent's element list. The `extend` comonad operation passes the subtree but not the parent context. A separate recursive helper is required:

```typescript
export const indicesAt = <V>(p: Pattern<V>): Pattern<number[]> => {
  const go = (indices: number[]) => (sub: Pattern<V>): Pattern<number[]> =>
    new Pattern({
      value: indices,
      elements: Data.array(sub.elements.map((e, i) => go([...indices, i])(e)))
    })
  return go([])(p)
}
```

### `combine` API design

The Haskell `Semigroup` instance implicitly selects the combination operation via type. Since TypeScript and Python lack type classes, `combine` explicitly accepts a combiner function as its first argument. This is an intentional idiomatic deviation:

- **Haskell**: `p1 <> p2` (Semigroup constraint on `v`)
- **TypeScript**: `combine(combineValues)(p1)(p2)`
- **Python**: `p1.combine(p2, combine_values)`

### Python `para_graph` topological ordering

`para_graph` must process elements in dependency order (sources before targets in directed graphs). The implementation should use the same topological sort that `StandardGraph` already provides, calling it once and iterating bottom-up.

### Python graph transforms: `GraphQuery` snapshot

`map_with_context` and `para_graph` both require a `GraphQuery` snapshot. A minimal Python `GraphQuery` adapter (simple dict-based lookup) should be built from the `StandardGraph` state at the start of the transform, then frozen for the duration of the transform call.

---

## Testing Strategy

### Haskell equivalence tests

For each new operation, include at least one test case that verifies the result matches the expected output from the Haskell reference. The Haskell test file is at `../pattern-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs`.

### Algebraic law tests

- `anyValue (const True)` always returns `true`
- `allValues (const True)` always returns `true`
- `allValues (const False)` on non-empty → `false`
- `matches p p` always `true`
- `contains p p` always `true` (reflexivity)
- `combine f (p1 <> p2) p3 == combine f p1 (combine f p2 p3)` (associativity with string concatenation)
- `para` on leaf: fold fn receives pattern and empty child results
- `depthAt (point v) == point 0` (leaf has depth 0)
- `sizeAt (point v) == point 1` (leaf has size 1)
- `indicesAt (point v) == point []` (root has empty index path)

### Cross-language equivalence

Where the Rust implementation has the same operation (all core ops), create cross-language equivalence tests that verify TypeScript and Python produce the same output as Rust for the same input.

---

## Complexity Tracking

No constitution violations. No complexity justification required.
