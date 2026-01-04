//! Tests for Pattern::all_values operation
//!
//! This module tests the all_values predicate function which checks if all
//! values in a pattern satisfy a given predicate.

use pattern_core::Pattern;

#[test]
fn test_all_values_atomic_pattern_all_match() {
    // T022: all_values with atomic pattern where all values match
    let pat = Pattern::point(5);
    assert!(pat.all_values(|v| *v > 0));
    assert!(pat.all_values(|v| *v == 5));
}

#[test]
fn test_all_values_atomic_pattern_not_all_match() {
    // T023: all_values with atomic pattern where not all values match
    let pat = Pattern::point(5);
    assert!(!pat.all_values(|v| *v > 10));
    assert!(!pat.all_values(|v| *v < 0));
}

#[test]
fn test_all_values_empty_pattern_vacuous_truth() {
    // T024: all_values with empty pattern (vacuous truth)
    // Note: In this implementation, a "point" still has a value, so there's no truly empty pattern
    // However, we can test the principle with patterns that have only values that should all pass
    let pat = Pattern::pattern(5, vec![]);
    // Even though there are no elements, the root value exists
    assert!(pat.all_values(|v| *v > 0));

    // Test with nested structure where all values should match
    let pat2 = Pattern::pattern(2, vec![Pattern::point(4), Pattern::point(6)]);
    assert!(pat2.all_values(|v| *v > 0)); // all positive
    assert!(pat2.all_values(|v| *v % 2 == 0)); // all even
}

#[test]
fn test_all_values_nested_pattern_all_match() {
    // T025: all_values with nested pattern where all values match
    let pat = Pattern::pattern(
        5,
        vec![
            Pattern::point(10),
            Pattern::pattern(3, vec![Pattern::point(15)]),
        ],
    );

    // All values are positive
    assert!(pat.all_values(|v| *v > 0));

    // All values are less than 20
    assert!(pat.all_values(|v| *v < 20));
}

#[test]
fn test_all_values_nested_pattern_one_fails() {
    // T026: all_values with nested pattern where one value fails
    let pat = Pattern::pattern(
        5,
        vec![
            Pattern::point(10),
            Pattern::pattern(-3, vec![Pattern::point(15)]),
        ],
    );

    // Not all values are positive (-3 fails)
    assert!(!pat.all_values(|v| *v > 0));

    // Not all values are greater than 10
    assert!(!pat.all_values(|v| *v > 10));
}

#[test]
fn test_all_values_deeply_nested() {
    // T027: all_values with deeply nested pattern (100+ levels)
    fn create_deep_pattern(depth: usize, value: i32) -> Pattern<i32> {
        if depth == 0 {
            Pattern::point(value)
        } else {
            Pattern::pattern(value, vec![create_deep_pattern(depth - 1, value + 1)])
        }
    }

    let deep_pat = create_deep_pattern(150, 1); // All values from 1 to 151

    // All values should be positive
    assert!(deep_pat.all_values(|v| *v > 0));

    // Not all values are greater than 100
    assert!(!deep_pat.all_values(|v| *v > 100));

    // All values should be <= 151
    assert!(deep_pat.all_values(|v| *v <= 151));
}

#[test]
fn test_all_values_large_flat_pattern() {
    // T028: all_values with large flat pattern (1000+ elements)
    let elements: Vec<Pattern<i32>> = (1..1000).map(|i| Pattern::point(i)).collect();

    let pat = Pattern::pattern(0, elements);

    // All values should be >= 0
    assert!(pat.all_values(|v| *v >= 0));

    // Not all values are positive (root is 0)
    assert!(!pat.all_values(|v| *v > 0));

    // All values should be < 1000
    assert!(pat.all_values(|v| *v < 1000));
}

#[test]
fn test_all_values_short_circuit_behavior() {
    // Additional test: Verify short-circuit behavior
    // This test uses a counter to verify that evaluation stops early on first failure
    use std::cell::Cell;

    let pat = Pattern::pattern(
        10,
        vec![
            Pattern::point(20),
            Pattern::point(5), // This fails the predicate (not > 10)
            Pattern::point(30),
            Pattern::point(40),
        ],
    );

    let counter = Cell::new(0);
    let result = pat.all_values(|v| {
        counter.set(counter.get() + 1);
        *v > 10
    });

    assert!(!result);
    // Due to pre-order traversal and short-circuit, we should evaluate:
    // 10 (root - fails), or 10 (root - passes), 20 (passes), 5 (fails, stops)
    // Total: Should stop early, not evaluate all 5 values
    assert!(
        counter.get() < 5,
        "Expected early termination, but evaluated {} values",
        counter.get()
    );
}

#[test]
fn test_all_values_vs_any_value_complementarity() {
    // Test the relationship between all_values and any_value
    let pat = Pattern::pattern(5, vec![Pattern::point(10), Pattern::point(3)]);

    // If all values are positive, then at least one value is positive
    if pat.all_values(|v| *v > 0) {
        assert!(pat.any_value(|v| *v > 0));
    }

    // Test negation relationship
    let all_positive = pat.all_values(|v| *v > 0);
    let any_non_positive = pat.any_value(|v| *v <= 0);

    // all_values(p) should be equivalent to !any_value(!p)
    assert_eq!(all_positive, !any_non_positive);
}
