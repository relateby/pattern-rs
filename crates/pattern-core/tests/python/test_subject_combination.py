"""
Tests for Subject combination strategies
"""
import pytest
import pattern_core


def test_subject_combination_merge_strategy():
    """Test merge strategy (default) combines labels and properties"""
    subject1 = pattern_core.Subject(
        identity="alice",
        labels={"Person"},
        properties={"name": pattern_core.Value.string("Alice")}
    )
    subject2 = pattern_core.Subject(
        identity="bob",
        labels={"Employee"},
        properties={"role": pattern_core.Value.string("Engineer")}
    )

    p1 = pattern_core.Pattern.point(subject1)
    p2 = pattern_core.Pattern.point(subject2)

    # Default merge strategy
    merged = p1.combine(p2)

    result_subject = merged.value
    assert result_subject.identity == "alice"  # First identity
    assert "Person" in result_subject.get_labels()  # Union of labels
    assert "Employee" in result_subject.get_labels()
    assert "name" in result_subject.get_properties()  # Merged properties
    assert "role" in result_subject.get_properties()


@pytest.mark.skip(reason="Strategy parameter not yet implemented for generic Pattern.combine()")
def test_subject_combination_first_strategy():
    """Test first wins strategy"""
    # TODO: Re-enable when Pattern.combine() supports strategy parameter
    pass


@pytest.mark.skip(reason="Strategy parameter not yet implemented for generic Pattern.combine()")
def test_subject_combination_last_strategy():
    """Test last wins strategy"""
    # TODO: Re-enable when Pattern.combine() supports strategy parameter
    pass


@pytest.mark.skip(reason="Strategy parameter not yet implemented for generic Pattern.combine()")
def test_subject_combination_empty_strategy():
    """Test empty strategy returns anonymous subject"""
    # TODO: Re-enable when Pattern.combine() supports strategy parameter
    pass


@pytest.mark.skip(reason="Custom function parameter not yet implemented for generic Pattern.combine()")
def test_subject_combination_custom_function():
    """Test custom combination function"""
    # TODO: Re-enable when Pattern.combine() supports custom function parameter
    pass


def test_subject_combination_with_elements():
    """Test that combination concatenates elements"""
    subject1 = pattern_core.Subject(identity="s1", labels=set(), properties={})
    subject2 = pattern_core.Subject(identity="s2", labels=set(), properties={})
    subject3 = pattern_core.Subject(identity="s3", labels=set(), properties={})
    subject4 = pattern_core.Subject(identity="s4", labels=set(), properties={})

    p1 = pattern_core.Pattern.pattern(
        subject1,
        [pattern_core.Pattern.point(subject2)]
    )
    p2 = pattern_core.Pattern.pattern(
        subject3,
        [pattern_core.Pattern.point(subject4)]
    )

    merged = p1.combine(p2)

    # Should have 2 elements (concatenated)
    assert merged.length() == 2
    elements = merged.elements
    assert elements[0].value.identity == "s2"
    assert elements[1].value.identity == "s4"


def test_subject_combination_associativity():
    """Test that combination is associative for merge strategy"""
    s1 = pattern_core.Subject(identity="a", labels={"L1"}, properties={})
    s2 = pattern_core.Subject(identity="b", labels={"L2"}, properties={})
    s3 = pattern_core.Subject(identity="c", labels={"L3"}, properties={})

    p1 = pattern_core.Pattern.point(s1)
    p2 = pattern_core.Pattern.point(s2)
    p3 = pattern_core.Pattern.point(s3)

    # (p1 + p2) + p3
    left = p1.combine(p2).combine(p3)

    # p1 + (p2 + p3)
    right = p1.combine(p2.combine(p3))

    # Should have same identity and labels
    assert left.value.identity == right.value.identity
    assert left.value.get_labels() == right.value.get_labels()


@pytest.mark.skip(reason="Strategy parameter not yet implemented for generic Pattern.combine()")
def test_subject_combination_invalid_strategy():
    """Test that invalid strategy raises error"""
    # TODO: Re-enable when Pattern.combine() supports strategy parameter
    pass
