//! Benchmarks for query operations
//!
//! This module benchmarks the performance of pattern query operations,
//! particularly focusing on short-circuit evaluation behavior.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use pattern_core::Pattern;

/// Create a large flat pattern with n elements
fn create_large_flat_pattern(n: usize) -> Pattern<i32> {
    let elements: Vec<Pattern<i32>> = (0..n).map(|i| Pattern::point(i as i32)).collect();
    Pattern::pattern(0, elements)
}

/// Create a deeply nested pattern with n levels
fn create_deep_pattern(depth: usize) -> Pattern<i32> {
    if depth == 0 {
        Pattern::point(0)
    } else {
        Pattern::pattern(depth as i32, vec![create_deep_pattern(depth - 1)])
    }
}

/// Benchmark any_value with early match (short-circuit)
fn bench_any_value_early_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("any_value_short_circuit");
    
    for size in [100, 1000, 10000].iter() {
        let pattern = create_large_flat_pattern(*size);
        
        group.bench_with_input(
            BenchmarkId::new("early_match", size),
            size,
            |b, _| {
                b.iter(|| {
                    // Match should be found at element 5 (short-circuit)
                    black_box(pattern.any_value(|v| *v == 5))
                })
            },
        );
    }
    
    group.finish();
}

/// Benchmark any_value with late match (near end)
fn bench_any_value_late_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("any_value_late_match");
    
    for size in [100, 1000, 10000].iter() {
        let pattern = create_large_flat_pattern(*size);
        
        group.bench_with_input(
            BenchmarkId::new("late_match", size),
            size,
            |b, _| {
                b.iter(|| {
                    // Match found near the end
                    black_box(pattern.any_value(|v| *v == (*size as i32 - 10)))
                })
            },
        );
    }
    
    group.finish();
}

/// Benchmark any_value with no match (worst case)
fn bench_any_value_no_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("any_value_no_match");
    
    for size in [100, 1000, 10000].iter() {
        let pattern = create_large_flat_pattern(*size);
        
        group.bench_with_input(
            BenchmarkId::new("no_match", size),
            size,
            |b, _| {
                b.iter(|| {
                    // No match - must check all values
                    black_box(pattern.any_value(|v| *v < 0))
                })
            },
        );
    }
    
    group.finish();
}

/// Benchmark any_value with deeply nested patterns
fn bench_any_value_deep_nesting(c: &mut Criterion) {
    let mut group = c.benchmark_group("any_value_deep_nesting");
    
    for depth in [10, 50, 100].iter() {
        let pattern = create_deep_pattern(*depth);
        
        group.bench_with_input(
            BenchmarkId::new("deep_pattern", depth),
            depth,
            |b, _| {
                b.iter(|| {
                    black_box(pattern.any_value(|v| *v > 50))
                })
            },
        );
    }
    
    group.finish();
}

/// Verify performance target: <100ms for 10,000 nodes
#[test]
fn verify_any_value_performance_target() {
    use std::time::Instant;
    
    let pattern = create_large_flat_pattern(10000);
    
    let start = Instant::now();
    let _ = pattern.any_value(|v| *v > 5000);
    let duration = start.elapsed();
    
    assert!(
        duration.as_millis() < 100,
        "any_value should complete in <100ms for 10,000 nodes, took {}ms",
        duration.as_millis()
    );
}

criterion_group!(
    benches,
    bench_any_value_early_match,
    bench_any_value_late_match,
    bench_any_value_no_match,
    bench_any_value_deep_nesting,
);

criterion_main!(benches);

