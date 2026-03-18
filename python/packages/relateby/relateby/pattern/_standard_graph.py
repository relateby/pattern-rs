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
    def __init__(self) -> None:
        self._nodes: dict[str, Pattern[Subject]] = {}
        self._relationships: dict[str, dict[str, object]] = {}
        self._annotations: dict[str, Pattern[Subject]] = {}
        self._walks: dict[str, Pattern[Subject]] = {}
        self._other: dict[str, Pattern[Subject]] = {}

    @classmethod
    def from_patterns(cls, patterns: list[Pattern[Subject]]) -> "StandardGraph":
        graph = cls()
        for pattern in patterns:
            graph._ingest(pattern)
        return graph

    @classmethod
    def from_gram(cls, input: str) -> "StandardGraph":
        return cls.from_patterns(parse_gram(input))

    @property
    def node_count(self) -> int:
        return len(self._nodes)

    @property
    def relationship_count(self) -> int:
        return len(self._relationships)

    @property
    def annotation_count(self) -> int:
        return len(self._annotations)

    @property
    def walk_count(self) -> int:
        return len(self._walks)

    @property
    def is_empty(self) -> bool:
        return (
            self.node_count == 0
            and self.relationship_count == 0
            and self.annotation_count == 0
            and self.walk_count == 0
            and len(self._other) == 0
        )

    @property
    def has_conflicts(self) -> bool:
        return False

    def nodes(self) -> Iterator[tuple[str, Pattern[Subject]]]:
        return iter(self._nodes.items())

    def relationships(self) -> Iterator[tuple[str, dict[str, object]]]:
        return iter(self._relationships.items())

    def annotations(self) -> Iterator[tuple[str, Pattern[Subject]]]:
        return iter(self._annotations.items())

    def walks(self) -> Iterator[tuple[str, Pattern[Subject]]]:
        return iter(self._walks.items())

    def other(self) -> list[Pattern[Subject]]:
        return list(self._other.values())

    def node(self, id: str) -> Optional[Pattern[Subject]]:
        return self._nodes.get(id)

    def relationship(self, id: str) -> Optional[dict[str, object]]:
        return self._relationships.get(id)

    def annotation(self, id: str) -> Optional[Pattern[Subject]]:
        return self._annotations.get(id)

    def walk(self, id: str) -> Optional[Pattern[Subject]]:
        return self._walks.get(id)

    def source(self, rel_id: str) -> Optional[Pattern[Subject]]:
        relationship = self.relationship(rel_id)
        if relationship is None:
            return None
        return self.node(str(relationship["source"]))

    def target(self, rel_id: str) -> Optional[Pattern[Subject]]:
        relationship = self.relationship(rel_id)
        if relationship is None:
            return None
        return self.node(str(relationship["target"]))

    def neighbors(self, node_id: str) -> list[Pattern[Subject]]:
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
