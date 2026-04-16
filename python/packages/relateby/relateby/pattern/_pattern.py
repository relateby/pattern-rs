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
    """A value paired with an ordered list of elements, each itself a Pattern[V].

    Patterns compose recursively: an atomic Pattern has no elements; any
    other Pattern is a value in the context of its constituent patterns.

    The structure is general-purpose — a Pattern can represent anything
    compositional. A Shakespearean sonnet, for instance, is a Pattern
    whose value is the title and whose elements are stanzas; each stanza
    is a Pattern whose elements are lines; each line is an atomic Pattern
    (a value with no elements).

    Graph elements are one specialisation: a Pattern with no elements is
    treated as a node, one with two elements as a relationship, and so on.
    That interpretation lives in StandardGraph, not here.

    Attributes:
        value: The value at this position in the composition.
        elements: Ordered list of constituent sub-patterns. Empty for
            atomic patterns.
    """

    value: V
    elements: list["Pattern[V]"] = field(default_factory=list)

    # --- Constructors ---

    @classmethod
    def point(cls, value: V) -> "Pattern[V]":
        """Create an atomic (leaf) pattern holding a single value.

        Args:
            value: The value for the atomic pattern.

        Returns:
            A Pattern with no children.
        """
        return cls(value=value, elements=[])

    @classmethod
    def of(cls, value: V) -> "Pattern[V]":
        """Alias for ``point``. Create an atomic pattern holding a single value.

        Args:
            value: The value for the atomic pattern.

        Returns:
            A Pattern with no children.
        """
        return cls.point(value)

    @classmethod
    def pattern(cls, value: V, elements: list["Pattern[V]"]) -> "Pattern[V]":
        """Create a non-atomic pattern with explicit children.

        Args:
            value: The value at the root of the new pattern.
            elements: Child patterns. Copied so the caller's list is
                not aliased.

        Returns:
            A composite Pattern whose elements are a copy of the
            supplied list.
        """
        return cls(value=value, elements=list(elements))

    @classmethod
    def from_list(cls, value: V, values: list[V]) -> "Pattern[V]":
        """Create a pattern whose children are atomic patterns over a list of values.

        Args:
            value: The value at the root of the new pattern.
            values: Sequence of values, each wrapped in an atomic child
                pattern.

        Returns:
            A Pattern whose elements are ``[point(v) for v in values]``.
        """
        return cls(value=value, elements=[cls.point(v) for v in values])

    @classmethod
    def unfold(cls, expand: Callable[[A], tuple[V, list[A]]], seed: A) -> "Pattern[V]":
        """Anamorphism: grow a Pattern tree from a seed value.

        Applies ``expand`` recursively until it returns an empty list of
        child seeds, at which point the branch becomes a leaf.

        Args:
            expand: A function ``seed -> (value, child_seeds)``.  When
                ``child_seeds`` is empty the node becomes atomic.
            seed: The initial seed value passed to ``expand``.

        Returns:
            A Pattern tree whose structure mirrors the recursive
            expansion of ``seed``.

        Example:
            >>> # Build a balanced binary tree of depth 2
            >>> def expand(n):
            ...     if n == 0:
            ...         return (n, [])
            ...     return (n, [n - 1, n - 1])
            >>> tree = Pattern.unfold(expand, 2)
            >>> tree.depth
            2
        """
        value, child_seeds = expand(seed)
        return cls(value=value, elements=[cls.unfold(expand, cs) for cs in child_seeds])

    # --- Computed properties ---

    @property
    def is_atomic(self) -> bool:
        """True when this pattern has no elements (is atomic)."""
        return len(self.elements) == 0

    @property
    def length(self) -> int:
        """Number of immediate child elements."""
        return len(self.elements)

    @property
    def size(self) -> int:
        """Total count of patterns in this composition, including this one (recursive)."""
        return 1 + sum(e.size for e in self.elements)

    @property
    def depth(self) -> int:
        """Maximum nesting depth of this pattern's elements (0 for atomic patterns)."""
        if self.is_atomic:
            return 0
        return 1 + max(e.depth for e in self.elements)

    # --- Operations ---

    def map(self, fn: Callable[[V], U]) -> "Pattern[U]":
        """Transform every value, preserving structure. Pre-order.

        Args:
            fn: A function applied to the value at each node.

        Returns:
            A new Pattern with the same shape and ``fn`` applied to
            every value.
        """
        return Pattern(value=fn(self.value), elements=[e.map(fn) for e in self.elements])

    def fold(self, init: R, fn: Callable[[R, V], R]) -> R:
        """Accumulate values across a pattern, visiting each value before its elements (pre-order).

        Args:
            init: Initial accumulator value.
            fn: A function ``(accumulator, value) -> new_accumulator``
                called for each value in order.

        Returns:
            The final accumulator after visiting every node.

        Example:
            >>> p = Pattern.from_list(1, [2, 3])
            >>> p.fold(0, lambda acc, v: acc + v)
            6
        """
        acc = fn(init, self.value)
        for e in self.elements:
            acc = e.fold(acc, fn)
        return acc

    def filter(self, predicate: Callable[["Pattern[V]"], bool]) -> list["Pattern[V]"]:
        """Collect all matching subtrees in pre-order.

        Args:
            predicate: A function that receives a Pattern node and
                returns True if that subtree should be included.

        Returns:
            A list of subtrees (Pattern nodes) for which the predicate
            returned True, in pre-order.
        """
        results: list[Pattern[V]] = []
        if predicate(self):
            results.append(self)
        for e in self.elements:
            results.extend(e.filter(predicate))
        return results

    def find_first(self, predicate: Callable[[V], bool]) -> Optional[V]:
        """Return the first value matching the predicate, or None.

        Args:
            predicate: A function applied to each value in pre-order.
                The search stops at the first match.

        Returns:
            The first matching value, or None if no value satisfies
            the predicate.
        """
        if predicate(self.value):
            return self.value
        for e in self.elements:
            result = e.find_first(predicate)
            if result is not None:
                return result
        return None

    def extend(self, fn: Callable[["Pattern[V]"], U]) -> "Pattern[U]":
        """Context-aware map: fn sees the full sub-pattern at each position (comonad).

        Args:
            fn: A function that receives the sub-pattern at each position
                and returns a new value for that position.

        Returns:
            A new Pattern with the same shape, where each node's value
            is the result of applying ``fn`` to the corresponding
            subtree of the original.
        """
        return Pattern(value=fn(self), elements=[e.extend(fn) for e in self.elements])

    def extract(self) -> V:
        """Return this pattern's value (comonad extract)."""
        return self.value

    def duplicate(self) -> "Pattern[Pattern[V]]":
        """Replace each position's value with its own sub-pattern (comonad duplicate)."""
        return Pattern(value=self, elements=[e.duplicate() for e in self.elements])

    def values(self) -> list[V]:
        """Return all values in pre-order traversal order."""
        return self.fold([], lambda acc, v: acc + [v])

    def any_value(self, predicate: Callable[[V], bool]) -> bool:
        """Return True if any value satisfies the predicate (short-circuits, pre-order).

        Args:
            predicate: A function applied to each value until one returns True.

        Returns:
            True if at least one value in the tree satisfies the predicate.
        """
        if predicate(self.value):
            return True
        return any(e.any_value(predicate) for e in self.elements)

    def all_values(self, predicate: Callable[[V], bool]) -> bool:
        """Return True if every value satisfies the predicate (short-circuits, pre-order).

        Args:
            predicate: A function applied to each value; the check stops
                at the first False.

        Returns:
            True if every value in the tree satisfies the predicate.
        """
        if not predicate(self.value):
            return False
        return all(e.all_values(predicate) for e in self.elements)

    def matches(self, other: "Pattern[V]") -> bool:
        """Return True if this pattern is structurally equal to other.

        Args:
            other: The pattern to compare against.

        Returns:
            True if self == other (deep structural equality).
        """
        return self == other

    def contains(self, needle: "Pattern[V]") -> bool:
        """Return True if needle appears anywhere in this pattern (including at root).

        Args:
            needle: The sub-pattern to search for.

        Returns:
            True if needle is equal to any subtree in this pattern.
        """
        return self == needle or any(e.contains(needle) for e in self.elements)

    def para(self, f: Callable[["Pattern[V]", list[R]], R]) -> R:
        """Structure-aware fold (paramorphism): f sees the sub-pattern and element results.

        Unlike ``fold``, ``f`` receives the entire current sub-pattern
        as its first argument alongside the already-computed results for
        each element, enabling transformations that need both the original
        structure and the accumulated sub-results. Processing is
        bottom-up (elements before the pattern containing them).

        Args:
            f: A function ``(sub_pattern, child_results) -> result``
                called for each node from leaves to root.

        Returns:
            The result of applying ``f`` at the root after all children
            have been processed.

        Example:
            >>> p = Pattern.from_list("root", ["a", "b"])
            >>> # Collect (value, child_count) pairs bottom-up
            >>> def annotate(sub, child_results):
            ...     return (sub.value, len(child_results))
            >>> p.para(annotate)
            ('root', 2)
        """
        return f(self, [e.para(f) for e in self.elements])

    def depth_at(self) -> "Pattern[int]":
        """Return a same-shape Pattern where each value is replaced by its nesting depth.

        Atomic patterns have nesting depth 0; each containing pattern's value is
        1 + the maximum nesting depth of its elements.
        """
        return self.extend(lambda sub: sub.depth)

    def size_at(self) -> "Pattern[int]":
        """Return a same-shape Pattern where each value is replaced by its composition size.

        Atomic patterns have size 1; each containing pattern's value is
        1 + the sum of its elements' sizes.
        """
        return self.extend(lambda sub: sub.size)

    def indices_at(self) -> "Pattern[list[int]]":
        """Return a same-shape Pattern where each value is its index path from the outermost position.

        The outermost pattern is annotated with ``[]``; its first element with ``[0]``,
        second element with ``[1]``; those elements' first elements with ``[0, 0]``,
        ``[1, 0]``, etc.

        Returns:
            A Pattern with the same composition shape where each position's value
            is the list of indices from the outermost pattern to that position.

        Example:
            >>> p = Pattern.from_list("outer", ["a", "b"])
            >>> indices = p.indices_at()
            >>> indices.value              # outermost position
            []
            >>> indices.elements[0].value  # first element
            [0]
            >>> indices.elements[1].value  # second element
            [1]
        """
        def go(pat: "Pattern[V]", path: list[int]) -> "Pattern[list[int]]":
            return Pattern(
                value=path,
                elements=[go(e, path + [i]) for i, e in enumerate(pat.elements)]
            )
        return go(self, [])

    def combine(self, other: "Pattern[V]", combine_values: Callable[[V, V], V]) -> "Pattern[V]":
        """Combine two patterns into one: merge roots and concatenate children.

        The root values of ``self`` and ``other`` are merged via
        ``combine_values``; the children of both patterns are
        concatenated (self's children first).

        Args:
            other: The pattern to merge with.
            combine_values: A function ``(self.value, other.value) ->
                merged_value`` used to produce the root value.

        Returns:
            A new Pattern whose value is ``combine_values(self.value,
            other.value)`` and whose elements are
            ``self.elements + other.elements``.

        Example:
            >>> a = Pattern.from_list("x", [1, 2])
            >>> b = Pattern.from_list("y", [3])
            >>> c = a.combine(b, lambda u, v: u + v)
            >>> c.value
            'xy'
            >>> len(c.elements)
            3
        """
        return Pattern(
            value=combine_values(self.value, other.value),
            elements=list(self.elements) + list(other.elements),
        )

    # --- Equality ---
    # @dataclass provides __eq__ that recursively compares fields.
    # For Pattern[Subject], Subject.__eq__ is also structural (dataclass default).

    def __iter__(self) -> Iterator["Pattern[V]"]:
        """Iterate over immediate child elements (shallow, not recursive)."""
        return iter(self.elements)


def unfold(expand: Callable[[A], tuple[V, list[A]]], seed: A) -> Pattern[V]:
    """Grow a Pattern tree from a seed value (module-level alias for Pattern.unfold).

    Args:
        expand: A function ``seed -> (value, child_seeds)`` applied
            recursively.  An empty ``child_seeds`` list terminates the
            branch.
        seed: The initial seed passed to ``expand``.

    Returns:
        A Pattern tree whose structure mirrors the recursive expansion
        of ``seed``.
    """
    return Pattern.unfold(expand, seed)
