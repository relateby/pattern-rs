# Benchmark Suite API Contracts

**Feature**: 003-test-infrastructure  
**Date**: 2025-01-27  
**Purpose**: Define API contracts for benchmark suite using Criterion

## Overview

This document defines the API contracts for the benchmark suite using Criterion. Benchmarks measure performance of pattern operations and help track performance over time and detect regressions.

## Benchmark Structure

### Benchmark Function Signature

Benchmarks follow Criterion's standard structure:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn benchmark_pattern_operation(c: &mut Criterion) {
    // Benchmark implementation
}
```

### Benchmark Group

```rust
criterion_group!(benches, benchmark_pattern_operation);
criterion_main!(benches);
```

## Benchmark API Contracts

### Pattern Operation Benchmarks

**Purpose**: Measure performance of core pattern operations.

**Operations to Benchmark** (to be implemented as features are ported):
- Pattern construction
- Pattern equality comparison
- Pattern combination/merging
- Pattern transformation
- Pattern matching
- Pattern serialization/deserialization

**Signature Pattern**:
```rust
fn benchmark_<operation_name>(c: &mut Criterion) {
    c.bench_function("<operation_name>", |b| {
        b.iter(|| {
            // Operation to benchmark
            black_box(operation(input))
        })
    });
}
```

**Parameters**:
- `c`: Criterion benchmark context
- Input data: Test patterns of various sizes/complexities

**Behavior**:
- Measures operation performance across multiple iterations
- Uses `black_box` to prevent optimization
- Reports consistent results (variance <10% per SC-006)
- Completes within reasonable time

### Parameterized Benchmarks

**Purpose**: Benchmark operations with varying input sizes.

**Signature Pattern**:
```rust
fn benchmark_<operation>_with_size(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000, 10000];
    
    let mut group = c.benchmark_group("<operation>");
    for size in sizes {
        let input = generate_test_pattern(size);
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &input,
            |b, input| {
                b.iter(|| {
                    black_box(operation(input))
                })
            },
        );
    }
    group.finish();
}
```

**Behavior**:
- Tests operation with different input sizes
- Provides performance scaling information
- Helps identify performance bottlenecks

### Throughput Benchmarks

**Purpose**: Measure throughput (operations per second).

**Signature Pattern**:
```rust
fn benchmark_<operation>_throughput(c: &mut Criterion) {
    c.bench_function("<operation> throughput", |b| {
        b.iter(|| {
            let result = black_box(operation(input));
            // Measure throughput
        })
    })
    .throughput(Throughput::Elements(input_size));
}
```

**Behavior**:
- Reports throughput metrics
- Useful for operations that process multiple items
- Helps track performance improvements

## Benchmark Configuration

### Criterion Configuration

```rust
use criterion::Criterion;

fn configure_criterion() -> Criterion {
    Criterion::default()
        .sample_size(100)        // Number of samples
        .warm_up_time(Duration::from_secs(1))  // Warm-up time
        .measurement_time(Duration::from_secs(3))  // Measurement time
        .confidence_level(0.95)  // Confidence level
        .significance_level(0.01)  // Significance level
}
```

### Workspace-Level Configuration

Benchmarks are configured at workspace level in `Cargo.toml`:

```toml
[[bench]]
name = "pattern_operations"
harness = false
```

## Benchmark Organization

### File Structure

```
benches/
├── pattern_operations.rs    # Core pattern operation benchmarks
├── codec_operations.rs      # Serialization/deserialization benchmarks
└── lib.rs                   # Shared benchmark utilities
```

### Benchmark Naming

- Use descriptive names: `benchmark_pattern_equality`, `benchmark_pattern_construction`
- Group related benchmarks: `benchmark_pattern_operations_*`
- Use parameterized names for size variants: `benchmark_operation_size_100`

## Performance Requirements

### Consistency

- Benchmark results must be consistent across runs (variance <10% per SC-006)
- Use appropriate sample sizes and measurement times
- Account for system variability

### Reproducibility

- Benchmarks must be reproducible
- Use fixed seeds for random data generation
- Document benchmark environment

### Reporting

- Benchmarks report:
  - Mean execution time
  - Standard deviation
  - Throughput (if applicable)
  - Comparison with previous runs (if available)

## WASM Compatibility

### Conditional Compilation

Benchmarks may need conditional compilation for WASM:

```rust
#[cfg(not(target_arch = "wasm32"))]
fn benchmark_pattern_operation(c: &mut Criterion) {
    // Benchmark implementation
}

#[cfg(target_arch = "wasm32")]
fn benchmark_pattern_operation_wasm(_c: &mut Criterion) {
    // Simplified or disabled benchmarks for WASM
    // WASM timing limitations may require alternative approach
}
```

**Note**: Criterion relies on system time which is limited in WASM. WASM benchmarks may need alternative timing approaches or be disabled.

## Usage Examples

### Basic Benchmark

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_pattern_construction(c: &mut Criterion) {
    c.bench_function("construct_pattern", |b| {
        b.iter(|| {
            let pattern = Pattern::new("value", vec![]);
            black_box(pattern)
        })
    });
}

criterion_group!(benches, benchmark_pattern_construction);
criterion_main!(benches);
```

### Parameterized Benchmark

```rust
fn benchmark_pattern_equality_by_size(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000];
    let mut group = c.benchmark_group("pattern_equality");
    
    for size in sizes {
        let pattern_a = generate_test_pattern(size);
        let pattern_b = generate_test_pattern(size);
        
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &(pattern_a, pattern_b),
            |b, (a, b)| {
                b.iter(|| {
                    black_box(a == b)
                })
            },
        );
    }
    
    group.finish();
}
```

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench pattern_operations

# Run with specific target
cargo bench --target x86_64-unknown-linux-gnu
```

## Requirements

- Benchmarks must be executable independently of test suite (FR-023)
- Benchmarks must provide consistent, reproducible results (FR-012, SC-006)
- Benchmarks must support measuring individual operations (FR-013)
- Benchmarks must integrate with workspace structure
- Benchmarks must support native Rust targets (WASM support is optional/conditional)

## Notes

- Pattern types are placeholders until defined in feature 004
- Benchmarks will be implemented as pattern operations are ported
- Performance targets will be established as baseline measurements are collected
- Benchmark suite focuses on core operations initially, expanding as features are added

