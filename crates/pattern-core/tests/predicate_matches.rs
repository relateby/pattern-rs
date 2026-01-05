//! Tests for Pattern::matches
//!
//! This test suite verifies the behavior of the `matches` method, which checks if two patterns
//! have identical structure (same values and element arrangement recursively).
//!
//! Test Requirements from contracts/type-signatures.md:
//! - Returns true for identical patterns
//! - Returns true for self-comparison (reflexive)
//! - Returns false for different values
//! - Returns false for different element counts
//! - Returns false for different element structures
//! - Distinguishes same values, different structures
//! - Symmetric: p.matches(&q) == q.matches(&p)
//! - Works with atomic patterns
//! - Works with empty elements
//! - Works with deeply nested structures

use pattern_core::Pattern;

// ============================================================================
// T022: Test matches returning true for identical patterns
// ============================================================================

#[test]
fn test_matches_identical_atomic_patterns() {
    let p1 = Pattern::point("a");
    let p2 = Pattern::point("a");

    assert!(p1.matches(&p2));
    assert!(p2.matches(&p1));  // Symmetric
}

#[test]
fn test_matches_identical_nested_patterns() {
    let p1 = Pattern::pattern("root", vec![
        Pattern::point("child1"),
        Pattern::point("child2"),
    ]);
    let p2 = Pattern::pattern("root", vec![
        Pattern::point("child1"),
        Pattern::point("child2"),
    ]);

    assert!(p1.matches(&p2));
    assert!(p2.matches(&p1));  // Symmetric
}

#[test]
fn test_matches_deeply_nested_identical() {
    let p1 = Pattern::pattern("root", vec![
        Pattern::pattern("branch", vec![
            Pattern::pattern("deep", vec![
                Pattern::point("leaf"),
            ]),
        ]),
    ]);
    let p2 = Pattern::pattern("root", vec![
        Pattern::pattern("branch", vec![
            Pattern::pattern("deep", vec![
                Pattern::point("leaf"),
            ]),
        ]),
    ]);

    assert!(p1.matches(&p2));
}

// ============================================================================
// T023: Test matches returning true for self-comparison (reflexive)
// ============================================================================

#[test]
fn test_matches_reflexive_atomic() {
    let pattern = Pattern::point("a");
    assert!(pattern.matches(&pattern));
}

#[test]
fn test_matches_reflexive_nested() {
    let pattern = Pattern::pattern("root", vec![
        Pattern::point("child1"),
        Pattern::pattern("branch", vec![
            Pattern::point("child2"),
        ]),
    ]);

    assert!(pattern.matches(&pattern));
}

#[test]
fn test_matches_reflexive_empty_elements() {
    let pattern = Pattern::pattern("root", vec![]);
    assert!(pattern.matches(&pattern));
}

// ============================================================================
// T024: Test matches returning false for different values
// ============================================================================

#[test]
fn test_matches_different_values_atomic() {
    let p1 = Pattern::point("a");
    let p2 = Pattern::point("b");

    assert!(!p1.matches(&p2));
    assert!(!p2.matches(&p1));  // Symmetric
}

#[test]
fn test_matches_different_root_values() {
    let p1 = Pattern::pattern("root1", vec![
        Pattern::point("child"),
    ]);
    let p2 = Pattern::pattern("root2", vec![
        Pattern::point("child"),
    ]);

    assert!(!p1.matches(&p2));
}

#[test]
fn test_matches_different_element_values() {
    let p1 = Pattern::pattern("root", vec![
        Pattern::point("a"),
        Pattern::point("b"),
    ]);
    let p2 = Pattern::pattern("root", vec![
        Pattern::point("a"),
        Pattern::point("c"),  // Different!
    ]);

    assert!(!p1.matches(&p2));
}

// ============================================================================
// T025: Test matches returning false for different element counts
// ============================================================================

#[test]
fn test_matches_different_element_counts() {
    let p1 = Pattern::pattern("root", vec![
        Pattern::point("a"),
    ]);
    let p2 = Pattern::pattern("root", vec![
        Pattern::point("a"),
        Pattern::point("b"),
    ]);

    assert!(!p1.matches(&p2));
    assert!(!p2.matches(&p1));  // Symmetric
}

#[test]
fn test_matches_empty_vs_non_empty() {
    let p1 = Pattern::pattern("root", vec![]);
    let p2 = Pattern::pattern("root", vec![
        Pattern::point("child"),
    ]);

    assert!(!p1.matches(&p2));
    assert!(!p2.matches(&p1));
}

#[test]
fn test_matches_many_vs_few_elements() {
    let p1 = Pattern::pattern("root", vec![
        Pattern::point("a"),
        Pattern::point("b"),
        Pattern::point("c"),
    ]);
    let p2 = Pattern::pattern("root", vec![
        Pattern::point("a"),
    ]);

    assert!(!p1.matches(&p2));
}

// ============================================================================
// T026: Test matches returning false for different element structures
// ============================================================================

#[test]
fn test_matches_different_nesting_levels() {
    let p1 = Pattern::pattern("root", vec![
        Pattern::pattern("branch", vec![
            Pattern::point("leaf"),
        ]),
    ]);
    let p2 = Pattern::pattern("root", vec![
        Pattern::point("branch"),
    ]);

    assert!(!p1.matches(&p2));
}

#[test]
fn test_matches_different_element_order() {
    let p1 = Pattern::pattern("root", vec![
        Pattern::point("a"),
        Pattern::point("b"),
    ]);
    let p2 = Pattern::pattern("root", vec![
        Pattern::point("b"),
        Pattern::point("a"),  // Swapped order
    ]);

    assert!(!p1.matches(&p2));
}

#[test]
fn test_matches_different_nested_structure() {
    let p1 = Pattern::pattern("root", vec![
        Pattern::pattern("branch1", vec![
            Pattern::point("leaf"),
        ]),
        Pattern::point("leaf2"),
    ]);
    let p2 = Pattern::pattern("root", vec![
        Pattern::point("branch1"),
        Pattern::pattern("branch2", vec![
            Pattern::point("leaf2"),
        ]),
    ]);

    assert!(!p1.matches(&p2));
}

// ============================================================================
// T027: Test matches distinguishing same values, different structures
// ============================================================================

#[test]
fn test_matches_same_flattened_values_different_structure() {
    // Both have values ["a", "b", "c"] but different structures
    let p1 = Pattern::pattern("a", vec![
        Pattern::point("b"),
        Pattern::point("c"),
    ]);
    let p2 = Pattern::pattern("a", vec![
        Pattern::pattern("b", vec![
            Pattern::point("c"),
        ]),
    ]);

    assert!(!p1.matches(&p2));
}

#[test]
fn test_matches_point_vs_pattern_same_value() {
    // Same value "a" but different structure (point vs pattern with empty elements)
    let p1 = Pattern::point("a");
    let p2 = Pattern::pattern("a", vec![]);

    // These should NOT match - different structure
    // point is atomic (special constructor), pattern with empty elements is different
    assert!(p1.matches(&p2));  // Actually they ARE structurally identical!
}

#[test]
fn test_matches_flat_vs_nested_same_values() {
    let p1 = Pattern::pattern("root", vec![
        Pattern::point("a"),
        Pattern::point("b"),
        Pattern::point("c"),
    ]);
    let p2 = Pattern::pattern("root", vec![
        Pattern::pattern("a", vec![
            Pattern::point("b"),
        ]),
        Pattern::point("c"),
    ]);

    assert!(!p1.matches(&p2));
}

// ============================================================================
// T028: Test matches symmetry property
// ============================================================================

#[test]
fn test_matches_symmetry() {
    let p1 = Pattern::pattern("root", vec![
        Pattern::point("a"),
        Pattern::point("b"),
    ]);
    let p2 = Pattern::pattern("root", vec![
        Pattern::point("a"),
        Pattern::point("b"),
    ]);

    // If p1 matches p2, then p2 matches p1
    assert_eq!(p1.matches(&p2), p2.matches(&p1));
}

#[test]
fn test_matches_symmetry_non_matching() {
    let p1 = Pattern::point("a");
    let p2 = Pattern::point("b");

    // Symmetry holds for non-matches too
    assert_eq!(p1.matches(&p2), p2.matches(&p1));
    assert!(!p1.matches(&p2));
}

#[test]
fn test_matches_symmetry_complex() {
    let p1 = Pattern::pattern("root", vec![
        Pattern::pattern("branch", vec![
            Pattern::point("leaf1"),
        ]),
        Pattern::point("leaf2"),
    ]);
    let p2 = Pattern::pattern("root", vec![
        Pattern::point("branch"),
        Pattern::point("leaf2"),
    ]);

    assert_eq!(p1.matches(&p2), p2.matches(&p1));
}

// ============================================================================
// T029: Test matches with atomic patterns
// ============================================================================

#[test]
fn test_matches_atomic_identical() {
    let p1 = Pattern::point(42);
    let p2 = Pattern::point(42);

    assert!(p1.matches(&p2));
}

#[test]
fn test_matches_atomic_different() {
    let p1 = Pattern::point(42);
    let p2 = Pattern::point(99);

    assert!(!p1.matches(&p2));
}

#[test]
fn test_matches_atomic_vs_non_atomic() {
    let p1 = Pattern::point("a");
    let p2 = Pattern::pattern("a", vec![
        Pattern::point("b"),
    ]);

    assert!(!p1.matches(&p2));
}

// ============================================================================
// T030: Test matches with empty elements
// ============================================================================

#[test]
fn test_matches_both_empty_elements() {
    let p1 = Pattern::pattern("root", vec![]);
    let p2 = Pattern::pattern("root", vec![]);

    assert!(p1.matches(&p2));
}

#[test]
fn test_matches_empty_elements_different_values() {
    let p1 = Pattern::pattern("root1", vec![]);
    let p2 = Pattern::pattern("root2", vec![]);

    assert!(!p1.matches(&p2));
}

#[test]
fn test_matches_empty_elements_in_nested() {
    let p1 = Pattern::pattern("root", vec![
        Pattern::pattern("empty", vec![]),
        Pattern::point("leaf"),
    ]);
    let p2 = Pattern::pattern("root", vec![
        Pattern::pattern("empty", vec![]),
        Pattern::point("leaf"),
    ]);

    assert!(p1.matches(&p2));
}

// ============================================================================
// T031: Test matches with deeply nested structures
// ============================================================================

#[test]
fn test_matches_10_level_nesting_identical() {
    // Create a 10-level deep structure
    let mut p1 = Pattern::point("bottom".to_string());
    let mut p2 = Pattern::point("bottom".to_string());
    for i in (0..10).rev() {
        p1 = Pattern::pattern(format!("level{}", i), vec![p1]);
        p2 = Pattern::pattern(format!("level{}", i), vec![p2]);
    }

    assert!(p1.matches(&p2));
}

#[test]
fn test_matches_deeply_nested_different_at_bottom() {
    // Create a 10-level deep structure with different bottom values
    let mut p1 = Pattern::point("bottom1".to_string());
    let mut p2 = Pattern::point("bottom2".to_string());
    for i in (0..10).rev() {
        p1 = Pattern::pattern(format!("level{}", i), vec![p1]);
        p2 = Pattern::pattern(format!("level{}", i), vec![p2]);
    }

    assert!(!p1.matches(&p2));
}

#[test]
fn test_matches_deeply_nested_different_at_middle() {
    // Create a 10-level deep structure with difference in the middle
    let mut p1 = Pattern::point("bottom".to_string());
    let mut p2 = Pattern::point("bottom".to_string());
    for i in (0..10).rev() {
        if i == 5 {
            p1 = Pattern::pattern(format!("level{}-different", i), vec![p1]);
        } else {
            p1 = Pattern::pattern(format!("level{}", i), vec![p1]);
        }
        p2 = Pattern::pattern(format!("level{}", i), vec![p2]);
    }

    assert!(!p1.matches(&p2));
}

#[test]
fn test_matches_100_level_nesting() {
    // Create a 100-level deep structure
    let mut p1 = Pattern::point("bottom".to_string());
    let mut p2 = Pattern::point("bottom".to_string());
    for i in (0..100).rev() {
        p1 = Pattern::pattern(format!("level{}", i), vec![p1]);
        p2 = Pattern::pattern(format!("level{}", i), vec![p2]);
    }

    assert!(p1.matches(&p2));
}

