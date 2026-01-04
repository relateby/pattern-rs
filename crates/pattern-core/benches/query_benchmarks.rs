//! Benchmarks for query operations
//!
//! This module benchmarks the performance of pattern query operations,
//! particularly focusing on short-circuit evaluation behavior.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
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

        group.bench_with_input(BenchmarkId::new("early_match", size), size, |b, _| {
            b.iter(|| {
                // Match should be found at element 5 (short-circuit)
                black_box(pattern.any_value(|v| *v == 5))
            })
        });
    }

    group.finish();
}

/// Benchmark any_value with late match (near end)
fn bench_any_value_late_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("any_value_late_match");

    for size in [100, 1000, 10000].iter() {
        let pattern = create_large_flat_pattern(*size);

        group.bench_with_input(BenchmarkId::new("late_match", size), size, |b, _| {
            b.iter(|| {
                // Match found near the end
                black_box(pattern.any_value(|v| *v == (*size as i32 - 10)))
            })
        });
    }

    group.finish();
}

/// Benchmark any_value with no match (worst case)
fn bench_any_value_no_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("any_value_no_match");

    for size in [100, 1000, 10000].iter() {
        let pattern = create_large_flat_pattern(*size);

        group.bench_with_input(BenchmarkId::new("no_match", size), size, |b, _| {
            b.iter(|| {
                // No match - must check all values
                black_box(pattern.any_value(|v| *v < 0))
            })
        });
    }

    group.finish();
}

/// Benchmark any_value with deeply nested patterns
fn bench_any_value_deep_nesting(c: &mut Criterion) {
    let mut group = c.benchmark_group("any_value_deep_nesting");

    for depth in [10, 50, 100].iter() {
        let pattern = create_deep_pattern(*depth);

        group.bench_with_input(BenchmarkId::new("deep_pattern", depth), depth, |b, _| {
            b.iter(|| black_box(pattern.any_value(|v| *v > 50)))
        });
    }

    group.finish();
}

/// Benchmark all_values with early failure (short-circuit)
fn bench_all_values_early_failure(c: &mut Criterion) {
    let mut group = c.benchmark_group("all_values_short_circuit");

    for size in [100, 1000, 10000].iter() {
        let pattern = create_large_flat_pattern(*size);

        group.bench_with_input(BenchmarkId::new("early_failure", size), size, |b, _| {
            b.iter(|| {
                // Failure should be found at element 5 (short-circuit)
                black_box(pattern.all_values(|v| *v != 5))
            })
        });
    }

    group.finish();
}

/// Benchmark all_values with late failure (near end)
fn bench_all_values_late_failure(c: &mut Criterion) {
    let mut group = c.benchmark_group("all_values_late_failure");

    for size in [100, 1000, 10000].iter() {
        let pattern = create_large_flat_pattern(*size);

        group.bench_with_input(BenchmarkId::new("late_failure", size), size, |b, _| {
            b.iter(|| {
                // Failure found near the end
                black_box(pattern.all_values(|v| *v != (*size as i32 - 10)))
            })
        });
    }

    group.finish();
}

/// Benchmark all_values with all passing (worst case)
fn bench_all_values_all_pass(c: &mut Criterion) {
    let mut group = c.benchmark_group("all_values_all_pass");

    for size in [100, 1000, 10000].iter() {
        let pattern = create_large_flat_pattern(*size);

        group.bench_with_input(BenchmarkId::new("all_pass", size), size, |b, _| {
            b.iter(|| {
                // All values pass - must check all values
                black_box(pattern.all_values(|v| *v >= 0))
            })
        });
    }

    group.finish();
}

/// Benchmark all_values with deeply nested patterns
fn bench_all_values_deep_nesting(c: &mut Criterion) {
    let mut group = c.benchmark_group("all_values_deep_nesting");

    for depth in [10, 50, 100].iter() {
        let pattern = create_deep_pattern(*depth);

        group.bench_with_input(BenchmarkId::new("deep_pattern", depth), depth, |b, _| {
            b.iter(|| black_box(pattern.all_values(|v| *v > 0)))
        });
    }

    group.finish();
}

/// Benchmark filter operation with various predicates
fn bench_filter_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("filter");

    for size in [100, 1000, 10000].iter() {
        let pattern = create_large_flat_pattern(*size);

        // Benchmark: filter for atomic patterns
        group.bench_with_input(BenchmarkId::new("atomic_patterns", size), size, |b, _| {
            b.iter(|| black_box(pattern.filter(|p| p.is_atomic())))
        });

        // Benchmark: filter with value predicate
        group.bench_with_input(BenchmarkId::new("value_predicate", size), size, |b, _| {
            b.iter(|| black_box(pattern.filter(|p| p.value > 50)))
        });

        // Benchmark: filter with structural predicate
        group.bench_with_input(
            BenchmarkId::new("structural_predicate", size),
            size,
            |b, _| b.iter(|| black_box(pattern.filter(|p| p.length() > 0))),
        );

        // Benchmark: filter with const true (worst case - all match)
        group.bench_with_input(BenchmarkId::new("const_true", size), size, |b, _| {
            b.iter(|| black_box(pattern.filter(|_| true)))
        });
    }

    group.finish();
}

/// Benchmark filter with deeply nested patterns
fn bench_filter_deep_nesting(c: &mut Criterion) {
    let mut group = c.benchmark_group("filter_deep_nesting");

    for depth in [10, 50, 100].iter() {
        let pattern = create_deep_pattern(*depth);

        group.bench_with_input(BenchmarkId::new("deep_pattern", depth), depth, |b, _| {
            b.iter(|| black_box(pattern.filter(|p| p.value % 2 == 0)))
        });
    }

    group.finish();
}

/// Verify performance target: <100ms for 10,000 nodes (any_value)
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

/// Verify performance target: <100ms for 10,000 nodes (all_values)
#[test]
fn verify_all_values_performance_target() {
    use std::time::Instant;

    let pattern = create_large_flat_pattern(10000);

    let start = Instant::now();
    let _ = pattern.all_values(|v| *v >= 0);
    let duration = start.elapsed();

    assert!(
        duration.as_millis() < 100,
        "all_values should complete in <100ms for 10,000 nodes, took {}ms",
        duration.as_millis()
    );
}

/// Verify performance target: <200ms for 10,000 nodes (filter)
#[test]
fn verify_filter_performance_target() {
    use std::time::Instant;

    let pattern = create_large_flat_pattern(10000);

    let start = Instant::now();
    let _ = pattern.filter(|p| p.value > 5000);
    let duration = start.elapsed();

    assert!(
        duration.as_millis() < 200,
        "filter should complete in <200ms for 10,000 nodes, took {}ms",
        duration.as_millis()
    );
}

criterion_group!(
    benches,
    bench_any_value_early_match,
    bench_any_value_late_match,
    bench_any_value_no_match,
    bench_any_value_deep_nesting,
    bench_all_values_early_failure,
    bench_all_values_late_failure,
    bench_all_values_all_pass,
    bench_all_values_deep_nesting,
    bench_filter_operations,
    bench_filter_deep_nesting,
);

criterion_main!(benches);
