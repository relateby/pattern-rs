//! Basic combination tests for Pattern<V> where V: Combinable
//!
//! These tests verify the core pattern combination operation that merges two patterns
//! by combining their values and concatenating their elements.

use pattern_core::{Combinable, Pattern};

// ============================================================================
// T010: Combining Atomic Patterns (No Elements)
// ============================================================================

#[test]
fn test_combine_atomic_patterns() {
    let p1 = Pattern::point("hello".to_string());
    let p2 = Pattern::point(" world".to_string());

    let result = p1.combine(p2);

    assert_eq!(result.value(), "hello world");
    assert_eq!(result.length(), 0); // No elements
    assert!(result.is_atomic());
}

#[test]
fn test_combine_atomic_patterns_empty_strings() {
    let p1 = Pattern::point("".to_string());
    let p2 = Pattern::point("test".to_string());

    let result = p1.combine(p2);

    assert_eq!(result.value(), "test");
    assert!(result.is_atomic());
}

// ============================================================================
// T011: Combining Patterns with Elements
// ============================================================================

#[test]
fn test_combine_patterns_with_elements() {
    let p1 = Pattern::pattern(
        "a".to_string(),
        vec![
            Pattern::point("b".to_string()),
            Pattern::point("c".to_string()),
        ],
    );

    let p2 = Pattern::pattern("d".to_string(), vec![Pattern::point("e".to_string())]);

    let result = p1.combine(p2);

    assert_eq!(result.value(), "ad");
    assert_eq!(result.length(), 3); // [b, c, e]

    // Verify element order: left elements first, then right elements
    let values: Vec<_> = result.elements().iter().map(|p| p.value()).collect();
    assert_eq!(values, vec!["b", "c", "e"]);
}

#[test]
fn test_combine_patterns_multiple_elements() {
    let p1 = Pattern::pattern(
        "x".to_string(),
        vec![
            Pattern::point("1".to_string()),
            Pattern::point("2".to_string()),
            Pattern::point("3".to_string()),
        ],
    );

    let p2 = Pattern::pattern(
        "y".to_string(),
        vec![
            Pattern::point("4".to_string()),
            Pattern::point("5".to_string()),
        ],
    );

    let result = p1.combine(p2);

    assert_eq!(result.value(), "xy");
    assert_eq!(result.length(), 5); // [1, 2, 3, 4, 5]
}

// ============================================================================
// T012: Combining Mixed Structures
// ============================================================================

#[test]
fn test_combine_atomic_with_pattern() {
    let p1 = Pattern::point("atomic".to_string());
    let p2 = Pattern::pattern(
        "parent".to_string(),
        vec![Pattern::point("child".to_string())],
    );

    let result = p1.combine(p2);

    assert_eq!(result.value(), "atomicparent");
    assert_eq!(result.length(), 1); // [child]
}

#[test]
fn test_combine_pattern_with_atomic() {
    let p1 = Pattern::pattern(
        "parent".to_string(),
        vec![Pattern::point("child".to_string())],
    );
    let p2 = Pattern::point("atomic".to_string());

    let result = p1.combine(p2);

    assert_eq!(result.value(), "parentatomic");
    assert_eq!(result.length(), 1); // [child]
}

// ============================================================================
// T013: Self-Combination
// ============================================================================

#[test]
fn test_combine_pattern_with_itself() {
    let p = Pattern::pattern(
        "self".to_string(),
        vec![
            Pattern::point("a".to_string()),
            Pattern::point("b".to_string()),
        ],
    );

    let result = p.clone().combine(p.clone());

    assert_eq!(result.value(), "selfself");
    assert_eq!(result.length(), 4); // [a, b, a, b]

    // Verify the pattern is well-formed
    assert!(result.elements().len() == 4);
}

// ============================================================================
// T014: Deep Nesting Preservation
// ============================================================================

#[test]
fn test_combine_deep_nesting() {
    // Create a deeply nested pattern (100+ levels)
    fn create_deep_pattern(depth: usize, prefix: &str) -> Pattern<String> {
        if depth == 0 {
            Pattern::point(format!("{}leaf", prefix))
        } else {
            Pattern::pattern(
                format!("{}level{}", prefix, depth),
                vec![create_deep_pattern(depth - 1, prefix)],
            )
        }
    }

    let p1 = create_deep_pattern(100, "a");
    let p2 = create_deep_pattern(100, "b");

    let result = p1.combine(p2);

    // Verify combination succeeded without stack overflow
    assert_eq!(result.length(), 2); // Two deep trees
    assert_eq!(result.depth(), 100); // Max depth of the two element trees

    // Verify values combined
    assert!(result.value().starts_with("alevel100"));
    assert!(result.value().ends_with("blevel100"));
}

// ============================================================================
// T015: Wide Patterns (1000+ Elements)
// ============================================================================

#[test]
fn test_combine_wide_patterns() {
    // Create patterns with many elements
    let elements1: Vec<_> = (0..1000)
        .map(|i| Pattern::point(format!("a{}", i)))
        .collect();

    let elements2: Vec<_> = (0..1000)
        .map(|i| Pattern::point(format!("b{}", i)))
        .collect();

    let p1 = Pattern::pattern("left".to_string(), elements1);
    let p2 = Pattern::pattern("right".to_string(), elements2);

    let result = p1.combine(p2);

    // Verify combination succeeded
    assert_eq!(result.value(), "leftright");
    assert_eq!(result.length(), 2000); // All elements preserved

    // Verify element order: left elements first
    assert_eq!(result.elements()[0].value(), "a0");
    assert_eq!(result.elements()[999].value(), "a999");
    assert_eq!(result.elements()[1000].value(), "b0");
    assert_eq!(result.elements()[1999].value(), "b999");
}

#[test]
fn test_combine_very_wide_patterns() {
    // Test with even more elements to verify performance
    let elements1: Vec<_> = (0..5000).map(|i| Pattern::point(i.to_string())).collect();

    let elements2: Vec<_> = (5000..10000)
        .map(|i| Pattern::point(i.to_string()))
        .collect();

    let p1 = Pattern::pattern("x".to_string(), elements1);
    let p2 = Pattern::pattern("y".to_string(), elements2);

    let result = p1.combine(p2);

    assert_eq!(result.value(), "xy");
    assert_eq!(result.length(), 10000);
}

// ============================================================================
// Additional Edge Cases
// ============================================================================

#[test]
fn test_combine_empty_elements() {
    let p1 = Pattern::pattern("a".to_string(), vec![]);
    let p2 = Pattern::pattern("b".to_string(), vec![]);

    let result = p1.combine(p2);

    assert_eq!(result.value(), "ab");
    assert_eq!(result.length(), 0);
}

#[test]
fn test_combine_nested_structures() {
    let p1 = Pattern::pattern(
        "root1".to_string(),
        vec![Pattern::pattern(
            "child1".to_string(),
            vec![Pattern::point("leaf1".to_string())],
        )],
    );

    let p2 = Pattern::pattern(
        "root2".to_string(),
        vec![Pattern::pattern(
            "child2".to_string(),
            vec![Pattern::point("leaf2".to_string())],
        )],
    );

    let result = p1.combine(p2);

    assert_eq!(result.value(), "root1root2");
    assert_eq!(result.length(), 2); // Two nested structures

    // Verify nested structures are preserved
    assert_eq!(result.elements()[0].value(), "child1");
    assert_eq!(result.elements()[1].value(), "child2");
}
