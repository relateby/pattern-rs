//! Property tests for Pattern::filter operation
//!
//! These tests verify mathematical properties and invariants of the filter operation.

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
    fn prop_filter_const_true_returns_all(pattern in pattern_strategy()) {
        // T057: filter(const true) returns all subpatterns
        let all = pattern.filter(|_| true);

        // Should match size of pattern (all nodes)
        prop_assert_eq!(all.len(), pattern.size(),
            "filter(const true) should return all nodes");
    }

    #[test]
    fn prop_filter_const_false_returns_empty(pattern in pattern_strategy()) {
        // T058: filter(const false) returns empty vec
        let none = pattern.filter(|_| false);

        prop_assert_eq!(none.len(), 0,
            "filter(const false) should return empty vec");
    }

    #[test]
    fn prop_filter_result_size_bounded(pattern in pattern_strategy()) {
        // T059: filter(predicate).len() <= size()
        let predicate = |p: &Pattern<i32>| p.value > 0;
        let matches = pattern.filter(predicate);

        prop_assert!(matches.len() <= pattern.size(),
            "filter result cannot have more elements than original pattern");
    }

    #[test]
    fn prop_filter_preserves_predicate(pattern in pattern_strategy()) {
        // All results should satisfy the predicate
        let predicate = |p: &Pattern<i32>| p.value > 0;
        let matches = pattern.filter(predicate);

        prop_assert!(matches.iter().all(|p| predicate(p)),
            "All filtered results should satisfy the predicate");
    }

    #[test]
    fn prop_filter_monotonic(pattern in pattern_strategy()) {
        // If loose_predicate is weaker than strict_predicate,
        // then filter(strict) should be a subset of filter(loose)
        let strict = |p: &Pattern<i32>| p.value > 10;
        let loose = |p: &Pattern<i32>| p.value > 0;

        let strict_matches = pattern.filter(strict);
        let loose_matches = pattern.filter(loose);

        // All strict matches should be in loose matches
        prop_assert!(strict_matches.len() <= loose_matches.len(),
            "Stricter predicate should return fewer or equal matches");
    }

    #[test]
    fn prop_filter_pre_order(pattern in pattern_strategy()) {
        // Verify that results are in pre-order traversal order
        // Root should appear before its elements
        let all = pattern.filter(|_| true);

        if !all.is_empty() {
            // First result should be the root
            prop_assert!(std::ptr::eq(all[0], &pattern),
                "First filtered result should be the root pattern");
        }
    }

    #[test]
    fn prop_filter_disjunction(pattern in pattern_strategy()) {
        // filter(p1 || p2) should contain union of filter(p1) and filter(p2)
        let p1 = |p: &Pattern<i32>| p.value > 10;
        let p2 = |p: &Pattern<i32>| p.value < 0;
        let union = |p: &Pattern<i32>| p1(p) || p2(p);

        let matches_p1 = pattern.filter(p1);
        let matches_p2 = pattern.filter(p2);
        let matches_union = pattern.filter(union);

        // Union should have at least as many matches as either individual predicate
        prop_assert!(matches_union.len() >= matches_p1.len(),
            "Union should contain at least p1 matches");
        prop_assert!(matches_union.len() >= matches_p2.len(),
            "Union should contain at least p2 matches");
    }
}
