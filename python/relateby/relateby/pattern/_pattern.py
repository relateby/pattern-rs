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

    # --- Equality ---
    # @dataclass provides __eq__ that recursively compares fields.
    # For Pattern[Subject], Subject.__eq__ is also structural (dataclass default).

    def __iter__(self) -> Iterator["Pattern[V]"]:
        """Iterate over immediate child elements."""
        return iter(self.elements)
