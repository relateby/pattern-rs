import importlib

import pytest

try:
    importlib.import_module("relateby._native.gram_codec")
except ModuleNotFoundError as native_import_error:
    pytestmark = pytest.mark.skip(reason=str(native_import_error))

from relateby.gram import (
    GramParseError,
    parse,
    parse_with_header,
    stringify,
    stringify_with_header,
)
from relateby.pattern import Pattern, Subject


# --- parse / stringify ---


def test_parse_returns_native_patterns():
    patterns = parse("(a)-->(b)")
    assert len(patterns) == 1
    assert all(isinstance(p, Pattern) for p in patterns)


def test_parse_empty_input_returns_empty_list():
    assert parse("") == []
    assert parse("   ") == []


def test_parse_raises_gram_parse_error_for_invalid_input():
    with pytest.raises(GramParseError) as exc_info:
        parse("not valid gram ##!!")
    assert exc_info.value.input == "not valid gram ##!!"
    assert exc_info.value.cause


def test_stringify_round_trip():
    patterns = parse("(alice:Person)-->(bob:Person)")
    gram = stringify(patterns)
    assert gram  # non-empty
    patterns2 = parse(gram)
    assert len(patterns2) == len(patterns)


def test_stringify_raises_for_null_values():
    from relateby.pattern import NullVal
    pattern = Pattern.point(Subject.from_id("alice").with_property("x", NullVal()))
    with pytest.raises(GramParseError) as exc_info:
        stringify([pattern])
    assert "not representable" in exc_info.value.cause


# --- parse_with_header ---


def test_parse_with_header_splits_header_from_patterns():
    header, patterns = parse_with_header("{version: 1} (a)-->(b)")
    assert header == {"version": 1}
    assert len(patterns) == 1


def test_parse_with_header_returns_none_when_no_header():
    header, patterns = parse_with_header("(a)-->(b)")
    assert header is None
    assert len(patterns) == 1


def test_parse_with_header_header_only():
    header, patterns = parse_with_header("{version: 1}")
    assert header == {"version": 1}
    assert patterns == []


def test_parse_with_header_empty_input():
    header, patterns = parse_with_header("")
    assert header is None
    assert patterns == []


def test_parse_with_header_raises_gram_parse_error_for_invalid_input():
    with pytest.raises(GramParseError) as exc_info:
        parse_with_header("not valid gram ##!!")
    assert exc_info.value.input == "not valid gram ##!!"


# --- stringify_with_header ---


def test_stringify_with_header_includes_header_in_output():
    patterns = parse("(a)-->(b)")
    gram = stringify_with_header({"version": 1}, patterns)
    assert "{" in gram
    assert "version" in gram


def test_stringify_with_header_none_header_equals_plain_stringify():
    patterns = parse("(a)-->(b)")
    plain = stringify(patterns)
    with_none = stringify_with_header(None, patterns)
    assert plain == with_none


def test_stringify_with_header_empty_patterns():
    gram = stringify_with_header({"version": 1}, [])
    assert "{" in gram
    assert "version" in gram


def test_stringify_with_header_full_round_trip():
    original_header = {"version": 2, "schema": "test"}
    original_patterns = parse("(alice:Person)-[:KNOWS]->(bob:Person)")

    gram = stringify_with_header(original_header, original_patterns)
    header2, patterns2 = parse_with_header(gram)

    assert header2 == original_header
    assert len(patterns2) == len(original_patterns)
