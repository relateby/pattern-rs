//! Property tests for Pattern::any_value operation
//!
//! These tests verify mathematical properties and invariants of the any_value operation.

use pattern_core::Pattern;
use proptest::prelude::*;

// Generator for simple patterns
fn pattern_strategy() -> impl Strategy<Value = Pattern<i32>> {
    let leaf = any::<i32>().prop_map(Pattern::point);
    leaf.prop_recursive(
        3,  // max depth
        10, // max size
        5,  // items per collection
        |inner| {
            (any::<i32>(), prop::collection::vec(inner, 0..5))
                .prop_map(|(v, elements)| Pattern::pattern(v, elements))
        },
    )
}

proptest! {
    #[test]
    fn prop_any_value_const_true_always_returns_true(pattern in pattern_strategy()) {
        // T016: any_value(const true) always returns true
        prop_assert!(pattern.any_value(|_| true));
    }

    #[test]
    fn prop_any_value_const_false_always_returns_false(pattern in pattern_strategy()) {
        // T017: any_value(const false) always returns false
        prop_assert!(!pattern.any_value(|_| false));
    }

    #[test]
    fn prop_any_value_consistent_with_iterator_any(pattern in pattern_strategy()) {
        // T018: any_value consistent with any() over values()
        let predicate = |v: &i32| *v > 0;

        let any_value_result = pattern.any_value(predicate);
        let values_any_result = pattern.values().into_iter().any(|v| predicate(v));

        prop_assert_eq!(any_value_result, values_any_result,
            "any_value should match Iterator::any over values()");
    }

    #[test]
    fn prop_any_value_with_negation(pattern in pattern_strategy()) {
        // Additional property: any_value(p) || any_value(!p) should be true for non-empty patterns
        let predicate = |v: &i32| *v > 0;
        let neg_predicate = |v: &i32| *v <= 0;

        let has_positive = pattern.any_value(predicate);
        let has_non_positive = pattern.any_value(neg_predicate);

        // At least one should be true (unless pattern is somehow empty of values)
        // For patterns with values, this should always hold
        if pattern.size() > 0 {
            prop_assert!(has_positive || has_non_positive,
                "For non-empty patterns, either some value satisfies p or some satisfies !p");
        }
    }

    #[test]
    fn prop_any_value_monotonic(pattern in pattern_strategy()) {
        // If any_value(strict_predicate) is true, then any_value(loose_predicate) should also be true
        // where loose_predicate is less restrictive
        let strict = |v: &i32| *v > 10;
        let loose = |v: &i32| *v > 5;

        if pattern.any_value(strict) {
            prop_assert!(pattern.any_value(loose),
                "If a stricter predicate matches, a looser one should too");
        }
    }
}
