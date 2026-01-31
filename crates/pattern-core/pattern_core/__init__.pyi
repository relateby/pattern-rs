"""
Type stubs for pattern_core Python bindings.

This module provides Python-friendly bindings for pattern-core, enabling
Python developers to programmatically construct and operate on Pattern and Subject instances.
"""

from typing import Any, Callable, Dict, Generic, List, Optional, Set, TypeVar, Union, overload

# Type variables for generic pattern operations
T = TypeVar('T')
V = TypeVar('V')
R = TypeVar('R')  # For para return type

class Value:
    """
    Value represents property value types that can be stored in Subject properties.

    Supports standard types (string, int, decimal, boolean, symbol) and extended types
    (array, map, range, measurement).
    """

    @staticmethod
    def string(s: str) -> 'Value':
        """
        Create a string value.

        Args:
            s: String value

        Returns:
            String value instance
        """
        ...

    @staticmethod
    def int(i: int) -> 'Value':
        """
        Create an integer value.

        Args:
            i: Integer value

        Returns:
            Integer value instance
        """
        ...

    @staticmethod
    def decimal(f: float) -> 'Value':
        """
        Create a decimal value.

        Args:
            f: Decimal/float value

        Returns:
            Decimal value instance
        """
        ...

    @staticmethod
    def boolean(b: bool) -> 'Value':
        """
        Create a boolean value.

        Args:
            b: Boolean value

        Returns:
            Boolean value instance
        """
        ...

    @staticmethod
    def symbol(s: str) -> 'Value':
        """
        Create a symbol value.

        Args:
            s: Symbol identifier string

        Returns:
            Symbol value instance
        """
        ...

    @staticmethod
    def array(items: List['Value']) -> 'Value':
        """
        Create an array value.

        Args:
            items: List of Value instances

        Returns:
            Array value instance
        """
        ...

    @staticmethod
    def map(items: Dict[str, 'Value']) -> 'Value':
        """
        Create a map value.

        Args:
            items: Dictionary mapping strings to Value instances

        Returns:
            Map value instance
        """
        ...

    @staticmethod
    def range(lower: Optional[float] = None, upper: Optional[float] = None) -> 'Value':
        """
        Create a range value.

        Args:
            lower: Lower bound (inclusive), None for unbounded
            upper: Upper bound (inclusive), None for unbounded

        Returns:
            Range value instance
        """
        ...

    @staticmethod
    def measurement(value: float, unit: str) -> 'Value':
        """
        Create a measurement value.

        Args:
            value: Numeric measurement value
            unit: Unit string (e.g., "meters", "kg")

        Returns:
            Measurement value instance
        """
        ...

    def as_string(self) -> str:
        """Extract string value. Raises TypeError if not a string."""
        ...

    def as_int(self) -> int:
        """Extract integer value. Raises TypeError if not an integer."""
        ...

    def as_decimal(self) -> float:
        """Extract decimal value. Raises TypeError if not a decimal."""
        ...

    def as_boolean(self) -> bool:
        """Extract boolean value. Raises TypeError if not a boolean."""
        ...

    def as_array(self) -> List['Value']:
        """Extract array value. Raises TypeError if not an array."""
        ...

    def as_map(self) -> Dict[str, 'Value']:
        """Extract map value. Raises TypeError if not a map."""
        ...

class Subject:
    """
    Self-descriptive value type with identity, labels, and properties.

    Subjects can represent nodes in a graph or any entity with properties and labels.
    """

    def __init__(
        self,
        identity: str,
        labels: Optional[Set[str]] = None,
        properties: Optional[Dict[str, Value]] = None
    ) -> None:
        """
        Create a new Subject.

        Args:
            identity: Symbol identifier (string)
            labels: Set of label strings (optional)
            properties: Map of property names to Value instances (optional)
        """
        ...

    @property
    def identity(self) -> str:
        """Get the subject's identity symbol."""
        ...

    def get_labels(self) -> Set[str]:
        """Get all labels as a Python set."""
        ...

    def get_properties(self) -> Dict[str, Value]:
        """Get all properties as a Python dictionary."""
        ...

    def add_label(self, label: str) -> None:
        """
        Add a label to the subject.

        Args:
            label: Label string to add
        """
        ...

    def remove_label(self, label: str) -> None:
        """
        Remove a label from the subject.

        Args:
            label: Label string to remove
        """
        ...

    def has_label(self, label: str) -> bool:
        """
        Check if subject has a specific label.

        Args:
            label: Label string to check

        Returns:
            True if label exists, False otherwise
        """
        ...

    def get_property(self, name: str) -> Optional[Value]:
        """
        Get property value by name.

        Args:
            name: Property name

        Returns:
            Value instance if property exists, None otherwise
        """
        ...

    def set_property(self, name: str, value: Union[Value, str, int, float, bool, list, dict]) -> None:
        """
        Set property value.

        Args:
            name: Property name
            value: Property value (Value instance or Python native type)
        """
        ...

    def remove_property(self, name: str) -> None:
        """
        Remove a property.

        Args:
            name: Property name to remove
        """
        ...

class Pattern(Generic[V]):
    """
    Recursive, nested structure (s-expression-like) that can hold any value type.

    Pattern[V] is fully generic - V can be primitives, objects, or even other Patterns,
    enabling nesting types like Pattern[Pattern[T]].

    A pattern consists of a valu and zero or more elements (patterns).
    The value decorates or says something about the pattern represented by the elements.
    Atomic patterns have no elements.
    """

    @staticmethod
    def point(value: Any) -> 'Pattern':
        """
        Create an atomic pattern (no elements).

        Args:
            value: The value for this pattern (any Python type)

        Returns:
            Atomic Pattern instance
        """
        ...

    @staticmethod
    def pattern(value: Any, elements: List['Pattern']) -> 'Pattern':
        """
        Create a pattern with value decoration and elements.

        The value decorates or describes the pattern represented by the elements.

        Args:
            value: The value decoration for this pattern
            elements: List of Pattern instances that form the pattern

        Returns:
            Pattern instance with value and elements
        """
        ...

    @staticmethod
    def of(value: Any) -> 'Pattern':
        """
        Alias for point(). Lift a value into a Pattern.

        This follows the functional programming convention where
        'of' is used to lift a value into a functor/applicative.

        Args:
            value: The value for this pattern (any Python type)

        Returns:
            Atomic Pattern instance
        """
        ...

    @staticmethod
    def from_values(values: List[Any]) -> List['Pattern']:
        """
        Convert a list of values into a list of patterns.

        Applies Pattern.of() (which is Pattern.point()) uniformly to every value,
        lifting each into a Pattern. Works on any type including Patterns.

        Args:
            values: List of values to convert (any type)

        Returns:
            List of Pattern instances

        Example:
            >>> # From primitives
            >>> patterns = Pattern.from_values([1, 2, 3])
            >>> len(patterns)
            3
            >>> # From patterns (creates Pattern<Pattern<T>>)
            >>> p1 = Pattern.point("a")
            >>> patterns = Pattern.from_values([p1])
            >>> patterns[0].value  # This is a Pattern!
            Pattern(value="a", elements=0)
        """
        ...

    @property
    def value(self) -> Any:
        """Get the pattern's value (can be any Python type including Pattern)."""
        ...

    @property
    def elements(self) -> List['Pattern']:
        """Get the pattern's elements (the patterns that make up this pattern)."""
        ...

    def is_atomic(self) -> bool:
        """
        Check if pattern is atomic (has no elements).

        Returns:
            True if pattern has no elements, False otherwise
        """
        ...

    def length(self) -> int:
        """
        Get the number of direct elements in this pattern.

        Returns:
            Number of elements
        """
        ...

    def size(self) -> int:
        """
        Get the total number of nodes in the pattern tree.

        Returns:
            Total node count (including this node)
        """
        ...

    def depth(self) -> int:
        """
        Get the maximum nesting depth of the pattern.

        Returns:
            Maximum depth (0 for atomic patterns)
        """
        ...

    def values(self) -> List[Any]:
        """
        Get all values as a flat list (pre-order traversal).

        Returns:
            List of all values (any type) in traversal order
        """
        ...

    def any_value(self, predicate: Callable[[Any], bool]) -> bool:
        """
        Check if any value satisfies the predicate.

        Args:
            predicate: Function that takes a value and returns bool

        Returns:
            True if any value satisfies predicate, False otherwise
        """
        ...

    def all_values(self, predicate: Callable[[Any], bool]) -> bool:
        """
        Check if all values satisfy the predicate.

        Args:
            predicate: Function that takes a value and returns bool

        Returns:
            True if all values satisfy predicate, False otherwise
        """
        ...

    def filter(self, predicate: Callable[['Pattern'], bool]) -> List['Pattern']:
        """
        Filter patterns by predicate.

        Args:
            predicate: Function that takes a Pattern and returns bool

        Returns:
            List of patterns that satisfy predicate
        """
        ...

    def find_first(self, predicate: Callable[['Pattern'], bool]) -> Optional['Pattern']:
        """
        Find first pattern matching predicate.

        Args:
            predicate: Function that takes a Pattern and returns bool

        Returns:
            First matching pattern, or None if not found
        """
        ...

    def matches(self, other: 'Pattern') -> bool:
        """
        Check if patterns have identical structure.

        Args:
            other: Pattern to compare with

        Returns:
            True if patterns match structurally
        """
        ...

    def contains(self, other: 'Pattern') -> bool:
        """
        Check if this pattern contains other as a subpattern.

        Args:
            other: Pattern to search for

        Returns:
            True if other is a subpattern
        """
        ...

    def map(self, func: Callable[[Any], Any]) -> 'Pattern':
        """
        Transform values while preserving structure.

        Args:
            func: Function that takes a value and returns a new value

        Returns:
            New Pattern with transformed values
        """
        ...

    def fold(self, init: Any, func: Callable[[Any, Any], Any]) -> Any:
        """
        Fold over all values with an accumulator.

        Args:
            init: Initial accumulator value
            func: Function that takes (accumulator, value) and returns new accumulator

        Returns:
            Final accumulator value
        """
        ...

    def para(self, func: Callable[['Pattern[V]', List[R]], R]) -> R:
        """
        Paramorphism: structure-aware fold with access to pattern structure.

        At each node, applies func to (current_pattern, element_results) where
        element_results is a list of results from recursively processing elements.
        Evaluation is bottom-up (elements before parents) and left-to-right.

        Atomic patterns receive an empty list for element_results.

        This enables structure-dependent computations like depth-weighted sums,
        computing multiple statistics in one pass (sum, count, depth), and
        structure-preserving transformations.

        Args:
            func: Function taking (Pattern[V], List[R]) -> R
                  - First arg is the pattern at current position (including value and elements)
                  - Second arg is list of results from recursive para on elements
                  - Returns result of type R

        Returns:
            Result of applying func across the entire pattern structure

        Example:
            >>> # Depth-weighted sum: value + sum of element results
            >>> pattern = Pattern.pattern(1, [
            ...     Pattern.point(2),
            ...     Pattern.pattern(3, [Pattern.point(4)])
            ... ])
            >>> result = pattern.para(lambda p, rs: p.value + sum(rs))
            >>> # Result: 1 + (2 + 0) + (3 + (4 + 0)) = 1 + 2 + 7 = 10

            >>> # Atomic pattern receives empty list
            >>> atomic = Pattern.point(5)
            >>> result = atomic.para(lambda p, rs: p.value + sum(rs))
            >>> # Result: 5 + 0 = 5

            >>> # Equivalent to fold for simple value aggregation
            >>> result = pattern.para(lambda p, rs: p.value + sum(rs))
            >>> fold_result = pattern.fold(0, lambda acc, v: acc + v)
            >>> # Both produce same sum of values
        """
        ...

    def combine(self, other: 'Pattern') -> 'Pattern':
        """
        Combine two patterns associatively.

        Args:
            other: Pattern to combine with

        Returns:
            Combined Pattern
        """
        ...

    @staticmethod
    def zip3(
        left: List['Pattern'],
        right: List['Pattern'],
        values: List[Any]
    ) -> List['Pattern']:
        """
        Create patterns by combining three lists pointwise (zipWith3).

        Takes three lists and combines them element-wise to create relationship patterns.
        Each resulting pattern has value from values list and elements [left, right].

        This is useful for creating relationships from separate lists of source nodes,
        target nodes, and relationship values.

        Args:
            left: First list of patterns (e.g., source nodes)
            right: Second list of patterns (e.g., target nodes)
            values: List of values for the new patterns (e.g., relationship types)

        Returns:
            List of patterns where each has value from values and elements [left[i], right[i]]

        Example:
            >>> sources = [Pattern.point("Alice"), Pattern.point("Bob")]
            >>> targets = [Pattern.point("Company"), Pattern.point("Project")]
            >>> rel_types = ["WORKS_FOR", "MANAGES"]
            >>> relationships = Pattern.zip3(sources, targets, rel_types)
        """
        ...

    @staticmethod
    def zip_with(
        left: List['Pattern'],
        right: List['Pattern'],
        value_fn: Callable[['Pattern', 'Pattern'], Any]
    ) -> List['Pattern']:
        """
        Create patterns by applying a function to pairs from two lists (zipWith2).

        Takes two lists of patterns and applies a function to each pair to compute
        the value for the resulting pattern. Useful when relationship values are
        derived from the patterns being connected.

        Args:
            left: First list of patterns (e.g., source nodes)
            right: Second list of patterns (e.g., target nodes)
            value_fn: Function that computes value from each pair of patterns

        Returns:
            List of patterns where each has value computed by value_fn

        Example:
            >>> people = [Pattern.point("Alice"), Pattern.point("Bob")]
            >>> companies = [Pattern.point("TechCorp"), Pattern.point("StartupInc")]
            >>> relationships = Pattern.zip_with(people, companies,
            ...     lambda p, c: f"{p.value}_WORKS_AT_{c.value}")
        """
        ...

    def extract(self) -> Any:
        """
        Extract value at current position (comonad operation).

        Returns:
            The pattern's value (can be any type)
        """
        ...

    def extend(self, func: Callable[['Pattern'], Any]) -> 'Pattern':
        """
        Apply function to all contexts (comonad operation).

        Args:
            func: Function that takes a Pattern and returns a value

        Returns:
            New Pattern with func applied to all contexts
        """
        ...

    def depth_at(self) -> 'Pattern':
        """
        Decorate each position with its depth.

        Returns:
            Pattern where each value is replaced with its depth (int)
        """
        ...

    def size_at(self) -> 'Pattern':
        """
        Decorate each position with its subtree size.

        Returns:
            Pattern where each value is replaced with subtree size (int)
        """
        ...

    def indices_at(self) -> 'Pattern':
        """
        Decorate each position with path from root.

        Returns:
            Pattern where each value is replaced with path indices (List[int])
        """
        ...

    def validate(self, rules: 'ValidationRules') -> None:
        """
        Validate pattern structure against rules.

        Args:
            rules: ValidationRules instance

        Raises:
            ValidationError: If validation fails
        """
        ...

    def analyze_structure(self) -> 'StructureAnalysis':
        """
        Analyze pattern structure.

        Returns:
            StructureAnalysis instance with analysis results
        """
        ...

class ValidationRules:
    """Configuration for pattern validation."""

    def __init__(
        self,
        max_depth: Optional[int] = None,
        max_elements: Optional[int] = None
    ) -> None:
        """
        Create validation rules.

        Args:
            max_depth: Maximum allowed nesting depth (None for unlimited)
            max_elements: Maximum allowed elements per pattern (None for unlimited)
        """
        ...

    @property
    def max_depth(self) -> Optional[int]:
        """Get maximum depth constraint."""
        ...

    @property
    def max_elements(self) -> Optional[int]:
        """Get maximum elements constraint."""
        ...

class ValidationError(ValueError):
    """Error raised when pattern validation fails."""

    @property
    def message(self) -> str:
        """Get error message."""
        ...

    @property
    def rule(self) -> str:
        """Get name of violated rule."""
        ...

    @property
    def location(self) -> List[str]:
        """Get location in pattern where violation occurred."""
        ...

class StructureAnalysis:
    """Result of pattern structure analysis."""

    @property
    def summary(self) -> str:
        """Get human-readable summary."""
        ...

    @property
    def depth_distribution(self) -> List[int]:
        """Get count of nodes at each depth."""
        ...

    @property
    def element_counts(self) -> List[int]:
        """Get element counts at each level."""
        ...

    @property
    def nesting_patterns(self) -> List[str]:
        """Get description of nesting patterns."""
        ...

__all__ = [
    'Value',
    'Subject',
    'Pattern',
    'ValidationRules',
    'ValidationError',
    'StructureAnalysis',
]
