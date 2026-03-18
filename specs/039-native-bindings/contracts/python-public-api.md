# Contract: Python Public API

**Feature**: 039-native-bindings
**Package**: `relateby-pattern` (PyPI)
**Namespaces**: `relateby.pattern`, `relateby.gram`
**Date**: 2026-03-17

The public API surface remains at the same import paths. Internal implementation changes from PyO3-backed classes to native Python dataclasses.

---

## Pattern[V]

```python
from __future__ import annotations
from dataclasses import dataclass, field
from typing import Callable, Generic, Iterator, Optional, TypeVar

V = TypeVar("V")
U = TypeVar("U")
R = TypeVar("R")

@dataclass
class Pattern(Generic[V]):
    value: V
    elements: list["Pattern[V]"] = field(default_factory=list)

    # Constructors
    @classmethod
    def point(cls, value: V) -> "Pattern[V]": ...
    @classmethod
    def of(cls, value: V) -> "Pattern[V]": ...          # alias for point

    # Computed properties
    @property
    def is_atomic(self) -> bool: ...
    @property
    def length(self) -> int: ...
    @property
    def size(self) -> int: ...
    @property
    def depth(self) -> int: ...

    # Operations (also available as standalone functions in relateby.pattern)
    def map(self, fn: Callable[[V], U]) -> "Pattern[U]": ...
    def fold(self, init: R, fn: Callable[[R, V], R]) -> R: ...
    def filter(self, predicate: Callable[["Pattern[V]"], bool]) -> list["Pattern[V]"]: ...
    def find_first(self, predicate: Callable[[V], bool]) -> Optional[V]: ...
    def extend(self, fn: Callable[["Pattern[V]"], U]) -> "Pattern[U]": ...
    def extract(self) -> V: ...
    def duplicate(self) -> "Pattern[Pattern[V]]": ...

    # Equality via dataclass __eq__ (structural, recursive)
```

---

## Subject

```python
from __future__ import annotations
from dataclasses import dataclass, field

@dataclass
class Subject:
    identity: str
    labels: set[str] = field(default_factory=set)
    properties: dict[str, "Value"] = field(default_factory=dict)

    @classmethod
    def from_id(cls, identity: str) -> "Subject": ...

    def with_label(self, label: str) -> "Subject": ...
    def with_property(self, name: str, value: "Value") -> "Subject": ...
```

---

## Value

```python
from __future__ import annotations
from dataclasses import dataclass
from typing import Union

# Tagged hierarchy — isinstance checks work; __eq__ is structural via dataclass

@dataclass
class StringVal:
    value: str

@dataclass
class IntVal:
    value: int

@dataclass
class FloatVal:
    value: float

@dataclass
class BoolVal:
    value: bool

@dataclass
class NullVal:
    pass

@dataclass
class SymbolVal:
    value: str

@dataclass
class TaggedStringVal:
    tag: str
    content: str

@dataclass
class ArrayVal:
    items: list["Value"]

@dataclass
class MapVal:
    entries: dict[str, "Value"]

@dataclass
class RangeVal:
    lower: float | None = None
    upper: float | None = None

@dataclass
class MeasurementVal:
    unit: str
    value: float

Value = Union[
    StringVal, IntVal, FloatVal, BoolVal, NullVal, SymbolVal,
    TaggedStringVal, ArrayVal, MapVal, RangeVal, MeasurementVal,
]
```

---

## StandardGraph

```python
from __future__ import annotations

class StandardGraph:
    @classmethod
    def from_patterns(cls, patterns: list[Pattern[Subject]]) -> "StandardGraph": ...
    @classmethod
    def from_gram(cls, input: str) -> "StandardGraph": ...     # parses + classifies

    @property
    def node_count(self) -> int: ...
    @property
    def relationship_count(self) -> int: ...
    @property
    def annotation_count(self) -> int: ...
    @property
    def walk_count(self) -> int: ...

    def nodes(self) -> Iterator[tuple[str, Pattern[Subject]]]: ...
    def relationships(self) -> Iterator[tuple[str, dict]]: ...     # dict has pattern/source/target
    def annotations(self) -> Iterator[tuple[str, Pattern[Subject]]]: ...
    def walks(self) -> Iterator[tuple[str, Pattern[Subject]]]: ...
    def other(self) -> list[Pattern[Subject]]: ...

    def node(self, id: str) -> Pattern[Subject] | None: ...
    def relationship(self, id: str) -> dict | None: ...
```

---

## relateby.gram (parse interface)

```python
from relateby.gram import parse_gram, gram_stringify, gram_validate

def parse_gram(input: str) -> list[Pattern[Subject]]:
    """Parse gram notation. Raises GramParseError on invalid input."""
    ...

def gram_stringify(patterns: list[Pattern[Subject]]) -> str:
    """Serialize patterns to gram notation. Raises GramSerializeError on failure."""
    ...

def gram_validate(input: str) -> list[str]:
    """Validate gram notation. Returns list of error strings (empty = valid)."""
    ...

class GramParseError(Exception):
    def __init__(self, input: str, cause: str): ...
    @property
    def input(self) -> str: ...
    @property
    def cause(self) -> str: ...
```

---

## Breaking changes from current API

| Current | New | Migration |
|---------|-----|-----------|
| `Pattern` is a PyO3 class | `Pattern` is a Python `@dataclass` | `pattern.value`, `pattern.elements` are now directly accessible |
| `Subject` is a PyO3 class | `Subject` is a Python `@dataclass` | Fields directly accessible; no `.identity()` method needed |
| `Value` uses factory class methods | `Value` is a union of dataclasses | `isinstance(v, StringVal)` replaces type-check methods |
| `ParseResult` returned from `parse_gram` | `list[Pattern[Subject]]` returned directly | Remove `.pattern_count` / `.identifiers` access |
| `SubjectBuilder` fluent API | `Subject.from_id().with_label().with_property()` | Same semantics, different API shape |
| `StandardGraph.from_gram()` requires separate import bridge | `StandardGraph.from_gram()` is natively in Python | No change for callers |
