# relateby.pattern — public API from pattern-core (no top-level pattern_core)
from relateby._native.pattern_core import Pattern
from relateby._native.pattern_core import StandardGraph as _NativeStandardGraph
from relateby._native.pattern_core import Subject as _Subject
from relateby._native.pattern_core import SubjectBuilder
from relateby._native.pattern_core import StructureAnalysis
from relateby._native.pattern_core import ValidationError
from relateby._native.pattern_core import ValidationRules
from relateby._native.pattern_core import Value

_Pattern = Pattern
Subject = _Subject


def _pattern_from_dict(d):
    """Convert a gram AST dict to a PatternSubject (Pattern[Subject])."""
    try:
        subj_data = d["subject"]
        subject = _Subject(
            identity=subj_data["identity"],
            labels=set(subj_data.get("labels", [])),
            properties=subj_data.get("properties", {}),
        )
        elements = [_pattern_from_dict(e) for e in d.get("elements", [])]
        if not elements:
            return _Pattern.point(subject)
        return _Pattern.pattern(subject, elements)
    except (KeyError, TypeError, ValueError) as exc:
        raise ValueError(
            f"relateby.pattern failed to reconstruct a Pattern from parsed Gram data: {exc}"
        ) from exc


def _standard_graph_from_gram(cls, input: str):
    """Parse gram notation into a StandardGraph."""
    from relateby._native import gram_codec as _gram  # lazy import

    try:
        pattern_dicts = _gram.parse_patterns_as_dicts(input)
        patterns = [_pattern_from_dict(d) for d in pattern_dicts]
        instance = cls()
        for pattern in patterns:
            instance.add_pattern(pattern)
        return instance
    except ValueError as exc:
        raise ValueError(f"relateby.pattern.StandardGraph.from_gram failed: {exc}") from exc
    except Exception as exc:
        raise ValueError(f"relateby.pattern.StandardGraph.from_gram failed: {exc}") from exc


StandardGraph = _NativeStandardGraph
StandardGraph.from_gram = classmethod(_standard_graph_from_gram)

__all__ = [
    "Pattern",
    "StandardGraph",
    "StructureAnalysis",
    "Subject",
    "SubjectBuilder",
    "ValidationError",
    "ValidationRules",
    "Value",
]
