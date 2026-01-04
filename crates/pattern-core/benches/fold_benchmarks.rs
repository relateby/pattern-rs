//! Performance benchmarks for Pattern fold operations
//!
//! This benchmark suite verifies performance targets:
//! - SC-002: 1000 nodes < 10ms
//! - SC-003: 100 nesting levels without stack overflow
//! - SC-009: 10,000 elements < 100MB memory

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use pattern_core::Pattern;

// ============================================================================
// Helper Functions to Build Test Patterns
// ============================================================================

/// Creates a flat pattern with n elements (all atomic)
fn create_flat_pattern(size: usize) -> Pattern<i32> {
    let elements: Vec<Pattern<i32>> = (1..size).map(|i| Pattern::point(i as i32)).collect();
    Pattern::pattern(0, elements)
}

/// Creates a deeply nested pattern (linear chain)
fn create_deep_pattern(depth: usize) -> Pattern<i32> {
    let mut pattern = Pattern::point(depth as i32);
    for i in (0..depth).rev() {
        pattern = Pattern::pattern(i as i32, vec![pattern]);
    }
    pattern
}

/// Creates a balanced binary tree pattern
fn create_balanced_tree(depth: usize, value: i32) -> Pattern<i32> {
    if depth == 0 {
        Pattern::point(value)
    } else {
        Pattern::pattern(
            value,
            vec![
                create_balanced_tree(depth - 1, value * 2),
                create_balanced_tree(depth - 1, value * 2 + 1),
            ],
        )
    }
}

/// Creates a wide pattern with many siblings at one level
fn create_wide_pattern(width: usize) -> Pattern<i32> {
    let elements: Vec<Pattern<i32>> = (0..width).map(|i| Pattern::point(i as i32)).collect();
    Pattern::pattern(-1, elements)
}

// ============================================================================
// T045: Large Pattern Benchmarks (1000+ nodes)
// ============================================================================

fn bench_fold_large_flat_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("fold_large_flat");

    for size in [100, 500, 1000, 2000, 5000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let pattern = create_flat_pattern(size);
            b.iter(|| {
                let sum = pattern.fold(0i32, |acc, v| black_box(acc + v));
                black_box(sum)
            });
        });
    }

    group.finish();
}

fn bench_fold_large_balanced_trees(c: &mut Criterion) {
    let mut group = c.benchmark_group("fold_balanced_tree");

    // Depth 10 = 2^10 - 1 = 1023 nodes
    // Depth 11 = 2^11 - 1 = 2047 nodes
    // Depth 12 = 2^12 - 1 = 4095 nodes
    for depth in [8, 9, 10, 11, 12].iter() {
        let nodes = (1 << (*depth + 1)) - 1;
        group.throughput(Throughput::Elements(nodes));
        group.bench_with_input(BenchmarkId::from_parameter(depth), depth, |b, &depth| {
            let pattern = create_balanced_tree(depth, 1);
            b.iter(|| {
                let sum = pattern.fold(0i32, |acc, v| black_box(acc + v));
                black_box(sum)
            });
        });
    }

    group.finish();
}

// ============================================================================
// Fold Operation Types
// ============================================================================

fn bench_fold_operations(c: &mut Criterion) {
    let pattern = create_flat_pattern(1000);

    let mut group = c.benchmark_group("fold_operations_1000");

    // Sum operation
    group.bench_function("sum", |b| {
        b.iter(|| {
            let sum = pattern.fold(0i32, |acc, v| black_box(acc + v));
            black_box(sum)
        });
    });

    // Count operation
    group.bench_function("count", |b| {
        b.iter(|| {
            let count = pattern.fold(0usize, |acc, _| black_box(acc + 1));
            black_box(count)
        });
    });

    // Max operation
    group.bench_function("max", |b| {
        b.iter(|| {
            let max = pattern.fold(i32::MIN, |acc, v| black_box(acc.max(*v)));
            black_box(max)
        });
    });

    // Collect to vector
    group.bench_function("collect_vec", |b| {
        b.iter(|| {
            let vec = pattern.fold(Vec::new(), |mut acc, v| {
                acc.push(*v);
                black_box(acc)
            });
            black_box(vec)
        });
    });

    group.finish();
}

// ============================================================================
// Values Method Benchmarks
// ============================================================================

fn bench_values_method(c: &mut Criterion) {
    let mut group = c.benchmark_group("values_method");

    for size in [100, 500, 1000, 2000, 5000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let pattern = create_flat_pattern(size);
            b.iter(|| {
                let values = pattern.values();
                black_box(values)
            });
        });
    }

    group.finish();
}

// ============================================================================
// Deep Nesting Benchmarks (verifies stack safety)
// ============================================================================

fn bench_fold_deep_nesting(c: &mut Criterion) {
    let mut group = c.benchmark_group("fold_deep_nesting");

    for depth in [10, 50, 100, 200, 500].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(depth), depth, |b, &depth| {
            let pattern = create_deep_pattern(depth);
            b.iter(|| {
                let sum = pattern.fold(0i32, |acc, v| black_box(acc + v));
                black_box(sum)
            });
        });
    }

    group.finish();
}

// ============================================================================
// Wide Pattern Benchmarks
// ============================================================================

fn bench_fold_wide_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("fold_wide_patterns");

    for width in [100, 500, 1000, 2000, 5000].iter() {
        group.throughput(Throughput::Elements(*width as u64));
        group.bench_with_input(BenchmarkId::from_parameter(width), width, |b, &width| {
            let pattern = create_wide_pattern(width);
            b.iter(|| {
                let sum = pattern.fold(0i32, |acc, v| black_box(acc + v));
                black_box(sum)
            });
        });
    }

    group.finish();
}

// ============================================================================
// Comparison: Fold vs Values + Iterator
// ============================================================================

fn bench_fold_vs_values_iterator(c: &mut Criterion) {
    let pattern = create_flat_pattern(1000);

    let mut group = c.benchmark_group("fold_vs_iterator");

    // Using fold directly
    group.bench_function("direct_fold", |b| {
        b.iter(|| {
            let sum = pattern.fold(0i32, |acc, v| black_box(acc + v));
            black_box(sum)
        });
    });

    // Using values() then iterator fold
    group.bench_function("values_then_iter_fold", |b| {
        b.iter(|| {
            let sum: i32 = pattern
                .values()
                .iter()
                .fold(0, |acc, &&v| black_box(acc + v));
            black_box(sum)
        });
    });

    // Using values() then iterator sum
    group.bench_function("values_then_iter_sum", |b| {
        b.iter(|| {
            let sum: i32 = pattern.values().iter().map(|&&v| v).sum();
            black_box(sum)
        });
    });

    group.finish();
}

// ============================================================================
// Compose with Map Benchmarks
// ============================================================================

fn bench_map_then_fold(c: &mut Criterion) {
    let pattern = create_flat_pattern(1000);

    let mut group = c.benchmark_group("map_then_fold");

    // Map then fold
    group.bench_function("map_then_fold", |b| {
        b.iter(|| {
            let sum = pattern
                .clone()
                .map(|&v| v * 2)
                .fold(0i32, |acc, v| black_box(acc + v));
            black_box(sum)
        });
    });

    // Fold with transformation inline
    group.bench_function("fold_with_inline_transform", |b| {
        b.iter(|| {
            let sum = pattern.fold(0i32, |acc, v| black_box(acc + (v * 2)));
            black_box(sum)
        });
    });

    group.finish();
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group!(
    benches,
    bench_fold_large_flat_patterns,
    bench_fold_large_balanced_trees,
    bench_fold_operations,
    bench_values_method,
    bench_fold_deep_nesting,
    bench_fold_wide_patterns,
    bench_fold_vs_values_iterator,
    bench_map_then_fold,
);

criterion_main!(benches);
