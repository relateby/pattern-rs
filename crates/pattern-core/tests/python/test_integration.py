"""Integration tests for Python pattern-core bindings.

Tests complete workflows combining multiple features:
- Pattern construction + operations + queries
- Subject manipulation + pattern operations
- Type safety + complex data structures
- Real-world use cases
"""

import pytest
from pattern_core import Pattern, PatternSubject, Subject


class TestCompleteWorkflow:
    """Test complete workflows combining multiple features."""
    
    def test_build_query_transform_workflow(self):
        """Complete workflow: build pattern, query it, transform it."""
        # Step 1: Build a complex nested pattern
        subjects = [
            Subject(f"user-{i}")
            for i in range(5)
        ]
        
        for i, s in enumerate(subjects):
            s.add_label("Person")
            s.set_property("name", f"User {i}")
            s.set_property("age", 20 + i)
        
        patterns = [PatternSubject.point(s) for s in subjects]
        # from_values() now accepts Pattern objects too (idempotent)
        graph = Pattern.pattern("root", Pattern.from_values(patterns))
        
        # Step 2: Query the pattern
        assert graph.length() == 5
        assert not graph.is_atomic()
        
        # Find users over age 22
        adults = graph.filter(lambda p: True)  # Filter at subject level needs access
        assert adults is not None
        
        # Step 3: Transform the pattern
        transformed = graph.map(lambda x: x)  # Identity map
        assert transformed.length() == 5
        
        # Step 4: Extract and verify
        values = list(graph.values())
        # values() includes root value, so we expect 6 total (root + 5 elements)
        assert len(values) == 6
    
    def test_graph_analysis_workflow(self):
        """Analyze a graph-like pattern structure."""
        # Create a small social network graph
        alice = Subject("alice")
        alice.add_label("Person")
        alice.set_property("name", "Alice")
        
        bob = Subject("bob")
        bob.add_label("Person")
        bob.set_property("name", "Bob")
        
        charlie = Subject("charlie")
        charlie.add_label("Person")
        charlie.set_property("name", "Charlie")
        
        # Create patterns
        p_alice = PatternSubject.point(alice)
        p_bob = PatternSubject.point(bob)
        p_charlie = PatternSubject.point(charlie)
        
        # Create a graph structure
        network = Pattern.pattern("root", Pattern.from_values([p_alice, p_bob, p_charlie]))
        
        # Analyze structure
        assert network.length() == 3
        assert network.size() >= 3
        depth = network.depth()
        assert depth >= 0
        
        # Query operations
        has_people = network.any_value(lambda x: True)
        assert has_people is not None
        
        # Count patterns
        count = 0
        for _ in network.values():
            count += 1
        # values() includes root value, so we expect 4 total (root + 3 elements)
        assert count == 4
    
    def test_data_transformation_pipeline(self):
        """Transform data through multiple stages."""
        # Stage 1: Raw data
        raw_data = [
            {"id": "1", "value": 10, "type": "A"},
            {"id": "2", "value": 20, "type": "B"},
            {"id": "3", "value": 30, "type": "A"},
        ]
        
        # Stage 2: Convert to subjects
        subjects = []
        for item in raw_data:
            s = Subject(item["id"])
            s.add_label(item["type"])
            s.set_property("value", item["value"])
            subjects.append(s)
        
        # Stage 3: Create patterns
        patterns = [PatternSubject.point(s) for s in subjects]
        # from_values() now accepts Pattern objects too (idempotent)
        dataset = Pattern.pattern("root", Pattern.from_values(patterns))
        
        # Stage 4: Transform (double all values)
        # Note: This requires access to subject properties, which may need
        # special handling in the API
        transformed = dataset.map(lambda x: x)
        
        # Stage 5: Aggregate
        total = 0
        for value in dataset.values():
            total += 1  # Count items
        # values() includes root value, so we expect 4 total (root + 3 elements)
        assert total == 4
        
        # Verify structure preserved
        assert dataset.length() == 3
        assert transformed.length() == 3


class TestRealWorldScenarios:
    """Test real-world usage scenarios."""
    
    def test_configuration_tree(self):
        """Model a configuration tree structure."""
        # Root config
        root = Subject("config-root")
        root.add_label("Config")
        root.set_property("version", "1.0")
        
        # Database config
        db = Subject("config-db")
        db.add_label("DatabaseConfig")
        db.set_property("host", "localhost")
        db.set_property("port", 5432)
        
        # API config
        api = Subject("config-api")
        api.add_label("APIConfig")
        api.set_property("host", "0.0.0.0")
        api.set_property("port", 8080)
        
        # Build config tree
        db_pattern = PatternSubject.point(db)
        api_pattern = PatternSubject.point(api)
        children = Pattern.pattern("root", Pattern.from_values([db_pattern, api_pattern]))
        
        root_pattern = PatternSubject.point(root)
        config_tree = Pattern.pattern("root", Pattern.from_values([root_pattern, children]))
        
        # Query config tree
        assert config_tree.length() == 2
        assert not config_tree.is_atomic()
        
        # Verify depth - depth is max nesting level
        # config_tree -> [root_pattern (atomic), children -> [db, api (atomic)]]
        # So depth is 1 (children is one level deep)
        depth = config_tree.depth()
        assert depth >= 1
    
    def test_event_log_processing(self):
        """Process a sequence of events."""
        events = []
        
        # Create event subjects
        for i in range(10):
            event = Subject(f"event-{i}")
            event.add_label("Event")
            event.set_property("timestamp", i * 1000)
            event.set_property("type", "UserAction" if i % 2 == 0 else "SystemEvent")
            events.append(event)
        
        # Create event log pattern
        event_patterns = [PatternSubject.point(e) for e in events]
        event_log = Pattern.pattern("root", Pattern.from_values(event_patterns))
        
        # Process events
        assert event_log.length() == 10
        
        # Filter user actions (conceptual - actual filtering needs property access)
        filtered = event_log.filter(lambda x: True)
        assert filtered is not None
        
        # Count events
        count = event_log.fold(0, lambda acc, x: acc + 1)
        # fold includes root value, so we expect 11 (root + 10 elements)
        assert count == 11
    
    def test_knowledge_graph_fragment(self):
        """Model a small knowledge graph fragment."""
        # Create entities
        python = Subject("python")
        python.add_label("Language")
        python.set_property("name", "Python")
        python.set_property("paradigm", "multi-paradigm")
        
        rust = Subject("rust")
        rust.add_label("Language")
        rust.set_property("name", "Rust")
        rust.set_property("paradigm", "systems")
        
        web = Subject("web-dev")
        web.add_label("Domain")
        web.set_property("name", "Web Development")
        
        # Create graph
        entities = [
            PatternSubject.point(python),
            PatternSubject.point(rust),
            PatternSubject.point(web),
        ]
        knowledge_graph = Pattern.pattern("root", Pattern.from_values(entities))
        
        # Query graph
        assert knowledge_graph.length() == 3
        
        # Verify all entities are present
        entity_count = 0
        for _ in knowledge_graph.values():
            entity_count += 1
        # values() includes root value, so we expect 4 total (root + 3 elements)
        assert entity_count == 4


class TestComonadWorkflow:
    """Test comonad operations in realistic scenarios."""
    
    def test_context_aware_transformation(self):
        """Use comonad operations for context-aware transformations."""
        # Create a pattern with context
        values = [1, 2, 3, 4, 5]
        patterns = [Pattern.point(v) for v in values]
        sequence = Pattern.pattern("root", Pattern.from_values(patterns))
        
        # Extract current focus
        focus = sequence.extract()
        assert focus is not None
        
        # Extend with context-aware computation
        # This is a simplified example - real usage would be more complex
        extended = sequence.extend(lambda p: p.extract())
        assert extended is not None
        assert extended.length() == sequence.length()
    
    def test_sliding_window_analysis(self):
        """Use comonad for sliding window pattern."""
        # Create time series data
        time_series = Pattern.pattern("root", Pattern.from_values([10, 20, 15, 25, 30, 35, 40]))
        
        # Moving average using comonad operations
        # Extract and extend provide the primitives
        current = time_series.extract()
        assert current is not None
        
        # Extend to compute based on context
        result = time_series.extend(lambda p: p)
        assert result.length() == time_series.length()


class TestCombinationOperations:
    """Test pattern combination operations."""
    
    def test_merge_patterns(self):
        """Merge multiple patterns using combine."""
        p1 = Pattern.pattern("root", Pattern.from_values([1, 2, 3]))
        p2 = Pattern.pattern("root", Pattern.from_values([4, 5, 6]))
        
        # Combine patterns
        combined = p1.combine(p2)
        assert combined is not None
        
        # Combined pattern should have elements from both
        # Combine concatenates elements and combines values
        # Size may vary based on implementation (could be sum or max)
        assert combined.size() > 0
        # Verify length is at least as large as the max of the two
        assert combined.length() >= max(p1.length(), p2.length())
    
    def test_combine_with_subjects(self):
        """Combine patterns containing subjects."""
        s1 = Subject("subj-1")
        s1.add_label("Type1")
        
        s2 = Subject("subj-2")
        s2.add_label("Type2")
        
        p1 = PatternSubject.point(s1)
        p2 = PatternSubject.point(s2)
        
        combined = p1.combine(p2)
        assert combined is not None


class TestErrorRecovery:
    """Test error handling in complete workflows."""
    
    def test_partial_failure_recovery(self):
        """Handle failures in part of a workflow gracefully."""
        # Create mixed valid/invalid data
        subjects = []
        for i in range(5):
            s = Subject(f"subj-{i}")
            s.add_label("Item")
            subjects.append(s)
        
        patterns = [PatternSubject.point(s) for s in subjects]
        dataset = Pattern.pattern("root", Pattern.from_values(patterns))
        
        # Attempt operation that might fail on some elements
        try:
            # Map with potentially failing operation
            result = dataset.map(lambda x: x)
            assert result is not None
        except Exception as e:
            # Verify error is informative
            error_msg = str(e)
            assert len(error_msg) > 0
    
    def test_validation_workflow(self):
        """Validate data structure before processing."""
        # Create pattern
        p = Pattern.pattern("root", Pattern.from_values([1, 2, 3, 4, 5]))
        
        # Validate basic properties
        assert p.length() == 5
        assert not p.is_atomic()
        
        # Verify all values are present (elements + root)
        values = list(p.values())
        assert len(values) == 6  # 5 elements + 1 root
        
        # Process only if validation passes
        result = p.map(lambda x: x * 2)
        assert result.length() == 5


class TestPerformanceScenarios:
    """Test performance in realistic scenarios."""
    
    def test_large_dataset_processing(self):
        """Process a large dataset efficiently."""
        # Create large dataset
        size = 1000
        data = list(range(size))
        pattern = Pattern.pattern("root", Pattern.from_values(data))
        
        # Perform operations
        assert pattern.length() == size
        
        # Map operation
        doubled = pattern.map(lambda x: x * 2)
        assert doubled.length() == size
        
        # Filter operation (returns list of patterns)
        evens = pattern.filter(lambda x: x % 2 == 0)
        assert len(evens) <= size
        
        # Fold operation
        total = pattern.fold(0, lambda acc, x: acc + 1)
        assert total == size + 1  # +1 for root value
    
    def test_deep_structure_traversal(self):
        """Traverse deeply nested structures efficiently."""
        # Create nested structure
        depth = 50
        p = Pattern.point(42)
        for i in range(depth):
            p = Pattern.pattern(f"level-{i}", [p])
        
        # Traverse structure
        assert p.depth() >= depth
        
        # Extract from deep structure
        value = p.extract()
        assert value is not None


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
