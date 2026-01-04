//! Property-based tests for Pattern combination associativity
//!
//! These tests use proptest to verify that pattern combination is associative:
//! (a ⊕ b) ⊕ c = a ⊕ (b ⊕ c) for all patterns a, b, c
//!
//! This is the defining property of the semigroup operation.

use pattern_core::{Combinable, Pattern};
use proptest::prelude::*;

// ============================================================================
// T020: Proptest Strategy for Generating Random Pattern<String>
// ============================================================================

/// Strategy for generating atomic patterns (no elements)
fn atomic_pattern() -> impl Strategy<Value = Pattern<String>> {
    any::<String>().prop_map(Pattern::point)
}

/// Strategy for generating shallow patterns (1-2 levels, 1-5 elements)
fn shallow_pattern() -> impl Strategy<Value = Pattern<String>> {
    (
        any::<String>(),
        prop::collection::vec(atomic_pattern(), 0..=5),
    )
        .prop_map(|(value, elements)| Pattern::pattern(value, elements))
}

/// Strategy for generating patterns with varying depths (0-10 levels)
fn varying_depth_pattern(max_depth: u32) -> BoxedStrategy<Pattern<String>> {
    let leaf = any::<String>().prop_map(Pattern::point).boxed();

    leaf.prop_recursive(
        max_depth, // Max depth
        256,       // Max nodes
        10,        // Items per collection
        |inner| {
            (any::<String>(), prop::collection::vec(inner, 0..=5))
                .prop_map(|(value, elements)| Pattern::pattern(value, elements))
                .boxed()
        },
    )
    .boxed()
}

/// Strategy for generating deeply nested patterns (20-50 levels)
fn deep_pattern() -> impl Strategy<Value = Pattern<String>> {
    varying_depth_pattern(50)
}

/// Strategy for generating wide patterns (many elements)
fn wide_pattern() -> impl Strategy<Value = Pattern<String>> {
    (
        any::<String>(),
        prop::collection::vec(atomic_pattern(), 10..=100),
    )
        .prop_map(|(value, elements)| Pattern::pattern(value, elements))
}

// ============================================================================
// T021: Associativity Property Test for Atomic Patterns
// ============================================================================

proptest! {
    #[test]
    fn test_associativity_atomic_patterns(
        a in atomic_pattern(),
        b in atomic_pattern(),
        c in atomic_pattern(),
    ) {
        let left = a.clone().combine(b.clone()).combine(c.clone());
        let right = a.combine(b.combine(c));

        prop_assert_eq!(left, right, "Associativity failed for atomic patterns");
    }
}

// ============================================================================
// T022: Associativity Property Test for Varying Depths
// ============================================================================

proptest! {
    #[test]
    fn test_associativity_varying_depths(
        a in varying_depth_pattern(10),
        b in varying_depth_pattern(10),
        c in varying_depth_pattern(10),
    ) {
        let left = a.clone().combine(b.clone()).combine(c.clone());
        let right = a.combine(b.combine(c));

        prop_assert_eq!(left, right, "Associativity failed for patterns with varying depths");
    }
}

// ============================================================================
// T023: Associativity Property Test for Varying Element Counts
// ============================================================================

proptest! {
    #[test]
    fn test_associativity_varying_element_counts(
        a in shallow_pattern(),
        b in shallow_pattern(),
        c in shallow_pattern(),
    ) {
        let left = a.clone().combine(b.clone()).combine(c.clone());
        let right = a.combine(b.combine(c));

        prop_assert_eq!(left, right, "Associativity failed for patterns with varying element counts");
    }
}

// ============================================================================
// T024: Associativity Property Test for Deeply Nested Patterns
// ============================================================================

proptest! {
    #[test]
    fn test_associativity_deep_nesting(
        a in deep_pattern(),
        b in deep_pattern(),
        c in deep_pattern(),
    ) {
        let left = a.clone().combine(b.clone()).combine(c.clone());
        let right = a.combine(b.combine(c));

        prop_assert_eq!(left, right, "Associativity failed for deeply nested patterns");
    }
}

// ============================================================================
// T025: Associativity Property Test for Wide Patterns
// ============================================================================

proptest! {
    #[test]
    fn test_associativity_wide_patterns(
        a in wide_pattern(),
        b in wide_pattern(),
        c in wide_pattern(),
    ) {
        let left = a.clone().combine(b.clone()).combine(c.clone());
        let right = a.combine(b.combine(c));

        prop_assert_eq!(left, right, "Associativity failed for wide patterns");
    }
}

// ============================================================================
// T026: Element Preservation Property Test
// ============================================================================

proptest! {
    #[test]
    fn test_element_preservation(
        p1 in shallow_pattern(),
        p2 in shallow_pattern(),
    ) {
        let len1 = p1.length();
        let len2 = p2.length();
        let result = p1.combine(p2);

        prop_assert_eq!(
            result.length(),
            len1 + len2,
            "Combined pattern should have all elements from both inputs"
        );
    }
}

// ============================================================================
// T027: Element Order Property Test
// ============================================================================

proptest! {
    #[test]
    fn test_element_order(
        p1 in shallow_pattern(),
        p2 in shallow_pattern(),
    ) {
        let p1_values: Vec<String> = p1.elements().iter()
            .map(|p| p.value().clone())
            .collect();
        let p2_values: Vec<String> = p2.elements().iter()
            .map(|p| p.value().clone())
            .collect();

        let result = p1.combine(p2);
        let result_values: Vec<String> = result.elements().iter()
            .map(|p| p.value().clone())
            .collect();

        // Verify left pattern elements come first
        for (i, expected) in p1_values.iter().enumerate() {
            prop_assert_eq!(
                &result_values[i],
                expected,
                "Left pattern elements should come first in order"
            );
        }

        // Verify right pattern elements come after
        let offset = p1_values.len();
        for (i, expected) in p2_values.iter().enumerate() {
            prop_assert_eq!(
                &result_values[offset + i],
                expected,
                "Right pattern elements should come after left elements in order"
            );
        }
    }
}

// ============================================================================
// T028: Value Combination Delegation Property Test
// ============================================================================

proptest! {
    #[test]
    fn test_value_combination_delegation(
        v1 in any::<String>(),
        v2 in any::<String>(),
    ) {
        let p1 = Pattern::point(v1.clone());
        let p2 = Pattern::point(v2.clone());

        let result = p1.combine(p2);
        let expected_value = v1.combine(v2);

        prop_assert_eq!(
            result.value(),
            &expected_value,
            "Pattern combination should delegate value combination to V::combine"
        );
    }
}

// ============================================================================
// Additional Property Tests
// ============================================================================

proptest! {
    /// Test that combining with empty patterns preserves structure
    #[test]
    fn test_combine_with_empty_elements(
        p in shallow_pattern(),
        v in any::<String>(),
    ) {
        let empty = Pattern::pattern(v.clone(), vec![]);
        let result1 = p.clone().combine(empty.clone());
        let result2 = empty.combine(p.clone());

        // Both should have the same number of elements as p
        prop_assert_eq!(result1.length(), p.length());
        prop_assert_eq!(result2.length(), p.length());
    }

    /// Test that self-combination doubles element count
    #[test]
    fn test_self_combination(p in shallow_pattern()) {
        let len = p.length();
        let result = p.clone().combine(p.clone());

        prop_assert_eq!(
            result.length(),
            len * 2,
            "Self-combination should double element count"
        );
    }

    /// Test associativity with mixed structure types
    #[test]
    fn test_associativity_mixed_structures(
        a in atomic_pattern(),
        b in shallow_pattern(),
        c in wide_pattern(),
    ) {
        let left = a.clone().combine(b.clone()).combine(c.clone());
        let right = a.combine(b.combine(c));

        prop_assert_eq!(left, right, "Associativity should hold for mixed structures");
    }
}
