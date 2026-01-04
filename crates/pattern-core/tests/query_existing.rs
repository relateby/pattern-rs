//! Tests for existing Pattern query operations
//!
//! This module provides comprehensive test coverage for the structural query operations
//! (length, size, depth, values) that were already implemented in previous features.
//! These tests ensure behavioral equivalence with the Haskell reference implementation
//! and prevent regressions.

use pattern_core::Pattern;

// ========== length() Tests ==========

#[test]
fn test_length_atomic_pattern() {
    // T063: length with atomic patterns should return 0
    let pat = Pattern::point(42);
    assert_eq!(pat.length(), 0);
}

#[test]
fn test_length_one_element() {
    // T064: length with pattern having 1 direct element
    let pat = Pattern::pattern(5, vec![Pattern::point(10)]);
    assert_eq!(pat.length(), 1);
}

#[test]
fn test_length_two_elements() {
    // T064: length with pattern having 2 direct elements
    let pat = Pattern::pattern(5, vec![Pattern::point(10), Pattern::point(3)]);
    assert_eq!(pat.length(), 2);
}

#[test]
fn test_length_many_elements() {
    // T064: length with pattern having many direct elements
    let elements: Vec<Pattern<i32>> = (0..10).map(Pattern::point).collect();
    let pat = Pattern::pattern(99, elements);
    assert_eq!(pat.length(), 10);
}

#[test]
fn test_length_only_counts_direct_elements() {
    // T065: length should only count direct elements, not nested descendants
    let pat = Pattern::pattern(
        1,
        vec![
            Pattern::point(2),
            Pattern::pattern(3, vec![Pattern::point(4), Pattern::point(5)]),
        ],
    );
    assert_eq!(pat.length(), 2); // Only direct elements: point(2) and pattern(3)
}

// ========== size() Tests ==========

#[test]
fn test_size_atomic_pattern() {
    // T066: size with atomic patterns should return 1
    let pat = Pattern::point(42);
    assert_eq!(pat.size(), 1);
}

#[test]
fn test_size_flat_pattern() {
    // T067: size with flat patterns (1 + direct element count)
    let pat = Pattern::pattern(5, vec![Pattern::point(10), Pattern::point(3)]);
    assert_eq!(pat.size(), 3); // 1 (root) + 2 (direct elements)
}

#[test]
fn test_size_deeply_nested() {
    // T068: size with deeply nested patterns (correct total count)
    fn create_deep_pattern(depth: usize) -> Pattern<i32> {
        if depth == 0 {
            Pattern::point(0)
        } else {
            Pattern::pattern(depth as i32, vec![create_deep_pattern(depth - 1)])
        }
    }

    let deep_pat = create_deep_pattern(100);
    assert_eq!(deep_pat.size(), 101); // 100 pattern nodes + 1 point node
}

#[test]
fn test_size_varying_branch_depths() {
    // T069: size with patterns having varying branch depths
    let pat = Pattern::pattern(
        1,
        vec![
            Pattern::point(2),
            Pattern::pattern(3, vec![Pattern::point(4)]),
            Pattern::pattern(
                5,
                vec![Pattern::pattern(
                    6,
                    vec![Pattern::point(7), Pattern::point(8)],
                )],
            ),
        ],
    );

    // Count: 1 (root) + 1 (point(2)) + 1 (pattern(3)) + 1 (point(4)) +
    //        1 (pattern(5)) + 1 (pattern(6)) + 1 (point(7)) + 1 (point(8)) = 8
    assert_eq!(pat.size(), 8);
}

// ========== depth() Tests ==========

#[test]
fn test_depth_atomic_pattern() {
    // T070: depth with atomic patterns should return 0
    let pat = Pattern::point(42);
    assert_eq!(pat.depth(), 0);
}

#[test]
fn test_depth_one_level_nesting() {
    // T071: depth with one level of nesting should return 1
    let pat = Pattern::pattern(5, vec![Pattern::point(10)]);
    assert_eq!(pat.depth(), 1);
}

#[test]
fn test_depth_deeply_nested() {
    // T072: depth with deeply nested patterns (correct max depth)
    fn create_deep_pattern(depth: usize) -> Pattern<i32> {
        if depth == 0 {
            Pattern::point(0)
        } else {
            Pattern::pattern(depth as i32, vec![create_deep_pattern(depth - 1)])
        }
    }

    let deep_pat = create_deep_pattern(100);
    assert_eq!(deep_pat.depth(), 100);
}

#[test]
fn test_depth_different_branch_depths() {
    // T073: depth with patterns having branches of different depths (returns maximum)
    let pat = Pattern::pattern(
        1,
        vec![
            Pattern::point(2),                            // depth 0
            Pattern::pattern(3, vec![Pattern::point(4)]), // depth 1
            Pattern::pattern(
                5,
                vec![Pattern::pattern(6, vec![Pattern::point(7)])], // depth 2
            ),
        ],
    );

    assert_eq!(pat.depth(), 3); // Maximum depth of any branch (1 + 2)
}

// ========== values() Tests ==========

#[test]
fn test_values_atomic_pattern() {
    // T074: values with atomic patterns (single-element list)
    let pat = Pattern::point(42);
    let vals = pat.values();
    assert_eq!(vals.len(), 1);
    assert_eq!(*vals[0], 42);
}

#[test]
fn test_values_nested_pattern_pre_order() {
    // T075: values with nested patterns (all values in pre-order)
    let pat = Pattern::pattern(
        1,
        vec![
            Pattern::point(2),
            Pattern::pattern(3, vec![Pattern::point(4)]),
            Pattern::point(5),
        ],
    );

    let vals = pat.values();
    assert_eq!(vals.len(), 5);

    // Pre-order: 1 (root), 2 (first element), 3 (second element), 4 (nested), 5 (third element)
    assert_eq!(*vals[0], 1);
    assert_eq!(*vals[1], 2);
    assert_eq!(*vals[2], 3);
    assert_eq!(*vals[3], 4);
    assert_eq!(*vals[4], 5);
}

#[test]
fn test_values_order_consistency() {
    // T076: values verifying order consistency (parent first, then elements)
    let pat = Pattern::pattern(
        "root",
        vec![
            Pattern::pattern("branch1", vec![Pattern::point("leaf1")]),
            Pattern::pattern("branch2", vec![Pattern::point("leaf2")]),
        ],
    );

    let vals = pat.values();

    // Pre-order: root, branch1, leaf1, branch2, leaf2
    assert_eq!(*vals[0], "root");
    assert_eq!(*vals[1], "branch1");
    assert_eq!(*vals[2], "leaf1");
    assert_eq!(*vals[3], "branch2");
    assert_eq!(*vals[4], "leaf2");
}

#[test]
fn test_values_with_duplicates() {
    // T077: values with duplicate values (should return all including duplicates)
    let pat = Pattern::pattern(
        5,
        vec![
            Pattern::point(5),
            Pattern::pattern(5, vec![Pattern::point(5)]),
        ],
    );

    let vals = pat.values();
    assert_eq!(vals.len(), 4);
    assert!(vals.iter().all(|v| **v == 5));
}

// ========== Integration Tests (operations working together) ==========

#[test]
fn test_size_equals_values_length() {
    // Invariant: size() should equal values().len()
    let pat = Pattern::pattern(
        1,
        vec![
            Pattern::point(2),
            Pattern::pattern(3, vec![Pattern::point(4), Pattern::point(5)]),
        ],
    );

    assert_eq!(pat.size(), pat.values().len());
}

#[test]
fn test_length_vs_size_relationship() {
    // For non-atomic patterns: size >= length + 1
    let pat = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);

    assert!(pat.size() >= pat.length() + 1);
}

#[test]
fn test_depth_zero_means_atomic() {
    // Invariant: depth == 0 if and only if pattern is atomic
    let atomic = Pattern::point(42);
    let non_atomic = Pattern::pattern(5, vec![Pattern::point(10)]);

    assert_eq!(atomic.depth(), 0);
    assert!(atomic.is_atomic());

    assert!(non_atomic.depth() > 0);
    assert!(!non_atomic.is_atomic());
}

#[test]
fn test_large_pattern_operations() {
    // Integration test with a large pattern
    let elements: Vec<Pattern<i32>> = (0..1000).map(Pattern::point).collect();
    let pat = Pattern::pattern(999, elements);

    assert_eq!(pat.length(), 1000);
    assert_eq!(pat.size(), 1001); // 1 root + 1000 elements
    assert_eq!(pat.depth(), 1);
    assert_eq!(pat.values().len(), 1001);
}

#[test]
fn test_empty_pattern_elements() {
    // Pattern with no elements but still has a value
    let pat = Pattern::pattern(42, vec![]);

    assert_eq!(pat.length(), 0);
    assert_eq!(pat.size(), 1); // Just the root
    assert_eq!(pat.depth(), 0); // No nested elements
    assert_eq!(pat.values().len(), 1);
    assert_eq!(*pat.values()[0], 42);
}
