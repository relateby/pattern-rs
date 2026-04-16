"""Graph transform functions for StandardGraph.

Standalone functions that transform StandardGraph instances, mirroring
the TypeScript graph/transforms.ts functions.
"""
from __future__ import annotations

from typing import Callable, Optional, TypeVar, Union

from ._pattern import Pattern
from ._subject import Subject
from ._standard_graph import StandardGraph

R = TypeVar("R")

# Substitution strategy for filter_graph:
# - "delete_container": remove the entire container when any child is removed
# - "splice_gap": remove the child, keep the container with remaining children
# - ("replace_with_surrogate", surrogate): replace removed child with surrogate pattern
Substitution = Union[str, tuple[str, "Pattern[Subject]"]]


class GraphQuery:
    """Frozen snapshot of StandardGraph query state.

    Created once at the start of a transform; does not reflect mutations
    made during the transform (snapshot semantics).
    """

    def __init__(self, graph: StandardGraph) -> None:
        self._nodes: dict[str, Pattern[Subject]] = dict(graph._nodes)
        self._rels: dict[str, dict[str, object]] = {
            k: dict(v) for k, v in graph._relationships.items()
        }

    def nodes(self) -> list[Pattern[Subject]]:
        return list(self._nodes.values())

    def relationships(self) -> list[Pattern[Subject]]:
        return [r["pattern"] for r in self._rels.values()]  # type: ignore[misc]

    def source(self, rel: Pattern[Subject]) -> Optional[Pattern[Subject]]:
        r = self._rels.get(rel.value.identity)
        if r is None:
            return None
        return self._nodes.get(str(r["source"]))

    def target(self, rel: Pattern[Subject]) -> Optional[Pattern[Subject]]:
        r = self._rels.get(rel.value.identity)
        if r is None:
            return None
        return self._nodes.get(str(r["target"]))

    def incident_rels(self, node: Pattern[Subject]) -> list[Pattern[Subject]]:
        node_id = node.value.identity
        return [
            r["pattern"]  # type: ignore[misc]
            for r in self._rels.values()
            if r["source"] == node_id or r["target"] == node_id
        ]

    def degree(self, node: Pattern[Subject]) -> int:
        return len(self.incident_rels(node))

    def node_by_id(self, identity: str) -> Optional[Pattern[Subject]]:
        return self._nodes.get(identity)

    def relationship_by_id(self, identity: str) -> Optional[Pattern[Subject]]:
        r = self._rels.get(identity)
        if r is None:
            return None
        return r["pattern"]  # type: ignore[return-value]


def _make_graph(
    nodes: dict[str, Pattern[Subject]],
    relationships: dict[str, dict[str, object]],
    annotations: dict[str, Pattern[Subject]],
    walks: dict[str, Pattern[Subject]],
    other: dict[str, Pattern[Subject]],
) -> StandardGraph:
    """Construct a StandardGraph directly from pre-classified element dicts."""
    g = StandardGraph()
    g._nodes = nodes
    g._relationships = relationships
    g._annotations = annotations
    g._walks = walks
    g._other = other
    return g


def _topological_order(graph: StandardGraph) -> list[Pattern[Subject]]:
    """Return graph elements in bottom-up topological order.

    Nodes first (leaves), then relationships (depend on nodes),
    then annotations, walks, and other last.
    """
    result: list[Pattern[Subject]] = []
    result.extend(graph._nodes.values())
    for rel_data in graph._relationships.values():
        result.append(rel_data["pattern"])  # type: ignore[arg-type]
    result.extend(graph._annotations.values())
    result.extend(graph._walks.values())
    result.extend(graph._other.values())
    return result


def map_graph(
    graph: StandardGraph,
    mappers: dict[str, Callable[[Pattern[Subject]], Pattern[Subject]]],
) -> StandardGraph:
    """Transform each element by its graph class.

    mappers keys: "node", "relationship", "annotation", "walk", "other".
    Elements whose class has no mapper pass through unchanged.
    """
    node_fn = mappers.get("node")
    rel_fn = mappers.get("relationship")
    ann_fn = mappers.get("annotation")
    walk_fn = mappers.get("walk")
    other_fn = mappers.get("other")

    nodes = {k: node_fn(v) if node_fn else v for k, v in graph._nodes.items()}

    rels: dict[str, dict[str, object]] = {}
    for k, rel_data in graph._relationships.items():
        p: Pattern[Subject] = rel_data["pattern"]  # type: ignore[assignment]
        new_p = rel_fn(p) if rel_fn else p
        new_data = dict(rel_data)
        new_data["pattern"] = new_p
        rels[k] = new_data

    annotations = {k: ann_fn(v) if ann_fn else v for k, v in graph._annotations.items()}
    walks = {k: walk_fn(v) if walk_fn else v for k, v in graph._walks.items()}
    other = {k: other_fn(v) if other_fn else v for k, v in graph._other.items()}

    return _make_graph(nodes, rels, annotations, walks, other)


def map_all_graph(
    graph: StandardGraph,
    f: Callable[[Pattern[Subject]], Pattern[Subject]],
) -> StandardGraph:
    """Apply the same transformation to all elements regardless of class."""
    return map_graph(graph, {
        "node": f, "relationship": f, "annotation": f, "walk": f, "other": f,
    })


def filter_graph(
    graph: StandardGraph,
    predicate: Callable[[str, Pattern[Subject]], bool],
    substitution: Substitution,
) -> StandardGraph:
    """Remove elements where predicate(class_name, pattern) returns False.

    substitution controls what happens to containers whose children are removed:
    - "delete_container": remove the entire container
    - "splice_gap": remove the child, keep the container with remaining children
    - ("replace_with_surrogate", surrogate): replace removed child with surrogate
    """
    removed: set[str] = set()
    for node_id, p in graph._nodes.items():
        if not predicate("node", p):
            removed.add(node_id)
    for rel_id, rel_data in graph._relationships.items():
        p = rel_data["pattern"]  # type: ignore[assignment]
        if not predicate("relationship", p):
            removed.add(rel_id)
    for ann_id, p in graph._annotations.items():
        if not predicate("annotation", p):
            removed.add(ann_id)
    for walk_id, p in graph._walks.items():
        if not predicate("walk", p):
            removed.add(walk_id)
    for other_id, p in graph._other.items():
        if not predicate("other", p):
            removed.add(other_id)

    nodes = {k: v for k, v in graph._nodes.items() if k not in removed}

    rels: dict[str, dict[str, object]] = {}
    for rel_id, rel_data in graph._relationships.items():
        if rel_id in removed:
            continue
        source_id = str(rel_data["source"])
        target_id = str(rel_data["target"])
        if source_id in removed or target_id in removed:
            if isinstance(substitution, tuple) and substitution[0] == "replace_with_surrogate":
                surrogate: Pattern[Subject] = substitution[1]  # type: ignore[assignment]
                p = rel_data["pattern"]  # type: ignore[assignment]
                new_els = [surrogate if e.value.identity in removed else e for e in p.elements]
                new_data = dict(rel_data)
                new_data["pattern"] = Pattern(value=p.value, elements=new_els)
                rels[rel_id] = new_data
            # "delete_container" and "splice_gap": skip relationship (binary container cannot gap)
        else:
            rels[rel_id] = rel_data

    annotations: dict[str, Pattern[Subject]] = {}
    for ann_id, p in graph._annotations.items():
        if ann_id in removed:
            continue
        if any(e.value.identity in removed for e in p.elements):
            if isinstance(substitution, tuple) and substitution[0] == "replace_with_surrogate":
                surrogate = substitution[1]  # type: ignore[assignment]
                new_els = [surrogate if e.value.identity in removed else e for e in p.elements]
                annotations[ann_id] = Pattern(value=p.value, elements=new_els)
            elif substitution == "delete_container":
                pass  # container removed
            elif substitution == "splice_gap":
                remaining = [e for e in p.elements if e.value.identity not in removed]
                annotations[ann_id] = Pattern(value=p.value, elements=remaining)
        else:
            annotations[ann_id] = p

    walks: dict[str, Pattern[Subject]] = {}
    for walk_id, p in graph._walks.items():
        if walk_id in removed:
            continue
        if any(e.value.identity in removed for e in p.elements):
            if isinstance(substitution, tuple) and substitution[0] == "replace_with_surrogate":
                surrogate = substitution[1]  # type: ignore[assignment]
                new_els = [surrogate if e.value.identity in removed else e for e in p.elements]
                walks[walk_id] = Pattern(value=p.value, elements=new_els)
            elif substitution == "delete_container":
                pass  # container removed
            elif substitution == "splice_gap":
                remaining = [e for e in p.elements if e.value.identity not in removed]
                walks[walk_id] = Pattern(value=p.value, elements=remaining)
        else:
            walks[walk_id] = p

    other = {k: v for k, v in graph._other.items() if k not in removed}

    return _make_graph(nodes, rels, annotations, walks, other)


def fold_graph(
    graph: StandardGraph,
    f: Callable[[str, Pattern[Subject]], R],
    empty: R,
    combine: Callable[[R, R], R],
) -> R:
    """Reduce all classified elements to a single value.

    f receives (class_name, pattern) for each element.
    Results are combined left-to-right with combine.
    """
    acc = empty
    for p in graph._nodes.values():
        acc = combine(acc, f("node", p))
    for rel_data in graph._relationships.values():
        p = rel_data["pattern"]  # type: ignore[assignment]
        acc = combine(acc, f("relationship", p))
    for p in graph._annotations.values():
        acc = combine(acc, f("annotation", p))
    for p in graph._walks.values():
        acc = combine(acc, f("walk", p))
    for p in graph._other.values():
        acc = combine(acc, f("other", p))
    return acc


def map_with_context(
    graph: StandardGraph,
    f: Callable[[GraphQuery, Pattern[Subject]], Pattern[Subject]],
) -> StandardGraph:
    """Transform each element with access to a frozen GraphQuery snapshot.

    The snapshot is captured at transform start and is not updated
    as elements are transformed.
    """
    query = GraphQuery(graph)

    nodes = {k: f(query, v) for k, v in graph._nodes.items()}

    rels: dict[str, dict[str, object]] = {}
    for k, rel_data in graph._relationships.items():
        p: Pattern[Subject] = rel_data["pattern"]  # type: ignore[assignment]
        new_data = dict(rel_data)
        new_data["pattern"] = f(query, p)
        rels[k] = new_data

    annotations = {k: f(query, v) for k, v in graph._annotations.items()}
    walks = {k: f(query, v) for k, v in graph._walks.items()}
    other = {k: f(query, v) for k, v in graph._other.items()}

    return _make_graph(nodes, rels, annotations, walks, other)


def para_graph(
    graph: StandardGraph,
    f: Callable[[GraphQuery, Pattern[Subject], dict[str, R]], R],
) -> dict[str, R]:
    """Bottom-up fold over all graph elements in topological order.

    f receives (query, pattern, sub_results) where sub_results maps
    the identity of already-processed child elements to their results.
    Returns dict mapping each element identity to its fold result.
    """
    query = GraphQuery(graph)
    results: dict[str, R] = {}

    for p in _topological_order(graph):
        identity = p.value.identity
        sub_results: dict[str, R] = {
            e.value.identity: results[e.value.identity]
            for e in p.elements
            if e.value.identity in results
        }
        results[identity] = f(query, p, sub_results)

    return results
