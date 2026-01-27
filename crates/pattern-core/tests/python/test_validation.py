"""
Tests for Pattern validation and structure analysis
"""
import pytest
import pattern_core


def test_validation_rules_creation():
    """Test creating validation rules"""
    rules = pattern_core.ValidationRules(max_depth=10, max_elements=100)
    assert rules.max_depth == 10
    assert rules.max_elements == 100
    
    # Optional parameters
    rules2 = pattern_core.ValidationRules()
    assert rules2.max_depth is None
    assert rules2.max_elements is None


def test_pattern_validate_passes():
    """Test validation passes for valid patterns"""
    pattern = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.point("child1"),
        pattern_core.Pattern.point("child2")
    ])
    
    rules = pattern_core.ValidationRules(max_depth=5, max_elements=10)
    
    # Should not raise
    pattern.validate(rules)


def test_pattern_validate_fails_max_depth():
    """Test validation fails when max depth exceeded"""
    # Create deeply nested pattern (depth = 3)
    pattern = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.pattern("child", [
            pattern_core.Pattern.pattern("grandchild", [
                pattern_core.Pattern.point("great-grandchild")
            ])
        ])
    ])
    
    rules = pattern_core.ValidationRules(max_depth=2)
    
    with pytest.raises(ValueError, match="Validation error"):
        pattern.validate(rules)


def test_pattern_validate_fails_max_elements():
    """Test validation fails when max elements exceeded"""
    # Create pattern with many elements
    elements = [pattern_core.Pattern.point(f"child{i}") for i in range(10)]
    pattern = pattern_core.Pattern.pattern("root", elements)
    
    rules = pattern_core.ValidationRules(max_elements=5)
    
    with pytest.raises(ValueError, match="Validation error"):
        pattern.validate(rules)


def test_pattern_analyze_structure():
    """Test structure analysis"""
    pattern = pattern_core.Pattern.pattern("root", [
        pattern_core.Pattern.pattern("a", [
            pattern_core.Pattern.point("x")
        ]),
        pattern_core.Pattern.point("b")
    ])
    
    analysis = pattern.analyze_structure()
    
    # Check analysis properties
    assert analysis.summary is not None
    assert isinstance(analysis.summary, str)
    assert len(analysis.summary) > 0
    
    assert analysis.depth_distribution is not None
    assert isinstance(analysis.depth_distribution, list)
    
    assert analysis.element_counts is not None
    assert isinstance(analysis.element_counts, list)
    
    assert analysis.nesting_patterns is not None
    assert isinstance(analysis.nesting_patterns, list)


def test_pattern_subject_validate():
    """Test validation for PatternSubject"""
    subject = pattern_core.Subject(
        identity="alice",
        labels={"Person"},
        properties={}
    )
    
    pattern = pattern_core.PatternSubject.pattern(
        subject,
        [pattern_core.PatternSubject.point(subject)]
    )
    
    rules = pattern_core.ValidationRules(max_depth=5)
    
    # Should not raise
    pattern.validate(rules)


def test_pattern_subject_analyze_structure():
    """Test structure analysis for PatternSubject"""
    subject = pattern_core.Subject(
        identity="alice",
        labels={"Person"},
        properties={}
    )
    
    pattern = pattern_core.PatternSubject.pattern(
        subject,
        [pattern_core.PatternSubject.point(subject)]
    )
    
    analysis = pattern.analyze_structure()
    
    assert analysis.summary is not None
    assert isinstance(analysis.summary, str)
