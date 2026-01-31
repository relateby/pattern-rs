"""
Tests for Pattern.para() - paramorphism operation

Validates structure-aware fold with access to pattern structure.
"""

import pattern_core


def test_para_atomic_pattern():
    """Test para on atomic pattern - receives empty element_results list"""
    atomic = pattern_core.Pattern.point(5)

    # Atomic pattern receives empty list
    result = atomic.para(lambda p, rs: p.value + sum(rs))

    assert result == 5, f"Expected 5, got {result}"


def test_para_simple_pattern():
    """Test para on simple pattern with atomic elements"""
    # Pattern: 1 with elements [2, 3]
    pattern = pattern_core.Pattern.pattern(1, [
        pattern_core.Pattern.point(2),
        pattern_core.Pattern.point(3)
    ])

    # Each element returns its value (2, 3)
    # Root receives value=1, element_results=[2, 3]
    # Result: 1 + (2 + 3) = 6
    result = pattern.para(lambda p, rs: p.value + sum(rs))

    assert result == 6, f"Expected 6, got {result}"


def test_para_nested_pattern():
    """Test para on nested pattern - depth-weighted sum"""
    # Pattern structure:
    #       1
    #      / \
    #     2   3
    #        /
    #       4
    pattern = pattern_core.Pattern.pattern(1, [
        pattern_core.Pattern.point(2),
        pattern_core.Pattern.pattern(3, [
            pattern_core.Pattern.point(4)
        ])
    ])

    # Evaluation (bottom-up, left-to-right):
    # 1. Pattern.point(4): value=4, rs=[] → 4 + 0 = 4
    # 2. Pattern.point(2): value=2, rs=[] → 2 + 0 = 2
    # 3. Pattern.pattern(3, [4]): value=3, rs=[4] → 3 + 4 = 7
    # 4. Pattern.pattern(1, [2, 7]): value=1, rs=[2, 7] → 1 + 2 + 7 = 10
    result = pattern.para(lambda p, rs: p.value + sum(rs))

    assert result == 10, f"Expected 10, got {result}"


def test_para_fold_parity():
    """Test that para can replicate fold behavior for simple value aggregation"""
    pattern = pattern_core.Pattern.pattern(10, [
        pattern_core.Pattern.point(20),
        pattern_core.Pattern.point(30),
        pattern_core.Pattern.pattern(40, [
            pattern_core.Pattern.point(50)
        ])
    ])

    # Para: value + sum of element results
    para_result = pattern.para(lambda p, rs: p.value + sum(rs))

    # Fold: accumulate all values
    fold_result = pattern.fold(0, lambda acc, v: acc + v)

    # Both should sum all values: 10 + 20 + 30 + 40 + 50 = 150
    assert para_result == fold_result, f"Para {para_result} != Fold {fold_result}"
    assert para_result == 150, f"Expected 150, got {para_result}"


def test_para_multi_statistics():
    """Test computing multiple statistics (sum, count, max_depth) in one pass"""
    # Pattern structure:
    #       1
    #      / \
    #     2   3
    #        /
    #       4
    pattern = pattern_core.Pattern.pattern(1, [
        pattern_core.Pattern.point(2),
        pattern_core.Pattern.pattern(3, [
            pattern_core.Pattern.point(4)
        ])
    ])

    def compute_stats(p, element_stats):
        """Returns (sum, count, max_depth)"""
        if not element_stats:
            # Atomic: just the value, count 1, depth 0
            return (p.value, 1, 0)

        # Aggregate element statistics
        elem_sum = sum(s[0] for s in element_stats)
        elem_count = sum(s[1] for s in element_stats)
        elem_max_depth = max(s[2] for s in element_stats)

        # Include current value and increment depth
        total_sum = p.value + elem_sum
        total_count = 1 + elem_count
        max_depth = 1 + elem_max_depth

        return (total_sum, total_count, max_depth)

    result = pattern.para(compute_stats)

    # Expected: sum=10 (1+2+3+4), count=4, max_depth=2 (root->3->4)
    assert result == (10, 4, 2), f"Expected (10, 4, 2), got {result}"


def test_para_structure_preserving_transformation():
    """Test structure-preserving transformation using para"""
    # Original pattern
    pattern = pattern_core.Pattern.pattern(1, [
        pattern_core.Pattern.point(2),
        pattern_core.Pattern.pattern(3, [
            pattern_core.Pattern.point(4)
        ])
    ])

    def double_values(p, transformed_elements):
        """Double the value, keep same structure"""
        new_value = p.value * 2
        if not transformed_elements:
            # Atomic: return new atomic pattern
            return pattern_core.Pattern.point(new_value)
        else:
            # Pattern: return new pattern with transformed elements
            return pattern_core.Pattern.pattern(new_value, transformed_elements)

    transformed = pattern.para(double_values)

    # Verify structure preserved
    assert transformed.value == 2, f"Root value should be 2, got {transformed.value}"
    assert len(transformed.elements) == 2, "Root should have 2 elements"

    # Check first element (atomic 2 -> 4)
    assert transformed.elements[0].value == 4
    assert transformed.elements[0].is_atomic()

    # Check second element (pattern 3 -> 6)
    assert transformed.elements[1].value == 6
    assert len(transformed.elements[1].elements) == 1
    assert transformed.elements[1].elements[0].value == 8


def test_para_access_to_pattern_structure():
    """Test that para callback has access to full pattern structure"""
    pattern = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.point("a"),
        pattern_core.Pattern.point("b")
    ])

    def check_structure(p, rs):
        """Verify we can access pattern structure"""
        # Check we can access value
        assert hasattr(p, 'value')

        # Check we can access elements
        assert hasattr(p, 'elements')

        # Check we can query structure
        assert hasattr(p, 'is_atomic')
        assert hasattr(p, 'length')

        # For root: not atomic, has 2 elements
        if p.value == "root":
            assert not p.is_atomic()
            assert p.length() == 2
        else:
            # For leaves: atomic, no elements
            assert p.is_atomic()
            assert p.length() == 0

        # Return a result
        return 1

    result = pattern.para(check_structure)
    assert result == 1


def test_para_with_subject_values():
    """Test para works with Pattern[Subject] (not PatternSubject)"""
    # Create subjects
    alice = pattern_core.Subject(
        identity="alice",
        labels={"Person"},
        properties={"age": pattern_core.Value.int(30)}
    )

    bob = pattern_core.Subject(
        identity="bob",
        labels={"Person"},
        properties={"age": pattern_core.Value.int(25)}
    )

    # Create pattern with Subject values
    pattern = pattern_core.Pattern.pattern(alice, [
        pattern_core.Pattern.point(bob)
    ])

    def count_people(p, counts):
        """Count how many Person subjects"""
        count = 1 if p.value.has_label("Person") else 0
        return count + sum(counts)

    result = pattern.para(count_people)
    assert result == 2, f"Expected 2 people, got {result}"


if __name__ == "__main__":
    # Run all tests
    test_para_atomic_pattern()
    test_para_simple_pattern()
    test_para_nested_pattern()
    test_para_fold_parity()
    test_para_multi_statistics()
    test_para_structure_preserving_transformation()
    test_para_access_to_pattern_structure()
    test_para_with_subject_values()

    print("✓ All paramorphism tests passed!")
