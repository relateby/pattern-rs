# relateby.gram — public API from gram-codec (no top-level gram_codec)
from relateby._native.gram_codec import ParseResult
from relateby._native.gram_codec import parse_gram as _parse_gram
from relateby._native.gram_codec import round_trip as _round_trip
from relateby._native.gram_codec import validate_gram as _validate_gram


def parse_gram(input: str) -> ParseResult:
    try:
        return _parse_gram(input)
    except ValueError as exc:
        raise ValueError(f"relateby.gram.parse_gram failed: {exc}") from exc


def validate_gram(input: str) -> bool:
    return _validate_gram(input)


def round_trip(input: str) -> str:
    try:
        return _round_trip(input)
    except ValueError as exc:
        raise ValueError(f"relateby.gram.round_trip failed: {exc}") from exc

__all__ = [
    "ParseResult",
    "parse_gram",
    "round_trip",
    "validate_gram",
]
