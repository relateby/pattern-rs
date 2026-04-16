# Language Parity Proposal: Pattern API Across Rust, TypeScript, and Python

**Date:** 2026-04-16
**Scope:** Core `Pattern<V>` and `Subject` data structure operations only (gram parsing/serialization excluded)
**Reference:** Haskell implementation in `../pattern-hs/libs/`

---

## Executive Summary

The Rust implementation is the most complete port of the Haskell reference, covering all core Pattern operations plus several Rust-idiomatic additions. TypeScript and Python have parity with each other but are missing a meaningful slice of the Pattern API — primarily structural predicates, the paramorphism (`para`), the unfold anamorphism, comonad helper operations, and the Semigroup/combine algebra. No implementation yet covers the Haskell `Applicative` instance, `paraWithScope`, or the `ScopeQuery`/`PatternKind`/`RepresentationMap` abstractions.

---

## Reference API Surface (Haskell)

The Haskell reference defines the following categories of Pattern operations (see `../pattern-hs/libs/pattern/src/Pattern/`):

| Category | Operations |
|---|---|
| Constructors | `point`, `pattern`, `fromList`, `unfold` |
| Structure | `length`, `size`, `depth`, `values`, `toTuple`, `flatten` |
| Predicates | `anyValue`, `allValues`, `matches`, `contains` |
| Queries | `filterPatterns`, `findPattern`, `findAllPatterns` |
| Functor | `fmap` |
| Foldable | `foldMap`, `foldr`, `foldl` |
| Paramorphism | `para`, `paraWithScope` |
| Traversable | `traverse`, `sequenceA` |
| Applicative | `pure`, `<*>` |
| Comonad | `extract`, `extend`, `duplicate` |
| Comonad helpers | `depthAt`, `sizeAt`, `indicesAt` |
| Semigroup | `(<>)` (combines values, concatenates elements) |
| Monoid | `mempty` |
| Anamorphism | `unfold` |

Subject additionally has: `addProperty`, `updateProperty`, `removeProperty`, `hasProperty`, Semigroup/Monoid instances.

---

## Parity Matrix

### Pattern Constructors and Queries

| Operation | Haskell | Rust | TypeScript | Python |
|---|---|---|---|---|
| `point(v)` | ✓ | ✓ | ✓ | ✓ |
| `pattern(v, elements)` | ✓ | ✓ | — ergonomic gap | — ergonomic gap |
| `fromList(v, [v])` | ✓ | ✓ | ✗ | ✗ |
| `unfold` (anamorphism) | ✓ | ✓ | ✗ | ✗ |
| `length` | ✓ | ✓ | ✓ | ✓ |
| `size` | ✓ | ✓ | ✓ | ✓ |
| `depth` | ✓ | ✓ | ✓ | ✓ |
| `is_atomic` / `isAtomic` | ✗ | ✓ | ✓ | ✓ |
| `values()` | ✓ | ✓ | ✓ | ✓ |
| `toTuple` / `flatten` | ✓ | ✗ | ✗ | ✗ |

### Pattern Predicates and Queries

| Operation | Haskell | Rust | TypeScript | Python |
|---|---|---|---|---|
| `anyValue` | ✓ | ✓ | ✗ | ✗ |
| `allValues` | ✓ | ✓ | ✗ | ✗ |
| `matches` (structural eq) | ✓ | ✓ | ✗ | ✗ |
| `contains` (subpattern) | ✓ | ✓ | ✗ | ✗ |
| `filter` patterns | ✓ | ✓ | ✓ | ✓ |
| `findFirst` | ✓ | ✓ | ✓ | ✓ |
| `findAll` | ✓ | via `filter` | via `filter` | via `filter` |

### Pattern Transformations

| Operation | Haskell | Rust | TypeScript | Python |
|---|---|---|---|---|
| `map` (Functor) | ✓ | ✓ | ✓ | ✓ |
| `fold` | ✓ | ✓ | ✓ | ✓ |
| `para` (paramorphism) | ✓ | ✓ | ✗ | ✗ |
| `paraWithScope` | ✓ | ✗ | ✗ | ✗ |
| `traverse` (Traversable) | ✓ | ✓ `traverse_option/result` | ✗ | ✗ |
| Applicative `pure`, `<*>` | ✓ | ✗ | ✗ | ✗ |
| `combine` (Semigroup `<>`) | ✓ | ✓ | ✗ | ✗ |
| `mempty` (Monoid) | ✓ | via `Default` | ✗ | ✗ |
| `zip3` / `zipWith` | ✗ | ✓ | ✗ | ✗ |
| `validate` / `analyzeStructure` | ✗ | ✓ | ✗ | ✗ |

### Comonad Operations

| Operation | Haskell | Rust | TypeScript | Python |
|---|---|---|---|---|
| `extract` | ✓ | ✓ | ✓ | ✓ |
| `extend` | ✓ | ✓ | ✓ | ✓ |
| `duplicate` | ✓ | ✗ | ✓ | ✓ |
| `depthAt` | ✓ | ✓ | ✗ | ✗ |
| `sizeAt` | ✓ | ✓ | ✗ | ✗ |
| `indicesAt` | ✓ | ✓ | ✗ | ✗ |

### Subject API

| Operation | Haskell | Rust | TypeScript | Python |
|---|---|---|---|---|
| `fromId` constructor | ✓ | ✓ | ✓ | ✓ |
| empty `subject()` | ✓ | ✗ | ✗ | ✗ |
| `addProperty` | ✓ | ✗ | ✗ | ✗ |
| `updateProperty` | ✓ | ✗ | ✗ | ✗ |
| `removeProperty` | ✓ | ✗ | ✗ | ✗ |
| `hasProperty` | ✓ | ✗ | ✗ | ✗ |
| Builder pattern | ✗ | ✓ | ✓ | ✓ |
| Semigroup/combine | ✓ | ✓ `Combinable` | ✗ | ✗ |

---

## Gap Analysis by Language

### Rust

Rust is the most complete implementation. Gaps relative to Haskell are intentional (no `Applicative`, `paraWithScope`) or low priority:

**Missing:**
- `duplicate` — The `extend`/`extract` comonad laws are complete, but `duplicate` is absent. It can be expressed as `extend(identity)`, making it trivially derivable, but its absence means Comonad as a full algebraic structure is not explicit.
- `toTuple` — Trivial accessor that exposes `(value, elements)` pair; Rust users have direct field access so this is cosmetic.
- Applicative instance — No `pure`/`apply` equivalent. Low priority: no use case has required it yet.
- `paraWithScope` — Requires `ScopeQuery` abstraction which is not yet ported.
- Explicit `Semigroup` trait impl via `<>` operator — `combine` exists but it is tied to the `Combinable` trait, not `std::ops::Add` or a standard Semigroup convention.

**Rust-only additions (not gaps, but worth noting for cross-port consideration):**
- `traverse_option`, `traverse_result`, `validate_all`, `sequence_option`, `sequence_result` — Rust ergonomic alternatives to Haskell's `Traversable` type class.
- `zip3`, `zip_with` — Rust-specific convenience combinators.
- `validate`, `analyze_structure` — Rust-specific structural analysis, no Haskell equivalent.
- `from_list` constructor name differs from Haskell's `fromList`.

### TypeScript

TypeScript and Python share the same gap profile — both were ported together as bindings over the WASM/native core. The TypeScript package is more developed on the graph side (full `GraphView`, `paraGraph`, `paraGraphFixed`, `unfoldGraph`, etc.) but the core `Pattern<V>` API surface is trimmed.

**Missing:**
- `anyValue` / `allValues` — simple but absent; users must use `fold` or `filter` as workarounds.
- `matches` — structural pattern equality beyond reference equality; use case: pattern template matching.
- `contains` — subpattern search; needed for graph query patterns.
- `para` — the paramorphism. The graph tier has `paraGraph` but there is no `para` on `Pattern<V>` directly, making it impossible to write structure-aware folds without going through graph infrastructure.
- `unfold` (anamorphism) — `unfoldGraph` exists for graphs, but unfolding a pure `Pattern<V>` tree from a seed is missing.
- `fromList` constructor — convenience constructor for flat lists.
- `depthAt`, `sizeAt`, `indicesAt` — comonad helper operations. `duplicate` is present, making these derivable, but they are not exposed.
- Semigroup / `combine` — no way to merge two patterns; consequence: no Monoid either.
- **Ergonomic gap on non-atomic construction:** There is no `Pattern.pattern(value, elements)` constructor. Users must construct `new Pattern({ value, elements })` directly via the Effect `Data.Class` constructor. This is valid but undiscoverable.

### Python

The Python API closely mirrors TypeScript (they share the pure-Python implementation in `relateby.pattern`) with the same gaps. Additionally:

**Missing (same as TypeScript):** `anyValue`, `allValues`, `matches`, `contains`, `para`, `unfold`, `fromList`, `depthAt`, `sizeAt`, `indicesAt`, `combine`.

**Additional ergonomic gap:** `Pattern(value=x, elements=[...])` works via dataclass constructor, but there is no explicit `Pattern.pattern(x, [...])` factory method. The Python convention of builder/factory class methods (like `Pattern.point()`) makes the absence of `Pattern.from_values(v, [v1, v2])` noticeable.

**Missing advanced graph operations vs TypeScript:** Python's `StandardGraph` does not expose `topoSort`, `mapGraph`, `mapAllGraph`, `filterGraph`, `foldGraph`, `mapWithContext`, `paraGraph`, or `unfoldGraph`. The Python tier only provides the basic query API — no transforms.

---

## Observations on Design Divergence

### TypeScript uses Effect.ts idioms throughout
TypeScript leans on `Effect.ts` for structural equality (`Data.Class`), immutable collections (`HashSet`, `HashMap`), and typed errors (`Data.TaggedError`). Operations are curried for `pipe()` composition. This is idiomatic and appropriate for the TypeScript ecosystem, not a gap. However, it means the TypeScript API surface will always look different from Rust/Python even when feature-equivalent.

### Python's pure-Python approach
The Python package is pure Python (the Rust PyO3 bindings expose only gram parsing internally). This makes it portable and easy to iterate, but means any advanced Pattern operations must be implemented in Python rather than delegating to Rust. Given that the Rust library already has all these operations, providing PyO3 bindings for the richer set would be more efficient than reimplementing in Python.

### The "construction gap" in TypeScript and Python
Both TypeScript and Python can only construct leaf nodes via the public API (`Pattern.point()`/`Pattern.of()`). Creating composite patterns requires either using gram string parsing or direct class/dataclass construction. The Haskell and Rust APIs provide explicit `pattern(v, elements)` constructors. Adding `Pattern.withElements(value, elements)` or `Pattern.from_values(v, [v])` would close this gap.

---

## Recommendations

### Priority 1 — Core Pattern API completeness (TypeScript and Python)

These are high-value, low-effort additions that round out the base Pattern API in both TypeScript and Python.

1. **Add `anyValue` / `allValues` predicates** — Short-circuit folds over boolean predicates. Expressible as `fold` specializations; add as standalone functions in `ops.ts` and `Pattern` class method.

2. **Add `matches` and `contains`** — Structural pattern comparison. `matches` is deep equality; `contains` is subpattern search. Both use `===` / `==` semantics on values and recursive descent on elements.

3. **Add `para` (paramorphism)** — Structure-aware fold where the fold function sees both the current pattern and pre-computed results from its elements. This is the foundation for many graph-query algorithms and is already present in Rust. For TypeScript, add `para<V, R>(f: (p: Pattern<V>, subResults: readonly R[]) => R) => (p: Pattern<V>) => R`.

4. **Add `unfold` anamorphism** — Expand a seed value into a `Pattern<V>` tree. Dual to `fold`. Rust has this; it enables programmatic pattern construction without gram parsing.

5. **Add non-atomic pattern constructor** — `Pattern.pattern(value, elements)` in TypeScript and `Pattern.from_values(value, [...])` in Python as ergonomic alternatives to direct construction. `fromList(v, [v])` should also be added for list-of-leaves construction.

6. **Add comonad helpers** — `depthAt`, `sizeAt`, `indicesAt`. These are `extend` specializations; Rust has them and they are useful for graph layout and display work.

### Priority 2 — Semigroup/combine algebra (TypeScript and Python)

7. **Add `combine` (Semigroup) for `Pattern<V>`** — The Haskell Semigroup instance concatenates elements and combines values. Rust has `Combinable` with the same semantics. TypeScript and Python should add `combine(a: Pattern<V>, b: Pattern<V>): Pattern<V>` where `V` is combinable (via a type parameter constraint in TypeScript, or duck-typed in Python). This enables constructing patterns incrementally and is used by the reconciliation system.

8. **Add Monoid identity** — `Pattern.empty()` or `Pattern.mempty()` returning an identity pattern (equivalent to `mempty` in Haskell, `Default` in Rust). Requires `V` to have an identity value (or accept explicit empty value).

### Priority 3 — Rust completeness

9. **Add `duplicate` to Rust** — Trivially `extend(|p| p.clone())`. Omission means the Comonad interface is not fully expressed in Rust; other implementations have it.

10. **Add Subject property mutation helpers** — `add_property`, `remove_property`, `has_property`, `update_property` on `Subject`. Haskell has these; Rust only offers the builder pattern (which creates new instances). Non-destructive equivalents that return a new `Subject` would match the Haskell API and be useful in graph transform contexts where subjects need modification after construction.

### Priority 4 — Python graph transforms

11. **Add graph transform functions to Python** — `mapGraph`, `filterGraph`, `foldGraph`, `mapWithContext`, `paraGraph` are present in TypeScript but absent from Python's `StandardGraph`. These are the most frequently used operations for processing graph data. Options: PyO3-bind the Rust implementations, or port the TypeScript implementations to Python.

### Not Recommended (for now)

- **Applicative instance** — No concrete use case identified across any existing codebase. Skip until needed.
- **`paraWithScope`** — Requires porting `ScopeQuery`/`TrivialScope`/`ScopeDict` abstractions. The benefit is scope-aware structural folds (e.g., finding siblings), but this is an advanced feature with no current consumer.
- **`PatternKind` / `RepresentationMap`** — These are high-abstraction Haskell constructs for schema-level pattern classification and bijective transformations. No use case exists in the current codebase. Defer.

---

## Summary Table of Recommended Work

| # | Change | Target | Effort | Value |
|---|---|---|---|---|
| 1 | Add `anyValue` / `allValues` | TS + Python | Low | Medium |
| 2 | Add `matches` / `contains` | TS + Python | Low | High |
| 3 | Add `para` (paramorphism) | TS + Python | Medium | High |
| 4 | Add `unfold` anamorphism | TS + Python | Medium | Medium |
| 5 | Add `Pattern.pattern()` constructor | TS + Python | Low | Medium |
| 6 | Add comonad helpers (`depthAt`, `sizeAt`, `indicesAt`) | TS + Python | Low | Low |
| 7 | Add `combine` (Semigroup) | TS + Python | Medium | Medium |
| 8 | Add `Pattern.empty()` (Monoid) | TS + Python | Low | Low |
| 9 | Add `duplicate` | Rust | Low | Medium |
| 10 | Add Subject property helpers | Rust + TS + Python | Low | Medium |
| 11 | Add graph transforms to Python | Python | High | High |
