//! Tests for Pattern traverse_option and sequence_option methods
//!
//! This file contains tests for:
//! - traverse_option: Apply Option-returning functions to all pattern values
//! - sequence_option: Flip Pattern<Option<T>> to Option<Pattern<T>>
//!
//! Tests verify:
//! - All-or-nothing semantics (any None → None)
//! - Structure preservation
//! - Proper traversal order (root first, then elements)

use pattern_core::Pattern;

// ====================================================================================
// Unit Tests for traverse_option (T016-T019)
// ====================================================================================

/// T016: Atomic pattern with Some - successful transformation
#[test]
fn traverse_option_atomic_some() {
    let pattern = Pattern::point("42");

    // Parse string to integer (should succeed)
    let result = pattern.traverse_option(|s| s.parse::<i32>().ok());

    assert!(result.is_some());
    let new_pattern = result.unwrap();
    assert_eq!(new_pattern.value, 42);
    assert_eq!(new_pattern.elements.len(), 0);
}

/// T017: Atomic pattern with None - transformation fails
#[test]
fn traverse_option_atomic_none() {
    let pattern = Pattern::point("invalid");

    // Parse string to integer (should fail)
    let result = pattern.traverse_option(|s| s.parse::<i32>().ok());

    assert!(result.is_none());
}

/// T018: Nested pattern with all Some - all transformations succeed
#[test]
fn traverse_option_nested_all_some() {
    let pattern = Pattern::pattern("1", vec![Pattern::point("2"), Pattern::point("3")]);

    // Parse all strings to integers (all should succeed)
    let result = pattern.traverse_option(|s| s.parse::<i32>().ok());

    assert!(result.is_some());
    let new_pattern = result.unwrap();
    assert_eq!(new_pattern.value, 1);
    assert_eq!(new_pattern.elements.len(), 2);
    assert_eq!(new_pattern.elements[0].value, 2);
    assert_eq!(new_pattern.elements[1].value, 3);
}

/// T019: Nested pattern with None - any None causes entire traversal to return None
#[test]
fn traverse_option_nested_with_none() {
    let pattern = Pattern::pattern(
        "1",
        vec![
            Pattern::point("2"),
            Pattern::point("invalid"), // This will fail to parse
            Pattern::point("3"),
        ],
    );

    // Parse strings to integers - second element fails
    let result = pattern.traverse_option(|s| s.parse::<i32>().ok());

    // All-or-nothing semantics: any None → None
    assert!(result.is_none());
}

// ====================================================================================
// Unit Tests for sequence_option (T038-T042)
// ====================================================================================

/// T038: Pattern<Option<T>> with all Some → Some(Pattern<T>)
#[test]
fn sequence_option_all_some() {
    let pattern = Pattern::pattern(
        Some(1),
        vec![Pattern::point(Some(2)), Pattern::point(Some(3))],
    );

    let result = pattern.sequence_option();

    assert!(result.is_some());
    let unwrapped = result.unwrap();
    assert_eq!(unwrapped.value, 1);
    assert_eq!(unwrapped.elements[0].value, 2);
    assert_eq!(unwrapped.elements[1].value, 3);
}

/// T039: Pattern<Option<T>> with at least one None → None
#[test]
fn sequence_option_with_none() {
    let pattern = Pattern::pattern(
        Some(1),
        vec![
            Pattern::point(Some(2)),
            Pattern::point(None), // This None should cause entire result to be None
            Pattern::point(Some(3)),
        ],
    );

    let result = pattern.sequence_option();

    // All-or-nothing: any None → None
    assert!(result.is_none());
}

/// T042: Nested pattern structure sequencing
#[test]
fn sequence_option_nested_structure() {
    let pattern = Pattern::pattern(
        Some(1),
        vec![Pattern::pattern(
            Some(2),
            vec![Pattern::point(Some(3)), Pattern::point(Some(4))],
        )],
    );

    let result = pattern.sequence_option();

    assert!(result.is_some());
    let unwrapped = result.unwrap();
    assert_eq!(unwrapped.value, 1);
    assert_eq!(unwrapped.elements[0].value, 2);
    assert_eq!(unwrapped.elements[0].elements[0].value, 3);
    assert_eq!(unwrapped.elements[0].elements[1].value, 4);
}

/// Test that None in root value short-circuits
#[test]
fn sequence_option_none_in_root() {
    let pattern = Pattern::pattern(
        None, // Root is None
        vec![Pattern::point(Some(2)), Pattern::point(Some(3))],
    );

    let result = pattern.sequence_option();

    assert!(result.is_none());
}

/// Test atomic pattern with Some
#[test]
fn sequence_option_atomic_some() {
    let pattern = Pattern::point(Some(42));

    let result = pattern.sequence_option();

    assert!(result.is_some());
    assert_eq!(result.unwrap().value, 42);
}

/// Test atomic pattern with None
#[test]
fn sequence_option_atomic_none() {
    let pattern: Pattern<Option<i32>> = Pattern::point(None);

    let result = pattern.sequence_option();

    assert!(result.is_none());
}
