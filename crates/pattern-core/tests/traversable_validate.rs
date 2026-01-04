//! Tests for Pattern validate_all operations
//!
//! This module tests the validate_all method for Pattern<V>, which applies
//! a validation function to all values and collects ALL errors (unlike
//! traverse_result which short-circuits on first error).
//!
//! Key difference from traverse_result:
//! - traverse_result: Returns first error encountered (short-circuits)
//! - validate_all: Returns ALL errors encountered (processes entire pattern)

use pattern_core::Pattern;

// ====================================================================================
// Basic Validate Tests (T049-T051)
// ====================================================================================

/// T049: Validate with all valid values → Ok(Pattern<W>)
#[test]
fn validate_all_valid() {
    let pattern = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);

    // All values pass validation
    let result: Result<Pattern<i32>, Vec<String>> = pattern.validate_all(|v| {
        if *v > 0 {
            Ok(*v * 10)
        } else {
            Err(format!("value {} is not positive", v))
        }
    });

    assert!(result.is_ok());
    let validated = result.unwrap();
    assert_eq!(validated.value, 10);
    assert_eq!(validated.elements[0].value, 20);
    assert_eq!(validated.elements[1].value, 30);
}

/// T050: Validate with one invalid value → Err(Vec<E>) with that error
#[test]
fn validate_one_invalid() {
    let pattern = Pattern::pattern(1, vec![Pattern::point(-2), Pattern::point(3)]);

    // Second element fails validation
    let result: Result<Pattern<i32>, Vec<String>> = pattern.validate_all(|v| {
        if *v > 0 {
            Ok(*v * 10)
        } else {
            Err(format!("value {} is not positive", v))
        }
    });

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0], "value -2 is not positive");
}

/// T051: Validate with multiple invalid values → Err(Vec<E>) with ALL errors
#[test]
fn validate_multiple_invalid() {
    let pattern = Pattern::pattern(
        -1,
        vec![
            Pattern::point(2),
            Pattern::point(-3),
            Pattern::point(4),
            Pattern::point(-5),
        ],
    );

    // Root, third element, and fifth element fail validation
    let result: Result<Pattern<i32>, Vec<String>> = pattern.validate_all(|v| {
        if *v > 0 {
            Ok(*v * 10)
        } else {
            Err(format!("value {} is not positive", v))
        }
    });

    assert!(result.is_err());
    let errors = result.unwrap_err();
    // Should have 3 errors: root (-1), element (-3), element (-5)
    assert_eq!(errors.len(), 3);
    assert!(errors.contains(&"value -1 is not positive".to_string()));
    assert!(errors.contains(&"value -3 is not positive".to_string()));
    assert!(errors.contains(&"value -5 is not positive".to_string()));
}

// ====================================================================================
// Error Ordering Tests (T052)
// ====================================================================================

/// T052: Validate error ordering - root first, then elements in order
#[test]
fn validate_error_ordering() {
    let pattern = Pattern::pattern(
        -1,
        vec![Pattern::point(2), Pattern::point(-3), Pattern::point(4)],
    );

    let result: Result<Pattern<i32>, Vec<String>> = pattern.validate_all(|v| {
        if *v > 0 {
            Ok(*v * 10)
        } else {
            Err(format!("value {} is not positive", v))
        }
    });

    assert!(result.is_err());
    let errors = result.unwrap_err();
    // Should have 2 errors in order: root (-1) first, then element (-3)
    assert_eq!(errors.len(), 2);
    assert_eq!(errors[0], "value -1 is not positive");
    assert_eq!(errors[1], "value -3 is not positive");
}

// ====================================================================================
// No Short-Circuit Tests (T053)
// ====================================================================================

/// T053: Validate processes all values (no short-circuit)
#[test]
fn validate_no_short_circuit() {
    use pattern_core::test_utils::helpers::EffectCounter;

    let pattern = Pattern::pattern(
        -1,
        vec![Pattern::point(-2), Pattern::point(-3), Pattern::point(-4)],
    );

    let counter = EffectCounter::new();
    let result: Result<Pattern<i32>, Vec<String>> = pattern.validate_all(|v| {
        counter.increment();
        if *v > 0 {
            Ok(*v * 10)
        } else {
            Err(format!("value {} is not positive", v))
        }
    });

    assert!(result.is_err());
    let errors = result.unwrap_err();
    // Should have 4 errors (all values invalid)
    assert_eq!(errors.len(), 4);

    // Should have called the function 4 times (all values processed, no short-circuit)
    assert_eq!(counter.count(), 4);
}

/// Compare with traverse_result which DOES short-circuit
#[test]
fn validate_vs_traverse_result_short_circuit() {
    use pattern_core::test_utils::helpers::EffectCounter;

    let pattern = Pattern::pattern(
        -1,
        vec![Pattern::point(-2), Pattern::point(-3), Pattern::point(-4)],
    );

    // Test traverse_result (should short-circuit)
    let traverse_counter = EffectCounter::new();
    let traverse_result: Result<Pattern<i32>, String> = pattern.traverse_result(|v| {
        traverse_counter.increment();
        if *v > 0 {
            Ok(*v * 10)
        } else {
            Err(format!("value {} is not positive", v))
        }
    });
    assert!(traverse_result.is_err());
    // traverse_result should stop at first error (root)
    assert_eq!(traverse_counter.count(), 1);

    // Test validate_all (should NOT short-circuit)
    let validate_counter = EffectCounter::new();
    let validate_result: Result<Pattern<i32>, Vec<String>> = pattern.validate_all(|v| {
        validate_counter.increment();
        if *v > 0 {
            Ok(*v * 10)
        } else {
            Err(format!("value {} is not positive", v))
        }
    });
    assert!(validate_result.is_err());
    // validate_all should process all values
    assert_eq!(validate_counter.count(), 4);
}

// ====================================================================================
// Nested Structure Tests
// ====================================================================================

/// Validate with nested structure and multiple errors
#[test]
fn validate_nested_multiple_errors() {
    let pattern = Pattern::pattern(
        -1,
        vec![
            Pattern::pattern(2, vec![Pattern::point(-3), Pattern::point(4)]),
            Pattern::point(-5),
        ],
    );

    let result: Result<Pattern<i32>, Vec<String>> = pattern.validate_all(|v| {
        if *v > 0 {
            Ok(*v * 10)
        } else {
            Err(format!("value {} is not positive", v))
        }
    });

    assert!(result.is_err());
    let errors = result.unwrap_err();
    // Should have 3 errors: root (-1), nested element (-3), element (-5)
    assert_eq!(errors.len(), 3);
    assert_eq!(errors[0], "value -1 is not positive");
    assert_eq!(errors[1], "value -3 is not positive");
    assert_eq!(errors[2], "value -5 is not positive");
}

// ====================================================================================
// Type Transformation Tests
// ====================================================================================

/// Validate with type transformation
#[test]
fn validate_type_transformation() {
    let pattern = Pattern::pattern(
        "1",
        vec![
            Pattern::point("2"),
            Pattern::point("invalid"),
            Pattern::point("3"),
            Pattern::point("bad"),
        ],
    );

    let result: Result<Pattern<i32>, Vec<String>> = pattern.validate_all(|s| {
        s.parse::<i32>()
            .map_err(|e| format!("parse error for '{}': {}", s, e))
    });

    assert!(result.is_err());
    let errors = result.unwrap_err();
    // Should have 2 errors for the two invalid strings
    assert_eq!(errors.len(), 2);
    assert!(errors[0].contains("invalid"));
    assert!(errors[1].contains("bad"));
}

// ====================================================================================
// Edge Cases
// ====================================================================================

/// Atomic pattern validation
#[test]
fn validate_atomic_valid() {
    let pattern = Pattern::point(5);

    let result: Result<Pattern<i32>, Vec<String>> = pattern.validate_all(|v| {
        if *v > 0 {
            Ok(*v * 10)
        } else {
            Err(format!("value {} is not positive", v))
        }
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 50);
}

/// Atomic pattern validation with error
#[test]
fn validate_atomic_invalid() {
    let pattern = Pattern::point(-5);

    let result: Result<Pattern<i32>, Vec<String>> = pattern.validate_all(|v| {
        if *v > 0 {
            Ok(*v * 10)
        } else {
            Err(format!("value {} is not positive", v))
        }
    });

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0], "value -5 is not positive");
}
