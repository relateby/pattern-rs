import importlib
from pathlib import Path

import pytest

try:
    importlib.import_module("relateby._native.pattern_core")
    importlib.import_module("relateby._native.gram_codec")
except ModuleNotFoundError as native_import_error:
    _NATIVE_IMPORT_ERROR = native_import_error
else:
    _NATIVE_IMPORT_ERROR = None

_PACKAGE_ROOT = Path(__file__).resolve().parents[1] / "relateby"


@pytest.mark.public_api
def test_public_stub_files_describe_supported_symbols():
    pattern_stub = (_PACKAGE_ROOT / "pattern" / "__init__.pyi").read_text(encoding="utf-8")
    gram_stub = (_PACKAGE_ROOT / "gram" / "__init__.pyi").read_text(encoding="utf-8")

    assert "class StandardGraph" in pattern_stub
    assert "def values" in pattern_stub
    assert "def from_gram" in pattern_stub
    # New operations from 047-ts-py-parity:
    assert "def pattern" in pattern_stub
    assert "def from_list" in pattern_stub
    assert "def unfold" in pattern_stub
    assert "def any_value" in pattern_stub
    assert "def all_values" in pattern_stub
    assert "def matches" in pattern_stub
    assert "def contains" in pattern_stub
    assert "def para" in pattern_stub
    assert "def combine" in pattern_stub
    assert "def depth_at" in pattern_stub
    assert "def size_at" in pattern_stub
    assert "def indices_at" in pattern_stub
    assert "class GraphQuery" in pattern_stub
    assert "def map_graph" in pattern_stub
    assert "def map_all_graph" in pattern_stub
    assert "def filter_graph" in pattern_stub
    assert "def fold_graph" in pattern_stub
    assert "def map_with_context" in pattern_stub
    assert "def para_graph" in pattern_stub
    assert "class GramParseError" in gram_stub
    assert "def parse" in gram_stub
    assert "def stringify" in gram_stub
    assert "def parse_with_header" in gram_stub
    assert "def stringify_with_header" in gram_stub
    assert "def gram_validate" in gram_stub
    assert "def round_trip" in gram_stub
    assert "parse_gram" in gram_stub      # legacy alias still present
    assert "gram_stringify" in gram_stub  # legacy alias still present

if _NATIVE_IMPORT_ERROR is not None:
    @pytest.mark.skip(reason="relateby native modules are only available from a built wheel or dev install")
    def test_public_api_requires_built_native_modules():
        raise AssertionError(str(_NATIVE_IMPORT_ERROR))
else:
    import relateby.gram as gram
    import relateby.pattern as pattern


    @pytest.mark.public_api
    def test_public_modules_expose_supported_symbols():
        assert hasattr(pattern, "Pattern")
        assert hasattr(pattern, "Subject")
        assert hasattr(pattern, "Value")
        assert hasattr(pattern, "StandardGraph")
        assert hasattr(pattern, "StringVal")

        assert hasattr(gram, "parse_gram")
        assert hasattr(gram, "gram_validate")
        assert hasattr(gram, "GramParseError")
        assert hasattr(gram, "round_trip")


    @pytest.mark.public_api
    def test_public_workflows_use_supported_imports_only():
        alice = (
            pattern.Subject.from_id("alice")
            .with_label("Person")
            .with_property("name", pattern.StringVal("Alice"))
        )
        alice_pattern = pattern.Pattern.point(alice)
        graph = pattern.StandardGraph.from_gram("(alice:Person)")

        assert alice_pattern.values()[0].identity == "alice"
        assert graph.node_count == 1
        parsed = gram.parse_gram("(alice:Person)")
        assert len(parsed) == 1
        assert parsed[0].value.identity == "alice"
        assert gram.gram_validate("(alice:Person)") == []
        assert gram.round_trip("(alice:Person)") == "(alice:Person)"


    @pytest.mark.public_api
    def test_standard_graph_from_gram_runs_from_wrapper_boundary():
        graph = pattern.StandardGraph.from_gram("(alice:Person)")

        assert graph.node_count == 1
        assert graph.node("alice") is not None


    @pytest.mark.public_api
    def test_invalid_public_workflow_raises_documented_exception_shape():
        with pytest.raises(gram.GramParseError) as exc_info:
            pattern.StandardGraph.from_gram("(alice")
        assert exc_info.value.input == "(alice"

        with pytest.raises(gram.GramParseError) as exc_info:
            gram.parse_gram("(alice")
        assert exc_info.value.input == "(alice"
