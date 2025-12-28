# Test Utilities API Contracts

**Feature**: 003-test-infrastructure  
**Date**: 2025-01-27  
**Purpose**: Define API contracts for test utility functions

**Related Documentation**:
- [gram-hs CLI Testing Guide](../../../docs/gram-hs-cli-testing-guide.md) - Guide for using `gram-hs` CLI tool with `--value-only`, `--deterministic`, and `--canonical` flags for equivalence checking

## Overview

This document defines the API contracts for test utilities including equivalence checking, test helpers, and property-based test generators. These utilities support testing across the gram-rs workspace.

## Equivalence Checking API

### `check_equivalence`

Compare outputs from gram-rs and gram-hs implementations for behavioral equivalence.

**Signature**:
```rust
pub fn check_equivalence<T>(
    gram_rs_output: &T,
    gram_hs_output: &T,
    options: &EquivalenceOptions
) -> EquivalenceResult
where
    T: Serialize + PartialEq + Debug,
```

**Parameters**:
- `gram_rs_output`: Output from gram-rs implementation
- `gram_hs_output`: Output from gram-hs implementation (or test data)
- `options`: Comparison options (approximate equality, field ignore list, etc.)

**Returns**: `EquivalenceResult` containing:
- `equivalent`: Boolean indicating if outputs match
- `differences`: List of differences (if not equivalent)
- `comparison_method`: Method used for comparison

**Behavior**:
- Compares outputs using specified options
- Reports differences clearly with field-level details
- Supports approximate equality for floating-point values
- Completes within 1 second per comparison (per SC-003)

**Errors**:
- Serialization errors if outputs cannot be serialized
- Comparison errors if comparison method fails

### `check_equivalence_from_test_data`

Check equivalence using extracted test data from gram-hs.

**Signature**:
```rust
pub fn check_equivalence_from_test_data<T, F>(
    test_case: &TestCase,
    gram_rs_impl: F,
    options: &EquivalenceOptions
) -> EquivalenceResult
where
    T: Serialize + PartialEq + Debug,
    F: FnOnce(&TestCaseInput) -> T,
```

**Parameters**:
- `test_case`: Test case from extracted gram-hs data
- `gram_rs_impl`: Function that executes gram-rs implementation
- `options`: Comparison options

**Returns**: `EquivalenceResult` (same as `check_equivalence`)

**Behavior**:
- Executes gram-rs implementation with test case input
- Compares output with expected output from test case
- Uses test data format from feature 002

## Test Helper API

### `assert_patterns_equal`

Compare two patterns for equality with detailed error messages.

**Signature**:
```rust
pub fn assert_patterns_equal<V>(
    actual: &Pattern<V>,
    expected: &Pattern<V>,
    msg: &str
) -> Result<(), PatternComparisonError>
where
    V: PartialEq + Debug,
```

**Parameters**:
- `actual`: Actual pattern value
- `expected`: Expected pattern value
- `msg`: Error message prefix if comparison fails

**Returns**: `Result<(), PatternComparisonError>`

**Behavior**:
- Compares patterns deeply (value and elements)
- Provides detailed error message showing differences
- Reduces boilerplate compared to manual comparison (50%+ per SC-007)

**Errors**:
- `PatternComparisonError` with details about mismatch

### `assert_pattern_structure_valid`

Validate that a pattern has valid structure.

**Signature**:
```rust
pub fn assert_pattern_structure_valid<V>(
    pattern: &Pattern<V>,
    rules: &ValidationRules
) -> Result<(), ValidationError>
where
    V: Debug,
```

**Parameters**:
- `pattern`: Pattern to validate
- `rules`: Validation rules to apply

**Returns**: `Result<(), ValidationError>`

**Behavior**:
- Validates pattern structure according to rules
- Checks constraints (depth limits, element counts, etc.)
- Works for all valid pattern structures including edge cases

**Errors**:
- `ValidationError` with details about invalid structure

### `assert_patterns_equivalent`

Compare patterns with equivalence checking options.

**Signature**:
```rust
pub fn assert_patterns_equivalent<V>(
    pattern_a: &Pattern<V>,
    pattern_b: &Pattern<V>,
    options: &PatternComparisonOptions
) -> Result<(), PatternComparisonError>
where
    V: PartialEq + Debug,
```

**Parameters**:
- `pattern_a`: First pattern
- `pattern_b`: Second pattern
- `options`: Comparison options (deep, shallow, ignore_fields, etc.)

**Returns**: `Result<(), PatternComparisonError>`

**Behavior**:
- Compares patterns with specified options
- Supports various comparison strategies
- Used by equivalence checking utilities

## Property-Based Test Generator API

### `pattern_generator`

Generate random patterns for property-based testing.

**Signature**:
```rust
pub fn pattern_generator<V>(
    value_strategy: impl Strategy<Value = V>,
    size_range: (usize, usize)
) -> impl Strategy<Value = Pattern<V>>
where
    V: Debug,
```

**Parameters**:
- `value_strategy`: Strategy for generating pattern values
- `size_range`: Range for pattern size/complexity (min, max)

**Returns**: Strategy that generates `Pattern<V>` values

**Behavior**:
- Generates valid pattern structures conforming to data model
- Respects size constraints
- Produces patterns suitable for property testing
- Generates at least 100 test cases per property (per SC-001)

**Constraints**:
- Generated patterns must be valid (conforms to pattern data model)
- Size must be within specified range
- Recursive generation must be bounded to prevent stack overflow

### `pattern_value_generator`

Generate random values for pattern values.

**Signature**:
```rust
pub fn pattern_value_generator<T>() -> impl Strategy<Value = T>
where
    T: Arbitrary,
```

**Parameters**: None (uses type's `Arbitrary` implementation)

**Returns**: Strategy that generates `T` values

**Behavior**:
- Generates values suitable for pattern values
- Works with proptest's `Arbitrary` trait
- Can be customized per type

## Configuration Types

### `EquivalenceOptions`

```rust
pub struct EquivalenceOptions {
    pub approximate_float_equality: bool,
    pub float_tolerance: f64,
    pub ignore_fields: Vec<String>,
    pub comparison_method: ComparisonMethod,
}
```

### `PatternComparisonOptions`

```rust
pub struct PatternComparisonOptions {
    pub deep: bool,
    pub ignore_fields: Vec<String>,
    pub approximate_equality: bool,
}
```

### `ValidationRules`

```rust
pub struct ValidationRules {
    pub max_depth: Option<usize>,
    pub max_elements: Option<usize>,
    pub required_fields: Vec<String>,
}
```

## Error Types

### `EquivalenceError`

```rust
pub enum EquivalenceError {
    SerializationFailed(String),
    ComparisonFailed(String),
    TestDataInvalid(String),
}
```

### `PatternComparisonError`

```rust
pub struct PatternComparisonError {
    pub message: String,
    pub differences: Vec<Difference>,
    pub path: Vec<String>,
}
```

### `ValidationError`

```rust
pub struct ValidationError {
    pub message: String,
    pub rule_violated: String,
    pub location: Vec<String>,
}
```

## Usage Examples

### Equivalence Checking

```rust
use test_utils::equivalence::{check_equivalence, EquivalenceOptions};

let result = check_equivalence(
    &gram_rs_output,
    &gram_hs_output,
    &EquivalenceOptions::default()
);

assert!(result.equivalent, "Outputs differ: {:?}", result.differences);
```

### Pattern Comparison

```rust
use test_utils::helpers::assert_patterns_equal;

assert_patterns_equal(&actual_pattern, &expected_pattern, "Patterns should match")?;
```

### Property-Based Testing

```rust
use proptest::prelude::*;
use test_utils::generators::pattern_generator;

proptest! {
    #[test]
    fn test_pattern_equality_symmetric(a in pattern_generator(any::<String>(), (0, 10))) {
        // Property: equality is symmetric
        prop_assert_eq!(a, a);
    }
}
```

## Requirements

- All functions must be available across all workspace crates (FR-024)
- Functions must work with pattern types once defined (feature 004)
- Error messages must be clear and actionable
- Performance must meet success criteria (SC-002, SC-003, SC-007)
- API must support both unit and integration tests (FR-018)

## Notes

- Pattern types (`Pattern<V>`) are placeholders until defined in feature 004
- Generators will be implemented once pattern types are available
- Test helpers depend on pattern type definitions
- API design follows Rust conventions (snake_case, Result types, etc.)

