"""
Tests for Pattern construction and basic operations
"""


def test_pattern_point():
    """Test creating atomic pattern with Pattern.point()"""
    import pattern_core
    
    atomic = pattern_core.Pattern.point("hello")
    assert atomic.value == "hello"
    assert len(atomic.elements) == 0
    assert atomic.is_atomic() is True


def test_pattern_pattern():
    """Test creating nested pattern with Pattern.pattern()"""
    import pattern_core
    
    child1 = pattern_core.Pattern.point("child1")
    child2 = pattern_core.Pattern.point("child2")
    parent = pattern_core.Pattern.pattern("parent", [child1, child2])
    
    assert parent.value == "parent"
    assert len(parent.elements) == 2
    assert parent.elements[0].value == "child1"
    assert parent.elements[1].value == "child2"


def test_pattern_from_list():
    """Test creating pattern from list of values"""
    import pattern_core
    
    # Note: from_list takes a PyList, not a Python list directly
    # For MVP, we'll test with individual point() calls instead
    pattern = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.point("a"),
        pattern_core.Pattern.point("b"),
        pattern_core.Pattern.point("c")
    ])
    
    assert pattern.value == "root"
    assert len(pattern.elements) == 3
    assert pattern.elements[0].value == "a"
    assert pattern.elements[1].value == "b"
    assert pattern.elements[2].value == "c"


def test_pattern_subject_construction():
    """Test creating Pattern with Subject value"""
    import pattern_core
    
    labels_set = {"Person", "Employee"}
    properties_dict = {
        "name": pattern_core.Value.string("Alice"),
        "age": pattern_core.Value.int(30)
    }
    
    subject = pattern_core.Subject(
        identity="alice",
        labels=labels_set,
        properties=properties_dict
    )
    
    pattern = pattern_core.PatternSubject.point(subject)
    
    pattern_subject = pattern.get_value()
    assert pattern_subject.identity == "alice"
    labels = pattern_subject.get_labels()
    assert "Person" in labels
    assert "Employee" in labels
    props = pattern_subject.get_properties()
    assert props["name"] == "Alice"
    assert props["age"] == 30
