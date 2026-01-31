# Research: Paramorphism in Python Bindings

**Feature**: 026-paramorphism-python-bindings  
**Date**: 2026-01-31

## Overview

This document records research decisions for exposing Rust `para` in the pattern-core Python bindings. There were no unresolved NEEDS CLARIFICATION items; research focuses on integration patterns and best practices.

---

## 1. PyO3 Callback Pattern for Recursive para

**Task**: Research how to implement `para` in Python bindings when Rust `para` expects a closure `Fn(&Pattern<V>, &[R]) -> R` and we need to call a Python callable at each node.

**Decision**: Use a Rust closure that captures the Python callable and the `Python` token. At each node, the Rust closure (1) builds a Python-visible “pattern view” (e.g. a thin wrapper or the existing PyPattern bound to the current node), (2) builds a Python list of element results (each result is already a PyObject from the recursive Python call), (3) calls the Python callable with `(pattern_view, element_results_list)`, (4) returns the resulting PyObject converted to Rust for the current `R`. Recursion is done in Rust (`para_with`); each step only calls into Python once per node.

**Rationale**: Matches the existing pattern for `fold` and `map`: Rust drives the traversal, Python provides the per-node logic. Keeping recursion in Rust avoids deep Python stack and preserves Rust’s element order and bottom-up semantics.

**Alternatives considered**:
- **Python-side recursion**: Rust exposes a “get current pattern + get element results” API and Python recurses. Rejected because it would duplicate traversal logic and make order/semantics harder to keep in sync with Rust.
- **Serialize pattern to Python and back**: Rejected; too costly and unnecessary since we already have PyPattern.

---

## 2. “Pattern View” Passed to the Callable

**Task**: What should the first argument to the Python callable be?

**Decision**: Pass the same `Pattern` (PyPattern) instance that Python users already have, bound to the current node. That is, the callable receives `(pattern_at_node, element_results)`. The Python type is the existing `Pattern` class; at each node we create a new PyPattern wrapping the current `&Pattern<V>` (or we expose a view that delegates to the current node). So the callable can use `.value`, `.elements`, `.depth()`, `.length()` etc. as today.

**Rationale**: Spec FR-002 and FR-007 require “a representation of the current pattern” and access to “structural information (e.g., depth, element count)”. Reusing the existing Pattern type keeps the API simple and consistent with the rest of the bindings.

**Alternatives considered**:
- **New “PatternView” type**: Would duplicate attributes (value, depth, length) and complicate the API; rejected.
- **Pass only value + depth + length**: Would preclude structure-preserving transformation (building a new Pattern from the current node and element results); rejected.

---

## 3. Type of “Element Results” in Python

**Task**: Should element results be a list of arbitrary Python objects or a typed sequence?

**Decision**: Pass a Python list of the results from the children. Each element is whatever the Python callable returned for that child (int, tuple, Pattern, etc.). So the second argument is `list[Any]` in type hints; at runtime it’s a list. Order is guaranteed: same as `pattern.elements` (depth-first, left-to-right).

**Rationale**: Rust `para` is generic over `R`; in Python we don’t have a single static type for `R`, so `list[Any]` (or `Sequence[Any]`) is appropriate. Homogeneous use (e.g. all ints or all Patterns) is a convention of the callable; we document that structure-preserving para should return Pattern for every node.

**Alternatives considered**:
- **Tuple instead of list**: List matches “sequence” and is mutable if the user wants to copy/transform; tuple would also work but list is consistent with `elements`. Chose list.
- **Typed generic**: e.g. `para[R](func: Callable[[Pattern, List[R]], R])`. Python typing can express this with a TypeVar; we’ll add that in .pyi for clarity.

---

## 4. Supporting Both Value Aggregation and Structure-Preserving para

**Task**: Rust `para` returns `R`; in Python we need to support (a) returning a single value (e.g. int, tuple) and (b) returning a new Pattern (structure-preserving).

**Decision**: One method `para(self, func: Callable[[Pattern, List[Any]], Any]) -> Any`. Return type is `Any`. When the callable returns a Pattern at every node, the root result is a Pattern; when it returns numbers/tuples, the root result is that type. No separate method or overload required; the same Rust implementation supports both. Type stubs can use an overload or a generic TypeVar for better inference (e.g. `def para(self, func: Callable[[Pattern, List[R]], R]) -> R`).

**Rationale**: Matches Rust’s single `para` API. Python’s dynamic typing allows one method; we document and type-hint the two common cases (value aggregation vs structure-preserving).

**Alternatives considered**:
- **Two methods** (e.g. `para_value` and `para_pattern`): Would duplicate logic and diverge from Rust; rejected.

---

## 5. Error Handling and GIL

**Task**: What if the Python callable raises or Rust panics? How to manage GIL?

**Decision**: (1) Rust code runs with GIL held when calling into Python (same as `fold`/`map`). (2) If the Python callable raises, PyO3 converts it to a Rust error and we return a `PyResult`; the binding layer converts that to a Python exception. (3) Invalid input (e.g. non-callable) is rejected in the binding with `TypeError`, consistent with other methods. No extra GIL handling beyond “obtain Python when calling the callable”.

**Rationale**: Aligns with existing pattern-core Python bindings and PyO3 best practices.

**Alternatives considered**: None; current approach is standard for PyO3 callbacks.

---

## 6. Order and Atomic Base Case

**Task**: Document and guarantee element order and atomic behavior.

**Decision**: (1) Element results are in the same order as `pattern.elements` (left-to-right). (2) For an atomic pattern, the callable receives that pattern and an empty list `[]`. These semantics are documented in the API contract and in docstrings, and tests verify order and atomic base case.

**Rationale**: Spec FR-003, FR-004, FR-006 require bottom-up evaluation, consistent order, and empty sequence for atomic patterns. Rust `para` already guarantees this; we only need to preserve it across the boundary and document it.

---

## 7. Relationship to fold

**Task**: How to document when `para` can replicate `fold`?

**Decision**: Document in API and user docs: “If your callable only uses the current pattern’s value and the sum (or other reduction) of element results, you can replicate fold. Example: `p.para(lambda pat, rs: pat.value + sum(rs))` is equivalent to `p.fold(0, lambda acc, v: acc + v)` for numeric values.” Add a short “Para vs fold” subsection in quickstart or docs.

**Rationale**: Spec FR-009 and SC-004 require documenting the relationship so users can choose the right operation.

---

---

## 8. PatternSubject Review and Removal

**Task**: Should we keep or remove the separate PatternSubject class in Python bindings?

**Decision**: **Review and remove if possible.** The spec clarifies that paramorphism is on the generic Pattern class only; when values are Subjects, use Pattern with Subject as the value type. The plan includes: (1) Review all usages of PatternSubject in tests, examples, and docs. (2) If removal is feasible without breaking 024 guarantees, remove PatternSubject: migrate callers to Pattern.point(subject) and Pattern.pattern(subject, elements); remove PyPatternSubject from python.rs and __init__.pyi; update __all__ and re-exports. (3) If removal is deferred (e.g. breaking change too large for this feature), deprecate PatternSubject and document migration to Pattern with Subject as value; add para only on Pattern.

**Rationale**: Simplifies the API to a single generic class; aligns with spec clarification and user request. Removal is preferred; deprecation is fallback if migration scope is too large for this branch.

**Alternatives considered**:
- **Keep PatternSubject and add para to both**: Rejected; spec and user input prefer a single generic Pattern.
- **Deprecate only, no removal**: Acceptable fallback if removal would require too many changes in one step.

---

## 9. Pattern as Generic Class in Python Type Stubs

**Task**: How should Pattern be declared in .pyi for type checkers?

**Decision**: Define Pattern as a **generic class** using `from typing import TypeVar, Generic`: e.g. `V = TypeVar('V')` and `class Pattern(Generic[V]):`. Constructors and methods use the type parameter: `def point(cls, value: V) -> Pattern[V]:`, `def pattern(cls, value: V, elements: List[Pattern[V]]) -> Pattern[V]:`, `def para(self, func: Callable[[Pattern[V], List[R]], R]) -> R` (with a second TypeVar R for the result). This allows `Pattern[int]`, `Pattern[Subject]`, and correct inference for para return type.

**Rationale**: Matches Rust’s `Pattern<V>`; improves type checker and IDE support; satisfies user request to “properly define Pattern as a generic class using TypeVar, Generic”.

**Alternatives considered**:
- **Pattern without Generic**: Rejected; loses value-type inference (e.g. Pattern[Subject]).
- **Separate PatternSubject as subclass**: Rejected; spec prefers single generic Pattern.

---

## Summary Table

| Topic              | Decision                                                                 | Rationale / Notes                    |
|-------------------|--------------------------------------------------------------------------|--------------------------------------|
| Recursion         | In Rust; closure calls Python once per node                             | Same as fold/map; preserves semantics |
| Pattern view      | Existing Pattern (PyPattern) bound to current node                      | Reuse API; supports depth/length     |
| Element results   | Python list, same order as elements; type `list[Any]` / TypeVar in .pyi  | Flexible; order guaranteed            |
| One method        | Single `para(func)`; return type Any (or generic R in stubs)             | Matches Rust; supports both uses     |
| Errors / GIL      | GIL when calling Python; Python exceptions propagated                   | Standard PyO3                         |
| Order & atomic    | Left-to-right; empty list for atomic                                    | Document and test                    |
| Para vs fold      | Document + example in API and quickstart                                | Meets FR-009, SC-004                 |
| PatternSubject    | Review and remove if possible; else deprecate                           | Single generic Pattern; spec clarification |
| Pattern Generic[V]| Define class Pattern(Generic[V]) in .pyi with TypeVar, Generic           | Correct typing; Pattern[int], Pattern[Subject] |
