# relateby.gram — parse/stringify gram notation
#
# parse/stringify return/accept list[Pattern[Subject]] (native Python dataclasses).
# parse_with_header/stringify_with_header handle the optional leading header record.
# Errors surface as GramParseError with .input and .cause attributes.

from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from relateby.pattern import Pattern
    from relateby.pattern._subject import Subject


class GramParseError(Exception):
    """Raised when gram notation parsing or serialization fails.

    Attributes:
        input: The original gram string that caused the error (empty string
            for serialization errors where there is no input text).
        cause: Human-readable description of what went wrong.

    Example::

        from relateby.gram import parse, GramParseError

        try:
            parse("(unclosed")
        except GramParseError as err:
            print(err.input)   # "(unclosed"
            print(err.cause)   # parse error description
    """

    def __init__(self, input: str, cause: str) -> None:
        super().__init__(f"Gram parse error: {cause}")
        self._input = input
        self._cause = cause

    @property
    def input(self) -> str:
        """The gram string that was being parsed when the error occurred."""
        return self._input

    @property
    def cause(self) -> str:
        """Human-readable description of what went wrong."""
        return self._cause


def _pattern_to_dict(p: "Pattern[Subject]") -> dict:
    from relateby.pattern._value import value_to_dict
    return {
        "subject": {
            "identity": p.value.identity,
            "labels": sorted(p.value.labels),
            "properties": {k: value_to_dict(v) for k, v in p.value.properties.items()},
        },
        "elements": [_pattern_to_dict(e) for e in p.elements],
    }


def parse(input: str) -> "list[Pattern[Subject]]":
    """Parse gram notation into a list of native ``Pattern[Subject]`` objects.

    Each top-level element in the gram document becomes one entry in the
    returned list.  An empty string or whitespace-only input returns ``[]``.

    Args:
        input: Gram notation string, e.g. ``"(alice)-[:KNOWS]->(bob)"``.

    Returns:
        List of ``Pattern[Subject]`` objects, one per top-level gram element.

    Raises:
        GramParseError: If *input* is not valid gram notation.

    Example::

        from relateby.gram import parse, stringify

        patterns = parse("(alice:Person)-[:KNOWS]->(bob:Person)")
        assert len(patterns) == 1

        gram = stringify(patterns)
        # "(alice:Person)-[:KNOWS]->(bob:Person)"
    """
    from relateby._native import gram_codec as _gram
    from relateby.pattern._decode import pattern_from_dict

    try:
        raw_patterns = _gram.parse(input)
        return [pattern_from_dict(d) for d in raw_patterns]
    except Exception as exc:
        raise GramParseError(input=input, cause=str(exc)) from exc


def stringify(patterns: "list[Pattern[Subject]]") -> str:
    """Serialize a list of ``Pattern[Subject]`` objects to gram notation.

    Args:
        patterns: List of patterns to serialize, typically the result of
            :func:`parse` or :func:`parse_with_header`.

    Returns:
        Gram notation string representing *patterns*.

    Raises:
        GramParseError: If any pattern contains a value type that cannot be
            represented in gram notation (e.g. ``NullVal``).

    Example::

        from relateby.gram import parse, stringify

        patterns = parse("(a)-->(b)")
        gram = stringify(patterns)
        # "(a)-->(b)"

        # round-trip is identity up to whitespace normalization
        assert parse(gram) == parse("(a)-->(b)")
    """
    from relateby._native import gram_codec as _gram

    try:
        raw = [_pattern_to_dict(p) for p in patterns]
        return _gram.stringify(raw)
    except Exception as exc:
        raise GramParseError(input="", cause=str(exc)) from exc


def parse_with_header(input: str) -> "tuple[dict | None, list[Pattern[Subject]]]":
    """Parse gram notation, separating an optional header record from the patterns.

    A *header* is a leading bare record — a ``{key: value, ...}`` block that
    appears before any graph elements and has no identity or labels.  It is
    commonly used to store document-level metadata such as schema version or
    provenance.

    Args:
        input: Gram notation string, optionally starting with a bare record
            header, e.g. ``"{version: 1} (alice)-[:KNOWS]->(bob)"``.

    Returns:
        A two-tuple ``(header, patterns)`` where *header* is a plain ``dict``
        if a leading bare record was present, or ``None`` otherwise, and
        *patterns* is the list of ``Pattern[Subject]`` elements that follow.

    Raises:
        GramParseError: If *input* is not valid gram notation.

    Example::

        from relateby.gram import parse_with_header, stringify_with_header

        # Document with a header
        header, patterns = parse_with_header(
            "{version: 1, source: 'export'} (alice)-[:KNOWS]->(bob)"
        )
        assert header == {"version": 1, "source": "export"}
        assert len(patterns) == 1

        # Document without a header
        header2, patterns2 = parse_with_header("(alice)-[:KNOWS]->(bob)")
        assert header2 is None

        # Round-trip
        gram = stringify_with_header(header, patterns)
        header3, patterns3 = parse_with_header(gram)
        assert header3 == header
    """
    from relateby._native import gram_codec as _gram
    from relateby.pattern._decode import pattern_from_dict

    try:
        raw = _gram.parse_with_header(input)
        header = raw.get("header")
        patterns = [pattern_from_dict(d) for d in raw.get("patterns", [])]
        return header, patterns
    except Exception as exc:
        raise GramParseError(input=input, cause=str(exc)) from exc


def stringify_with_header(
    header: "dict | None",
    patterns: "list[Pattern[Subject]]",
) -> str:
    """Serialize a header record and patterns to gram notation.

    Produces a gram document whose first element is the header bare record
    followed by the serialized patterns.  Pass ``None`` as *header* to
    produce output identical to :func:`stringify`.

    Args:
        header: A plain ``dict`` of scalar values to write as the leading
            bare record, or ``None`` to omit the header entirely.
        patterns: List of ``Pattern[Subject]`` objects to serialize.

    Returns:
        Gram notation string with the header (if provided) followed by the
        patterns, separated by a newline.

    Raises:
        GramParseError: If any value in *header* or *patterns* cannot be
            represented in gram notation (e.g. ``NullVal``).

    Example::

        from relateby.gram import parse, parse_with_header, stringify_with_header

        patterns = parse("(alice)-[:KNOWS]->(bob)")
        gram = stringify_with_header({"version": 1}, patterns)
        # '{version: 1}\\n(alice)-[:KNOWS]->(bob)'

        # Omitting the header is equivalent to plain stringify
        gram_no_header = stringify_with_header(None, patterns)
        # '(alice)-[:KNOWS]->(bob)'

        # Full round-trip
        header2, patterns2 = parse_with_header(gram)
        assert header2 == {"version": 1}
    """
    from relateby._native import gram_codec as _gram

    try:
        raw = {
            "header": header,
            "patterns": [_pattern_to_dict(p) for p in patterns],
        }
        return _gram.stringify_with_header(raw)
    except Exception as exc:
        raise GramParseError(input="", cause=str(exc)) from exc


def gram_validate(input: str) -> list[str]:
    """Validate gram notation and return a list of error strings.

    Returns an empty list if the input is valid.
    """
    from relateby._native import gram_codec as _gram
    try:
        return _gram.gram_validate(input)
    except Exception as exc:
        return [str(exc)]


def round_trip(input: str) -> str:
    """Parse and re-serialize gram notation (normalizes formatting)."""
    from relateby._native import gram_codec as _gram
    try:
        return _gram.round_trip(input)
    except Exception as exc:
        raise GramParseError(input=input, cause=str(exc)) from exc


# Legacy aliases
parse_gram = parse
gram_stringify = stringify


__all__ = [
    "GramParseError",
    "parse",
    "stringify",
    "parse_with_header",
    "stringify_with_header",
    "gram_validate",
    "round_trip",
    # legacy aliases
    "parse_gram",
    "gram_stringify",
]
