import importlib

import pytest

from relateby.pattern import Pattern, StandardGraph, Subject

try:
    importlib.import_module("relateby._native.gram_codec")
except ModuleNotFoundError:
    HAS_NATIVE_GRAM = False
else:
    HAS_NATIVE_GRAM = True


def node(identity: str) -> Pattern[Subject]:
    return Pattern.point(Subject.from_id(identity))


def relationship(identity: str, source: str, target: str) -> Pattern[Subject]:
    return Pattern(
        value=Subject.from_id(identity),
        elements=[node(source), node(target)],
    )


def test_from_patterns_classifies_nodes_and_relationships():
    graph = StandardGraph.from_patterns([relationship("r1", "alice", "bob")])

    assert graph.node_count == 2
    assert graph.relationship_count == 1
    assert graph.node("alice") is not None
    relationship_entry = graph.relationship("r1")
    assert relationship_entry is not None
    assert relationship_entry["source"] == "alice"
    assert relationship_entry["target"] == "bob"


def test_from_patterns_classifies_annotations_walks_and_other():
    rel1 = relationship("r1", "a", "b")
    rel2 = relationship("r2", "b", "c")
    annotation = Pattern(value=Subject.from_id("ann1"), elements=[node("annotated")])
    walk = Pattern(value=Subject.from_id("walk1"), elements=[rel1, rel2])
    other = Pattern(
        value=Subject.from_id("other1"),
        elements=[node("x"), node("y"), node("z")],
    )

    graph = StandardGraph.from_patterns([annotation, walk, other])

    assert graph.annotation_count == 1
    assert graph.walk_count == 1
    assert graph.relationship_count == 2
    assert graph.node_count == 4
    assert [pattern.value.identity for pattern in graph.other()] == ["other1"]


def test_lookup_helpers_return_expected_values():
    graph = StandardGraph.from_patterns([relationship("r1", "alice", "bob")])

    assert graph.source("r1") is not None
    assert graph.source("r1").value.identity == "alice"
    assert graph.target("r1") is not None
    assert graph.target("r1").value.identity == "bob"
    assert [pattern.value.identity for pattern in graph.neighbors("alice")] == ["bob"]
    assert graph.degree("alice") == 1
    assert graph.node("missing") is None
    assert graph.relationship("missing") is None


@pytest.mark.skipif(not HAS_NATIVE_GRAM, reason="native gram codec not available")
def test_from_gram_composes_parse_and_classify():
    graph = StandardGraph.from_gram("(a:Person)-->(b:Person)")

    assert graph.node_count == 2
    assert graph.relationship_count == 1
    assert graph.node("a") is not None
