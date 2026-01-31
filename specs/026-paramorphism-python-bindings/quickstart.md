# Quickstart: Paramorphism in Python Bindings

**Feature**: 026-paramorphism-python-bindings  
**Date**: 2026-01-31

## Prerequisites

- pattern-core Python bindings installed (see [024 quickstart](../../024-python-pattern-core/quickstart.md) or `crates/pattern-core/README.md`).
- Build with: `cd crates/pattern-core && maturin develop --uv --features python`

## What is para?

`para` (paramorphism) is structure-aware folding: your function receives **the current pattern** and **the list of results from its elements**, and returns a result for this node. Evaluation is bottom-up, so you can aggregate over structure (depth, element count, element order) in one pass.

- **fold**: only sees values; good for simple reduction (e.g. sum).
- **para**: sees pattern + element results; good for depth-weighted sums, nesting stats, or building a new pattern from structure.

## 1. Value aggregation: depth-weighted sum

```python
import pattern_core

p = pattern_core.Pattern.pattern(10, [
    pattern_core.Pattern.point(5),
    pattern_core.Pattern.point(3),
])
# Structure: 10 has elements [5, 3]. Depth: root=1, leaves=0.

def depth_weighted(pat, element_results):
    return pat.value * pat.depth() + sum(element_results)

result = p.para(depth_weighted)
# Root: 10*1 + (5+0) + (3+0) = 10 + 5 + 3 = 18 (or as per Rust example)
print(result)
```

## 2. Multiple statistics in one pass

```python
import pattern_core

p = pattern_core.Pattern.pattern(1, [
    pattern_core.Pattern.pattern(2, [pattern_core.Pattern.point(3)]),
    pattern_core.Pattern.point(4),
])

def nesting_stats(pat, rs):
    child_sum = sum(r[0] for r in rs)
    child_count = sum(r[1] for r in rs)
    child_max_d = max((r[2] for r in rs), default=0)
    return (pat.value + child_sum, 1 + child_count, max(pat.depth(), child_max_d))

total_sum, node_count, max_depth = p.para(nesting_stats)
print(total_sum, node_count, max_depth)  # e.g. 10, 4, 2
```

## 3. Structure-preserving transformation

Build a new pattern with the same structure but transformed values (e.g. scale by depth):

```python
import pattern_core

p = pattern_core.Pattern.pattern(1, [
    pattern_core.Pattern.pattern(2, [pattern_core.Pattern.point(3)]),
])

def scale_by_depth(pat, element_results):
    new_value = pat.value * (pat.depth() + 1)
    return pattern_core.Pattern.pattern(new_value, list(element_results))

transformed = p.para(scale_by_depth)
# Same shape; values scaled by (depth+1)
print(transformed.value, transformed.elements[0].value, transformed.elements[0].elements[0].value)
```

## 4. Atomic pattern (base case)

For a pattern with no elements, your function receives that pattern and an empty list:

```python
atomic = pattern_core.Pattern.point(42)
result = atomic.para(lambda pat, rs: (pat.value, len(rs)))
# result == (42, 0); rs is []
print(result)
```

## 5. Para vs fold

When you only need to reduce over values (e.g. sum), you can use either:

- **fold**: `p.fold(0, lambda acc, v: acc + v)`
- **para**: `p.para(lambda pat, rs: pat.value + sum(rs))`

Both give the same result for a simple sum. Use `para` when you need the current pattern (depth, length, structure) or the ordered list of element results.

## Order guarantee

Element results are always in the **same order** as `pattern.elements` (left-to-right). You can rely on this for order-sensitive aggregations (e.g. “first element result”, “sequence of values”).

## Pattern as generic type

In type stubs, Pattern is declared as a generic class: `class Pattern(Generic[V]):` (using `from typing import TypeVar, Generic`). Use `Pattern[int]`, `Pattern[Subject]`, etc. for typed code. When values are Subjects, use `Pattern.point(subject)` and `Pattern.pattern(subject, elements)`; no separate PatternSubject class is required.

## Next steps

- Run Rust paramorphism examples: `cargo run --example paramorphism_usage` (in `examples/pattern-core/`).
- Compare Python output with Rust for the same pattern and logic.
- See [contracts/python-api-para.md](contracts/python-api-para.md) for the full API contract.
