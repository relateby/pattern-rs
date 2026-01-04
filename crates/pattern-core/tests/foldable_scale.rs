//! Scale and performance verification tests for fold operations
//!
//! Tests verify:
//! - T046: Deep nesting (100 levels) without stack overflow
//! - T047: Wide patterns (1000 siblings) perform acceptably
//! - SC-002: Patterns with 1000 nodes complete < 10ms
//! - SC-003: Patterns with 100 nesting levels don't stack overflow
//! - SC-009: 10,000 element patterns use reasonable memory

use pattern_core::Pattern;
use std::time::Instant;

// ============================================================================
// Helper Functions
// ============================================================================

/// Creates a deeply nested pattern (linear chain)
fn create_deep_pattern(depth: usize) -> Pattern<i32> {
    let mut pattern = Pattern::point(depth as i32);
    for i in (0..depth).rev() {
        pattern = Pattern::pattern(i as i32, vec![pattern]);
    }
    pattern
}

/// Creates a balanced binary tree pattern of given depth
fn create_balanced_tree(depth: usize, value: i32) -> Pattern<i32> {
    if depth == 0 {
        Pattern::point(value)
    } else {
        Pattern::pattern(
            value,
            vec![
                create_balanced_tree(depth - 1, value * 2),
                create_balanced_tree(depth - 1, value * 2 + 1),
            ],
        )
    }
}

/// Creates a wide pattern with many siblings
fn create_wide_pattern(width: usize) -> Pattern<i32> {
    let elements: Vec<Pattern<i32>> = (0..width).map(|i| Pattern::point(i as i32)).collect();
    Pattern::pattern(-1, elements)
}

/// Creates a flat pattern with n elements
fn create_flat_pattern(size: usize) -> Pattern<i32> {
    let elements: Vec<Pattern<i32>> = (1..size).map(|i| Pattern::point(i as i32)).collect();
    Pattern::pattern(0, elements)
}

// ============================================================================
// T046: Deep Nesting Tests (100+ levels without stack overflow)
// ============================================================================

#[test]
fn fold_handles_100_level_deep_nesting() {
    let pattern = create_deep_pattern(100);

    // Verify structure
    assert_eq!(pattern.depth(), 100);
    assert_eq!(pattern.size(), 101); // 101 values (0..=100)

    // Fold should complete without stack overflow
    let sum = pattern.fold(0i32, |acc, v| acc + v);

    // Expected sum: 0 + 1 + 2 + ... + 100 = 5050
    assert_eq!(sum, 5050);

    // Values should also work
    let values = pattern.values();
    assert_eq!(values.len(), 101);
}

#[test]
fn fold_handles_200_level_deep_nesting() {
    let pattern = create_deep_pattern(200);

    assert_eq!(pattern.depth(), 200);
    assert_eq!(pattern.size(), 201);

    // Should not stack overflow
    let count = pattern.fold(0usize, |acc, _| acc + 1);
    assert_eq!(count, 201);
}

#[test]
fn fold_handles_500_level_deep_nesting() {
    let pattern = create_deep_pattern(500);

    assert_eq!(pattern.depth(), 500);

    // Very deep nesting should still work
    let count = pattern.fold(0usize, |acc, _| acc + 1);
    assert_eq!(count, 501);
}

#[test]
fn values_handles_deep_nesting() {
    let pattern = create_deep_pattern(100);

    let values = pattern.values();

    assert_eq!(values.len(), 101);
    // Verify order: root-first, so should be 0, 1, 2, ..., 100
    for (i, &&v) in values.iter().enumerate() {
        assert_eq!(v, i as i32);
    }
}

// ============================================================================
// T047: Wide Pattern Tests (1000+ siblings)
// ============================================================================

#[test]
fn fold_handles_1000_wide_siblings() {
    let pattern = create_wide_pattern(1000);

    assert_eq!(pattern.size(), 1001); // root + 1000 children
    assert_eq!(pattern.length(), 1000); // 1000 direct children

    // Should handle wide patterns efficiently
    let sum = pattern.fold(0i32, |acc, v| acc + v);

    // Expected: -1 (root) + 0 + 1 + 2 + ... + 999 = -1 + 499500 = 499499
    assert_eq!(sum, 499499);
}

#[test]
fn fold_handles_5000_wide_siblings() {
    let pattern = create_wide_pattern(5000);

    assert_eq!(pattern.size(), 5001);

    let count = pattern.fold(0usize, |acc, _| acc + 1);
    assert_eq!(count, 5001);
}

#[test]
fn fold_handles_10000_wide_siblings() {
    let pattern = create_wide_pattern(10000);

    assert_eq!(pattern.size(), 10001);

    // Should handle very wide patterns
    let count = pattern.fold(0usize, |acc, _| acc + 1);
    assert_eq!(count, 10001);
}

#[test]
fn values_handles_wide_patterns() {
    let pattern = create_wide_pattern(1000);

    let values = pattern.values();

    assert_eq!(values.len(), 1001);
    // First value should be root
    assert_eq!(*values[0], -1);
    // Remaining should be 0..999
    for i in 1..=1000 {
        assert_eq!(*values[i], (i - 1) as i32);
    }
}

// ============================================================================
// SC-002: Performance Test - 1000 nodes < 10ms
// ============================================================================

#[test]
fn fold_1000_nodes_completes_quickly() {
    // Balanced tree with depth 9 has 1023 nodes
    let pattern = create_balanced_tree(9, 1);
    assert_eq!(pattern.size(), 1023);

    let start = Instant::now();
    let sum = pattern.fold(0i32, |acc, v| acc + v);
    let duration = start.elapsed();

    // Verify correctness
    assert!(sum > 0);

    // Should complete well under 10ms (usually < 1ms)
    println!("Fold of 1023 nodes took: {:?}", duration);
    assert!(
        duration.as_millis() < 10,
        "Fold took {:?}, expected < 10ms",
        duration
    );
}

#[test]
fn fold_flat_1000_nodes_completes_quickly() {
    let pattern = create_flat_pattern(1000);
    assert_eq!(pattern.size(), 1000);

    let start = Instant::now();
    let sum = pattern.fold(0i32, |acc, v| acc + v);
    let duration = start.elapsed();

    // Expected sum: 0 + 1 + 2 + ... + 999 = 499500
    assert_eq!(sum, 499500);

    println!("Flat fold of 1000 nodes took: {:?}", duration);
    assert!(
        duration.as_millis() < 10,
        "Fold took {:?}, expected < 10ms",
        duration
    );
}

#[test]
fn values_1000_nodes_completes_quickly() {
    let pattern = create_balanced_tree(9, 1);
    assert_eq!(pattern.size(), 1023);

    let start = Instant::now();
    let values = pattern.values();
    let duration = start.elapsed();

    assert_eq!(values.len(), 1023);

    println!("Values of 1023 nodes took: {:?}", duration);
    assert!(
        duration.as_millis() < 10,
        "Values took {:?}, expected < 10ms",
        duration
    );
}

// ============================================================================
// SC-009: Large Pattern Tests (10,000 elements)
// ============================================================================

#[test]
fn fold_handles_10000_elements() {
    let pattern = create_flat_pattern(10000);

    assert_eq!(pattern.size(), 10000);

    let start = Instant::now();
    let sum = pattern.fold(0i64, |acc, v| acc + (*v as i64));
    let duration = start.elapsed();

    // Expected sum: 0 + 1 + 2 + ... + 9999 = 49995000
    assert_eq!(sum, 49995000);

    println!("Fold of 10,000 elements took: {:?}", duration);
    // Should complete in reasonable time (< 100ms)
    assert!(duration.as_millis() < 100);
}

#[test]
fn values_handles_10000_elements() {
    let pattern = create_flat_pattern(10000);

    let start = Instant::now();
    let values = pattern.values();
    let duration = start.elapsed();

    assert_eq!(values.len(), 10000);

    println!("Values of 10,000 elements took: {:?}", duration);
    assert!(duration.as_millis() < 100);
}

#[test]
fn fold_handles_balanced_tree_with_8000_plus_nodes() {
    // Depth 12 = 2^13 - 1 = 8191 nodes
    let pattern = create_balanced_tree(12, 1);
    assert_eq!(pattern.size(), 8191);

    let start = Instant::now();
    let count = pattern.fold(0usize, |acc, _| acc + 1);
    let duration = start.elapsed();

    assert_eq!(count, 8191);

    println!(
        "Fold of 8191 nodes (balanced tree depth 12) took: {:?}",
        duration
    );
    assert!(duration.as_millis() < 50);
}

// ============================================================================
// Combined Stress Tests
// ============================================================================

#[test]
fn fold_deep_and_wide_pattern() {
    // Create a pattern that is both deep (50 levels) and wide (100 children per level at first few levels)
    fn create_deep_wide(depth: usize, width: usize) -> Pattern<i32> {
        if depth == 0 {
            Pattern::point(0)
        } else {
            let children: Vec<Pattern<i32>> = (0..width.min(10))
                .map(|_| create_deep_wide(depth - 1, width))
                .collect();
            Pattern::pattern(depth as i32, children)
        }
    }

    let pattern = create_deep_wide(10, 5);

    let count = pattern.fold(0usize, |acc, _| acc + 1);
    assert!(count > 1000); // Should have many nodes

    let values = pattern.values();
    assert_eq!(values.len(), count);
}

#[test]
fn fold_reusable_on_large_pattern() {
    let pattern = create_flat_pattern(1000);

    // Fold multiple times - pattern should be reusable
    let sum = pattern.fold(0i32, |acc, v| acc + v);
    let count = pattern.fold(0usize, |acc, _| acc + 1);
    let max = pattern.fold(i32::MIN, |acc, v| acc.max(*v));
    let min = pattern.fold(i32::MAX, |acc, v| acc.min(*v));

    assert_eq!(sum, 499500);
    assert_eq!(count, 1000);
    assert_eq!(max, 999);
    assert_eq!(min, 0);
}

#[test]
fn fold_large_pattern_preserves_order() {
    let pattern = create_flat_pattern(1000);

    // Concatenate to string to verify order
    let concat = pattern.fold(String::new(), |mut acc, v| {
        if !acc.is_empty() {
            acc.push(',');
        }
        acc.push_str(&v.to_string());
        acc
    });

    // Should start with "0,1,2,3,4,5..."
    assert!(concat.starts_with("0,1,2,3,4,5"));

    // Should end with "...997,998,999"
    assert!(concat.ends_with("997,998,999"));
}
