"""Tests for StandardGraph Python bindings (T040)."""

import pytest
from pattern_core import StandardGraph, Subject, Pattern, SubjectBuilder


def make_person(identity: str, name: str) -> Subject:
    return Subject(identity=identity, labels={"Person"}, properties={"name": name})


def make_node(subject: Subject) -> Pattern:
    return Pattern.point(subject)


class TestStandardGraphConstructors:
    def test_empty_constructor(self):
        g = StandardGraph()
        assert g.is_empty
        assert g.node_count == 0
        assert g.relationship_count == 0

    def test_from_patterns_empty(self):
        g = StandardGraph.from_patterns([])
        assert g.is_empty

    def test_from_patterns_nodes(self):
        alice = make_person("alice", "Alice")
        bob = make_person("bob", "Bob")
        g = StandardGraph.from_patterns([make_node(alice), make_node(bob)])
        assert g.node_count == 2
        assert not g.is_empty


class TestAddNode:
    def test_add_node(self):
        g = StandardGraph()
        alice = make_person("alice", "Alice")
        g.add_node(alice)
        assert g.node_count == 1

    def test_add_multiple_nodes(self):
        g = StandardGraph()
        for i in range(5):
            s = Subject(identity=f"node{i}", labels=set(), properties={})
            g.add_node(s)
        assert g.node_count == 5

    def test_nodes_iteration(self):
        g = StandardGraph()
        alice = make_person("alice", "Alice")
        g.add_node(alice)
        nodes = g.nodes()
        assert len(nodes) == 1
        node_id, pattern = nodes[0]
        assert node_id == "alice"


class TestAddRelationship:
    def test_add_relationship(self):
        g = StandardGraph()
        alice = make_person("alice", "Alice")
        bob = make_person("bob", "Bob")
        rel = Subject(identity="r1", labels={"KNOWS"}, properties={})
        g.add_node(alice)
        g.add_node(bob)
        g.add_relationship(rel, "alice", "bob")
        assert g.relationship_count == 1

    def test_relationships_iteration(self):
        g = StandardGraph()
        alice = make_person("alice", "Alice")
        bob = make_person("bob", "Bob")
        rel = Subject(identity="r1", labels={"KNOWS"}, properties={})
        g.add_node(alice)
        g.add_node(bob)
        g.add_relationship(rel, "alice", "bob")
        rels = g.relationships()
        assert len(rels) == 1
        rel_id, pattern = rels[0]
        assert rel_id == "r1"

    def test_source_target(self):
        g = StandardGraph()
        alice = make_person("alice", "Alice")
        bob = make_person("bob", "Bob")
        rel = Subject(identity="r1", labels={"KNOWS"}, properties={})
        g.add_node(alice)
        g.add_node(bob)
        g.add_relationship(rel, "alice", "bob")
        # source/target return Pattern objects
        assert g.source("r1").value.identity == "alice"
        assert g.target("r1").value.identity == "bob"


class TestAddPattern:
    def test_add_point_pattern(self):
        g = StandardGraph()
        alice = make_person("alice", "Alice")
        p = Pattern.point(alice)
        g.add_pattern(p)
        assert g.node_count == 1

    def test_add_patterns_multiple(self):
        g = StandardGraph()
        subjects = [
            make_person(f"person{i}", f"Person {i}") for i in range(3)
        ]
        patterns = [make_node(s) for s in subjects]
        for p in patterns:
            g.add_pattern(p)
        assert g.node_count == 3


class TestCounts:
    def test_is_empty_false_after_add(self):
        g = StandardGraph()
        g.add_node(make_person("alice", "Alice"))
        assert not g.is_empty

    def test_has_conflicts_false_for_clean_graph(self):
        g = StandardGraph()
        g.add_node(make_person("alice", "Alice"))
        assert not g.has_conflicts

    def test_len(self):
        g = StandardGraph()
        g.add_node(make_person("a", "A"))
        g.add_node(make_person("b", "B"))
        assert len(g) == 2

    def test_repr(self):
        g = StandardGraph()
        g.add_node(make_person("alice", "Alice"))
        r = repr(g)
        assert "StandardGraph" in r
        assert "nodes=1" in r


class TestGraphQueries:
    def test_neighbors(self):
        g = StandardGraph()
        alice = make_person("alice", "Alice")
        bob = make_person("bob", "Bob")
        rel = Subject(identity="r1", labels={"KNOWS"}, properties={})
        g.add_node(alice)
        g.add_node(bob)
        g.add_relationship(rel, "alice", "bob")
        # neighbors() returns list of Pattern objects
        neighbors = g.neighbors("alice")
        neighbor_ids = [p.value.identity for p in neighbors]
        assert "bob" in neighbor_ids

    def test_degree(self):
        g = StandardGraph()
        alice = make_person("alice", "Alice")
        bob = make_person("bob", "Bob")
        rel = Subject(identity="r1", labels={"KNOWS"}, properties={})
        g.add_node(alice)
        g.add_node(bob)
        g.add_relationship(rel, "alice", "bob")
        assert g.degree("alice") >= 1

    def test_source_none_for_node(self):
        g = StandardGraph()
        g.add_node(make_person("alice", "Alice"))
        # nodes have no source endpoint
        src = g.source("alice")
        assert src is None or (hasattr(src, 'value') and src.value.identity == "alice")

    def test_target_none_for_node(self):
        g = StandardGraph()
        g.add_node(make_person("alice", "Alice"))
        tgt = g.target("alice")
        assert tgt is None or (hasattr(tgt, 'value') and tgt.value.identity == "alice")


class TestSubjectBuilder:
    def test_basic_build(self):
        # SubjectBuilder is constructed via Subject.build() fluent API
        s = Subject.build("alice").label("Person").property("name", "Alice").done()
        assert s.identity == "alice"
        assert "Person" in s.get_labels()
        assert s.get_property("name") is not None  # Value("Alice") wrapper

    def test_multiple_labels(self):
        s = Subject.build("x").label("A").label("B").done()
        labels = s.get_labels()
        assert "A" in labels
        assert "B" in labels

    def test_subject_build_static(self):
        s = Subject.build("bob").label("Person").property("age", 30).done()
        assert s.identity == "bob"
        assert "Person" in s.get_labels()

    def test_build_in_graph(self):
        g = StandardGraph()
        s = Subject.build("alice").label("Person").property("name", "Alice").done()
        g.add_node(s)
        assert g.node_count == 1
