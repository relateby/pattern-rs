# Quickstart: Testing Infrastructure

**Feature**: 003-test-infrastructure  
**Date**: 2025-01-27

## Overview

This guide provides quick start instructions for using the testing infrastructure in gram-rs. The infrastructure includes property-based testing, equivalence checking, snapshot testing, benchmarks, and test helpers.

## Prerequisites

- Rust toolchain (MSRV: 1.70.0)
- Cargo workspace structure (from feature 002)
- Pattern types defined (feature 004) - for pattern-specific testing

## Setup

### 1. Add Test Dependencies

Add test dependencies to your crate's `Cargo.toml`:

```toml
[dev-dependencies]
proptest = { version = "1.0", features = ["wasm"] }  # Property-based testing
insta = "1.0"                                         # Snapshot testing
criterion = { version = "0.5", features = ["html_reports"] }  # Benchmarking

# Test utilities (if using separate crate)
test-utils = { path = "../test-utils" }

# Or if test utilities are in pattern-core
pattern-core = { path = "../pattern-core" }
```

### 2. Workspace-Level Benchmarks

Add benchmark configuration to workspace root `Cargo.toml`:

```toml
[[bench]]
name = "pattern_operations"
harness = false
```

## Property-Based Testing

### Basic Property Test

```rust
use proptest::prelude::*;
use test_utils::generators::pattern_generator;

proptest! {
    #[test]
    fn test_pattern_equality_symmetric(
        a in pattern_generator(any::<String>(), (0, 10))
    ) {
        // Property: equality is symmetric
        prop_assert_eq!(a, a);
    }
}
```

### Custom Property Test

```rust
use proptest::prelude::*;

#[proptest]
fn test_pattern_combination_associative(
    a in pattern_generator(any::<String>(), (0, 5)),
    b in pattern_generator(any::<String>(), (0, 5)),
    c in pattern_generator(any::<String>(), (0, 5)),
) {
    // Property: combination is associative
    let ab = a.combine(&b);
    let bc = b.combine(&c);
    let ab_c = ab.combine(&c);
    let a_bc = a.combine(&bc);
    
    prop_assert_eq!(ab_c, a_bc);
}
```

### Running Property Tests

```bash
# Run all property tests
cargo test --test property

# Run specific property test
cargo test test_pattern_equality_symmetric

# Run with more test cases
PROPTEST_CASES=1000 cargo test
```

## Equivalence Checking

### Using Test Data

```rust
use test_utils::equivalence::check_equivalence_from_test_data;
use test_utils::equivalence::EquivalenceOptions;

#[test]
fn test_equivalence_with_gram_hs() {
    let test_case = load_test_case("example_test");
    let options = EquivalenceOptions::default();
    
    let result = check_equivalence_from_test_data(
        &test_case,
        |input| {
            // Execute gram-rs implementation
            gram_rs_operation(input)
        },
        &options,
    );
    
    assert!(result.equivalent, "Outputs differ: {:?}", result.differences);
}
```

### Direct Comparison

```rust
use test_utils::equivalence::check_equivalence;

#[test]
fn test_direct_equivalence() {
    let gram_rs_output = gram_rs_operation(&input);
    let gram_hs_output = gram_hs_operation(&input);  // If available
    
    let result = check_equivalence(
        &gram_rs_output,
        &gram_hs_output,
        &EquivalenceOptions::default(),
    );
    
    assert!(result.equivalent);
}
```

## Snapshot Testing

### Basic Snapshot Test

```rust
use insta::assert_snapshot;

#[test]
fn test_pattern_serialization() {
    let pattern = create_test_pattern();
    let serialized = pattern.serialize();
    
    assert_snapshot!(serialized);
}
```

### Snapshot with Name

```rust
use insta::assert_snapshot;

#[test]
fn test_pattern_formatting() {
    let pattern = create_test_pattern();
    let formatted = format!("{}", pattern);
    
    assert_snapshot!("pattern_format", formatted);
}
```

### Reviewing Snapshots

```bash
# Run tests (snapshots will be created/compared)
cargo test

# Review snapshot changes
cargo insta review

# Accept all snapshot changes
cargo insta accept
```

## Test Helpers

### Pattern Comparison

```rust
use test_utils::helpers::assert_patterns_equal;

#[test]
fn test_pattern_operations() {
    let actual = operation(&input);
    let expected = create_expected_pattern();
    
    assert_patterns_equal(&actual, &expected, "Patterns should match")?;
}
```

### Pattern Validation

```rust
use test_utils::helpers::assert_pattern_structure_valid;
use test_utils::helpers::ValidationRules;

#[test]
fn test_pattern_structure() {
    let pattern = create_pattern();
    let rules = ValidationRules {
        max_depth: Some(10),
        max_elements: Some(100),
        required_fields: vec![],
    };
    
    assert_pattern_structure_valid(&pattern, &rules)?;
}
```

## Benchmarking

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

# View benchmark results
open target/criterion/index.html
```

## Test Data Extraction

### Extract Test Cases from gram-hs

```bash
# Extract test cases (when implemented)
./scripts/sync-tests/extract.sh ../gram-hs > tests/common/test_cases.json

# Validate extracted test cases
cargo test test_validate_test_cases
```

### Using Extracted Test Cases

```rust
use test_utils::test_data::load_test_cases;

#[test]
fn test_with_extracted_data() {
    let test_cases = load_test_cases("tests/common/test_cases.json")?;
    
    for test_case in test_cases {
        let result = gram_rs_operation(&test_case.input);
        assert_eq!(result, test_case.expected);
    }
}
```

## Workspace Integration

### Running All Tests

```bash
# Run all workspace tests
cargo test --workspace

# Run tests for specific crate
cargo test -p pattern-core

# Run property tests only
cargo test --test property --workspace
```

### CI Integration

Tests run automatically in CI:

```yaml
# .github/workflows/ci.yml
- name: Run tests
  run: cargo test --workspace

- name: Run benchmarks (optional)
  run: cargo bench --workspace -- --test
```

## Common Patterns

### Property Test with Custom Generator

```rust
proptest! {
    #[test]
    fn test_with_custom_generator(
        pattern in pattern_generator(
            any::<String>(),
            (0, 10)  // size range
        )
    ) {
        // Test property
        prop_assert!(pattern.is_valid());
    }
}
```

### Equivalence Test with Options

```rust
let options = EquivalenceOptions {
    approximate_float_equality: true,
    float_tolerance: 1e-6,
    ignore_fields: vec!["metadata".to_string()],
    comparison_method: ComparisonMethod::Json,
};

let result = check_equivalence(&output_a, &output_b, &options);
```

### Snapshot with Metadata

```rust
use insta::with_settings;

#[test]
fn test_with_settings() {
    with_settings!({
        snapshot_path => "snapshots",
        prepend_module_to_snapshot => false,
    }, {
        assert_snapshot!(output);
    });
}
```

## Troubleshooting

### Property Tests Too Slow

```rust
#[proptest(cases = 50)]  // Reduce test cases
fn test_slow_property(...) {
    // ...
}
```

### Snapshots Failing Unnecessarily

```rust
// Use snapshot filters for formatting differences
insta::with_settings!({
    filters => vec![
        (r"\d{4}-\d{2}-\d{2}", "[DATE]"),
        (r"0x[0-9a-f]+", "[HEX]"),
    ],
}, {
    assert_snapshot!(output);
});
```

### Benchmarks Inconsistent

- Ensure system is idle
- Use fixed seeds for random data
- Increase sample size
- Check for background processes

### WASM Compatibility

```rust
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_native_only() {
    // Test that requires native features
}

#[cfg(target_arch = "wasm32")]
#[test]
fn test_wasm_compatible() {
    // WASM-compatible test
}
```

## Next Steps

- Review [API contracts](./contracts/) for detailed API documentation
- See [data model](./data-model.md) for data structure details
- Check [research](./research.md) for implementation decisions
- Refer to feature 004 for pattern type definitions

## Resources

- [proptest documentation](https://docs.rs/proptest/)
- [insta documentation](https://docs.rs/insta/)
- [criterion documentation](https://docs.rs/criterion/)
- [Rust testing guide](https://doc.rust-lang.org/book/ch11-00-testing.html)

