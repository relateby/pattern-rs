from __future__ import annotations

from typing import Any, Callable, Generic, Iterator, Optional, TypeVar, Union

V = TypeVar("V")
U = TypeVar("U")
R = TypeVar("R")


class StringVal:
    value: str
    def __init__(self, value: str) -> None: ...


class IntVal:
    value: int
    def __init__(self, value: int) -> None: ...


class FloatVal:
    value: float
    def __init__(self, value: float) -> None: ...


class BoolVal:
    value: bool
    def __init__(self, value: bool) -> None: ...


class NullVal:
    def __init__(self) -> None: ...


class SymbolVal:
    value: str
    def __init__(self, value: str) -> None: ...


class TaggedStringVal:
    tag: str
    content: str
    def __init__(self, tag: str, content: str) -> None: ...


class ArrayVal:
    items: list["Value"]
    def __init__(self, items: list["Value"]) -> None: ...


class MapVal:
    entries: dict[str, "Value"]
    def __init__(self, entries: dict[str, "Value"]) -> None: ...


class RangeVal:
    lower: Optional[float]
    upper: Optional[float]
    def __init__(self, lower: Optional[float] = ..., upper: Optional[float] = ...) -> None: ...


class MeasurementVal:
    unit: str
    value: float
    def __init__(self, unit: str, value: float) -> None: ...


Value = Union[
    StringVal,
    IntVal,
    FloatVal,
    BoolVal,
    NullVal,
    SymbolVal,
    TaggedStringVal,
    ArrayVal,
    MapVal,
    RangeVal,
    MeasurementVal,
]


class Subject:
    identity: str
    labels: set[str]
    properties: dict[str, Value]
    def __init__(
        self,
        identity: str,
        labels: Optional[set[str]] = ...,
        properties: Optional[dict[str, Value]] = ...,
    ) -> None: ...
    @classmethod
    def from_id(cls, identity: str) -> "Subject": ...
    def with_label(self, label: str) -> "Subject": ...
    def with_property(self, name: str, value: Value) -> "Subject": ...


class Pattern(Generic[V]):
    value: V
    elements: list["Pattern[V]"]
    def __init__(self, value: V, elements: Optional[list["Pattern[V]"]] = ...) -> None: ...
    @classmethod
    def point(cls, value: V) -> "Pattern[V]": ...
    @classmethod
    def of(cls, value: V) -> "Pattern[V]": ...
    @property
    def is_atomic(self) -> bool: ...
    @property
    def length(self) -> int: ...
    @property
    def size(self) -> int: ...
    @property
    def depth(self) -> int: ...
    def map(self, fn: Callable[[V], U]) -> "Pattern[U]": ...
    def fold(self, init: R, fn: Callable[[R, V], R]) -> R: ...
    def filter(self, predicate: Callable[["Pattern[V]"], bool]) -> list["Pattern[V]"]: ...
    def find_first(self, predicate: Callable[[V], bool]) -> Optional[V]: ...
    def extend(self, fn: Callable[["Pattern[V]"], U]) -> "Pattern[U]": ...
    def extract(self) -> V: ...
    def duplicate(self) -> "Pattern[Pattern[V]]": ...
    def values(self) -> list[V]: ...
    def __iter__(self) -> Iterator["Pattern[V]"]: ...


def value_from_dict(d: object) -> Value: ...
def value_to_dict(v: Value) -> object: ...
def pattern_from_dict(d: dict[str, Any]) -> Pattern[Subject]: ...


class ValidationRules:
    def __init__(self, max_depth: Optional[int] = ..., max_elements: Optional[int] = ...) -> None: ...


class ValidationError(ValueError): ...


class StructureAnalysis: ...


class SubjectBuilder:
    def label(self, label: str) -> "SubjectBuilder": ...
    def property(self, key: str, value: Any) -> "SubjectBuilder": ...
    def done(self) -> Subject: ...


class StandardGraph:
    @classmethod
    def from_gram(cls, input: str) -> "StandardGraph": ...
    def node(self, id: str) -> Optional[Pattern[Any]]: ...
    @property
    def node_count(self) -> int: ...
