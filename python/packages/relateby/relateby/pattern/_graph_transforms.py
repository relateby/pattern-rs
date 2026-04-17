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
    """Frozen snapshot of StandardGraph query state for use inside transforms.

    Created once at the start of a transform function; it captures the
    nodes and relationships as they existed at that moment and does not
    reflect any mutations made during the transform (snapshot semantics).
    This allows transform functions to safely query graph topology while
    building the output.

    Attributes:
        _nodes: Snapshot of node patterns keyed by identity.
        _rels: Snapshot of relationship data dicts keyed by identity.
    """

    def __init__(self, graph: StandardGraph) -> None:
        self._nodes: dict[str, Pattern[Subject]] = dict(graph._nodes)
        self._rels: dict[str, dict[str, object]] = {
            k: dict(v) for k, v in graph._relationships.items()
        }

    def nodes(self) -> list[Pattern[Subject]]:
        """Return all node patterns captured in this snapshot.

        Returns:
            A list of node Patterns.
        """
        return list(self._nodes.values())

    def relationships(self) -> list[Pattern[Subject]]:
        """Return all relationship patterns captured in this snapshot.

        Returns:
            A list of relationship Patterns.
        """
        return [r["pattern"] for r in self._rels.values()]  # type: ignore[misc]

    def source(self, rel: Pattern[Subject]) -> Optional[Pattern[Subject]]:
        """Return the source node pattern for a relationship pattern.

        Args:
            rel: A relationship Pattern whose identity is used for
                the lookup.

        Returns:
            The source node Pattern, or None if the relationship or its
            source node is not found in the snapshot.
        """
        r = self._rels.get(rel.value.identity)
        if r is None:
            return None
        return self._nodes.get(str(r["source"]))

    def target(self, rel: Pattern[Subject]) -> Optional[Pattern[Subject]]:
        """Return the target node pattern for a relationship pattern.

        Args:
            rel: A relationship Pattern whose identity is used for
                the lookup.

        Returns:
            The target node Pattern, or None if the relationship or its
            target node is not found in the snapshot.
        """
        r = self._rels.get(rel.value.identity)
        if r is None:
            return None
        return self._nodes.get(str(r["target"]))

    def incident_rels(self, node: Pattern[Subject]) -> list[Pattern[Subject]]:
        """Return all relationship patterns that include the given node as source or target.

        Args:
            node: A node Pattern whose identity is used for the lookup.

        Returns:
            A list of incident relationship Patterns.
        """
        node_id = node.value.identity
        return [
            r["pattern"]  # type: ignore[misc]
            for r in self._rels.values()
            if r["source"] == node_id or r["target"] == node_id
        ]

    def degree(self, node: Pattern[Subject]) -> int:
        """Return the number of relationships incident to the given node.

        Args:
            node: A node Pattern whose identity is used for the lookup.

        Returns:
            The count of incident relationships (undirected degree).
        """
        return len(self.incident_rels(node))

    def node_by_id(self, identity: str) -> Optional[Pattern[Subject]]:
        """Look up a node pattern by its string identity.

        Args:
            identity: The identity of the node to retrieve.

        Returns:
            The matching node Pattern, or None if not found.
        """
        return self._nodes.get(identity)

    def relationship_by_id(self, identity: str) -> Optional[Pattern[Subject]]:
        """Look up a relationship pattern by its string identity.

        Args:
            identity: The identity of the relationship to retrieve.

        Returns:
            The matching relationship Pattern, or None if not found.
        """
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

    Args:
        graph: The source graph whose elements will be transformed.
        mappers: A dict whose keys are graph-class names (``"node"``,
            ``"relationship"``, ``"annotation"``, ``"walk"``,
            ``"other"``) and whose values are transformation functions
            ``Pattern[Subject] -> Pattern[Subject]``.  Elements whose
            class has no entry in ``mappers`` pass through unchanged.

    Returns:
        A new StandardGraph with transformed elements.
    """
    node_fn = mappers.get("node")
    rel_fn = mappers.get("relationship")
    ann_fn = mappers.get("annotation")
    walk_fn = mappers.get("walk")
    other_fn = mappers.get("other")

    # Rebuild via from_patterns so relationship source/target metadata and dict
    # keys stay consistent with transformed pattern shapes (identities, endpoints).
    # Ingest containers (relationships, annotations, walks, other) before top-level
    # nodes so embedded endpoint copies do not overwrite mapped node patterns.
    patterns: list[Pattern[Subject]] = []

    for rel_data in graph._relationships.values():
        p: Pattern[Subject] = rel_data["pattern"]  # type: ignore[assignment]
        patterns.append(rel_fn(p) if rel_fn else p)

    patterns.extend(ann_fn(v) if ann_fn else v for v in graph._annotations.values())
    patterns.extend(walk_fn(v) if walk_fn else v for v in graph._walks.values())
    patterns.extend(other_fn(v) if other_fn else v for v in graph._other.values())

    patterns.extend(node_fn(v) if node_fn else v for v in graph._nodes.values())

    return StandardGraph.from_patterns(patterns)


def map_all_graph(
    graph: StandardGraph,
    f: Callable[[Pattern[Subject]], Pattern[Subject]],
) -> StandardGraph:
    """Apply the same transformation to every element regardless of its class.

    Args:
        graph: The source graph.
        f: A function ``Pattern[Subject] -> Pattern[Subject]`` applied
            uniformly to every element.

    Returns:
        A new StandardGraph with every element replaced by ``f(element)``.
    """
    return map_graph(graph, {
        "node": f, "relationship": f, "annotation": f, "walk": f, "other": f,
    })


def filter_graph(
    graph: StandardGraph,
    predicate: Callable[[str, Pattern[Subject]], bool],
    substitution: Substitution,
) -> StandardGraph:
    """Remove elements where ``predicate(class_name, pattern)`` returns False.

    When a removed element is referenced by a container (e.g. a node
    that is the source of a relationship), the ``substitution`` strategy
    determines how to handle the container:

    - ``"delete_container"``: remove the container entirely.
    - ``"splice_gap"``: drop the removed child but keep the container
      with its remaining children.
    - ``("replace_with_surrogate", surrogate)``: replace the removed
      child with the provided ``surrogate`` Pattern.

    Args:
        graph: The source graph.
        predicate: A function ``(class_name, pattern) -> bool`` where
            ``class_name`` is one of ``"node"``, ``"relationship"``,
            ``"annotation"``, ``"walk"``, or ``"other"``.  Return False
            to remove the element.
        substitution: Strategy for handling containers whose children
            are removed.  One of the three forms described above.

    Returns:
        A new StandardGraph with failing elements removed or replaced
        according to ``substitution``.
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
                new_data["source"] = new_els[0].value.identity
                new_data["target"] = new_els[1].value.identity
                for ep in new_els:
                    if len(ep.elements) == 0:
                        nodes.setdefault(ep.value.identity, ep)
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

    Iterates over nodes, relationships, annotations, walks, and other
    elements in that order, accumulating results left-to-right.

    Args:
        graph: The source graph.
        f: A function ``(class_name, pattern) -> R`` applied to each
            element, where ``class_name`` is one of ``"node"``,
            ``"relationship"``, ``"annotation"``, ``"walk"``, or
            ``"other"``.
        empty: The initial accumulator value (returned as-is when the
            graph is empty).
        combine: A function ``(accumulator, element_result) ->
            new_accumulator`` used to fold results left-to-right.

    Returns:
        The final accumulated value after visiting every element.
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

    A ``GraphQuery`` snapshot is captured once before any transformation
    begins and passed to ``f`` for every element.  Because the snapshot
    is immutable, queries inside ``f`` always reflect the original graph
    topology, not the intermediate results being built.

    Args:
        graph: The source graph.
        f: A function ``(query, pattern) -> Pattern[Subject]`` applied
            to every element.  ``query`` is the frozen snapshot.

    Returns:
        A new StandardGraph with every element replaced by
        ``f(query, element)``.
    """
    query = GraphQuery(graph)

    patterns: list[Pattern[Subject]] = []
    for rel_data in graph._relationships.values():
        p: Pattern[Subject] = rel_data["pattern"]  # type: ignore[assignment]
        patterns.append(f(query, p))

    patterns.extend(f(query, v) for v in graph._annotations.values())
    patterns.extend(f(query, v) for v in graph._walks.values())
    patterns.extend(f(query, v) for v in graph._other.values())
    patterns.extend(f(query, v) for v in graph._nodes.values())

    return StandardGraph.from_patterns(patterns)


def para_graph(
    graph: StandardGraph,
    f: Callable[[GraphQuery, Pattern[Subject], dict[str, R]], R],
) -> dict[str, R]:
    """Bottom-up paramorphism over all graph elements in topological order.

    Elements are visited leaves-first (nodes, then relationships, then
    annotations, walks, and other).  When ``f`` is called for an
    element, the results for all of its direct children are already
    available in ``sub_results``, enabling fold functions that combine
    child results with the current element's structure.

    Args:
        graph: The source graph.
        f: A function ``(query, pattern, sub_results) -> R`` where:

            - ``query`` is a frozen ``GraphQuery`` snapshot.
            - ``pattern`` is the current element Pattern.
            - ``sub_results`` is a ``dict[str, R]`` mapping the
              identities of already-processed direct children to their
              fold results.

    Returns:
        A ``dict[str, R]`` mapping every element's identity to its
        computed fold result.

    Example:
        >>> # Count the total number of nodes reachable from each element
        >>> from relateby.pattern import StandardGraph
        >>> from relateby.pattern._graph_transforms import para_graph
        >>> g = StandardGraph.from_gram("(a)-[r]->(b)")
        >>> def count_nodes(query, pat, sub):
        ...     own = 1 if query.node_by_id(pat.value.identity) else 0
        ...     return own + sum(sub.values())
        >>> results = para_graph(g, count_nodes)
        >>> results["r"]  # relationship sees both endpoint results
        2
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
