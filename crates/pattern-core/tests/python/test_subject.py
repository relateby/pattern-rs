"""
Tests for Subject construction and operations
"""


def test_subject_construction():
    """Test creating Subject with identity, labels, and properties"""
    import pattern_core
    
    # Create labels set
    labels_set = {"Person", "Employee"}
    
    # Create properties dict
    properties_dict = {
        "name": pattern_core.Value.string("Alice"),
        "age": pattern_core.Value.int(30)
    }
    
    subject = pattern_core.Subject(
        identity="alice",
        labels=labels_set,
        properties=properties_dict
    )
    
    assert subject.identity == "alice"
    labels = subject.get_labels()
    assert "Person" in labels
    assert "Employee" in labels
    props = subject.get_properties()
    assert props["name"] == "Alice"
    assert props["age"] == 30


def test_subject_labels():
    """Test Subject label operations"""
    import pattern_core
    
    subject = pattern_core.Subject(identity="test")
    
    subject.add_label("Person")
    assert subject.has_label("Person") is True
    
    subject.add_label("Employee")
    assert subject.has_label("Employee") is True
    
    subject.remove_label("Person")
    assert subject.has_label("Person") is False


def test_subject_properties():
    """Test Subject property operations"""
    import pattern_core
    
    subject = pattern_core.Subject(identity="test")
    
    subject.set_property("name", pattern_core.Value.string("Alice"))
    name_prop = subject.get_property("name")
    assert name_prop is not None
    assert name_prop.as_string() == "Alice"
    
    subject.set_property("age", pattern_core.Value.int(30))
    age_prop = subject.get_property("age")
    assert age_prop is not None
    assert age_prop.as_int() == 30
    
    subject.remove_property("age")
    assert subject.get_property("age") is None
