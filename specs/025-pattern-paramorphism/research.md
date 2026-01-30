# Research: Pattern Paramorphism

**Feature**: 025-pattern-paramorphism  
**Date**: 2026-01-30

## Overview

Paramorphism is a structure-aware folding operation. This document consolidates findings from the gram-hs reference implementation, documentation, and porting guide to guide the Rust implementation.

---

## 1. Reference Implementation Semantics

**Source**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` (lines 1188–1190)

```haskell
para :: (Pattern v -> [r] -> r) -> Pattern v -> r
para f (Pattern v els) = 
  f (Pattern v els) (map (para f) els)
```

**Decision**: Implement para in Rust as a method that (1) recursively computes results for each element via `para(f)`, (2) collects them into a slice, (3) calls the user function with `(self, &element_results)`.

**Rationale**: This is the standard paramorphism pattern for recursive structures: process elements first (bottom-up), then combine with the current node. No alternatives needed; gram-hs is authoritative.

**Alternatives considered**: None. Lazy/streaming or parallel variants are out of scope per spec.

---

## 2. Rust Porting Pattern

**Source**: `../gram-hs/docs/reference/PORTING-GUIDE.md` (lines 386–543)

**Decision**: Use references throughout: `&self`, `&Pattern<V>`, `&[R]` for the folding function. Collect element results into `Vec<R>` before calling the user function so the closure receives a slice. Use a single public method `para<R, F>(&self, f: F) -> R` where `F: Fn(&Pattern<V>, &[R]) -> R`.

**Rationale**: Avoids ownership issues; matches gram-hs semantics (pattern and list of results). Collecting into `Vec<R>` is necessary because Rust does not have Haskell’s lazy list; the slice `&[R]` gives the same interface as `[r]` in Haskell.

**Alternatives considered**: Passing an iterator of results—rejected because the closure may need to index or iterate multiple times; a slice is simpler and matches the reference. Taking ownership of the pattern—rejected because spec and constitution require non-destructive, reusable API.

---

## 3. Relationship to Foldable and Comonad

**Source**: `../gram-hs/docs/reference/features/paramorphism.md` (Relationship to Other Operations)

**Decision**: Document in quickstart and inline docs that (1) **Foldable** (e.g. `fold`) gives value-only folding; (2) **Paramorphism** (`para`) gives structure + element results for aggregation; (3) **Comonad** (e.g. `extend`) gives structure-aware transformation (returns a Pattern). Do not change Foldable or Comonad implementations; para is additive.

**Rationale**: Clarifies when to use para vs fold vs extend and satisfies spec success criterion SC-006.

**Alternatives considered**: None.

---

## 4. Property Tests to Port

**Source**: `../gram-hs/libs/pattern/tests/Spec/Pattern/Properties.hs` (T025–T030) and `CoreSpec.hs` (paramorphism describe block)

**Decision**: Port the following as unit and property tests:

- **Structure access**: `para(|p, _| p.depth())` equals `depth`; `para(|p, _| p.elements().len())` equals `elements().len()`.
- **Value access**: `para(|p, rs| { let mut v = vec![p.value().clone()]; for r in rs { v.extend(r.clone()); } v })` equals pre-order value list (equivalent to Foldable toList).
- **Relationship to Foldable**: `para(|p, rs| p.value() + rs.iter().sum())` equals `fold(0, |a, v| a + v)` for numeric patterns.
- **Order preservation**: Same value-access test implies left-to-right element order.
- **Edge cases**: Atomic pattern (empty element results); pattern with empty elements list; single element; nested patterns (unit tests from CoreSpec.hs T001–T010, T041–T048).

**Rationale**: Ensures behavioral equivalence with gram-hs and covers spec success criteria SC-004 and SC-005.

**Alternatives considered**: None; these tests are the reference validation strategy.

---

## 5. Folding Function Type and Cloning

**Decision**: Folding function type is `F: Fn(&Pattern<V>, &[R]) -> R`. The implementation will call `f(self, &child_results)`. For recursive calls, we need to pass the closure by reference to avoid moving it (same pattern as existing `fold_with` in pattern.rs). Use an internal `para_with(&self, f: &F) -> R` that takes `F` by reference; public `para` can take `F` by value and call `para_with` with `&f`.

**Rationale**: Matches existing fold_with pattern in pattern-core; avoids cloning the closure on every recursive call.

**Alternatives considered**: Requiring `F: FnMut`—rejected because para does not need mutable state. Taking `f` by reference in the public API—rejected for ergonomics (callers typically pass a closure by value).

---

## Summary Table

| Topic | Decision | Rationale |
|-------|----------|------------|
| Semantics | Port gram-hs para exactly | Reference implementation fidelity |
| Rust API | `para<R, F>(&self, f: F) -> R` with `F: Fn(&Pattern<V>, &[R]) -> R` | References, no ownership transfer; matches PORTING-GUIDE |
| Recursion | Internal `para_with(&self, f: &F)` | Avoid cloning closure; same as fold_with |
| Docs | Explain para vs fold vs extend | SC-006; user guidance |
| Tests | Port Properties.hs + CoreSpec.hs paramorphism tests | Equivalence and edge cases |

All NEEDS CLARIFICATION items from Technical Context are resolved; no open research questions.
