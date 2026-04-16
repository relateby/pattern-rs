"""Tests for graph transform functions: map_graph, map_all_graph, filter_graph,
fold_graph, map_with_context, para_graph.
"""
from relateby.pattern import (
    Pattern,
    Subject,
    StandardGraph,
    map_graph,
    map_all_graph,
    filter_graph,
    fold_graph,
    map_with_context,
    para_graph,
)


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------


def make_node(identity: str, label: str | None = None) -> Pattern[Subject]:
    s = Subject.from_id(identity)
    if label:
        s = s.with_label(label)
    return Pattern.point(s)


def make_relationship(
    rel_id: str,
    source: Pattern[Subject],
    target: Pattern[Subject],
) -> Pattern[Subject]:
    return Pattern(value=Subject.from_id(rel_id), elements=[source, target])


def make_annotation(ann_id: str, inner: Pattern[Subject]) -> Pattern[Subject]:
    return Pattern(value=Subject.from_id(ann_id), elements=[inner])


def make_simple_graph() -> StandardGraph:
    """Graph: (n1)-[r1]->(n2)"""
    n1 = make_node("n1", "Person")
    n2 = make_node("n2", "Person")
    rel = make_relationship("r1", n1, n2)
    return StandardGraph.from_patterns([rel])


def make_walk_graph() -> StandardGraph:
    """Graph: (n1)-[r1]->(n2)-[r2]->(n3) as a walk w1."""
    n1 = make_node("n1")
    n2 = make_node("n2")
    n3 = make_node("n3")
    r1 = make_relationship("r1", n1, n2)
    r2 = make_relationship("r2", n2, n3)
    walk = Pattern(value=Subject.from_id("w1"), elements=[r1, r2])
    return StandardGraph.from_patterns([walk])


def make_annotation_graph() -> StandardGraph:
    """Graph: standalone n2 + annotation ann1 wrapping n1."""
    n1 = make_node("n1")
    n2 = make_node("n2")
    ann = make_annotation("ann1", n1)
    return StandardGraph.from_patterns([ann, n2])


# ---------------------------------------------------------------------------
# map_graph
# ---------------------------------------------------------------------------


def test_map_graph_transforms_nodes():
    graph = make_simple_graph()
    result = map_graph(graph, {
        "node": lambda p: Pattern.point(p.value.with_label("Visited"))
    })
    for _, n in result.nodes():
        assert "Visited" in n.value.labels


def test_map_graph_preserves_non_node_elements():
    graph = make_simple_graph()
    result = map_graph(graph, {
        "node": lambda p: Pattern.point(p.value.with_label("Tagged"))
    })
    # relationship count unchanged
    assert result.relationship_count == graph.relationship_count
    # relationship identity preserved
    assert result.relationship("r1") is not None


def test_map_graph_unspecified_class_passes_through():
    graph = make_simple_graph()
    result = map_graph(graph, {"node": lambda p: p})
    # No mapper for "relationship" → relationship unchanged
    r1 = result.relationship("r1")
    assert r1 is not None


def test_map_graph_empty_mappers_is_identity():
    graph = make_simple_graph()
    result = map_graph(graph, {})
    assert result.node_count == graph.node_count
    assert result.relationship_count == graph.relationship_count


def test_map_graph_transforms_relationship():
    graph = make_simple_graph()

    def tag_rel(p: Pattern[Subject]) -> Pattern[Subject]:
        return Pattern(value=p.value.with_label("TAGGED"), elements=list(p.elements))

    result = map_graph(graph, {"relationship": tag_rel})
    r1 = result.relationship("r1")
    assert r1 is not None
    assert "TAGGED" in r1["pattern"].value.labels  # type: ignore[index]


def test_map_graph_rebuilds_relationship_source_target_when_endpoints_change():
    """Relationship metadata must match pattern elements after mapping."""
    graph = make_simple_graph()

    def rewire_rel(p: Pattern[Subject]) -> Pattern[Subject]:
        left = make_node("src2")
        right = make_node("tgt2")
        return Pattern(value=p.value, elements=[left, right])

    result = map_graph(graph, {"relationship": rewire_rel})
    r1 = result.relationship("r1")
    assert r1 is not None
    assert str(r1["source"]) == "src2"
    assert str(r1["target"]) == "tgt2"
    src = result.source("r1")
    tgt = result.target("r1")
    assert src is not None and tgt is not None
    assert src.value.identity == "src2"
    assert tgt.value.identity == "tgt2"


# ---------------------------------------------------------------------------
# map_all_graph
# ---------------------------------------------------------------------------


def test_map_all_graph_transforms_all_elements():
    graph = make_simple_graph()

    def add_label(p: Pattern[Subject]) -> Pattern[Subject]:
        return Pattern(value=p.value.with_label("All"), elements=list(p.elements))

    result = map_all_graph(graph, add_label)
    for _, n in result.nodes():
        assert "All" in n.value.labels


def test_map_all_graph_identity_preserves_structure():
    graph = make_simple_graph()
    result = map_all_graph(graph, lambda p: p)
    assert result.node_count == graph.node_count
    assert result.relationship_count == graph.relationship_count


def test_map_all_graph_applies_to_annotations():
    graph = make_annotation_graph()

    def tag(p: Pattern[Subject]) -> Pattern[Subject]:
        return Pattern(value=p.value.with_label("X"), elements=list(p.elements))

    result = map_all_graph(graph, tag)
    ann = result.annotation("ann1")
    assert ann is not None
    assert "X" in ann.value.labels


# ---------------------------------------------------------------------------
# filter_graph
# ---------------------------------------------------------------------------


def test_filter_graph_removes_specified_nodes():
    graph = make_simple_graph()
    result = filter_graph(
        graph,
        lambda cls, p: p.value.identity != "n1",
        "delete_container",
    )
    assert result.node("n1") is None
    assert result.node("n2") is not None


def test_filter_graph_delete_container_removes_relationship_when_node_removed():
    graph = make_simple_graph()
    result = filter_graph(
        graph,
        lambda cls, p: p.value.identity != "n1",
        "delete_container",
    )
    # r1 depends on n1; delete_container removes it
    assert result.relationship("r1") is None


def test_filter_graph_splice_gap_also_removes_relationship():
    """For binary containers (relationships), splice_gap behaves like delete_container."""
    graph = make_simple_graph()
    result = filter_graph(
        graph,
        lambda cls, p: p.value.identity != "n1",
        "splice_gap",
    )
    assert result.relationship("r1") is None


def test_filter_graph_splice_gap_collapses_walk():
    """Removing a relationship from a walk with splice_gap keeps the walk but closes the gap."""
    graph = make_walk_graph()
    result = filter_graph(
        graph,
        lambda cls, p: p.value.identity != "r1",
        "splice_gap",
    )
    # Walk remains but with r1 spliced out
    assert result.walk_count == 1
    w1 = result.walk("w1")
    assert w1 is not None
    assert len(w1.elements) == 1
    assert w1.elements[0].value.identity == "r2"


def test_filter_graph_delete_container_removes_walk():
    """Removing a relationship from a walk with delete_container removes the whole walk."""
    graph = make_walk_graph()
    result = filter_graph(
        graph,
        lambda cls, p: p.value.identity != "r1",
        "delete_container",
    )
    assert result.walk_count == 0


def test_filter_graph_splice_gap_collapses_annotation():
    """Removing an annotation's child with splice_gap keeps the annotation with empty elements."""
    graph = make_annotation_graph()
    result = filter_graph(
        graph,
        lambda cls, p: p.value.identity != "n1",
        "splice_gap",
    )
    assert result.annotation_count == 1
    ann = result.annotation("ann1")
    assert ann is not None
    assert len(ann.elements) == 0


def test_filter_graph_delete_container_removes_annotation():
    graph = make_annotation_graph()
    result = filter_graph(
        graph,
        lambda cls, p: p.value.identity != "n1",
        "delete_container",
    )
    assert result.annotation_count == 0
    assert result.node("n2") is not None


def test_filter_graph_replace_with_surrogate_in_annotation():
    graph = make_annotation_graph()
    surrogate = make_node("REMOVED")
    result = filter_graph(
        graph,
        lambda cls, p: p.value.identity != "n1",
        ("replace_with_surrogate", surrogate),
    )
    # Annotation remains; n1 replaced by surrogate
    assert result.annotation_count == 1
    ann = result.annotation("ann1")
    assert ann is not None
    assert ann.elements[0].value.identity == "REMOVED"


def test_filter_graph_replace_with_surrogate_in_relationship_updates_endpoints():
    graph = make_simple_graph()
    surrogate = make_node("REMOVED")
    result = filter_graph(
        graph,
        lambda cls, p: p.value.identity != "n1",
        ("replace_with_surrogate", surrogate),
    )
    assert result.relationship_count == 1
    r1 = result.relationship("r1")
    assert r1 is not None
    assert str(r1["source"]) == "REMOVED"
    assert str(r1["target"]) == "n2"
    src = result.source("r1")
    assert src is not None
    assert src.value.identity == "REMOVED"


def test_filter_graph_keeps_all_when_predicate_always_true():
    graph = make_simple_graph()
    result = filter_graph(graph, lambda cls, p: True, "delete_container")
    assert result.node_count == graph.node_count
    assert result.relationship_count == graph.relationship_count


def test_filter_graph_removes_all_nodes_when_predicate_always_false():
    graph = make_simple_graph()
    result = filter_graph(graph, lambda cls, p: False, "delete_container")
    assert result.node_count == 0
    assert result.relationship_count == 0


# ---------------------------------------------------------------------------
# fold_graph
# ---------------------------------------------------------------------------


def test_fold_graph_counts_all_elements():
    graph = make_simple_graph()
    count = fold_graph(graph, lambda cls, p: 1, 0, lambda a, b: a + b)
    # 2 nodes + 1 relationship = 3
    assert count == 3


def test_fold_graph_reduces_to_single_value():
    graph = make_simple_graph()
    # Concatenate all identities
    ids = fold_graph(
        graph,
        lambda cls, p: [p.value.identity],
        [],
        lambda a, b: a + b,
    )
    assert sorted(ids) == ["n1", "n2", "r1"]


def test_fold_graph_collects_class_names():
    graph = make_simple_graph()
    classes = fold_graph(
        graph,
        lambda cls, p: {cls},
        set(),
        lambda a, b: a | b,
    )
    assert "node" in classes
    assert "relationship" in classes


def test_fold_graph_empty_graph_returns_empty():
    graph = StandardGraph()
    result = fold_graph(graph, lambda cls, p: 1, 0, lambda a, b: a + b)
    assert result == 0


def test_fold_graph_includes_annotations():
    graph = make_annotation_graph()
    classes = fold_graph(
        graph,
        lambda cls, p: {cls},
        set(),
        lambda a, b: a | b,
    )
    assert "node" in classes
    assert "annotation" in classes


# ---------------------------------------------------------------------------
# map_with_context
# ---------------------------------------------------------------------------


def test_map_with_context_provides_query_access():
    graph = make_simple_graph()
    queries_seen = []

    def f(query, p):
        queries_seen.append(query)
        return p

    map_with_context(graph, f)
    assert len(queries_seen) > 0


def test_map_with_context_snapshot_is_frozen():
    """All f calls see the original graph state (snapshot does not update mid-transform)."""
    graph = make_simple_graph()
    node_counts_seen = []

    def f(query, p):
        node_counts_seen.append(len(query.nodes()))
        return p

    map_with_context(graph, f)
    # Every call to f should see the original 2 nodes
    assert all(c == 2 for c in node_counts_seen)


def test_map_with_context_query_returns_nodes():
    graph = make_simple_graph()
    all_query_nodes: list[list] = []

    def f(query, p):
        all_query_nodes.append(query.nodes())
        return p

    map_with_context(graph, f)
    # query.nodes() should always return the 2 original nodes
    node_ids = {n.value.identity for nodes in all_query_nodes for n in nodes}
    assert "n1" in node_ids
    assert "n2" in node_ids


def test_map_with_context_identity_transform_preserves_structure():
    graph = make_simple_graph()
    result = map_with_context(graph, lambda q, p: p)
    assert result.node_count == graph.node_count
    assert result.relationship_count == graph.relationship_count


def test_map_with_context_transforms_nodes():
    graph = make_simple_graph()
    result = map_with_context(
        graph,
        lambda q, p: Pattern(value=p.value.with_label("CTX"), elements=list(p.elements)),
    )
    for _, n in result.nodes():
        assert "CTX" in n.value.labels


# ---------------------------------------------------------------------------
# para_graph
# ---------------------------------------------------------------------------


def test_para_graph_leaf_nodes_get_empty_sub_results():
    n1 = make_node("n1")
    graph = StandardGraph.from_patterns([n1])
    received: dict[str, dict] = {}

    def record(query, p, sub_results):
        received[p.value.identity] = dict(sub_results)
        return len(sub_results)

    para_graph(graph, record)
    assert received["n1"] == {}


def test_para_graph_processes_nodes_before_relationships():
    graph = make_simple_graph()
    visit_order: list[str] = []

    def record(query, p, sub_results):
        visit_order.append(p.value.identity)
        return p.value.identity

    para_graph(graph, record)
    assert visit_order.index("n1") < visit_order.index("r1")
    assert visit_order.index("n2") < visit_order.index("r1")


def test_para_graph_relationship_receives_node_sub_results():
    graph = make_simple_graph()

    def count_with_deps(query, p, sub_results):
        return 1 + len(sub_results)

    result = para_graph(graph, count_with_deps)
    # Nodes have no sub_results → 1
    assert result["n1"] == 1
    assert result["n2"] == 1
    # r1 has n1 and n2 as elements → 1 + 2 = 3
    assert result["r1"] == 3


def test_para_graph_returns_result_for_all_elements():
    graph = make_simple_graph()
    result = para_graph(graph, lambda q, p, sr: p.value.identity)
    assert result.keys() == {"n1", "n2", "r1"}


def test_para_graph_on_empty_graph_returns_empty_dict():
    graph = StandardGraph()
    result = para_graph(graph, lambda q, p, sr: 0)
    assert result == {}


def test_para_graph_walk_graph_processes_in_topo_order():
    graph = make_walk_graph()
    visit_order: list[str] = []

    def record(query, p, sub_results):
        visit_order.append(p.value.identity)
        return p.value.identity

    para_graph(graph, record)
    # Nodes before relationships, relationships before walk
    r1_idx = visit_order.index("r1")
    r2_idx = visit_order.index("r2")
    w1_idx = visit_order.index("w1")
    assert visit_order.index("n1") < r1_idx
    assert visit_order.index("n2") < r1_idx
    assert visit_order.index("n3") < r2_idx
    assert r1_idx < w1_idx
    assert r2_idx < w1_idx
