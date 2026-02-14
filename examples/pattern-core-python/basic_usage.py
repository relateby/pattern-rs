#!/usr/bin/env python3
"""
Basic usage examples for pattern-core Python bindings.

Demonstrates:
- Creating atomic and nested patterns
- Working with Subjects (identity, labels, properties)
- Creating Pattern[Subject] instances
- Basic pattern inspection
"""

import sys

try:
    import relateby.pattern
except ImportError:
    print("ERROR: relateby.pattern not found. Install with: pip install relateby")
    print("Or from TestPyPI: pip install --index-url https://test.pypi.org/simple/ relateby")
    sys.exit(1)


def example_pattern_of_alias():
    """Pattern.of() is an alias for Pattern.point() (functor/applicative convention)."""
    print("=" * 60)
    print("Example 0: Pattern.of() Alias")
    print("=" * 60)

    # Both create atomic patterns
    p1 = relateby.pattern.Pattern.point(42)
    p2 = relateby.pattern.Pattern.of(42)

    print(f"Pattern.point(42): {p1}")
    print(f"Pattern.of(42): {p2}")
    print(f"Both have same value: {p1.value == p2.value}")
    print()


def example_from_values():
    """Pattern.from_values() converts a list of values to a list of atomic patterns."""
    print("=" * 60)
    print("Example 0b: Pattern.from_values()")
    print("=" * 60)

    # Convert list of values to list of patterns
    values = [1, 2, 3, 4, 5]
    patterns = relateby.pattern.Pattern.from_values(values)

    print(f"Input values: {values}")
    print(f"Number of patterns created: {len(patterns)}")
    print(f"Pattern values: {[p.value for p in patterns]}")

    # Use with Pattern.pattern() to create nested structure
    root = relateby.pattern.Pattern.pattern("numbers", patterns)
    print(f"\nNested pattern: value='{root.value}', elements={root.length()}")
    print()


def example_atomic_pattern():
    """Create and inspect atomic patterns."""
    print("=" * 60)
    print("Example 1: Atomic Patterns")
    print("=" * 60)

    # Create atomic pattern
    atomic = relateby.pattern.Pattern.point("hello")

    print(f"Value: {atomic.value}")
    print(f"Elements: {atomic.elements}")
    print(f"Is atomic: {atomic.is_atomic()}")
    print(f"Length: {atomic.length()}")
    print(f"Size: {atomic.size()}")
    print(f"Depth: {atomic.depth()}")
    print()


def example_nested_pattern():
    """Create and inspect nested patterns."""
    print("=" * 60)
    print("Example 2: Nested Patterns")
    print("=" * 60)

    # Create atomic patterns (elements)
    elem1 = relateby.pattern.Pattern.point("elem1")
    elem2 = relateby.pattern.Pattern.point("elem2")
    elem3 = relateby.pattern.Pattern.point("elem3")

    # Create decorated pattern (value decorates the elements)
    decorated = relateby.pattern.Pattern.pattern("decoration", [elem1, elem2, elem3])

    print(f"Decoration value: {decorated.value}")
    print(f"Number of elements (length): {decorated.length()}")
    print(f"Total nodes (size): {decorated.size()}")
    print(f"Maximum depth: {decorated.depth()}")
    print(f"Is atomic: {decorated.is_atomic()}")

    # Access elements
    print(f"\nElements:")
    for i, elem in enumerate(decorated.elements):
        print(f"  Element {i}: {elem.value}")
    print()


def example_pattern_from_list():
    """Create pattern from list of values using from_values()."""
    print("=" * 60)
    print("Example 3: Pattern from List (using from_values)")
    print("=" * 60)

    # Convert values to patterns (elements), then create decorated pattern
    elements = relateby.pattern.Pattern.from_values(["a", "b", "c", "d"])
    pattern = relateby.pattern.Pattern.pattern("root", elements)

    print(f"Root value: {pattern.value}")
    print(f"Length: {pattern.length()}")
    print(f"All values: {pattern.values()}")
    print()


def example_deeply_nested_pattern():
    """Create deeply nested pattern."""
    print("=" * 60)
    print("Example 4: Deeply Nested Pattern")
    print("=" * 60)

    # Build tree structure
    leaf = relateby.pattern.Pattern.point("leaf")
    level2 = relateby.pattern.Pattern.pattern("level2", [leaf])
    level1 = relateby.pattern.Pattern.pattern("level1", [level2])
    root = relateby.pattern.Pattern.pattern("root", [level1])

    print(f"Root value: {root.value}")
    print(f"Maximum depth: {root.depth()}")
    print(f"Total size: {root.size()}")
    print(f"All values: {root.values()}")
    print()


def example_value_types():
    """Create and work with Value types."""
    print("=" * 60)
    print("Example 5: Value Types")
    print("=" * 60)

    # Create different value types
    str_val = relateby.pattern.Value.string("hello")
    int_val = relateby.pattern.Value.int(42)
    decimal_val = relateby.pattern.Value.decimal(3.14)
    bool_val = relateby.pattern.Value.boolean(True)
    symbol_val = relateby.pattern.Value.symbol("alice")

    print(f"String value: {str_val.as_string()}")
    print(f"Integer value: {int_val.as_int()}")
    print(f"Decimal value: {decimal_val.as_decimal()}")
    print(f"Boolean value: {bool_val.as_boolean()}")
    print(f"Symbol value: {symbol_val.as_string()}")  # Symbols are strings

    # Array and map
    array_val = relateby.pattern.Value.array([
        relateby.pattern.Value.int(1),
        relateby.pattern.Value.int(2),
        relateby.pattern.Value.int(3)
    ])

    map_val = relateby.pattern.Value.map({
        "key1": relateby.pattern.Value.string("value1"),
        "key2": relateby.pattern.Value.int(42)
    })

    print(f"\nArray: {array_val.as_array()}")
    print(f"Map: {map_val.as_map()}")

    # Range and measurement
    range_val = relateby.pattern.Value.range(lower=0.0, upper=100.0)
    measurement_val = relateby.pattern.Value.measurement(42.5, "meters")

    print(f"Range: {range_val}")
    print(f"Measurement: {measurement_val}")
    print()


def example_subject_basic():
    """Create and work with basic Subjects."""
    print("=" * 60)
    print("Example 6: Basic Subject")
    print("=" * 60)

    # Create Subject with identity only
    subject = relateby.pattern.Subject(identity="alice")

    print(f"Identity: {subject.identity}")
    print(f"Labels: {subject.get_labels()}")
    print(f"Properties: {subject.get_properties()}")
    print()


def example_subject_with_labels():
    """Create Subject with labels."""
    print("=" * 60)
    print("Example 7: Subject with Labels")
    print("=" * 60)

    # Create Subject with labels
    subject = relateby.pattern.Subject(
        identity="alice",
        labels={"Person", "Employee", "Developer"}
    )

    print(f"Identity: {subject.identity}")
    print(f"Labels: {subject.get_labels()}")

    # Add and remove labels
    subject.add_label("Manager")
    print(f"After adding Manager: {subject.get_labels()}")

    subject.remove_label("Developer")
    print(f"After removing Developer: {subject.get_labels()}")

    # Check label
    has_employee = subject.has_label("Employee")
    print(f"Has Employee label: {has_employee}")
    print()


def example_subject_with_properties():
    """Create Subject with properties."""
    print("=" * 60)
    print("Example 8: Subject with Properties")
    print("=" * 60)

    # Create Subject with properties
    subject = relateby.pattern.Subject(
        identity="alice",
        properties={
            "name": relateby.pattern.Value.string("Alice"),
            "age": relateby.pattern.Value.int(30),
            "email": relateby.pattern.Value.string("alice@example.com")
        }
    )

    print(f"Identity: {subject.identity}")
    print(f"Properties: {subject.get_properties()}")

    # Get specific property
    name_value = subject.get_property("name")
    if name_value:
        print(f"Name: {name_value.as_string()}")

    # Set and remove properties
    subject.set_property("department", relateby.pattern.Value.string("Engineering"))
    print(f"After adding department: {subject.get_properties()}")

    subject.remove_property("email")
    print(f"After removing email: {subject.get_properties()}")
    print()


def example_pattern_subject():
    """Create Pattern with Subject value."""
    print("=" * 60)
    print("Example 9: Pattern[Subject]")
    print("=" * 60)

    # Create Subject
    subject = relateby.pattern.Subject(
        identity="alice",
        labels={"Person", "Employee"},
        properties={
            "name": relateby.pattern.Value.string("Alice"),
            "age": relateby.pattern.Value.int(30)
        }
    )

    # Create Pattern[Subject]
    pattern = relateby.pattern.Pattern.point(subject)

    print(f"Pattern value (identity): {pattern.value.identity}")
    print(f"Pattern is atomic: {pattern.is_atomic()}")

    # Access Subject properties through pattern
    subject_value = pattern.value
    print(f"Subject labels: {subject_value.get_labels()}")
    print(f"Subject properties: {subject_value.get_properties()}")
    print()


def example_pattern_subject_nested():
    """Create nested Pattern[Subject]."""
    print("=" * 60)
    print("Example 10: Nested Pattern[Subject]")
    print("=" * 60)

    # Create multiple Subjects
    alice = relateby.pattern.Subject(
        identity="alice",
        labels={"Person"},
        properties={"name": relateby.pattern.Value.string("Alice")}
    )

    bob = relateby.pattern.Subject(
        identity="bob",
        labels={"Person"},
        properties={"name": relateby.pattern.Value.string("Bob")}
    )

    charlie = relateby.pattern.Subject(
        identity="charlie",
        labels={"Person"},
        properties={"name": relateby.pattern.Value.string("Charlie")}
    )

    # Create Pattern[Subject] instances
    bob_pattern = relateby.pattern.Pattern.point(bob)
    charlie_pattern = relateby.pattern.Pattern.point(charlie)

    # Alice knows Bob and Charlie
    alice_pattern = relateby.pattern.Pattern.pattern(
        alice,
        [bob_pattern, charlie_pattern]
    )

    print(f"Alice identity: {alice_pattern.value.identity}")
    print(f"Alice knows {alice_pattern.length()} people")
    print(f"Total subjects: {alice_pattern.size()}")

    # Get all subjects in the pattern
    all_subjects = alice_pattern.values()
    print(f"All subjects: {[s.identity for s in all_subjects]}")
    print()


def main():
    """Run all examples."""
    print("\n" + "=" * 60)
    print("PATTERN-CORE PYTHON BINDINGS - BASIC USAGE EXAMPLES")
    print("=" * 60 + "\n")

    example_pattern_of_alias()
    example_from_values()
    example_atomic_pattern()
    example_nested_pattern()
    example_pattern_from_list()
    example_deeply_nested_pattern()
    example_value_types()
    example_subject_basic()
    example_subject_with_labels()
    example_subject_with_properties()
    example_pattern_subject()
    example_pattern_subject_nested()

    print("=" * 60)
    print("All basic usage examples completed successfully!")
    print("=" * 60)


if __name__ == "__main__":
    main()
