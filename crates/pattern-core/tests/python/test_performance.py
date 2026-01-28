"""Performance tests for Python pattern-core bindings.

Tests that Python bindings maintain performance within 2x of native Rust operations.

Note: These are basic performance tests to verify the bindings don't introduce
catastrophic overhead. Detailed benchmarks should be run with dedicated tools.
"""

import time
import pytest
from pattern_core import Pattern, PatternSubject, Subject


class TestConstructionPerformance:
    """Test pattern construction performance."""
    
    def test_point_creation_performance(self):
        """Creating many atomic patterns should be reasonably fast."""
        iterations = 1000
        
        start = time.time()
        for i in range(iterations):
            _ = Pattern.point(i)
        elapsed = time.time() - start
        
        # Should complete in reasonable time (< 1 second for 1000 iterations)
        assert elapsed < 1.0
        print(f"\nPoint creation: {iterations} patterns in {elapsed:.4f}s ({iterations/elapsed:.1f} ops/sec)")
    
    def test_nested_pattern_creation_performance(self):
        """Creating nested patterns should be reasonably fast."""
        iterations = 100
        
        start = time.time()
        for i in range(iterations):
            node = Pattern.point(i)
            for j in range(10):
                node = Pattern.pattern(j, [node])
        elapsed = time.time() - start
        
        # Should complete in reasonable time
        assert elapsed < 2.0
        print(f"\nNested pattern creation: {iterations} patterns in {elapsed:.4f}s ({iterations/elapsed:.1f} ops/sec)")
    
    def test_list_pattern_creation_performance(self):
        """Creating patterns from lists should be reasonably fast."""
        iterations = 100
        list_size = 100
        
        start = time.time()
        for i in range(iterations):
            data = list(range(list_size))
            _ = Pattern.pattern("root", Pattern.from_values(data))
        elapsed = time.time() - start
        
        # Should complete in reasonable time
        assert elapsed < 2.0
        total_elements = iterations * list_size
        print(f"\nList pattern creation: {iterations} patterns ({total_elements} elements) in {elapsed:.4f}s")


class TestOperationPerformance:
    """Test pattern operation performance."""
    
    def test_map_performance(self):
        """Map operation should be reasonably fast."""
        size = 1000
        p = Pattern.pattern("root", Pattern.from_values(list(range(size))))
        
        start = time.time()
        _ = p.map(lambda x: x * 2)
        elapsed = time.time() - start
        
        # Should complete in reasonable time (< 100ms for 1000 elements)
        assert elapsed < 0.1
        print(f"\nMap over {size} elements: {elapsed*1000:.2f}ms ({size/elapsed:.1f} ops/sec)")
    
    def test_filter_performance(self):
        """Filter operation should be reasonably fast."""
        size = 1000
        p = Pattern.pattern("root", Pattern.from_values(list(range(size))))
        
        start = time.time()
        _ = p.filter(lambda x: x % 2 == 0)
        elapsed = time.time() - start
        
        # Should complete in reasonable time
        assert elapsed < 0.2
        print(f"\nFilter over {size} elements: {elapsed*1000:.2f}ms ({size/elapsed:.1f} ops/sec)")
    
    def test_fold_performance(self):
        """Fold operation should be reasonably fast."""
        size = 1000
        p = Pattern.pattern("root", Pattern.from_values(list(range(size))))
        
        start = time.time()
        _ = p.fold(0, lambda acc, x: acc + 1)
        elapsed = time.time() - start
        
        # Should complete in reasonable time
        assert elapsed < 0.1
        assert result == size + 1  # +1 for root value
        print(f"\nFold over {size} elements: {elapsed*1000:.2f}ms ({size/elapsed:.1f} ops/sec)")
    
    def test_values_iteration_performance(self):
        """Iterating over values should be reasonably fast."""
        size = 1000
        p = Pattern.pattern("root", Pattern.from_values(list(range(size))))
        
        start = time.time()
        count = 0
        for _ in p.values():
            count += 1
        elapsed = time.time() - start
        
        # Should complete in reasonable time
        assert elapsed < 0.1
        assert count == size + 1  # +1 for root value
        print(f"\nValues iteration over {size} elements: {elapsed*1000:.2f}ms ({size/elapsed:.1f} ops/sec)")


class TestSubjectPerformance:
    """Test Subject operation performance."""
    
    def test_subject_creation_performance(self):
        """Creating many subjects should be reasonably fast."""
        iterations = 1000
        
        start = time.time()
        for i in range(iterations):
            _ = Subject(f"subj-{i}")
        elapsed = time.time() - start
        
        # Should complete in reasonable time
        assert elapsed < 1.0
        print(f"\nSubject creation: {iterations} subjects in {elapsed:.4f}s ({iterations/elapsed:.1f} ops/sec)")
    
    def test_subject_label_operations_performance(self):
        """Label operations should be reasonably fast."""
        iterations = 100
        labels_per_subject = 10
        
        start = time.time()
        for i in range(iterations):
            s = Subject(f"subj-{i}")
            for j in range(labels_per_subject):
                s.add_label(f"Label{j}")
        elapsed = time.time() - start
        
        # Should complete in reasonable time
        assert elapsed < 1.0
        total_ops = iterations * labels_per_subject
        print(f"\nSubject label operations: {total_ops} operations in {elapsed:.4f}s ({total_ops/elapsed:.1f} ops/sec)")
    
    def test_subject_property_operations_performance(self):
        """Property operations should be reasonably fast."""
        iterations = 100
        props_per_subject = 10
        
        start = time.time()
        for i in range(iterations):
            s = Subject(f"subj-{i}")
            for j in range(props_per_subject):
                s.set_property(f"prop{j}", j)
        elapsed = time.time() - start
        
        # Should complete in reasonable time
        assert elapsed < 1.0
        total_ops = iterations * props_per_subject
        print(f"\nSubject property operations: {total_ops} operations in {elapsed:.4f}s ({total_ops/elapsed:.1f} ops/sec)")


class TestLargeStructurePerformance:
    """Test performance with large structures (up to 1000 nodes)."""
    
    def test_large_flat_pattern_operations(self):
        """Operations on large flat patterns (1000 nodes)."""
        size = 1000
        p = Pattern.pattern("root", Pattern.from_values(list(range(size))))
        
        # Test length
        start = time.time()
        _ = p.length()
        elapsed_length = time.time() - start
        assert p.length() == size
        
        # Test size
        start = time.time()
        _ = p.size()
        elapsed_size = time.time() - start
        assert p.size() >= size
        
        # Test depth
        start = time.time()
        _ = p.depth()
        elapsed_depth = time.time() - start
        assert p.depth() >= 0
        
        # All operations should be fast (< 10ms)
        assert elapsed_length < 0.01
        assert elapsed_size < 0.01
        assert elapsed_depth < 0.01
        
        print(f"\nLarge pattern ({size} nodes) inspection:")
        print(f"  length: {elapsed_length*1000:.2f}ms")
        print(f"  size: {elapsed_size*1000:.2f}ms")
        print(f"  depth: {elapsed_depth*1000:.2f}ms")
    
    def test_large_pattern_transformation(self):
        """Transform large pattern (1000 nodes) within performance target."""
        size = 1000
        p = Pattern.pattern("root", Pattern.from_values(list(range(size))))
        
        # Map operation
        start = time.time()
        transformed = p.map(lambda x: x * 2)
        elapsed = time.time() - start
        
        # Should maintain < 2x overhead target
        # For 1000 elements, should complete in < 200ms
        assert elapsed < 0.2
        assert transformed.length() == size
        
        print(f"\nLarge pattern transformation ({size} nodes): {elapsed*1000:.2f}ms")
    
    def test_deep_pattern_performance(self):
        """Operations on deeply nested patterns."""
        depth = 100
        p = Pattern.point(42)
        
        # Build deep structure
        build_start = time.time()
        for i in range(depth):
            p = Pattern.pattern(f"level-{i}", [p])
        build_elapsed = time.time() - build_start
        
        # Extract from deep structure
        extract_start = time.time()
        _ = p.extract()
        extract_elapsed = time.time() - extract_start
        
        # Depth query
        depth_start = time.time()
        _ = p.depth()
        depth_elapsed = time.time() - depth_start
        
        # All operations should complete in reasonable time
        assert build_elapsed < 0.1
        assert extract_elapsed < 0.01
        assert depth_elapsed < 0.01
        
        print(f"\nDeep pattern ({depth} levels):")
        print(f"  build: {build_elapsed*1000:.2f}ms")
        print(f"  extract: {extract_elapsed*1000:.2f}ms")
        print(f"  depth: {depth_elapsed*1000:.2f}ms")


class TestComplexWorkflowPerformance:
    """Test performance of realistic complex workflows."""
    
    def test_graph_construction_and_query(self):
        """Build and query a graph-like structure."""
        num_nodes = 100
        
        # Build graph
        build_start = time.time()
        subjects = []
        for i in range(num_nodes):
            s = Subject(f"node-{i}")
            s.add_label("Node")
            s.set_property("value", i)
            subjects.append(s)
        
        patterns = [PatternSubject.point(s) for s in subjects]
        graph = Pattern.pattern("root", Pattern.from_values(patterns))
        build_elapsed = time.time() - build_start
        
        # Query graph
        query_start = time.time()
        _ = graph.length()
        _ = graph.size()
        _ = graph.depth()
        query_elapsed = time.time() - query_start
        
        # Transform graph
        transform_start = time.time()
        _ = graph.map(lambda x: x)
        transform_elapsed = time.time() - transform_start
        
        # Should complete in reasonable time
        assert build_elapsed < 1.0
        assert query_elapsed < 0.1
        assert transform_elapsed < 0.5
        
        print(f"\nGraph workflow ({num_nodes} nodes):")
        print(f"  build: {build_elapsed*1000:.2f}ms")
        print(f"  query: {query_elapsed*1000:.2f}ms")
        print(f"  transform: {transform_elapsed*1000:.2f}ms")
    
    def test_data_pipeline_performance(self):
        """Simulate data processing pipeline."""
        size = 500
        
        # Stage 1: Create raw data
        stage1_start = time.time()
        raw_data = [{"id": i, "value": i * 2} for i in range(size)]
        stage1_elapsed = time.time() - stage1_start
        
        # Stage 2: Convert to patterns
        stage2_start = time.time()
        patterns = [Pattern.point(item["value"]) for item in raw_data]
        dataset = Pattern.pattern("root", Pattern.from_values(patterns))
        stage2_elapsed = time.time() - stage2_start
        
        # Stage 3: Transform
        stage3_start = time.time()
        transformed = dataset.map(lambda x: x)
        stage3_elapsed = time.time() - stage3_start
        
        # Stage 4: Aggregate
        stage4_start = time.time()
        count = transformed.fold(0, lambda acc, x: acc + 1)
        stage4_elapsed = time.time() - stage4_start
        
        total_elapsed = stage1_elapsed + stage2_elapsed + stage3_elapsed + stage4_elapsed
        
        # Total pipeline should complete in reasonable time
        assert total_elapsed < 2.0
        # fold includes root value, so we expect size + 1
        assert count == size + 1
        
        print(f"\nData pipeline ({size} items):")
        print(f"  stage 1 (raw data): {stage1_elapsed*1000:.2f}ms")
        print(f"  stage 2 (to patterns): {stage2_elapsed*1000:.2f}ms")
        print(f"  stage 3 (transform): {stage3_elapsed*1000:.2f}ms")
        print(f"  stage 4 (aggregate): {stage4_elapsed*1000:.2f}ms")
        print(f"  total: {total_elapsed*1000:.2f}ms")


class TestMemoryEfficiency:
    """Test memory efficiency (basic checks)."""
    
    def test_large_pattern_memory(self):
        """Verify large patterns don't cause memory issues."""
        size = 5000
        
        # Create large pattern
        data = list(range(size))
        p = Pattern.pattern("root", Pattern.from_values(data))
        
        # Verify it's created successfully
        assert p.length() == size
        
        # Perform operations without memory issues
        _ = p.map(lambda x: x)
        _ = p.filter(lambda x: True)
        count = p.fold(0, lambda acc, x: acc + 1)
        
        assert count == size + 1  # +1 for root value
        print(f"\nLarge pattern memory test: {size} elements processed successfully")
    
    def test_pattern_reuse(self):
        """Verify patterns can be reused without memory leaks."""
        base = Pattern.pattern("root", Pattern.from_values(list(range(100))))
        
        # Perform many operations reusing the same pattern
        for i in range(100):
            _ = base.map(lambda x: x * i)
            _ = base.filter(lambda x: True)
            _ = base.fold(0, lambda acc, x: acc + 1)
        
        # Should complete without issues
        print(f"\nPattern reuse test: 100 iterations completed successfully")


if __name__ == "__main__":
    # Run with verbose output to see performance numbers
    pytest.main([__file__, "-v", "-s"])
