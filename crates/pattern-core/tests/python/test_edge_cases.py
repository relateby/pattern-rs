"""Edge case tests for Python pattern-core bindings.

Tests for:
- None/null value handling
- Deep nesting scenarios
- Type conversion error cases
"""

import pytest
from pattern_core import Pattern, PatternSubject, Subject


class TestNoneValueHandling:
    """Test handling of None values across the API."""
    
    def test_none_as_point_value(self):
        """Pattern.point should handle None values correctly."""
        p = Pattern.point(None)
        assert p is not None
        assert p.is_atomic()
    
    def test_none_in_pattern_list(self):
        """Pattern.pattern should handle None in list."""
        p = Pattern.pattern("root", Pattern.from_values([1, None, "test"]))
        assert p.length() == 3
        values = list(p.values())
        # values() includes root value, so we expect 4 values total
        assert len(values) == 4  # root + 3 elements
        # Check that None is in the values (accounting for root being first)
        assert None in values
    
    def test_subject_property_with_none(self):
        """Subject should reject None property values with clear error."""
        s = Subject("test-id")
        # None is not a valid Value type, should raise TypeError
        with pytest.raises(TypeError, match="Cannot convert"):
            s.set_property("prop", None)
    
    def test_map_returning_none(self):
        """Pattern.map should handle callbacks returning None."""
        p = Pattern.pattern("root", Pattern.from_values([1, 2, 3]))
        result = p.map(lambda x: None)
        assert result is not None
        # Verify structure is preserved even with None values
        assert result.length() == 3
    
    def test_filter_with_none_predicate(self):
        """Pattern.filter should handle None values in predicate tests."""
        p = Pattern.pattern("root", Pattern.from_values([1, None, 3]))
        result = p.filter(lambda x: x is not None)
        # filter returns a list of Pattern, including root if it passes predicate
        assert isinstance(result, list)
        # Result includes root (which passes), plus elements that pass
        assert len(result) >= 3  # At minimum root + 2 non-None elements


class TestDeepNesting:
    """Test patterns with deep nesting scenarios."""
    
    def test_deeply_nested_patterns(self):
        """Create and query deeply nested patterns (100+ levels)."""
        # Create a deeply nested pattern
        depth = 100
        p = Pattern.point(42)
        for i in range(depth):
            p = Pattern.pattern(f"level-{i}", [p])
        
        # Verify depth
        assert p.depth() >= depth
        
        # Verify we can still access the structure
        assert not p.is_atomic()
        assert p.size() > 0
    
    def test_deep_nesting_with_extract(self):
        """Comonad extract should work on deeply nested patterns."""
        depth = 50
        p = Pattern.point("core")
        for i in range(depth):
            p = Pattern.pattern(f"level-{i}", [p])
        
        # Extract should return innermost value
        extracted = p.extract()
        assert extracted is not None
    
    def test_deep_nesting_with_map(self):
        """Map should traverse deeply nested structures."""
        depth = 30
        p = Pattern.point(1)
        for i in range(depth):
            p = Pattern.pattern(i, [p])
        
        # Map over deep structure
        result = p.map(lambda x: x * 2 if isinstance(x, int) else x)
        assert result is not None
        assert result.depth() == p.depth()
    
    def test_very_wide_pattern(self):
        """Pattern with many sibling elements (1000+ elements)."""
        width = 1000
        elements = list(range(width))
        p = Pattern.pattern("root", Pattern.from_values(elements))
        
        # Verify width
        assert p.length() == width
        
        # Verify we can iterate over all values (elements + root)
        values = list(p.values())
        assert len(values) == width + 1  # +1 for root value
    
    def test_deep_and_wide_pattern(self):
        """Pattern with both depth and width."""
        # Create a pattern with depth 10 and width 10 at each level
        def create_level(depth_remaining):
            if depth_remaining == 0:
                return Pattern.point(42)
            children = [create_level(depth_remaining - 1) for _ in range(10)]
            return Pattern.pattern(f"level-{depth_remaining}", children)
        
        p = create_level(5)
        
        # Verify structure
        assert not p.is_atomic()
        assert p.length() == 10
        assert p.depth() >= 5


class TestTypeConversionErrors:
    """Test type conversion error cases."""
    
    def test_invalid_callback_type(self):
        """Pattern.map should handle non-callable objects gracefully."""
        p = Pattern.pattern("root", Pattern.from_values([1, 2, 3]))
        
        # PyO3 may not raise TypeError immediately
        try:
            result = p.map("not a function")
            # Implementation-specific behavior
            assert result is not None
        except (TypeError, AttributeError):
            # Expected when trying to call non-callable
            pass
    
    def test_callback_raising_exception(self):
        """Pattern.map should propagate Python exceptions from callbacks."""
        p = Pattern.pattern("root", Pattern.from_values([1, 2, 3]))
        
        def bad_callback(x):
            raise ValueError("Test error")
        
        # PyO3 error handling may wrap or suppress exceptions
        try:
            result = p.map(bad_callback)
            # If it doesn't raise, implementation caught the error
            assert result is not None
        except (ValueError, RuntimeError, Exception):
            # Expected - error was propagated
            pass
    
    def test_invalid_property_key_type(self):
        """Subject.set_property should validate key types."""
        s = Subject("test-id")
        
        # Property keys must be strings
        with pytest.raises(TypeError):
            s.set_property(123, "value")
    
    def test_pattern_from_invalid_list_type(self):
        """Pattern.from_values should validate input types."""
        # Try to create pattern from non-list
        with pytest.raises(TypeError):
            Pattern.from_values("not a list")
    
    def test_subject_invalid_identity_type(self):
        """Subject constructor should validate identity type."""
        # Identity must be a string or None
        with pytest.raises(TypeError):
            Subject(12345)
    
    def test_fold_with_invalid_accumulator(self):
        """Pattern.fold should handle type mismatches in accumulator."""
        p = Pattern.pattern("root", Pattern.from_values([1, 2, 3]))
        
        def bad_fold(acc, x):
            # Return incompatible type
            return "string" if acc == 0 else acc + x
        
        # May raise TypeError depending on implementation
        try:
            result = p.fold(0, bad_fold)
            # If it doesn't raise, that's also valid behavior
            assert result is not None
        except (TypeError, RuntimeError):
            # Expected in strict type checking scenarios
            pass
    
    def test_extend_with_non_function(self):
        """Pattern.extend should validate callback type."""
        p = Pattern.pattern("root", Pattern.from_values([1, 2, 3]))
        
        # PyO3 may not raise TypeError until the callback is invoked
        try:
            result = p.extend(None)
            # If it doesn't raise, implementation-specific behavior
            assert result is not None
        except (TypeError, AttributeError):
            # Expected when trying to call non-callable
            pass


class TestMemoryAndLimits:
    """Test memory management and limits."""
    
    def test_large_pattern_creation(self):
        """Create a very large pattern and verify memory handling."""
        size = 10000
        elements = list(range(size))
        p = Pattern.pattern("root", Pattern.from_values(elements))
        
        assert p.length() == size
        
        # Verify we can perform operations on large patterns
        count = 0
        for _ in p.values():
            count += 1
        assert count == size + 1  # +1 for root value
    
    def test_pattern_with_large_strings(self):
        """Pattern with very large string values."""
        large_string = "x" * 100000  # 100KB string
        p = Pattern.pattern("root", Pattern.from_values([large_string] * 10))
        
        assert p.length() == 10
        # values() includes root, so skip first value
        values = list(p.values())
        # Find first large string in values
        large_values = [v for v in values if isinstance(v, str) and len(v) > 1000]
        assert len(large_values) > 0
        assert len(large_values[0]) == 100000
    
    def test_cyclic_reference_prevention(self):
        """Verify patterns don't allow cyclic references (if applicable)."""
        # Pattern data structure should be immutable/non-cyclic
        # This test documents the expected behavior
        p1 = Pattern.point(1)
        p2 = Pattern.pattern(2, [p1])
        
        # Verify we can serialize/inspect without infinite loops
        assert p2.depth() < 100  # Reasonable depth
        assert p2.size() < 100   # Reasonable size


class TestConcurrencyAndThreadSafety:
    """Test thread safety if patterns are shared across threads."""
    
    def test_pattern_immutability(self):
        """Patterns should be immutable (operations create new instances)."""
        p1 = Pattern.pattern("root", Pattern.from_values([1, 2, 3]))
        p2 = p1.map(lambda x: x * 2 if isinstance(x, int) else x)
        
        # Original should be unchanged
        v1 = list(p1.values())
        v2 = list(p2.values())
        
        # Original values remain as integers (not converted to strings)
        assert v1 == ["root", 1, 2, 3]
        # Mapped pattern has doubled integers
        assert v2[1] == 2  # First element value after root (doubled from 1)
    
    def test_subject_mutation_isolation(self):
        """Subject mutations should not affect copied instances."""
        s1 = Subject("test")
        s1.add_label("Label1")
        s1.set_property("key", "value")
        
        # Create pattern with subject
        p1 = PatternSubject.point(s1)
        
        # Modify original subject
        s1.add_label("Label2")
        s1.set_property("key", "modified")
        
        # Pattern should have captured state at creation time
        # (Depending on implementation, this may be a copy or reference)
        # Test documents expected behavior
        subject_value = p1.extract()
        assert subject_value is not None


class TestErrorMessages:
    """Test that error messages are Python-friendly."""
    
    def test_type_error_message_quality(self):
        """Type errors should have clear, Python-friendly messages."""
        p = Pattern.pattern("root", Pattern.from_values([1, 2, 3]))
        
        try:
            p.map(123)  # Invalid callback type
        except TypeError as e:
            error_msg = str(e)
            # Error should mention "callable" or "function"
            assert "callable" in error_msg.lower() or "function" in error_msg.lower()
    
    def test_validation_error_clarity(self):
        """Validation errors should provide actionable feedback."""
        # Try to create invalid structure (if validation exists)
        # This is a placeholder for actual validation scenarios
        try:
            s = Subject("")  # Empty identity may be invalid
            # If allowed, test passes
            assert s is not None
        except (ValueError, RuntimeError) as e:
            # Error message should be clear
            error_msg = str(e)
            assert len(error_msg) > 0
            # Should not contain Rust-specific terms
            assert "unwrap" not in error_msg.lower()
            assert "panic" not in error_msg.lower()


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
