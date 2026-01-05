//! Integration tests for Pattern Default trait with iterators and standard library
//!
//! These tests verify that the default pattern integrates naturally with
//! Rust's iterator methods (fold, reduce) and standard library functions.
//!
//! # Test Organization
//!
//! - **Fold Tests**: Using Pattern::default() as initial value
//! - **Empty Collection Tests**: Handling empty iterators
//! - **Standard Library Tests**: mem::take, unwrap_or_default
//! - **Existing Operation Tests**: Integration with map, values, etc.

use pattern_core::{Combinable, Pattern};

// ============================================================================
// T035: Fold with Default Initial Value
// ============================================================================

#[test]
fn test_fold_multiple_patterns() {
    let patterns = vec![
        Pattern::point("hello".to_string()),
        Pattern::point(" ".to_string()),
        Pattern::point("world".to_string()),
    ];

    let result = patterns
        .into_iter()
        .fold(Pattern::default(), |acc, p| acc.combine(p));

    assert_eq!(result.value(), "hello world");
    assert_eq!(result.length(), 0); // All atomic
}

#[test]
fn test_fold_patterns_with_elements() {
    let patterns = vec![
        Pattern::pattern("a".to_string(), vec![Pattern::point("1".to_string())]),
        Pattern::pattern("b".to_string(), vec![Pattern::point("2".to_string())]),
        Pattern::pattern("c".to_string(), vec![Pattern::point("3".to_string())]),
    ];

    let result = patterns
        .into_iter()
        .fold(Pattern::default(), |acc, p| acc.combine(p));

    assert_eq!(result.value(), "abc");
    assert_eq!(result.length(), 3); // Three children: 1, 2, 3
}

#[test]
fn test_fold_mixed_structures() {
    let patterns = vec![
        Pattern::point("atomic".to_string()),
        Pattern::pattern(
            "compound".to_string(),
            vec![
                Pattern::point("x".to_string()),
                Pattern::point("y".to_string()),
            ],
        ),
        Pattern::point("atomic2".to_string()),
    ];

    let result = patterns
        .into_iter()
        .fold(Pattern::default(), |acc, p| acc.combine(p));

    assert_eq!(result.value(), "atomiccompoundatomic2");
    assert_eq!(result.length(), 2); // Two children: x, y
}

#[test]
fn test_fold_large_collection() {
    // Test folding many patterns
    let patterns: Vec<_> = (0..100).map(|i| Pattern::point(i.to_string())).collect();

    let result = patterns
        .into_iter()
        .fold(Pattern::default(), |acc, p| acc.combine(p));

    // Verify all numbers are concatenated
    assert!(result.value().starts_with("01234"));
    assert!(result.value().ends_with("99"));
    assert_eq!(result.length(), 0); // All atomic
}

// ============================================================================
// T036: Fold with Empty Collection Returns Default
// ============================================================================

#[test]
fn test_fold_empty_collection() {
    let patterns: Vec<Pattern<String>> = vec![];

    let result = patterns
        .into_iter()
        .fold(Pattern::default(), |acc, p| acc.combine(p));

    // Folding empty collection returns the initial value (default)
    assert_eq!(result, Pattern::default());
    assert_eq!(result.value(), "");
    assert_eq!(result.length(), 0);
}

#[test]
fn test_fold_empty_vec_pattern() {
    let patterns: Vec<Pattern<Vec<i32>>> = vec![];

    let result = patterns
        .into_iter()
        .fold(Pattern::default(), |acc, p| acc.combine(p));

    assert_eq!(result, Pattern::default());
    let expected: Vec<i32> = vec![];
    assert_eq!(result.value(), &expected);
}

#[test]
fn test_fold_empty_unit_pattern() {
    let patterns: Vec<Pattern<()>> = vec![];

    let result = patterns
        .into_iter()
        .fold(Pattern::default(), |acc, p| acc.combine(p));

    assert_eq!(result, Pattern::default());
    assert_eq!(result.value(), &());
}

// ============================================================================
// T037: Fold with Single Pattern Returns That Pattern
// ============================================================================

#[test]
fn test_fold_single_pattern() {
    let pattern = Pattern::point("only".to_string());
    let patterns = vec![pattern.clone()];

    let result = patterns
        .into_iter()
        .fold(Pattern::default(), |acc, p| acc.combine(p));

    // Folding single pattern with default returns that pattern (identity)
    assert_eq!(result, pattern);
    assert_eq!(result.value(), "only");
}

#[test]
fn test_fold_single_compound_pattern() {
    let pattern = Pattern::pattern(
        "root".to_string(),
        vec![
            Pattern::point("a".to_string()),
            Pattern::point("b".to_string()),
        ],
    );
    let patterns = vec![pattern.clone()];

    let result = patterns
        .into_iter()
        .fold(Pattern::default(), |acc, p| acc.combine(p));

    assert_eq!(result, pattern);
    assert_eq!(result.length(), 2);
}

// ============================================================================
// T038: Reduce with unwrap_or_default Pattern
// ============================================================================

#[test]
fn test_reduce_with_unwrap_or_default() {
    let patterns = vec![
        Pattern::point("a".to_string()),
        Pattern::point("b".to_string()),
        Pattern::point("c".to_string()),
    ];

    let result = patterns
        .into_iter()
        .reduce(|acc, p| acc.combine(p))
        .unwrap_or_default();

    assert_eq!(result.value(), "abc");
}

#[test]
fn test_reduce_empty_with_unwrap_or_default() {
    let patterns: Vec<Pattern<String>> = vec![];

    let result = patterns
        .into_iter()
        .reduce(|acc, p| acc.combine(p))
        .unwrap_or_default();

    // Empty reduce with unwrap_or_default gives default pattern
    assert_eq!(result, Pattern::default());
    assert_eq!(result.value(), "");
}

#[test]
fn test_reduce_single_with_unwrap_or_default() {
    let pattern = Pattern::point("single".to_string());
    let patterns = vec![pattern.clone()];

    let result = patterns
        .into_iter()
        .reduce(|acc, p| acc.combine(p))
        .unwrap_or_default();

    // Single element reduce returns that element
    assert_eq!(result, pattern);
}

// ============================================================================
// T039: mem::take with Pattern Uses Default
// ============================================================================

#[test]
fn test_mem_take_uses_default() {
    use std::mem;

    let mut pattern = Pattern::point("original".to_string());

    // mem::take replaces with default and returns original
    let taken = mem::take(&mut pattern);

    assert_eq!(taken.value(), "original");
    assert_eq!(pattern, Pattern::default());
    assert_eq!(pattern.value(), "");
}

#[test]
fn test_mem_take_compound_pattern() {
    use std::mem;

    let mut pattern = Pattern::pattern(
        "parent".to_string(),
        vec![Pattern::point("child".to_string())],
    );

    let taken = mem::take(&mut pattern);

    assert_eq!(taken.value(), "parent");
    assert_eq!(taken.length(), 1);
    assert_eq!(pattern, Pattern::default());
    assert_eq!(pattern.length(), 0);
}

// ============================================================================
// T040: Incremental Accumulation Starting from Default
// ============================================================================

#[test]
fn test_incremental_accumulation() {
    let mut accumulator = Pattern::<String>::default();

    // Start from default and incrementally add patterns
    let items = vec!["first", "second", "third"];

    for item in items {
        let p = Pattern::point(item.to_string());
        accumulator = accumulator.combine(p);
    }

    assert_eq!(accumulator.value(), "firstsecondthird");
}

#[test]
fn test_incremental_with_compound_patterns() {
    let mut accumulator = Pattern::<String>::default();

    // Incrementally build complex structure
    for i in 0..5 {
        let p = Pattern::pattern(
            format!("val{}", i),
            vec![Pattern::point(format!("elem{}", i))],
        );
        accumulator = accumulator.combine(p);
    }

    assert_eq!(accumulator.value(), "val0val1val2val3val4");
    assert_eq!(accumulator.length(), 5); // Five children
}

// ============================================================================
// T041: map() Over Default Pattern Preserves Identity
// ============================================================================

#[test]
fn test_map_over_default() {
    let empty = Pattern::<String>::default();

    // Mapping over default pattern preserves identity
    let mapped = empty.map(|s| s.to_uppercase());

    assert_eq!(mapped, Pattern::default());
    assert_eq!(mapped.value(), "");
}

#[test]
fn test_map_over_default_then_combine() {
    let empty = Pattern::<String>::default();
    let p = Pattern::point("test".to_string());

    // Mapping over default doesn't break identity
    let mapped_empty = empty.map(|s| s.to_uppercase());
    let result = mapped_empty.combine(p.clone());

    assert_eq!(result, p);
}

// ============================================================================
// T042: values() on Default Pattern Returns Single Default Value
// ============================================================================

#[test]
fn test_values_on_default() {
    let empty: Pattern<String> = Pattern::default();

    // Default pattern has single value (the default value)
    let values = empty.values();

    assert_eq!(values.len(), 1);
    assert_eq!(values[0], "");
}

#[test]
fn test_values_on_default_vec() {
    let empty: Pattern<Vec<i32>> = Pattern::default();

    let values = empty.values();

    assert_eq!(values.len(), 1);
    let expected = Vec::<i32>::new();
    assert_eq!(values[0], &expected);
}

// ============================================================================
// Additional Integration Tests
// ============================================================================

#[test]
fn test_filter_then_fold() {
    let patterns = vec![
        Pattern::point("keep".to_string()),
        Pattern::point("".to_string()), // Empty string
        Pattern::point("this".to_string()),
        Pattern::point("".to_string()),
        Pattern::point("text".to_string()),
    ];

    // Filter out empty values, then fold
    let result = patterns
        .into_iter()
        .filter(|p| !p.value().is_empty())
        .fold(Pattern::default(), |acc, p| acc.combine(p));

    assert_eq!(result.value(), "keepthistext");
}

#[test]
fn test_chain_iterators_with_fold() {
    let batch1 = vec![
        Pattern::point("a".to_string()),
        Pattern::point("b".to_string()),
    ];
    let batch2 = vec![
        Pattern::point("c".to_string()),
        Pattern::point("d".to_string()),
    ];

    // Chain two iterators and fold
    let result = batch1
        .into_iter()
        .chain(batch2.into_iter())
        .fold(Pattern::default(), |acc, p| acc.combine(p));

    assert_eq!(result.value(), "abcd");
}

#[test]
fn test_collect_then_fold() {
    // Generate patterns, collect them, then fold
    let result = (0..10)
        .map(|i| Pattern::point(i.to_string()))
        .collect::<Vec<_>>()
        .into_iter()
        .fold(Pattern::default(), |acc, p| acc.combine(p));

    let expected: String = (0..10).map(|i| i.to_string()).collect();
    assert_eq!(result.value(), &expected);
}

#[test]
fn test_default_with_iterator_combinators() {
    let patterns = vec![
        Pattern::point("1".to_string()),
        Pattern::point("2".to_string()),
        Pattern::point("3".to_string()),
    ];

    // Use various iterator combinators with default
    let result = patterns
        .into_iter()
        .map(|p| p) // Identity map
        .filter(|p| !p.value().is_empty())
        .fold(Pattern::default(), |acc, p| acc.combine(p));

    assert_eq!(result.value(), "123");
}
