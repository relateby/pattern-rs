"""Native Python value types for gram patterns.

Each variant is a @dataclass with structural equality via dataclass __eq__.
The value_from_dict function decodes the JSON interchange format from the
Rust gram-codec.
"""
from __future__ import annotations

from dataclasses import dataclass
from typing import Union


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


def value_from_dict(d: object) -> Value:
    """Decode a JSON interchange value to a native Value.

    The JSON interchange format uses:
    - Native JSON primitives for string, int/float, bool, null, array, map
    - Tagged objects {"type": "symbol"|"range"|"tagged"|"measurement", ...}

    Mirrors json_to_value in crates/gram-codec/src/json.rs.
    """
    if d is None:
        return NullVal()
    if isinstance(d, bool):
        return BoolVal(d)
    if isinstance(d, int):
        return IntVal(d)
    if isinstance(d, float):
        return FloatVal(d)
    if isinstance(d, str):
        return StringVal(d)
    if isinstance(d, list):
        return ArrayVal(items=[value_from_dict(item) for item in d])
    if isinstance(d, dict):
        type_tag = d.get("type")
        if type_tag == "symbol":
            return SymbolVal(value=str(d["value"]))
        if type_tag == "range":
            return RangeVal(
                lower=float(d["lower"]) if d.get("lower") is not None else None,
                upper=float(d["upper"]) if d.get("upper") is not None else None,
            )
        if type_tag == "measurement":
            return MeasurementVal(unit=str(d["unit"]), value=float(d["value"]))
        if type_tag == "tagged":
            return TaggedStringVal(tag=str(d["tag"]), content=str(d["content"]))
        # Plain object without "type" key → MapVal
        return MapVal(entries={k: value_from_dict(v) for k, v in d.items()})
    raise ValueError(f"Cannot decode value: {d!r}")


def value_to_dict(v: Value) -> object:
    """Encode a native Value back to the JSON interchange format.

    Mirrors value_to_json in crates/gram-codec/src/ast.rs.
    """
    if isinstance(v, StringVal):
        return v.value
    if isinstance(v, IntVal):
        return v.value
    if isinstance(v, FloatVal):
        return v.value
    if isinstance(v, BoolVal):
        return v.value
    if isinstance(v, NullVal):
        return None
    if isinstance(v, SymbolVal):
        return {"type": "symbol", "value": v.value}
    if isinstance(v, TaggedStringVal):
        return {"type": "tagged", "tag": v.tag, "content": v.content}
    if isinstance(v, ArrayVal):
        return [value_to_dict(item) for item in v.items]
    if isinstance(v, MapVal):
        return {k: value_to_dict(val) for k, val in v.entries.items()}
    if isinstance(v, RangeVal):
        return {"type": "range", "lower": v.lower, "upper": v.upper}
    if isinstance(v, MeasurementVal):
        return {"type": "measurement", "unit": v.unit, "value": v.value}
    raise ValueError(f"Unknown value type: {type(v)}")
