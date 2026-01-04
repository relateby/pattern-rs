//! Integration tests for traversable with map and fold
//!
//! This file contains integration tests verifying that:
//! - traverse_option composes cleanly with map (Functor)
//! - traverse_result composes cleanly with map (Functor)
//! - traverse operations compose cleanly with fold (Foldable)
//! - Complex pipelines work correctly (map → traverse → fold)
//!
//! Tests ensure traversable integrates seamlessly with existing Pattern operations

use pattern_core::Pattern;

// ====================================================================================
// Functor Composition Tests: map → traverse
// ====================================================================================

/// Test that map (Functor) composes with traverse_option
#[test]
fn map_then_traverse_option() {
    let pattern = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);

    // First map to double values, then traverse to parse as strings
    let result = pattern
        .map(|v| format!("{}", v * 2))
        .traverse_option(|s| s.parse::<i32>().ok());

    assert!(result.is_some());
    let unwrapped = result.unwrap();
    assert_eq!(unwrapped.value, 2);
    assert_eq!(unwrapped.elements[0].value, 4);
    assert_eq!(unwrapped.elements[1].value, 6);
}

/// Test that map (Functor) composes with traverse_result
#[test]
fn map_then_traverse_result() {
    let pattern = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);

    // First map to strings, then traverse to parse back to integers
    let result: Result<Pattern<i32>, String> = pattern
        .map(|v| format!("{}", v * 10))
        .traverse_result(|s| s.parse::<i32>().map_err(|e| format!("parse error: {}", e)));

    assert!(result.is_ok());
    let unwrapped = result.unwrap();
    assert_eq!(unwrapped.value, 10);
    assert_eq!(unwrapped.elements[0].value, 20);
    assert_eq!(unwrapped.elements[1].value, 30);
}

/// Test traverse → map composition
#[test]
fn traverse_then_map() {
    let pattern = Pattern::pattern("1", vec![Pattern::point("2"), Pattern::point("3")]);

    // First traverse to parse, then map to multiply
    let result = pattern
        .traverse_option(|s| s.parse::<i32>().ok())
        .map(|p| p.map(|v| v * 100));

    assert!(result.is_some());
    let unwrapped = result.unwrap();
    assert_eq!(unwrapped.value, 100);
    assert_eq!(unwrapped.elements[0].value, 200);
    assert_eq!(unwrapped.elements[1].value, 300);
}

// ====================================================================================
// Foldable Composition Tests: traverse → fold
// ====================================================================================

/// Test that traverse_option composes with fold
#[test]
fn traverse_option_then_fold() {
    let pattern = Pattern::pattern("1", vec![Pattern::point("2"), Pattern::point("3")]);

    // Traverse to parse, then fold to sum
    let result = pattern
        .traverse_option(|s| s.parse::<i32>().ok())
        .map(|p| p.fold(0, |acc, v| acc + v));

    assert_eq!(result, Some(6)); // 1 + 2 + 3
}

/// Test that traverse_result composes with fold
#[test]
fn traverse_result_then_fold() {
    let pattern = Pattern::pattern("1", vec![Pattern::point("2"), Pattern::point("3")]);

    // Traverse to parse, then fold to sum
    let result: Result<i32, String> = pattern
        .traverse_result(|s| s.parse::<i32>().map_err(|e| format!("error: {}", e)))
        .map(|p| p.fold(0, |acc, v| acc + v));

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 6); // 1 + 2 + 3
}

/// Test that traverse_result with error short-circuits before fold
#[test]
fn traverse_error_prevents_fold() {
    let pattern = Pattern::pattern("1", vec![Pattern::point("invalid"), Pattern::point("3")]);

    // Traverse fails, so fold never executes
    let result: Result<i32, String> = pattern
        .traverse_result(|s| s.parse::<i32>().map_err(|e| format!("error: {}", e)))
        .map(|p| p.fold(0, |acc, v| acc + v));

    assert!(result.is_err());
}

// ====================================================================================
// Complex Pipeline Tests: map → traverse → fold
// ====================================================================================

/// Test full pipeline: map → traverse_option → fold
#[test]
fn full_pipeline_map_traverse_fold() {
    let pattern = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);

    // Complex pipeline:
    // 1. Map to strings
    // 2. Traverse to parse with validation
    // 3. Fold to compute product
    let result = pattern
        .map(|v| format!("{}", v * 10)) // "10", "20", "30"
        .traverse_option(|s| {
            s.parse::<i32>().ok().filter(|&n| n > 0) // Parse and validate
        })
        .map(|p| p.fold(1, |acc, v| acc * v)); // Product

    assert_eq!(result, Some(6000)); // 10 * 20 * 30
}

/// Test full pipeline with validation error
#[test]
fn full_pipeline_with_validation_error() {
    let pattern = Pattern::pattern(-1, vec![Pattern::point(2), Pattern::point(3)]);

    // Pipeline with validation that fails
    let result = pattern
        .map(|v| v * 10)
        .traverse_result(|&v| {
            if v > 0 {
                Ok(v)
            } else {
                Err(format!("negative value: {}", v))
            }
        })
        .map(|p| p.fold(0, |acc, v| acc + v));

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "negative value: -10");
}

// ====================================================================================
// Validate Composition Tests
// ====================================================================================

/// Test that validate_all composes with map
#[test]
fn map_then_validate_all() {
    let pattern = Pattern::pattern(1, vec![Pattern::point(-2), Pattern::point(3)]);

    // Map then validate - validate should collect all errors
    let result: Result<Pattern<i32>, Vec<String>> = pattern.map(|v| v * 10).validate_all(|&v| {
        if v > 0 {
            Ok(v)
        } else {
            Err(format!("negative: {}", v))
        }
    });

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("negative: -20"));
}

/// Test validate_all then fold
#[test]
fn validate_all_then_fold() {
    let pattern = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);

    // Validate all values positive, then sum
    let result: Result<i32, Vec<String>> = pattern
        .validate_all(|&v| {
            if v > 0 {
                Ok(v * 10)
            } else {
                Err(format!("negative: {}", v))
            }
        })
        .map(|p| p.fold(0, |acc, v| acc + v));

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 60); // 10 + 20 + 30
}

// ====================================================================================
// Sequence Composition Tests
// ====================================================================================

/// Test sequence_option with map
#[test]
fn map_to_option_then_sequence() {
    let pattern = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);

    // Map to Option values, then sequence
    let result = pattern
        .map(|&v| if v > 0 { Some(v * 10) } else { None })
        .sequence_option();

    assert!(result.is_some());
    let unwrapped = result.unwrap();
    assert_eq!(unwrapped.value, 10);
    assert_eq!(unwrapped.elements[0].value, 20);
}

/// Test sequence_result with fold
#[test]
fn sequence_result_then_fold() {
    let pattern: Pattern<Result<i32, String>> =
        Pattern::pattern(Ok(10), vec![Pattern::point(Ok(20)), Pattern::point(Ok(30))]);

    // Sequence to unwrap Results, then fold to sum
    let result: Result<i32, String> = pattern
        .sequence_result()
        .map(|p| p.fold(0, |acc, v| acc + v));

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 60);
}

// ====================================================================================
// Nested Composition Tests
// ====================================================================================

/// Test deeply nested map and traverse operations
#[test]
fn nested_map_traverse_composition() {
    let pattern = Pattern::pattern(
        1,
        vec![
            Pattern::pattern(2, vec![Pattern::point(3)]),
            Pattern::point(4),
        ],
    );

    // Complex nested pipeline
    let result = pattern
        .map(|v| v * 2) // Double everything
        .map(|v| format!("{}", v)) // Convert to strings
        .traverse_option(|s| s.parse::<i32>().ok()) // Parse back
        .map(|p| p.map(|v| v + 1)); // Increment all

    assert!(result.is_some());
    let unwrapped = result.unwrap();
    assert_eq!(unwrapped.value, 3); // (1 * 2) + 1
    assert_eq!(unwrapped.elements[0].value, 5); // (2 * 2) + 1
    assert_eq!(unwrapped.elements[0].elements[0].value, 7); // (3 * 2) + 1
    assert_eq!(unwrapped.elements[1].value, 9); // (4 * 2) + 1
}
