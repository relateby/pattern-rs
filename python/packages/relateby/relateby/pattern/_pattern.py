"""Native Python Pattern type — recursive tree structure.

Generic over value type V. Implemented as a @dataclass.
All operations run purely in Python — no Rust round-trip per operation.
"""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import Callable, Generic, Iterator, Optional, TypeVar

V = TypeVar("V")
U = TypeVar("U")
R = TypeVar("R")
A = TypeVar("A")


@dataclass
class Pattern(Generic[V]):
    value: V
    elements: list["Pattern[V]"] = field(default_factory=list)

    # --- Constructors ---

    @classmethod
    def point(cls, value: V) -> "Pattern[V]":
        return cls(value=value, elements=[])

    @classmethod
    def of(cls, value: V) -> "Pattern[V]":
        return cls.point(value)

    @classmethod
    def pattern(cls, value: V, elements: list["Pattern[V]"]) -> "Pattern[V]":
        """Create a non-atomic pattern with explicit children."""
        return cls(value=value, elements=list(elements))

    @classmethod
    def from_list(cls, value: V, values: list[V]) -> "Pattern[V]":
        """Create a pattern whose children are atomic patterns over a list of values."""
        return cls(value=value, elements=[cls.point(v) for v in values])

    @classmethod
    def unfold(cls, expand: Callable[[A], tuple[V, list[A]]], seed: A) -> "Pattern[V]":
        """Anamorphism: expand a seed value into a Pattern[V] tree.
        expand returns (value, child_seeds); terminates when child_seeds is empty.
        """
        value, child_seeds = expand(seed)
        return cls(value=value, elements=[cls.unfold(expand, cs) for cs in child_seeds])

    # --- Computed properties ---

    @property
    def is_atomic(self) -> bool:
        return len(self.elements) == 0

    @property
    def length(self) -> int:
        return len(self.elements)

    @property
    def size(self) -> int:
        return 1 + sum(e.size for e in self.elements)

    @property
    def depth(self) -> int:
        if self.is_atomic:
            return 0
        return 1 + max(e.depth for e in self.elements)

    # --- Operations ---

    def map(self, fn: Callable[[V], U]) -> "Pattern[U]":
        """Transform every value, preserving structure. Pre-order."""
        return Pattern(value=fn(self.value), elements=[e.map(fn) for e in self.elements])

    def fold(self, init: R, fn: Callable[[R, V], R]) -> R:
        """Accumulate values via pre-order traversal (root first)."""
        acc = fn(init, self.value)
        for e in self.elements:
            acc = e.fold(acc, fn)
        return acc

    def filter(self, predicate: Callable[["Pattern[V]"], bool]) -> list["Pattern[V]"]:
        """Collect all matching subtrees in pre-order."""
        results: list[Pattern[V]] = []
        if predicate(self):
            results.append(self)
        for e in self.elements:
            results.extend(e.filter(predicate))
        return results

    def find_first(self, predicate: Callable[[V], bool]) -> Optional[V]:
        """Return the first value matching the predicate, or None."""
        if predicate(self.value):
            return self.value
        for e in self.elements:
            result = e.find_first(predicate)
            if result is not None:
                return result
        return None

    def extend(self, fn: Callable[["Pattern[V]"], U]) -> "Pattern[U]":
        """Context-aware map: fn sees the full subtree at each position (comonad)."""
        return Pattern(value=fn(self), elements=[e.extend(fn) for e in self.elements])

    def extract(self) -> V:
        """Extract the root value (comonad)."""
        return self.value

    def duplicate(self) -> "Pattern[Pattern[V]]":
        """Each node's value becomes its own subtree (comonad)."""
        return Pattern(value=self, elements=[e.duplicate() for e in self.elements])

    def values(self) -> list[V]:
        """Return all values in pre-order traversal order."""
        return self.fold([], lambda acc, v: acc + [v])

    def any_value(self, predicate: Callable[[V], bool]) -> bool:
        """Return True if any value in the tree satisfies the predicate. Short-circuits pre-order."""
        if predicate(self.value):
            return True
        return any(e.any_value(predicate) for e in self.elements)

    def all_values(self, predicate: Callable[[V], bool]) -> bool:
        """Return True if every value in the tree satisfies the predicate. Short-circuits pre-order."""
        if not predicate(self.value):
            return False
        return all(e.all_values(predicate) for e in self.elements)

    def matches(self, other: "Pattern[V]") -> bool:
        """Return True if this pattern is structurally equal to other."""
        return self == other

    def contains(self, needle: "Pattern[V]") -> bool:
        """Return True if needle appears anywhere in this pattern (including at root)."""
        return self == needle or any(e.contains(needle) for e in self.elements)

    def para(self, f: Callable[["Pattern[V]", list[R]], R]) -> R:
        """Structure-aware fold: f receives both the current sub-pattern and pre-computed child results. Bottom-up."""
        return f(self, [e.para(f) for e in self.elements])

    def depth_at(self) -> "Pattern[int]":
        """Annotate every position with its depth (0 for leaves)."""
        return self.extend(lambda sub: sub.depth)

    def size_at(self) -> "Pattern[int]":
        """Annotate every position with its subtree size (1 for leaves)."""
        return self.extend(lambda sub: sub.size)

    def indices_at(self) -> "Pattern[list[int]]":
        """Annotate every position with its root-path index list ([] for root)."""
        def go(pat: "Pattern[V]", path: list[int]) -> "Pattern[list[int]]":
            return Pattern(
                value=path,
                elements=[go(e, path + [i]) for i, e in enumerate(pat.elements)]
            )
        return go(self, [])

    def combine(self, other: "Pattern[V]", combine_values: Callable[[V, V], V]) -> "Pattern[V]":
        """Combine two patterns: merge root values via combine_values, concatenate elements."""
        return Pattern(
            value=combine_values(self.value, other.value),
            elements=list(self.elements) + list(other.elements),
        )

    # --- Equality ---
    # @dataclass provides __eq__ that recursively compares fields.
    # For Pattern[Subject], Subject.__eq__ is also structural (dataclass default).

    def __iter__(self) -> Iterator["Pattern[V]"]:
        """Iterate over immediate child elements."""
        return iter(self.elements)


def unfold(expand: Callable[[A], tuple[V, list[A]]], seed: A) -> Pattern[V]:
    """Standalone unfold function. Equivalent to Pattern.unfold(expand, seed)."""
    return Pattern.unfold(expand, seed)
