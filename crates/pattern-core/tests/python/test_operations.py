"""
Tests for Pattern operations (inspection, queries, transformations, combination, comonad)
"""
import pattern_core


def test_pattern_inspection():
    """Test Pattern inspection methods (length, size, depth, is_atomic, values)"""
    # Atomic pattern
    atomic = pattern_core.Pattern.point("hello")
    assert atomic.length() == 0
    assert atomic.size() == 1
    assert atomic.depth() == 0
    assert atomic.is_atomic() is True
    
    # Nested pattern
    child1 = pattern_core.Pattern.point("child1")
    child2 = pattern_core.Pattern.point("child2")
    parent = pattern_core.Pattern.pattern("parent", [child1, child2])
    
    assert parent.length() == 2
    assert parent.size() == 3  # parent + 2 children
    assert parent.depth() == 1
    assert parent.is_atomic() is False
    
    # Deeply nested
    grandchild = pattern_core.Pattern.point("grandchild")
    nested_child = pattern_core.Pattern.pattern("nested", [grandchild])
    deep_parent = pattern_core.Pattern.pattern("root", [nested_child])
    
    assert deep_parent.depth() == 2
    assert deep_parent.size() == 3  # root + nested + grandchild


def test_pattern_queries():
    """Test Pattern query methods (any_value, all_values, filter, find_first, matches, contains)"""
    # Create test pattern
    pattern = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.point("hello"),
        pattern_core.Pattern.point("world"),
        pattern_core.Pattern.point("python")
    ])
    
    # any_value - check if any value contains "o"
    assert pattern.any_value(lambda v: "o" in v) is True
    assert pattern.any_value(lambda v: "z" in v) is False
    
    # all_values - check if all values are strings
    assert pattern.all_values(lambda v: isinstance(v, str)) is True
    
    # filter - find patterns with value length > 4
    filtered = pattern.filter(lambda p: len(p.value) > 4)
    assert len(filtered) == 3  # "hello", "world", and "python"
    
    # find_first - find first pattern with value starting with "w"
    found = pattern.find_first(lambda p: p.value.startswith("w"))
    assert found is not None
    assert found.value == "world"
    
    # matches - check if patterns have identical structure
    pattern2 = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.point("hello"),
        pattern_core.Pattern.point("world"),
        pattern_core.Pattern.point("python")
    ])
    assert pattern.matches(pattern2) is True
    
    # contains - check if pattern contains subpattern
    subpattern = pattern_core.Pattern.point("hello")
    assert pattern.contains(subpattern) is True


def test_pattern_transformations():
    """Test Pattern transformation methods (map, fold)"""
    # Create test pattern
    pattern = pattern_core.Pattern.pattern("hello", [
        pattern_core.Pattern.point("world"),
        pattern_core.Pattern.point("python")
    ])
    
    # map - transform values to uppercase
    upper = pattern.map(str.upper)
    assert upper.value == "HELLO"
    assert upper.elements[0].value == "WORLD"
    assert upper.elements[1].value == "PYTHON"
    
    # fold - concatenate all values
    result = pattern.fold("", lambda acc, v: acc + v + " ")
    assert "hello world python " in result


def test_pattern_combination():
    """Test Pattern combination method"""
    # Combine atomic patterns
    p1 = pattern_core.Pattern.point("hello")
    p2 = pattern_core.Pattern.point(" world")
    combined = p1.combine(p2)
    
    # For string values, combination concatenates
    assert combined.value == "hello world"
    
    # Combine patterns with elements
    p3 = pattern_core.Pattern.pattern("a", [
        pattern_core.Pattern.point("b"),
        pattern_core.Pattern.point("c")
    ])
    p4 = pattern_core.Pattern.pattern("d", [
        pattern_core.Pattern.point("e")
    ])
    result = p3.combine(p4)
    assert result.value == "ad"
    assert len(result.elements) == 3  # b, c, e


def test_pattern_comonad():
    """Test Pattern comonad operations (extract, extend, depth_at, size_at, indices_at)"""
    # Create test pattern
    pattern = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.pattern("a", [
            pattern_core.Pattern.point("x")
        ]),
        pattern_core.Pattern.point("b")
    ])
    
    # extract - get value at current position
    assert pattern.extract() == "root"
    
    # extend - decorate with depth (returns integers, not strings)
    depths = pattern.extend(lambda p: p.depth())
    assert depths.value == 2  # root depth
    assert depths.elements[0].value == 1  # "a" depth
    assert depths.elements[1].value == 0  # "b" depth
    
    # depth_at - helper for depth decoration (returns integers, not strings)
    depths2 = pattern.depth_at()
    assert depths2.value == 2
    assert depths2.elements[0].value == 1
    
    # size_at - decorate with subtree size (returns integers, not strings)
    sizes = pattern.size_at()
    assert sizes.value == 4  # root + a + x + b
    assert sizes.elements[0].value == 2  # a + x
    assert sizes.elements[1].value == 1  # b
    
    # indices_at - decorate with path from root (returns list, not string)
    indices = pattern.indices_at()
    assert indices.value == []  # root path is empty list
    assert indices.elements[0].value == [0]  # "a" path
    assert indices.elements[1].value == [1]  # "b" path


def test_pattern_subject_operations():
    """Test PatternSubject operations"""
    # Create PatternSubject
    subject1 = pattern_core.Subject(
        identity="alice",
        labels={"Person"},
        properties={"name": pattern_core.Value.string("Alice")}
    )
    subject2 = pattern_core.Subject(
        identity="bob",
        labels={"Person"},
        properties={"name": pattern_core.Value.string("Bob")}
    )
    
    pattern = pattern_core.PatternSubject.pattern(
        subject1,
        [pattern_core.PatternSubject.point(subject2)]
    )
    
    # Inspection methods
    assert pattern.length() == 1
    assert pattern.size() == 2
    assert pattern.depth() == 1
    
    # Query methods
    assert pattern.any_value(lambda s: s.identity == "bob") is True
    assert pattern.all_values(lambda s: "Person" in s.get_labels()) is True
    
    # Map operation
    def add_label(s):
        s.add_label("Employee")
        return s
    
    mapped = pattern.map(add_label)
    assert mapped.get_value().has_label("Employee") is True
