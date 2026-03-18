"""StandardGraph Python example.

Demonstrates using StandardGraph and Subject via the
relateby.pattern public API.

Run after installing the combined package from the repo:
    cd python/relateby && python -m pip install .
    cd ../..
    python examples/pattern-core-python/standard_graph.py
"""

from relateby.pattern import StandardGraph, StringVal, Subject


def main():
    parsed = StandardGraph.from_gram("(eve:Person)-[:KNOWS]->(frank:Person)")
    print(f"Parsed graph nodes: {parsed.node_count}")

    # --- Basic graph construction ---
    g = StandardGraph()
    assert g.is_empty, "new graph is empty"

    alice = Subject(identity="alice", labels={"Person"}, properties={"name": StringVal("Alice")})
    bob = Subject(identity="bob", labels={"Person"}, properties={"name": StringVal("Bob")})
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

    # --- Immutable Subject helpers ---
    carol = Subject.from_id("carol").with_label("Person")
    g.add_node(carol)
    print(f"After adding Carol: {g.node_count} nodes")

    dave = Subject.from_id("dave").with_label("Person")
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
