//! Regression tests for short-circuit behavior with early termination
//!
//! These tests verify that any_value and all_values provide TRUE short-circuit behavior,
//! meaning they terminate traversal early (not just predicate evaluation) when a result
//! is found. This was fixed after discovering the initial fold-based implementation only
//! provided predicate-level short-circuit due to || and && operators, but still traversed
//! all nodes.

use pattern_core::Pattern;
use std::cell::Cell;

#[test]
fn verify_any_value_true_short_circuit() {
    let pat = Pattern::pattern(
        1,
        vec![
            Pattern::point(2),
            Pattern::point(5), // This should match and STOP traversal
            Pattern::point(3),
            Pattern::point(4),
        ],
    );

    let calls = Cell::new(0);

    let result = pat.any_value(|v| {
        calls.set(calls.get() + 1);
        eprintln!("any_value: Checking value {}", v);
        *v == 5
    });

    assert!(result);
    eprintln!("Total any_value calls: {}", calls.get());

    // With TRUE short-circuit, should only visit: 1, 2, 5 (stop)
    // Values 3 and 4 should NOT be visited
    assert_eq!(
        calls.get(),
        3,
        "Should visit exactly 3 nodes before finding match"
    );
}

#[test]
fn verify_all_values_true_short_circuit() {
    let pat = Pattern::pattern(
        15, // Root value that passes
        vec![
            Pattern::point(20),
            Pattern::point(5), // This fails, should STOP traversal
            Pattern::point(30),
            Pattern::point(40),
        ],
    );

    let calls = Cell::new(0);

    let result = pat.all_values(|v| {
        calls.set(calls.get() + 1);
        eprintln!("all_values: Checking value {}", v);
        *v > 10
    });

    assert!(!result);
    eprintln!("Total all_values calls: {}", calls.get());

    // With TRUE short-circuit, should only visit: 15 (pass), 20 (pass), 5 (fail, stop)
    // Values 30 and 40 should NOT be visited
    assert_eq!(
        calls.get(),
        3,
        "Should visit exactly 3 nodes before finding failure"
    );
}

#[test]
fn verify_any_value_no_early_termination_when_no_match() {
    let pat = Pattern::pattern(
        1,
        vec![Pattern::point(2), Pattern::point(3), Pattern::point(4)],
    );

    let calls = Cell::new(0);

    let result = pat.any_value(|v| {
        calls.set(calls.get() + 1);
        *v == 99 // Never matches
    });

    assert!(!result);
    // Should visit all 4 nodes when no match found
    assert_eq!(calls.get(), 4, "Should visit all nodes when no match");
}

#[test]
fn verify_all_values_no_early_termination_when_all_pass() {
    let pat = Pattern::pattern(
        1,
        vec![Pattern::point(2), Pattern::point(3), Pattern::point(4)],
    );

    let calls = Cell::new(0);

    let result = pat.all_values(|v| {
        calls.set(calls.get() + 1);
        *v > 0 // All match
    });

    assert!(result);
    // Should visit all 4 nodes when all pass
    assert_eq!(calls.get(), 4, "Should visit all nodes when all pass");
}
