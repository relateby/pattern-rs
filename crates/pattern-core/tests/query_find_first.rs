//! Tests for Pattern::find_first
//!
//! This test suite verifies the behavior of the `find_first` method, which finds the first
//! subpattern matching a predicate using depth-first pre-order traversal.
//!
//! Test Requirements from contracts/type-signatures.md:
//! - Returns Some for root pattern when root matches
//! - Returns Some for element pattern when element matches
//! - Returns Some for deeply nested pattern when nested pattern matches
//! - Returns None when no patterns match
//! - Returns first match in pre-order traversal when multiple match
//! - Works with atomic patterns (no elements)
//! - Works with empty elements
//! - Predicate receives correct pattern reference
//! - Predicate can examine value and structure
//! - Handles deeply nested structures (100+ levels)

use pattern_core::Pattern;

// ============================================================================
// T005: Test find_first returning Some when root matches predicate
// ============================================================================

#[test]
fn test_find_first_root_matches() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::point("child1"), Pattern::point("child2")],
    );

    // Root matches: should return the root pattern itself
    let result = pattern.find_first(|p| p.value == "root");
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "root");
}

#[test]
fn test_find_first_root_matches_by_structure() {
    let pattern = Pattern::pattern("root", vec![Pattern::point("child")]);

    // Root matches by element count
    let result = pattern.find_first(|p| p.length() == 1);
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "root");
}

// ============================================================================
// T006: Test find_first returning Some when element matches predicate
// ============================================================================

#[test]
fn test_find_first_element_matches() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::point("target"), Pattern::point("other")],
    );

    // First element matches
    let result = pattern.find_first(|p| p.value == "target");
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "target");
}

#[test]
fn test_find_first_second_element_matches() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::point("first"), Pattern::point("target")],
    );

    // Second element matches (root doesn't match)
    let result = pattern.find_first(|p| p.value == "target");
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "target");
}

// ============================================================================
// T007: Test find_first returning Some for deeply nested matching pattern
// ============================================================================

#[test]
fn test_find_first_deeply_nested() {
    let pattern = Pattern::pattern(
        "root",
        vec![
            Pattern::pattern(
                "branch1",
                vec![Pattern::pattern("branch2", vec![Pattern::point("target")])],
            ),
            Pattern::point("sibling"),
        ],
    );

    // Find deeply nested target
    let result = pattern.find_first(|p| p.value == "target");
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "target");
}

#[test]
fn test_find_first_very_deep_nesting() {
    // Create a deeply nested structure (10 levels) using String type
    let mut pattern = Pattern::point("target".to_string());
    for i in (0..10).rev() {
        pattern = Pattern::pattern(format!("level{}", i), vec![pattern]);
    }

    // Find the deeply nested target
    let result = pattern.find_first(|p| p.value == "target");
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "target");
}

// ============================================================================
// T008: Test find_first returning None when no patterns match
// ============================================================================

#[test]
fn test_find_first_no_matches() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::point("child1"), Pattern::point("child2")],
    );

    // No pattern has value "nonexistent"
    let result = pattern.find_first(|p| p.value == "nonexistent");
    assert!(result.is_none());
}

#[test]
fn test_find_first_no_matches_structural_predicate() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::point("child1"), Pattern::point("child2")],
    );

    // No pattern has more than 2 elements
    let result = pattern.find_first(|p| p.length() > 2);
    assert!(result.is_none());
}

// ============================================================================
// T009: Test find_first returning first match in pre-order when multiple match
// ============================================================================

#[test]
fn test_find_first_pre_order_traversal() {
    let pattern = Pattern::pattern(
        "root",
        vec![
            Pattern::pattern("branch", vec![Pattern::point("match2")]),
            Pattern::point("match1"),
        ],
    );

    // Multiple patterns are atomic (no elements)
    // Pre-order: root, branch, match2, match1
    // First atomic pattern should be match2, not match1
    let result = pattern.find_first(|p| p.is_atomic());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "match2");
}

#[test]
fn test_find_first_multiple_value_matches() {
    let pattern = Pattern::pattern(
        "a",
        vec![
            Pattern::pattern("a", vec![Pattern::point("a")]),
            Pattern::point("a"),
        ],
    );

    // All patterns have value "a", should return root (first in pre-order)
    let result = pattern.find_first(|p| p.value == "a");
    assert!(result.is_some());
    // Verify it's the root by checking element count
    assert_eq!(result.unwrap().length(), 2);
}

#[test]
fn test_find_first_left_before_right() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::point("left"), Pattern::point("right")],
    );

    // Both children are atomic, should return left (first in pre-order)
    let result = pattern.find_first(|p| p.is_atomic());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "left");
}

// ============================================================================
// T010: Test find_first with atomic patterns
// ============================================================================

#[test]
fn test_find_first_atomic_pattern_matches_itself() {
    let pattern = Pattern::point("atomic");

    // Atomic pattern should find itself
    let result = pattern.find_first(|p| p.value == "atomic");
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "atomic");
}

#[test]
fn test_find_first_atomic_pattern_no_match() {
    let pattern = Pattern::point("atomic");

    // Atomic pattern with no match
    let result = pattern.find_first(|p| p.value == "other");
    assert!(result.is_none());
}

#[test]
fn test_find_first_atomic_pattern_structural() {
    let pattern = Pattern::point("atomic");

    // Find by structural property (is_atomic)
    let result = pattern.find_first(|p| p.is_atomic());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "atomic");
}

// ============================================================================
// T011: Test find_first with empty elements
// ============================================================================

#[test]
fn test_find_first_empty_elements() {
    let pattern = Pattern::pattern("root", vec![]);

    // Pattern with empty elements should find itself if predicate matches
    let result = pattern.find_first(|p| p.value == "root");
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "root");
}

#[test]
fn test_find_first_empty_elements_is_atomic() {
    let pattern = Pattern::pattern("root", vec![]);

    // Pattern with empty elements IS atomic (is_atomic checks if elements.is_empty())
    // So it should find itself
    let result = pattern.find_first(|p| p.is_atomic());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "root");
}

#[test]
fn test_find_first_empty_elements_structural() {
    let pattern = Pattern::pattern("root", vec![]);

    // Find by structural property (length == 0)
    let result = pattern.find_first(|p| p.length() == 0);
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "root");
}

// ============================================================================
// T012: Test find_first with deep nesting (100+ levels)
// ============================================================================

#[test]
fn test_find_first_100_level_nesting() {
    // Create a 100-level deep structure using String type
    let mut pattern = Pattern::point("bottom".to_string());
    for i in (0..100).rev() {
        pattern = Pattern::pattern(format!("level{}", i), vec![pattern]);
    }

    // Should handle 100 levels without stack overflow
    let result = pattern.find_first(|p| p.value == "bottom");
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "bottom");
}

#[test]
fn test_find_first_120_level_nesting() {
    // Create a 120-level deep structure (exceeds 100 to test limits) using String type
    let mut pattern = Pattern::point("bottom".to_string());
    for i in (0..120).rev() {
        pattern = Pattern::pattern(format!("level{}", i), vec![pattern]);
    }

    // Should handle 120 levels without stack overflow
    let result = pattern.find_first(|p| p.value == "bottom");
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "bottom");
}

// ============================================================================
// T013: Test predicate examining value and structure
// ============================================================================

#[test]
fn test_find_first_combined_value_and_structure() {
    let pattern = Pattern::pattern(
        "root",
        vec![
            Pattern::pattern(
                "branch",
                vec![Pattern::point("child1"), Pattern::point("child2")],
            ),
            Pattern::point("leaf"),
        ],
    );

    // Find pattern with value "branch" AND 2 elements
    let result = pattern.find_first(|p| p.value == "branch" && p.length() == 2);
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "branch");
}

#[test]
fn test_find_first_structural_predicate_depth() {
    let pattern = Pattern::pattern(
        "root",
        vec![
            Pattern::pattern(
                "branch",
                vec![Pattern::pattern("deep", vec![Pattern::point("deepest")])],
            ),
            Pattern::point("shallow"),
        ],
    );

    // Find first pattern with depth >= 2
    let result = pattern.find_first(|p| p.depth() >= 2);
    assert!(result.is_some());
    // Root has depth 3, so it should match first
    assert_eq!(result.unwrap().value, "root");
}

#[test]
fn test_find_first_structural_predicate_size() {
    let pattern = Pattern::pattern(
        "root",
        vec![
            Pattern::pattern(
                "branch",
                vec![
                    Pattern::point("child1"),
                    Pattern::point("child2"),
                    Pattern::point("child3"),
                ],
            ),
            Pattern::point("leaf"),
        ],
    );

    // Find first pattern with size >= 3
    let result = pattern.find_first(|p| p.size() >= 3);
    assert!(result.is_some());
    // Root has size 6, so it should match first
    assert_eq!(result.unwrap().value, "root");
}

// ============================================================================
// T014: Test find_first integration with other Pattern methods
// ============================================================================

#[test]
fn test_find_first_with_any_value() {
    let pattern = Pattern::pattern(
        1,
        vec![
            Pattern::pattern(2, vec![Pattern::point(5), Pattern::point(10)]),
            Pattern::point(3),
        ],
    );

    // Find first pattern where any value is greater than 8
    let result = pattern.find_first(|p| p.any_value(|v| *v > 8));
    assert!(result.is_some());
    // Root has 10, so it matches
    assert_eq!(result.unwrap().value, 1);
}

#[test]
fn test_find_first_with_all_values() {
    let pattern = Pattern::pattern(
        1,
        vec![
            Pattern::pattern(2, vec![Pattern::point(5), Pattern::point(10)]),
            Pattern::point(-3),
        ],
    );

    // Find first pattern where all values are positive
    let result = pattern.find_first(|p| p.all_values(|v| *v > 0));
    assert!(result.is_some());
    // Branch with value 2 has all positive values
    assert_eq!(result.unwrap().value, 2);
}

#[test]
fn test_find_first_with_filter_consistency() {
    let pattern = Pattern::pattern(
        "root",
        vec![
            Pattern::point("a"),
            Pattern::pattern("branch", vec![Pattern::point("b")]),
            Pattern::point("c"),
        ],
    );

    // find_first and filter should be consistent
    let predicate = |p: &Pattern<&str>| p.is_atomic();
    let first = pattern.find_first(predicate);
    let all_matches = pattern.filter(predicate);

    assert!(first.is_some());
    assert!(!all_matches.is_empty());
    // first should be the same as all_matches[0]
    assert_eq!(first.unwrap().value, all_matches[0].value);
}

#[test]
fn test_find_first_returns_reference_not_clone() {
    let pattern = Pattern::pattern(
        "root".to_string(),
        vec![Pattern::point("child".to_string())],
    );

    let result = pattern.find_first(|p| p.value == "child");
    assert!(result.is_some());

    // Verify it's a reference by checking pointer equality
    let child_ref = &pattern.elements[0];
    let result_ref = result.unwrap();
    assert!(std::ptr::eq(child_ref, result_ref));
}
