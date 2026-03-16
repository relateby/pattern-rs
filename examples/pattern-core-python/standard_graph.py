"""StandardGraph Python example (T043).

Demonstrates using StandardGraph, SubjectBuilder, and from_gram via the
relateby.pattern public API.

Run after installing the relateby wheel:
    cd python/relateby && pip install -e .
    python examples/pattern-core-python/standard_graph.py
"""

from pattern_core import StandardGraph, Subject, SubjectBuilder


def main():
    # --- Basic graph construction ---
    g = StandardGraph()
    assert g.is_empty, "new graph is empty"

    alice = Subject(identity="alice", labels={"Person"}, properties={"name": "Alice"})
    bob = Subject(identity="bob", labels={"Person"}, properties={"name": "Bob"})
    rel = Subject(identity="r1", labels={"KNOWS"}, properties={})

    g.add_node(alice)
    g.add_node(bob)
    # Pass Subject objects directly — no need to spell out identity strings
    g.add_relationship(rel, alice, bob)

    print(f"Nodes: {g.node_count}")           # 2
    print(f"Relationships: {g.relationship_count}")  # 1
    print(f"Source of r1: {g.source('r1')}")  # alice
    print(f"Target of r1: {g.target('r1')}")  # bob
    print(f"Neighbors of alice: {g.neighbors('alice')}")
    print(repr(g))

    # --- SubjectBuilder fluent API ---
    carol = (
        SubjectBuilder("carol")
        .label("Person")
        .property("name", "Carol")
        .done()
    )
    g.add_node(carol)
    print(f"After adding Carol: {g.node_count} nodes")

    # Subject.build() shorthand
    dave = Subject.build("dave").label("Person").property("age", 40).done()
    g.add_node(dave)
    print(f"After adding Dave: {g.node_count} nodes")

    # --- Iteration ---
    print("\nAll nodes:")
    for node_id, pattern in g.nodes():
        print(f"  {node_id}")

    print("\nAll relationships:")
    for rel_id, pattern in g.relationships():
        print(f"  {rel_id}")

    print("\n✓ StandardGraph Python example complete")


if __name__ == "__main__":
    main()
