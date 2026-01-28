# Python API Contract

**Feature**: 024-python-pattern-core  
**Date**: 2026-01-27

## Module: pattern_core

Python module providing bindings to pattern-core Rust crate.

### Classes

#### Pattern

Generic pattern class for recursive, nested structures.

```python
class Pattern:
    value: Any
    elements: List[Pattern]
    
    @staticmethod
    def of(value: Any) -> Pattern:
        """Create atomic pattern (alias for point)."""
        ...
    
    @staticmethod
    def point(value: Any) -> Pattern:
        """Create atomic pattern (no elements)."""
        ...
    
    @staticmethod
    def pattern(value: Any, elements: List[Pattern]) -> Pattern:
        """Create pattern with value and elements."""
        ...
    
    @staticmethod
    def from_values(values: List[Any]) -> List[Pattern]:
        """Convert list of values to list of patterns (each value lifted via point)."""
        ...
    
    def length(self) -> int:
        """Return number of direct elements."""
        ...
    
    def size(self) -> int:
        """Return total number of nodes."""
        ...
    
    def depth(self) -> int:
        """Return maximum nesting depth."""
        ...
    
    def is_atomic(self) -> bool:
        """Return True if pattern has no elements."""
        ...
    
    def values(self) -> List[Any]:
        """Extract all values as flat list (pre-order)."""
        ...
    
    def any_value(self, predicate: Callable[[Any], bool]) -> bool:
        """Check if any value satisfies predicate."""
        ...
    
    def all_values(self, predicate: Callable[[Any], bool]) -> bool:
        """Check if all values satisfy predicate."""
        ...
    
    def filter(self, predicate: Callable[[Pattern], bool]) -> Pattern:
        """Extract subpatterns matching predicate."""
        ...
    
    def find_first(self, predicate: Callable[[Pattern], bool]) -> Optional[Pattern]:
        """Find first subpattern matching predicate."""
        ...
    
    def matches(self, other: Pattern) -> bool:
        """Check if patterns have identical structure."""
        ...
    
    def contains(self, other: Pattern) -> bool:
        """Check if pattern contains other as subpattern."""
        ...
    
    def map(self, func: Callable[[Any], Any]) -> Pattern:
        """Transform values while preserving structure."""
        ...
    
    def fold(self, init: Any, func: Callable[[Any, Any], Any]) -> Any:
        """Fold over all values."""
        ...
    
    def combine(self, other: Pattern) -> Pattern:
        """Combine two patterns associatively."""
        ...
    
    def extract(self) -> Any:
        """Extract value at current position (comonad)."""
        ...
    
    def extend(self, func: Callable[[Pattern], Any]) -> Pattern:
        """Apply function to all contexts (comonad)."""
        ...
    
    def depth_at(self) -> Pattern[int]:
        """Decorate each position with depth."""
        ...
    
    def size_at(self) -> Pattern[int]:
        """Decorate each position with subtree size."""
        ...
    
    def indices_at(self) -> Pattern[List[int]]:
        """Decorate each position with path from root."""
        ...
    
    def validate(self, rules: ValidationRules) -> None:
        """Validate pattern structure. Raises ValidationError if invalid."""
        ...
    
    def analyze_structure(self) -> StructureAnalysis:
        """Analyze pattern structure."""
        ...
```

#### PatternSubject

Specialized Pattern class for Pattern<Subject>.

```python
class PatternSubject(Pattern):
    value: Subject
    elements: List[PatternSubject]
    
    # Inherits all Pattern methods
    # Subject-specific methods if needed
```

#### Subject

Self-descriptive value with identity, labels, and properties.

```python
class Subject:
    identity: str
    labels: Set[str]
    properties: Dict[str, Value]
    
    def __init__(
        self,
        identity: str,
        labels: Optional[Set[str]] = None,
        properties: Optional[Dict[str, Value]] = None
    ) -> None:
        """Create Subject with identity, labels, and properties."""
        ...
    
    def add_label(self, label: str) -> None:
        """Add a label."""
        ...
    
    def remove_label(self, label: str) -> None:
        """Remove a label."""
        ...
    
    def has_label(self, label: str) -> bool:
        """Check if label exists."""
        ...
    
    def get_property(self, name: str) -> Optional[Value]:
        """Get property value."""
        ...
    
    def set_property(self, name: str, value: Value) -> None:
        """Set property value."""
        ...
    
    def remove_property(self, name: str) -> None:
        """Remove property."""
        ...
```

#### Value

Enum representing property value types.

```python
class Value:
    @staticmethod
    def string(s: str) -> Value:
        """Create string value."""
        ...
    
    @staticmethod
    def int(i: int) -> Value:
        """Create integer value."""
        ...
    
    @staticmethod
    def decimal(f: float) -> Value:
        """Create decimal value."""
        ...
    
    @staticmethod
    def boolean(b: bool) -> Value:
        """Create boolean value."""
        ...
    
    @staticmethod
    def symbol(s: str) -> Value:
        """Create symbol value."""
        ...
    
    @staticmethod
    def array(items: List[Value]) -> Value:
        """Create array value."""
        ...
    
    @staticmethod
    def map(items: Dict[str, Value]) -> Value:
        """Create map value."""
        ...
    
    @staticmethod
    def range(lower: Optional[float], upper: Optional[float]) -> Value:
        """Create range value."""
        ...
    
    @staticmethod
    def measurement(value: float, unit: str) -> Value:
        """Create measurement value."""
        ...
    
    def as_string(self) -> str:
        """Extract string value. Raises TypeError if not string."""
        ...
    
    def as_int(self) -> int:
        """Extract integer value. Raises TypeError if not int."""
        ...
    
    def as_decimal(self) -> float:
        """Extract decimal value. Raises TypeError if not decimal."""
        ...
    
    def as_boolean(self) -> bool:
        """Extract boolean value. Raises TypeError if not boolean."""
        ...
    
    def as_array(self) -> List[Value]:
        """Extract array value. Raises TypeError if not array."""
        ...
    
    def as_map(self) -> Dict[str, Value]:
        """Extract map value. Raises TypeError if not map."""
        ...
```

#### ValidationRules

Configuration for pattern validation.

```python
class ValidationRules:
    max_depth: Optional[int]
    max_elements: Optional[int]
    
    def __init__(
        self,
        max_depth: Optional[int] = None,
        max_elements: Optional[int] = None
    ) -> None:
        """Create validation rules."""
        ...
```

#### StructureAnalysis

Result of pattern structure analysis.

```python
class StructureAnalysis:
    summary: str
    depth_distribution: List[int]  # Count of nodes at each depth (index = depth)
    element_counts: List[int]
    nesting_patterns: List[str]
```

### Exceptions

#### ValidationError

Raised when pattern validation fails.

```python
class ValidationError(ValueError):
    message: str
    rule: str
    location: Optional[str]
```

## Type Hints

All classes and methods include type hints for static type checking with mypy and pyright.

## Error Handling

- **ValueError**: Invalid input (validation failures, invalid arguments)
- **TypeError**: Type conversion errors (wrong Python types)
- **RecursionError**: Stack overflow (deeply nested patterns)
- **RuntimeError**: Unexpected errors (internal errors)

## Performance Guarantees

- Pattern operations: O(n) where n is number of nodes
- Python-Rust boundary: <2x overhead compared to native Rust
- Large patterns (1000+ nodes): Handled efficiently
- Deep nesting (100+ levels): Stack overflow protection

## Thread Safety

Pattern and Subject instances are thread-safe (immutable by default, PyO3 handles GIL).

## Memory Management

Pattern instances are managed by Python's garbage collector. Rust memory is automatically freed when Python objects are garbage collected.
