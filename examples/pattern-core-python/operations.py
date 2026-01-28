#!/usr/bin/env python3
"""
Pattern operations examples for pattern-core Python bindings.

Demonstrates:
- Query operations (any_value, all_values, filter, find_first)
- Transformation operations (map, fold, combine)
- Pattern matching (matches, contains)
- Value extraction and aggregation
"""

import sys

try:
    import pattern_core
except ImportError:
    print("ERROR: pattern_core module not found.")
    print("Build it with: cd crates/pattern-core && maturin develop --uv --features python")
    sys.exit(1)


def example_map():
    """Transform pattern values with map."""
    print("=" * 60)
    print("Example 1: Map - Transform Values")
    print("=" * 60)
    
    # Create pattern
    pattern = pattern_core.Pattern.pattern("hello", pattern_core.Pattern.from_values(["world", "python", "patterns"]))
    
    print(f"Original values: {pattern.values()}")
    
    # Transform to uppercase
    upper = pattern.map(str.upper)
    print(f"Uppercase values: {upper.values()}")
    
    # Transform to length
    lengths = pattern.map(len)
    print(f"Length values: {lengths.values()}")
    print()


def example_filter():
    """Filter patterns by predicate."""
    print("=" * 60)
    print("Example 2: Filter - Select Patterns")
    print("=" * 60)
    
    # Create pattern
    pattern = pattern_core.Pattern.pattern("numbers", pattern_core.Pattern.from_values(["1", "2", "3", "4", "5"]))
    
    print(f"All values: {pattern.values()}")
    
    # Filter patterns with even numbers
    evens = pattern.filter(lambda p: int(p.value) % 2 == 0)
    print(f"Even numbers: {[int(p.value) for p in evens]}")
    
    # Filter atomic patterns only
    atomic_only = pattern.filter(lambda p: p.is_atomic())
    print(f"Atomic patterns count: {len(atomic_only)}")
    print()


def example_any_all():
    """Check values with any_value and all_values."""
    print("=" * 60)
    print("Example 3: Any/All - Query Values")
    print("=" * 60)
    
    # Create pattern
    pattern = pattern_core.Pattern.pattern("data", pattern_core.Pattern.from_values(["apple", "apricot", "avocado"]))
    
    print(f"Values: {pattern.values()}")
    
    # Check if any value starts with 'a'
    has_a = pattern.any_value(lambda v: v.startswith("a"))
    print(f"Any value starts with 'a': {has_a}")
    
    # Check if all values are strings
    all_strings = pattern.all_values(lambda v: isinstance(v, str))
    print(f"All values are strings: {all_strings}")
    
    # Check if all values have length > 5
    all_long = pattern.all_values(lambda v: len(v) > 5)
    print(f"All values longer than 5 chars: {all_long}")
    print()


def example_find_first():
    """Find first matching pattern."""
    print("=" * 60)
    print("Example 4: Find First - Search Patterns")
    print("=" * 60)
    
    # Create pattern
    pattern = pattern_core.Pattern.pattern("fruits", pattern_core.Pattern.from_values([
        "apple", "banana", "cherry", "date", "elderberry"
    ]))
    
    print(f"All fruits: {pattern.values()}")
    
    # Find first fruit starting with 'c'
    found = pattern.find_first(lambda p: p.value.startswith("c"))
    if found:
        print(f"First fruit starting with 'c': {found.value}")
    else:
        print("No fruit found starting with 'c'")
    
    # Find first fruit with length > 6
    found_long = pattern.find_first(lambda p: len(p.value) > 6)
    if found_long:
        print(f"First fruit longer than 6 chars: {found_long.value}")
    print()


def example_fold():
    """Fold over pattern values - aggregating Subject properties."""
    print("=" * 60)
    print("Example 5: Fold - Aggregate Subject Properties")
    print("=" * 60)
    
    # Create a Team entity (abstract concept, not a person)
    team_entity = pattern_core.Subject(
        identity="engineering_team",
        labels={"Team", "Department"},
        properties={
            "name": pattern_core.Value.string("Engineering Team"),
            "total_budget": pattern_core.Value.int(0),  # Will be set to aggregate
            "headcount": pattern_core.Value.int(0)       # Will be set to count
        }
    )
    
    # Create team members
    manager = pattern_core.Subject(
        identity="manager",
        labels={"Person", "Manager"},
        properties={
            "name": pattern_core.Value.string("Diana"),
            "salary": pattern_core.Value.int(150000)
        }
    )
    
    dev1 = pattern_core.Subject(
        identity="dev1",
        labels={"Person", "Developer"},
        properties={
            "name": pattern_core.Value.string("Alice"),
            "salary": pattern_core.Value.int(120000)
        }
    )
    
    dev2 = pattern_core.Subject(
        identity="dev2",
        labels={"Person", "Developer"},
        properties={
            "name": pattern_core.Value.string("Bob"),
            "salary": pattern_core.Value.int(115000)
        }
    )
    
    dev3 = pattern_core.Subject(
        identity="dev3",
        labels={"Person", "Developer"},
        properties={
            "name": pattern_core.Value.string("Charlie"),
            "salary": pattern_core.Value.int(110000)
        }
    )
    
    # Create pattern hierarchy: Team contains Manager and Developers
    team = pattern_core.PatternSubject.pattern(team_entity, [
        pattern_core.PatternSubject.point(manager),
        pattern_core.PatternSubject.point(dev1),
        pattern_core.PatternSubject.point(dev2),
        pattern_core.PatternSubject.point(dev3)
    ])
    
    print(f"Team: {team_entity.get_property('name').as_string()}")
    print(f"Team structure size: {team.size()} entities")
    
    # Aggregate: Sum all salaries (including manager)
    def sum_salaries(acc, subject):
        salary_prop = subject.get_property("salary")
        if salary_prop:
            return acc + salary_prop.as_int()
        return acc
    
    total_budget = team.fold(0, sum_salaries)
    print(f"Total salary budget: ${total_budget:,}")
    
    # Aggregate: Count people (not the Team entity)
    def count_people(acc, subject):
        if subject.has_label("Person"):
            return acc + 1
        return acc
    
    headcount = team.fold(0, count_people)
    print(f"Headcount: {headcount} people")
    
    # Aggregate: Count developers specifically
    def count_devs(acc, subject):
        if subject.has_label("Developer"):
            return acc + 1
        return acc
    
    dev_count = team.fold(0, count_devs)
    print(f"Developers: {dev_count}")
    
    # Now update the Team entity's properties with aggregated values
    team_entity.set_property("total_budget", pattern_core.Value.int(total_budget))
    team_entity.set_property("headcount", pattern_core.Value.int(headcount))
    
    print(f"\nTeam entity updated:")
    print(f"  - Total budget: ${team_entity.get_property('total_budget').as_int():,}")
    print(f"  - Headcount: {team_entity.get_property('headcount').as_int()}")
    print()


def example_combine():
    """Combine patterns."""
    print("=" * 60)
    print("Example 6: Combine - Merge Patterns")
    print("=" * 60)
    
    # Create two patterns
    pattern1 = pattern_core.Pattern.point("hello")
    pattern2 = pattern_core.Pattern.point(" world")
    
    print(f"Pattern 1: {pattern1.value}")
    print(f"Pattern 2: {pattern2.value}")
    
    # Combine patterns
    combined = pattern1.combine(pattern2)
    print(f"Combined: {combined.value}")
    print()


def example_matches():
    """Check if patterns match structurally."""
    print("=" * 60)
    print("Example 7: Matches - Structural Equality")
    print("=" * 60)
    
    # Create patterns with same structure
    pattern1 = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.point("a"),
        pattern_core.Pattern.point("b")
    ])
    
    pattern2 = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.point("a"),
        pattern_core.Pattern.point("b")
    ])
    
    pattern3 = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.point("x"),
        pattern_core.Pattern.point("y")
    ])
    
    print(f"Pattern 1 values: {pattern1.values()}")
    print(f"Pattern 2 values: {pattern2.values()}")
    print(f"Pattern 3 values: {pattern3.values()}")
    
    print(f"Pattern1 matches Pattern2: {pattern1.matches(pattern2)}")
    print(f"Pattern1 matches Pattern3: {pattern1.matches(pattern3)}")
    print()


def example_contains():
    """Check if pattern contains subpattern."""
    print("=" * 60)
    print("Example 8: Contains - Subpattern Search")
    print("=" * 60)
    
    # Create decorated pattern (value decorates the elements)
    decorated = pattern_core.Pattern.pattern("decoration", [
        pattern_core.Pattern.point("elem1"),
        pattern_core.Pattern.point("elem2"),
        pattern_core.Pattern.point("elem3")
    ])
    
    # Create search patterns
    elem1 = pattern_core.Pattern.point("elem1")
    elem4 = pattern_core.Pattern.point("elem4")
    
    print(f"Pattern values: {decorated.values()}")
    
    print(f"Contains 'elem1': {decorated.contains(elem1)}")
    print(f"Contains 'elem4': {decorated.contains(elem4)}")
    print()


def example_values_extraction():
    """Extract all values from pattern."""
    print("=" * 60)
    print("Example 9: Values - Extract All Values")
    print("=" * 60)
    
    # Create nested pattern
    pattern = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.pattern("branch1", [
            pattern_core.Pattern.point("leaf1"),
            pattern_core.Pattern.point("leaf2")
        ]),
        pattern_core.Pattern.point("leaf3")
    ])
    
    # Get all values (pre-order traversal)
    all_values = pattern.values()
    print(f"All values: {all_values}")
    print(f"Total values: {len(all_values)}")
    print()


def example_structural_properties():
    """Inspect pattern structural properties."""
    print("=" * 60)
    print("Example 10: Structural Properties")
    print("=" * 60)
    
    # Create pattern
    pattern = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.pattern("branch", [
            pattern_core.Pattern.point("leaf1"),
            pattern_core.Pattern.point("leaf2")
        ]),
        pattern_core.Pattern.point("leaf3")
    ])
    
    print(f"Length (direct elements): {pattern.length()}")
    print(f"Size (total nodes): {pattern.size()}")
    print(f"Depth (max nesting): {pattern.depth()}")
    print(f"Is atomic: {pattern.is_atomic()}")
    print()


def example_complex_transformation():
    """Complex transformation pipeline."""
    print("=" * 60)
    print("Example 11: Complex Transformation Pipeline")
    print("=" * 60)
    
    # Create data pattern
    data = pattern_core.Pattern.pattern("data", pattern_core.Pattern.from_values([
        "apple", "banana", "cherry", "date", "elderberry", "fig"
    ]))
    
    print(f"Original data: {data.values()[1:]}")  # Skip root "data"
    
    # Transform: uppercase
    step1 = data.map(str.upper)
    print(f"Step 1 (uppercase): {step1.values()[1:]}")
    
    # Filter: keep only words with 'E'
    step2_patterns = step1.filter(lambda p: 'E' in p.value)
    print(f"Step 2 (contains 'E'): {[p.value for p in step2_patterns]}")
    
    # Fold: concatenate with commas
    if step2_patterns:
        # Create pattern from filtered results
        filtered_pattern = pattern_core.Pattern.pattern(
            "filtered",
            step2_patterns
        )
        result = filtered_pattern.fold("", lambda acc, val: 
            acc + ("," if acc and val != "filtered" else "") + (val if val != "filtered" else "")
        )
        print(f"Step 3 (concatenated): {result}")
    print()


def example_pattern_subject_operations():
    """Operations on PatternSubject."""
    print("=" * 60)
    print("Example 12: PatternSubject Operations")
    print("=" * 60)
    
    # Create Subjects
    alice = pattern_core.Subject(
        identity="alice",
        labels={"Person", "Employee"},
        properties={"name": pattern_core.Value.string("Alice")}
    )
    
    bob = pattern_core.Subject(
        identity="bob",
        labels={"Person", "Manager"},
        properties={"name": pattern_core.Value.string("Bob")}
    )
    
    charlie = pattern_core.Subject(
        identity="charlie",
        labels={"Person", "Employee"},
        properties={"name": pattern_core.Value.string("Charlie")}
    )
    
    # Create pattern
    bob_pattern = pattern_core.PatternSubject.point(bob)
    charlie_pattern = pattern_core.PatternSubject.point(charlie)
    alice_pattern = pattern_core.PatternSubject.pattern(alice, [bob_pattern, charlie_pattern])
    
    print(f"Total subjects: {alice_pattern.size()}")
    
    # Query: Find all employees
    employees = alice_pattern.filter(lambda p: p.get_value().has_label("Employee"))
    print(f"Employees: {[p.get_value().identity for p in employees]}")
    
    # Query: Check if any manager exists
    has_manager = alice_pattern.any_value(lambda s: s.has_label("Manager"))
    print(f"Has manager: {has_manager}")
    
    # Map: Transform subjects (add a label)
    def add_verified_label(subject):
        subject.add_label("Verified")
        return subject
    
    verified = alice_pattern.map(add_verified_label)
    all_subjects = verified.values()
    print(f"All subjects have Verified label: {all([s.has_label('Verified') for s in all_subjects])}")
    print()


def main():
    """Run all examples."""
    print("\n" + "=" * 60)
    print("PATTERN-CORE PYTHON BINDINGS - OPERATIONS EXAMPLES")
    print("=" * 60 + "\n")
    
    example_map()
    example_filter()
    example_any_all()
    example_find_first()
    example_fold()
    example_combine()
    example_matches()
    example_contains()
    example_values_extraction()
    example_structural_properties()
    example_complex_transformation()
    example_pattern_subject_operations()
    
    print("=" * 60)
    print("All operations examples completed successfully!")
    print("=" * 60)


if __name__ == "__main__":
    main()
