"""StandardGraph Python example using the public relateby.pattern API."""

from relateby.pattern import Pattern, StandardGraph, StringVal, Subject


def main():
    parsed = StandardGraph.from_gram("(eve:Person)-[:KNOWS]->(frank:Person)")
    print(f"Parsed graph nodes: {parsed.node_count}")

    # --- Graph construction from native Pattern[Subject] values ---
    alice = Pattern.point(
        Subject.from_id("alice").with_label("Person").with_property("name", StringVal("Alice"))
    )
    bob = Pattern.point(
        Subject.from_id("bob").with_label("Person").with_property("name", StringVal("Bob"))
    )
    relationship = Pattern(
        value=Subject.from_id("r1").with_label("KNOWS"),
        elements=[alice, bob],
    )

    g = StandardGraph.from_patterns([relationship])
    print(f"Nodes: {g.node_count}")           # 2
    print(f"Relationships: {g.relationship_count}")  # 1
    print(f"Source of r1: {g.source('r1').value.identity}")  # alice
    print(f"Target of r1: {g.target('r1').value.identity}")  # bob
    print(f"Neighbors of alice: {[node.value.identity for node in g.neighbors('alice')]}")

    # --- Iteration ---
    print("\nAll nodes:")
    for node_id, pattern in g.nodes():
        print(f"  {node_id}: {sorted(pattern.value.labels)}")

    print("\nAll relationships:")
    for rel_id, rel in g.relationships():
        print(f"  {rel_id}: {rel['source']} -> {rel['target']}")

    print("\n✓ StandardGraph Python example complete")


if __name__ == "__main__":
    main()
