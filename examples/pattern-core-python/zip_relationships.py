#!/usr/bin/env python3
"""
Relationship creation examples using zip3 and zip_with.

Demonstrates:
- Creating relationships from pre-computed lists (zip3)
- Creating relationships with derived values (zip_with)
- Real-world graph patterns
"""

import sys

try:
    import pattern_core
except ImportError:
    print("ERROR: pattern_core module not found.")
    print("Build it with: cd crates/pattern-core && maturin develop --uv --features python")
    sys.exit(1)


def example_zip3_simple():
    """Create relationships from three separate lists."""
    print("=" * 60)
    print("Example 1: Simple Relationships with zip3")
    print("=" * 60)

    # Create source nodes (people)
    sources = [
        pattern_core.Pattern.point("Alice"),
        pattern_core.Pattern.point("Bob"),
        pattern_core.Pattern.point("Charlie"),
    ]

    # Create target nodes (organizations)
    targets = [
        pattern_core.Pattern.point("TechCorp"),
        pattern_core.Pattern.point("StartupInc"),
        pattern_core.Pattern.point("Enterprise Ltd"),
    ]

    # Relationship types from external source (e.g., database, CSV)
    relationship_types = ["WORKS_FOR", "FOUNDED", "CONSULTS_WITH"]

    # Create relationships using zip3
    relationships = pattern_core.Pattern.zip3(sources, targets, relationship_types)

    print(f"Created {len(relationships)} relationships:")
    for rel in relationships:
        src = rel.elements[0].value
        tgt = rel.elements[1].value
        rel_type = rel.value
        print(f"  ({src}) -[:{rel_type}]-> ({tgt})")
    print()


def example_zip3_subjects():
    """Create relationships between Subject nodes."""
    print("=" * 60)
    print("Example 2: Subject Relationships with zip3")
    print("=" * 60)

    # Create people as Subjects
    alice = pattern_core.Subject(
        identity="alice",
        labels={"Person", "Employee"},
        properties={
            "name": pattern_core.Value.string("Alice Johnson"),
            "role": pattern_core.Value.string("Engineer")
        }
    )

    bob = pattern_core.Subject(
        identity="bob",
        labels={"Person", "Employee"},
        properties={
            "name": pattern_core.Value.string("Bob Smith"),
            "role": pattern_core.Value.string("Manager")
        }
    )

    # Create companies as Subjects
    techcorp = pattern_core.Subject(
        identity="techcorp",
        labels={"Company", "Organization"},
        properties={
            "name": pattern_core.Value.string("TechCorp"),
            "industry": pattern_core.Value.string("Technology")
        }
    )

    startup = pattern_core.Subject(
        identity="startup",
        labels={"Company", "Organization"},
        properties={
            "name": pattern_core.Value.string("StartupInc"),
            "industry": pattern_core.Value.string("SaaS")
        }
    )

    # Create Pattern[Subject] nodes
    people = [
        pattern_core.Pattern.point(alice),
        pattern_core.Pattern.point(bob),
    ]

    companies = [
        pattern_core.Pattern.point(techcorp),
        pattern_core.Pattern.point(startup),
    ]

    # Relationship data from external source
    employment_relations = ["WORKS_FOR", "MANAGES"]

    # Create relationships
    # Note: For relationship identities, we extract the identity from Subject values
    people_patterns = [pattern_core.Pattern.point(p.value.identity) for p in people]
    company_patterns = [pattern_core.Pattern.point(c.value.identity) for c in companies]

    relationships = pattern_core.Pattern.zip3(people_patterns, company_patterns, employment_relations)

    print(f"Created {len(relationships)} employment relationships:")
    for rel in relationships:
        person_id = rel.elements[0].value
        company_id = rel.elements[1].value
        rel_type = rel.value
        print(f"  ({person_id}) -[:{rel_type}]-> ({company_id})")
    print()


def example_zip_with_simple():
    """Create relationships with computed values."""
    print("=" * 60)
    print("Example 3: Computed Relationships with zip_with")
    print("=" * 60)

    # Create nodes
    people = [
        pattern_core.Pattern.point("Alice"),
        pattern_core.Pattern.point("Bob"),
        pattern_core.Pattern.point("Charlie"),
    ]

    companies = [
        pattern_core.Pattern.point("TechCorp"),
        pattern_core.Pattern.point("StartupInc"),
        pattern_core.Pattern.point("Enterprise"),
    ]

    # Derive relationship type from the nodes being connected
    def compute_relationship(person, company):
        # Business logic: determine relationship type based on node values
        person_name = person.value
        company_name = company.value

        # Example logic
        if "Startup" in company_name:
            return f"FOUNDED_BY_{person_name}"
        elif "Enterprise" in company_name:
            return f"CONSULTS"
        else:
            return f"EMPLOYED_AT"

    # Create relationships with computed values
    relationships = pattern_core.Pattern.zip_with(people, companies, compute_relationship)

    print(f"Created {len(relationships)} computed relationships:")
    for rel in relationships:
        src = rel.elements[0].value
        tgt = rel.elements[1].value
        rel_type = rel.value
        print(f"  ({src}) -[:{rel_type}]-> ({tgt})")
    print()


def example_zip_with_conditional():
    """Create relationships with conditional logic."""
    print("=" * 60)
    print("Example 4: Conditional Relationships with zip_with")
    print("=" * 60)

    # Create nodes with context
    users = [
        pattern_core.Pattern.point("admin_user"),
        pattern_core.Pattern.point("regular_user"),
        pattern_core.Pattern.point("guest_user"),
    ]

    resources = [
        pattern_core.Pattern.point("database"),
        pattern_core.Pattern.point("api"),
        pattern_core.Pattern.point("public_page"),
    ]

    # Determine access type based on user and resource
    def determine_access(user, resource):
        user_type = user.value.split("_")[0]  # Extract role
        resource_type = resource.value

        if user_type == "admin":
            return "FULL_ACCESS"
        elif user_type == "regular":
            if resource_type == "database":
                return "READ_ONLY"
            else:
                return "READ_WRITE"
        else:  # guest
            if resource_type == "public_page":
                return "READ_ONLY"
            else:
                return "NO_ACCESS"

    access_relations = pattern_core.Pattern.zip_with(users, resources, determine_access)

    print(f"Created {len(access_relations)} access control relationships:")
    for rel in access_relations:
        user = rel.elements[0].value
        resource = rel.elements[1].value
        access = rel.value
        print(f"  ({user}) -[:{access}]-> ({resource})")
    print()


def example_bulk_import():
    """Simulate bulk import from external data."""
    print("=" * 60)
    print("Example 5: Bulk Import Pattern (Real-World Use Case)")
    print("=" * 60)

    # Simulate data from CSV or database query
    # In real scenario: data = pd.read_csv("relationships.csv")
    import_data = [
        ("Alice", "Bob", "KNOWS"),
        ("Bob", "Charlie", "WORKS_WITH"),
        ("Charlie", "Alice", "REPORTS_TO"),
        ("Alice", "Diana", "MANAGES"),
        ("Diana", "Bob", "MENTORS"),
    ]

    print(f"Importing {len(import_data)} relationships from external source...")

    # Extract columns
    sources = [pattern_core.Pattern.point(row[0]) for row in import_data]
    targets = [pattern_core.Pattern.point(row[1]) for row in import_data]
    rel_types = [row[2] for row in import_data]

    # Bulk create relationships
    relationships = pattern_core.Pattern.zip3(sources, targets, rel_types)

    print(f"Successfully imported {len(relationships)} relationships:")
    for i, rel in enumerate(relationships, 1):
        src = rel.elements[0].value
        tgt = rel.elements[1].value
        rel_type = rel.value
        print(f"  {i}. ({src}) -[:{rel_type}]-> ({tgt})")
    print()


def example_graph_queries():
    """Build a graph and perform queries."""
    print("=" * 60)
    print("Example 6: Graph Building and Queries")
    print("=" * 60)

    # Build a social network
    people = ["Alice", "Bob", "Charlie", "Diana", "Eve"]

    # Friendship connections (undirected, so we create both directions)
    friendships = [
        ("Alice", "Bob"),
        ("Alice", "Charlie"),
        ("Bob", "Charlie"),
        ("Charlie", "Diana"),
        ("Diana", "Eve"),
    ]

    # Create friendship relationships
    sources = [pattern_core.Pattern.point(src) for src, _ in friendships]
    targets = [pattern_core.Pattern.point(tgt) for _, tgt in friendships]
    rel_type = ["FRIENDS_WITH"] * len(friendships)

    relationships = pattern_core.Pattern.zip3(sources, targets, rel_type)

    print(f"Social network with {len(people)} people and {len(relationships)} friendships:")

    # Query: Find all of Alice's friends
    alice_friends = [
        rel.elements[1].value
        for rel in relationships
        if rel.elements[0].value == "Alice"
    ]
    print(f"\nAlice's friends: {', '.join(alice_friends)}")

    # Query: Count connections per person
    connection_counts = {}
    for rel in relationships:
        src = rel.elements[0].value
        connection_counts[src] = connection_counts.get(src, 0) + 1

    print(f"\nConnection counts:")
    for person, count in sorted(connection_counts.items()):
        print(f"  {person}: {count} connections")
    print()


def example_comparison():
    """Compare zip3 vs zip_with approaches."""
    print("=" * 60)
    print("Example 7: Comparing zip3 vs zip_with")
    print("=" * 60)

    users = [
        pattern_core.Pattern.point("user1"),
        pattern_core.Pattern.point("user2"),
        pattern_core.Pattern.point("user3"),
    ]

    posts = [
        pattern_core.Pattern.point("post_a"),
        pattern_core.Pattern.point("post_b"),
        pattern_core.Pattern.point("post_c"),
    ]

    print("Approach 1: Pre-computed values (zip3)")
    print("  Use when: Data from database, CSV, API")

    actions = ["CREATED", "LIKED", "COMMENTED_ON"]
    rels_zip3 = pattern_core.Pattern.zip3(users, posts, actions)

    for rel in rels_zip3:
        print(f"    ({rel.elements[0].value}) -[:{rel.value}]-> ({rel.elements[1].value})")

    print("\nApproach 2: Computed values (zip_with)")
    print("  Use when: Values derived from nodes, business rules")

    rels_zip_with = pattern_core.Pattern.zip_with(
        users,
        posts,
        lambda u, p: f"ACTION_BY_{u.value}_ON_{p.value}"
    )

    for rel in rels_zip_with:
        print(f"    ({rel.elements[0].value}) -[:{rel.value}]-> ({rel.elements[1].value})")

    print()


def main():
    """Run all examples."""
    print("\n" + "=" * 60)
    print("PATTERN-CORE: RELATIONSHIP CREATION (ZIP3 & ZIP_WITH)")
    print("=" * 60 + "\n")

    example_zip3_simple()
    example_zip3_subjects()
    example_zip_with_simple()
    example_zip_with_conditional()
    example_bulk_import()
    example_graph_queries()
    example_comparison()

    print("=" * 60)
    print("All relationship creation examples completed!")
    print("=" * 60)


if __name__ == "__main__":
    main()
