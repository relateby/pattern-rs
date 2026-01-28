"""
Pytest configuration for pattern-core Python tests
"""
import pytest

# Add the pattern-core module to the path if needed
# This will be updated once the module is built and installed
# sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../../target/release'))

@pytest.fixture
def sample_subject_data():
    """Fixture providing sample subject data for tests"""
    return {
        "identity": "alice",
        "labels": {"Person", "Employee"},
        "properties": {
            "name": "Alice",
            "age": 30
        }
    }

@pytest.fixture
def sample_pattern_data():
    """Fixture providing sample pattern data for tests"""
    return {
        "value": "root",
        "elements": [
            {"value": "child1", "elements": []},
            {"value": "child2", "elements": []}
        ]
    }
