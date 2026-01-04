//! Property tests for Pattern::all_values operation
//!
//! These tests verify mathematical properties and invariants of the all_values operation.

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
    fn prop_all_values_const_true_always_returns_true(pattern in pattern_strategy()) {
        // T034: all_values(const true) always returns true
        prop_assert!(pattern.all_values(|_| true));
    }

    #[test]
    fn prop_all_values_const_false_always_returns_false(pattern in pattern_strategy()) {
        // T035: all_values(const false) always returns false
        prop_assert!(!pattern.all_values(|_| false));
    }

    #[test]
    fn prop_all_values_consistent_with_iterator_all(pattern in pattern_strategy()) {
        // T036: all_values consistent with all() over values()
        let predicate = |v: &i32| *v > 0;

        let all_values_result = pattern.all_values(predicate);
        let values_all_result = pattern.values().into_iter().all(|v| predicate(v));

        prop_assert_eq!(all_values_result, values_all_result,
            "all_values should match Iterator::all over values()");
    }

    #[test]
    fn prop_all_values_complementary_to_any_value(pattern in pattern_strategy()) {
        // T037: all_values(p) ≡ !any_value(!p)
        let predicate = |v: &i32| *v > 0;
        let neg_predicate = |v: &i32| *v <= 0;

        let all_positive = pattern.all_values(predicate);
        let any_non_positive = pattern.any_value(neg_predicate);

        prop_assert_eq!(all_positive, !any_non_positive,
            "all_values(p) should equal !any_value(!p)");
    }

    #[test]
    fn prop_all_values_monotonic(pattern in pattern_strategy()) {
        // If all_values(loose_predicate) is true, then all_values(strict_predicate)
        // implies loose as well (if strict passes, loose should pass too)
        let loose = |v: &i32| *v > -100;
        let strict = |v: &i32| *v > 0;

        if pattern.all_values(strict) {
            prop_assert!(pattern.all_values(loose),
                "If all values satisfy a strict predicate, they should satisfy a looser one");
        }
    }

    #[test]
    fn prop_all_values_implies_any_value(pattern in pattern_strategy()) {
        // If all_values(p) is true and pattern is non-empty, then any_value(p) is also true
        let predicate = |v: &i32| *v > 0;

        if pattern.all_values(predicate) && pattern.size() > 0 {
            prop_assert!(pattern.any_value(predicate),
                "If all values satisfy predicate, then at least one does (for non-empty patterns)");
        }
    }
}
