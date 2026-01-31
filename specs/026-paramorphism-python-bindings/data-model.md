# Data Model: Paramorphism in Python Bindings

**Feature**: 026-paramorphism-python-bindings  
**Date**: 2026-01-31

## Overview

This document describes the data and API model for the paramorphism feature in the pattern-core Python bindings. It extends the existing Pattern model (see 024-python-pattern-core data-model.md) with one new operation and clarifies the types involved.

## Entities

### Pattern (extended, generic)

**Python Class**: `pattern_core.Pattern(Generic[V])` — Generic pattern type with value type `V` (e.g. `int`, `Subject`). Defined in type stubs using `from typing import TypeVar, Generic`: `class Pattern(Generic[V]):`. No separate PatternSubject class; use `Pattern[Subject]` (or `Pattern` with Subject as value) when values are Subjects.

**New Method**:
- `para(self, func: Callable[[Pattern[V], List[R]], R]) -> R` — Structure-aware folding. `func` receives the current pattern (at this node) and a list of results from this pattern’s elements (same order as `elements`). Returns the aggregated result for the root. For atomic patterns, the second argument is an empty list.

No new attributes or constructors. Existing attributes (`value`, `elements`) and methods (`depth()`, `length()`, etc.) are used by the callable. Type stubs declare Pattern as Generic[V] so that `Pattern.point(value)` returns `Pattern[V]` and `para` can be typed with a result TypeVar R.

### Paramorphism callable

**Concept**: User-provided Python callable used as the folding function.

**Signature**: `(pattern: Pattern, element_results: List[Any]) -> Any`

- **pattern**: The Pattern at the current node (same type as `self` in method calls). Supports `.value`, `.elements`, `.depth()`, `.length()`, etc.
- **element_results**: List of results from applying `para` recursively to each element, in the same order as `pattern.elements`. Empty list when the pattern is atomic.
- **Return**: Any Python object. Common cases:
  - **Value aggregation**: e.g. `int`, `float`, `tuple` (e.g. (sum, count, max_depth)). Root result has that type.
  - **Structure-preserving**: Return a new `Pattern` at each node; root result is a `Pattern` with the same structure and transformed values.

**Invariants**:
- Called exactly once per node, in bottom-up order.
- Element results are in left-to-right (depth-first) order.

### Element results

**Concept**: The list passed as the second argument to the callable.

**Type**: `List[Any]` (in practice, homogeneous per use: e.g. all ints or all Patterns).

**Order**: Same as `pattern.elements` (left-to-right).

**Length**: Equals `len(pattern.elements)`; 0 for atomic patterns.

## PatternSubject (deprecated / removed)

**Design decision**: No separate `PatternSubject` class is required for this feature. The spec and plan call for reviewing PatternSubject and removing it if possible; when values are Subjects, use the generic `Pattern` with Subject as the value type (e.g. `Pattern.point(subject)`, `Pattern.pattern(subject, elements)`). Type stubs use `Pattern[Subject]` for patterns whose values are Subjects. If PatternSubject is removed, all existing usages are migrated to Pattern; if deferred, PatternSubject is deprecated and para is exposed only on Pattern.

## State and Validation

- **Immutability**: The pattern is not modified by `para`; the callable may build and return new values or new Patterns.
- **No new validation rules**: Existing pattern structure invariants apply; `para` does not change them.

## Relationships

- **Pattern[V]** —has one— **para** (method).
- **para** —invokes— **Paramorphism callable** (user-provided).
- **Paramorphism callable** —receives— **Pattern[V]** (current node) and **Element results** (list).
- **Element results** —produced by— recursive **para** on each element.
- **Pattern** is declared as **Generic[V]** in .pyi (TypeVar V, typing.Generic).
