#!/usr/bin/env python3
"""
Advanced examples for pattern-core Python bindings.

Demonstrates:
- Comonad operations (extract, extend, depth_at, size_at, indices_at)
- Complex Subject structures
- Pattern validation with rules
- Structure analysis
- Real-world use cases (file trees, social graphs, data pipelines)
"""

import sys

try:
    import relateby.pattern
    from relateby.pattern import Pattern, Subject, Value, ValidationRules
except ImportError:
    print("ERROR: relateby.pattern not found. Install with: pip install relateby")
    print("Build it with: cd crates/pattern-core && maturin develop --uv --features python")
    sys.exit(1)


def example_comonad_extract():
    """Extract value at current position."""
    print("=" * 60)
    print("Example 1: Comonad Extract")
    print("=" * 60)

    # Create pattern
    pattern = relateby.pattern.Pattern.point("hello")

    # Extract value
    value = pattern.extract()
    print(f"Extracted value: {value}")

    # Extract from nested pattern
    nested = relateby.pattern.Pattern.pattern("root", [
        relateby.pattern.Pattern.point("elem")
    ])
    root_value = nested.extract()
    print(f"Extracted root value: {root_value}")
    print()


def example_comonad_extend():
    """Extend pattern with function applied to contexts."""
    print("=" * 60)
    print("Example 2: Comonad Extend")
    print("=" * 60)

    # Create pattern
    pattern = relateby.pattern.Pattern.pattern("root", [
        relateby.pattern.Pattern.point("a"),
        relateby.pattern.Pattern.point("b"),
        relateby.pattern.Pattern.point("c")
    ])

    print(f"Original values: {pattern.values()}")

    # Extend with size function
    def get_size(p: Pattern) -> str:
        return str(p.size())

    sizes = pattern.extend(get_size)
    print(f"Sizes at each position: {sizes.values()}")

    # Extend with depth function
    def get_depth(p: Pattern) -> str:
        return str(p.depth())

    depths = pattern.extend(get_depth)
    print(f"Depths at each position: {depths.values()}")
    print()


def example_depth_at():
    """Decorate pattern with depth information."""
    print("=" * 60)
    print("Example 3: Depth At Each Position")
    print("=" * 60)

    # Create deeply nested pattern
    leaf = relateby.pattern.Pattern.point("leaf")
    level2 = relateby.pattern.Pattern.pattern("level2", [leaf])
    level1 = relateby.pattern.Pattern.pattern("level1", [level2])
    root = relateby.pattern.Pattern.pattern("root", [level1])

    print(f"Original structure: {root.values()}")

    # Decorate with depths
    depths = root.depth_at()
    print(f"Depth at each position: {depths.values()}")
    print()


def example_size_at():
    """Decorate pattern with subtree size."""
    print("=" * 60)
    print("Example 4: Subtree Size At Each Position")
    print("=" * 60)

    # Create pattern
    pattern = relateby.pattern.Pattern.pattern("root", [
        relateby.pattern.Pattern.pattern("branch1", [
            relateby.pattern.Pattern.point("leaf1"),
            relateby.pattern.Pattern.point("leaf2")
        ]),
        relateby.pattern.Pattern.pattern("branch2", [
            relateby.pattern.Pattern.point("leaf3")
        ])
    ])

    print(f"Original structure: {pattern.values()}")
    print(f"Total size: {pattern.size()}")

    # Decorate with sizes
    sizes = pattern.size_at()
    print(f"Subtree size at each position: {sizes.values()}")
    print()


def example_indices_at():
    """Decorate pattern with path indices."""
    print("=" * 60)
    print("Example 5: Path Indices At Each Position")
    print("=" * 60)

    # Create pattern
    pattern = relateby.pattern.Pattern.pattern("root", [
        relateby.pattern.Pattern.point("elem0"),
        relateby.pattern.Pattern.point("elem1"),
        relateby.pattern.Pattern.point("elem2")
    ])

    print(f"Original structure: {pattern.values()}")

    # Decorate with indices
    indices = pattern.indices_at()
    print(f"Path indices at each position: {indices.values()}")
    print()


def example_complex_subject():
    """Create complex Subject with rich properties."""
    print("=" * 60)
    print("Example 6: Complex Subject Structure")
    print("=" * 60)

    # Create Subject with nested properties
    person = relateby.pattern.Subject(
        identity="alice",
        labels={"Person", "Employee", "Developer", "TeamLead"},
        properties={
            "name": relateby.pattern.Value.string("Alice Johnson"),
            "age": relateby.pattern.Value.int(35),
            "email": relateby.pattern.Value.string("alice@example.com"),
            "skills": relateby.pattern.Value.array([
                relateby.pattern.Value.string("Python"),
                relateby.pattern.Value.string("Rust"),
                relateby.pattern.Value.string("TypeScript")
            ]),
            "metadata": relateby.pattern.Value.map({
                "department": relateby.pattern.Value.string("Engineering"),
                "level": relateby.pattern.Value.string("Senior"),
                "years": relateby.pattern.Value.int(8)
            }),
            "salary_range": relateby.pattern.Value.range(lower=100000.0, upper=150000.0),
            "height": relateby.pattern.Value.measurement(175.0, "cm")
        }
    )

    print(f"Identity: {person.identity}")
    print(f"Labels: {person.get_labels()}")

    # Access nested properties
    skills = person.get_property("skills")
    if skills:
        skills_array = skills.as_array()
        # Note: as_array() returns Python list, not List[Value]
        print(f"Skills: {skills_array}")

    metadata = person.get_property("metadata")
    if metadata:
        metadata_dict = metadata.as_map()
        # Note: as_map() returns Python dict, not Dict[str, Value]
        print(f"Metadata: {metadata_dict}")
    print()


def example_validation():
    """Validate patterns with rules."""
    print("=" * 60)
    print("Example 7: Pattern Validation")
    print("=" * 60)

    # Create pattern
    pattern = relateby.pattern.Pattern.pattern("root", [
        relateby.pattern.Pattern.point("elem1"),
        relateby.pattern.Pattern.point("elem2")
    ])

    # Create validation rules
    rules = relateby.pattern.ValidationRules(
        max_depth=5,
        max_elements=10
    )

    try:
        pattern.validate(rules)
        print("✓ Pattern is valid")
        print(f"  Depth: {pattern.depth()} (max: 5)")
        print(f"  Length: {pattern.length()} (max: 10)")
    except relateby.pattern.ValidationError as e:
        print(f"✗ Validation failed: {e.message}")
        print(f"  Rule: {e.rule}")

    # Test with invalid pattern (too deep)
    def create_deep_pattern(depth: int) -> Pattern:
        """Create pattern with specified depth."""
        if depth == 0:
            return relateby.pattern.Pattern.point("leaf")
        else:
            child = create_deep_pattern(depth - 1)
            return relateby.pattern.Pattern.pattern(f"level{depth}", [child])

    deep_pattern = create_deep_pattern(10)
    strict_rules = relateby.pattern.ValidationRules(max_depth=5)

    try:
        deep_pattern.validate(strict_rules)
        print("✓ Deep pattern is valid")
    except ValueError as e:
        # ValidationError extends ValueError
        print(f"✗ Deep pattern validation failed (expected)")
        print(f"  Reason: Pattern depth {deep_pattern.depth()} exceeds max depth 5")
    print()


def example_structure_analysis():
    """Analyze pattern structure."""
    print("=" * 60)
    print("Example 8: Structure Analysis")
    print("=" * 60)

    # Create complex pattern
    pattern = relateby.pattern.Pattern.pattern("root", [
        relateby.pattern.Pattern.pattern("branch1", [
            relateby.pattern.Pattern.point("leaf1"),
            relateby.pattern.Pattern.point("leaf2"),
            relateby.pattern.Pattern.point("leaf3")
        ]),
        relateby.pattern.Pattern.pattern("branch2", [
            relateby.pattern.Pattern.pattern("subbranch", [
                relateby.pattern.Pattern.point("leaf4")
            ]),
            relateby.pattern.Pattern.point("leaf5")
        ]),
        relateby.pattern.Pattern.point("leaf6")
    ])

    # Analyze structure
    analysis = pattern.analyze_structure()

    print(f"Summary: {analysis.summary}")
    print(f"Depth distribution: {analysis.depth_distribution}")
    print(f"Element counts: {analysis.element_counts}")
    print(f"Nesting patterns: {analysis.nesting_patterns}")
    print()


def example_file_tree():
    """Model a file system tree."""
    print("=" * 60)
    print("Example 9: File System Tree (Real-World Use Case)")
    print("=" * 60)

    # Build file system structure
    src_dir = relateby.pattern.Pattern.pattern("src", [
        relateby.pattern.Pattern.point("main.py"),
        relateby.pattern.Pattern.point("utils.py"),
        relateby.pattern.Pattern.pattern("models", [
            relateby.pattern.Pattern.point("user.py"),
            relateby.pattern.Pattern.point("post.py")
        ])
    ])

    tests_dir = relateby.pattern.Pattern.pattern("tests", [
        relateby.pattern.Pattern.point("test_main.py"),
        relateby.pattern.Pattern.point("test_utils.py")
    ])

    project = relateby.pattern.Pattern.pattern("myproject", [
        src_dir,
        tests_dir,
        relateby.pattern.Pattern.point("README.md"),
        relateby.pattern.Pattern.point("setup.py")
    ])

    print(f"Project structure:")
    print(f"  Total files/dirs: {project.size()}")
    print(f"  Max depth: {project.depth()}")
    print(f"  All paths: {project.values()}")

    # Find all Python files
    python_files = project.filter(lambda p: p.value.endswith(".py"))
    print(f"  Python files: {[p.value for p in python_files]}")

    # Analyze structure
    analysis = project.analyze_structure()
    print(f"  Structure summary: {analysis.summary}")
    print()


def example_social_graph():
    """Model a social network graph."""
    print("=" * 60)
    print("Example 10: Social Network Graph (Real-World Use Case)")
    print("=" * 60)

    # Create people
    alice = relateby.pattern.Subject(
        identity="alice",
        labels={"Person", "Developer"},
        properties={
            "name": relateby.pattern.Value.string("Alice"),
            "age": relateby.pattern.Value.int(30)
        }
    )

    bob = relateby.pattern.Subject(
        identity="bob",
        labels={"Person", "Designer"},
        properties={
            "name": relateby.pattern.Value.string("Bob"),
            "age": relateby.pattern.Value.int(28)
        }
    )

    charlie = relateby.pattern.Subject(
        identity="charlie",
        labels={"Person", "Manager"},
        properties={
            "name": relateby.pattern.Value.string("Charlie"),
            "age": relateby.pattern.Value.int(35)
        }
    )

    dave = relateby.pattern.Subject(
        identity="dave",
        labels={"Person", "Developer"},
        properties={
            "name": relateby.pattern.Value.string("Dave"),
            "age": relateby.pattern.Value.int(32)
        }
    )

    # Build social graph (who knows whom)
    bob_pattern = relateby.pattern.Pattern.point(bob)
    charlie_pattern = relateby.pattern.Pattern.point(charlie)
    dave_pattern = relateby.pattern.Pattern.point(dave)

    # Alice knows Bob and Charlie
    alice_graph = relateby.pattern.Pattern.pattern(alice, [bob_pattern, charlie_pattern])

    # Charlie knows Dave
    charlie_with_friends = relateby.pattern.Pattern.pattern(charlie, [dave_pattern])

    print("Social Network:")
    print(f"  Alice knows {alice_graph.length()} people")
    print(f"  Charlie's network: {charlie_with_friends.size()} people")
    print(f"  Total people in Alice's network: {alice_graph.size()}")

    # Query: Find all developers
    developers = alice_graph.filter(lambda p: p.value.has_label("Developer"))
    dev_names = [p.value.get_property("name").as_string() for p in developers if p.value.get_property("name")]
    print(f"  Developers in network: {dev_names}")

    # Query: Find average age
    def sum_ages(acc: int, subject: Subject) -> int:
        age_prop = subject.get_property("age")
        if age_prop:
            return acc + age_prop.as_int()
        return acc

    total_age = alice_graph.fold(0, sum_ages)
    avg_age = total_age / alice_graph.size()
    print(f"  Average age: {avg_age:.1f}")
    print()


def example_data_pipeline():
    """Build a data transformation pipeline."""
    print("=" * 60)
    print("Example 11: Data Pipeline (Real-World Use Case)")
    print("=" * 60)

    # Create data pattern
    data = relateby.pattern.Pattern.pattern("data", relateby.pattern.Pattern.from_values([
        "apple", "banana", "cherry", "date", "elderberry", "fig", "grape"
    ]))

    print(f"Input data: {data.values()[1:]}")  # Skip root "data"

    # Pipeline: Filter -> Transform -> Aggregate
    # Step 1: Filter (keep fruits with 'e')
    filtered = data.filter(lambda p: 'e' in p.value)
    print(f"Step 1 (contains 'e'): {[p.value for p in filtered]}")

    # Step 2: Transform (uppercase)
    if filtered:
        filtered_pattern = relateby.pattern.Pattern.pattern("filtered", filtered)
        transformed = filtered_pattern.map(str.upper)
        print(f"Step 2 (uppercase): {transformed.values()[1:]}")  # Skip root

        # Step 3: Aggregate (concatenate)
        result = transformed.fold("", lambda acc, val:
            acc + ("," if acc and val != "FILTERED" else "") + (val if val != "FILTERED" else "")
        )
        print(f"Step 3 (concatenated): {result}")
    print()


def example_pattern_composition():
    """Compose patterns from smaller patterns."""
    print("=" * 60)
    print("Example 12: Pattern Composition")
    print("=" * 60)

    # Create reusable subpatterns
    left_subtree = relateby.pattern.Pattern.pattern("left", [
        relateby.pattern.Pattern.point("L1"),
        relateby.pattern.Pattern.point("L2")
    ])

    right_subtree = relateby.pattern.Pattern.pattern("right", [
        relateby.pattern.Pattern.point("R1"),
        relateby.pattern.Pattern.point("R2"),
        relateby.pattern.Pattern.point("R3")
    ])

    # Compose into larger pattern
    tree = relateby.pattern.Pattern.pattern("root", [left_subtree, right_subtree])

    print(f"Composed tree:")
    print(f"  Total nodes: {tree.size()}")
    print(f"  All values: {tree.values()}")

    # Analyze composition
    analysis = tree.analyze_structure()
    print(f"  Structure: {analysis.summary}")
    print()


def main():
    """Run all advanced examples."""
    print("\n" + "=" * 60)
    print("PATTERN-CORE PYTHON BINDINGS - ADVANCED EXAMPLES")
    print("=" * 60 + "\n")

    example_comonad_extract()
    example_comonad_extend()
    example_depth_at()
    example_size_at()
    example_indices_at()
    example_complex_subject()
    example_validation()
    example_structure_analysis()
    example_file_tree()
    example_social_graph()
    example_data_pipeline()
    example_pattern_composition()

    print("=" * 60)
    print("All advanced examples completed successfully!")
    print("=" * 60)


if __name__ == "__main__":
    main()
