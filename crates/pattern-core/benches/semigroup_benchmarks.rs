//! Performance benchmarks for Pattern combination operations
//!
//! These benchmarks verify that combination operations meet performance targets:
//! - <1ms for combining 1000-element patterns
//! - <100ms for folding 100 patterns
//! - Reasonable performance for deep nesting (100+ levels)

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pattern_core::Pattern;

// ============================================================================
// T046: Benchmark for Combining 1000-Element Patterns
// ============================================================================

fn bench_combine_wide_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("combine_wide");

    for size in [100, 500, 1000, 5000].iter() {
        let p1 = Pattern::pattern(
            "left".to_string(),
            (0..*size)
                .map(|i| Pattern::point(format!("e{}", i)))
                .collect(),
        );
        let p2 = Pattern::pattern(
            "right".to_string(),
            (0..*size)
                .map(|i| Pattern::point(format!("e{}", i)))
                .collect(),
        );

        group.bench_with_input(BenchmarkId::new("combine", size), size, |b, _| {
            b.iter(|| {
                let result = p1.clone().combine(p2.clone());
                black_box(result)
            });
        });
    }

    group.finish();
}

// ============================================================================
// T047: Benchmark for Combining Deep Patterns
// ============================================================================

fn bench_combine_deep_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("combine_deep");

    // Helper to create a deeply nested pattern
    fn create_deep_pattern(depth: usize) -> Pattern<String> {
        let mut pattern = Pattern::point(format!("leaf{}", depth));
        for i in (0..depth).rev() {
            pattern = Pattern::pattern(format!("level{}", i), vec![pattern]);
        }
        pattern
    }

    for depth in [10, 50, 100, 200].iter() {
        let p1 = create_deep_pattern(*depth);
        let p2 = create_deep_pattern(*depth);

        group.bench_with_input(BenchmarkId::new("combine", depth), depth, |b, _| {
            b.iter(|| {
                let result = p1.clone().combine(p2.clone());
                black_box(result)
            });
        });
    }

    group.finish();
}

// ============================================================================
// T048: Benchmark for Folding 100 Patterns
// ============================================================================

fn bench_fold_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("fold_patterns");

    for count in [10, 50, 100, 500].iter() {
        let patterns: Vec<_> = (0..*count)
            .map(|i| Pattern::pattern(format!("p{}", i), vec![Pattern::point(format!("e{}", i))]))
            .collect();

        group.bench_with_input(BenchmarkId::new("fold", count), count, |b, _| {
            b.iter(|| {
                let result = patterns
                    .clone()
                    .into_iter()
                    .reduce(|acc, p| acc.combine(p))
                    .unwrap();
                black_box(result)
            });
        });
    }

    group.finish();
}

// ============================================================================
// Additional Benchmarks
// ============================================================================

fn bench_combine_atomic(c: &mut Criterion) {
    let mut group = c.benchmark_group("combine_atomic");

    let p1 = Pattern::point("hello".to_string());
    let p2 = Pattern::point(" world".to_string());

    group.bench_function("atomic", |b| {
        b.iter(|| {
            let result = p1.clone().combine(p2.clone());
            black_box(result)
        });
    });

    group.finish();
}

fn bench_combine_mixed_structures(c: &mut Criterion) {
    let mut group = c.benchmark_group("combine_mixed");

    // Create patterns with varying structure complexity
    let atomic = Pattern::point("atomic".to_string());
    let shallow = Pattern::pattern(
        "shallow".to_string(),
        vec![
            Pattern::point("a".to_string()),
            Pattern::point("b".to_string()),
        ],
    );
    let nested = Pattern::pattern(
        "nested".to_string(),
        vec![Pattern::pattern(
            "inner".to_string(),
            vec![Pattern::point("leaf".to_string())],
        )],
    );

    group.bench_function("atomic_with_shallow", |b| {
        b.iter(|| {
            let result = atomic.clone().combine(shallow.clone());
            black_box(result)
        });
    });

    group.bench_function("shallow_with_nested", |b| {
        b.iter(|| {
            let result = shallow.clone().combine(nested.clone());
            black_box(result)
        });
    });

    group.bench_function("nested_with_nested", |b| {
        b.iter(|| {
            let result = nested.clone().combine(nested.clone());
            black_box(result)
        });
    });

    group.finish();
}

fn bench_self_combination(c: &mut Criterion) {
    let mut group = c.benchmark_group("self_combination");

    for size in [10, 100, 1000].iter() {
        let pattern = Pattern::pattern(
            "pattern".to_string(),
            (0..*size)
                .map(|i| Pattern::point(format!("e{}", i)))
                .collect(),
        );

        group.bench_with_input(BenchmarkId::new("self_combine", size), size, |b, _| {
            b.iter(|| {
                let result = pattern.clone().combine(pattern.clone());
                black_box(result)
            });
        });
    }

    group.finish();
}

fn bench_sequential_combinations(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_combinations");

    // Benchmark the difference between ((a ⊕ b) ⊕ c) and fold
    for count in [5, 10, 20].iter() {
        let patterns: Vec<_> = (0..*count)
            .map(|i| Pattern::point(format!("{}", i)))
            .collect();

        group.bench_with_input(BenchmarkId::new("manual_chain", count), count, |b, _| {
            b.iter(|| {
                let mut result = patterns[0].clone();
                for p in &patterns[1..] {
                    result = result.combine(p.clone());
                }
                black_box(result)
            });
        });

        group.bench_with_input(BenchmarkId::new("iterator_fold", count), count, |b, _| {
            b.iter(|| {
                let result = patterns
                    .clone()
                    .into_iter()
                    .reduce(|acc, p| acc.combine(p))
                    .unwrap();
                black_box(result)
            });
        });
    }

    group.finish();
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group!(
    benches,
    bench_combine_atomic,
    bench_combine_wide_patterns,
    bench_combine_deep_patterns,
    bench_fold_patterns,
    bench_combine_mixed_structures,
    bench_self_combination,
    bench_sequential_combinations,
);
criterion_main!(benches);
