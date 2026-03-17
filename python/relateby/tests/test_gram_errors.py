import importlib

import pytest

try:
    importlib.import_module("relateby._native.gram_codec")
except ModuleNotFoundError as native_import_error:
    pytestmark = pytest.mark.skip(reason=str(native_import_error))

from relateby.gram import GramParseError, gram_validate, parse_gram


def test_parse_gram_raises_structured_error_for_invalid_input():
    input_text = "not valid gram ##!!"

    with pytest.raises(GramParseError) as exc_info:
        parse_gram(input_text)

    assert exc_info.value.input == input_text
    assert exc_info.value.cause


def test_parse_gram_returns_native_patterns_for_valid_input():
    patterns = parse_gram("(alice:Person)")

    assert len(patterns) == 1
    assert patterns[0].value.identity == "alice"


def test_gram_validate_returns_empty_list_when_valid_and_errors_when_invalid():
    assert gram_validate("(alice:Person)") == []
    assert gram_validate("not valid gram ##!!")
