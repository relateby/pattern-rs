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
    import pattern_core
    from pattern_core import Pattern, PatternSubject, Subject, Value, ValidationRules
except ImportError:
    print("ERROR: pattern_core module not found.")
    print("Build it with: cd crates/pattern-core && maturin develop --uv --features python")
    sys.exit(1)


def example_comonad_extract():
    """Extract value at current position."""
    print("=" * 60)
    print("Example 1: Comonad Extract")
    print("=" * 60)
    
    # Create pattern
    pattern = pattern_core.Pattern.point("hello")
    
    # Extract value
    value = pattern.extract()
    print(f"Extracted value: {value}")
    
    # Extract from nested pattern
    nested = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.point("elem")
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
    pattern = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.point("a"),
        pattern_core.Pattern.point("b"),
        pattern_core.Pattern.point("c")
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
    leaf = pattern_core.Pattern.point("leaf")
    level2 = pattern_core.Pattern.pattern("level2", [leaf])
    level1 = pattern_core.Pattern.pattern("level1", [level2])
    root = pattern_core.Pattern.pattern("root", [level1])
    
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
    pattern = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.pattern("branch1", [
            pattern_core.Pattern.point("leaf1"),
            pattern_core.Pattern.point("leaf2")
        ]),
        pattern_core.Pattern.pattern("branch2", [
            pattern_core.Pattern.point("leaf3")
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
    pattern = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.point("elem0"),
        pattern_core.Pattern.point("elem1"),
        pattern_core.Pattern.point("elem2")
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
    person = pattern_core.Subject(
        identity="alice",
        labels={"Person", "Employee", "Developer", "TeamLead"},
        properties={
            "name": pattern_core.Value.string("Alice Johnson"),
            "age": pattern_core.Value.int(35),
            "email": pattern_core.Value.string("alice@example.com"),
            "skills": pattern_core.Value.array([
                pattern_core.Value.string("Python"),
                pattern_core.Value.string("Rust"),
                pattern_core.Value.string("TypeScript")
            ]),
            "metadata": pattern_core.Value.map({
                "department": pattern_core.Value.string("Engineering"),
                "level": pattern_core.Value.string("Senior"),
                "years": pattern_core.Value.int(8)
            }),
            "salary_range": pattern_core.Value.range(lower=100000.0, upper=150000.0),
            "height": pattern_core.Value.measurement(175.0, "cm")
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
    pattern = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.point("elem1"),
        pattern_core.Pattern.point("elem2")
    ])
    
    # Create validation rules
    rules = pattern_core.ValidationRules(
        max_depth=5,
        max_elements=10
    )
    
    try:
        pattern.validate(rules)
        print("✓ Pattern is valid")
        print(f"  Depth: {pattern.depth()} (max: 5)")
        print(f"  Length: {pattern.length()} (max: 10)")
    except pattern_core.ValidationError as e:
        print(f"✗ Validation failed: {e.message}")
        print(f"  Rule: {e.rule}")
    
    # Test with invalid pattern (too deep)
    def create_deep_pattern(depth: int) -> Pattern:
        """Create pattern with specified depth."""
        if depth == 0:
            return pattern_core.Pattern.point("leaf")
        else:
            child = create_deep_pattern(depth - 1)
            return pattern_core.Pattern.pattern(f"level{depth}", [child])
    
    deep_pattern = create_deep_pattern(10)
    strict_rules = pattern_core.ValidationRules(max_depth=5)
    
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
    pattern = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.pattern("branch1", [
            pattern_core.Pattern.point("leaf1"),
            pattern_core.Pattern.point("leaf2"),
            pattern_core.Pattern.point("leaf3")
        ]),
        pattern_core.Pattern.pattern("branch2", [
            pattern_core.Pattern.pattern("subbranch", [
                pattern_core.Pattern.point("leaf4")
            ]),
            pattern_core.Pattern.point("leaf5")
        ]),
        pattern_core.Pattern.point("leaf6")
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
    src_dir = pattern_core.Pattern.pattern("src", [
        pattern_core.Pattern.point("main.py"),
        pattern_core.Pattern.point("utils.py"),
        pattern_core.Pattern.pattern("models", [
            pattern_core.Pattern.point("user.py"),
            pattern_core.Pattern.point("post.py")
        ])
    ])
    
    tests_dir = pattern_core.Pattern.pattern("tests", [
        pattern_core.Pattern.point("test_main.py"),
        pattern_core.Pattern.point("test_utils.py")
    ])
    
    project = pattern_core.Pattern.pattern("myproject", [
        src_dir,
        tests_dir,
        pattern_core.Pattern.point("README.md"),
        pattern_core.Pattern.point("setup.py")
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
    alice = pattern_core.Subject(
        identity="alice",
        labels={"Person", "Developer"},
        properties={
            "name": pattern_core.Value.string("Alice"),
            "age": pattern_core.Value.int(30)
        }
    )
    
    bob = pattern_core.Subject(
        identity="bob",
        labels={"Person", "Designer"},
        properties={
            "name": pattern_core.Value.string("Bob"),
            "age": pattern_core.Value.int(28)
        }
    )
    
    charlie = pattern_core.Subject(
        identity="charlie",
        labels={"Person", "Manager"},
        properties={
            "name": pattern_core.Value.string("Charlie"),
            "age": pattern_core.Value.int(35)
        }
    )
    
    dave = pattern_core.Subject(
        identity="dave",
        labels={"Person", "Developer"},
        properties={
            "name": pattern_core.Value.string("Dave"),
            "age": pattern_core.Value.int(32)
        }
    )
    
    # Build social graph (who knows whom)
    bob_pattern = pattern_core.PatternSubject.point(bob)
    charlie_pattern = pattern_core.PatternSubject.point(charlie)
    dave_pattern = pattern_core.PatternSubject.point(dave)
    
    # Alice knows Bob and Charlie
    alice_graph = pattern_core.PatternSubject.pattern(alice, [bob_pattern, charlie_pattern])
    
    # Charlie knows Dave
    charlie_with_friends = pattern_core.PatternSubject.pattern(charlie, [dave_pattern])
    
    print("Social Network:")
    print(f"  Alice knows {alice_graph.length()} people")
    print(f"  Charlie's network: {charlie_with_friends.size()} people")
    print(f"  Total people in Alice's network: {alice_graph.size()}")
    
    # Query: Find all developers
    developers = alice_graph.filter(lambda p: p.get_value().has_label("Developer"))
    dev_names = [p.get_value().get_property("name").as_string() for p in developers if p.get_value().get_property("name")]
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
    data = pattern_core.Pattern.pattern("data", pattern_core.Pattern.from_values([
        "apple", "banana", "cherry", "date", "elderberry", "fig", "grape"
    ]))
    
    print(f"Input data: {data.values()[1:]}")  # Skip root "data"
    
    # Pipeline: Filter -> Transform -> Aggregate
    # Step 1: Filter (keep fruits with 'e')
    filtered = data.filter(lambda p: 'e' in p.value)
    print(f"Step 1 (contains 'e'): {[p.value for p in filtered]}")
    
    # Step 2: Transform (uppercase)
    if filtered:
        filtered_pattern = pattern_core.Pattern.pattern("filtered", filtered)
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
    left_subtree = pattern_core.Pattern.pattern("left", [
        pattern_core.Pattern.point("L1"),
        pattern_core.Pattern.point("L2")
    ])
    
    right_subtree = pattern_core.Pattern.pattern("right", [
        pattern_core.Pattern.point("R1"),
        pattern_core.Pattern.point("R2"),
        pattern_core.Pattern.point("R3")
    ])
    
    # Compose into larger pattern
    tree = pattern_core.Pattern.pattern("root", [left_subtree, right_subtree])
    
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
