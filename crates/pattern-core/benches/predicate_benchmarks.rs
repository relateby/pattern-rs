//! Performance benchmarks for predicate matching functions
//!
//! This benchmark suite verifies the performance targets from the specification:
//! - SC-005: find_first with early match < 10ms for 1000-node patterns
//! - SC-006: All operations < 100ms for 1000-node patterns with 100-level depth

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pattern_core::Pattern;

// ============================================================================
// Helper functions to build test patterns
// ============================================================================

/// Creates a flat pattern with n atomic children
fn create_flat_pattern(n: usize) -> Pattern<i32> {
    let elements: Vec<_> = (0..n).map(|i| Pattern::point(i as i32)).collect();
    Pattern::pattern(-1, elements)
}

/// Creates a deeply nested pattern with d levels
fn create_deep_pattern(depth: usize) -> Pattern<i32> {
    let mut pattern = Pattern::point(0);
    for i in 1..=depth {
        pattern = Pattern::pattern(i as i32, vec![pattern]);
    }
    pattern
}

/// Creates a balanced binary tree pattern with given depth
fn create_balanced_tree(depth: usize) -> Pattern<i32> {
    if depth == 0 {
        Pattern::point(0)
    } else {
        let left = create_balanced_tree(depth - 1);
        let right = create_balanced_tree(depth - 1);
        Pattern::pattern(depth as i32, vec![left, right])
    }
}

// ============================================================================
// T064: Benchmark find_first with early match (1000 nodes, match in first 10)
// ============================================================================

fn bench_find_first_early_match(c: &mut Criterion) {
    let pattern = create_flat_pattern(1000);

    c.bench_function("find_first_early_match", |b| {
        b.iter(|| {
            // Find value 5 (which is in the first 10 nodes)
            black_box(pattern.find_first(|p| p.value == 5))
        });
    });
}

// ============================================================================
// T065: Benchmark find_first with no match (worst case)
// ============================================================================

fn bench_find_first_no_match(c: &mut Criterion) {
    let pattern = create_flat_pattern(1000);

    c.bench_function("find_first_no_match", |b| {
        b.iter(|| {
            // Search for non-existent value
            black_box(pattern.find_first(|p| p.value == 9999))
        });
    });
}

fn bench_find_first_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_first_by_size");

    for size in [100, 500, 1000, 2000].iter() {
        let pattern = create_flat_pattern(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
            b.iter(|| {
                // Find value near the end
                black_box(pattern.find_first(|p| p.value == (*size as i32 - 10)))
            });
        });
    }

    group.finish();
}

// ============================================================================
// T066: Benchmark matches for large patterns with deep nesting
// ============================================================================

fn bench_matches_large_patterns(c: &mut Criterion) {
    // Create two identical 1000-node flat patterns
    let p1 = create_flat_pattern(1000);
    let p2 = create_flat_pattern(1000);

    c.bench_function("matches_1000_nodes_identical", |b| {
        b.iter(|| black_box(p1.matches(&p2)));
    });
}

fn bench_matches_deep_nesting(c: &mut Criterion) {
    // Create two identical patterns with 100-level depth
    let p1 = create_deep_pattern(100);
    let p2 = create_deep_pattern(100);

    c.bench_function("matches_100_levels_identical", |b| {
        b.iter(|| black_box(p1.matches(&p2)));
    });
}

fn bench_matches_early_mismatch(c: &mut Criterion) {
    let p1 = create_flat_pattern(1000);
    let p2 = Pattern::pattern(-2, vec![]); // Different root

    c.bench_function("matches_early_mismatch", |b| {
        b.iter(|| black_box(p1.matches(&p2)));
    });
}

// ============================================================================
// T067: Benchmark contains for large patterns with deep nesting
// ============================================================================

fn bench_contains_large_patterns(c: &mut Criterion) {
    let pattern = create_flat_pattern(1000);
    let subpattern = Pattern::point(500); // Middle of the pattern

    c.bench_function("contains_1000_nodes_middle", |b| {
        b.iter(|| black_box(pattern.contains(&subpattern)));
    });
}

fn bench_contains_deep_nesting(c: &mut Criterion) {
    let pattern = create_deep_pattern(100);
    let subpattern = Pattern::point(0); // At the bottom

    c.bench_function("contains_100_levels_bottom", |b| {
        b.iter(|| black_box(pattern.contains(&subpattern)));
    });
}

fn bench_contains_not_found(c: &mut Criterion) {
    let pattern = create_flat_pattern(1000);
    let subpattern = Pattern::point(9999); // Not in pattern

    c.bench_function("contains_not_found", |b| {
        b.iter(|| black_box(pattern.contains(&subpattern)));
    });
}

// ============================================================================
// T068: Benchmark deep nesting to verify no stack overflow
// ============================================================================

fn bench_deep_nesting_operations(c: &mut Criterion) {
    let pattern = create_deep_pattern(100);

    c.bench_function("find_first_100_levels", |b| {
        b.iter(|| black_box(pattern.find_first(|p| p.value == 50)));
    });
}

fn bench_deep_nesting_matches(c: &mut Criterion) {
    let p1 = create_deep_pattern(120);
    let p2 = create_deep_pattern(120);

    c.bench_function("matches_120_levels", |b| {
        b.iter(|| black_box(p1.matches(&p2)));
    });
}

// ============================================================================
// Additional performance benchmarks
// ============================================================================

fn bench_balanced_tree_operations(c: &mut Criterion) {
    // Binary tree with depth 10 has 2^10 = 1024 nodes
    let tree = create_balanced_tree(10);

    c.bench_function("find_first_balanced_tree_1024", |b| {
        b.iter(|| black_box(tree.find_first(|p| p.value == 5)));
    });
}

fn bench_matches_vs_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("matches_vs_contains");

    let pattern = create_flat_pattern(500);
    let identical = create_flat_pattern(500);

    group.bench_function("matches", |b| {
        b.iter(|| black_box(pattern.matches(&identical)));
    });

    group.bench_function("contains_self", |b| {
        b.iter(|| black_box(pattern.contains(&pattern)));
    });

    group.finish();
}

// ============================================================================
// Criterion configuration
// ============================================================================

criterion_group!(
    benches,
    bench_find_first_early_match,
    bench_find_first_no_match,
    bench_find_first_sizes,
    bench_matches_large_patterns,
    bench_matches_deep_nesting,
    bench_matches_early_mismatch,
    bench_contains_large_patterns,
    bench_contains_deep_nesting,
    bench_contains_not_found,
    bench_deep_nesting_operations,
    bench_deep_nesting_matches,
    bench_balanced_tree_operations,
    bench_matches_vs_contains,
);

criterion_main!(benches);
