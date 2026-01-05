//! Unit tests for Pattern Default trait implementation
//!
//! These tests verify that the Default trait is correctly implemented for patterns
//! and that the default pattern acts as an identity element for combination operations.
//!
//! # Test Organization
//!
//! - **Default Creation Tests**: Verify default patterns can be created for various value types
//! - **Identity Law Tests**: Verify left and right identity laws hold for sample patterns
//! - **Structure Tests**: Verify default patterns have the expected structure

use pattern_core::{Combinable, Pattern};

// ============================================================================
// T011: Default Creation for String Patterns
// ============================================================================

#[test]
fn test_default_string_pattern() {
    let empty: Pattern<String> = Pattern::default();

    // Default pattern has empty string value
    assert_eq!(empty.value(), "");

    // Default pattern has no elements
    assert_eq!(empty.length(), 0);
    assert!(empty.is_atomic());

    // Default pattern is well-formed
    assert_eq!(empty.elements().len(), 0);
}

#[test]
fn test_default_string_structure() {
    let empty: Pattern<String> = Pattern::default();

    // Verify structure matches Pattern::point(String::default())
    let expected = Pattern::point(String::new());
    assert_eq!(empty, expected);
}

// ============================================================================
// T012: Default Creation for Vec Patterns
// ============================================================================

#[test]
fn test_default_vec_pattern() {
    let empty: Pattern<Vec<i32>> = Pattern::default();

    // Default pattern has empty vector value
    let expected: Vec<i32> = vec![];
    assert_eq!(empty.value(), &expected);

    // Default pattern has no elements
    assert_eq!(empty.length(), 0);
    assert!(empty.is_atomic());
}

#[test]
fn test_default_vec_structure() {
    let empty: Pattern<Vec<i32>> = Pattern::default();

    // Verify structure matches Pattern::point(Vec::default())
    let expected = Pattern::point(Vec::<i32>::new());
    assert_eq!(empty, expected);
}

#[test]
fn test_default_vec_string_pattern() {
    let empty: Pattern<Vec<String>> = Pattern::default();

    // Default pattern has empty vector of strings
    assert_eq!(empty.value(), &Vec::<String>::new());
    assert_eq!(empty.length(), 0);
    assert!(empty.is_atomic());
}

// ============================================================================
// T013: Default Creation for Unit Patterns
// ============================================================================

#[test]
fn test_default_unit_pattern() {
    let empty: Pattern<()> = Pattern::default();

    // Default pattern has unit value
    assert_eq!(empty.value(), &());

    // Default pattern has no elements
    assert_eq!(empty.length(), 0);
    assert!(empty.is_atomic());
}

#[test]
fn test_default_unit_structure() {
    let empty: Pattern<()> = Pattern::default();

    // Verify structure matches Pattern::point(())
    let expected = Pattern::point(());
    assert_eq!(empty, expected);
}

// ============================================================================
// T014: Default Creation for Integer Patterns
// ============================================================================

#[test]
fn test_default_i32_pattern() {
    let empty: Pattern<i32> = Pattern::default();

    // Default pattern has zero value
    assert_eq!(empty.value(), &0);

    // Default pattern has no elements
    assert_eq!(empty.length(), 0);
    assert!(empty.is_atomic());
}

#[test]
fn test_default_i32_structure() {
    let empty: Pattern<i32> = Pattern::default();

    // Verify structure matches Pattern::point(0)
    let expected = Pattern::point(0);
    assert_eq!(empty, expected);
}

#[test]
fn test_default_u64_pattern() {
    let empty: Pattern<u64> = Pattern::default();

    // Default pattern has zero value
    assert_eq!(empty.value(), &0u64);
    assert_eq!(empty.length(), 0);
}

#[test]
fn test_default_i64_pattern() {
    let empty: Pattern<i64> = Pattern::default();

    // Default pattern has zero value
    assert_eq!(empty.value(), &0i64);
    assert_eq!(empty.length(), 0);
}

// ============================================================================
// T015: Left Identity with Atomic String Pattern
// ============================================================================

#[test]
fn test_left_identity_atomic_string() {
    let empty = Pattern::<String>::default();
    let p = Pattern::point("hello".to_string());

    // Left identity: empty.combine(p) == p
    let result = empty.combine(p.clone());
    assert_eq!(result, p);
}

#[test]
fn test_left_identity_various_strings() {
    let empty = Pattern::<String>::default();

    // Test with different string values
    for s in &["test", "hello world", "123", ""] {
        let p = Pattern::point(s.to_string());
        let result = empty.clone().combine(p.clone());
        assert_eq!(result, p, "Left identity failed for string: {}", s);
    }
}

// ============================================================================
// T016: Right Identity with Atomic String Pattern
// ============================================================================

#[test]
fn test_right_identity_atomic_string() {
    let empty = Pattern::<String>::default();
    let p = Pattern::point("world".to_string());

    // Right identity: p.combine(empty) == p
    let result = p.clone().combine(empty);
    assert_eq!(result, p);
}

#[test]
fn test_right_identity_various_strings() {
    let empty = Pattern::<String>::default();

    // Test with different string values
    for s in &["test", "hello world", "123", ""] {
        let p = Pattern::point(s.to_string());
        let result = p.clone().combine(empty.clone());
        assert_eq!(result, p, "Right identity failed for string: {}", s);
    }
}

// ============================================================================
// T017: Identity with Compound Pattern (Has Elements)
// ============================================================================

#[test]
fn test_left_identity_compound_pattern() {
    let empty = Pattern::<String>::default();
    let p = Pattern::pattern(
        "parent".to_string(),
        vec![
            Pattern::point("child1".to_string()),
            Pattern::point("child2".to_string()),
        ],
    );

    // Left identity: empty.combine(p) == p
    let result = empty.combine(p.clone());
    assert_eq!(result, p);

    // Verify structure is preserved
    assert_eq!(result.value(), "parent");
    assert_eq!(result.length(), 2);
    assert_eq!(result.elements()[0].value(), "child1");
    assert_eq!(result.elements()[1].value(), "child2");
}

#[test]
fn test_right_identity_compound_pattern() {
    let empty = Pattern::<String>::default();
    let p = Pattern::pattern(
        "parent".to_string(),
        vec![
            Pattern::point("child1".to_string()),
            Pattern::point("child2".to_string()),
        ],
    );

    // Right identity: p.combine(empty) == p
    let result = p.clone().combine(empty);
    assert_eq!(result, p);

    // Verify structure is preserved
    assert_eq!(result.value(), "parent");
    assert_eq!(result.length(), 2);
}

#[test]
fn test_identity_nested_pattern() {
    let empty = Pattern::<String>::default();
    let p = Pattern::pattern(
        "root".to_string(),
        vec![Pattern::pattern(
            "level1".to_string(),
            vec![Pattern::point("leaf".to_string())],
        )],
    );

    // Both identity laws hold
    assert_eq!(empty.clone().combine(p.clone()), p);
    assert_eq!(p.clone().combine(empty), p);
}

// ============================================================================
// Additional Identity Tests
// ============================================================================

#[test]
fn test_default_combine_itself() {
    let empty1 = Pattern::<String>::default();
    let empty2 = Pattern::<String>::default();

    // Combining two defaults yields a default
    let result = empty1.combine(empty2.clone());
    assert_eq!(result, empty2);
    assert_eq!(result.value(), "");
    assert_eq!(result.length(), 0);
}

#[test]
fn test_identity_with_vec_patterns() {
    let empty = Pattern::<Vec<i32>>::default();
    let p = Pattern::point(vec![1, 2, 3]);

    // Both identity laws hold for Vec
    assert_eq!(empty.clone().combine(p.clone()), p);
    assert_eq!(p.clone().combine(empty), p);
}

#[test]
fn test_identity_with_unit_patterns() {
    let empty = Pattern::<()>::default();
    let p = Pattern::point(());

    // Both identity laws hold for unit (trivial case)
    assert_eq!(empty.clone().combine(p.clone()), p);
    assert_eq!(p.clone().combine(empty), p);
}

#[test]
fn test_identity_with_multiple_elements() {
    let empty = Pattern::<String>::default();
    let p = Pattern::pattern(
        "root".to_string(),
        vec![
            Pattern::point("a".to_string()),
            Pattern::point("b".to_string()),
            Pattern::point("c".to_string()),
            Pattern::point("d".to_string()),
        ],
    );

    // Identity laws hold with many elements
    assert_eq!(empty.clone().combine(p.clone()), p);
    assert_eq!(p.clone().combine(empty), p);
}

// ============================================================================
// Structure and Values Tests
// ============================================================================

#[test]
fn test_default_values_method() {
    let empty: Pattern<String> = Pattern::default();

    // Default pattern has single value (the default value)
    let values = empty.values();
    assert_eq!(values.len(), 1);
    assert_eq!(values[0], "");
}

#[test]
fn test_default_size() {
    let empty: Pattern<String> = Pattern::default();

    // Default pattern size is 1 (just the root node)
    assert_eq!(empty.size(), 1);
}

#[test]
fn test_default_depth() {
    let empty: Pattern<String> = Pattern::default();

    // Default pattern depth is 0 (atomic pattern)
    assert_eq!(empty.depth(), 0);
}
