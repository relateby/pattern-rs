# Python API Contract: Pattern Parity Operations

**Package**: `relateby-pattern` (installed as `pip install relateby-pattern`)
**Import**: `from relateby.pattern import Pattern, ...`
**Python version**: 3.8+

---

## New Class Methods on `Pattern[V]`

Added to `python/packages/relateby/relateby/pattern/_pattern.py`:

```python
class Pattern(Generic[V]):
    # existing methods ...

    @classmethod
    def pattern(cls, value: V, elements: list["Pattern[V]"]) -> "Pattern[V]":
        """Create a non-atomic pattern with explicit children."""
        ...

    @classmethod
    def from_list(cls, value: V, values: list[V]) -> "Pattern[V]":
        """Create a pattern whose children are atomic patterns over a list of values."""
        ...

    @classmethod
    def unfold(cls, expand: Callable[[A], tuple[V, list[A]]], seed: A) -> "Pattern[V]":
        """
        Anamorphism: expand a seed value into a Pattern[V] tree.
        expand returns (value, child_seeds); terminates when child_seeds is empty.
        """
        ...
```

---

## New Instance Methods on `Pattern[V]`

```python
class Pattern(Generic[V]):
    # existing methods ...

    # --- Predicates ---

    def any_value(self, predicate: Callable[[V], bool]) -> bool:
        """Return True if any value satisfies the predicate (short-circuit, pre-order)."""
        ...

    def all_values(self, predicate: Callable[[V], bool]) -> bool:
        """Return True if all values satisfy the predicate (short-circuit, pre-order)."""
        ...

    def matches(self, other: "Pattern[V]") -> bool:
        """Return True if this pattern and other have identical structure and equal values."""
        ...

    def contains(self, needle: "Pattern[V]") -> bool:
        """Return True if needle appears as a sub-pattern anywhere in this pattern."""
        ...

    # --- Transformations ---

    def para(self, f: Callable[["Pattern[V]", list[R]], R]) -> R:
        """
        Paramorphism: structure-aware bottom-up fold.
        f receives (current_pattern, [child_results]).
        """
        ...

    def combine(self, other: "Pattern[V]", combine_values: Callable[[V, V], V]) -> "Pattern[V]":
        """
        Combine two patterns: merge root values via combine_values, concatenate elements.
        combine_values SHOULD be associative.
        """
        ...

    # --- Comonad helpers ---

    def depth_at(self) -> "Pattern[int]":
        """Annotate each position with the depth of its subtree (leaf = 0)."""
        ...

    def size_at(self) -> "Pattern[int]":
        """Annotate each position with the total node count of its subtree."""
        ...

    def indices_at(self) -> "Pattern[list[int]]":
        """Annotate each position with the 0-based index path from root to that position."""
        ...
```

---

## New Module-Level Function

Added to `_pattern.py`, exported from `__init__.py`:

```python
def unfold(expand: Callable[[A], tuple[V, list[A]]], seed: A) -> Pattern[V]:
    """
    Standalone unfold function (mirrors TypeScript style).
    Equivalent to Pattern.unfold(expand, seed).
    """
    ...
```

---

## New Graph Transform Module

New file: `python/packages/relateby/relateby/pattern/_graph_transforms.py`

Standalone functions exported from `relateby.pattern`:

```python
from relateby.pattern import StandardGraph, Pattern, Subject

def map_graph(
    graph: StandardGraph,
    mappers: dict[str, Callable[[Pattern[Subject]], Pattern[Subject]]]
) -> StandardGraph:
    """
    Transform each element by class.
    mappers keys: "node", "relationship", "annotation", "walk", "other".
    Unspecified classes pass through unchanged.
    """
    ...

def map_all_graph(
    graph: StandardGraph,
    f: Callable[[Pattern[Subject]], Pattern[Subject]]
) -> StandardGraph:
    """Apply the same transformation to all elements regardless of class."""
    ...

def filter_graph(
    graph: StandardGraph,
    predicate: Callable[[str, Pattern[Subject]], bool],
    substitution: "Substitution"
) -> StandardGraph:
    """
    Remove elements where predicate returns False.
    predicate receives (graph_class_name, pattern).
    substitution: "delete_container" | "splice_gap" | ("replace_with_surrogate", Pattern[Subject])
    """
    ...

def fold_graph(
    graph: StandardGraph,
    f: Callable[[str, Pattern[Subject]], R],
    empty: R,
    combine: Callable[[R, R], R]
) -> R:
    """Reduce all classified elements to a single value using a monoid-like API."""
    ...

def map_with_context(
    graph: StandardGraph,
    f: Callable[["GraphQuery", Pattern[Subject]], Pattern[Subject]]
) -> StandardGraph:
    """
    Transform each element with access to a snapshot GraphQuery.
    The snapshot is frozen at the start of the transformation.
    """
    ...

def para_graph(
    graph: StandardGraph,
    f: Callable[["GraphQuery", Pattern[Subject], dict[str, R]], R]
) -> dict[str, R]:
    """
    Bottom-up fold in topological order.
    f receives (graph_query, pattern, {identity: result_for_dependency}).
    Returns dict mapping element identity to its fold result.
    """
    ...
```

---

## Updated `__init__.py` Exports

```python
# In relateby/pattern/__init__.py — add to existing imports:
from ._graph_transforms import (
    map_graph,
    map_all_graph,
    filter_graph,
    fold_graph,
    map_with_context,
    para_graph,
)

# Also export unfold at module level for standalone use:
from ._pattern import unfold

__all__ = [
    # existing ...
    "map_graph",
    "map_all_graph",
    "filter_graph",
    "fold_graph",
    "map_with_context",
    "para_graph",
    "unfold",
]
```

---

## Usage Examples

```python
from relateby.pattern import Pattern, Subject, map_graph, para_graph

# any_value / all_values
p = Pattern.from_list(Subject.from_id("root"), ["a", "b", "c"])  # wrong—values need Subject
# Correct: use with Subject values
has_person = p.any_value(lambda s: "Person" in s.labels)
all_identified = p.all_values(lambda s: s.identity != "")

# para: compute tree height
def height(pat: Pattern, child_heights: list[int]) -> int:
    return 0 if not child_heights else 1 + max(child_heights)

h = my_pattern.para(height)

# unfold: build descending chain
def expand(n: int) -> tuple[str, list[int]]:
    return (str(n), [n - 1] if n > 0 else [])

chain = Pattern.unfold(expand, 5)

# combine (string subjects)
merged = pat_a.combine(pat_b, lambda a, b: Subject(
    identity=a.identity or b.identity,
    labels=a.labels | b.labels,
    properties={**b.properties, **a.properties},
))

# depth_at / size_at / indices_at
depths = my_pattern.depth_at()     # Pattern[int]
sizes  = my_pattern.size_at()      # Pattern[int]
paths  = my_pattern.indices_at()   # Pattern[list[int]]

# map_graph
from relateby.pattern import map_graph
updated = map_graph(my_graph, {
    "node": lambda p: p.map(lambda s: s.with_label("Visited"))
})
```

---

## Breaking Changes

None. All additions are new methods and exports; no existing API is modified.

---

## Compatibility

All new operations are pure Python, in-memory only. No additional dependencies. Requires Python 3.8+.
