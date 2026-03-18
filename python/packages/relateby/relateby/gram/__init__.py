# relateby.gram — parse/stringify gram notation
#
# parse_gram returns list[Pattern[Subject]] (native Python dataclasses).
# Errors surface as GramParseError with .input and .cause attributes.

from __future__ import annotations

import json as _json
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from relateby.pattern import Pattern
    from relateby.pattern._subject import Subject


class GramParseError(Exception):
    """Raised when gram parsing or serialization fails."""

    def __init__(self, input: str, cause: str) -> None:
        super().__init__(f"Gram parse error: {cause}")
        self._input = input
        self._cause = cause

    @property
    def input(self) -> str:
        return self._input

    @property
    def cause(self) -> str:
        return self._cause


def parse_gram(input: str) -> "list[Pattern[Subject]]":
    """Parse gram notation and return native Pattern[Subject] objects.

    Raises:
        GramParseError: If the input is not valid gram notation.
    """
    from relateby._native import gram_codec as _gram
    from relateby.pattern._decode import pattern_from_dict

    try:
        json_str = _gram.gram_parse_to_json(input)
    except Exception as exc:
        raise GramParseError(input=input, cause=str(exc)) from exc

    try:
        raw_patterns = _json.loads(json_str)
        return [pattern_from_dict(d) for d in raw_patterns]
    except Exception as exc:
        raise GramParseError(input=input, cause=str(exc)) from exc


def gram_stringify(patterns: "list[Pattern[Subject]]") -> str:
    """Serialize Pattern[Subject] objects to gram notation.

    Raises:
        GramParseError: If serialization fails.
    """
    from relateby._native import gram_codec as _gram
    from relateby.pattern._value import value_to_dict

    def _pattern_to_dict(p: "Pattern[Subject]") -> dict:
        return {
            "subject": {
                "identity": p.value.identity,
                "labels": sorted(p.value.labels),
                "properties": {k: value_to_dict(v) for k, v in p.value.properties.items()},
            },
            "elements": [_pattern_to_dict(e) for e in p.elements],
        }

    try:
        raw = [_pattern_to_dict(p) for p in patterns]
        json_str = _json.dumps(raw)
        return _gram.gram_stringify_from_json(json_str)
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


# Legacy aliases
def round_trip(input: str) -> str:
    """Parse and re-serialize gram notation (normalizes formatting)."""
    from relateby._native import gram_codec as _gram
    try:
        return _gram.round_trip(input)
    except Exception as exc:
        raise GramParseError(input=input, cause=str(exc)) from exc


__all__ = [
    "GramParseError",
    "parse_gram",
    "gram_stringify",
    "gram_validate",
    "round_trip",
]
