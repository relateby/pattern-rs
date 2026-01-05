//! Property-based tests for predicate matching functions
//!
//! This test suite uses proptest to verify mathematical properties and relationships
//! between the predicate matching functions: find_first, matches, and contains.
//!
//! Properties verified:
//! - find_first consistency with filter
//! - find_first returns first element
//! - matches reflexivity
//! - matches symmetry
//! - contains reflexivity
//! - contains transitivity
//! - matches implies contains

use pattern_core::Pattern;
use proptest::prelude::*;

// ============================================================================
// Pattern generators for property testing
// ============================================================================

/// Generates arbitrary atomic patterns with i32 values
fn arb_atomic_pattern() -> impl Strategy<Value = Pattern<i32>> {
    any::<i32>().prop_map(|v| Pattern::point(v))
}

/// Generates arbitrary patterns with up to 3 levels of nesting
fn arb_nested_pattern() -> impl Strategy<Value = Pattern<i32>> {
    let leaf = any::<i32>().prop_map(|v| Pattern::point(v));

    leaf.prop_recursive(
        3,  // max depth
        10, // max total nodes
        3,  // max items per collection
        |inner| {
            (any::<i32>(), prop::collection::vec(inner, 0..=3))
                .prop_map(|(v, elements)| Pattern::pattern(v, elements))
        },
    )
}

// ============================================================================
// T054: Property test for find_first consistency with filter
// ============================================================================

proptest! {
    #[test]
    fn prop_find_first_consistent_with_filter(pattern in arb_nested_pattern()) {
        // Predicate: patterns with positive values
        let predicate = |p: &Pattern<i32>| p.value > 0;

        let first = pattern.find_first(predicate);
        let all = pattern.filter(predicate);

        // If find_first returns Some, filter should not be empty
        if first.is_some() {
            prop_assert!(!all.is_empty());
            // And first should be the same as the first element of filter
            prop_assert_eq!(first.unwrap().value, all[0].value);
        }

        // If find_first returns None, filter should be empty
        if first.is_none() {
            prop_assert!(all.is_empty());
        }
    }
}

proptest! {
    #[test]
    fn prop_find_first_none_iff_filter_empty(pattern in arb_nested_pattern()) {
        let predicate = |p: &Pattern<i32>| p.value < -1000;

        let first = pattern.find_first(predicate);
        let all = pattern.filter(predicate);

        // find_first returns None if and only if filter is empty
        prop_assert_eq!(first.is_none(), all.is_empty());
    }
}

// ============================================================================
// T055: Property test for find_first returning first element
// ============================================================================

proptest! {
    #[test]
    fn prop_find_first_is_first_in_filter(pattern in arb_nested_pattern()) {
        let predicate = |p: &Pattern<i32>| p.value % 2 == 0;  // Even values

        let first = pattern.find_first(predicate);
        let all = pattern.filter(predicate);

        if let Some(f) = first {
            prop_assert!(!all.is_empty());
            // First element from find_first should match first from filter
            // (comparing by pointer equality)
            prop_assert!(std::ptr::eq(f, all[0]));
        }
    }
}

proptest! {
    #[test]
    fn prop_find_first_satisfies_predicate(pattern in arb_nested_pattern()) {
        let predicate = |p: &Pattern<i32>| p.value > 10;

        if let Some(found) = pattern.find_first(predicate) {
            // If we found something, it must satisfy the predicate
            prop_assert!(predicate(found));
        }
    }
}

// ============================================================================
// T056: Property test for matches reflexivity
// ============================================================================

proptest! {
    #[test]
    fn prop_matches_reflexive(pattern in arb_nested_pattern()) {
        // Every pattern matches itself
        prop_assert!(pattern.matches(&pattern));
    }
}

proptest! {
    #[test]
    fn prop_matches_reflexive_atomic(value in any::<i32>()) {
        let pattern = Pattern::point(value);
        prop_assert!(pattern.matches(&pattern));
    }
}

// ============================================================================
// T057: Property test for matches symmetry
// ============================================================================

proptest! {
    #[test]
    fn prop_matches_symmetric(p1 in arb_nested_pattern(), p2 in arb_nested_pattern()) {
        // p1.matches(&p2) == p2.matches(&p1)
        prop_assert_eq!(p1.matches(&p2), p2.matches(&p1));
    }
}

proptest! {
    #[test]
    fn prop_matches_symmetric_atomic(v1 in any::<i32>(), v2 in any::<i32>()) {
        let p1 = Pattern::point(v1);
        let p2 = Pattern::point(v2);
        prop_assert_eq!(p1.matches(&p2), p2.matches(&p1));
    }
}

proptest! {
    #[test]
    fn prop_matches_identical_patterns(pattern in arb_nested_pattern()) {
        // Create a clone (identical structure)
        let clone = Pattern {
            value: pattern.value,
            elements: pattern.elements.clone(),
        };

        // Identical patterns should match
        prop_assert!(pattern.matches(&clone));
        prop_assert!(clone.matches(&pattern));
    }
}

// ============================================================================
// T058: Property test for contains reflexivity
// ============================================================================

proptest! {
    #[test]
    fn prop_contains_reflexive(pattern in arb_nested_pattern()) {
        // Every pattern contains itself
        prop_assert!(pattern.contains(&pattern));
    }
}

proptest! {
    #[test]
    fn prop_contains_reflexive_atomic(value in any::<i32>()) {
        let pattern = Pattern::point(value);
        prop_assert!(pattern.contains(&pattern));
    }
}

// ============================================================================
// T059: Property test for contains transitivity
// ============================================================================

#[test]
fn test_contains_transitive_simple() {
    // Create a simple transitive chain: a contains b, b contains c
    let c = Pattern::point(1);
    let b = Pattern::pattern(2, vec![c.clone()]);
    let a = Pattern::pattern(3, vec![b.clone()]);

    // If a contains b and b contains c, then a contains c
    assert!(a.contains(&b));
    assert!(b.contains(&c));
    assert!(a.contains(&c)); // Transitivity
}

proptest! {
    #[test]
    fn prop_contains_transitive_nested(inner in arb_atomic_pattern()) {
        // Build nested structure: outer contains middle, middle contains inner
        let middle = Pattern::pattern(42, vec![inner.clone()]);
        let outer = Pattern::pattern(99, vec![middle.clone()]);

        prop_assert!(outer.contains(&middle));
        prop_assert!(middle.contains(&inner));
        prop_assert!(outer.contains(&inner));  // Transitivity
    }
}

// ============================================================================
// T060: Property test for matches implies contains
// ============================================================================

proptest! {
    #[test]
    fn prop_matches_implies_contains(p1 in arb_nested_pattern(), p2 in arb_nested_pattern()) {
        // If p1 matches p2, then p1 contains p2 (and vice versa)
        if p1.matches(&p2) {
            prop_assert!(p1.contains(&p2));
            prop_assert!(p2.contains(&p1));
        }
    }
}

proptest! {
    #[test]
    fn prop_matches_implies_contains_atomic(v1 in any::<i32>(), v2 in any::<i32>()) {
        let p1 = Pattern::point(v1);
        let p2 = Pattern::point(v2);

        if p1.matches(&p2) {
            prop_assert!(p1.contains(&p2));
            prop_assert!(p2.contains(&p1));
        }
    }
}

// ============================================================================
// T061: Additional property tests for robustness
// ============================================================================

proptest! {
    #[test]
    fn prop_contains_element_means_contains_pattern(pattern in arb_nested_pattern()) {
        // If pattern has elements, it should contain each of them
        for element in &pattern.elements {
            prop_assert!(pattern.contains(element));
        }
    }
}

proptest! {
    #[test]
    fn prop_find_first_always_returns_valid_ref(pattern in arb_nested_pattern()) {
        let predicate = |_: &Pattern<i32>| true;  // Always matches

        // Should always find something (at least the root)
        let result = pattern.find_first(predicate);
        prop_assert!(result.is_some());

        // And it should be a valid reference (we can access its value)
        if let Some(found) = result {
            let _ = found.value;  // Should not panic
        }
    }
}

proptest! {
    #[test]
    fn prop_matches_distinguishes_different_structures(
        v in any::<i32>(),
        elements1 in prop::collection::vec(any::<i32>(), 1..=3),
        elements2 in prop::collection::vec(any::<i32>(), 1..=3)
    ) {
        // Create two patterns with same root value but potentially different element counts
        let p1 = Pattern::pattern(v, elements1.iter().map(|&e| Pattern::point(e)).collect());
        let p2 = Pattern::pattern(v, elements2.iter().map(|&e| Pattern::point(e)).collect());

        // If they have different element counts, they shouldn't match
        if elements1.len() != elements2.len() {
            prop_assert!(!p1.matches(&p2));
        }
    }
}

proptest! {
    #[test]
    fn prop_contains_not_symmetric(inner in arb_atomic_pattern()) {
        let outer = Pattern::pattern(42, vec![inner.clone()]);

        // outer contains inner, but inner doesn't contain outer (not symmetric)
        prop_assert!(outer.contains(&inner));
        prop_assert!(!inner.contains(&outer));
    }
}
