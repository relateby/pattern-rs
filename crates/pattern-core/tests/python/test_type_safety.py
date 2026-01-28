"""
Type safety tests for pattern_core Python bindings.

These tests verify that type hints work correctly with static type checkers
(mypy, pyright) and that the API provides good type inference.

Note: Some assertions use values that type checkers infer as Optional (e.g. result
of Value.string, Pattern.point). Those usages are intentional to validate that
type hints and narrowing work correctly.
"""

from typing import List, Optional

import pytest

# These imports will be used by type checkers
try:
    from pattern_core import (
        Pattern,
        PatternSubject,
        Subject,
        Value,
        ValidationRules,
        ValidationError,
        StructureAnalysis,
    )
    PATTERN_CORE_AVAILABLE = True
except ImportError:
    PATTERN_CORE_AVAILABLE = False
    # Provide mock classes for type checking when module isn't built yet
    class Value:  # type: ignore
        @staticmethod
        def string(s: str) -> 'Value': ...
        @staticmethod
        def int(i: int) -> 'Value': ...
        @staticmethod
        def decimal(f: float) -> 'Value': ...
        @staticmethod
        def boolean(b: bool) -> 'Value': ...
        @staticmethod
        def symbol(s: str) -> 'Value': ...
        @staticmethod
        def array(items: list) -> 'Value': ...
    
    class Subject:  # type: ignore
        def __init__(self, identity: str, labels: Optional[set] = None, properties: Optional[dict] = None): ...
    
    class Pattern:  # type: ignore
        @staticmethod
        def point(value) -> 'Pattern': ...
        @staticmethod
        def pattern(value, elements: List['Pattern']) -> 'Pattern': ...
        @staticmethod
        def from_values(values: list) -> List['Pattern']: ...
    
    class PatternSubject:  # type: ignore
        @staticmethod
        def point(subject: Subject) -> 'PatternSubject': ...
        @staticmethod
        def pattern(subject: Subject, elements: list) -> 'PatternSubject': ...
    
    class ValidationRules:  # type: ignore
        def __init__(self, max_depth: Optional[int] = None, max_elements: Optional[int] = None): ...
    
    class ValidationError(ValueError):  # type: ignore
        pass
    
    class StructureAnalysis:  # type: ignore
        pass

    pytest.skip("pattern_core not built", allow_module_level=True)


def test_value_type_annotations() -> None:
    """Test that Value class has correct type annotations."""
    # Type checkers should verify these calls are valid
    str_val: Value = Value.string("hello")
    int_val: Value = Value.int(42)
    decimal_val: Value = Value.decimal(3.14)
    bool_val: Value = Value.boolean(True)
    symbol_val: Value = Value.symbol("alice")
    
    # These should type-check correctly
    assert isinstance(str_val, Value) or not PATTERN_CORE_AVAILABLE
    assert isinstance(int_val, Value) or not PATTERN_CORE_AVAILABLE
    assert isinstance(decimal_val, Value) or not PATTERN_CORE_AVAILABLE
    assert isinstance(bool_val, Value) or not PATTERN_CORE_AVAILABLE
    assert isinstance(symbol_val, Value) or not PATTERN_CORE_AVAILABLE


def test_value_array_and_map_types() -> None:
    """Test that Value array and map types are correctly typed."""
    # Type checkers should verify these
    array_val: Value = Value.array([Value.int(1), Value.int(2)])
    map_val: Value = Value.map({"key": Value.string("value")})
    range_val: Value = Value.range(lower=0.0, upper=100.0)
    measurement_val: Value = Value.measurement(42.5, "meters")
    
    assert isinstance(array_val, Value) or not PATTERN_CORE_AVAILABLE
    assert isinstance(map_val, Value) or not PATTERN_CORE_AVAILABLE
    assert isinstance(range_val, Value) or not PATTERN_CORE_AVAILABLE
    assert isinstance(measurement_val, Value) or not PATTERN_CORE_AVAILABLE


def test_subject_type_annotations() -> None:
    """Test that Subject class has correct type annotations."""
    # Type checkers should verify parameter types
    subject: Subject = Subject(
        identity="alice",
        labels={"Person", "Employee"},
        properties={"name": Value.string("Alice"), "age": Value.int(30)}
    )
    
    assert isinstance(subject, Subject) or not PATTERN_CORE_AVAILABLE


def test_pattern_construction_types() -> None:
    """Test that Pattern construction methods have correct type signatures."""
    # Type checkers should infer Pattern type correctly
    atomic: Pattern = Pattern.point("hello")
    nested: Pattern = Pattern.pattern("parent", [Pattern.point("child")])
    from_list: Pattern = Pattern.pattern("root", Pattern.from_values(["a", "b", "c"]))
    
    assert isinstance(atomic, Pattern) or not PATTERN_CORE_AVAILABLE
    assert isinstance(nested, Pattern) or not PATTERN_CORE_AVAILABLE
    assert isinstance(from_list, Pattern) or not PATTERN_CORE_AVAILABLE


def test_pattern_operations_types() -> None:
    """Test that Pattern operations have correct type signatures."""
    pattern: Pattern = Pattern.point("hello")
    
    # Type checkers should verify return types
    length: int = pattern.length()
    size: int = pattern.size()
    depth: int = pattern.depth()
    is_atomic: bool = pattern.is_atomic()
    values: List[str] = pattern.values()
    elements: List[Pattern] = pattern.elements
    
    # Type checkers should verify callback signatures
    any_result: bool = pattern.any_value(lambda v: v == "hello")
    all_result: bool = pattern.all_values(lambda v: isinstance(v, str))
    filtered: List[Pattern] = pattern.filter(lambda p: p.is_atomic())
    found: Optional[Pattern] = pattern.find_first(lambda p: p.value == "hello")
    
    assert isinstance(length, int) or not PATTERN_CORE_AVAILABLE
    assert isinstance(size, int) or not PATTERN_CORE_AVAILABLE
    assert isinstance(depth, int) or not PATTERN_CORE_AVAILABLE
    assert isinstance(is_atomic, bool) or not PATTERN_CORE_AVAILABLE
    assert isinstance(values, list) or not PATTERN_CORE_AVAILABLE
    assert isinstance(elements, list) or not PATTERN_CORE_AVAILABLE


def test_pattern_transformation_types() -> None:
    """Test that Pattern transformation methods have correct type signatures."""
    pattern: Pattern = Pattern.point("hello")
    
    # Type checkers should verify transformation signatures
    mapped: Pattern = pattern.map(str.upper)
    folded: str = pattern.fold("", lambda acc, val: acc + str(val))
    combined: Pattern = pattern.combine(Pattern.point("world"))
    
    assert isinstance(mapped, Pattern) or not PATTERN_CORE_AVAILABLE
    assert isinstance(folded, str) or not PATTERN_CORE_AVAILABLE
    assert isinstance(combined, Pattern) or not PATTERN_CORE_AVAILABLE


def test_pattern_comonad_types() -> None:
    """Test that Pattern comonad operations have correct type signatures."""
    pattern: Pattern = Pattern.point("hello")
    
    # Type checkers should verify comonad signatures
    extracted: str = pattern.extract()
    extended: Pattern = pattern.extend(lambda p: p.value)
    depth_at: Pattern = pattern.depth_at()
    size_at: Pattern = pattern.size_at()
    indices_at: Pattern = pattern.indices_at()
    
    assert isinstance(extracted, str) or not PATTERN_CORE_AVAILABLE
    assert isinstance(extended, Pattern) or not PATTERN_CORE_AVAILABLE
    assert isinstance(depth_at, Pattern) or not PATTERN_CORE_AVAILABLE
    assert isinstance(size_at, Pattern) or not PATTERN_CORE_AVAILABLE
    assert isinstance(indices_at, Pattern) or not PATTERN_CORE_AVAILABLE


def test_pattern_subject_types() -> None:
    """Test that PatternSubject has correct type signatures."""
    subject: Subject = Subject(identity="alice", labels={"Person"})
    
    # Type checkers should verify PatternSubject types
    atomic: PatternSubject = PatternSubject.point(subject)
    nested: PatternSubject = PatternSubject.pattern(subject, [atomic])
    
    # Type checkers should verify Subject-specific methods
    value: Subject = atomic.get_value()
    elements: List[PatternSubject] = nested.get_elements()
    values: List[Subject] = nested.values()
    
    assert isinstance(atomic, PatternSubject) or not PATTERN_CORE_AVAILABLE
    assert isinstance(nested, PatternSubject) or not PATTERN_CORE_AVAILABLE
    assert isinstance(value, Subject) or not PATTERN_CORE_AVAILABLE
    assert isinstance(elements, list) or not PATTERN_CORE_AVAILABLE
    assert isinstance(values, list) or not PATTERN_CORE_AVAILABLE


def test_validation_types() -> None:
    """Test that validation classes have correct type signatures."""
    # Type checkers should verify ValidationRules construction
    rules: ValidationRules = ValidationRules(max_depth=10, max_elements=100)
    
    pattern: Pattern = Pattern.point("hello")
    
    # Type checkers should verify validate method signature
    try:
        pattern.validate(rules)
    except ValidationError as e:
        # Type checkers should verify ValidationError properties
        message: str = e.message
        rule: str = e.rule
        location: List[str] = e.location
        assert isinstance(message, str) or not PATTERN_CORE_AVAILABLE
        assert isinstance(rule, str) or not PATTERN_CORE_AVAILABLE
        assert isinstance(location, list) or not PATTERN_CORE_AVAILABLE


def test_structure_analysis_types() -> None:
    """Test that StructureAnalysis has correct type signatures."""
    pattern: Pattern = Pattern.point("hello")
    
    # Type checkers should verify StructureAnalysis properties
    analysis: StructureAnalysis = pattern.analyze_structure()
    
    summary: str = analysis.summary
    depth_dist: List[int] = analysis.depth_distribution
    elem_counts: List[int] = analysis.element_counts
    nesting: List[str] = analysis.nesting_patterns
    
    assert isinstance(analysis, StructureAnalysis) or not PATTERN_CORE_AVAILABLE
    assert isinstance(summary, str) or not PATTERN_CORE_AVAILABLE
    assert isinstance(depth_dist, list) or not PATTERN_CORE_AVAILABLE
    assert isinstance(elem_counts, list) or not PATTERN_CORE_AVAILABLE
    assert isinstance(nesting, list) or not PATTERN_CORE_AVAILABLE


def test_type_checking_validation() -> None:
    """
    Test cases that should be caught by static type checkers.
    
    These are intentionally commented out because they would fail type checking.
    Uncomment to verify type checker is working correctly.
    """
    # The following should produce type errors in mypy/pyright:
    
    # Type error: string expected, got int
    # bad_value: Value = Value.string(42)  # type: ignore
    
    # Type error: int expected, got string
    # bad_int: Value = Value.int("hello")  # type: ignore
    
    # Type error: Pattern expected in list, got string
    # bad_pattern: Pattern = Pattern.pattern("root", ["not", "patterns"])  # type: ignore
    
    # Type error: Subject expected, got string
    # bad_subject: PatternSubject = PatternSubject.point("not_a_subject")  # type: ignore
    
    # Type error: callback should return bool, not string
    # pattern = Pattern.point("hello")
    # bad_filter = pattern.any_value(lambda v: "string_not_bool")  # type: ignore
    
    # Type error: wrong callback signature for map
    # bad_map = pattern.map(lambda p: p)  # type: ignore  # should take value, not pattern
    
    pass


if __name__ == "__main__":
    # Run tests
    if not PATTERN_CORE_AVAILABLE:
        print("WARNING: pattern_core module not available. Tests will only verify type annotations.")
        print("Build the module with 'maturin develop' to run runtime tests.")
    
    print("Running type safety tests...")
    test_value_type_annotations()
    test_value_array_and_map_types()
    test_subject_type_annotations()
    test_pattern_construction_types()
    test_pattern_operations_types()
    test_pattern_transformation_types()
    test_pattern_comonad_types()
    test_pattern_subject_types()
    test_validation_types()
    test_structure_analysis_types()
    test_type_checking_validation()
    
    print("✓ All type safety tests passed!")
    print("\nTo verify type checking, run:")
    print("  mypy tests/python/test_type_safety.py")
    print("  pyright tests/python/test_type_safety.py")
