//! Behavioral equivalence tests for Pattern Ord implementation
//!
//! This module verifies that the Rust Ord implementation matches the behavior
//! of the Haskell reference implementation in gram-hs. These tests are ported
//! from the Haskell test suite to ensure cross-language behavioral equivalence.

use pattern_core::Pattern;
use std::cmp::Ordering;

// ============================================================================
// T025: Port gram-hs test - Atomic pattern comparison examples
// ============================================================================

#[test]
fn test_atomic_comparison_examples() {
    // These examples are derived from the Haskell Ord instance documentation
    // at gram-hs/libs/pattern/src/Pattern/Core.hs (lines 329-334)

    // Example: min (point "a") (point "b") == Pattern "a" []
    let p1 = Pattern::point("a");
    let p2 = Pattern::point("b");
    assert_eq!(std::cmp::min(p1.clone(), p2.clone()), p1);
    assert!(p1 < p2);

    // Example: max (point "a") (point "b") == Pattern "b" []
    assert_eq!(std::cmp::max(p1.clone(), p2.clone()), p2);

    // Numeric examples
    let p3 = Pattern::point(1);
    let p4 = Pattern::point(2);
    assert_eq!(std::cmp::min(p3.clone(), p4.clone()), p3);
    assert_eq!(std::cmp::max(p3.clone(), p4.clone()), p4);

    // Equal patterns
    let p5 = Pattern::point(5);
    let p6 = Pattern::point(5);
    assert_eq!(p5.cmp(&p6), Ordering::Equal);
    assert_eq!(std::cmp::min(p5.clone(), p6.clone()), p5);
    assert_eq!(std::cmp::max(p5.clone(), p6.clone()), p5);
}

// ============================================================================
// T026: Port gram-hs test - Nested pattern comparison examples
// ============================================================================

#[test]
fn test_nested_comparison_examples() {
    // Based on Haskell Ord instance implementation:
    // compare (Pattern v1 es1) (Pattern v2 es2) =
    //   case compare v1 v2 of
    //     EQ -> compare es1 es2
    //     other -> other

    // Example 1: Different values, elements don't matter
    let p1 = Pattern::pattern(1, vec![Pattern::point(100)]);
    let p2 = Pattern::pattern(2, vec![Pattern::point(50)]);
    assert!(p1 < p2); // 1 < 2, elements ignored

    // Example 2: Same value, compare elements
    let p3 = Pattern::pattern(5, vec![Pattern::point(1), Pattern::point(2)]);
    let p4 = Pattern::pattern(5, vec![Pattern::point(1), Pattern::point(3)]);
    assert!(p3 < p4); // Values equal, [1,2] < [1,3]

    // Example 3: Same value, same first element, different second
    let p5 = Pattern::pattern(
        10,
        vec![
            Pattern::point(1),
            Pattern::pattern(2, vec![Pattern::point(5)]),
        ],
    );
    let p6 = Pattern::pattern(
        10,
        vec![
            Pattern::point(1),
            Pattern::pattern(2, vec![Pattern::point(6)]),
        ],
    );
    assert!(p5 < p6); // Nested comparison: 5 < 6

    // Example 4: Empty elements vs non-empty
    let p7 = Pattern::pattern(7, vec![]);
    let p8 = Pattern::pattern(7, vec![Pattern::point(1)]);
    assert!(p7 < p8); // [] < [1]

    // Example 5: Different element counts
    let p9 = Pattern::pattern(3, vec![Pattern::point(1)]);
    let p10 = Pattern::pattern(3, vec![Pattern::point(1), Pattern::point(2)]);
    assert!(p9 < p10); // [1] < [1, 2]
}

// ============================================================================
// T027: Port gram-hs test - Deep structural comparison examples
// ============================================================================

#[test]
fn test_deep_structural_comparison() {
    // Test cases that exercise recursive comparison through multiple levels

    // Example 1: Three-level nesting
    let p1 = Pattern::pattern(
        1,
        vec![Pattern::pattern(
            2,
            vec![Pattern::pattern(3, vec![Pattern::point(4)])],
        )],
    );
    let p2 = Pattern::pattern(
        1,
        vec![Pattern::pattern(
            2,
            vec![Pattern::pattern(3, vec![Pattern::point(5)])],
        )],
    );
    assert!(p1 < p2); // Difference at deepest level: 4 < 5

    // Example 2: Difference at middle level
    let p3 = Pattern::pattern(
        1,
        vec![Pattern::pattern(
            2,
            vec![Pattern::pattern(3, vec![Pattern::point(10)])],
        )],
    );
    let p4 = Pattern::pattern(
        1,
        vec![Pattern::pattern(
            2,
            vec![Pattern::pattern(4, vec![Pattern::point(10)])],
        )],
    );
    assert!(p3 < p4); // Difference at middle level: 3 < 4

    // Example 3: Difference at top level
    let p5 = Pattern::pattern(
        1,
        vec![Pattern::pattern(
            2,
            vec![Pattern::pattern(3, vec![Pattern::point(10)])],
        )],
    );
    let p6 = Pattern::pattern(
        2,
        vec![Pattern::pattern(
            2,
            vec![Pattern::pattern(3, vec![Pattern::point(10)])],
        )],
    );
    assert!(p5 < p6); // Difference at root level: 1 < 2

    // Example 4: Multiple branches
    let p7 = Pattern::pattern(
        1,
        vec![
            Pattern::pattern(2, vec![Pattern::point(3)]),
            Pattern::pattern(4, vec![Pattern::point(5)]),
        ],
    );
    let p8 = Pattern::pattern(
        1,
        vec![
            Pattern::pattern(2, vec![Pattern::point(3)]),
            Pattern::pattern(4, vec![Pattern::point(6)]),
        ],
    );
    assert!(p7 < p8); // Second branch differs: 5 < 6

    // Example 5: Identical deep structures
    let p9 = Pattern::pattern(
        1,
        vec![Pattern::pattern(
            2,
            vec![
                Pattern::point(3),
                Pattern::pattern(4, vec![Pattern::point(5)]),
            ],
        )],
    );
    let p10 = p9.clone();
    assert_eq!(p9.cmp(&p10), Ordering::Equal);
}

// ============================================================================
// T028: Port gram-hs test - Min/max examples
// ============================================================================

#[test]
fn test_min_max_examples() {
    // Test cases demonstrating min/max behavior from Haskell reference

    // Example 1: Simple min/max
    let p1 = Pattern::point(10);
    let p2 = Pattern::point(20);
    assert_eq!(std::cmp::min(p1.clone(), p2.clone()), p1);
    assert_eq!(std::cmp::max(p1.clone(), p2.clone()), p2);

    // Example 2: Min/max with nested patterns
    let p3 = Pattern::pattern(5, vec![Pattern::point(100)]);
    let p4 = Pattern::pattern(5, vec![Pattern::point(200)]);
    assert_eq!(std::cmp::min(p3.clone(), p4.clone()), p3);
    assert_eq!(std::cmp::max(p3.clone(), p4.clone()), p4);

    // Example 3: Min/max determined by value
    let p5 = Pattern::pattern(3, vec![Pattern::point(999)]);
    let p6 = Pattern::pattern(4, vec![Pattern::point(1)]);
    assert_eq!(std::cmp::min(p5.clone(), p6.clone()), p5); // 3 < 4
    assert_eq!(std::cmp::max(p5.clone(), p6.clone()), p6);

    // Example 4: Min/max with different element counts
    let p7 = Pattern::pattern(7, vec![]);
    let p8 = Pattern::pattern(7, vec![Pattern::point(1)]);
    assert_eq!(std::cmp::min(p7.clone(), p8.clone()), p7); // [] < [1]
    assert_eq!(std::cmp::max(p7.clone(), p8.clone()), p8);

    // Example 5: Min/max in collections
    let patterns = vec![
        Pattern::point(15),
        Pattern::point(3),
        Pattern::point(42),
        Pattern::point(7),
        Pattern::point(23),
    ];

    let min_pattern = patterns.iter().min().unwrap();
    let max_pattern = patterns.iter().max().unwrap();

    assert_eq!(min_pattern, &Pattern::point(3));
    assert_eq!(max_pattern, &Pattern::point(42));

    // Example 6: Clamp operation
    let lower = Pattern::point(10);
    let upper = Pattern::point(20);

    let too_low = Pattern::point(5);
    let in_range = Pattern::point(15);
    let too_high = Pattern::point(25);

    assert_eq!(too_low.clone().clamp(lower.clone(), upper.clone()), lower);
    assert_eq!(
        in_range.clone().clamp(lower.clone(), upper.clone()),
        in_range
    );
    assert_eq!(too_high.clone().clamp(lower.clone(), upper.clone()), upper);
}

// ============================================================================
// Additional equivalence tests
// ============================================================================

#[test]
fn test_ordering_consistency() {
    // Verify that all comparison methods give consistent results

    let p1 = Pattern::pattern(5, vec![Pattern::point(1), Pattern::point(2)]);
    let p2 = Pattern::pattern(5, vec![Pattern::point(1), Pattern::point(3)]);

    // cmp, partial_cmp, and operators should all agree
    assert_eq!(p1.cmp(&p2), Ordering::Less);
    assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Less));
    assert!(p1 < p2);
    assert!(p1 <= p2);
    assert!(!(p1 > p2));
    assert!(!(p1 >= p2));
    assert!(p1 != p2);
}

#[test]
fn test_lexicographic_behavior() {
    // Verify lexicographic ordering matches Haskell's list comparison

    // Test case 1: First element differs
    let p1 = Pattern::pattern(0, vec![Pattern::point(1), Pattern::point(999)]);
    let p2 = Pattern::pattern(0, vec![Pattern::point(2), Pattern::point(1)]);
    assert!(p1 < p2); // [1, _] < [2, _]

    // Test case 2: First element same, second differs
    let p3 = Pattern::pattern(0, vec![Pattern::point(5), Pattern::point(10)]);
    let p4 = Pattern::pattern(0, vec![Pattern::point(5), Pattern::point(20)]);
    assert!(p3 < p4); // [5, 10] < [5, 20]

    // Test case 3: Prefix comparison
    let p5 = Pattern::pattern(0, vec![Pattern::point(1), Pattern::point(2)]);
    let p6 = Pattern::pattern(
        0,
        vec![Pattern::point(1), Pattern::point(2), Pattern::point(3)],
    );
    assert!(p5 < p6); // [1, 2] < [1, 2, 3]
}

#[test]
fn test_value_precedence_equivalence() {
    // Verify that value comparison takes absolute precedence
    // This matches the Haskell implementation exactly

    // No matter what elements are, value determines ordering
    let low_value_huge_elements = Pattern::pattern(1, vec![Pattern::point(i32::MAX); 100]);

    let high_value_empty = Pattern::pattern(2, vec![]);

    assert!(low_value_huge_elements < high_value_empty);
    assert_eq!(
        low_value_huge_elements.cmp(&high_value_empty),
        Ordering::Less
    );
}
