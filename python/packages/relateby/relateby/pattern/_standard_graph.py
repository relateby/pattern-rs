"""Native Python StandardGraph.

Classifies Pattern[Subject] values into nodes, relationships, annotations,
walks, and other patterns using the same shape rules as the gram-hs reference.
"""
from __future__ import annotations

from typing import Iterator, Optional

from relateby.gram import parse_gram

from ._pattern import Pattern
from ._subject import Subject


class StandardGraph:
    """An immutable-style graph built by classifying ``Pattern[Subject]`` values.

    Patterns are classified into nodes, relationships, annotations,
    walks, and other according to the same shape rules used by the
    gram-hs reference implementation.  Once ingested, elements are
    accessible via identity-keyed lookups or typed iterators.

    Use the class methods ``from_patterns`` or ``from_gram`` to
    construct a graph rather than calling ``__init__`` directly.
    """

    def __init__(self) -> None:
        self._nodes: dict[str, Pattern[Subject]] = {}
        self._relationships: dict[str, dict[str, object]] = {}
        self._annotations: dict[str, Pattern[Subject]] = {}
        self._walks: dict[str, Pattern[Subject]] = {}
        self._other: dict[str, Pattern[Subject]] = {}

    @classmethod
    def from_patterns(cls, patterns: list[Pattern[Subject]]) -> "StandardGraph":
        """Build a StandardGraph from a list of ``Pattern[Subject]`` values.

        Each pattern is classified (node, relationship, annotation,
        walk, or other) and its components are recursively ingested.

        Args:
            patterns: The patterns to ingest.

        Returns:
            A new StandardGraph containing all classified elements.
        """
        graph = cls()
        for pattern in patterns:
            graph._ingest(pattern)
        return graph

    @classmethod
    def from_gram(cls, input: str) -> "StandardGraph":
        """Parse Gram notation and build a StandardGraph.

        Args:
            input: A string of Gram notation.

        Returns:
            A new StandardGraph built from the parsed patterns.
        """
        return cls.from_patterns(parse_gram(input))

    @property
    def node_count(self) -> int:
        """Number of node patterns currently stored in the graph."""
        return len(self._nodes)

    @property
    def relationship_count(self) -> int:
        """Number of relationship patterns currently stored in the graph."""
        return len(self._relationships)

    @property
    def annotation_count(self) -> int:
        """Number of annotation patterns currently stored in the graph."""
        return len(self._annotations)

    @property
    def walk_count(self) -> int:
        """Number of walk patterns currently stored in the graph."""
        return len(self._walks)

    @property
    def is_empty(self) -> bool:
        """True when the graph contains no classified or other elements."""
        return (
            self.node_count == 0
            and self.relationship_count == 0
            and self.annotation_count == 0
            and self.walk_count == 0
            and len(self._other) == 0
        )

    @property
    def has_conflicts(self) -> bool:
        """True when the graph contains conflicting elements (always False in this implementation)."""
        return False

    def nodes(self) -> Iterator[tuple[str, Pattern[Subject]]]:
        """Iterate over ``(identity, pattern)`` pairs for all node elements."""
        return iter(self._nodes.items())

    def relationships(self) -> Iterator[tuple[str, dict[str, object]]]:
        """Iterate over ``(identity, rel_data)`` pairs for all relationship elements.

        Each ``rel_data`` dict has keys ``"pattern"``, ``"source"``, and
        ``"target"``.
        """
        return iter(self._relationships.items())

    def annotations(self) -> Iterator[tuple[str, Pattern[Subject]]]:
        """Iterate over ``(identity, pattern)`` pairs for all annotation elements."""
        return iter(self._annotations.items())

    def walks(self) -> Iterator[tuple[str, Pattern[Subject]]]:
        """Iterate over ``(identity, pattern)`` pairs for all walk elements."""
        return iter(self._walks.items())

    def other(self) -> list[Pattern[Subject]]:
        """Return a list of patterns that did not match any standard classification."""
        return list(self._other.values())

    def node(self, id: str) -> Optional[Pattern[Subject]]:
        """Look up a node pattern by identity.

        Args:
            id: The identity of the node to retrieve.

        Returns:
            The matching node Pattern, or None if not found.
        """
        return self._nodes.get(id)

    def relationship(self, id: str) -> Optional[dict[str, object]]:
        """Look up relationship data by identity.

        Args:
            id: The identity of the relationship to retrieve.

        Returns:
            A dict with keys ``"pattern"``, ``"source"``, and
            ``"target"``, or None if not found.
        """
        return self._relationships.get(id)

    def annotation(self, id: str) -> Optional[Pattern[Subject]]:
        """Look up an annotation pattern by identity.

        Args:
            id: The identity of the annotation to retrieve.

        Returns:
            The matching annotation Pattern, or None if not found.
        """
        return self._annotations.get(id)

    def walk(self, id: str) -> Optional[Pattern[Subject]]:
        """Look up a walk pattern by identity.

        Args:
            id: The identity of the walk to retrieve.

        Returns:
            The matching walk Pattern, or None if not found.
        """
        return self._walks.get(id)

    def source(self, rel_id: str) -> Optional[Pattern[Subject]]:
        """Return the source node pattern of a relationship.

        Args:
            rel_id: The identity of the relationship.

        Returns:
            The source node Pattern, or None if the relationship or its
            source node is not found.
        """
        relationship = self.relationship(rel_id)
        if relationship is None:
            return None
        return self.node(str(relationship["source"]))

    def target(self, rel_id: str) -> Optional[Pattern[Subject]]:
        """Return the target node pattern of a relationship.

        Args:
            rel_id: The identity of the relationship.

        Returns:
            The target node Pattern, or None if the relationship or its
            target node is not found.
        """
        relationship = self.relationship(rel_id)
        if relationship is None:
            return None
        return self.node(str(relationship["target"]))

    def neighbors(self, node_id: str) -> list[Pattern[Subject]]:
        """Return all node patterns adjacent to the given node (undirected).

        A node is considered adjacent if it appears as the source or
        target of any relationship whose other endpoint is ``node_id``.

        Args:
            node_id: The identity of the node whose neighbors are wanted.

        Returns:
            A list of adjacent node Patterns (may be empty).
        """
        neighbors: list[Pattern[Subject]] = []
        for relationship in self._relationships.values():
            if relationship["source"] == node_id:
                neighbor = self.node(str(relationship["target"]))
                if neighbor is not None:
                    neighbors.append(neighbor)
            elif relationship["target"] == node_id:
                neighbor = self.node(str(relationship["source"]))
                if neighbor is not None:
                    neighbors.append(neighbor)
        return neighbors

    def degree(self, node_id: str) -> int:
        """Return the number of adjacent nodes (undirected degree).

        Args:
            node_id: The identity of the node.

        Returns:
            The count of distinct neighbors (incident relationship
            endpoints on the other side).
        """
        return len(self.neighbors(node_id))

    def _ingest(self, pattern: Pattern[Subject]) -> None:
        classification = classify_pattern(pattern)
        if classification == "node":
            self._nodes[pattern.value.identity] = pattern
        elif classification == "relationship":
            source_pattern = pattern.elements[0]
            target_pattern = pattern.elements[1]
            self._ingest(source_pattern)
            self._ingest(target_pattern)
            self._relationships[pattern.value.identity] = {
                "pattern": pattern,
                "source": source_pattern.value.identity,
                "target": target_pattern.value.identity,
            }
        elif classification == "walk":
            for element in pattern.elements:
                self._ingest(element)
            self._walks[pattern.value.identity] = pattern
        elif classification == "annotation":
            inner = pattern.elements[0]
            self._ingest(inner)
            self._annotations[pattern.value.identity] = pattern
        else:
            self._other[pattern.value.identity] = pattern


def classify_pattern(pattern: Pattern[Subject]) -> str:
    if len(pattern.elements) == 0:
        return "node"
    if len(pattern.elements) == 1:
        return "annotation"
    if len(pattern.elements) == 2 and all(is_node_like(element) for element in pattern.elements):
        return "relationship"
    if (
        len(pattern.elements) >= 1
        and all(is_relationship_like(element) for element in pattern.elements)
        and is_valid_walk(pattern.elements)
    ):
        return "walk"
    return "other"


def is_node_like(pattern: Pattern[Subject]) -> bool:
    return len(pattern.elements) == 0


def is_relationship_like(pattern: Pattern[Subject]) -> bool:
    return len(pattern.elements) == 2 and all(is_node_like(element) for element in pattern.elements)


def is_valid_walk(patterns: list[Pattern[Subject]]) -> bool:
    if not patterns:
        return False

    active: list[Pattern[Subject]] = []
    for pattern in patterns:
        if len(pattern.elements) != 2:
            return False

        left, right = pattern.elements
        if not active:
            active = [left, right]
            continue

        next_active: list[Pattern[Subject]] = []
        if any(candidate.value.identity == left.value.identity for candidate in active):
            next_active.append(right)
        if any(candidate.value.identity == right.value.identity for candidate in active):
            next_active.append(left)
        active = next_active

    return len(active) > 0
