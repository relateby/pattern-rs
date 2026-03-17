# relateby.pattern — native Python Pattern, Subject, and Value types
#
# These are pure Python dataclasses — no PyO3 round-trip per operation.
# Structural equality works via dataclass __eq__.

from ._pattern import Pattern
from ._subject import Subject
from ._value import (
    Value,
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
    value_from_dict,
    value_to_dict,
)
from ._decode import pattern_from_dict

# --- Legacy PyO3-backed types (available under original names for compatibility) ---
# These are the old WASM/PyO3-backed implementations. New code should use the
# native Pattern, Subject, and Value classes above.
try:
    from relateby._native.pattern_core import Pattern as _LegacyPattern
    from relateby._native.pattern_core import Subject as _LegacySubject
    from relateby._native.pattern_core import StandardGraph as _NativeStandardGraph
    from relateby._native.pattern_core import SubjectBuilder
    from relateby._native.pattern_core import StructureAnalysis
    from relateby._native.pattern_core import ValidationError
    from relateby._native.pattern_core import ValidationRules
    from relateby._native.pattern_core import Value as _LegacyValue

    def _standard_graph_from_gram(cls, input: str):
        """Parse gram notation into a StandardGraph (uses native Python decode)."""
        import json as _json
        from relateby._native import gram_codec as _gram
        try:
            json_str = _gram.gram_parse_to_json(input)
            pattern_dicts = _json.loads(json_str)
            patterns = [_LegacyPattern.point(
                _LegacySubject(
                    identity=d["subject"]["identity"],
                    labels=set(d["subject"].get("labels", [])),
                    properties=d["subject"].get("properties", {}),
                )
            ) for d in pattern_dicts]
            instance = cls()
            for pattern in patterns:
                instance.add_pattern(pattern)
            return instance
        except Exception as exc:
            raise ValueError(f"StandardGraph.from_gram failed: {exc}") from exc

    StandardGraph = _NativeStandardGraph
    StandardGraph.from_gram = classmethod(_standard_graph_from_gram)

except ImportError:
    # Native extension not built — StandardGraph unavailable
    StandardGraph = None  # type: ignore[assignment]
    SubjectBuilder = None  # type: ignore[assignment]
    StructureAnalysis = None  # type: ignore[assignment]
    ValidationError = None  # type: ignore[assignment]
    ValidationRules = None  # type: ignore[assignment]


__all__ = [
    # Native types (primary API)
    "Pattern",
    "Subject",
    "Value",
    "StringVal",
    "IntVal",
    "FloatVal",
    "BoolVal",
    "NullVal",
    "SymbolVal",
    "TaggedStringVal",
    "ArrayVal",
    "MapVal",
    "RangeVal",
    "MeasurementVal",
    "value_from_dict",
    "value_to_dict",
    "pattern_from_dict",
    # Legacy types (for compatibility)
    "StandardGraph",
    "SubjectBuilder",
    "StructureAnalysis",
    "ValidationError",
    "ValidationRules",
]
