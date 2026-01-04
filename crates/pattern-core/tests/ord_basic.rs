//! Basic comparison tests for Pattern Ord implementation
//!
//! This module tests the fundamental ordering and comparison operations for patterns,
//! including atomic patterns, nested patterns, and various edge cases like deep nesting
//! and wide patterns.

use pattern_core::Pattern;
use std::cmp::Ordering;

// ============================================================================
// T009: Compare two atomic patterns with different values
// ============================================================================

#[test]
fn test_compare_atomic_patterns_different_values() {
    let p1 = Pattern::point(1);
    let p2 = Pattern::point(2);

    // Using cmp
    assert_eq!(p1.cmp(&p2), Ordering::Less);
    assert_eq!(p2.cmp(&p1), Ordering::Greater);

    // Using comparison operators
    assert!(p1 < p2);
    assert!(p1 <= p2);
    assert!(p2 > p1);
    assert!(p2 >= p1);
    assert!(p1 != p2);
}

// ============================================================================
// T010: Compare two atomic patterns with same value
// ============================================================================

#[test]
fn test_compare_atomic_patterns_same_value() {
    let p1 = Pattern::point(5);
    let p2 = Pattern::point(5);

    // Using cmp
    assert_eq!(p1.cmp(&p2), Ordering::Equal);

    // Using comparison operators
    assert!(p1 == p2);
    assert!(p1 <= p2);
    assert!(p1 >= p2);
    assert!(!(p1 < p2));
    assert!(!(p1 > p2));

    // Reflexivity: comparing with self
    assert_eq!(p1.cmp(&p1), Ordering::Equal);
}

// ============================================================================
// T011: Compare nested patterns with different values (value-first precedence)
// ============================================================================

#[test]
fn test_compare_nested_patterns_different_values() {
    // Values differ - elements should not matter
    let p1 = Pattern::pattern(3, vec![Pattern::point(100)]);
    let p2 = Pattern::pattern(4, vec![Pattern::point(1)]);

    assert!(p1 < p2); // 3 < 4, elements not compared

    // Even with many elements
    let p3 = Pattern::pattern(
        1,
        vec![
            Pattern::point(999),
            Pattern::point(888),
            Pattern::point(777),
        ],
    );
    let p4 = Pattern::pattern(2, vec![Pattern::point(1)]);

    assert!(p3 < p4); // Value takes precedence
}

// ============================================================================
// T012: Compare nested patterns with same value but different elements
// ============================================================================

#[test]
fn test_compare_nested_patterns_same_value_different_elements() {
    // Same value, first element differs
    let p1 = Pattern::pattern(5, vec![Pattern::point(1)]);
    let p2 = Pattern::pattern(5, vec![Pattern::point(2)]);

    assert!(p1 < p2); // Values equal, 1 < 2

    // Same value, elements differ at second position
    let p3 = Pattern::pattern(5, vec![Pattern::point(1), Pattern::point(3)]);
    let p4 = Pattern::pattern(5, vec![Pattern::point(1), Pattern::point(4)]);

    assert!(p3 < p4); // First elements equal, 3 < 4

    // Nested elements also compared
    let p5 = Pattern::pattern(5, vec![Pattern::pattern(1, vec![Pattern::point(10)])]);
    let p6 = Pattern::pattern(5, vec![Pattern::pattern(1, vec![Pattern::point(20)])]);

    assert!(p5 < p6); // Recursively compared
}

// ============================================================================
// T013: Compare patterns where one is a prefix of another
// ============================================================================

#[test]
fn test_compare_patterns_prefix() {
    // Shorter pattern is less than longer when it's a prefix
    let p1 = Pattern::pattern(5, vec![Pattern::point(1)]);
    let p2 = Pattern::pattern(5, vec![Pattern::point(1), Pattern::point(2)]);

    assert!(p1 < p2); // [1] < [1, 2]

    // Empty elements vs non-empty
    let p3 = Pattern::pattern(5, vec![]);
    let p4 = Pattern::pattern(5, vec![Pattern::point(1)]);

    assert!(p3 < p4); // [] < [1]

    // Three levels of nesting
    let p5 = Pattern::pattern(5, vec![Pattern::point(1), Pattern::point(2)]);
    let p6 = Pattern::pattern(
        5,
        vec![Pattern::point(1), Pattern::point(2), Pattern::point(3)],
    );

    assert!(p5 < p6); // [1, 2] < [1, 2, 3]
}

// ============================================================================
// T014: Compare deeply nested patterns (50+ levels)
// ============================================================================

#[test]
fn test_compare_deeply_nested_patterns() {
    fn create_deep_pattern(depth: usize, value: i32) -> Pattern<i32> {
        if depth == 0 {
            Pattern::point(value)
        } else {
            Pattern::pattern(value, vec![create_deep_pattern(depth - 1, value + 1)])
        }
    }

    // Compare deeply nested patterns (80 levels)
    let p1 = create_deep_pattern(80, 0);
    let p2 = create_deep_pattern(80, 0);

    assert_eq!(p1.cmp(&p2), Ordering::Equal); // Identical structures

    // Different at root level
    let p3 = create_deep_pattern(80, 0);
    let p4 = create_deep_pattern(80, 1);

    assert!(p3 < p4); // 0 < 1, depth doesn't matter

    // Different at leaf level - must traverse entire depth
    let p5 = Pattern::pattern(0, vec![create_deep_pattern(79, 1)]);
    let p6 = Pattern::pattern(0, vec![create_deep_pattern(79, 2)]);

    assert!(p5 < p6); // Must compare through all 80 levels
}

// ============================================================================
// T015: Compare wide patterns (1000+ elements)
// ============================================================================

#[test]
fn test_compare_wide_patterns() {
    // Create wide patterns with many elements
    let elements1: Vec<Pattern<i32>> = (0..1500).map(Pattern::point).collect();
    let p1 = Pattern::pattern(0, elements1);

    let elements2: Vec<Pattern<i32>> = (0..1500).map(Pattern::point).collect();
    let p2 = Pattern::pattern(0, elements2);

    assert_eq!(p1.cmp(&p2), Ordering::Equal); // Identical

    // Different at element 1000
    let mut elements3: Vec<Pattern<i32>> = (0..1500).map(Pattern::point).collect();
    elements3[1000] = Pattern::point(9999); // Change middle element
    let p3 = Pattern::pattern(0, elements3);

    assert!(p2 < p3); // Differs at position 1000

    // Different lengths
    let elements4: Vec<Pattern<i32>> = (0..1000).map(Pattern::point).collect();
    let p4 = Pattern::pattern(0, elements4);

    let elements5: Vec<Pattern<i32>> = (0..2000).map(Pattern::point).collect();
    let p5 = Pattern::pattern(0, elements5);

    assert!(p4 < p5); // 1000 elements < 2000 elements
}

// ============================================================================
// T029: Sort small collection of patterns (10 patterns)
// ============================================================================

#[test]
fn test_sort_small_collection() {
    let mut patterns = vec![
        Pattern::point(5),
        Pattern::point(2),
        Pattern::point(8),
        Pattern::point(1),
        Pattern::point(9),
        Pattern::point(3),
        Pattern::point(7),
        Pattern::point(4),
        Pattern::point(6),
        Pattern::point(10),
    ];

    patterns.sort();

    let expected: Vec<Pattern<i32>> = (1..=10).map(Pattern::point).collect();
    assert_eq!(patterns, expected);
}

// ============================================================================
// T030: Sort large collection of patterns (1000 patterns)
// ============================================================================

#[test]
fn test_sort_large_collection() {
    // Create 1000 patterns with pseudo-random values (using deterministic pattern)
    // This avoids needing the rand crate while still testing sorting at scale
    let mut patterns: Vec<Pattern<i32>> = (0..1000)
        .map(|i| Pattern::point((i * 7919) % 10000)) // Prime multiplier for distribution
        .collect();

    // Sort them
    patterns.sort();

    // Verify sorted order
    for i in 0..patterns.len() - 1 {
        assert!(patterns[i] <= patterns[i + 1], "Not sorted at index {}", i);
    }
}

// ============================================================================
// T031: Binary search in sorted pattern vector
// ============================================================================

#[test]
fn test_binary_search() {
    let patterns: Vec<Pattern<i32>> = (0..100).step_by(2).map(Pattern::point).collect();

    // Search for existing element
    let target = Pattern::point(50);
    match patterns.binary_search(&target) {
        Ok(index) => assert_eq!(patterns[index], target),
        Err(_) => panic!("Should have found pattern"),
    }

    // Search for non-existing element
    let target2 = Pattern::point(51); // Odd number, not in collection
    match patterns.binary_search(&target2) {
        Ok(_) => panic!("Should not have found pattern"),
        Err(index) => {
            // index is where it would be inserted
            assert!(index > 0 && index < patterns.len());
            assert!(patterns[index - 1] < target2);
            assert!(target2 < patterns[index]);
        }
    }
}

// ============================================================================
// T032: Verify sort stability with equal patterns
// ============================================================================

#[test]
fn test_sort_stability() {
    // Create patterns with same value but track original positions
    #[derive(Debug)]
    struct Tagged {
        pattern: Pattern<i32>,
        original_index: usize,
    }

    let mut tagged: Vec<Tagged> = vec![
        Tagged {
            pattern: Pattern::point(5),
            original_index: 0,
        },
        Tagged {
            pattern: Pattern::point(3),
            original_index: 1,
        },
        Tagged {
            pattern: Pattern::point(5),
            original_index: 2,
        },
        Tagged {
            pattern: Pattern::point(3),
            original_index: 3,
        },
        Tagged {
            pattern: Pattern::point(5),
            original_index: 4,
        },
    ];

    // stable_sort preserves relative order of equal elements
    tagged.sort_by(|a, b| a.pattern.cmp(&b.pattern));

    // Verify all 3's come before all 5's
    let threes_end = tagged
        .iter()
        .position(|t| t.pattern == Pattern::point(5))
        .unwrap();
    for i in 0..threes_end {
        assert_eq!(tagged[i].pattern, Pattern::point(3));
    }
    for i in threes_end..tagged.len() {
        assert_eq!(tagged[i].pattern, Pattern::point(5));
    }

    // Verify relative order preserved for equal elements (3's)
    assert!(tagged[0].original_index < tagged[1].original_index);

    // Verify relative order preserved for equal elements (5's)
    assert!(tagged[2].original_index < tagged[3].original_index);
    assert!(tagged[3].original_index < tagged[4].original_index);
}
