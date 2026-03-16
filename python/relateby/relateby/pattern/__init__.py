# relateby.pattern — public API from pattern-core (no top-level pattern_core)
from relateby._native.pattern_core import *  # noqa: F401, F403
from relateby._native.pattern_core import StandardGraph as _NativeStandardGraph
from relateby._native.pattern_core import Subject as _Subject
from relateby._native.pattern_core import Pattern as _Pattern


def _pattern_from_dict(d):
    """Convert a gram AST dict to a PatternSubject (Pattern[Subject])."""
    subj_data = d["subject"]
    subject = _Subject(
        identity=subj_data["identity"],
        labels=set(subj_data.get("labels", [])),
        properties=subj_data.get("properties", {}),
    )
    elements = [_pattern_from_dict(e) for e in d.get("elements", [])]
    if not elements:
        return _Pattern.point(subject)
    p = _Pattern.pattern(subject)
    for elem in elements:
        p.add_element(elem)
    return p


class StandardGraph(_NativeStandardGraph):
    """StandardGraph with gram parsing support (T019).

    Extends the native StandardGraph with a ``from_gram`` classmethod that
    bridges the gram-codec parser with the pattern-core graph builder.
    """

    @classmethod
    def from_gram(cls, input: str) -> "StandardGraph":
        """Parse gram notation into a StandardGraph.

        Args:
            input: Gram notation string (e.g. ``"(alice:Person) (bob:Person)"``)

        Returns:
            StandardGraph populated with the parsed patterns.

        Raises:
            ValueError: If the gram notation is invalid.
        """
        from relateby._native import gram_codec as _gram  # lazy import

        pattern_dicts = _gram.parse_patterns_as_dicts(input)
        patterns = [_pattern_from_dict(d) for d in pattern_dicts]
        instance = cls()  # creates empty graph via PyO3 __init__
        for pattern in patterns:
            instance.add_pattern(pattern)
        return instance
