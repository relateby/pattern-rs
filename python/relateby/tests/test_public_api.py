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
    assert "def from_gram" in pattern_stub
    assert "class ParseResult" in gram_stub
    assert "def parse_gram" in gram_stub
    assert "def validate_gram" in gram_stub
    assert "def round_trip" in gram_stub

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
        assert hasattr(pattern, "ValidationRules")
        assert hasattr(pattern, "StandardGraph")

        assert hasattr(gram, "parse_gram")
        assert hasattr(gram, "validate_gram")
        assert hasattr(gram, "round_trip")


    @pytest.mark.public_api
    def test_public_workflows_use_supported_imports_only():
        alice = pattern.Subject("alice", {"Person"}, {
            "name": pattern.Value.string("Alice"),
            "active": True,
        })
        alice_pattern = pattern.Pattern.point(alice)
        graph = pattern.StandardGraph.from_patterns([alice_pattern])

        assert graph.node_count == 1
        assert alice.get_property("active").as_boolean() is True
        assert gram.parse_gram("(alice:Person)").pattern_count == 1
        assert gram.validate_gram("(alice:Person)") is True
        assert gram.round_trip("(alice:Person)") == "(alice:Person)"


    @pytest.mark.public_api
    def test_standard_graph_from_gram_runs_from_wrapper_boundary():
        graph = pattern.StandardGraph.from_gram("(alice:Person)")

        assert graph.node_count == 1
        assert graph.node("alice") is not None


    @pytest.mark.public_api
    def test_invalid_public_workflow_raises_documented_exception_shape():
        with pytest.raises(ValueError, match="relateby.pattern"):
            pattern.StandardGraph.from_gram("(alice")

        with pytest.raises(Exception):
            pattern.Pattern.pattern("root", [pattern.Pattern.point("child")]).validate(
                pattern.ValidationRules(max_depth=0)
            )

        with pytest.raises(ValueError, match="relateby.gram.parse_gram"):
            gram.parse_gram("(alice")
