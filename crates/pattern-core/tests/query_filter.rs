//! Tests for Pattern::filter operation
//!
//! This module tests the filter operation which extracts all subpatterns
//! (including root) that satisfy a given pattern predicate.

use pattern_core::Pattern;

#[test]
fn test_filter_atomic_patterns_only() {
    // T043: filter with predicate matching atomic patterns only
    let pat = Pattern::pattern(
        "root",
        vec![
            Pattern::point("leaf1"),
            Pattern::pattern("branch", vec![Pattern::point("leaf2")]),
            Pattern::point("leaf3"),
        ],
    );

    let atomics = pat.filter(|p| p.is_atomic());

    // Should find leaf1, leaf2, leaf3
    assert_eq!(atomics.len(), 3);
    assert!(atomics.iter().all(|p| p.is_atomic()));
}

#[test]
fn test_filter_root_pattern_matches() {
    // T044: filter with predicate matching root pattern
    let pat = Pattern::pattern("root", vec![Pattern::point("leaf")]);

    let matches = pat.filter(|p| p.value == "root");

    // Should find root pattern
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].value, "root");
}

#[test]
fn test_filter_no_matches() {
    // T045: filter with predicate matching no patterns (empty result)
    let pat = Pattern::pattern(5, vec![Pattern::point(10), Pattern::point(3)]);

    let matches = pat.filter(|p| p.value > 100);

    // Should find no matches
    assert_eq!(matches.len(), 0);
}

#[test]
fn test_filter_all_patterns() {
    // T046: filter with predicate matching all patterns (const true)
    let pat = Pattern::pattern(
        5,
        vec![
            Pattern::point(10),
            Pattern::pattern(3, vec![Pattern::point(15)]),
        ],
    );

    let matches = pat.filter(|_| true);

    // Should find all 4 patterns: root(5), point(10), pattern(3), point(15)
    assert_eq!(matches.len(), 4);
}

#[test]
fn test_filter_complex_structural_predicate() {
    // T047: filter with complex structural predicate (length > 0 && depth < 3)
    let pat = Pattern::pattern(
        "root",
        vec![
            Pattern::point("leaf1"),
            Pattern::pattern("branch", vec![Pattern::point("leaf2")]),
            Pattern::pattern(
                "deep",
                vec![Pattern::pattern("deeper", vec![Pattern::point("leaf3")])],
            ),
        ],
    );

    let matches = pat.filter(|p| p.length() > 0 && p.depth() < 3);

    // Should find: root (length=3, depth=3), branch (length=1, depth=1)
    // Not: deep (depth=3), deeper (length=1, depth=1 but inside deep structure)
    // Actually, let's be clearer: patterns with elements AND not too deep themselves
    assert!(matches.len() >= 2);
    assert!(matches.iter().all(|p| p.length() > 0 && p.depth() < 3));
}

#[test]
fn test_filter_structural_and_value_predicate() {
    // T048: filter with predicates combining structural and value properties
    let pat = Pattern::pattern(
        10,
        vec![
            Pattern::point(5),
            Pattern::pattern(20, vec![Pattern::point(3)]),
            Pattern::pattern(15, vec![]),
        ],
    );

    let matches = pat.filter(|p| p.value > 10 && p.length() > 0);

    // Should find: pattern(20, [point(3)]) - value > 10 and has elements
    // Not: pattern(15, []) - length is 0
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].value, 20);
}

#[test]
fn test_filter_pre_order_traversal() {
    // T049: filter returns results in pre-order traversal order
    let pat = Pattern::pattern(
        1,
        vec![
            Pattern::point(2),
            Pattern::pattern(3, vec![Pattern::point(4)]),
            Pattern::point(5),
        ],
    );

    let all_matches = pat.filter(|_| true);

    // Pre-order: 1, 2, 3, 4, 5
    assert_eq!(all_matches.len(), 5);
    assert_eq!(all_matches[0].value, 1); // root
    assert_eq!(all_matches[1].value, 2); // first element
    assert_eq!(all_matches[2].value, 3); // second element (pattern)
    assert_eq!(all_matches[3].value, 4); // nested in second element
    assert_eq!(all_matches[4].value, 5); // third element
}

#[test]
fn test_filter_deeply_nested() {
    // T050: filter with deeply nested pattern (100+ levels)
    fn create_deep_pattern(depth: usize, value: i32) -> Pattern<i32> {
        if depth == 0 {
            Pattern::point(value)
        } else {
            Pattern::pattern(value, vec![create_deep_pattern(depth - 1, value + 1)])
        }
    }

    let deep_pat = create_deep_pattern(150, 0);

    // Find all patterns with even values
    let even_matches = deep_pat.filter(|p| p.value % 2 == 0);

    // Values are 0, 1, 2, ..., 150
    // Even values: 0, 2, 4, ..., 150 = 76 values
    assert_eq!(even_matches.len(), 76);
    assert!(even_matches.iter().all(|p| p.value % 2 == 0));
}

#[test]
fn test_filter_large_flat_pattern() {
    // T051: filter with large flat pattern (1000+ elements)
    let elements: Vec<Pattern<i32>> = (0..1000).map(|i| Pattern::point(i)).collect();

    let pat = Pattern::pattern(999, elements);

    // Find patterns with values > 500
    let high_value_matches = pat.filter(|p| p.value > 500);

    // Should find: 501, 502, ..., 999 (elements) + 999 (root) if > 500
    // Root value is 999, so it matches
    // Elements 501-999 = 499 elements
    // Total: 1 (root) + 499 (elements) = 500
    assert_eq!(high_value_matches.len(), 500);
    assert!(high_value_matches.iter().all(|p| p.value > 500));
}

#[test]
fn test_filter_returns_references() {
    // Additional test: Verify filter returns references, not clones
    let pat = Pattern::pattern("root", vec![Pattern::point("leaf")]);

    let matches = pat.filter(|_| true);

    // Should be references to patterns in original structure
    assert_eq!(matches.len(), 2);

    // Verify they point to the actual pattern structure
    assert!(std::ptr::eq(matches[0], &pat));
    assert!(std::ptr::eq(matches[1], &pat.elements[0]));
}
