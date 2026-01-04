//! Integration tests for fold with other Pattern operations
//!
//! Tests for composing fold with map (functor) and other functional patterns.

use pattern_core::Pattern;

// ============================================================================
// T031: Map-then-fold composition test
// ============================================================================

#[test]
fn map_then_fold_composition() {
    let pattern = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);

    // Map to double values, then fold to sum
    let result = pattern.map(|&v| v * 2).fold(0, |acc, &v| acc + v);

    assert_eq!(result, 12); // (1*2) + (2*2) + (3*2) = 2 + 4 + 6
}

#[test]
fn map_transform_type_then_fold() {
    let pattern = Pattern::pattern("hello", vec![Pattern::point("world"), Pattern::point("!")]);

    // Map to lengths, then fold to sum
    let total_length = pattern.map(|s| s.len()).fold(0, |acc, &len| acc + len);

    assert_eq!(total_length, 11); // 5 + 5 + 1
}

#[test]
fn map_then_fold_preserves_structure() {
    let pattern = Pattern::pattern(
        1,
        vec![
            Pattern::pattern(2, vec![Pattern::point(3)]),
            Pattern::point(4),
        ],
    );

    // Map then fold should process all values in correct order
    let result = pattern.map(|&v| v + 10).fold(0, |acc, &v| acc + v);

    assert_eq!(result, 50); // (1+10) + (2+10) + (3+10) + (4+10) = 11 + 12 + 13 + 14
}

// ============================================================================
// T032: Fold-multiple-times test (pattern reuse)
// ============================================================================

#[test]
fn fold_multiple_times_on_same_pattern() {
    let pattern = Pattern::pattern(2, vec![Pattern::point(3), Pattern::point(4)]);

    // Multiple independent folds
    let sum = pattern.fold(0, |acc, &v| acc + v);
    let product = pattern.fold(1, |acc, &v| acc * v);
    let count = pattern.fold(0, |acc, _| acc + 1);
    let max = pattern.fold(i32::MIN, |acc, &v| acc.max(v));

    assert_eq!(sum, 9); // 2 + 3 + 4
    assert_eq!(product, 24); // 2 * 3 * 4
    assert_eq!(count, 3);
    assert_eq!(max, 4);
}

#[test]
fn fold_after_fold_independent() {
    let pattern = Pattern::pattern("a", vec![Pattern::point("b"), Pattern::point("c")]);

    let concat1 = pattern.fold(String::new(), |acc, s| acc + s);
    let concat2 = pattern.fold(String::from("start:"), |acc, s| acc + s);

    assert_eq!(concat1, "abc");
    assert_eq!(concat2, "start:abc");
}

// ============================================================================
// T033: Pattern-unchanged-after-fold test
// ============================================================================

#[test]
fn pattern_unchanged_after_single_fold() {
    let pattern = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);
    let original = pattern.clone();

    let _ = pattern.fold(0, |acc, &v| acc + v);

    assert_eq!(pattern, original);
}

#[test]
fn pattern_unchanged_after_multiple_folds() {
    let pattern = Pattern::pattern("x", vec![Pattern::point("y"), Pattern::point("z")]);
    let original = pattern.clone();

    let _ = pattern.fold(String::new(), |acc, s| acc + s);
    let _ = pattern.fold(0, |acc, _| acc + 1);
    let _ = pattern.fold(Vec::new(), |mut acc: Vec<String>, s| {
        acc.push(s.to_string());
        acc
    });

    assert_eq!(pattern, original);
}

#[test]
fn pattern_reusable_after_fold() {
    let pattern = Pattern::pattern(5, vec![Pattern::point(10)]);

    // First use
    let sum1 = pattern.fold(0, |acc, &v| acc + v);

    // Second use (pattern should still be valid)
    let sum2 = pattern.fold(100, |acc, &v| acc + v);

    assert_eq!(sum1, 15); // 5 + 10
    assert_eq!(sum2, 115); // 100 + 5 + 10
}

// ============================================================================
// T034: Complex pipeline test (map, fold, compare)
// ============================================================================

#[test]
fn complex_pipeline_map_fold_compare() {
    let pattern1 = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);
    let pattern2 = Pattern::pattern(10, vec![Pattern::point(20), Pattern::point(30)]);

    // Transform both patterns and compare results
    let sum1 = pattern1.clone().map(|&v| v * 2).fold(0, |acc, &v| acc + v);
    let sum2 = pattern2.clone().map(|&v| v / 10).fold(0, |acc, &v| acc + v);

    assert_eq!(sum1, 12); // (1*2) + (2*2) + (3*2)
    assert_eq!(sum2, 6); // (10/10) + (20/10) + (30/10)
    assert!(sum1 > sum2);
}

#[test]
fn pipeline_with_filtering_logic() {
    let pattern = Pattern::pattern(
        1,
        vec![
            Pattern::point(2),
            Pattern::point(3),
            Pattern::point(4),
            Pattern::point(5),
        ],
    );

    // Count even numbers using fold
    let even_count = pattern.fold(0, |acc, &v| if v % 2 == 0 { acc + 1 } else { acc });

    // Sum odd numbers using fold
    let odd_sum = pattern.fold(0, |acc, &v| if v % 2 != 0 { acc + v } else { acc });

    assert_eq!(even_count, 2); // 2, 4
    assert_eq!(odd_sum, 9); // 1 + 3 + 5
}

#[test]
fn pipeline_map_to_option_then_fold() {
    let pattern = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);

    // Map to Option, then collect valid values with fold
    let with_options = pattern.map(|&v| if v > 1 { Some(v * 10) } else { None });

    let valid_values = with_options.fold(Vec::new(), |mut acc, opt| {
        if let Some(val) = opt {
            acc.push(*val);
        }
        acc
    });

    assert_eq!(valid_values, vec![20, 30]); // 1 filtered out, 2 and 3 mapped
}

#[test]
fn pipeline_with_values_and_iterator() {
    let pattern = Pattern::pattern(
        1,
        vec![Pattern::point(2), Pattern::point(3), Pattern::point(4)],
    );

    // Use values() to get Vec, then use Iterator methods
    let doubled_sum: i32 = pattern.values().iter().map(|&&v| v * 2).sum();

    assert_eq!(doubled_sum, 20); // (1*2) + (2*2) + (3*2) + (4*2)

    // Compare with direct fold
    let direct_fold_doubled = pattern.fold(0, |acc, &v| acc + (v * 2));

    assert_eq!(doubled_sum, direct_fold_doubled);
}

#[test]
fn pipeline_nested_transformations() {
    let inner = Pattern::pattern(5, vec![Pattern::point(6)]);
    let pattern = Pattern::pattern(1, vec![Pattern::point(2), inner, Pattern::point(7)]);

    // Chain: map to double, then map to string, then fold to concatenate
    let result = pattern
        .map(|&v| v * 2)
        .map(|&v| v.to_string())
        .fold(String::new(), |acc, s| {
            if acc.is_empty() {
                s.to_string()
            } else {
                format!("{}-{}", acc, s)
            }
        });

    assert_eq!(result, "2-4-10-12-14"); // 1*2, 2*2, 5*2, 6*2, 7*2
}

// ============================================================================
// Additional integration scenarios
// ============================================================================

#[test]
fn fold_with_size_and_depth_queries() {
    let pattern = Pattern::pattern(
        1,
        vec![
            Pattern::pattern(2, vec![Pattern::point(3)]),
            Pattern::point(4),
        ],
    );

    // Verify fold count matches size
    let fold_count = pattern.fold(0, |acc, _| acc + 1);
    assert_eq!(fold_count, pattern.size());

    // Verify structure preserved
    assert_eq!(pattern.depth(), 2);
    assert_eq!(pattern.length(), 2);
}

#[test]
fn map_fold_with_custom_types() {
    #[derive(Clone, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = Point { x: 5, y: 6 };

    let pattern = Pattern::pattern(p1, vec![Pattern::point(p2), Pattern::point(p3)]);

    // Map to x coordinates, then fold to sum
    let x_sum = pattern.map(|p| p.x).fold(0, |acc, &x| acc + x);

    assert_eq!(x_sum, 9); // 1 + 3 + 5
}
