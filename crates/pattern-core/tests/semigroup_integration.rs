//! Integration tests for Pattern combination with iterators
//!
//! These tests verify that pattern combination works correctly with
//! standard iterator methods like fold and reduce.

use pattern_core::{Combinable, Pattern};

// ============================================================================
// T032: Fold/Reduce with 4 Patterns
// ============================================================================

#[test]
fn test_reduce_four_patterns() {
    let patterns = vec![
        Pattern::point("a".to_string()),
        Pattern::point("b".to_string()),
        Pattern::point("c".to_string()),
        Pattern::point("d".to_string()),
    ];

    let result = patterns
        .into_iter()
        .reduce(|acc, p| acc.combine(p))
        .unwrap();

    assert_eq!(result.value(), "abcd");
    assert_eq!(result.length(), 0); // All atomic
}

#[test]
fn test_fold_four_patterns() {
    let patterns = vec![
        Pattern::point("b".to_string()),
        Pattern::point("c".to_string()),
        Pattern::point("d".to_string()),
    ];

    let initial = Pattern::point("a".to_string());
    let result = patterns.into_iter().fold(initial, |acc, p| acc.combine(p));

    assert_eq!(result.value(), "abcd");
}

#[test]
fn test_reduce_patterns_with_elements() {
    let patterns = vec![
        Pattern::pattern("1".to_string(), vec![Pattern::point("a".to_string())]),
        Pattern::pattern("2".to_string(), vec![Pattern::point("b".to_string())]),
        Pattern::pattern("3".to_string(), vec![Pattern::point("c".to_string())]),
        Pattern::pattern("4".to_string(), vec![Pattern::point("d".to_string())]),
    ];

    let result = patterns
        .into_iter()
        .reduce(|acc, p| acc.combine(p))
        .unwrap();

    assert_eq!(result.value(), "1234");
    assert_eq!(result.length(), 4); // [a, b, c, d]

    let values: Vec<_> = result.elements().iter().map(|p| p.value()).collect();
    assert_eq!(values, vec!["a", "b", "c", "d"]);
}

// ============================================================================
// T033: Fold/Reduce with Single Pattern
// ============================================================================

#[test]
fn test_reduce_single_pattern() {
    let patterns = vec![Pattern::point("only".to_string())];

    let result = patterns
        .into_iter()
        .reduce(|acc, p| acc.combine(p))
        .unwrap();

    assert_eq!(result.value(), "only");
}

#[test]
fn test_fold_single_pattern() {
    let patterns = vec![Pattern::point("second".to_string())];

    let initial = Pattern::point("first".to_string());
    let result = patterns.into_iter().fold(initial, |acc, p| acc.combine(p));

    assert_eq!(result.value(), "firstsecond");
}

// ============================================================================
// T034: Fold/Reduce with Varying Structures
// ============================================================================

#[test]
fn test_reduce_mixed_atomic_and_compound() {
    let patterns = vec![
        Pattern::point("a".to_string()),
        Pattern::pattern(
            "b".to_string(),
            vec![
                Pattern::point("x".to_string()),
                Pattern::point("y".to_string()),
            ],
        ),
        Pattern::point("c".to_string()),
        Pattern::pattern("d".to_string(), vec![Pattern::point("z".to_string())]),
    ];

    let result = patterns
        .into_iter()
        .reduce(|acc, p| acc.combine(p))
        .unwrap();

    assert_eq!(result.value(), "abcd");
    assert_eq!(result.length(), 3); // [x, y, z]
}

#[test]
fn test_fold_nested_structures() {
    let patterns = vec![
        Pattern::pattern(
            "level1".to_string(),
            vec![Pattern::pattern(
                "level2a".to_string(),
                vec![Pattern::point("leaf1".to_string())],
            )],
        ),
        Pattern::pattern(
            "level1b".to_string(),
            vec![Pattern::pattern(
                "level2b".to_string(),
                vec![Pattern::point("leaf2".to_string())],
            )],
        ),
    ];

    let initial = Pattern::point("root".to_string());
    let result = patterns.into_iter().fold(initial, |acc, p| acc.combine(p));

    assert_eq!(result.value(), "rootlevel1level1b");
    assert_eq!(result.length(), 2); // Two nested structures
}

// ============================================================================
// T035: Empty Collection Behavior
// ============================================================================

#[test]
fn test_reduce_empty_collection() {
    let patterns: Vec<Pattern<String>> = vec![];

    let result = patterns.into_iter().reduce(|acc, p| acc.combine(p));

    assert!(
        result.is_none(),
        "Reduce on empty collection should return None"
    );
}

#[test]
fn test_fold_empty_collection() {
    let patterns: Vec<Pattern<String>> = vec![];

    let initial = Pattern::point("initial".to_string());
    let result = patterns
        .into_iter()
        .fold(initial.clone(), |acc, p| acc.combine(p));

    assert_eq!(
        result, initial,
        "Fold with empty collection should return initial value"
    );
}

// ============================================================================
// T036: Combining 100 Patterns in Sequence
// ============================================================================

#[test]
fn test_reduce_100_patterns() {
    let patterns: Vec<_> = (0..100).map(|i| Pattern::point(i.to_string())).collect();

    let result = patterns
        .into_iter()
        .reduce(|acc, p| acc.combine(p))
        .unwrap();

    // Verify the value is the concatenation of all numbers
    let expected: String = (0..100).map(|i| i.to_string()).collect();
    assert_eq!(result.value(), &expected);
    assert_eq!(result.length(), 0); // All atomic
}

#[test]
fn test_fold_100_patterns_with_elements() {
    let patterns: Vec<_> = (0..100)
        .map(|i| Pattern::pattern(format!("p{}", i), vec![Pattern::point(format!("e{}", i))]))
        .collect();

    let initial = Pattern::point("start".to_string());
    let result = patterns.into_iter().fold(initial, |acc, p| acc.combine(p));

    // Verify value concatenation
    assert!(result.value().starts_with("start"));
    assert!(result.value().ends_with("p99"));

    // Verify all 100 elements are present
    assert_eq!(result.length(), 100);

    // Verify element order
    assert_eq!(result.elements()[0].value(), "e0");
    assert_eq!(result.elements()[99].value(), "e99");
}

#[test]
fn test_performance_large_fold() {
    // Test that folding many patterns completes efficiently
    let patterns: Vec<_> = (0..1000)
        .map(|i| Pattern::point(format!("{}", i)))
        .collect();

    let start = std::time::Instant::now();
    let result = patterns
        .into_iter()
        .reduce(|acc, p| acc.combine(p))
        .unwrap();
    let duration = start.elapsed();

    // Verify correctness
    assert!(result.value().starts_with("0"));
    assert!(result.value().ends_with("999"));

    // Verify reasonable performance (should be much less than 100ms)
    assert!(
        duration.as_millis() < 100,
        "Folding 1000 patterns took {:?}, expected <100ms",
        duration
    );
}

// ============================================================================
// Additional Integration Tests
// ============================================================================

#[test]
fn test_chained_operations() {
    // Test combining with other pattern operations
    let patterns = vec![
        Pattern::point("hello".to_string()),
        Pattern::point(" ".to_string()),
        Pattern::point("world".to_string()),
    ];

    let combined = patterns
        .into_iter()
        .reduce(|acc, p| acc.combine(p))
        .unwrap();

    // Now map over the combined result
    let upper = combined.map(|s| s.to_uppercase());
    assert_eq!(upper.value(), "HELLO WORLD");
}

#[test]
fn test_filter_then_combine() {
    let patterns = vec![
        Pattern::point("keep".to_string()),
        Pattern::point("".to_string()),
        Pattern::point("this".to_string()),
        Pattern::point("".to_string()),
        Pattern::point("text".to_string()),
    ];

    let result = patterns
        .into_iter()
        .filter(|p| !p.value().is_empty())
        .reduce(|acc, p| acc.combine(p))
        .unwrap();

    assert_eq!(result.value(), "keepthistext");
}

#[test]
fn test_collect_and_combine() {
    // Create patterns, collect them, then combine
    let result = (0..10)
        .map(|i| Pattern::point(i.to_string()))
        .collect::<Vec<_>>()
        .into_iter()
        .reduce(|acc, p| acc.combine(p))
        .unwrap();

    let expected: String = (0..10).map(|i| i.to_string()).collect();
    assert_eq!(result.value(), &expected);
}
