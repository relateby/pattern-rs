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
    
    p1 = pattern_core.PatternSubject.point(subject1)
    p2 = pattern_core.PatternSubject.point(subject2)
    
    # Default merge strategy
    merged = p1.combine(p2)
    
    result_subject = merged.get_value()
    assert result_subject.identity == "alice"  # First identity
    assert "Person" in result_subject.get_labels()  # Union of labels
    assert "Employee" in result_subject.get_labels()
    assert "name" in result_subject.get_properties()  # Merged properties
    assert "role" in result_subject.get_properties()


def test_subject_combination_first_strategy():
    """Test first wins strategy"""
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
    
    p1 = pattern_core.PatternSubject.point(subject1)
    p2 = pattern_core.PatternSubject.point(subject2)
    
    # First wins
    first = p1.combine(p2, strategy="first")
    
    result_subject = first.get_value()
    assert result_subject.identity == "alice"
    assert "Person" in result_subject.get_labels()
    assert "Employee" not in result_subject.get_labels()
    assert "name" in result_subject.get_properties()
    assert "role" not in result_subject.get_properties()


def test_subject_combination_last_strategy():
    """Test last wins strategy"""
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
    
    p1 = pattern_core.PatternSubject.point(subject1)
    p2 = pattern_core.PatternSubject.point(subject2)
    
    # Last wins
    last = p1.combine(p2, strategy="last")
    
    result_subject = last.get_value()
    assert result_subject.identity == "bob"
    assert "Person" not in result_subject.get_labels()
    assert "Employee" in result_subject.get_labels()
    assert "name" not in result_subject.get_properties()
    assert "role" in result_subject.get_properties()


def test_subject_combination_empty_strategy():
    """Test empty strategy returns anonymous subject"""
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
    
    p1 = pattern_core.PatternSubject.point(subject1)
    p2 = pattern_core.PatternSubject.point(subject2)
    
    # Empty strategy
    empty = p1.combine(p2, strategy="empty")
    
    result_subject = empty.get_value()
    assert result_subject.identity == "_"
    assert len(result_subject.get_labels()) == 0
    assert len(result_subject.get_properties()) == 0


def test_subject_combination_custom_function():
    """Test custom combination function"""
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
    
    p1 = pattern_core.PatternSubject.point(subject1)
    p2 = pattern_core.PatternSubject.point(subject2)
    
    # Custom function: take second identity but merge everything else
    def custom_merge(s1, s2):
        return pattern_core.Subject(
            identity=s2.identity,  # Use second identity
            labels=s1.get_labels() | s2.get_labels(),  # Union
            properties={**s1.get_properties(), **s2.get_properties()}  # Merge
        )
    
    custom = p1.combine(p2, combine_func=custom_merge)
    
    result_subject = custom.get_value()
    assert result_subject.identity == "bob"  # Custom logic
    assert "Person" in result_subject.get_labels()  # Union
    assert "Employee" in result_subject.get_labels()
    assert "name" in result_subject.get_properties()  # Merged
    assert "role" in result_subject.get_properties()


def test_subject_combination_with_elements():
    """Test that combination concatenates elements"""
    subject1 = pattern_core.Subject(identity="s1", labels=set(), properties={})
    subject2 = pattern_core.Subject(identity="s2", labels=set(), properties={})
    subject3 = pattern_core.Subject(identity="s3", labels=set(), properties={})
    subject4 = pattern_core.Subject(identity="s4", labels=set(), properties={})
    
    p1 = pattern_core.PatternSubject.pattern(
        subject1,
        [pattern_core.PatternSubject.point(subject2)]
    )
    p2 = pattern_core.PatternSubject.pattern(
        subject3,
        [pattern_core.PatternSubject.point(subject4)]
    )
    
    merged = p1.combine(p2)
    
    # Should have 2 elements (concatenated)
    assert merged.length() == 2
    elements = merged.get_elements()
    assert elements[0].get_value().identity == "s2"
    assert elements[1].get_value().identity == "s4"


def test_subject_combination_associativity():
    """Test that combination is associative for merge strategy"""
    s1 = pattern_core.Subject(identity="a", labels={"L1"}, properties={})
    s2 = pattern_core.Subject(identity="b", labels={"L2"}, properties={})
    s3 = pattern_core.Subject(identity="c", labels={"L3"}, properties={})
    
    p1 = pattern_core.PatternSubject.point(s1)
    p2 = pattern_core.PatternSubject.point(s2)
    p3 = pattern_core.PatternSubject.point(s3)
    
    # (p1 + p2) + p3
    left = p1.combine(p2).combine(p3)
    
    # p1 + (p2 + p3)
    right = p1.combine(p2.combine(p3))
    
    # Should have same identity and labels
    assert left.get_value().identity == right.get_value().identity
    assert left.get_value().get_labels() == right.get_value().get_labels()


def test_subject_combination_invalid_strategy():
    """Test that invalid strategy raises error"""
    subject1 = pattern_core.Subject(identity="s1", labels=set(), properties={})
    subject2 = pattern_core.Subject(identity="s2", labels=set(), properties={})
    
    p1 = pattern_core.PatternSubject.point(subject1)
    p2 = pattern_core.PatternSubject.point(subject2)
    
    with pytest.raises(RuntimeError, match="Unknown combination strategy"):
        p1.combine(p2, strategy="invalid")
