//! Property-based tests for Pattern Ord implementation
//!
//! This module verifies that the Ord implementation satisfies all mathematical
//! properties and laws required by the Ord trait, using property-based testing
//! with proptest.

use pattern_core::Pattern;
use proptest::prelude::*;
use std::cmp::Ordering;

// ============================================================================
// Test Data Generators
// ============================================================================

/// Generate arbitrary patterns for property testing
fn pattern_strategy() -> impl Strategy<Value = Pattern<i32>> {
    let leaf = any::<i32>().prop_map(Pattern::point);
    leaf.prop_recursive(
        4,  // max depth
        20, // max total nodes
        10, // items per collection
        |inner| {
            (any::<i32>(), prop::collection::vec(inner, 0..10))
                .prop_map(|(v, elements)| Pattern::pattern(v, elements))
        },
    )
}

// ============================================================================
// T017: Property test - Reflexivity (x.cmp(&x) == Equal)
// ============================================================================

proptest! {
    #[test]
    fn prop_reflexivity(pattern in pattern_strategy()) {
        // Every pattern should be equal to itself
        prop_assert_eq!(pattern.cmp(&pattern), Ordering::Equal);

        // Clone should also be equal
        let clone = pattern.clone();
        prop_assert_eq!(pattern.cmp(&clone), Ordering::Equal);
        prop_assert_eq!(clone.cmp(&pattern), Ordering::Equal);
    }
}

// ============================================================================
// T018: Property test - Antisymmetry (if x < y then y > x)
// ============================================================================

proptest! {
    #[test]
    fn prop_antisymmetry(p1 in pattern_strategy(), p2 in pattern_strategy()) {
        let ordering_12 = p1.cmp(&p2);
        let ordering_21 = p2.cmp(&p1);

        // Verify antisymmetry: cmp(a,b) = reverse(cmp(b,a))
        match ordering_12 {
            Ordering::Less => prop_assert_eq!(ordering_21, Ordering::Greater),
            Ordering::Equal => prop_assert_eq!(ordering_21, Ordering::Equal),
            Ordering::Greater => prop_assert_eq!(ordering_21, Ordering::Less),
        }
    }
}

// ============================================================================
// T019: Property test - Transitivity (if x < y and y < z then x < z)
// ============================================================================

proptest! {
    #[test]
    fn prop_transitivity(p1 in pattern_strategy(), p2 in pattern_strategy(), p3 in pattern_strategy()) {
        use Ordering::*;

        let ord_12 = p1.cmp(&p2);
        let ord_23 = p2.cmp(&p3);
        let ord_13 = p1.cmp(&p3);

        // Verify transitivity for all combinations
        match (ord_12, ord_23) {
            (Less, Less) => prop_assert_eq!(ord_13, Less, "p1 < p2 < p3 implies p1 < p3"),
            (Less, Equal) => prop_assert_eq!(ord_13, Less, "p1 < p2 == p3 implies p1 < p3"),
            (Equal, Less) => prop_assert_eq!(ord_13, Less, "p1 == p2 < p3 implies p1 < p3"),
            (Equal, Equal) => prop_assert_eq!(ord_13, Equal, "p1 == p2 == p3 implies p1 == p3"),
            (Equal, Greater) => prop_assert_eq!(ord_13, Greater, "p1 == p2 > p3 implies p1 > p3"),
            (Greater, Equal) => prop_assert_eq!(ord_13, Greater, "p1 > p2 == p3 implies p1 > p3"),
            (Greater, Greater) => prop_assert_eq!(ord_13, Greater, "p1 > p2 > p3 implies p1 > p3"),
            // Mixed cases don't imply specific ordering
            _ => {}
        }
    }
}

// ============================================================================
// T020: Property test - Totality (exactly one of x < y, x == y, x > y)
// ============================================================================

proptest! {
    #[test]
    fn prop_totality(p1 in pattern_strategy(), p2 in pattern_strategy()) {
        let ordering = p1.cmp(&p2);

        // Exactly one of these should be true
        let is_less = ordering == Ordering::Less;
        let is_equal = ordering == Ordering::Equal;
        let is_greater = ordering == Ordering::Greater;

        // XOR check: exactly one should be true
        let count = [is_less, is_equal, is_greater].iter().filter(|&&x| x).count();
        prop_assert_eq!(count, 1, "Exactly one ordering relationship must hold");

        // Verify using comparison operators
        match ordering {
            Ordering::Less => {
                prop_assert!(p1 < p2);
                prop_assert!(!(p1 == p2));
                prop_assert!(!(p1 > p2));
            }
            Ordering::Equal => {
                prop_assert!(!(p1 < p2));
                prop_assert!(p1 == p2);
                prop_assert!(!(p1 > p2));
            }
            Ordering::Greater => {
                prop_assert!(!(p1 < p2));
                prop_assert!(!(p1 == p2));
                prop_assert!(p1 > p2);
            }
        }
    }
}

// ============================================================================
// T021: Property test - Consistency with Eq (x == y implies x.cmp(&y) == Equal)
// ============================================================================

proptest! {
    #[test]
    fn prop_consistency_with_eq(p1 in pattern_strategy(), p2 in pattern_strategy()) {
        let eq_result = p1 == p2;
        let cmp_result = p1.cmp(&p2);

        // If patterns are equal, cmp must return Equal
        if eq_result {
            prop_assert_eq!(cmp_result, Ordering::Equal, "== implies cmp returns Equal");
        }

        // If cmp returns Equal, patterns must be equal
        if cmp_result == Ordering::Equal {
            prop_assert!(eq_result, "cmp Equal implies ==");
        }

        // Equivalence: (p1 == p2) <==> (p1.cmp(&p2) == Equal)
        prop_assert_eq!(eq_result, cmp_result == Ordering::Equal);
    }
}

// ============================================================================
// T022: Property test - Value precedence (if values differ, elements not compared)
// ============================================================================

proptest! {
    #[test]
    fn prop_value_precedence(
        v1 in any::<i32>().prop_filter("values must differ", |v| *v != 0),
        v2 in any::<i32>().prop_filter("values must differ", |v| *v != 0),
        elements1 in prop::collection::vec(pattern_strategy(), 0..5),
        elements2 in prop::collection::vec(pattern_strategy(), 0..5),
    ) {
        // Only test when values actually differ
        if v1 == v2 {
            return Ok(());
        }

        let p1 = Pattern::pattern(v1, elements1.clone());
        let p2 = Pattern::pattern(v2, elements2.clone());

        // Ordering should be determined by values alone
        let expected = v1.cmp(&v2);
        prop_assert_eq!(p1.cmp(&p2), expected, "Ordering should match value ordering");

        // Changing elements should not affect ordering when values differ
        let p1_alt = Pattern::pattern(v1, vec![]);
        let p2_alt = Pattern::pattern(v2, vec![]);

        prop_assert_eq!(p1_alt.cmp(&p2_alt), expected, "Empty elements give same ordering");
        prop_assert_eq!(p1.cmp(&p2_alt), expected, "Different elements don't affect ordering");
        prop_assert_eq!(p1_alt.cmp(&p2), expected, "Different elements don't affect ordering");
    }
}

// ============================================================================
// T023: Property test - Lexicographic element ordering (element-by-element comparison)
// ============================================================================

proptest! {
    #[test]
    fn prop_lexicographic_elements(
        v in any::<i32>(),
        elements1 in prop::collection::vec(pattern_strategy(), 1..10),
        elements2 in prop::collection::vec(pattern_strategy(), 1..10),
    ) {
        let p1 = Pattern::pattern(v, elements1.clone());
        let p2 = Pattern::pattern(v, elements2.clone());

        // When values are equal, element ordering should match Vec ordering
        let expected = elements1.cmp(&elements2);
        prop_assert_eq!(p1.cmp(&p2), expected, "Element comparison should be lexicographic");
    }

    #[test]
    fn prop_lexicographic_prefix(
        v in any::<i32>(),
        prefix in prop::collection::vec(pattern_strategy(), 1..5),
        extra in pattern_strategy(),
    ) {
        // Pattern with prefix is less than pattern with prefix + extra
        let mut extended = prefix.clone();
        extended.push(extra);

        let p_short = Pattern::pattern(v, prefix);
        let p_long = Pattern::pattern(v, extended);

        // Shorter should be less (prefix property)
        prop_assert!(p_short < p_long, "Prefix should be less than extended");
    }

    #[test]
    fn prop_lexicographic_first_difference(
        v in any::<i32>(),
        prefix in prop::collection::vec(pattern_strategy(), 0..3),
        elem1 in pattern_strategy(),
        elem2 in pattern_strategy(),
        suffix in prop::collection::vec(pattern_strategy(), 0..3),
    ) {
        // Skip if elements are equal (no difference to test)
        if elem1 == elem2 {
            return Ok(());
        }

        // Build patterns: value + prefix + elem1/elem2 + suffix
        let mut elements1 = prefix.clone();
        elements1.push(elem1.clone());
        elements1.extend(suffix.clone());

        let mut elements2 = prefix.clone();
        elements2.push(elem2.clone());
        elements2.extend(suffix.clone());

        let p1 = Pattern::pattern(v, elements1);
        let p2 = Pattern::pattern(v, elements2);

        // Ordering should match the first differing element
        let expected = elem1.cmp(&elem2);
        prop_assert_eq!(p1.cmp(&p2), expected,
            "Ordering should be determined by first differing element");
    }
}

// ============================================================================
// Additional Property: Ord enables sorting
// ============================================================================

proptest! {
    #[test]
    fn prop_sorting_works(mut patterns in prop::collection::vec(pattern_strategy(), 0..50)) {
        // Sort the patterns
        patterns.sort();

        // Verify sorted order
        for i in 0..patterns.len().saturating_sub(1) {
            prop_assert!(patterns[i] <= patterns[i + 1],
                "Patterns should be in sorted order at index {}", i);
        }
    }
}

// ============================================================================
// Additional Property: Comparison operators consistent with cmp
// ============================================================================

proptest! {
    #[test]
    fn prop_operators_match_cmp(p1 in pattern_strategy(), p2 in pattern_strategy()) {
        let ordering = p1.cmp(&p2);

        // Verify all comparison operators match cmp result
        match ordering {
            Ordering::Less => {
                prop_assert!(p1 < p2, "< should match Less");
                prop_assert!(p1 <= p2, "<= should match Less");
                prop_assert!(!(p1 > p2), "> should not match Less");
                prop_assert!(!(p1 >= p2), ">= should not match Less");
                prop_assert!(p1 != p2, "!= should match Less");
            }
            Ordering::Equal => {
                prop_assert!(!(p1 < p2), "< should not match Equal");
                prop_assert!(p1 <= p2, "<= should match Equal");
                prop_assert!(!(p1 > p2), "> should not match Equal");
                prop_assert!(p1 >= p2, ">= should match Equal");
                prop_assert!(p1 == p2, "== should match Equal");
            }
            Ordering::Greater => {
                prop_assert!(!(p1 < p2), "< should not match Greater");
                prop_assert!(!(p1 <= p2), "<= should not match Greater");
                prop_assert!(p1 > p2, "> should match Greater");
                prop_assert!(p1 >= p2, ">= should match Greater");
                prop_assert!(p1 != p2, "!= should match Greater");
            }
        }
    }
}
