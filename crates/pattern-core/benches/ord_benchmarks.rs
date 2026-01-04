//! Performance benchmarks for Pattern Ord operations
//!
//! This benchmark suite verifies performance targets:
//! - Comparison operations complete in <100ms for large patterns
//! - Sorting 10,000 patterns completes in <200ms
//! - Deep patterns (200+ levels) don't cause stack overflow
//! - Wide patterns (5,000+ elements) complete in <500ms

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use pattern_core::Pattern;

// ============================================================================
// Helper Functions to Build Test Patterns
// ============================================================================

/// Creates an atomic pattern
fn create_atomic_pattern(value: i32) -> Pattern<i32> {
    Pattern::point(value)
}

/// Creates a flat pattern with n elements (all atomic)
fn create_flat_pattern(size: usize, root_value: i32) -> Pattern<i32> {
    let elements: Vec<Pattern<i32>> = (1..=size).map(|i| Pattern::point(i as i32)).collect();
    Pattern::pattern(root_value, elements)
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

/// Creates a collection of patterns with varying values
fn create_pattern_collection(count: usize) -> Vec<Pattern<i32>> {
    (0..count)
        .map(|i| Pattern::point((i * 7919) % 10000))
        .collect()
}

// ============================================================================
// T060: Benchmark - Compare atomic patterns (baseline)
// ============================================================================

fn bench_compare_atomic(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_atomic");

    let p1 = create_atomic_pattern(100);
    let p2 = create_atomic_pattern(200);
    let p_equal = create_atomic_pattern(100);

    group.bench_function("different_values", |b| {
        b.iter(|| black_box(&p1).cmp(black_box(&p2)))
    });

    group.bench_function("equal_values", |b| {
        b.iter(|| black_box(&p1).cmp(black_box(&p_equal)))
    });

    group.finish();
}

// ============================================================================
// T061: Benchmark - Compare nested patterns (various depths)
// ============================================================================

fn bench_compare_nested(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_nested");

    for depth in [10, 50, 100, 150, 200].iter() {
        let p1 = create_deep_pattern(*depth);
        let p2 = create_deep_pattern(*depth);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("depth_{}", depth)),
            depth,
            |b, _| b.iter(|| black_box(&p1).cmp(black_box(&p2))),
        );
    }

    // Test early termination (values differ at root)
    let p_early_diff = create_deep_pattern(200);
    let mut p_early_diff_2 = create_deep_pattern(200);
    p_early_diff_2.value = 999; // Different root value

    group.bench_function("depth_200_early_diff", |b| {
        b.iter(|| black_box(&p_early_diff).cmp(black_box(&p_early_diff_2)))
    });

    group.finish();
}

// ============================================================================
// T062: Benchmark - Compare wide patterns (various widths)
// ============================================================================

fn bench_compare_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_wide");

    for width in [100, 500, 1000, 2500, 5000].iter() {
        let p1 = create_flat_pattern(*width, 0);
        let p2 = create_flat_pattern(*width, 0);

        group.throughput(Throughput::Elements(*width as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("width_{}", width)),
            width,
            |b, _| b.iter(|| black_box(&p1).cmp(black_box(&p2))),
        );
    }

    // Test early termination (first element differs)
    let p_early = create_flat_pattern(5000, 0);
    let mut p_early_diff = create_flat_pattern(5000, 0);
    if !p_early_diff.elements.is_empty() {
        p_early_diff.elements[0].value = 999; // Change first element
    }

    group.bench_function("width_5000_early_diff", |b| {
        b.iter(|| black_box(&p_early).cmp(black_box(&p_early_diff)))
    });

    group.finish();
}

// ============================================================================
// T063: Benchmark - Sort 10,000 patterns (verify <200ms target)
// ============================================================================

fn bench_sort_collection(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort_patterns");

    // Target: <200ms for 10,000 patterns
    for count in [1000, 5000, 10000].iter() {
        let patterns = create_pattern_collection(*count);

        group.throughput(Throughput::Elements(*count as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("count_{}", count)),
            count,
            |b, _| {
                b.iter_with_setup(
                    || patterns.clone(),
                    |mut data| {
                        data.sort();
                        black_box(data)
                    },
                )
            },
        );
    }

    group.finish();
}

// ============================================================================
// T064: Benchmark - Deep pattern comparison (200+ levels, verify no stack overflow)
// ============================================================================

fn bench_deep_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("deep_comparison_stress");

    // Test very deep patterns to ensure no stack overflow
    for depth in [100, 200, 250, 300].iter() {
        let p1 = create_deep_pattern(*depth);
        let p2 = create_deep_pattern(*depth);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("depth_{}", depth)),
            depth,
            |b, _| b.iter(|| black_box(&p1).cmp(black_box(&p2))),
        );
    }

    // Test balanced tree (less linear recursion)
    for depth in [8, 10, 12].iter() {
        let p1 = create_balanced_tree(*depth, 1);
        let p2 = create_balanced_tree(*depth, 1);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("balanced_depth_{}", depth)),
            depth,
            |b, _| b.iter(|| black_box(&p1).cmp(black_box(&p2))),
        );
    }

    group.finish();
}

// ============================================================================
// T065: Benchmark - Wide pattern comparison (5,000+ elements, verify <500ms target)
// ============================================================================

fn bench_wide_comparison_stress(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide_comparison_stress");

    // Target: <500ms for 5,000+ elements
    for width in [1000, 2500, 5000, 7500, 10000].iter() {
        let p1 = create_flat_pattern(*width, 0);
        let p2 = create_flat_pattern(*width, 0);

        group.throughput(Throughput::Elements(*width as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("width_{}", width)),
            width,
            |b, _| b.iter(|| black_box(&p1).cmp(black_box(&p2))),
        );
    }

    // Test worst case: difference at end
    let p_end_1 = create_flat_pattern(5000, 0);
    let mut p_end_2 = create_flat_pattern(5000, 0);
    if let Some(last) = p_end_2.elements.last_mut() {
        last.value = 999; // Change last element
    }

    group.bench_function("width_5000_diff_at_end", |b| {
        b.iter(|| black_box(&p_end_1).cmp(black_box(&p_end_2)))
    });

    group.finish();
}

// ============================================================================
// Additional Benchmarks - Real-world Scenarios
// ============================================================================

fn bench_min_max_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("min_max_operations");

    for count in [100, 1000, 10000].iter() {
        let patterns = create_pattern_collection(*count);

        group.throughput(Throughput::Elements(*count as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("min_{}", count)),
            count,
            |b, _| b.iter(|| patterns.iter().min()),
        );

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("max_{}", count)),
            count,
            |b, _| b.iter(|| patterns.iter().max()),
        );
    }

    group.finish();
}

fn bench_binary_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_search");

    let mut patterns = create_pattern_collection(10000);
    patterns.sort();

    let target = Pattern::point(5000);

    group.bench_function("search_10000_patterns", |b| {
        b.iter(|| black_box(&patterns).binary_search(black_box(&target)))
    });

    group.finish();
}

fn bench_btreeset_operations(c: &mut Criterion) {
    use std::collections::BTreeSet;

    let mut group = c.benchmark_group("btreeset_operations");

    let patterns = create_pattern_collection(1000);

    group.bench_function("insert_1000_patterns", |b| {
        b.iter_with_setup(
            || BTreeSet::new(),
            |mut set| {
                for p in &patterns {
                    set.insert(p.clone());
                }
                black_box(set)
            },
        )
    });

    let mut set: BTreeSet<Pattern<i32>> = patterns.iter().cloned().collect();
    let query = Pattern::point(5000);

    group.bench_function("contains_query", |b| {
        b.iter(|| black_box(&set).contains(black_box(&query)))
    });

    group.finish();
}

// ============================================================================
// Benchmark Groups
// ============================================================================

criterion_group!(
    benches,
    bench_compare_atomic,
    bench_compare_nested,
    bench_compare_wide,
    bench_sort_collection,
    bench_deep_comparison,
    bench_wide_comparison_stress,
    bench_min_max_operations,
    bench_binary_search,
    bench_btreeset_operations
);

criterion_main!(benches);
