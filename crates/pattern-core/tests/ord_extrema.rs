//! Extrema tests for Pattern Ord implementation
//!
//! This module tests finding minimum and maximum patterns in collections,
//! as well as related operations like clamp, min_by, and max_by.

use pattern_core::Pattern;

// ============================================================================
// T036: Find minimum pattern in small collection (5 patterns)
// ============================================================================

#[test]
fn test_min_small_collection() {
    let patterns = vec![
        Pattern::point(5),
        Pattern::point(2),
        Pattern::point(8),
        Pattern::point(1),
        Pattern::point(4),
    ];

    let min = patterns.iter().min().unwrap();
    assert_eq!(min, &Pattern::point(1));

    // Verify it's actually the minimum
    for p in &patterns {
        assert!(min <= p);
    }
}

// ============================================================================
// T037: Find maximum pattern in small collection (5 patterns)
// ============================================================================

#[test]
fn test_max_small_collection() {
    let patterns = vec![
        Pattern::point(5),
        Pattern::point(2),
        Pattern::point(8),
        Pattern::point(1),
        Pattern::point(4),
    ];

    let max = patterns.iter().max().unwrap();
    assert_eq!(max, &Pattern::point(8));

    // Verify it's actually the maximum
    for p in &patterns {
        assert!(max >= p);
    }
}

// ============================================================================
// T038: Find minimum in large collection (1000 patterns)
// ============================================================================

#[test]
fn test_min_large_collection() {
    // Create 1000 patterns with deterministic pseudo-random values
    let patterns: Vec<Pattern<i32>> = (0..1000)
        .map(|i| Pattern::point((i * 7919) % 10000))
        .collect();

    let min = patterns.iter().min().unwrap();

    // Verify it's actually the minimum
    for p in &patterns {
        assert!(
            min <= p,
            "Found pattern {:?} less than supposed min {:?}",
            p,
            min
        );
    }

    // Should be the smallest value in the modulo range
    let values: Vec<i32> = patterns.iter().map(|p| p.value).collect();
    let min_value = values.iter().min().unwrap();
    assert_eq!(min.value, *min_value);
}

// ============================================================================
// T039: Find maximum in large collection (1000 patterns)
// ============================================================================

#[test]
fn test_max_large_collection() {
    // Create 1000 patterns with deterministic pseudo-random values
    let patterns: Vec<Pattern<i32>> = (0..1000)
        .map(|i| Pattern::point((i * 7919) % 10000))
        .collect();

    let max = patterns.iter().max().unwrap();

    // Verify it's actually the maximum
    for p in &patterns {
        assert!(
            max >= p,
            "Found pattern {:?} greater than supposed max {:?}",
            p,
            max
        );
    }

    // Should be the largest value in the modulo range
    let values: Vec<i32> = patterns.iter().map(|p| p.value).collect();
    let max_value = values.iter().max().unwrap();
    assert_eq!(max.value, *max_value);
}

// ============================================================================
// T040: Min/max with single-element collection
// ============================================================================

#[test]
fn test_min_max_single_element() {
    let patterns = vec![Pattern::point(42)];

    let min = patterns.iter().min().unwrap();
    let max = patterns.iter().max().unwrap();

    // Both should be the same element
    assert_eq!(min, &Pattern::point(42));
    assert_eq!(max, &Pattern::point(42));
    assert_eq!(min, max);
}

// ============================================================================
// T041: Min/max with duplicate patterns
// ============================================================================

#[test]
fn test_min_max_duplicates() {
    let patterns = vec![
        Pattern::point(5),
        Pattern::point(2),
        Pattern::point(8),
        Pattern::point(2), // Duplicate minimum
        Pattern::point(8), // Duplicate maximum
        Pattern::point(4),
        Pattern::point(2), // Another duplicate minimum
    ];

    let min = patterns.iter().min().unwrap();
    let max = patterns.iter().max().unwrap();

    assert_eq!(min, &Pattern::point(2));
    assert_eq!(max, &Pattern::point(8));

    // Count occurrences
    let min_count = patterns.iter().filter(|p| *p == min).count();
    let max_count = patterns.iter().filter(|p| *p == max).count();

    assert_eq!(min_count, 3);
    assert_eq!(max_count, 2);
}

// ============================================================================
// T042: Clamp pattern to min/max range
// ============================================================================

#[test]
fn test_clamp_pattern() {
    let lower = Pattern::point(10);
    let upper = Pattern::point(20);

    // Below range - should clamp to lower
    let too_low = Pattern::point(5);
    assert_eq!(too_low.clone().clamp(lower.clone(), upper.clone()), lower);

    // Within range - should stay unchanged
    let in_range = Pattern::point(15);
    assert_eq!(
        in_range.clone().clamp(lower.clone(), upper.clone()),
        in_range
    );

    // Above range - should clamp to upper
    let too_high = Pattern::point(25);
    assert_eq!(too_high.clone().clamp(lower.clone(), upper.clone()), upper);

    // At boundaries - should stay at boundary
    assert_eq!(lower.clone().clamp(lower.clone(), upper.clone()), lower);
    assert_eq!(upper.clone().clamp(lower.clone(), upper.clone()), upper);
}

// ============================================================================
// T043: Iterator min()/max() methods work correctly
// ============================================================================

#[test]
fn test_iterator_min_max_methods() {
    let patterns = vec![
        Pattern::point(15),
        Pattern::point(3),
        Pattern::point(42),
        Pattern::point(7),
        Pattern::point(23),
    ];

    // Test iterator min()
    let min = patterns.iter().min();
    assert!(min.is_some());
    assert_eq!(min.unwrap(), &Pattern::point(3));

    // Test iterator max()
    let max = patterns.iter().max();
    assert!(max.is_some());
    assert_eq!(max.unwrap(), &Pattern::point(42));

    // Test min_by with custom comparison
    let min_by = patterns.iter().min_by(|a, b| a.cmp(b));
    assert_eq!(min_by, min);

    // Test max_by with custom comparison
    let max_by = patterns.iter().max_by(|a, b| a.cmp(b));
    assert_eq!(max_by, max);

    // Test min_by_key
    let min_by_key = patterns.iter().min_by_key(|p| p.value);
    assert_eq!(min_by_key.unwrap().value, 3);

    // Test max_by_key
    let max_by_key = patterns.iter().max_by_key(|p| p.value);
    assert_eq!(max_by_key.unwrap().value, 42);
}

// ============================================================================
// Additional extrema tests
// ============================================================================

#[test]
fn test_min_max_with_nested_patterns() {
    // Test extrema with nested patterns
    let patterns = vec![
        Pattern::pattern(5, vec![Pattern::point(10)]),
        Pattern::pattern(3, vec![Pattern::point(20)]),
        Pattern::pattern(7, vec![Pattern::point(5)]),
        Pattern::pattern(3, vec![Pattern::point(15)]),
    ];

    let min = patterns.iter().min().unwrap();
    let max = patterns.iter().max().unwrap();

    // Min should have value 3 (first by value comparison)
    assert_eq!(min.value, 3);

    // Max should have value 7
    assert_eq!(max.value, 7);

    // Among patterns with value 3, min should be the one with elements [15]
    assert_eq!(min, &Pattern::pattern(3, vec![Pattern::point(15)]));
}

#[test]
fn test_min_max_empty_collection() {
    let patterns: Vec<Pattern<i32>> = vec![];

    // min() and max() on empty collections return None
    assert!(patterns.iter().min().is_none());
    assert!(patterns.iter().max().is_none());
}

#[test]
fn test_extrema_with_std_functions() {
    // Test std::cmp::min and std::cmp::max functions
    let p1 = Pattern::point(10);
    let p2 = Pattern::point(20);

    assert_eq!(std::cmp::min(p1.clone(), p2.clone()), p1);
    assert_eq!(std::cmp::max(p1.clone(), p2.clone()), p2);

    // Test with equal patterns
    let p3 = Pattern::point(15);
    let p4 = Pattern::point(15);

    assert_eq!(std::cmp::min(p3.clone(), p4.clone()), p3);
    assert_eq!(std::cmp::max(p3.clone(), p4.clone()), p3); // Returns first one
}

#[test]
fn test_min_max_value_precedence() {
    // Verify that value takes precedence in min/max operations
    let patterns = vec![
        Pattern::pattern(5, vec![Pattern::point(1000)]),
        Pattern::pattern(3, vec![Pattern::point(1)]),
        Pattern::pattern(7, vec![]),
    ];

    let min = patterns.iter().min().unwrap();
    let max = patterns.iter().max().unwrap();

    // Min is determined by value (3), regardless of elements
    assert_eq!(min.value, 3);

    // Max is determined by value (7), regardless of elements
    assert_eq!(max.value, 7);
}

#[test]
fn test_extrema_consistent_with_sorting() {
    // Verify that min/max are consistent with sorting
    let mut patterns = vec![
        Pattern::point(15),
        Pattern::point(3),
        Pattern::point(42),
        Pattern::point(7),
        Pattern::point(23),
    ];

    let min_before = patterns.iter().min().unwrap().clone();
    let max_before = patterns.iter().max().unwrap().clone();

    patterns.sort();

    // First element after sorting should be min
    assert_eq!(patterns.first().unwrap(), &min_before);

    // Last element after sorting should be max
    assert_eq!(patterns.last().unwrap(), &max_before);
}
