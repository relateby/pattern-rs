#!/usr/bin/env python3
"""
Type safety examples for pattern-core Python bindings.

Demonstrates:
- Type hints for Pattern, Subject, Value classes
- Using type checkers (mypy, pyright)
- Generic types and type variables
- Type-safe callbacks and transformations
- Optional return values

Run with type checkers:
  mypy type_safety.py
  pyright type_safety.py
"""

import sys
from typing import List, Optional, Callable, Any

try:
    from pattern_core import (
    Pattern,
    PatternSubject,
    Subject,
    Value,
    ValidationError,
    ValidationRules,
)
except ImportError:
    print("ERROR: pattern_core module not found.")
    print("Build it with: cd crates/pattern-core && maturin develop --uv --features python")
    sys.exit(1)


def example_typed_pattern_construction() -> None:
    """Example with type-annotated pattern construction."""
    print("=" * 60)
    print("Example 1: Type-Annotated Pattern Construction")
    print("=" * 60)
    
    # Type checkers verify these are Pattern instances
    atomic: Pattern = Pattern.point("hello")
    nested: Pattern = Pattern.pattern("root", [atomic])
    from_values_pattern: Pattern = Pattern.pattern("data", Pattern.from_values(["a", "b", "c"]))
    
    print(f"Atomic value: {atomic.value}")
    print(f"Nested length: {nested.length()}")
    print(f"From values size: {from_values_pattern.size()}")
    print()


def example_typed_subject_construction() -> None:
    """Example with type-annotated Subject construction."""
    print("=" * 60)
    print("Example 2: Type-Annotated Subject Construction")
    print("=" * 60)
    
    # Type checkers verify parameter types
    subject: Subject = Subject(
        identity="alice",
        labels={"Person", "Employee"},
        properties={
            "name": Value.string("Alice"),
            "age": Value.int(30)
        }
    )
    
    # Type checkers know get_labels returns Set[str]
    labels: set = subject.get_labels()
    print(f"Labels: {labels}")
    
    # Type checkers know get_property returns Optional[Value]
    name_value: Optional[Value] = subject.get_property("name")
    if name_value:  # Type narrowing - checker knows it's Value here
        name: str = name_value.as_string()
        print(f"Name: {name}")
    print()


def example_typed_operations() -> None:
    """Example with type-annotated pattern operations."""
    print("=" * 60)
    print("Example 3: Type-Annotated Operations")
    print("=" * 60)
    
    pattern: Pattern = Pattern.pattern("data", Pattern.from_values(["a", "b", "c"]))
    
    # Type checkers verify return types
    length: int = pattern.length()
    size: int = pattern.size()
    depth: int = pattern.depth()
    is_atomic: bool = pattern.is_atomic()
    
    print(f"Length: {length}, Size: {size}, Depth: {depth}, Atomic: {is_atomic}")
    
    # Type checkers verify callback signatures
    has_a: bool = pattern.any_value(lambda v: v == "a")
    all_str: bool = pattern.all_values(lambda v: isinstance(v, str))
    
    print(f"Has 'a': {has_a}, All strings: {all_str}")
    print()


def example_typed_callbacks() -> None:
    """Example with type-annotated callbacks."""
    print("=" * 60)
    print("Example 4: Type-Annotated Callbacks")
    print("=" * 60)
    
    pattern: Pattern = Pattern.pattern("data", Pattern.from_values(["hello", "world"]))
    
    # Type checker verifies callback signature: Callable[[Any], Any]
    def to_upper(value: Any) -> str:
        return str(value).upper()
    
    mapped: Pattern = pattern.map(to_upper)
    print(f"Mapped values: {mapped.values()}")
    
    # Type checker verifies fold callback: Callable[[Any, Any], Any]
    def concat(acc: str, val: Any) -> str:
        return acc + str(val)
    
    result: str = pattern.fold("", concat)
    print(f"Folded result: {result}")
    
    # Type checker verifies filter callback: Callable[[Pattern], bool]
    def is_data_node(p: Pattern) -> bool:
        return p.value == "data"
    
    filtered: List[Pattern] = pattern.filter(is_data_node)
    print(f"Filtered count: {len(filtered)}")
    print()


def example_typed_pattern_subject() -> None:
    """Example with type-annotated PatternSubject."""
    print("=" * 60)
    print("Example 5: Type-Annotated PatternSubject")
    print("=" * 60)
    
    # Create typed Subject
    subject: Subject = Subject(
        identity="alice",
        labels={"Person"}
    )
    
    # Type checkers know this is PatternSubject
    pattern: PatternSubject = PatternSubject.point(subject)
    
    # Type checkers know get_value returns Subject
    value: Subject = pattern.get_value()
    print(f"Subject identity: {value.identity}")
    
    # Type checkers know values returns List[Subject]
    subjects: List[Subject] = pattern.values()
    print(f"Total subjects: {len(subjects)}")
    print()


def example_optional_handling() -> None:
    """Example with proper Optional handling."""
    print("=" * 60)
    print("Example 6: Optional Type Handling")
    print("=" * 60)
    
    pattern: Pattern = Pattern.pattern("data", Pattern.from_values(["a", "b", "c"]))
    
    # Type checker knows find_first returns Optional[Pattern]
    found: Optional[Pattern] = pattern.find_first(lambda p: p.value == "b")
    
    # Proper Optional handling with type narrowing
    if found is not None:
        # Type checker knows found is Pattern here (not None)
        value: str = found.value
        print(f"Found value: {value}")
    else:
        print("Not found")
    
    # Subject property access with Optional
    subject: Subject = Subject(
        identity="alice",
        properties={"name": Value.string("Alice")}
    )
    
    prop: Optional[Value] = subject.get_property("name")
    if prop:  # Type narrowing
        name: str = prop.as_string()
        print(f"Property name: {name}")
    
    missing: Optional[Value] = subject.get_property("missing")
    if missing is None:
        print("Property 'missing' not found (as expected)")
    print()


def example_validation_with_types() -> None:
    """Example with type-annotated validation."""
    print("=" * 60)
    print("Example 7: Type-Annotated Validation")
    print("=" * 60)
    
    pattern: Pattern = Pattern.pattern("data", Pattern.from_values(["a", "b"]))
    
    # Type checker knows ValidationRules constructor signature
    rules: ValidationRules = ValidationRules(
        max_depth=10,
        max_elements=100
    )
    
    # Type checker knows validate returns None or raises
    try:
        pattern.validate(rules)
        print("Pattern is valid")
    except ValidationError as e:
        # Type checker knows ValidationError properties
        message: str = e.message
        rule: str = e.rule
        location: Optional[str] = e.location
        print(f"Validation failed: {message}")
        print(f"Rule: {rule}, Location: {location}")
    print()


def create_typed_processor() -> Callable[[Pattern], List[str]]:
    """Return a type-safe pattern processor function."""
    
    def processor(p: Pattern) -> List[str]:
        """Process pattern and return list of values."""
        # Type checker verifies all method calls and return types
        if p.is_atomic():
            return [p.value]
        else:
            return p.values()
    
    return processor


def example_higher_order_functions() -> None:
    """Example with higher-order functions and types."""
    print("=" * 60)
    print("Example 8: Higher-Order Functions with Types")
    print("=" * 60)
    
    # Create pattern
    pattern: Pattern = Pattern.pattern("data", Pattern.from_values(["x", "y", "z"]))
    
    # Get typed processor
    processor: Callable[[Pattern], List[str]] = create_typed_processor()
    
    # Type checker verifies processor call and return type
    values: List[str] = processor(pattern)
    print(f"Processed values: {values}")
    print()


def example_generic_transformation() -> None:
    """Example with generic transformation function."""
    print("=" * 60)
    print("Example 9: Generic Transformation Function")
    print("=" * 60)
    
    def transform_pattern(
        p: Pattern,
        transformer: Callable[[str], str]
    ) -> Pattern:
        """Apply transformer to all pattern values."""
        return p.map(transformer)
    
    # Create pattern
    pattern: Pattern = Pattern.pattern("data", Pattern.from_values(["a", "b", "c"]))
    
    # Transform with uppercase
    upper: Pattern = transform_pattern(pattern, str.upper)
    print(f"Uppercase: {upper.values()}")
    
    # Transform with reverse
    reversed_p: Pattern = transform_pattern(pattern, lambda s: s[::-1])
    print(f"Reversed: {reversed_p.values()}")
    print()


def example_type_narrowing() -> None:
    """Example demonstrating type narrowing."""
    print("=" * 60)
    print("Example 10: Type Narrowing")
    print("=" * 60)
    
    # Create pattern
    pattern: Pattern = Pattern.point("test")
    
    # Type checker understands conditional type narrowing
    found: Optional[Pattern] = pattern.find_first(lambda p: True)
    
    if found:
        # Type narrowed to Pattern (not None)
        val: str = found.value  # Type checker knows this is safe
        print(f"Found (type narrowed): {val}")
    
    # Type narrowing with isinstance
    subject: Subject = Subject(identity="test")
    prop: Optional[Value] = subject.get_property("key")
    
    if prop is not None:
        # Type narrowed to Value (not None)
        # Safe to call Value methods
        try:
            s: str = prop.as_string()
            print(f"Property value: {s}")
        except (TypeError, ValueError):
            print("Property exists but is not a string")
    print()


def main() -> None:
    """Run all type safety examples."""
    print("\n" + "=" * 60)
    print("PATTERN-CORE PYTHON BINDINGS - TYPE SAFETY EXAMPLES")
    print("=" * 60 + "\n")
    
    example_typed_pattern_construction()
    example_typed_subject_construction()
    example_typed_operations()
    example_typed_callbacks()
    example_typed_pattern_subject()
    example_optional_handling()
    example_validation_with_types()
    example_higher_order_functions()
    example_generic_transformation()
    example_type_narrowing()
    
    print("=" * 60)
    print("All type safety examples completed successfully!")
    print("=" * 60)
    print("\nTo verify type safety, run:")
    print("  mypy type_safety.py")
    print("  pyright type_safety.py")


if __name__ == "__main__":
    main()
