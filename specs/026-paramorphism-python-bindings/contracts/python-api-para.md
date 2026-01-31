# Python API Contract: Paramorphism (para)

**Feature**: 026-paramorphism-python-bindings  
**Date**: 2026-01-31

## Module: pattern_core

Extension to the existing **generic** Pattern API. Pattern is declared as `class Pattern(Generic[V]):` in type stubs (`from typing import TypeVar, Generic`). Paramorphism is exposed on Pattern only; when values are Subjects, use `Pattern[Subject]` (or Pattern with Subject as the value type). No separate PatternSubject class is required for para.

---

## Pattern.para

Structure-aware folding: the callable receives the current pattern and the list of results from its elements (in element order), and returns an aggregated result for this node.

**Signature** (type stubs use Generic[V] and a result TypeVar R):

```python
def para(self, func: Callable[[Pattern[V], List[R]], R]) -> R:
    """
    Paramorphism: structure-aware fold.

    func(pattern, element_results) -> result
    - pattern: the Pattern at the current node (same type as self).
    - element_results: list of results from para on each element, in same order as pattern.elements.
      Empty list when pattern has no elements (atomic).

    Evaluation is bottom-up: element results are computed first, then func is called.
    Returns the result at the root.
    """
    ...
```

**Parameters**:
- **self**: Pattern[V] instance (e.g. Pattern[int], Pattern[Subject]).
- **func**: Callable of two arguments:
  - First: the Pattern at the current node (read-only view; `.value`, `.elements`, `.depth()`, `.length()` etc. available).
  - Second: list of results from applying `para` to each element, in left-to-right order; empty list for atomic patterns.
  - Returns: any Python object (typically same type for all nodes in one use: e.g. int, tuple, or Pattern for structure-preserving).

**Returns**: The result of applying `func` at the root after all nodes have been processed (type matches the callable’s return type).

**Semantics**:
- **Order**: Element results are in the same order as `pattern.elements` (depth-first, left-to-right).
- **Atomic**: For a pattern with no elements, `func` receives that pattern and `[]`.
- **Non-destructive**: The pattern is not modified.
- **Single traversal**: Each node is visited exactly once.

**Errors**:
- If `func` is not callable: `TypeError`.
- If `func` raises: exception propagates.
- Rust errors from the binding layer are converted to Python exceptions (e.g. `ValueError`, `TypeError`) consistent with other pattern-core Python methods.

**Type hints (.pyi)**:
- Pattern is declared as `class Pattern(Generic[V]):` using `from typing import TypeVar, Generic`. Use a second TypeVar R for the result: `def para(self, func: Callable[[Pattern[V], List[R]], R]) -> R` so that type checkers infer the return type from the callable.

---

## Examples (contract-level)

**Value aggregation (depth-weighted sum)**:

```python
p = Pattern.pattern(10, [Pattern.point(5), Pattern.point(3)])
result = p.para(lambda pat, rs: pat.value * pat.depth() + sum(rs))
# result: int
```

**Multiple statistics in one pass**:

```python
def stats(pat: Pattern, rs: List[tuple]) -> tuple:
    s, c, d = (sum(x[0] for x in rs), sum(x[1] for x in rs), max([x[2] for x in rs], default=0))
    return (pat.value + s, 1 + c, max(pat.depth(), d))
p.para(stats)  # -> (sum, count, max_depth)
```

**Structure-preserving transformation**:

```python
def scale_by_depth(pat: Pattern, rs: List[Pattern]) -> Pattern:
    new_val = pat.value * (pat.depth() + 1)
    return Pattern.pattern(new_val, list(rs))
p.para(scale_by_depth)  # -> Pattern with same structure, scaled values
```

**Parity with fold** (value-only reduction):

```python
# p.para(lambda pat, rs: pat.value + sum(rs))  is equivalent to  p.fold(0, lambda acc, v: acc + v)
# when values are numeric and we want sum.
```
