//! Tests for Pattern::any_value operation
//!
//! This module tests the any_value predicate function which checks if at least
//! one value in a pattern satisfies a given predicate.

use pattern_core::Pattern;

#[test]
fn test_any_value_atomic_pattern_matching() {
    // T005: any_value with atomic pattern containing matching value
    let pat = Pattern::point(5);
    assert!(pat.any_value(|v| *v > 0));
    assert!(!pat.any_value(|v| *v > 10));
}

#[test]
fn test_any_value_atomic_pattern_non_matching() {
    // T006: any_value with atomic pattern containing non-matching value
    let pat = Pattern::point(5);
    assert!(!pat.any_value(|v| *v < 0));
    assert!(!pat.any_value(|v| *v > 10));
}

#[test]
fn test_any_value_nested_pattern_matches_at_different_levels() {
    // T007: any_value with nested pattern where value matches at different levels
    let pat = Pattern::pattern(
        5,
        vec![
            Pattern::point(10),
            Pattern::pattern(3, vec![Pattern::point(15)]),
        ],
    );

    // Match at root level
    assert!(pat.any_value(|v| *v == 5));

    // Match at first level element
    assert!(pat.any_value(|v| *v == 10));

    // Match at second level element
    assert!(pat.any_value(|v| *v == 15));

    // Match with predicate
    assert!(pat.any_value(|v| *v > 12));
}

#[test]
fn test_any_value_no_matching_values() {
    // T008: any_value with pattern containing no matching values
    let pat = Pattern::pattern(5, vec![Pattern::point(10), Pattern::point(3)]);

    assert!(!pat.any_value(|v| *v < 0));
    assert!(!pat.any_value(|v| *v > 20));
}

#[test]
fn test_any_value_deeply_nested() {
    // T009: any_value with deeply nested pattern (100+ levels)
    fn create_deep_pattern(depth: usize, value: i32) -> Pattern<i32> {
        if depth == 0 {
            Pattern::point(value)
        } else {
            Pattern::pattern(value, vec![create_deep_pattern(depth - 1, value + 1)])
        }
    }

    let deep_pat = create_deep_pattern(150, 0);

    // Should find value at any level
    assert!(deep_pat.any_value(|v| *v == 0)); // root
    assert!(deep_pat.any_value(|v| *v == 75)); // middle
    assert!(deep_pat.any_value(|v| *v == 150)); // leaf
    assert!(deep_pat.any_value(|v| *v > 100));

    // Should not find non-existent value
    assert!(!deep_pat.any_value(|v| *v < 0));
    assert!(!deep_pat.any_value(|v| *v > 150));
}

#[test]
fn test_any_value_large_flat_pattern() {
    // T010: any_value with large flat pattern (1000+ elements)
    let elements: Vec<Pattern<i32>> = (0..1000).map(|i| Pattern::point(i)).collect();

    let pat = Pattern::pattern(999, elements);

    // Should find values
    assert!(pat.any_value(|v| *v == 0)); // first element
    assert!(pat.any_value(|v| *v == 500)); // middle element
    assert!(pat.any_value(|v| *v == 999)); // root or last element
    assert!(pat.any_value(|v| *v > 500));

    // Should not find non-existent value
    assert!(!pat.any_value(|v| *v < 0));
    assert!(!pat.any_value(|v| *v > 1000));
}

#[test]
fn test_any_value_short_circuit_behavior() {
    // Additional test: Verify short-circuit behavior
    // This test uses a counter to verify that evaluation stops early
    use std::cell::Cell;

    let pat = Pattern::pattern(
        1,
        vec![
            Pattern::point(2),
            Pattern::point(5), // This matches
            Pattern::point(3),
            Pattern::point(4),
        ],
    );

    let counter = Cell::new(0);
    let result = pat.any_value(|v| {
        counter.set(counter.get() + 1);
        *v == 5
    });

    assert!(result);
    // Due to pre-order traversal and short-circuit, we should evaluate:
    // 1 (root), 2 (first element), 5 (second element - matches, stops)
    // Total: 3 evaluations (not all 5 values)
    assert!(
        counter.get() <= 3,
        "Expected early termination, but evaluated {} values",
        counter.get()
    );
}
